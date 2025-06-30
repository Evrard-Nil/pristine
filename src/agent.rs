use std::collections::HashMap;
use std::sync::Arc;

use chrono::Utc;

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

const MAX_PAST_EVENTS: usize = 5;

pub struct AgentContext {
    memories: HashMap<String, String>,

    last_action_output: Option<String>,
    last_thought: Option<String>,

    known_open_issues: Vec<github::Issue>,
    known_closed_issues_titles: Vec<String>,
    past_events: Vec<String>,
    new_event: Vec<String>,

    error: Option<String>,
}

impl AgentContext {
    fn build_contextual_prompt(&self) -> String {
        let mut prompt = String::new();
        let current_time = Utc::now();
        prompt.push_str(&format!("Current time: {}\n", current_time.to_rfc3339()));

        if !self.memories.is_empty() {
            prompt.push_str("\nMemories:\n");
            for (key, value) in &self.memories {
                prompt.push_str(&format!("{}: {}\n", key, value));
            }
        }

        if !self.known_open_issues.is_empty() {
            prompt.push_str("\nKnown Open Issues:\n");
            for issue in &self.known_open_issues {
                let duration = current_time.signed_duration_since(issue.updated_at);
                let time_ago = format_duration(duration);
                prompt.push_str(&format!(
                    "Issue #{}: {} (Updated {} ago)\nState: {}\nLabels: {:?}\nBody: {}\nComments and Updates: {:?}\n",
                    issue.number,
                    issue.title,
                    time_ago,
                    issue.state,
                    issue.labels,
                    issue.body,
                    issue.comments
                ));
            }
        }

        if !self.known_closed_issues_titles.is_empty() {
            prompt.push_str("\nKnown Closed Issues:\n");
            for title in &self.known_closed_issues_titles {
                prompt.push_str(&format!("{}\n", title));
            }
        }

        let past_events_to_display = self
            .past_events
            .iter()
            .rev()
            .take(MAX_PAST_EVENTS)
            .collect::<Vec<_>>();
        if !past_events_to_display.is_empty() {
            prompt.push_str("\nPast Events (most recent first):\n");
            for event in past_events_to_display.into_iter().rev() {
                prompt.push_str(&format!("{}\n", event));
            }
        }

        if !self.new_event.is_empty() {
            prompt.push_str("\nNew Events:\n");
            for event in &self.new_event {
                prompt.push_str(&format!("{}\n", event));
            }
        }

        if let Some(last_action_output) = &self.last_action_output {
            prompt.push_str(&format!("Last Action Output: {}\n", last_action_output));
        }
        if let Some(last_thought) = &self.last_thought {
            prompt.push_str(&format!("Last Thought: {}\n", last_thought));
        }

        if let Some(error) = &self.error {
            prompt.push_str(&format!("\nError: {}\n", error));
        }
        prompt.push_str("\n");

        prompt
    }
}

fn format_duration(duration: chrono::Duration) -> String {
    if duration.num_days() > 0 {
        format!("{} days", duration.num_days())
    } else if duration.num_hours() > 0 {
        format!("{} hours", duration.num_hours())
    } else if duration.num_minutes() > 0 {
        format!("{} minutes", duration.num_minutes())
    } else {
        format!("{} seconds", duration.num_seconds())
    }
}

pub struct Agent {
    github: github::GitHubClient,
    repo: repository::RepositoryManager,
    llm: llm::LlmClient,
    monitor: Arc<Monitor>,
    known_issues: Vec<github::Issue>,

    agent_context: AgentContext,
}

