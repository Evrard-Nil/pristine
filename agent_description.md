### Pristine Agent

The Pristine Agent is an AI-powered GitHub Issue Management agent designed to autonomously manage and maintain the state of issues within a specified GitHub repository. It acts as a vigilant assistant, continuously monitoring the repository for changes and proactively creating, updating, or closing issues as needed.

**Key Features:**

*   **Issue Management:** Creates new issues for detected problems (e.g., missing documentation, bugs, logic flaws, missing unit tests), updates existing issues, adds/removes labels, comments on issues, and closes resolved issues.
*   **Repository Monitoring:** Periodically pulls the latest changes from the configured GitHub repository and detects new commits, new issues, or updates to existing issues (including comments and state changes).
*   **Contextual Awareness:** Maintains an internal memory to store important information, enabling it to remember past actions, project details, and user preferences for long-term continuity.
*   **LLM-Powered Reasoning:** Utilizes a Large Language Model (LLM) to analyze the current repository state, past events, and internal memories to decide on the most appropriate actions to take.
*   **Command Execution:** Can execute shell commands within the cloned repository to gather information or perform necessary operations.
*   **File System Interaction:** Able to list all files in the repository and read the content of specific files.
*   **Self-Completion:** Can mark its current task as complete, pausing further LLM inference until new external events trigger a re-evaluation.
*   **Monitoring Dashboard:** Provides a web-based dashboard to visualize the agent's action history and LLM call logs, offering transparency into its operations.

**Inputs:**

*   **Environment Variables:**
    *   `GITHUB_PERSONAL_ACCESS_TOKEN` (Mandatory): For GitHub API authentication.
    *   `OPENAI_API_KEY` (Mandatory): For LLM API authentication.
    *   `OPENAI_API_BASE` (Optional): Base URL for the OpenAI API (default: `https://api.openai.com`).
    *   `OPENAI_API_MODEL` (Optional): LLM model name (default: `gpt-3.5-turbo`).
    *   `GITHUB_REPOSITORY_OWNER` (Mandatory): Owner of the target GitHub repository.
    *   `GITHUB_REPOSITORY_NAME` (Mandatory): Name of the target GitHub repository.
    *   `GITHUB_REPOSITORY_ISSUES_BRANCH` (Optional): Branch for issue-related operations (default: `issues`).
*   **GitHub Repository Events:** New commits, new issues, issue comments, and issue updates on the configured repository.

**Outputs:**

*   **GitHub API Actions:** (HTTP) Creates, updates, comments on, labels, and closes issues.
*   **Local Repository Operations:** (File System) Clones and pulls the target GitHub repository, reads files, and executes commands.
*   **LLM Inference:** (HTTP) Sends prompts to and receives responses from the configured LLM.
*   **Console Logs:** (stdout) Detailed logs of agent's thought process, actions taken, and their results.
*   **Web Dashboard:** (HTTP on port 5005) Provides a real-time monitoring interface for action and LLM call history.