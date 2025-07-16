### Agent Name
Pristine

### Description
Pristine is an autonomous AI agent designed to manage and maintain the state of issues within a specified GitHub repository. It continuously monitors the repository for new events, leverages a Large Language Model (LLM) for decision-making, and executes a range of actions to ensure the repository's issues are well-organized, up-to-date, and actionable.

### Functionality
Pristine's core responsibilities include:
*   **Issue Management:** Automatically creates, updates, and closes GitHub issues. It can add or remove labels, comment on issues, and edit issue titles or bodies.
*   **Event Monitoring:** Detects new commits, new issues, and updates to existing issues (e.g., new comments) by regularly pulling from the GitHub repository.
*   **Intelligent Decision-Making:** Utilizes an LLM to "think" about the current state of the repository and decide on the most appropriate actions. This includes identifying potential documentation gaps, bugs, or missing tests, and creating corresponding issues.
*   **Contextual Memory:** Maintains an internal memory to store and retrieve important information, ensuring continuity and informed decision-making across its operations.
*   **Repository Interaction:** Can list and read files from the cloned GitHub repository, and execute shell commands within the repository's context.
*   **Self-Correction:** Attempts to parse and validate LLM outputs to ensure actions are well-formed.
*   **Monitoring Dashboard:** Provides a web-based dashboard for real-time monitoring of its actions and LLM interactions.

### Inputs
*   **Environment Variables:**
    *   `GITHUB_PERSONAL_ACCESS_TOKEN`: For GitHub API authentication.
    *   `OPENAI_API_KEY` (or `OPENAI_KEY`): For OpenAI LLM API authentication.
    *   `OPENAI_API_BASE`: (Optional) Base URL for the OpenAI API (defaults to `https://api.openai.com`).
    *   `OPENAI_API_MODEL`: (Optional) Specifies the LLM model to use (defaults to `gpt-3.5-turbo`).
    *   `GITHUB_REPOSITORY_OWNER`: The owner (user or organization) of the target GitHub repository.
    *   `GITHUB_REPOSITORY_NAME`: The name of the target GitHub repository.
    *   `GITHUB_REPOSITORY_ISSUES_BRANCH`: (Optional) The Git branch where issues are managed (defaults to `issues`).
*   **GitHub Repository Data:** New commits, issue creation, issue updates (including comments).

### Outputs
*   **GitHub API Calls:** (HTTP) Creates, updates, closes issues; adds/removes labels; posts comments.
*   **Local File System:** (Disk I/O) Clones and pulls updates from the GitHub repository; reads file contents.
*   **Shell Commands:** (stdout/stderr) Executes commands within the repository and captures their output.
*   **LLM Inference Requests:** (HTTP) Sends prompts to the configured LLM and receives text responses.
*   **Console Output:** (stdout) Logs operational messages, thoughts, and action results.
*   **Web Dashboard:** (HTTP) Serves an HTML dashboard and JSON API endpoints for action and LLM call logs, accessible via port 5005.

### Configuration
Pristine is configured via environment variables, allowing users to specify GitHub authentication details, OpenAI API keys and models, and the target GitHub repository.