impl Agent {
    pub async fn new(config: &config::Config) -> anyhow::Result<Self> {
        let github = github::GitHubClient::new(config).await?;
        let (repo_dir, repo) = github.clone_repository().await?;
        let repo = repository::RepositoryManager::new(repo_dir, repo, config)?;
        let mut llm = llm::LlmClient::new(config)?;
        let monitor = Arc::new(Monitor::new());
        llm.set_monitor(monitor.clone());
        let (known_issues, _, _) = github.list_all_issues(None, &vec![]).await?;
        let known_closed_issues_titles = known_issues
            .iter()
            .filter(|issue| issue.state == "closed")
            .map(|issue| issue.title.clone())
            .collect::<Vec<String>>();
        let known_open_issues = known_issues
            .iter()
            .filter(|issue| issue.state == "open")
            .cloned()
            .collect::<Vec<github::Issue>>();

        Ok(Self {
            github,
            repo,
            llm,
            monitor,
            known_issues,
            agent_context: AgentContext {
                memories: HashMap::new(),
                known_open_issues,
                known_closed_issues_titles,
                past_events: Vec::new(),
                new_event: Vec::new(),

                last_action_output: None,
                last_thought: None,
                error: None,
            },
        })
    }

    pub fn get_memory(&self, key: &str) -> Option<&String> {
        self.agent_context.memories.get(key)
    }

    pub fn set_memory(&mut self, key: String, value: String) {
        self.agent_context.memories.insert(key, value);
    }

    pub fn remove_memory(&mut self, key: &str) {
        self.agent_context.memories.remove(key);
    }

    pub fn get_monitor(&self) -> Arc<Monitor> {
        self.monitor.clone()
    }

    pub async fn check_for_events(&mut self) -> Vec<String> {
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
            let event = format!(
                "New commit detected: {} - {}",
                commit.id(),
                commit.message().unwrap_or("No message")
            );
            events.push(event);
        }

        // Get all current issues
        let (current_issues, new_issues, new_updates) =
            match self.github.list_all_issues(None, &self.known_issues).await {
                Ok(issues) => issues,
                Err(e) => {
                    println!("Failed to list issues: {}", e);
                    return events;
                }
            };
        if new_issues {
            events.push("New issue detected".to_string());
        }
        if new_updates {
            events.push("New updates on existing issues detected".to_string());
        }
        self.known_issues = current_issues;
        self.agent_context.known_open_issues = self
            .known_issues
            .iter()
            .filter(|issue| issue.state == "open")
            .cloned()
            .collect();
        self.agent_context.known_closed_issues_titles = self
            .known_issues
            .iter()
            .filter(|issue| issue.state == "closed")
            .map(|issue| issue.title.clone())
            .collect();

