### Agent Name
Pristine Agent

### Description
The Pristine Agent is an autonomous AI agent designed to manage GitHub repository issues. It continuously monitors a specified repository for new commits and issue updates, leveraging a Large Language Model (LLM) to analyze the situation, formulate thoughts, and decide on appropriate actions. Its primary goal is to streamline issue management by identifying new documentation needs, scoping human-created issues, responding to comments, detecting code-related tasks (like TODOs, logic flaws, and test improvements), and maintaining an organized issue backlog.

### Functionality
*   **GitHub Issue Management**: Creates new issues, retrieves existing ones, updates issue titles and bodies, closes issues, adds/removes labels, and comments on issues.
*   **Repository Monitoring**: Automatically pulls the latest changes from the configured GitHub repository and detects new commits, issues, and updates to existing issues or their comments.
*   **AI-Powered Decision Making**: Utilizes an integrated LLM to process current context, past events, and internal memories to generate strategic thoughts and decide on a sequence of actions to take.
*   **Internal Context Management**: Stores and retrieves key-value pair "memories" to maintain continuity and important information across its operational cycles.
*   **Real-time Monitoring Dashboard**: Provides a local web interface to display logs of all executed actions and LLM calls, offering transparency into the agent's operations.

### Inputs
*   **Environment Variables**: Configured via environment variables for API keys and repository details.
    *   `OPENAI_API_KEY`: Required for LLM authentication.
    *   `OPENAI_API_BASE`: Optional, defaults to `https://api.openai.com`.
    *   `OPENAI_API_MODEL`: Optional, defaults to `gpt-3.5-turbo`.
    *   `GITHUB_REPOSITORY_OWNER`: Required, the owner of the target GitHub repository.
    *   `GITHUB_REPOSITORY_NAME`: Required, the name of the target GitHub repository.
    *   `GITHUB_REPOSITORY_ISSUES_BRANCH`: Optional, the Git branch to manage issues on, defaults to `issues`.
*   **GitHub Repository Events**: New commits, new issues, updates to existing issues (e.g., body changes, new comments, label changes).

### Outputs
*   **GitHub Repository Modifications**: Creates, updates, or closes issues; adds or removes labels; and posts comments on issues via the GitHub API.
*   **LLM Inferences**: Generates text responses from the configured LLM based on internal prompts.
*   **Internal State Updates**: Modifies its internal memory store based on actions taken or information gathered.
*   **Web Dashboard (HTTP on port 5005)**: Serves a web page displaying a chronological log of all actions performed and LLM interactions, including timestamps, durations, and results.