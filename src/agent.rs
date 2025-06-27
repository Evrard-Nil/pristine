use octocrab::models::IssueState;
use std::collections::HashMap;
use std::sync::Arc;

use crate::actions::{Actions, action_system_prompt, thinking_system_prompt};
use crate::config;
use crate::github;
use crate::llm;
use crate::monitoring::Monitor;
use crate::repository;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Event {
    NewCommit {
        commit_hash: String,
        message: String,
    },
    NewIssue {
        issue_number: u64,
        title: String,
        body: String,
    },
    NewComment {
        issue_number: u64,
        body: String,
    },
    NewPullRequest {
        pull_request_number: u64,
        title: String,
        body: String,
    },
}

pub struct Agent {
    memories: HashMap<String, String>,
    past_actions: Vec<Actions>,
    past_events: Vec<Event>,
    new_event: Vec<Event>,
    last_action_output: Option<String>,
    last_thought: Option<String>,

    github: github::GitHubClient,
    repo: repository::RepositoryManager,
    llm: llm::LlmClient,
    monitor: Arc<Monitor>,
    current_issues: Vec<octocrab::models::issues::Issue>,
    current_open_issues_title: Vec<(u64, String)>,
    known_issues: HashMap<u64, (String, chrono::DateTime<chrono::Utc>)>, // issue_number -> (last_body, last_updated)
    known_comments: HashMap<u64, Vec<(u64, String)>>, // issue_number -> Vec<(comment_id, comment_body)>
}

impl Agent {
    pub async fn new(config: &config::Config) -> anyhow::Result<Self> {
        let github = github::GitHubClient::new(config).await?;
        let (repo_dir, repo) = github.clone_repository().await?;
        let repo = repository::RepositoryManager::new(repo_dir, repo, config)?;
        let mut llm = llm::LlmClient::new(config)?;
        let monitor = Arc::new(Monitor::new());
        llm.set_monitor(monitor.clone());
        let current_issues = github.list_all_issues(None).await?;
        let current_open_issues_title = current_issues
            .iter()
            .filter(|issue| issue.state == IssueState::Open)
            .map(|issue| (issue.number, issue.title.clone()))
            .collect::<Vec<(u64, String)>>();
        // Initialize known issues and comments from current state
        let mut known_issues = HashMap::new();
        let mut known_comments = HashMap::new();

        for issue in &current_issues {
            known_issues.insert(
                issue.number,
                (issue.body.clone().unwrap_or_default(), issue.updated_at),
            );

            // Get initial comments for each issue
            if let Ok(comments) = github.get_issue_comments(issue.number).await {
                let comment_data: Vec<(u64, String)> = comments
                    .into_iter()
                    .map(|c| (c.id.0, c.body.unwrap_or_default()))
                    .collect();
                known_comments.insert(issue.number, comment_data);
            }
        }

        Ok(Self {
            memories: HashMap::new(),
            past_actions: Vec::new(),
            github,
            repo,
            llm,
            monitor,
            last_action_output: None,
            last_thought: None,
            new_event: Vec::new(),
            past_events: Vec::new(),
            current_issues,
            current_open_issues_title,
            known_issues,
            known_comments,
        })
    }

    pub fn get_memory(&self, key: &str) -> Option<&String> {
        self.memories.get(key)
    }

    pub fn set_memory(&mut self, key: String, value: String) {
        self.memories.insert(key, value);
    }

    pub fn remove_memory(&mut self, key: &str) {
        self.memories.remove(key);
    }

    pub fn get_monitor(&self) -> Arc<Monitor> {
        self.monitor.clone()
    }

