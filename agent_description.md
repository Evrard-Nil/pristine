### Agent Name
Pristine

### Description
Pristine is an AI agent designed to autonomously manage issues within a GitHub repository. Its primary goal is to maintain the health and organization of the repository's issues by detecting problems, creating new issues, updating existing ones, and closing resolved tasks. It leverages a Large Language Model (LLM) for reasoning and decision-making, adapting its behavior based on repository events and its internal memory.

### Functionality
*   **GitHub Issue Management**: Creates, retrieves, updates (title, body), closes, comments on, and adds/removes labels from GitHub issues.
*   **Repository Monitoring**: Continuously monitors the configured GitHub repository for new commits, new issues, and updates to existing issues (including new comments).
*   **LLM-Powered Reasoning**: Utilizes an OpenAI-compatible LLM to analyze the current repository state, past events, and internal memories to formulate thoughts and decide on a sequence of actions.
*   **Contextual Memory**: Stores and retrieves key-value pairs as "memories" to maintain long-term context and continuity across its operations.
*   **File System Interaction**: Can list all files in the cloned repository and read the content of specific files.
*   **Command Execution**: Can execute shell commands within the cloned repository's directory, allowing for dynamic interactions with the codebase (e.g., running linters, tests, or other scripts).
*   **Self-Regulation**: Can pause its inference loop by "marking complete" until new external events (like new commits or issue updates) are detected.
*   **Monitoring Dashboard**: Provides a local web interface (accessible via HTTP on port 5005) to visualize a history of executed actions and LLM calls, including their inputs, outputs, and durations.

### Inputs
*   **Environment Variables**:
    *   `GITHUB_PERSONAL_ACCESS_TOKEN`: For GitHub API authentication.
    *   `OPENAI_API_KEY` (or `OPENAI_KEY`): For OpenAI API authentication.
    *   `OPENAI_API_BASE`: (Optional) Base URL for the OpenAI API (defaults to `https://api.openai.com`).
    *   `OPENAI_API_MODEL`: (Optional) OpenAI model name (defaults to `gpt-3.5-turbo`).
    *   `GITHUB_REPOSITORY_OWNER`: The owner (user or organization) of the target GitHub repository.
    *   `GITHUB_REPOSITORY_NAME`: The name of the target GitHub repository.
    *   `GITHUB_REPOSITORY_ISSUES_BRANCH`: (Optional) The branch to use for issue management (defaults to `issues`).
*   **GitHub Events**: New commits, new issues, issue updates (including comments).
*   **Human Interaction**: Implicitly, human users interact by creating or updating issues and comments on GitHub, which the agent detects and responds to.

### Outputs
*   **GitHub Actions**: Creates new issues, adds comments, updates issue titles/bodies, closes issues, and manages issue labels on the configured GitHub repository.
*   **Console Output**: Logs its operational status, detected events, LLM thoughts, and action results to `stdout`.
*   **Web Dashboard (HTTP)**: Serves a real-time monitoring dashboard showing detailed logs of all actions performed and LLM calls made.