### Agent Name
Pristine

### Description
Pristine is an AI agent designed to automate and assist with issue management on GitHub repositories. It continuously monitors a specified repository for new commits, issues, comments, and pull requests. Leveraging a Large Language Model (LLM), Pristine analyzes the repository's codebase, documentation, and existing issues to identify and manage tasks such as detecting missing documentation, bugs, and unit tests. It can create, update, label, comment on, and close GitHub issues, aiming to keep the issue tracker organized and actionable. Pristine also maintains an internal memory to ensure continuity and context in its operations. For transparency and monitoring, it hosts a local web dashboard displaying its action history and LLM interactions.

### Capabilities
*   **GitHub Issue Management**:
    *   Creates new issues with specified titles, bodies, and labels.
    *   Retrieves details of specific issues, including comments and labels.
    *   Adds or removes labels from issues.
    *   Closes issues.
    *   Adds comments to issues.
    *   Edits the title or body of existing issues.
    *   Lists all open or closed issues in the repository.
*   **Repository Interaction**:
    *   Clones and pulls the latest changes from the configured GitHub repository.
    *   Detects new commits and updates in the repository.
    *   Lists all files within the repository.
    *   Reads the content of specific files.
    *   Executes shell commands within the cloned repository's directory.
*   **AI-Powered Decision Making**:
    *   Utilizes a Large Language Model (LLM) for inference, generating thoughts and deciding on actions based on the current context and events.
    *   Detects and creates issues for missing documentation, bugs, and missing unit tests.
    *   Answers human comments on issues.
    *   Maps TODOs in code to issues.
    *   Prioritizes issues and manages labels.
*   **Internal Context Management**:
    *   Stores and updates key-value pair memories within its context for long-term retention of important information (e.g., code snippets, user preferences, issue summaries).
    *   Removes irrelevant memories from its context.
*   **Monitoring and Observability**:
    *   Hosts a web dashboard to display a log of all actions taken and LLM calls made, including timestamps, durations, and results.
*   **Flow Control**:
    *   Can pause its execution for a specified duration (`Sleep` action).
    *   Can mark its current task as complete, preventing further inference until a new external event occurs (`MarkComplete` action).

### Inputs
*   **Environment Variables**: Configures GitHub authentication, OpenAI API details, and repository specifics.
*   **GitHub Repository Events**: New commits, new issues, issue updates (title, body, state, comments), new pull requests.
*   **Human Input (indirect)**: Comments on GitHub issues, which the agent can read and respond to.

### Outputs
*   **GitHub**: New issues, updated issues, comments on issues, label changes, closed issues.
*   **Web Dashboard (HTTP)**: Provides a real-time view of agent actions and LLM interactions on `http://0.0.0.0:5005`.
*   **Stdout/Stderr**: Logs its operations and any errors encountered during execution.

### Configuration (Environment Variables)
*   `GITHUB_PERSONAL_ACCESS_TOKEN`: Required for GitHub API authentication.
*   `OPENAI_API_KEY`: Required for OpenAI API authentication.
*   `OPENAI_API_BASE`: (Optional) Base URL for the OpenAI API (default: `https://api.openai.com`).
*   `OPENAI_API_MODEL`: (Optional) Specifies the OpenAI model for LLM inference (default: `gpt-3.5-turbo`).
*   `OPENAI_KEY`: (Optional) Alternative OpenAI API key if `OPENAI_API_KEY` is not set.
*   `GITHUB_REPOSITORY_OWNER`: Required owner of the GitHub repository.
*   `GITHUB_REPOSITORY_NAME`: Required name of the GitHub repository.
*   `GITHUB_REPOSITORY_ISSUES_BRANCH`: (Optional) The branch where the agent manages issues (default: `issues`).