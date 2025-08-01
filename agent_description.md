### Pristine Agent

Pristine is an autonomous AI agent designed to manage issues within a specified GitHub repository. It leverages a Large Language Model (LLM) to understand the repository's state, detect problems, and take appropriate actions to maintain and improve issue tracking.

#### Overview
The agent continuously monitors a GitHub repository for new commits and issue updates. Based on these events and its internal context, it uses an LLM to "think" and decide on a series of actions. Its primary goal is to help maintain the quality of the repository by identifying and addressing issues related to documentation, bugs, and missing tests.

#### Key Capabilities
*   **Issue Management:** Creates, updates, comments on, labels, and closes GitHub issues.
*   **Codebase Interaction:** Clones and pulls the target GitHub repository, lists and reads files, and executes shell commands within the repository's context.
*   **Contextual Memory:** Stores and retrieves important information (memories) to maintain continuity and context across its operations.
*   **Event Monitoring:** Detects new commits and changes in GitHub issues (new, updated, commented).
*   **LLM-driven Decisions:** Utilizes an LLM to generate thoughts and decide on a sequence of actions based on the current context and observed events.
*   **Self-Monitoring Dashboard:** Provides a web-based dashboard to view its action history and LLM call logs, accessible via a web browser.

#### Operation Flow
Pristine operates in a continuous loop:
1.  **Event Detection:** It pulls the latest changes from the GitHub repository and checks for new commits, new issues, or updates to existing issues.
2.  **Thinking Phase:** If new events are detected or its previous task is not marked complete, it generates a detailed thought process using an LLM. This process considers its internal memories, known issues, and recent events to formulate a set of actions.
3.  **Action Execution:** It executes the determined actions, which can involve interacting with the GitHub API, managing its internal memory, or running shell commands.
4.  **Loop & Sleep:** After executing actions, it updates its internal state and pauses for a short duration before repeating the cycle. It can also mark itself "complete" to pause active inference until new external events occur.

#### Inputs
*   **Environment Variables:**
    *   `GITHUB_PERSONAL_ACCESS_TOKEN`: Required for authenticating with the GitHub API.
    *   `OPENAI_API_KEY` (or `OPENAI_KEY`): Required for authenticating with the OpenAI API.
    *   `OPENAI_API_BASE`: (Optional) Specifies the base URL for the OpenAI API. Defaults to `https://api.openai.com`.
    *   `OPENAI_API_MODEL`: (Optional) Specifies the OpenAI model to use. Defaults to `gpt-3.5-turbo`.
    *   `GITHUB_REPOSITORY_OWNER`: The owner (user or organization) of the GitHub repository.
    *   `GITHUB_REPOSITORY_NAME`: The name of the GitHub repository.
    *   `GITHUB_REPOSITORY_ISSUES_BRANCH`: (Optional) The branch used for issue-related operations. Defaults to `issues`.
*   **GitHub Repository Events:** New commits, new issues, and updates to existing issues (e.g., new comments, state changes).

#### Outputs
*   **GitHub Repository:** Modifies the repository by creating, updating, labeling, commenting on, and closing issues.
*   **Console Output (stdout):** Provides detailed logs of its internal thoughts, executed actions, and their results.
*   **Web Dashboard (HTTP on port 5005):** A web-based user interface displaying historical logs of all executed actions and LLM calls, providing transparency into the agent's operations.