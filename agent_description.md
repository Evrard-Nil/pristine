### Pristine Agent

Pristine is an AI agent designed to manage and maintain GitHub issues within a repository. It acts as an intelligent issue manager, identifying and addressing various aspects of repository health related to documentation, code quality, and testing.

#### Key Capabilities
*   **Issue Management:** Automatically creates, updates, and closes GitHub issues based on observed events and its internal reasoning.
*   **Problem Detection:** Identifies and creates issues for:
    *   Missing, outdated, or incomplete documentation.
    *   Bugs or logic flaws in the codebase.
    *   Missing or inadequate unit tests.
*   **Contextual Reasoning:** Utilizes a Large Language Model (LLM) to analyze the repository state, past actions, and new events to make informed decisions. It maintains "memories" to ensure continuity and context over time.
*   **Event-Driven Operation:** Continuously monitors the target GitHub repository for new commits, issue updates, and new comments, reacting autonomously to changes.
*   **Repository Interaction:** Can list and read files within the repository's local clone and execute shell commands to gather information.
*   **Issue Prioritization & Labeling:** Applies standard labels (`documentation`, `bug`, `enhancement`, `test`, `needs-human-input`, `ready-for-approval`, `ready-for-implementation-by-ai`, `duplicate`, `p0`, `p1`, `p2`) and prioritizes issues.
*   **Monitoring Dashboard:** Provides a web-based dashboard for real-time visibility into the agent's actions and LLM interactions.

#### How it Works
Pristine operates in a continuous loop:
1.  **Event Check:** It pulls the latest changes from the GitHub repository and detects new commits, issues, or updates to existing issues.
2.  **Think:** Using an LLM, it processes the current repository context, its internal memories, and any new events to determine the most appropriate next steps. The LLM outputs a structured list of actions.
3.  **Act:** It executes the decided actions, which can include interacting with GitHub (creating/updating issues, commenting), reading repository files, or running shell commands.
4.  **Monitor:** All actions and LLM calls are logged and viewable via a local web dashboard.

#### Inputs
*   **Environment Variables:**
    *   `GITHUB_PERSONAL_ACCESS_TOKEN`: Required for GitHub API authentication.
    *   `OPENAI_API_KEY` (or `OPENAI_KEY`): Required for LLM API authentication.
    *   `GITHUB_REPOSITORY_OWNER`: The owner of the target GitHub repository.
    *   `GITHUB_REPOSITORY_NAME`: The name of the target GitHub repository.
    *   `OPENAI_API_BASE` (Optional): Custom base URL for the OpenAI API (defaults to `https://api.openai.com`).
    *   `OPENAI_API_MODEL` (Optional): The LLM model to use (defaults to `gpt-3.5-turbo`).
    *   `GITHUB_REPOSITORY_ISSUES_BRANCH` (Optional): The branch used for issue management (defaults to `issues`).
*   **External Events:** New commits, issue creation, issue updates, and comments on the configured GitHub repository.

#### Outputs
*   **GitHub Repository:** Creates, updates, closes, labels, and comments on issues.
*   **Local Repository Clone:** Performs file system operations (listing, reading files) and executes shell commands.
*   **Web Dashboard (HTTP):** Accessible on port `5005`, displaying a history of agent actions and LLM calls.
*   **Console Output:** Logs its operations and decisions to standard output.