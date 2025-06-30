### Agent Name
Pristine

### Overview
Pristine is an AI agent designed to autonomously manage GitHub repository issues. It continuously monitors a specified GitHub repository, analyzes its state, and takes actions to maintain and improve the issue tracking process. Its primary goal is to assist in maintaining a clean and actionable issue backlog.

### Key Features
*   **Intelligent Issue Management:** Automatically detects, scopes, creates, updates, and closes issues based on repository activity and code analysis. It can identify new documentation needs, logic flaws, testing improvements, and suggest enhancements.
*   **Codebase Awareness:** Clones the target repository locally and can read and analyze files within it to inform its issue management decisions.
*   **Contextual Memory:** Maintains a dynamic memory of key-value pairs, past actions, known issues, and recent events, enabling long-term coherent and informed operations.
*   **GitHub Integration:** Interacts directly with GitHub using a GitHub App to create, retrieve, update, label, comment on, and close issues. It also tracks new commits and updates to issues.
*   **LLM-Powered Reasoning:** Leverages a Large Language Model (LLM) for "thinking" about the current situation and "deciding" on appropriate actions based on the gathered context.
*   **Real-time Monitoring Dashboard:** Provides a simple web-based dashboard to visualize the agent's action history and LLM interactions, accessible via a web browser.

### Operations
Pristine operates in a continuous loop:
1.  **Event Checking:** Monitors the GitHub repository for new commits, new issues, and updates to existing issues.
2.  **Thinking:** Analyzes the current context, including new events, past actions, and stored memories, to formulate a strategic approach.
3.  **Decision Making:** Selects a sequence of actions to perform based on its thinking process.
4.  **Acting:** Executes chosen actions, which can include:
    *   Reading repository files.
    *   Storing or removing memories in its context.
    *   Interacting with GitHub issues (creating, updating, commenting, adding/removing labels, closing).
    *   Performing additional LLM inferences for specific tasks.
    *   Pausing its operation for a specified duration.

### Inputs
*   **Environment Variables:**
    *   `OPENAI_API_KEY` (Required): Your API key for authenticating with the OpenAI service.
    *   `OPENAI_API_BASE` (Optional): The base URL for the OpenAI API (defaults to `https://api.openai.com`).
    *   `OPENAI_API_MODEL` (Optional): The specific OpenAI model to be used for LLM inference (defaults to `gpt-3.5-turbo`).
    *   `GITHUB_REPOSITORY_OWNER` (Required): The owner (user or organization) of the GitHub repository the agent will manage.
    *   `GITHUB_REPOSITORY_NAME` (Required): The name of the GitHub repository the agent will manage.
    *   `GITHUB_REPOSITORY_ISSUES_BRANCH` (Optional): The name of the Git branch where issues-related changes might be managed or tracked (defaults to `issues`).
*   **GitHub Repository Data:** New commits, existing issues (titles, bodies, labels, comments), and repository file contents.

### Outputs
*   **GitHub Repository Modifications:** Creates, updates, and closes issues; adds/removes labels; posts comments on issues.
*   **Internal State Updates:** Manages its internal memory and context based on observations and actions.
*   **Monitoring Dashboard (HTTP on port 5000):** Provides logs of all executed actions and LLM calls for transparency and debugging.
*   **Console Output (stdout):** Provides real-time logs of agent activities and decisions.