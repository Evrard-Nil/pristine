### Pristine Agent

Pristine is an AI agent designed to automate and assist with GitHub issue management. It continuously monitors a specified GitHub repository, leveraging a Large Language Model (LLM) to intelligently create, update, and close issues, manage labels, and respond to comments.

#### Key Capabilities

*   **Intelligent Issue Management**: Creates new issues for detected bugs, missing documentation, or absent unit tests. It also updates existing issues based on new events and closes issues when they are resolved.
*   **Codebase Monitoring**: Clones and periodically pulls the target GitHub repository to detect new commits and changes.
*   **Contextual Awareness**: Stores and retrieves "memories" to maintain continuity and remember important information, such as user preferences or summaries of past issues.
*   **LLM-Powered Decision Making**: Utilizes an OpenAI-compatible LLM to analyze repository context, generate thoughts, and decide on appropriate actions, including crafting detailed issue descriptions and comments.
*   **Event-Driven Responses**: Reacts to various GitHub events, including new commits, new issues, updates to existing issues, and new comments.
*   **Interactive Dashboard**: Provides a local web interface for real-time monitoring of the agent's actions and LLM interactions.

#### Operation

The agent operates in a continuous loop:
1.  **Checks for Events**: It pulls the latest changes from the GitHub repository and fetches all issues to identify new commits, new issues, or updates to existing issues or comments.
2.  **Thinks**: If new events are detected or the agent is not marked complete, it uses an LLM to analyze the current context (memories, open issues, recent events, last action output) and formulate a plan, which includes a natural language thought process and a list of actions in JSON format.
3.  **Acts**: It executes the planned actions, which can involve interacting with the GitHub API (e.g., creating issues, commenting), managing its internal memory, reading repository files, or running shell commands.
4.  **Logs**: All actions and LLM calls are logged and accessible via the web dashboard.
5.  **Sleeps**: After performing actions, the agent pauses for a short duration before repeating the cycle.

#### Inputs

*   **Environment Variables**:
    *   `GITHUB_PERSONAL_ACCESS_TOKEN`: Your GitHub Personal Access Token for API authentication.
    *   `OPENAI_API_KEY`: Your OpenAI API key for LLM inference.
    *   `OPENAI_API_BASE` (Optional): Custom base URL for the OpenAI API (defaults to `https://api.openai.com`).
    *   `OPENAI_API_MODEL` (Optional): The specific OpenAI model to use (defaults to `gpt-3.5-turbo`).
    *   `GITHUB_REPOSITORY_OWNER`: The owner (user or organization) of the GitHub repository to manage.
    *   `GITHUB_REPOSITORY_NAME`: The name of the GitHub repository to manage.
    *   `GITHUB_REPOSITORY_ISSUES_BRANCH` (Optional): The branch name where issues-related changes are managed (defaults to `issues`).
*   **GitHub Repository Data**: Commits, files, issues, comments, and pull requests from the specified repository.
*   **Web Dashboard Requests (HTTP)**: User requests to `http://0.0.0.0:5005` for monitoring data.

#### Outputs

*   **GitHub Actions (HTTP)**: Creates, updates, closes, labels, and comments on issues in the designated GitHub repository.
*   **Command Line (stdout/stderr)**: Logs operational details, LLM interactions, and command outputs.
*   **Web Dashboard (HTTP)**: Serves an HTML dashboard and JSON API endpoints (`/api/actions`, `/api/llm-calls`) providing historical logs of agent activities and LLM calls.