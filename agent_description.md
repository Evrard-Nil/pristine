### Agent Name
Pristine

### Description
Pristine is an autonomous AI agent designed to manage GitHub repository issues. It continuously monitors a specified repository for new events like commits and issue updates, then utilizes a Large Language Model (LLM) to "think" about the current state and "decide" on appropriate actions. Its core responsibilities include creating, updating, and closing issues, managing labels, and interacting with the repository's codebase by listing and reading files. Pristine also maintains an internal memory to store relevant information and offers a web-based dashboard for real-time monitoring of its activities.

### Key Features
*   **GitHub Issue Management:** Automates the lifecycle of GitHub issues, including creation, updates, labeling, commenting, and closing.
*   **Repository Interaction:** Clones and pulls the target GitHub repository, allowing it to list and read file contents.
*   **Event-Driven Operation:** Detects new commits and changes in GitHub issues to trigger its decision-making process.
*   **AI-Powered Reasoning:** Utilizes a configurable Large Language Model (LLM) for strategic thinking and action planning based on the current repository context.
*   **Contextual Memory:** Stores and retrieves key-value memories to maintain continuity and inform future decisions.
*   **Self-Healing:** Includes retry mechanisms for GitHub and LLM API calls, automatically refreshing authentication tokens when needed.
*   **Monitoring Dashboard:** Provides a web interface to view a history of executed actions and LLM interactions.

### Inputs
*   **Environment Variables:**
    *   `OPENAI_API_KEY` (required): OpenAI API key for LLM inference.
    *   `OPENAI_API_BASE` (optional): Custom base URL for the OpenAI API (defaults to `https://api.openai.com`).
    *   `OPENAI_API_MODEL` (optional): Specifies the LLM model to use (defaults to `gpt-3.5-turbo`).
    *   `GITHUB_REPOSITORY_OWNER` (required): The owner (username or organization) of the GitHub repository.
    *   `GITHUB_REPOSITORY_NAME` (required): The name of the GitHub repository.
    *   `GITHUB_REPOSITORY_ISSUES_BRANCH` (optional): The branch used for managing issues (defaults to `issues`).
*   **GitHub Events:** New commits, new issues, and updates/comments on existing issues.

### Outputs
*   **GitHub API Calls:** Creates, updates, comments on, and closes issues; manages issue labels.
*   **Internal Memory:** Stores and updates contextual information (key-value pairs).
*   **LLM Inferences:** Generated text responses from the Large Language Model.
*   **Console Output (stdout):** Detailed logs of the agent's operations, thoughts, and actions.
*   **Web Dashboard (HTTP on port 5005):** Provides a real-time display of action and LLM call history.