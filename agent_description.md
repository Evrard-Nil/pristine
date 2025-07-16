### Agent Name
Pristine

### Overview
Pristine is an AI agent designed to autonomously manage GitHub issues within a specified repository. It continuously monitors the repository for new commits, issues, and comments, and uses a Large Language Model (LLM) to "think" and decide on appropriate actions. Its primary goal is to maintain the health of the repository's issues by identifying and creating issues for missing documentation, bugs, and missing tests, as well as updating and closing existing issues.

### Key Capabilities
*   **GitHub Issue Management**: Creates, retrieves, updates (title, body, labels), comments on, and closes GitHub issues.
*   **Repository Interaction**: Clones the target GitHub repository into a temporary directory, pulls the latest changes, detects new commits, lists files, reads file content, and executes shell commands within the repository.
*   **LLM-Powered Decision Making**: Utilizes an external LLM to process its current context (including memories, open issues, and recent events) and generate a sequence of actions to take.
*   **Contextual Memory**: Stores and retrieves key-value pair "memories" to maintain long-term context and continuity across operations.
*   **Event-Driven Operation**: Can mark itself as complete and pause active inference until new external events (e.g., new commits, issue updates) are detected.
*   **Monitoring Dashboard**: Provides a real-time web-based dashboard (accessible on port 5005) to monitor its actions and LLM interactions.

### Operation
Pristine operates in a continuous loop:
1.  **Event Check**: It first checks for new commits in the cloned repository and new/updated issues or comments on GitHub.
2.  **Thinking Phase**: If new events are detected or the agent is not marked complete, it formulates a detailed prompt based on its current context and sends it to the configured LLM. The LLM's response dictates the agent's next actions.
3.  **Acting Phase**: It executes the actions decided by the LLM, which can involve interacting with GitHub, the local repository, or its internal memory.
4.  **Monitoring**: All actions and LLM calls are logged and viewable via the web dashboard.
5.  **Loop/Sleep**: After acting, it sleeps for a short duration before repeating the process. If marked complete, it will only react to new external events.

### Inputs
*   **Environment Variables**: Configuration parameters for GitHub authentication, OpenAI API, and repository details.
*   **GitHub Repository**: Changes in commits, issues, and comments are detected by polling the specified GitHub repository.

### Outputs
*   **GitHub Actions**: Creates, updates, comments on, and closes GitHub issues.
*   **Local Repository Interactions**: Executes shell commands and reads file contents, with outputs used internally for decision-making.
*   **Internal Memory**: Stores and updates key-value pairs.
*   **Console Output (stdout)**: Provides real-time logs, thoughts, and action results.
*   **Web Dashboard (HTTP on port 5005)**: Serves a web interface displaying historical logs of actions taken and LLM calls made.

### Configuration
Pristine requires the following environment variables for operation:
*   `GITHUB_PERSONAL_ACCESS_TOKEN`: Your GitHub Personal Access Token for API authentication.
*   `OPENAI_API_KEY`: Your OpenAI API key for LLM inference. An alternative `OPENAI_KEY` is also checked.
*   `OPENAI_API_BASE`: (Optional) The base URL for the OpenAI API (defaults to `https://api.openai.com`).
*   `OPENAI_API_MODEL`: (Optional) The specific OpenAI model to use (defaults to `gpt-3.5-turbo`).
*   `GITHUB_REPOSITORY_OWNER`: The owner (username or organization) of the GitHub repository.
*   `GITHUB_REPOSITORY_NAME`: The name of the GitHub repository.
*   `GITHUB_REPOSITORY_ISSUES_BRANCH`: (Optional) The branch used for issue management (defaults to `issues`).
*   `OPENAI_ORG_ID`: (Optional) OpenAI organization ID, if required by your OpenAI setup.