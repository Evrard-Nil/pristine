### Agent: Pristine

Pristine is an AI agent designed to automate and assist with GitHub issue management. It continuously monitors a specified GitHub repository, identifies potential issues, and takes actions to maintain the quality and organization of the project's issues.

**Main Functions:**

*   **GitHub Issue Management**: Automatically creates, updates, closes, labels, comments on, and edits GitHub issues. It can detect new documentation issues, bugs, logic flaws, and missing unit tests.
*   **Codebase Analysis**: Analyzes the repository's code, documentation, and existing issues to understand the project and identify areas for improvement or problems.
*   **LLM-Powered Reasoning**: Utilizes Large Language Models (LLMs) to process context, formulate thoughts, and decide on appropriate issue management actions.
*   **Contextual Memory**: Maintains an internal memory to store important information, such as code snippets, issue summaries, and user preferences, ensuring continuity and informed decision-making.
*   **Repository Monitoring**: Regularly checks the connected GitHub repository for new commits, issue updates, and comments, reacting to changes as needed.
*   **Interactive Dashboard**: Provides a web-based dashboard to visualize a history of actions taken and LLM calls made by the agent.

**Inputs:**

*   **GitHub Repository Events**: (HTTP/Webhook) New commits, issue creation, updates, and comments on issues.
*   **Environment Variables**: (System) Configuration details for GitHub authentication, OpenAI API access, and repository specifics.

**Outputs:**

*   **GitHub Issues**: (HTTP) Creation of new issues, updates to existing issues (title, body, labels), comments on issues, and closing of resolved issues.
*   **LLM Inferences**: (Internal) Generated text, strategic thoughts, and action plans.
*   **Web Dashboard**: (HTTP) A user interface accessible on port 5005 displaying historical logs of agent actions and LLM interactions.

**Important Details:**

Pristine operates autonomously, aiming to keep the number of open issues minimal by focusing on actionable tasks and avoiding duplicates. It prioritizes issues based on importance and urgency and only comments or closes issues when necessary. It requires a GitHub Personal Access Token and OpenAI API Key for full functionality.