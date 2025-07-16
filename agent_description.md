### Pristine Agent

The Pristine Agent is an autonomous AI agent designed to manage GitHub repository issues. It continuously monitors a designated GitHub repository for new commits and issue updates, leveraging a Large Language Model (LLM) to analyze the repository's state and determine necessary actions.

**Main Functions:**

*   **Intelligent Issue Management**: Automatically identifies and addresses issues related to documentation gaps, potential bugs, and missing unit tests within the codebase. It creates small, actionable, and focused issues with appropriate labels.
*   **LLM-Driven Reasoning**: Utilizes an LLM to "think" and generate a sequence of actions based on the current repository context, past events, and its internal memories.
*   **Repository Interaction**: Clones and keeps the target GitHub repository up-to-date, allowing it to list files, read file content, and execute shell commands within the repository's directory.
*   **GitHub Issue Operations**: Performs a wide range of GitHub issue management tasks, including:
    *   Creating new issues with specified titles, bodies, and labels.
    *   Retrieving details of existing issues.
    *   Adding and removing labels from issues.
    *   Closing issues.
    *   Commenting on issues.
    *   Editing issue titles and bodies.
*   **Contextual Memory**: Stores and updates key-value pair memories to maintain context and continuity across its operations, helping it remember important information, code snippets, or user preferences.
*   **Event-Driven Operation**: Reacts to external events such as new commits or changes in GitHub issues, prompting it to re-evaluate its state and decide on new actions.
*   **Monitoring Dashboard**: Provides a local web server with a dashboard to visualize its action history and LLM call logs in real-time.

**Inputs:**

*   **GitHub Repository (via HTTP/Git)**: Monitored for new commits, new issues, updated issues, and new comments on issues.
*   **Environment Variables**: Configuration details for GitHub API access and OpenAI API access.

**Outputs:**

*   **GitHub Issues (via HTTP)**: Creates, updates, comments on, labels, and closes issues.
*   **Command Line (stdout/stderr)**: Logs of the agent's thinking process, action execution, and any errors.
*   **Web Dashboard (HTTP)**: A real-time monitoring interface accessible via a web browser on port `5005`.

**Configuration:**

The agent requires the following environment variables:

*   `GITHUB_PERSONAL_ACCESS_TOKEN`: Your GitHub Personal Access Token for authentication.
*   `OPENAI_API_KEY`: Your OpenAI API key for LLM inference.
*   `GITHUB_REPOSITORY_OWNER`: The owner (username or organization) of the GitHub repository.
*   `GITHUB_REPOSITORY_NAME`: The name of the GitHub repository to manage.
*   `OPENAI_API_BASE` (Optional): The base URL for the OpenAI API (defaults to `https://api.openai.com`).
*   `OPENAI_API_MODEL` (Optional): The LLM model to use (defaults to `gpt-3.5-turbo`).
*   `GITHUB_REPOSITORY_ISSUES_BRANCH` (Optional): The branch used for managing issues (defaults to `issues`).

The agent can also load these variables from a `.env` file.