### Name
Pristine

### Description
Pristine is an AI agent designed to automate and assist with GitHub repository issue management. It continuously monitors a specified GitHub repository for new events, such as commits, issues, and comments, and intelligently reacts by creating, updating, or closing issues, adding labels, and commenting, all guided by an integrated Large Language Model (LLM). Its primary goal is to help maintain a "pristine" state of issues, focusing on detecting and addressing documentation gaps, code bugs, and missing tests.

### Key Features
*   **GitHub Issue Management:** Creates, updates, closes, labels, and comments on GitHub issues.
*   **Codebase Interaction:** Can list and read files within the cloned repository, and execute shell commands.
*   **LLM-Powered Decision Making:** Utilizes an OpenAI-compatible LLM for generating thoughts and deciding on actions based on current context and repository state.
*   **Contextual Memory:** Stores and retrieves key information (memories) to maintain continuity and understanding across operations.
*   **Event-Driven Operation:** Actively monitors the GitHub repository for new commits, issue updates, and comments, triggering agent activity.
*   **Web Monitoring Dashboard:** Provides a local web interface to view a history of agent actions and LLM calls.

### Inputs
*   **Environment Variables:** Configuration for GitHub authentication, OpenAI API access, and target repository details.
*   **GitHub Repository Events (via polling):** New commits, issue creation/updates, and comments on issues.

### Outputs
*   **GitHub Actions:** Creation of new issues, updates to existing issue bodies/titles, adding/removing labels, and comments.
*   **LLM Inferences:** Calls to the configured OpenAI model.
*   **Console Output (stdout):** Logs detailing agent operations and decisions.
*   **Web Dashboard (HTTP):** Real-time monitoring of agent activities and LLM interactions accessible via a web browser.

### Environment Variables
*   `GITHUB_PERSONAL_ACCESS_TOKEN`: Your GitHub Personal Access Token for repository access.
*   `OPENAI_API_KEY`: Your OpenAI API key for LLM inference.
*   `OPENAI_KEY`: (Alternative/Fallback) OpenAI API key.
*   `OPENAI_API_BASE`: The base URL for the OpenAI API (defaults to `https://api.openai.com`).
*   `OPENAI_API_MODEL`: The LLM model to use (defaults to `gpt-3.5-turbo`).
*   `GITHUB_REPOSITORY_OWNER`: The owner (user or organization) of the GitHub repository.
*   `GITHUB_REPOSITORY_NAME`: The name of the GitHub repository to manage.
*   `GITHUB_REPOSITORY_ISSUES_BRANCH`: The branch used for issue management (defaults to `issues`).