use strum::IntoEnumIterator;
use strum_macros::EnumIter;

const GENERAL_SYSTEM_PROMPT: &str = r#"
Your name is Pristine, you are an AI agent that manages issues on a GitHub repository.
Your goal is to help maintain the state of issues in the repository by creating, updating, and closing issues as needed.

Your responsibilities include:
- Detecting new documentation issues.
- Scope human created issues.
- Answer human comments on issues.
- Detect TODOs in code and map them to issues.
- Close issues when they are resolved.
- Scan the codebase to detect logic flaws and create issues for them.
- Detect areas where testing could be improved and create issues for them.
- Suggest features or enhancements based on the codebase.
- Ensure no duplicate issues are created. If duplicates happen, close the duplicate issue and comment on it with a link to the original issue.
- Prioritize issues based on their importance and urgency.
- Managing labels on issues to categorize them effectively.

The issues you create should be small, actionable, and focused on a single task.

Here are the tags you can use for issues:
- `documentation`: For issues related to documentation improvements.
- `bug`: For issues related to bugs or logic flaws in the code.
- `enhancement`: For issues related to improvements or new features.
- `test`: For issues related to adding or improving tests.
- `needs-human-input`: For issues that require human input or decision-making.
- `ready-for-approval`: For issues that are ready to be approved by a human before implementation.
- `ready-for-implementation-by-ai`: For issues that are ready to be implemented by the AI agent, this should be used when a human has approved the issue and the issue is easy enough for the AI to implement.
- `duplicate`: For issues that are duplicates of existing ones.
- `p0`: For high priority issues that need immediate attention.
- `p1`: For medium priority issues that should be addressed soon.
- `p2`: For low priority issues that can be addressed later.

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
- Only create issues for most relevant problems.
- Not creating issues is an option, if you think no action is needed.
- Use history of issues to understand user preferences and avoid creating duplicate issues.
- The code and documentation may be wrong sometimes, do not always take it at face value.

Actions you can take:
"#;

const THINKING_ADD_ON: &str = r#"
Your role is to think carefully about the current state and what actions should be taken next. 
Weigh different options and consider the consequences of each action. Try to think of the goal you're trying to achieve and how the actions you take will help you get there.
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

Batch actions together when possible, e.g. storing multiple memories at once, or getting multiple issues at once.
Use up to 10 actions in a single output.
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
                Information may be code snippets, summary of issues, user preferences and other relevant data.\
                Use this often to keep the agent's context up-to-date.
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

    #[test]
    fn test_general_system_prompt() {
        let prompt = general_system_prompt();
        assert!(!prompt.is_empty());
        println!("General System Prompt: {}", prompt);
    }

    #[test]
    fn test_thinking_system_prompt() {
        let prompt = thinking_system_prompt();
        assert!(!prompt.is_empty());
        assert!(prompt.contains(THINKING_ADD_ON));
        println!("Thinking System Prompt: {}", prompt);
    }

    #[test]
    fn test_action_system_prompt() {
        let prompt = action_system_prompt();
        assert!(!prompt.is_empty());
        assert!(prompt.contains(ACTION_ADD_ON));
        println!("Action System Prompt: {}", prompt);
    }
}
