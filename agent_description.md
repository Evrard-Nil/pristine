### Pristine Agent

Pristine is an AI agent designed to automate and streamline GitHub issue management. It continuously monitors a specified GitHub repository for new events, intelligently analyzes code and documentation, and creates, updates, or closes issues as needed.

#### Functionality

*   **GitHub Issue Management**: Creates new issues, updates existing issue titles and bodies, adds or removes labels, and comments on issues. It aims to keep the number of open issues minimal and focused.
*   **Repository Monitoring & Analysis**: Clones and monitors a GitHub repository for new commits, issues, comments, and pull requests. It uses an integrated Large Language Model (LLM) to:
    *   Detect missing, outdated, or incomplete documentation.
    *   Identify bugs or logic flaws in the codebase.
    *   Suggest missing unit tests for functions or modules.
    *   Respond to human comments on issues.
    *   Map TODOs in code to new issues.
*   **Contextual Reasoning**: Maintains an internal memory (context) to remember past actions, events, and important information, ensuring continuity in its operations.
*   **Command Execution**: Capable of executing shell commands within the cloned repository's environment to gather information or perform tasks.
*   **Prioritization & Labeling**: Prioritizes issues (e.g., `p0`, `p1`, `p2`) and applies relevant labels (e.g., `documentation`, `bug`, `enhancement`, `test`, `needs-human-input`) for effective categorization.
*   **Monitoring Dashboard**: Provides a web-based dashboard on port `5005` to visualize a history of the agent's actions and LLM calls.

#### Inputs

*   **Environment Variables**: Configured via environment variables.
*   **GitHub Repository Events**: Automatically triggered by activity in the configured GitHub repository (new commits, issue updates, comments, pull requests).
*   **Human Interaction**: Processes and responds to comments made on GitHub issues.

#### Outputs

*   **GitHub Issues**: Creates new issues, modifies existing ones, adds/removes labels, and posts comments.
*   **LLM Responses**: Generates text responses based on prompts for analysis and decision-making.
*   **Command Line Output**: Displays results from executed shell commands.
*   **Web Dashboard (HTTP)**: Provides a user interface to view detailed logs of agent actions and LLM interactions.

#### Configuration

The agent requires the following environment variables for operation:

*   `GITHUB_PERSONAL_ACCESS_TOKEN` (Required): Your GitHub Personal Access Token for API authentication.
*   `OPENAI_API_KEY` (Required): Your OpenAI API key for LLM inference. `OPENAI_KEY` can be used as an alternative if `OPENAI_API_KEY` is not set.
*   `GITHUB_REPOSITORY_OWNER` (Required): The GitHub username or organization that owns the target repository.
*   `GITHUB_REPOSITORY_NAME` (Required): The name of the GitHub repository the agent will manage.
*   `OPENAI_API_BASE` (Optional): The base URL for the OpenAI API (defaults to `https://api.openai.com`).
*   `OPENAI_API_MODEL` (Optional): The specific LLM model to use (defaults to `gpt-3.5-turbo`).
*   `GITHUB_REPOSITORY_ISSUES_BRANCH` (Optional): The branch in the repository that the agent will primarily use for issue-related operations (defaults to `issues`).