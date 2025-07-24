### Agent Name
Pristine

### Description
Pristine is an autonomous AI agent designed to manage and maintain GitHub repository issues. It continuously monitors a specified GitHub repository for new commits and issue updates, leveraging a Large Language Model (LLM) to analyze the repository state and decide on appropriate actions. Its primary goal is to help maintain the quality of the codebase by identifying and creating issues for missing documentation, bugs, and absent unit tests, as well as managing existing issues by responding to comments, applying labels, and closing resolved tasks. Pristine also provides a web-based dashboard for real-time monitoring of its operations.

### Key Functions
*   **GitHub Issue Management**: Creates, retrieves, updates, labels, comments on, and closes GitHub issues.
*   **Repository Interaction**: Clones and pulls the target GitHub repository, lists and reads files, and executes shell commands within the repository's context.
*   **Contextual Reasoning**: Utilizes an LLM to process its current state, including internal memories, past actions, repository events, and open issues, to generate thoughts and decide on a sequence of actions.
*   **Event Monitoring**: Automatically detects new commits and changes in GitHub issues (e.g., new issues, updates to existing issues).
*   **Internal Memory Management**: Stores and retrieves key-value pairs to maintain long-term context and continuity in its operations.
*   **Operational Visibility**: Offers a web dashboard to display a detailed log of all executed actions and LLM interactions.

### Inputs
*   **Environment Variables**:
    *   `GITHUB_PERSONAL_ACCESS_TOKEN`: For GitHub API authentication.
    *   `OPENAI_API_KEY`: For OpenAI LLM API authentication.
    *   `OPENAI_API_BASE` (Optional): Base URL for the OpenAI API (defaults to `https://api.openai.com`).
    *   `OPENAI_API_MODEL` (Optional): LLM model name (defaults to `gpt-3.5-turbo`).
    *   `GITHUB_REPOSITORY_OWNER`: Owner of the target GitHub repository.
    *   `GITHUB_REPOSITORY_NAME`: Name of the target GitHub repository.
    *   `GITHUB_REPOSITORY_ISSUES_BRANCH` (Optional): The Git branch for issue-related operations (defaults to `issues`).
*   **GitHub Repository**: The agent reads the repository's file structure, content, commit history, and issue data.
*   **LLM Responses**: Receives structured text (JSON) containing thoughts and a list of actions from the LLM.

### Outputs
*   **GitHub API Actions**: Modifies the target GitHub repository by creating/updating issues, adding/removing labels, and posting comments.
*   **Local Repository Operations**: Performs file system operations (listing, reading) and executes shell commands within its cloned repository.
*   **LLM Inferences**: Sends prompts to the configured LLM to generate responses.
*   **Internal Memory**: Stores and updates key-value pairs within its operational context.
*   **Web Dashboard (HTTP)**: Provides a monitoring dashboard accessible via HTTP on port 5005, displaying action and LLM call logs.