### Pristine Issue Management Agent

Pristine is an AI agent designed to autonomously manage and maintain issues within a specified GitHub repository. It continuously monitors the repository for changes and new events, leveraging a large language model (LLM) to make decisions and execute actions.

**Key Capabilities:**
*   **Issue Management:** Creates, updates, closes, and prioritizes GitHub issues.
*   **Problem Detection:** Identifies and creates issues for missing documentation, outdated code, bugs, logic flaws, and insufficient unit tests.
*   **Contextual Awareness:** Stores and utilizes "memories" to maintain long-term context, including code snippets, issue summaries, and user preferences.
*   **Event Monitoring:** Detects new commits, new issues, and updates to existing issues or comments on the configured repository.
*   **Human Interaction:** Answers human comments on issues and can flag issues requiring human input.
*   **Dashboard:** Provides a local web-based dashboard to monitor the agent's actions and LLM interactions in real-time.

**Inputs:**
*   **GitHub Repository Events:** New commits, new/updated issues, and new comments are pulled from the configured GitHub repository.
*   **Environment Variables:**
    *   `GITHUB_PERSONAL_ACCESS_TOKEN` (Required): GitHub API authentication.
    *   `OPENAI_API_KEY` (Required): OpenAI API authentication.
    *   `OPENAI_API_BASE` (Optional): Custom base URL for OpenAI API (defaults to `https://api.openai.com`).
    *   `OPENAI_API_MODEL` (Optional): Specific OpenAI model to use (defaults to `gpt-3.5-turbo`).
    *   `GITHUB_REPOSITORY_OWNER` (Required): Owner of the target GitHub repository.
    *   `GITHUB_REPOSITORY_NAME` (Required): Name of the target GitHub repository.
    *   `GITHUB_REPOSITORY_ISSUES_BRANCH` (Optional): Branch for issue-related changes (defaults to `issues`).

**Outputs:**
*   **GitHub Actions (HTTP):** Creates, comments on, updates (title, body, labels), and closes issues directly on the GitHub repository.
*   **LLM Inferences (HTTP):** Sends prompts to and receives responses from the configured LLM.
*   **Monitoring Dashboard (HTTP on Port 5005):** Provides a web interface to view the agent's action history and LLM call logs.
*   **Standard Output:** Logs operational details and agent activities.