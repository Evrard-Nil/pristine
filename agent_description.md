### Agent Name
Pristine

### Overview
Pristine is an AI agent designed to autonomously manage issues within a GitHub repository. It continuously monitors the specified repository for new commits, issue updates, and comments. Leveraging a Large Language Model (LLM), Pristine analyzes the repository's state and its own internal context to identify, create, update, and close issues as needed. It aims to help maintain the repository by detecting documentation gaps, bugs, and missing tests, and by responding to human interactions on issues. A web-based dashboard is available for real-time monitoring of its activities.

### Key Capabilities
*   **GitHub Issue Management**: Creates new issues with titles, descriptions, and labels; updates existing issue bodies and titles; adds or removes labels; closes issues; and comments on issues.
*   **Codebase Analysis**: Clones the target GitHub repository locally, lists all files, reads file contents, and can execute shell commands within the repository to gather information for issue management.
*   **LLM-Powered Decision Making**: Utilizes an OpenAI-compatible LLM to process current context, past actions, and new events to "think" and decide on the next set of actions.
*   **Contextual Memory**: Stores and retrieves key-value pair "memories" to maintain continuity and remember important information, user preferences, and past decisions.
*   **Event-Driven Operation**: Automatically detects and reacts to new commits, updates to existing issues, and new comments on issues in the monitored GitHub repository.
*   **Monitoring Dashboard**: Provides a local web interface to visualize the agent's action history and LLM call logs, offering transparency into its operations.

### Operational Flow
The agent operates in a continuous loop:
1.  **Event Checking**: It pulls the latest changes from the GitHub repository and checks for new commits, updated issues, or new comments.
2.  **Thinking**: If new events are detected or the agent is not marked complete, it generates a "thought" using an LLM, analyzing the current state, memories, and events to determine necessary actions.
3.  **Acting**: It executes a batch of actions decided by the LLM, which can include GitHub operations (creating/updating issues, commenting), local repository interactions (reading files, running commands), or internal context management (storing/removing memories).
4.  **Logging**: All actions and LLM calls are logged and made available through the monitoring dashboard.
5.  **Sleeping**: After acting, the agent pauses for a short duration before repeating the cycle, unless explicitly marked complete, in which case it waits for external events to resume.

### Inputs
*   **Environment Variables**:
    *   `GITHUB_PERSONAL_ACCESS_TOKEN`: Required for GitHub API authentication.
    *   `OPENAI_API_KEY`: Required for OpenAI LLM API authentication.
    *   `OPENAI_API_BASE`: (Optional) Specifies the base URL for the OpenAI API (defaults to `https://api.openai.com`).
    *   `OPENAI_API_MODEL`: (Optional) Specifies the LLM model to use (defaults to `gpt-3.5-turbo`).
    *   `GITHUB_REPOSITORY_OWNER`: Required for identifying the GitHub repository owner.
    *   `GITHUB_REPOSITORY_NAME`: Required for identifying the GitHub repository name.
    *   `GITHUB_REPOSITORY_ISSUES_BRANCH`: (Optional) Specifies the branch for issues-related operations (defaults to `issues`).
*   **GitHub Repository**: New commits, updates to issues, and new comments within the configured repository.

### Outputs
*   **GitHub Actions**: Creates, updates, or closes issues; adds or removes labels; and posts comments on issues.
*   **Local Repository Operations**: Clones the target GitHub repository, reads file contents, and executes shell commands.
*   **Web Dashboard**: Provides a monitoring interface accessible via HTTP on port `5005`, displaying action history and LLM call logs.
*   **Console Output**: Logs its operational steps, LLM interactions, and action results to the console.