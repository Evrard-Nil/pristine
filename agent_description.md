### Agent Name
Pristine

### Description
Pristine is an AI agent designed to autonomously manage and maintain GitHub issues within a specified repository. It leverages large language models (LLMs) to understand the project context, detect potential problems like missing documentation, bugs, or absent unit tests, and create actionable issues. Pristine also monitors repository activity, responds to human interactions on issues, and prioritizes tasks to streamline repository maintenance. It operates by continuously observing the repository for new events (commits, issue updates) and deciding on appropriate actions, such as creating, updating, or closing issues, and managing labels.

### Key Features
*   **Intelligent Issue Management**: Automatically identifies and creates GitHub issues for detected problems (e.g., missing documentation, bugs, missing tests).
*   **Contextual Understanding**: Maintains an internal memory and tracks past events and known issues to inform its decisions and ensure continuity.
*   **GitHub Integration**: Interacts directly with the GitHub API to perform actions like creating, getting, updating, labeling, commenting on, and closing issues. It also clones and monitors the repository for new commits.
*   **LLM-Powered Reasoning**: Utilizes an OpenAI LLM for complex decision-making and text generation based on a detailed system prompt and current context.
*   **Real-time Monitoring Dashboard**: Hosts a local web server (on port 5005) that provides a dashboard to visualize the agent's action history and LLM call logs, offering transparency into its operations.
*   **Robustness**: Includes retry mechanisms for LLM API calls to handle transient failures.

### Inputs
*   **Environment Variables**: Configuration parameters for GitHub and OpenAI API access.
*   **GitHub Repository**: Changes in code (new commits), new issues, updates to existing issues, and comments on issues.

### Outputs
*   **GitHub Issues**: Creation of new issues with titles, descriptions, and labels; updates to existing issue bodies and titles; addition/removal of labels; comments on issues; and closing of issues.
*   **Console Logs**: Detailed logs of its operations, thoughts, and action outputs.
*   **Web Dashboard (HTTP)**: Provides a user interface displaying logs of all actions taken and LLM inferences made by the agent.

### Environment Variables
*   **Required**:
    *   `GITHUB_PERSONAL_ACCESS_TOKEN`: GitHub Personal Access Token for API authentication.
    *   `OPENAI_API_KEY`: OpenAI API key for LLM inference.
    *   `GITHUB_REPOSITORY_OWNER`: The owner (user or organization) of the target GitHub repository.
    *   `GITHUB_REPOSITORY_NAME`: The name of the target GitHub repository.
*   **Optional**:
    *   `OPENAI_API_BASE`: Base URL for the OpenAI API (defaults to `https://api.openai.com`).
    *   `OPENAI_API_MODEL`: The OpenAI model to use (defaults to `gpt-3.5-turbo`).
    *   `GITHUB_REPOSITORY_ISSUES_BRANCH`: The Git branch to focus on for issue-related changes (defaults to `issues`).

### Dashboard
The agent includes a built-in web server accessible via `http://0.0.0.0:5005`. This dashboard provides a real-time view of the agent's operations, including a history of all actions executed and a detailed log of LLM calls, including prompts and responses.