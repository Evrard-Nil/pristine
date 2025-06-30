### Pristine Agent

The Pristine Agent is an AI-powered assistant designed to autonomously manage and maintain GitHub repository issues. It continuously monitors a designated repository for new events, analyzes the current state using a Large Language Model (LLM), and performs actions to keep issues organized, up-to-date, and actionable.

**Key Features:**

*   **GitHub Issue Management**: Creates, updates, closes, comments on, and manages labels for GitHub issues. It aims to detect new documentation issues, scope human-created issues, answer comments, map TODOs to issues, identify logic flaws, suggest test improvements, and propose new features.
*   **Repository Monitoring**: Clones the target GitHub repository and periodically pulls updates to detect new commits and changes. It can list all files and read the content of specific files within the repository.
*   **LLM-Powered Decision Making**: Utilizes an OpenAI-compatible Large Language Model to "think" about the repository's state and "decide" on a sequence of actions to take.
*   **Contextual Memory**: Maintains an internal memory to store and retrieve important information, such as code snippets, issue summaries, or user preferences, ensuring continuity in its operations.
*   **Dynamic Adaptation**: Includes logic to refresh GitHub access tokens and retry LLM calls in case of transient errors, ensuring robust operation.
*   **Monitoring Dashboard**: Provides a web-based dashboard for real-time monitoring of the agent's executed actions and LLM interactions.

**Inputs:**

*   **Environment Variables**:
    *   `OPENAI_API_KEY`: OpenAI API key (required).
    *   `OPENAI_API_BASE`: OpenAI API base URL (defaults to `https://api.openai.com`).
    *   `OPENAI_API_MODEL`: OpenAI model name (defaults to `gpt-3.5-turbo`).
    *   `GITHUB_REPOSITORY_OWNER`: GitHub repository owner (required).
    *   `GITHUB_REPOSITORY_NAME`: GitHub repository name (required).
    *   `GITHUB_REPOSITORY_ISSUES_BRANCH`: Branch for issue management (defaults to `issues`).
*   **GitHub Repository Events**: New commits, new issues, updates to existing issues (comments, state changes, label changes).

**Outputs:**

*   **GitHub Repository**: Creates, updates, and closes issues; adds/removes labels; posts comments.
*   **Internal Logs**: Records of all executed actions and LLM calls for monitoring purposes.
*   **Web Dashboard (HTTP on port 5005)**: Displays action history and LLM call history for user review.