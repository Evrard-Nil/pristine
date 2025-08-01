### Pristine Agent

Pristine is an autonomous AI agent designed to manage GitHub issues within a specified repository. It continuously monitors the repository for changes and uses a Large Language Model (LLM) to analyze the situation, make decisions, and execute actions to maintain and improve the issue tracker.

**Functionality:**

*   **Issue Management**: Creates, retrieves, closes, comments on, and manages labels and content for GitHub issues.
*   **Codebase Interaction**: Can list all files in the repository, read the content of specific files, and execute shell commands within the local repository clone.
*   **Contextual Memory**: Stores and retrieves "memories" (key-value pairs) to maintain long-term context and continuity across operations.
*   **Event Monitoring**: Automatically checks for new commits, new issues, and updates to existing issues in the configured GitHub repository.
*   **Autonomous Decision-Making**: Utilizes an LLM to generate a thought process and decide on a sequence of actions based on the current repository state and detected events.
*   **Self-Completion**: Can mark its current task as complete, pausing active inference until new external events occur.
*   **Monitoring Dashboard**: Provides a web-based dashboard to view the agent's action history and LLM call logs.

**Inputs:**

*   **GitHub Repository**: Reads code, documentation, and existing issues/comments (HTTP/Git).
*   **GitHub Events**: Detects new commits, issues, and issue updates (Git polling, GitHub API).
*   **LLM Responses**: Receives generated text from a Large Language Model (HTTP API).
*   **Configuration**: Environment variables for API keys and repository details (Environment).

**Outputs:**

*   **GitHub Actions**: Creates, updates, comments on, labels, and closes issues on the configured GitHub repository (HTTP API).
*   **Shell Command Results**: Executes commands locally and processes their standard output and error (stdout/stderr).
*   **Internal Memories**: Stores and manages key-value pair data for its internal context.
*   **LLM Requests**: Sends prompts to the configured Large Language Model (HTTP API).
*   **Monitoring Dashboard**: Exposes action and LLM call logs via a web interface (HTTP on port 5005).

**Configuration:**

The agent is configured using the following environment variables:

*   `GITHUB_PERSONAL_ACCESS_TOKEN`: Your GitHub Personal Access Token for repository access.
*   `OPENAI_API_KEY`: Your OpenAI API key for LLM inference.
*   `OPENAI_API_BASE`: (Optional) The base URL for the OpenAI API (defaults to `https://api.openai.com`).
*   `OPENAI_API_MODEL`: (Optional) The OpenAI model to use for LLM inference (defaults to `gpt-3.5-turbo`).
*   `GITHUB_REPOSITORY_OWNER`: The owner (user or organization) of the GitHub repository to manage.
*   `GITHUB_REPOSITORY_NAME`: The name of the GitHub repository to manage.
*   `GITHUB_REPOSITORY_ISSUES_BRANCH`: (Optional) The Git branch where the agent will operate for issues (defaults to `issues`).