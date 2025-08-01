### Agent Name
Pristine

### Overview
Pristine is an AI agent designed to automate and assist with issue management on a GitHub repository. It continuously monitors the repository for new events such as commits, issues, and comments, and uses a Large Language Model (LLM) to analyze the context and decide on appropriate actions. Its primary goal is to maintain a clean and actionable issue backlog by identifying, creating, updating, and closing issues related to documentation, bugs, and missing tests.

### Key Capabilities
*   **Issue Lifecycle Management**: Creates new issues with clear titles, bodies, and relevant labels; updates existing issues based on new information or events; and closes issues when they are resolved or no longer relevant.
*   **Code Quality Monitoring**: Automatically detects potential issues in the codebase, including:
    *   Missing, outdated, or incomplete documentation.
    *   Bugs or logic flaws.
    *   Missing unit tests.
*   **Contextual Awareness**: Utilizes a memory system to store and recall important information, such as code snippets, issue summaries, and user preferences, ensuring continuity in its operations.
*   **Human Interaction**: Capable of answering human comments on issues and flagging issues that require human input.
*   **Prioritization & Labeling**: Prioritizes issues (P0, P1, P2) and applies various labels (`documentation`, `bug`, `enhancement`, `test`, `needs-human-input`, etc.) for effective categorization.
*   **Repository Interaction**: Can list and read files within the repository and execute shell commands to gather information.
*   **Monitoring Dashboard**: Provides a web-based dashboard to visualize the agent's action history and LLM call logs.

### Inputs
*   **GitHub Repository Events**: Monitors for new commits, new issues, issue updates, and new comments on issues.
*   **Environment Variables**: Configured via environment variables for GitHub authentication and OpenAI API access.
*   **Human Input**: Indirectly receives input through comments on GitHub issues.

### Outputs
*   **GitHub Actions**:
    *   Creates new issues.
    *   Retrieves issue details.
    *   Adds or removes labels from issues.
    *   Closes issues.
    *   Comments on issues.
    *   Edits issue titles and bodies.
*   **Repository Operations**:
    *   Clones and pulls the target GitHub repository.
    *   Lists files and reads file content from the repository.
    *   Executes arbitrary shell commands within the cloned repository.
*   **LLM Inferences**: Sends system and user prompts to an OpenAI-compatible LLM and processes its responses.
*   **Web Dashboard (HTTP on port 5005)**: Provides a real-time monitoring interface for the agent's activities.
*   **Console Logs**: Outputs detailed logs of its operations, thoughts, and actions.

### Configuration
The agent requires the following environment variables for operation:
*   `GITHUB_PERSONAL_ACCESS_TOKEN`: Your GitHub Personal Access Token for repository access.
*   `OPENAI_API_KEY`: Your OpenAI API key. `OPENAI_KEY` is also checked as a fallback.
*   `OPENAI_API_BASE`: (Optional) The base URL for the OpenAI API (defaults to `https://api.openai.com`).
*   `OPENAI_API_MODEL`: (Optional) The OpenAI model to use for inference (defaults to `gpt-3.5-turbo`).
*   `GITHUB_REPOSITORY_OWNER`: The owner (user or organization) of the GitHub repository.
*   `GITHUB_REPOSITORY_NAME`: The name of the GitHub repository.
*   `GITHUB_REPOSITORY_ISSUES_BRANCH`: (Optional) The branch used for issue management (defaults to `issues`).