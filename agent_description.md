### Agent Name
Pristine

### Description
Pristine is an AI agent designed to autonomously manage issues within a GitHub repository. It continuously monitors the repository for new events such as commits and issue updates, using an integrated Large Language Model (LLM) to analyze the current state and decide on appropriate actions.

The agent's primary responsibilities include:
*   **Issue Management**: Creating, updating, labeling, commenting on, and closing GitHub issues. It aims to detect and create issues for missing documentation, bugs, and insufficient unit tests.
*   **Codebase Interaction**: Cloning and keeping a local copy of the repository, listing files, reading file contents, and executing shell commands within the repository context.
*   **Contextual Memory**: Storing and retrieving key-value pair "memories" to maintain long-term context and continuity across its operations.
*   **Event-Driven Operation**: Reacting to new commits and changes in GitHub issues (e.g., new comments, state changes) to trigger new thinking and action cycles.
*   **Self-Completion**: The agent can mark its current task as complete, pausing further LLM inference until a new external event occurs.

Pristine also provides a web-based dashboard for real-time monitoring of its actions and LLM interactions.

### Capabilities
*   Manages GitHub issues (create, get, update, label, comment, close).
*   Interacts with the local repository (clone, pull, list files, read files, run commands).
*   Utilizes a Large Language Model for decision-making and content generation.
*   Maintains internal memory to retain context over time.
*   Monitors repository activity for new events.
*   Provides a web dashboard to visualize agent activity and LLM calls.

### Inputs
*   **Environment Variables**:
    *   `GITHUB_PERSONAL_ACCESS_TOKEN`: GitHub personal access token for API authentication.
    *   `OPENAI_API_KEY` (or `OPENAI_KEY`): OpenAI API key for LLM access.
    *   `OPENAI_API_BASE` (Optional): Base URL for the OpenAI API (defaults to `https://api.openai.com`).
    *   `OPENAI_API_MODEL` (Optional): Specifies the LLM model to use (defaults to `gpt-3.5-turbo`).
    *   `GITHUB_REPOSITORY_OWNER`: Owner of the GitHub repository.
    *   `GITHUB_REPOSITORY_NAME`: Name of the GitHub repository.
    *   `GITHUB_REPOSITORY_ISSUES_BRANCH` (Optional): Git branch used for issue management (defaults to `issues`).
*   **GitHub Repository Events**: New commits, new issues, updated issues (e.g., new comments, title/body edits, label changes, state changes).
*   **LLM Inference**: System and user prompts provided to the LLM for generating thoughts and actions.

### Outputs
*   **GitHub**: New issues, updated issue titles/bodies, comments on issues, added/removed labels, closed issues.
*   **Local File System**: A cloned copy of the target GitHub repository.
*   **Standard Output (Console)**: Detailed logs of its thinking process, actions taken, and their results.
*   **Web Dashboard (HTTP on port 5005)**: A user interface displaying historical logs of actions and LLM calls.