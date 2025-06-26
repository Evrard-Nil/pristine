use octocrab::{
    current,
    models::{
        IssueState,
        events::payload::{EventPayload, IssuesEventAction, WrappedEventPayload},
    },
};
use serde::de;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use tokio::sync::mpsc::Receiver;

pub mod config;
pub mod github;
pub mod llm;
pub mod repository;

const GENERAL_SYSTEM_PROMPT: &str = r#"
You are a system that manages issues on a GitHub repository. 
Your goal is to help maintain the highest quality of code in the most efficient way possible.

Your responsibilities include:
- Detecting new documentation issues.
- Scope human created issues.
- Answer human comments on issues.
- Detect TODOs in code and map them to issues.
- Close issues when they are resolved.
- Scan the codebase to dectect logic flaws and create issues for them.
- Detect areas where testing could be improved and create issues for them.
- Suggest features or enhancements based on the codebase.
- Ensure no duplicate issues are created. If duplicates happen, close the duplicate issue and comment on it with a link to the original issue.

The issues you create should be small, actionable, and focused on a single task.

Here are the tags you can use for issues:
- `documentation`: For issues related to documentation improvements.
- `bug`: For issues related to bugs or logic flaws in the code.
- `enhancement`: For issues related to improvements or new features.
- `test`: For issues related to adding or improving tests.
- `needs-human-input`: For issues that require human input or decision-making.
- `ready-for-implementation`: For issues that are ready to be worked on.

You will be provided with the current context, which includes:
- The memories you have stored.
- Past actions you have taken.
- The output of the last action you took.
- The current size of the context.
- The list of actions you can take.
- The list of open issues in the repository.

A few rules to follow:
- Avoid opening issues that are too broad or vague.
- Always provide a clear title and description for the issues you create.
- Do not create issues that are duplicates of existing ones. 

Actions you can take:
"#;

const THINKING_ADD_ON: &str = r#"
Your role is to think carefully about the current state and what actions should be taken next. 
Output your thoughts in a concise manner.
"#;

const ACTION_ADD_ON: &str = r#"
Your role is to decide on the list of actions to take. Your output should be a JSON array of actions.
Each action should be a JSON representing of the `Actions` enum.
You should only output the JSON array, nothing else.
Example of output:
```json
[
    "ReadAllTheCodeBase",
    {
        "GithubCreateIssue": {
            "title": "Test Issue", 
            "body": "This is a test issue",
            "labels": [
                "bug",
                "test"
            ]
        }
    },
    {
        "StoreOrUpdateMemoryInContext": {
            "key": "test_key",
            "value": "test_value"
        }
    }
]
```

Batch actions together when possible, e.g. storing multiple memories at once.
"#;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, EnumIter)]
pub enum Actions {
    // Repo I/O
    ReadAllTheCodeBase,
    RunLLMInference {
        system_prompt: String,
        user_prompt: String,
    },
    ListAllFiles,
    ReadASingleFile {
        path: String,
    },

    // Context Management
    StoreOrUpdateMemoryInContext {
        key: String,
        value: String,
    },
    RemoveMemoryFromContext {
        key: String,
    },

    // Github
    GithubCreateIssue {
        title: String,
        body: String,
        labels: Vec<String>,
    },
    GithubListIssues {
        state: String,
    },
    GithubGetIssue {
        issue_number: u64,
    },
    GithubAddLabelToIssue {
        issue_number: u64,
        label: String,
    },
    GithubRemoveLabelFromIssue {
        issue_number: u64,
        label: String,
    },
    GithubCloseIssue {
        issue_number: u64,
    },
    GithubCommentOnIssue {
        issue_number: u64,
        body: String,
    },
    GithubEditBodyOfIssue {
        issue_number: u64,
        body: String,
    },
    GithubEditTitleOfIssue {
        issue_number: u64,
        title: String,
    },

    Sleep {
        duration: u64, // Duration in seconds
    },
}

impl Actions {
    pub fn name(&self) -> &str {
        match self {
            Actions::ReadAllTheCodeBase => "read_all_the_code_base",
            Actions::RunLLMInference { .. } => "run_llm_inference",
            Actions::ListAllFiles => "list_all_files",
            Actions::ReadASingleFile { .. } => "read_a_single_file",
            Actions::StoreOrUpdateMemoryInContext { .. } => "store_or_update_memory_in_context",
            Actions::RemoveMemoryFromContext { .. } => "remove_memory_from_context",
            Actions::GithubCreateIssue { .. } => "github_create_issue",
            Actions::GithubListIssues { .. } => "github_list_issues",
            Actions::GithubGetIssue { .. } => "github_get_issue",
            Actions::GithubAddLabelToIssue { .. } => "github_add_label_to_issue",
            Actions::GithubRemoveLabelFromIssue { .. } => "github_remove_label_from_issue",
            Actions::GithubCloseIssue { .. } => "github_close_issue",
            Actions::GithubCommentOnIssue { .. } => "github_comment_on_issue",
            Actions::GithubEditBodyOfIssue { .. } => "github_edit_body_of_issue",
            Actions::GithubEditTitleOfIssue { .. } => "github_edit_title_of_issue",
            Actions::Sleep { .. } => "sleep",
        }
    }

