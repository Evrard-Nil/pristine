### Pristine Agent

Pristine is an AI agent designed to automate and assist with GitHub issue management. It continuously monitors a specified repository for changes and events, leverages a Large Language Model (LLM) to intelligently decide on appropriate actions, and then executes those actions on GitHub.

**Main Functions:**

*   **GitHub Issue Management**: Creates, updates, closes, comments on, and applies labels to GitHub issues. It aims to keep issues small, actionable, and focused.
*   **Codebase Interaction**: Can list and read files within the repository, and execute shell commands, to gather context and identify potential issues.
*   **Intelligent Decision Making**: Utilizes an LLM (OpenAI) to analyze repository context, past actions, and new events to determine necessary issue management tasks. This includes detecting missing documentation, bugs, and incomplete unit tests.
*   **Contextual Memory**: Stores and retrieves "memories" (key-value pairs) to maintain long-term understanding and continuity across operations, remembering past decisions and relevant information.
*   **Event-Driven Operation**: Actively checks for and reacts to new commits, issue updates, comments, and pull requests in the monitored repository.
*   **Monitoring Dashboard**: Provides a local web interface accessible via HTTP to view a real-time history of the agent's actions and LLM interactions.

**Key Responsibilities:**

*   Detecting and creating issues for missing, outdated, or incomplete documentation.
*   Identifying and reporting bugs or logic flaws in the code.
*   Detecting missing unit tests and suggesting specific test cases.
*   Responding to human comments on issues.
*   Mapping TODOs in code to new issues.
*   Closing issues when they are resolved.
*   Prioritizing issues and managing labels (`documentation`, `bug`, `enhancement`, `test`, `needs-human-input`, `ready-for-approval`, `ready-for-implementation-by-ai`, `duplicate`, `p0`, `p1`, `p2`).

**Inputs:**

*   **Environment Variables**:
    *   `GITHUB_PERSONAL_ACCESS_TOKEN`: Required for GitHub API authentication.
    *   `OPENAI_API_KEY` (or `OPENAI_KEY`): Required for OpenAI LLM authentication.
    *   `OPENAI_API_BASE`: (Optional) Base URL for OpenAI API (defaults to `https://api.openai.com`).
    *   `OPENAI_API_MODEL`: (Optional) Default LLM model to use (defaults to `gpt-3.5-turbo`).
    *   `GITHUB_REPOSITORY_OWNER`: Owner of the target GitHub repository.
    *   `GITHUB_REPOSITORY_NAME`: Name of the target GitHub repository.
    *   `GITHUB_REPOSITORY_ISSUES_BRANCH`: (Optional) Git branch for issue-related operations (defaults to `issues`).
*   **GitHub Repository (via Git/API)**: Code changes (commits), new or updated issues, comments on issues, and pull requests.

**Outputs:**

*   **GitHub (via API)**: New issues, updated issue bodies/titles, comments on issues, added/removed labels, closed issues.
*   **Local Web Server (HTTP)**: A dashboard displaying action history and LLM call logs, accessible on port `5005`.
*   **Console (stdout/stderr)**: Logs and debug information from the agent's operations.