### Agent Name
Pristine

### Overview
Pristine is an AI agent designed to autonomously manage GitHub repository issues. It continuously monitors a specified repository for new events and changes, leveraging a Large Language Model (LLM) to analyze the context, make informed decisions, and execute actions to maintain and improve the repository's issue tracker.

### Key Capabilities
*   **Intelligent Issue Management:** Automatically identifies, creates, updates, and closes GitHub issues related to documentation, bugs, and missing tests.
*   **Contextual Awareness:** Maintains an internal memory to remember past interactions, code snippets, and user preferences, ensuring continuity in its operations.
*   **Codebase Interaction:** Can list files, read file contents, and execute shell commands within the cloned repository to gather necessary information.
*   **Event-Driven Operation:** Reacts to new commits, new issues, and updates on existing issues (e.g., new comments) to trigger its thinking and action cycles.
*   **LLM-Powered Reasoning:** Utilizes a configurable LLM to process complex information, generate thoughts, and decide on the most appropriate actions.
*   **Monitoring Dashboard:** Provides a real-time web interface to observe the agent's actions and LLM interactions.

### Inputs
*   **Environment Variables:** Configuration parameters for GitHub and LLM API access.
*   **GitHub Repository Events:** New commits, new issues, and updates to existing issues (e.g., new comments, label changes). These are detected automatically by the agent.
*   **Human Interaction (indirect):** The agent processes and responds to comments made by humans on GitHub issues.

### Outputs
*   **GitHub Actions:** Creates, updates, closes, comments on, and manages labels for issues.
*   **Console Output (stdout):** Provides detailed logs of its operations, thoughts, and action results.
*   **Web Dashboard (HTTP on port 5005):** Displays a user-friendly interface with historical logs of all actions taken and LLM calls made by the agent.

### Environment Variables
*   `GITHUB_PERSONAL_ACCESS_TOKEN`: Your GitHub Personal Access Token for repository access. (Required)
*   `OPENAI_API_KEY`: Your OpenAI API key for LLM inference. (Required)
*   `OPENAI_API_BASE`: (Optional) Base URL for the OpenAI API. Defaults to `https://api.openai.com`.
*   `OPENAI_API_MODEL`: (Optional) The LLM model to use. Defaults to `gpt-3.5-turbo`.
*   `GITHUB_REPOSITORY_OWNER`: The owner (user or organization) of the target GitHub repository. (Required)
*   `GITHUB_REPOSITORY_NAME`: The name of the target GitHub repository. (Required)
*   `GITHUB_REPOSITORY_ISSUES_BRANCH`: (Optional) The Git branch to use for issue management. Defaults to `issues`.
*   `OPENAI_KEY`: (Optional) An alternative environment variable for `OPENAI_API_KEY` if the primary is not set.