    pub async fn check_for_events(&mut self) -> Vec<Event> {
        let mut events = vec![];

        // Check for new commits
        if self.repo.pull().is_err() {
            println!("Failed to pull the latest changes from the repository.");
            return events;
        }
        if self.repo.new_commit() {
            let Ok(commit) = self.repo.get_latest_commit() else {
                println!("Failed to get the latest commit.");
                return events;
            };
            println!("New commit detected: {}", commit.id());
            let event = Event::NewCommit {
                commit_hash: commit.id().to_string(),
                message: commit.message().unwrap_or("").to_string(),
            };
            events.push(event);
        }

        // Get all current issues
        let current_issues = match self.github.list_all_issues(Some("all".to_string())).await {
            Ok(issues) => issues,
            Err(e) => {
                println!("Failed to list issues: {}", e);
                return events;
            }
        };

        // Check for new issues
        for issue in &current_issues {
            if !self.known_issues.contains_key(&issue.number) {
                println!("New issue detected: #{} - {}", issue.number, issue.title);
                let event = Event::NewIssue {
                    issue_number: issue.number,
                    title: issue.title.clone(),
                    body: issue.body.clone().unwrap_or_default(),
                };
                events.push(event);

                // Add to known issues
                self.known_issues.insert(
                    issue.number,
                    (issue.body.clone().unwrap_or_default(), issue.updated_at),
                );
            }
        }

        // Check for new comments on open issues
        for issue in &current_issues {
            if issue.state == IssueState::Open {
                match self.github.get_issue_comments(issue.number).await {
                    Ok(comments) => {
                        let current_comment_data: Vec<(u64, String)> = comments
                            .iter()
                            .map(|c| (c.id.0, c.body.clone().unwrap_or_default()))
                            .collect();

                        // Get known comments for this issue
                        let known_comments_for_issue = self
                            .known_comments
                            .get(&issue.number)
                            .cloned()
                            .unwrap_or_default();

                        // Check for new comments
                        for (comment_id, comment_body) in &current_comment_data {
                            if !known_comments_for_issue
                                .iter()
                                .any(|(id, _)| id == comment_id)
                            {
                                println!("New comment detected on issue #{}", issue.number);
                                let event = Event::NewComment {
                                    issue_number: issue.number,
                                    body: comment_body.clone(),
                                };
                                events.push(event);
                            }
                        }

                        // Update known comments
                        self.known_comments
                            .insert(issue.number, current_comment_data);
                    }
                    Err(e) => {
                        println!("Failed to get comments for issue #{}: {}", issue.number, e);
                    }
                }
            }
        }

        // Update current issues list
        self.current_issues = current_issues;
        self.current_open_issues_title = self
            .current_issues
            .iter()
            .filter(|issue| issue.state == IssueState::Open)
            .map(|issue| (issue.number, issue.title.clone()))
            .collect();

        events
    }

    pub async fn start(mut self) -> ! {
        loop {
            self.past_events.extend(self.new_event.clone());
            self.new_event.clear();
            let new_events = self.check_for_events().await;
            if !new_events.is_empty() {
                println!("New events detected: {:?}", new_events);
            }
            self.new_event = new_events;

            self.think().await;
            let actions = self.decide().await;
            if actions.is_empty() {
                println!("No actions decided. Waiting for new events...");
            } else {
                println!("Decided actions: {:?}", actions);
                for action in actions {
                    self.past_actions.push(action.clone());
                    self.act(action).await;
                }
            }
            // Sleep for a while before the next iteration
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }
    }

    pub async fn think(&mut self) {
        // This function should implement the thinking logic of the agent.
        // It should analyze the current state, past actions, and events to decide what to do next.
        // For now, we will just print the current state.
        println!("Thinking about the current state...");

        let mut prompt = self.build_contextual_prompt();
        prompt.push_str("\n\nNow, think about what actions to take next.\n");

        let thought = self
            .llm
            .generate_text(&thinking_system_prompt(), &prompt)
            .await
            .unwrap();
        self.last_thought = Some(thought.clone());
        println!("Thought: {}", thought);
    }

    fn build_contextual_prompt(&self) -> String {
        let mut prompt = String::new();
        prompt.push_str("\n\nCurrent context:\n");
        prompt.push_str(&format!("Memories: {:?}\n", self.memories));
        prompt.push_str(&format!(
            "Past actions (old to new): {:?}\n",
            self.past_actions
        ));
        prompt.push_str(&format!(
            "Past events (old to new): {:?}\n",
            self.past_events
        ));
        prompt.push_str(&format!("New event: {:?}\n", self.new_event));
        prompt.push_str(&format!(
            "Last action output: {:?}\n",
            self.last_action_output
        ));
        prompt.push_str(&format!("Last thought: {:?}\n", self.last_thought));
        prompt.push_str(&format!(
            "Current open issues: {:?}\n",
            self.current_open_issues_title
        ));
        prompt
    }

    pub async fn decide(&mut self) -> Vec<Actions> {
        let mut prompt = self.build_contextual_prompt();
        prompt.push_str("\n\nNow, decide on the actions to take next.\n");

        let actions_json = self
            .llm
            .generate_text(&action_system_prompt(), &prompt)
            .await
            .unwrap();
        println!("Actions JSON: {}", actions_json);

        let actions: Vec<Actions> = serde_json::from_str(&actions_json).unwrap_or_else(|_| {
            println!("Failed to parse actions JSON: {}", actions_json);
            self.last_action_output =
                Some(format!("Failed to parse actions JSON: {}", actions_json));
            vec![]
        });

        actions
    }

