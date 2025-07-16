### Agent Name
Pristine

### Description
Pristine is an autonomous AI agent designed to manage and maintain GitHub repository issues. It continuously monitors a designated repository for new commits and issue updates, leveraging a Large Language Model (LLM) to analyze the current state and decide on appropriate actions. Its core responsibilities include identifying and creating issues for missing documentation, bugs, and tests, responding to human comments, closing resolved issues, and prioritizing tasks.

Pristine maintains an internal memory to retain context and continuity across its operations. It interacts with the GitHub API to perform issue-related tasks and can also interact with the local repository by listing files, reading content, and executing shell commands. For transparency and monitoring, Pristine provides a web-based dashboard displaying its action history and LLM interactions.

### Key Functions
*   **GitHub Issue Management**: Creates, reads, updates (title, body), comments on, adds/removes labels from, and closes GitHub issues.
*   **Codebase Interaction**: Clones and manages a local Git repository, pulls latest changes, lists files, reads file content, and executes shell commands within the repository.
*   **AI-driven Decision Making**: Uses an LLM to analyze the repository state, past events, and internal memories to formulate thoughts and determine the next set of actions.
*   **Context Management**: Stores and retrieves key-value pair "memories" to retain important information and maintain operational continuity.
*   **Event Monitoring**: Automatically detects new commits and updates to GitHub issues, triggering new rounds of decision-making.
*   **Monitoring Dashboard**: Provides a web interface to view detailed logs of agent actions and LLM calls.

### Inputs
*   **GitHub Repository (HTTP/Git)**: New commits, new issues, and updates to existing issues (e.g., new comments, state changes) from the configured GitHub repository.
*   **Environment Variables**: Configuration parameters such as GitHub authentication tokens, repository details, and LLM API keys.

### Outputs
*   **GitHub (HTTP)**: Creation of new issues, updates to existing issue titles and bodies, addition/removal of labels, and comments on issues.
*   **Local Repository (Filesystem/Shell)**: Results of file listing, file content reads, and shell command executions within the cloned repository.
*   **Web Dashboard (HTTP)**: A live monitoring dashboard accessible on port 5005, displaying logs of all executed actions and LLM calls.
*   **Console (stdout)**: Detailed logs of the agent's thought process, actions taken, and their outputs.