    pub fn desc(&self) -> &str {
        match self {
            Actions::ReadAllTheCodeBase => {
                "Read all the code in the repository and returns all code content."
            }
            Actions::RunLLMInference { .. } => {
                "Run LLM inference with the provided system and user prompts.\
                Returns the generated text from the LLM."
            }
            Actions::ListAllFiles => {
                "List all files in the repository and returns a list of file paths. e.g. ['src/lib.rs', 'src/main.rs']"
            }
            Actions::ReadASingleFile { .. } => {
                "Read a single file in the repository and returns the content of the file.\
                The file is identified by its path, which is a string.\
                Use this to read the content of a specific file in the repository."
            }
            Actions::StoreOrUpdateMemoryInContext { .. } => {
                "Store or update a memory in the context.\
                The memory is a key-value pair where the key is a string and the value is a string.\
                Use this to store important information that you should remember for future actions.\
                This is essential for the agent to maintain context and continuity in its actions.\
                "
            }
            Actions::RemoveMemoryFromContext { .. } => {
                "Remove a memory from the context. The memory is identified by a key which is a string.\
                Use this to remove information that is no longer relevant or needed."
            }
            Actions::GithubCreateIssue { .. } => {
                "Create a new issue in the GitHub repository.\
                The issue is identified by a title and a body, both of which are strings.\
                You can also specify labels for the issue."
            }
            Actions::GithubListIssues { .. } => {
                "List all issues in the GitHub repository.\
                You can specify the state of the issues to list (e.g., 'open', 'closed').\
                Returns a list of issues."
            }
            Actions::GithubGetIssue { .. } => {
                "Get a specific issue from the GitHub repository.\
                The issue is identified by its number, which is a u64.\
                Returns the details of the issue."
            }
            Actions::GithubAddLabelToIssue { .. } => {
                "Add a label to a specific issue in the GitHub repository.\
                The issue is identified by its number, which is a u64, and the label is a string."
            }
            Actions::GithubRemoveLabelFromIssue { .. } => {
                "Remove a label from a specific issue in the GitHub repository.\
                The issue is identified by its number, which is a u64, and the label is a string."
            }
            Actions::GithubCloseIssue { .. } => {
                "Close a specific issue in the GitHub repository.\
                The issue is identified by its number, which is a u64."
            }
            Actions::GithubCommentOnIssue { .. } => {
                "Add a comment to a specific issue in the GitHub repository.\
                The issue is identified by its number, which is a u64, and the comment body is a string."
            }
            Actions::GithubEditBodyOfIssue { .. } => {
                "Edit the body of a specific issue in the GitHub repository.\
                The issue is identified by its number, which is a u64, and the new body is a string."
            }
            Actions::GithubEditTitleOfIssue { .. } => {
                "Edit the title of a specific issue in the GitHub repository.\
                The issue is identified by its number, which is a u64, and the new title is a string."
            }
            Actions::Sleep { .. } => {
                "Sleep for a specified duration in seconds.\
                This action is used to pause the agent's execution for a while."
            }
        }
    }
}

pub fn general_system_prompt() -> String {
    let mut p = GENERAL_SYSTEM_PROMPT.to_string();
    for action in Actions::iter() {
        let name = action.name();
        let desc = action.desc();
        let json = serde_json::to_string(&action).unwrap();
        p.push_str(&format!("- `{}`: {} (JSON: `{}`)\n", name, desc, json));
    }
    p.push_str("\n");
    p
}

pub fn thinking_system_prompt() -> String {
    format!("{}{}", general_system_prompt(), THINKING_ADD_ON)
}

pub fn action_system_prompt() -> String {
    format!("{}{}", general_system_prompt(), ACTION_ADD_ON)
}

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
    memories: std::collections::HashMap<String, String>,
    past_actions: Vec<Actions>,
    past_events: Vec<Event>,
    new_event: Vec<Event>,
    last_action_output: Option<String>,
    last_thought: Option<String>,

    config: config::Config,
    github: github::GitHubClient,
    repo: repository::RepositoryManager,
    llm: llm::LlmClient,
    current_issues: Vec<octocrab::models::issues::Issue>,
    current_open_issues_title: Vec<(u64, String)>,
    max_issue_number: u64,
    last_event_date: chrono::DateTime<chrono::Utc>,
}

