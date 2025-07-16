### Pristine AI Agent

The Pristine AI Agent is an autonomous system designed to manage and maintain GitHub repository issues. It continuously monitors a designated repository for changes and uses a large language model (LLM) to analyze the context and decide on appropriate actions.

#### Overview
Pristine's primary goal is to assist in keeping the repository's issues organized and up-to-date. It identifies potential problems like missing documentation, bugs, or absent unit tests, creating detailed, actionable issues. It also interacts with existing issues by responding to comments, prioritizing tasks, managing labels, and closing resolved issues.

#### Key Features
*   **Intelligent Issue Management**: Automatically detects and creates GitHub issues for documentation gaps, code bugs, and missing tests.
*   **Contextual Awareness**: Maintains a persistent memory and tracks past events and current GitHub issues to inform its decisions.
*   **Event-Driven Operation**: Monitors the linked GitHub repository for new commits and issue updates, reacting to changes in real-time.
*   **Action Execution**: Capable of performing various operations, including reading files, running shell commands, and extensive GitHub API interactions (creating, getting, updating, labeling, commenting on, and closing issues).
*   **LLM Integration**: Leverages a configurable Large Language Model (e.g., OpenAI's GPT models) for "thinking" processes and generating actionable plans.
*   **Monitoring Dashboard**: Provides a web-based dashboard (accessible via HTTP) to view a history of all actions taken and LLM calls made by the agent.
*   **Configurable**: Easily configured via environment variables to connect to specific GitHub repositories and LLM services.

#### How it Works
The agent operates in a continuous loop:
1.  **Event Check**: It pulls the latest changes from the GitHub repository and fetches recent issue updates to identify new commits, issues, or changes to existing issues.
2.  **Thinking**: It constructs a comprehensive prompt using its internal memory, current open issues, and recent events. This prompt is sent to an LLM, which generates a "thought" and a structured list of actions to take.
3.  **Acting**: It executes the actions decided by the LLM, which can include interacting with the repository's files, managing its internal memory, or performing various GitHub API operations.
4.  **Completion**: The agent can mark its current task as complete, pausing further LLM inference until new external events occur, reducing unnecessary activity.

#### Inputs
*   **Environment Variables**:
    *   `GITHUB_PERSONAL_ACCESS_TOKEN`: For GitHub API authentication.
    *   `OPENAI_API_KEY`: For OpenAI LLM authentication.
    *   `OPENAI_API_BASE` (Optional): OpenAI API endpoint URL.
    *   `OPENAI_API_MODEL` (Optional): Specific OpenAI model to use.
    *   `GITHUB_REPOSITORY_OWNER`: Owner of the target GitHub repository.
    *   `GITHUB_REPOSITORY_NAME`: Name of the target GitHub repository.
    *   `GITHUB_REPOSITORY_ISSUES_BRANCH` (Optional): Branch for issue management.
*   **GitHub Repository Events**: New commits, new issues, updates to existing issues (e.g., comments, state changes).

#### Outputs
*   **GitHub Repository**: Creates, updates, comments on, and closes issues. Manages issue labels.
*   **Local File System**: Clones and interacts with a local copy of the repository.
*   **LLM Calls**: Sends prompts to and receives responses from the configured LLM.
*   **Web Dashboard**: Serves an interactive monitoring dashboard on port `5005` (HTTP).
*   **Console Output**: Logs its operational status, thoughts, and action results to standard output.