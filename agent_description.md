### Agent Name
Pristine

### Overview
Pristine is an autonomous AI agent designed to manage and maintain issues on a specified GitHub repository. It continuously monitors the repository for new commits and issue updates, leveraging a Large Language Model (LLM) to analyze the repository context and decide on appropriate actions. Its primary goal is to help maintain the quality and organization of the repository by identifying and addressing issues related to documentation, bugs, and testing.

### Key Capabilities
*   **GitHub Issue Management:** Creates new issues (e.g., for missing documentation, bugs, or tests), updates existing issues (e.g., editing body/title, adding/removing labels), comments on issues, and closes resolved issues.
*   **Repository Interaction:** Clones the target GitHub repository, pulls the latest changes, lists files, reads file contents, and executes shell commands within the cloned repository's environment.
*   **Contextual Reasoning:** Maintains an internal memory of past actions, thoughts, and known issues to inform its decision-making process. It uses this context to build prompts for the LLM.
*   **LLM-Powered Decision Making:** Utilizes a configurable LLM (e.g., OpenAI's GPT models) to "think" about the current state of the repository and generate a sequence of actions to take.
*   **Event-Driven Operation:** Automatically detects and reacts to new commits and changes in GitHub issues, resetting its completion status to re-evaluate the situation and take new actions.
*   **Self-Monitoring Dashboard:** Provides a local web interface for real-time monitoring of its executed actions and LLM calls, offering transparency into its operations.

### Operation
The agent operates in a continuous loop. In each cycle, it:
1.  Checks for new events, such as new commits or updates to GitHub issues.
2.  Updates its internal context, including a history of past events and the current state of open and closed issues.
3.  If not marked complete, it uses the LLM to generate a "thought" and a list of actions based on the current context.
4.  Executes the decided actions sequentially, logging their outputs and durations.
5.  Updates its internal state with the results of the actions.
The agent can be explicitly marked "complete" to pause LLM inference until a new external event occurs, preventing unnecessary activity.

### Inputs
*   **Environment Variables:**
    *   `GITHUB_PERSONAL_ACCESS_TOKEN`: A mandatory token for authenticating with the GitHub API.
    *   `OPENAI_API_KEY`: A mandatory key for authenticating with the OpenAI API.
    *   `OPENAI_API_BASE` (Optional): The base URL for the OpenAI API (defaults to `https://api.openai.com`).
    *   `OPENAI_API_MODEL` (Optional): The specific LLM model to use for inference (defaults to `gpt-3.5-turbo`).
    *   `GITHUB_REPOSITORY_OWNER`: The owner (user or organization) of the GitHub repository to manage.
    *   `GITHUB_REPOSITORY_NAME`: The name of the GitHub repository to manage.
    *   `GITHUB_REPOSITORY_ISSUES_BRANCH` (Optional): The Git branch within the repository that the agent will use for operations (defaults to `issues`).
*   **GitHub Repository Events:** New commits, new issues, and updates to existing issues (e.g., new comments, label changes, state changes).
*   **LLM Prompts:** System and user prompts dynamically generated based on the agent's context for LLM inference.

### Outputs
*   **GitHub Repository (External):**
    *   Creates new issues with specified titles, bodies, and labels.
    *   Updates existing issues by editing their titles or bodies.
    *   Adds or removes labels from issues.
    *   Closes issues.
    *   Adds comments to issues.
*   **Local Repository Clone (File System):**
    *   Performs Git pull operations to keep the local clone up-to-date.
    *   Reads the content of specified files.
    *   Executes arbitrary shell commands within the cloned repository's directory.
*   **Internal Memory (In-memory):**
    *   Stores and retrieves key-value pairs to maintain long-term context and continuity.
*   **Monitoring Dashboard (HTTP):**
    *   Provides a web-based dashboard accessible via HTTP on port `5005` (e.g., `http://localhost:5005`). This dashboard displays logs of all executed actions and LLM calls, including timestamps, durations, and results.
*   **Console Output (stdout):**
    *   Logs its operational status, detected events, LLM thoughts, and action outputs to the standard console.