        events
    }

    pub async fn start(mut self) -> ! {
        println!("Starting agent...");
        println!("System prompt: \n{}", thinking_system_prompt());
        loop {
            self.agent_context
                .past_events
                .extend(self.agent_context.new_event.drain(..));
            // Trim past_events to MAX_PAST_EVENTS
            if self.agent_context.past_events.len() > MAX_PAST_EVENTS {
                self.agent_context
                    .past_events
                    .drain(0..self.agent_context.past_events.len() - MAX_PAST_EVENTS);
            }
            let new_events = self.check_for_events().await;
            if !new_events.is_empty() {
                println!("New events detected: {:?}", new_events);
            }
            self.agent_context.new_event = new_events;

            self.think().await;
            let actions = self.decide().await;
            if actions.is_empty() {
                println!("No actions decided. Waiting for new events...");
            } else {
                println!("Decided actions:");
                for action in &actions {
                    println!("{:?}", action);
                }
                let mut outputs = String::new();
                for action in actions {
                    let o = self.act(action.clone()).await;
                    println!("Action output: {}", o);
                    outputs.push_str(format!("Action: {:?}\nOutput: {}\n", action, o).as_str());
                }
                // Update the last action and output in the agent context
                self.agent_context.last_action_output = Some(outputs.clone());
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

        let mut prompt = self.agent_context.build_contextual_prompt();
        prompt.push_str("\n\nNow, think about what actions to take next.\n");

        let thought = self
            .llm
            .generate_text(&thinking_system_prompt(), &prompt)
            .await
            .unwrap();
        println!("Thought: {}", thought);
        self.agent_context.last_thought = Some(thought);
    }

    pub async fn decide(&mut self) -> Vec<Actions> {
        let mut prompt = self.agent_context.build_contextual_prompt();
        prompt.push_str("\n\nNow, decide on the actions to take next.\n");

        let actions_json = self
            .llm
            .generate_text(&action_system_prompt(), &prompt)
            .await
            .unwrap();
        println!("Actions JSON: {}", actions_json);

        let actions: Vec<Actions> = serde_json::from_str(&actions_json).unwrap_or_else(|_| {
            println!("Failed to parse actions JSON: {}", actions_json);
            self.agent_context.error =
                Some(format!("Failed to parse actions JSON: {}", actions_json));
            vec![]
        });

        actions
    }

    pub async fn act(&mut self, action: Actions) -> String {
        println!("Acting on action: {:?}", action);
        let start_time = std::time::Instant::now();
        let action_clone = action.clone();

        let output: String = match action {
            Actions::ListAllFiles => self
                .repo
                .list_all_files()
                .await
                .unwrap_or_default()
                .join(", "),
            Actions::ReadASingleFile { path } => {
                let file_content = self.repo.read_file(&path).await.unwrap_or_default();
                // clip the content to a reasonable length for display
                let clipped_content = if file_content.len() > 5000 {
                    format!("{}...", &file_content[..5000])
                } else {
                    file_content
                };
                clipped_content
            }
            Actions::StoreOrUpdateMemoryInContext { key, value } => {
                let ouput = format!("Stored memory: {} = {}", key, value);
                self.set_memory(key, value);
                ouput
            }
            Actions::RemoveMemoryFromContext { key } => {
                self.remove_memory(&key);
                format!("Removed memory: {}", key)
            }
            Actions::GithubCreateIssue {
                title,
                body,
                labels,
            } => {
                match self
                    .github
                    .create_issue(title.clone(), body.clone(), labels)
                    .await
                {
                    Ok(i) => {
                        format!("Created issue: {} - {}", i, title)
                    }
                    Err(err) => {
                        println!("Error creating issue: {}", err);
                        format!("Failed to create issue: {}", err)
                    }
                }
            }
            Actions::GithubGetIssue { issue_number } => {
                let issue = self.github.get_issue(issue_number).await.unwrap();
                serde_json::to_string(&issue).unwrap_or_else(|_| {
                    println!("Failed to serialize issue: {}", issue_number);
                    format!("Failed to serialize issue: {}", issue_number)
                })
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
                format!("Added label '{}' to issue #{}", label, issue_number)
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
                format!("Removed label '{}' from issue #{}", label, issue_number)
            }
            Actions::GithubCloseIssue { issue_number } => {
                self.github.close_issue(issue_number).await.unwrap();
                println!("Closed issue #{}", issue_number);
                format!("Closed issue #{}", issue_number)
            }
            Actions::GithubCommentOnIssue { issue_number, body } => {
                self.github
                    .comment_on_issue(issue_number, &body)
                    .await
                    .unwrap();
                println!("Commented on issue #{}: {}", issue_number, body);
                format!("Commented on issue #{}: {}", issue_number, body)
            }
            Actions::GithubEditBodyOfIssue { issue_number, body } => {
                self.github
                    .edit_issue_body(issue_number, &body)
                    .await
                    .unwrap();
                println!("Edited body of issue #{}: {}", issue_number, body);
                format!("Edited body of issue #{}: {}", issue_number, body)
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
                format!("Edited title of issue #{}: {}", issue_number, title)
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
                response
            }
            Actions::Sleep { duration } => {
                println!("Sleeping for {} seconds...", duration);
                tokio::time::sleep(std::time::Duration::from_secs(duration)).await;
                println!("Woke up after {} seconds.", duration);
                format!("Slept for {} seconds.", duration)
            }
        };

        // Log the action execution
        let duration_ms = start_time.elapsed().as_millis() as u64;
        self.monitor
            .log_action(action_clone, output.clone(), duration_ms);
        println!("Action executed in {} ms", duration_ms);
        output
    }
}
