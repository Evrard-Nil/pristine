### Pristine Agent

Pristine is an AI agent designed to automate and assist with GitHub issue management. It continuously monitors a specified GitHub repository, identifies potential problems, and creates, updates, or closes issues as needed.

**Main Functions:**

*   **Intelligent Issue Management**: Automatically creates, updates, closes, prioritizes, and manages labels for GitHub issues. It aims to keep issues small, actionable, and focused, avoiding duplicates.
*   **Codebase Analysis**: Detects various issues within the repository's codebase and documentation, including:
    *   Missing, outdated, or incomplete documentation for code elements.
    *   Bugs or logic flaws in the code.
    *   Missing unit tests for functions or modules.
    *   TODO comments in code, mapping them to actionable issues.
*   **Event-Driven Operation**: Reacts to new commits, updates to existing issues, new comments, and new pull requests within the monitored repository.
*   **LLM-Powered Decision Making**: Utilizes a Large Language Model (LLM) to analyze the current context, past actions, and detected events, then "thinks" to decide on the most appropriate next actions.
*   **Contextual Memory**: Maintains an internal memory to store and retrieve important information (e.g., code snippets, issue summaries, user preferences) to ensure continuity and informed decision-making.
*   **Repository Interaction**: Clones the target GitHub repository locally, pulls latest changes, lists files, reads file contents, and can execute shell commands within the cloned repository environment.
*   **Monitoring Dashboard**: Provides a local web interface to visualize the agent's action history and LLM call logs, offering transparency into its operations.

**Inputs:**

*   **Environment Variables**: Configured via environment variables for GitHub authentication and repository details, and OpenAI API access.
    *   `GITHUB_PERSONAL_ACCESS_TOKEN`: Personal Access Token for GitHub API authentication.
    *   `OPENAI_API_KEY`: API key for OpenAI LLM services.
    *   `OPENAI_API_BASE`: (Optional) Base URL for the OpenAI API (defaults to `https://api.openai.com`).
    *   `OPENAI_API_MODEL`: (Optional) Specific LLM model to use (defaults to `gpt-3.5-turbo`).
    *   `GITHUB_REPOSITORY_OWNER`: Owner of the GitHub repository.
    *   `GITHUB_REPOSITORY_NAME`: Name of the GitHub repository.
    *   `GITHUB_REPOSITORY_ISSUES_BRANCH`: (Optional) Git branch for issue-related operations (defaults to `issues`).
*   **GitHub Repository Events**: New commits, issue creation/updates, new comments, and new pull requests from the configured GitHub repository.

**Outputs:**

*   **GitHub API (HTTP)**: Creates new issues, updates existing issues (title, body, labels), adds comments to issues, and closes issues.
*   **LLM Inferences (HTTP)**: Sends prompts to and receives generated text responses from the configured LLM.
*   **Monitoring Dashboard (HTTP)**: Serves a web interface on port `5005` displaying logs of all actions taken and LLM calls made by the agent.
*   **Console Output (stdout)**: Provides detailed operational logs and debugging information.