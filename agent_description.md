### Pristine Agent

The Pristine Agent is an autonomous AI agent designed to manage GitHub issues within a specified repository. It continuously monitors the repository for new events (like commits or issue updates), uses a Large Language Model (LLM) to "think" and decide on appropriate actions, and then executes those actions to maintain the repository's issue state.

**Key Capabilities:**

*   **GitHub Issue Management:**
    *   Detects new documentation issues, bugs, and missing unit tests.
    *   Creates, reads, updates (title, body, comments), and closes GitHub issues.
    *   Manages issue labels (add/remove) for categorization and prioritization (e.g., `documentation`, `bug`, `enhancement`, `test`, `p0`, `p1`, `p2`).
    *   Answers human comments on issues and maps TODOs in code to new issues.
*   **Repository Interaction:**
    *   Clones and pulls the latest changes from the target GitHub repository.
    *   Lists and reads files within the repository.
    *   Executes shell commands within the repository's context.
*   **Intelligent Decision Making:**
    *   Utilizes a configured LLM (e.g., OpenAI GPT models) to analyze the current context (including past events, open issues, and internal memories) and determine the next best actions.
    *   Maintains an internal memory to store important information and ensure continuity.
*   **Autonomous Operation:**
    *   Operates in a continuous loop, checking for new events, thinking, and acting.
    *   Can mark itself as "complete" to pause inference until new external events occur.

**Operation:**

The agent operates by:
1.  **Initializing:** Cloning the target GitHub repository and fetching existing issues.
2.  **Event Monitoring:** Periodically checking for new commits and updates to GitHub issues.
3.  **Thinking:** Constructing a detailed prompt based on its internal state (memories, open issues, recent events, last action output, last thought) and sending it to the LLM to generate a thought process and a list of actions in JSON format.
4.  **Acting:** Executing the actions decided by the LLM, which can include interacting with GitHub, reading repository files, running commands, or managing its internal context.
5.  **Looping:** Repeating the process, incorporating the outcomes of its last actions into the next thinking phase.

**Monitoring:**

The agent provides a built-in web-based dashboard for real-time monitoring of its activities.
*   **Dashboard:** Accessible via HTTP on port `5005` (e.g., `http://localhost:5005`).
*   **Logs:** Displays a history of executed actions and LLM calls, including timestamps, durations, and detailed inputs/outputs.

**Inputs:**

*   **Environment Variables:**
    *   `GITHUB_PERSONAL_ACCESS_TOKEN` (required): GitHub token for API authentication.
    *   `OPENAI_API_KEY` (required): API key for the Large Language Model.
    *   `OPENAI_API_BASE` (optional): Base URL for the LLM API (defaults to `https://api.openai.com`).
    *   `OPENAI_API_MODEL` (optional): Name of the LLM model to use (defaults to `gpt-3.5-turbo`).
    *   `GITHUB_REPOSITORY_OWNER` (required): Owner (user or organization) of the target GitHub repository.
    *   `GITHUB_REPOSITORY_NAME` (required): Name of the target GitHub repository.
    *   `GITHUB_REPOSITORY_ISSUES_BRANCH` (optional): Git branch to operate on (defaults to `issues`).

**Outputs:**

*   **GitHub:** New issues, updated issues, comments on issues, label changes.
*   **Local Repository:** Outputs from executed shell commands, content of read files.
*   **Web (HTTP):** Monitoring dashboard on port `5005`.