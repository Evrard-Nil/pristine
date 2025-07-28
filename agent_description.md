### Agent Name
Pristine

### Overview
Pristine is an AI agent designed to autonomously manage and maintain GitHub repository issues. It continuously monitors a designated repository for new events (like commits or issue updates), uses a Large Language Model (LLM) to "think" and decide on appropriate actions, and then executes those actions. Its primary goal is to help maintain the quality of the repository by identifying and creating issues for documentation gaps, bugs, and missing tests, as well as managing existing issues by responding to comments, applying labels, and closing resolved tasks.

### Key Capabilities
*   **GitHub Issue Management:** Creates, retrieves, updates, comments on, labels, and closes GitHub issues.
*   **Repository Interaction:** Clones the target GitHub repository, pulls latest changes, detects new commits, lists and reads files, and executes shell commands within the repository.
*   **LLM-Powered Decision Making:** Utilizes an LLM to analyze the current context (memories, open issues, recent events) and generate a thought process, followed by a structured list of actions to take.
*   **Contextual Memory:** Can store and retrieve key-value memories to maintain continuity and remember important information across its operational cycles.
*   **Event-Driven Operation:** Reacts to new commits and updates on GitHub issues, resetting its completion status to re-evaluate the situation.
*   **Self-Monitoring Dashboard:** Provides a web-based dashboard on port 5005 to display a real-time history of its executed actions and LLM calls, offering transparency into its operations.

### Operation
Pristine operates in a continuous loop:
1.  **Event Check:** It pulls the latest changes from the configured GitHub repository and checks for new commits or updates to issues.
2.  **Context Building:** It compiles a detailed context for the LLM, including current time, stored memories, open issues, past events, and the output of its last actions.
3.  **Thinking Phase:** It sends the contextual prompt to an LLM, which generates a "thought" (a natural language explanation of its reasoning) and a JSON array of actions to perform.
4.  **Action Execution:** It executes the decided actions sequentially, which can involve interacting with GitHub, the local file system, running shell commands, or making further LLM calls.
5.  **Monitoring & Logging:** All actions and LLM calls are logged and made available through a local web dashboard.
6.  **Sleep/Wait:** After executing actions, it waits for a short duration or, if marked complete, pauses until a new external event triggers a re-evaluation.

### Inputs
*   **Environment Variables:** Configuration parameters for GitHub authentication, OpenAI API access, and repository details.
*   **GitHub Repository:** New commits, new issues, issue comments, and other issue state changes.
*   **User Prompts (via LLM):** System and user prompts provided to the LLM for decision-making.

### Outputs
*   **GitHub (API):** New issues, updated issue titles/bodies, comments on issues, added/removed labels, closed issues.
*   **Local File System/Shell:** Results of file listing, file content, and command execution outputs.
*   **LLM (API):** Requests for text generation.
*   **Web Dashboard (HTTP):** A dashboard displaying action history and LLM call history, accessible via `http://0.0.0.0:5005`.
*   **Stdout:** Detailed logs of its operational flow, thoughts, and action results.

### Environment Variables
*   `GITHUB_PERSONAL_ACCESS_TOKEN`: Your GitHub Personal Access Token for repository access.
*   `OPENAI_API_KEY` (or `OPENAI_KEY`): Your OpenAI API key for LLM access.
*   `OPENAI_API_BASE`: (Optional) The base URL for the OpenAI API (defaults to `https://api.openai.com`).
*   `OPENAI_API_MODEL`: (Optional) The specific OpenAI model to use (defaults to `gpt-3.5-turbo`).
*   `GITHUB_REPOSITORY_OWNER`: The GitHub username or organization that owns the repository.
*   `GITHUB_REPOSITORY_NAME`: The name of the GitHub repository to manage.
*   `GITHUB_REPOSITORY_ISSUES_BRANCH`: (Optional) The Git branch dedicated to issue-related changes (defaults to `issues`).