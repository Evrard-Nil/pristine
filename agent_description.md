### Agent Name
Pristine

### Description
Pristine is an autonomous AI agent designed to manage and maintain issues within a specified GitHub repository. It continuously monitors the repository for new commits and issue updates, leveraging a Large Language Model (LLM) to analyze the codebase and issue tracker. Its core function is to identify and address various repository health aspects, including missing documentation, potential bugs, and absent unit tests, by creating, updating, and closing relevant GitHub issues. Pristine aims to keep the issue tracker organized, actionable, and aligned with the project's state.

### Key Features
*   **Continuous Monitoring**: Automatically pulls the latest repository changes and fetches GitHub issue updates to react to new events.
*   **Intelligent Issue Management**: Utilizes an LLM to analyze the repository context and generate appropriate GitHub issues for documentation gaps, code bugs, and missing tests.
*   **Contextual Memory**: Stores and retrieves key-value pair memories to maintain a persistent context across its operations, ensuring continuity in its decision-making.
*   **GitHub Interaction**: Capable of a wide range of GitHub operations including creating new issues, retrieving issue details, adding/removing labels, commenting, and editing issue titles and bodies.
*   **Repository Interaction**: Can list all files, read specific file contents, and execute shell commands within the cloned repository for in-depth analysis.
*   **LLM Integration**: Interacts with an OpenAI-compatible LLM to guide its "thinking" process, generating a thought process and a list of actions to take.
*   **Monitoring Dashboard**: Provides a web-based dashboard accessible at `http://0.0.0.0:5005` to view real-time logs of agent actions and LLM calls.
*   **Event-Driven Completion**: Can mark its current task as complete and pause active inference until new external events (e.g., new commits, issue changes) are detected.

### Inputs
*   **GitHub Repository**: The agent clones and monitors a specified GitHub repository for code changes and issue activity.
*   **LLM Responses**: The agent's decision-making is driven by text responses generated from a configured Large Language Model.
*   **Environment Variables**: Configuration parameters for GitHub and OpenAI API access, as well as repository details.

### Outputs
*   **GitHub Issues**: Creates new issues, adds comments, manages labels, edits issue details (title, body), and closes issues.
*   **Repository Operations**: Executes shell commands and interacts with the file system within its local clone of the repository.
*   **Internal Memory**: Stores and retrieves contextual information in its internal memory.
*   **Web Dashboard (HTTP)**: Serves a monitoring dashboard displaying logs of its actions and LLM interactions.
*   **Console Logs (stdout)**: Provides detailed operational logs for real-time monitoring and debugging.

### Environment Variables
The agent relies on environment variables for its configuration and API access. These can be loaded from a `.env` file or set directly in the environment.
*   `GITHUB_PERSONAL_ACCESS_TOKEN`: Required for GitHub API authentication.
*   `OPENAI_API_KEY`: Required for OpenAI API authentication.
*   `OPENAI_KEY`: An alternative environment variable for the OpenAI API key.
*   `OPENAI_API_BASE`: (Optional) Specifies the base URL for the OpenAI API (defaults to `https://api.openai.com`).
*   `OPENAI_API_MODEL`: (Optional) Specifies the OpenAI model to use (defaults to `gpt-3.5-turbo`).
*   `GITHUB_REPOSITORY_OWNER`: Required. The owner (username or organization) of the GitHub repository to manage.
*   `GITHUB_REPOSITORY_NAME`: Required. The name of the GitHub repository to manage.
*   `GITHUB_REPOSITORY_ISSUES_BRANCH`: (Optional) The Git branch where the agent will operate for issue-related changes (defaults to `issues`).