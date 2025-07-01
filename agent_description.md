### Agent Name
Pristine

### Main Functions
Pristine is an AI agent designed to autonomously manage and maintain GitHub issues within a specified repository. It operates in a continuous loop, performing the following core functions:

*   **Event Monitoring**: Continuously checks the designated GitHub repository for new commits, new issues, and updates to existing issues (including new comments). It pulls the latest repository changes to detect new commits.
*   **Cognitive Processing**: Utilizes a Large Language Model (LLM) to:
    *   **Think**: Analyze the current state, including internal memories, open/closed issues, and recent events, to formulate a strategic thought process.
    *   **Decide**: Based on its thoughts and the current context, determine a sequence of actions to execute.
*   **Action Execution**: Performs a variety of actions, including:
    *   **Repository Interaction**: Lists all files or reads the content of specific files within the cloned repository.
    *   **Context Management**: Stores, updates, or removes key-value memories to maintain an ongoing understanding of its environment and tasks.
    *   **GitHub Issue Management**: Creates new issues with specified titles, bodies, and labels; retrieves issue details; adds or removes labels; closes issues; adds comments to issues; and edits issue titles or bodies.
    *   **LLM Inference**: Can initiate arbitrary LLM calls with custom system and user prompts for more complex reasoning or content generation.
    *   **Temporal Control**: Pauses its execution for a specified duration.
*   **Monitoring and Logging**: Maintains detailed logs of all executed actions and LLM calls, providing a transparent record of its operations.
*   **Web Dashboard**: Hosts a local web server that provides a real-time dashboard to visualize its action and LLM call history.

### Important Details
*   **Purpose**: Aims to automate issue management, including identifying documentation needs, scoping human-created issues, responding to comments, mapping code TODOs to issues, detecting logic flaws, suggesting test improvements, and managing issue prioritization and labels.
*   **Issue Principles**: Focuses on creating small, actionable, and non-duplicate issues. It prioritizes issues and strives to keep the number of open issues minimal.
*   **Adaptability**: Learns from past interactions and user preferences by storing information in its internal memory.
*   **Robustness**: Includes retry mechanisms for GitHub API calls, especially when encountering authentication issues, by automatically refreshing its GitHub App installation token.

### Inputs
*   **Environment Variables (via `dotenvy` and direct reads)**:
    *   `OPENAI_API_KEY` (required): API key for OpenAI LLM services.
    *   `GITHUB_REPOSITORY_OWNER` (required): GitHub username or organization name of the repository owner.
    *   `GITHUB_REPOSITORY_NAME` (required): Name of the GitHub repository to manage.
    *   `OPENAI_API_BASE` (optional): Base URL for the OpenAI API (defaults to `https://api.openai.com`).
    *   `OPENAI_API_MODEL` (optional): Specific OpenAI model to use (defaults to `gpt-3.5-turbo`).
    *   `GITHUB_REPOSITORY_ISSUES_BRANCH` (optional): The Git branch where issue-related changes are managed (defaults to `issues`).
*   **GitHub Events**: Changes in the target GitHub repository, including new commits, new issues, and updates/comments on existing issues.

### Outputs
*   **GitHub Repository**: Creates, updates, closes, comments on, and manages labels for issues.
*   **Web Dashboard (HTTP)**: Serves a web interface on port `5005` (e.g., `http://0.0.0.0:5005`) displaying logs of agent actions and LLM interactions.
*   **Console (stdout)**: Provides real-time logging of its thought processes, detected events, and action execution.