impl Agent {
    pub async fn new(config: config::Config) -> anyhow::Result<Self> {
        let github = github::GitHubClient::new(&config).await?;
        let (repo_dir, repo) = github.clone_repository()?;
        let repo = repository::RepositoryManager::new(repo_dir, repo, &config)?;
        let llm = llm::LlmClient::new(&config)?;
        let current_issues = github.list_all_issues(None).await?;
        let current_open_issues_title = current_issues
            .iter()
            .filter(|issue| issue.state == IssueState::Open)
            .map(|issue| (issue.number, issue.title.clone()))
            .collect::<Vec<(u64, String)>>();
        let max_issue_number = current_issues
            .iter()
            .map(|issue| issue.number)
            .max()
            .unwrap_or(0);
        let github_events = github.events().await.unwrap_or_default();
        let last_event_date = github_events
            .iter()
            .map(|event| event.created_at)
            .max()
            .unwrap_or(chrono::DateTime::default());

        Ok(Self {
            memories: std::collections::HashMap::new(),
            past_actions: Vec::new(),
            config,
            github,
            repo,
            llm,
            last_action_output: None,
            last_thought: None,
            new_event: Vec::new(),
            past_events: Vec::new(),
            current_issues,
            max_issue_number,
            last_event_date,
            current_open_issues_title,
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

    pub async fn check_for_events(&mut self) -> Vec<Event> {
        let mut events = vec![];

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
        let github_events = self.github.events().await.unwrap_or_default();
        let mut max_id = self.last_event_date;
        for event in github_events {
            if event.created_at <= self.last_event_date {
                continue; // Skip events that are older than the last processed event
            }
            println!("Processing event: {:?}", event);
            max_id = max_id.max(event.created_at);

            match event.payload {
                Some(WrappedEventPayload {
                    specific: Some(EventPayload::IssuesEvent(issue_event)),
                    ..
                }) => {
                    if issue_event.action == IssuesEventAction::Opened {
                        let event = Event::NewIssue {
                            issue_number: issue_event.issue.number,
                            title: issue_event.issue.title,
                            body: issue_event.issue.body.unwrap_or("".to_string()),
                        };
                        events.push(event);
                    }
                }
                Some(WrappedEventPayload {
                    specific: Some(EventPayload::IssueCommentEvent(comment_event)),
                    ..
                }) => {
                    let event = Event::NewComment {
                        issue_number: comment_event.issue.number,
                        body: comment_event.comment.body.unwrap_or("".to_string()),
                    };
                    events.push(event);
                }
                _ => {
                    // Handle other event types if needed
                    println!("Unhandled event type: {:?}", event);
                }
            }
        }
        self.last_event_date = max_id;

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
        println!("Prompt for thinking: \n{}", prompt);
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

    pub async fn decide(&self) -> Vec<Actions> {
        let mut prompt = self.build_contextual_prompt();
        prompt.push_str("\n\nNow, decide on the actions to take next.\n");
        println!("Prompt for deciding actions: \n{}", prompt);
        let actions_json = self
            .llm
            .generate_text(&action_system_prompt(), &prompt)
            .await
            .unwrap();
        println!("Actions JSON: {}", actions_json);

        let actions: Vec<Actions> = serde_json::from_str(&actions_json).unwrap_or_else(|_| {
            println!("Failed to parse actions JSON: {}", actions_json);
            vec![]
        });

        actions
    }

    pub async fn act(&mut self, action: Actions) {
        println!("Acting on action: {:?}", action);
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
                self.set_memory(key, value);
            }
            Actions::RemoveMemoryFromContext { key } => {
                self.remove_memory(&key);
            }
            Actions::GithubCreateIssue {
                title,
                body,
                labels,
            } => {
                let issue = self.github.create_issue(title, body, labels).await.unwrap();
                println!("Created issue #{}", issue.number);
                self.max_issue_number = issue.number.max(self.max_issue_number);
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_actions_ser() {
        let actions = vec![
            Actions::ReadAllTheCodeBase,
            Actions::GithubCreateIssue {
                title: "Test Issue".to_string(),
                body: "This is a test issue".to_string(),
                labels: vec!["bug".to_string(), "test".to_string()],
            },
            Actions::StoreOrUpdateMemoryInContext {
                key: "test_key".to_string(),
                value: "test_value".to_string(),
            },
        ];
        let serialized = serde_json::to_string(&actions).unwrap();
        println!("Serialized Actions: {}", serialized);
    }
}