    pub async fn act(&mut self, action: Actions) {
        println!("Acting on action: {:?}", action);
        let start_time = std::time::Instant::now();
        let action_clone = action.clone();

        match action {
            Actions::ReadAllTheCodeBase => {
                let code = self.repo.read_all_code().await.unwrap_or_default();
                self.last_action_output = Some(code);
            }
            Actions::ListAllFiles => {
                let files = self.repo.list_all_files().await.unwrap_or_default();
                self.last_action_output = Some(files.join(", "));
            }
            Actions::ReadASingleFile { path } => {
                let content = self.repo.read_file(&path).await.unwrap_or_default();
                self.last_action_output = Some(content);
            }
            Actions::StoreOrUpdateMemoryInContext { key, value } => {
                self.last_action_output = Some(format!("Stored memory: {} = {}", key, value));
                self.set_memory(key, value);
            }
            Actions::RemoveMemoryFromContext { key } => {
                self.last_action_output = Some(format!("Removed memory: {}", key));
                self.remove_memory(&key);
            }
            Actions::GithubCreateIssue {
                title,
                body,
                labels,
            } => {
                let issue = match self
                    .github
                    .create_issue(title.clone(), body.clone(), labels)
                    .await
                {
                    Ok(i) => {
                        self.last_action_output =
                            Some(format!("Created issue: #{} - {}", i.number, i.title));
                        i
                    }
                    Err(err) => {
                        self.last_action_output =
                            Some(format!("Failed to create issue: {}", err.to_string()));
                        println!("Error creating issue: {}", err);
                        return;
                    }
                };
                println!("Created issue #{}", issue.number);

                // Add to known issues
                self.known_issues
                    .insert(issue.number, (body, issue.updated_at));

                // Initialize empty comments for new issue
                self.known_comments.insert(issue.number, Vec::new());
            }
            Actions::GithubListIssues { state } => {
                let issues = self
                    .github
                    .list_all_issues(Some(state.clone()))
                    .await
                    .unwrap_or_default();
                self.current_issues = issues.clone();
                if state == "open" || state == "all" {
                    self.current_open_issues_title = issues
                        .iter()
                        .filter(|issue| issue.state == IssueState::Open)
                        .map(|issue| (issue.number, issue.title.clone()))
                        .collect::<Vec<(u64, String)>>();
                }
                println!("Listed issues: {:?}", issues);
            }
            Actions::GithubGetIssue { issue_number } => {
                let issue = self.github.get_issue(issue_number).await.unwrap();
                println!("Got issue: {:?}", issue);
            }
            Actions::GithubAddLabelToIssue {
                issue_number,
                label,
            } => {
                self.github
                    .add_label_to_issue(issue_number, &label)
                    .await
                    .unwrap();
                println!("Added label '{}' to issue #{}", label, issue_number);
            }
            Actions::GithubRemoveLabelFromIssue {
                issue_number,
                label,
            } => {
                self.github
                    .remove_label_from_issue(issue_number, &label)
                    .await
                    .unwrap();
                println!("Removed label '{}' from issue #{}", label, issue_number);
            }
            Actions::GithubCloseIssue { issue_number } => {
                self.github.close_issue(issue_number).await.unwrap();
                println!("Closed issue #{}", issue_number);
            }
            Actions::GithubCommentOnIssue { issue_number, body } => {
                self.github
                    .comment_on_issue(issue_number, &body)
                    .await
                    .unwrap();
                println!("Commented on issue #{}: {}", issue_number, body);
            }
            Actions::GithubEditBodyOfIssue { issue_number, body } => {
                self.github
                    .edit_issue_body(issue_number, &body)
                    .await
                    .unwrap();
                println!("Edited body of issue #{}: {}", issue_number, body);
            }
            Actions::GithubEditTitleOfIssue {
                issue_number,
                title,
            } => {
                self.github
                    .edit_issue_title(issue_number, &title)
                    .await
                    .unwrap();
                println!("Edited title of issue #{}: {}", issue_number, title);
            }
            Actions::RunLLMInference {
                system_prompt,
                user_prompt,
            } => {
                let response = self
                    .llm
                    .generate_text(&system_prompt, &user_prompt)
                    .await
                    .unwrap_or_default();
                println!("LLM response: {}", response);
                self.last_action_output = Some(response);
            }
            Actions::Sleep { duration } => {
                println!("Sleeping for {} seconds...", duration);
                tokio::time::sleep(std::time::Duration::from_secs(duration)).await;
                println!("Woke up after {} seconds.", duration);
            }
        }

        // Log the action execution
        let duration_ms = start_time.elapsed().as_millis() as u64;
        self.monitor
            .log_action(action_clone, self.last_action_output.clone(), duration_ms);
    }
}
