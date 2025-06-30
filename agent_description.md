### Agent Name
Pristine Agent

### Description
The Pristine Agent is an AI-powered GitHub issue management agent designed to autonomously monitor and interact with a specified GitHub repository. Its primary goal is to assist in maintaining the health and organization of repository issues by detecting changes, thinking about appropriate responses, and executing actions such as creating, updating, or closing issues, and managing labels and comments. It leverages a Large Language Model (LLM) for its reasoning and decision-making processes and provides a web-based dashboard for monitoring its activities.

### Key Features
*   **GitHub Issue Management**: Creates, updates, closes, labels, and comments on GitHub issues.
*   **Repository Monitoring**: Continuously pulls the target GitHub repository to detect new commits and monitors GitHub for new or updated issues.
*   **LLM-Powered Intelligence**: Utilizes an external Large Language Model for "thinking" (analyzing context and formulating strategies) and "deciding" (selecting specific actions to perform).
*   **Contextual Memory**: Maintains an internal memory to store and retrieve important information, allowing for persistent context and continuity in its operations.
*   **Action Execution**: Executes a predefined set of actions, including interacting with the file system (listing/reading files), managing its internal memory, and performing GitHub operations.
*   **Self-Correction**: Includes logic to refresh GitHub access tokens and retry operations in case of authentication failures.
*   **Monitoring Dashboard**: Provides a web interface to view logs of all executed actions and LLM calls, offering transparency into the agent's operations.

### Inputs
*   **Environment Variables**:
    *   `OPENAI_API_KEY`: API key for OpenAI LLM services (required).
    *   `OPENAI_API_BASE`: Base URL for the OpenAI API (defaults to `https://api.openai.com`).
    *   `OPENAI_API_MODEL`: Specific OpenAI model to use (defaults to `gpt-3.5-turbo`).
    *   `GITHUB_REPOSITORY_OWNER`: GitHub repository owner (user or organization) (required).
    *   `GITHUB_REPOSITORY_NAME`: GitHub repository name (required).
    *   `GITHUB_REPOSITORY_ISSUES_BRANCH`: Git branch for issue management (defaults to `issues`).
*   **GitHub Repository**: Codebase content and commit history (via Git clone).
*   **GitHub Issues**: Current state, new issues, comments, and updates on existing issues.

### Outputs
*   **GitHub Actions**:
    *   New issues created with specified titles, bodies, and labels.
    *   Updates to existing issues (title, body, state, labels).
    *   Comments added to issues.
*   **Internal Logs**:
    *   Detailed logs of all executed actions, including their results and duration.
    *   Detailed logs of all LLM calls, including system prompts, user prompts, responses, and duration.
*   **Web Server (HTTP)**:
    *   **Dashboard**: HTML interface providing a real-time view of action and LLM call history.
    *   **API Endpoints**: JSON data for action logs (`/api/actions`) and LLM call logs (`/api/llm-calls`).

### Monitoring
The agent exposes a web server on port `5005` (HTTP) that provides a dashboard to monitor its operations. This dashboard displays a history of actions taken and LLM calls made, including details like timestamps, durations, and content of prompts/responses.