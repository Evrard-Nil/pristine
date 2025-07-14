### Agent Name
Pristine

### Description
Pristine is an AI agent designed to automate and assist with GitHub repository issue management. It continuously monitors a specified GitHub repository for new commits, issues, and comments. Utilizing a Large Language Model (LLM), Pristine analyzes the repository context to identify problems such as missing documentation, bugs, or inadequate tests, and then manages issues accordingly. It can create, update, label, comment on, and close issues, and also interacts with the local repository by listing/reading files and executing commands. Pristine provides a web-based dashboard for real-time monitoring of its operations.

### Functions
*   **GitHub Issue Management**: Creates new issues, retrieves details of existing issues, adds or removes labels, posts comments, and closes issues. It aims to ensure issues are small, actionable, and focused on single tasks.
*   **Repository Interaction**: Clones the target GitHub repository locally to analyze its structure and content. It can list all files, read the content of specific files, and execute shell commands within the cloned repository.
*   **LLM-Powered Decision Making**: Uses a Large Language Model (LLM) to process repository context, past events, and stored memories to generate "thoughts" and determine the next set of actions. Includes robust retry mechanisms for LLM calls.
*   **Context and Memory Management**: Maintains an internal context, including a system for storing and retrieving "memories" (key-value pairs) to ensure continuity and long-term understanding of the project and user preferences.
*   **Self-Monitoring and Dashboard**: Logs all actions taken and LLM calls made by the agent. This data is accessible through a real-time web dashboard, providing transparency into the agent's operations.
*   **Continuous Operation**: Operates in a perpetual loop, regularly pulling the latest repository changes, checking for new events (e.g., commits, issue updates), and executing actions. It can be explicitly marked as complete to pause inference until new external events occur.

### Inputs
*   **Environment Variables**: Configured through environment variables for secure access to GitHub (`GITHUB_PERSONAL_ACCESS_TOKEN`, `GITHUB_REPOSITORY_OWNER`, `GITHUB_REPOSITORY_NAME`, `GITHUB_REPOSITORY_ISSUES_BRANCH`) and OpenAI APIs (`OPENAI_API_KEY`, `OPENAI_API_BASE`, `OPENAI_API_MODEL`).
*   **GitHub Repository Events**: Monitors for new commits, newly created issues, updates to existing issues, and new comments on issues or pull requests.

### Outputs
*   **GitHub Actions**: Creates, updates, labels, comments on, and closes issues directly on the configured GitHub repository.
*   **Local Repository Operations**: Performs file system read operations and executes shell commands within the locally cloned repository.
*   **LLM Interactions**: Sends prompts to and receives generated text responses from the OpenAI API.
*   **Web Dashboard (HTTP)**: Serves a monitoring dashboard on port 5005, providing a user interface to view action and LLM call logs.