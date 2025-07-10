### Agent Name
Pristine

### Description
Pristine is an AI agent designed to automate and streamline issue management on GitHub repositories. It continuously monitors a specified repository for changes, analyzes the codebase and existing issues, and leverages a Large Language Model (LLM) to intelligently create, update, and close issues. Its primary goal is to help maintain the state of issues by detecting and addressing various problems such as missing documentation, bugs, logic flaws, and absent unit tests.

### Key Features
*   **Intelligent Issue Management:** Identifies and creates detailed, actionable issues for documentation gaps, code bugs, and missing tests. It also manages issue labels and priorities (P0, P1, P2).
*   **GitHub Integration:** Seamlessly interacts with GitHub to clone repositories, fetch/create/update issues, add comments, and manage labels.
*   **Codebase Awareness:** Can list and read files from the repository and execute shell commands to understand the project context.
*   **Contextual Reasoning:** Maintains an internal memory and tracks past events and actions to make informed decisions and ensure continuity in its operations.
*   **Event-Driven Operation:** Reacts to new commits, issue updates, and comments on the repository, triggering new rounds of analysis and action.
*   **Monitoring Dashboard:** Provides a web-based interface (accessible via HTTP on port 5005) to view the agent's action history and LLM interactions in real-time.

### Capabilities (Actions)
Pristine can perform a range of actions, including:
*   **Run LLM Inference:** Generates text based on provided system and user prompts.
*   **Repository Interaction:** Lists all files, reads specific files, and runs shell commands within the cloned repository.
*   **Context Management:** Stores, updates, and removes key-value memories in its internal context.
*   **GitHub Issue Management:**
    *   Creates new issues with specified titles, bodies, and labels.
    *   Retrieves details of specific issues.
    *   Adds or removes labels from issues.
    *   Closes issues.
    *   Comments on issues.
    *   Edits the title or body of existing issues.
*   **Control Flow:** Pauses its operation for a specified duration or marks its current task as complete, awaiting new external events.

### Inputs
*   **Medium:** GitHub Webhooks (implicitly through polling the repository), Environment Variables.
*   **Details:**
    *   New commits pushed to the monitored GitHub repository.
    *   New or updated issues (including new comments) on the GitHub repository.
    *   Human comments on existing issues, which the agent can respond to.

### Outputs
*   **Medium:** GitHub API, Local File System, HTTP, stdout.
*   **Details:**
    *   **GitHub Actions:** Creates new issues, updates existing issues (title, body, state), adds comments, and manages labels on the specified GitHub repository.
    *   **Local Repository:** Maintains a local clone of the GitHub repository, pulling the latest changes.
    *   **Monitoring Dashboard (HTTP):** Provides a web interface on port 5005 displaying a history of agent actions and LLM calls.
    *   **Console Output (stdout):** Logs agent activity, thoughts, and action outputs.

### Environment Variables
The agent requires the following environment variables for configuration and authentication:
*   `GITHUB_PERSONAL_ACCESS_TOKEN`: Your GitHub Personal Access Token for repository access.
*   `OPENAI_API_KEY`: Your OpenAI API key for LLM inference (or `OPENAI_KEY` can be set as an alternative).
*   `OPENAI_API_BASE` (Optional): Custom base URL for the OpenAI API (defaults to `https://api.openai.com`).
*   `OPENAI_API_MODEL` (Optional): The OpenAI model to use for inference (defaults to `gpt-3.5-turbo`).
*   `GITHUB_REPOSITORY_OWNER`: The owner (user or organization) of the target GitHub repository.
*   `GITHUB_REPOSITORY_NAME`: The name of the target GitHub repository.
*   `GITHUB_REPOSITORY_ISSUES_BRANCH` (Optional): The branch in the GitHub repository that the agent will use for managing issues (defaults to `issues`).