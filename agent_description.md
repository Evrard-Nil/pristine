### Agent Name
Pristine Agent

### Description
The Pristine Agent is an AI-powered system designed to autonomously manage GitHub issues within a specified repository. It continuously monitors the repository for new events such as commits, issues, comments, and pull requests. Leveraging a Large Language Model (LLM), the agent analyzes the current state, formulates a "thought" process, and then "decides" on a series of actions to maintain the issue tracker effectively. It can create, update, close, label, and comment on issues, and also interact with the repository's codebase by listing and reading files. The agent includes a built-in web dashboard for real-time monitoring of its activities and LLM interactions.

### Functions
*   **GitHub Issue Management**: Creates new issues, retrieves existing issue details, adds or removes labels, closes issues, and comments on issues. It can also edit the title and body of existing issues.
*   **Repository Interaction**: Clones and pulls the latest changes from a GitHub repository, detects new commits, and allows listing and reading files within the cloned repository.
*   **AI-Powered Decision Making**: Integrates with a Large Language Model (LLM) to generate internal "thoughts" based on the current context and then decide on a sequence of actions to execute.
*   **Context and Memory Management**: Maintains an internal memory to store and retrieve key-value pairs, helping the agent remember important information and maintain continuity across its operations.
*   **Self-Monitoring Dashboard**: Provides a web-based interface to monitor the agent's executed actions and LLM calls in real-time.

### Inputs
*   **Environment Variables**: Configuration for GitHub App credentials (`OPENAI_API_KEY`, `OPENAI_API_BASE`, `OPENAI_API_MODEL`, `GITHUB_REPOSITORY_OWNER`, `GITHUB_REPOSITORY_NAME`, `GITHUB_REPOSITORY_ISSUES_BRANCH`).
*   **GitHub Repository (API/Git)**: New commits, new issues, updates to existing issues (e.g., comments, state changes), and pull requests.
*   **Internal Context**: Agent's own stored memories, previous action outputs, and generated thoughts.

### Outputs
*   **GitHub (API)**: Creation of new issues, updates to issue titles/bodies, addition/removal of labels, closing of issues, and posting comments on issues.
*   **Large Language Model (API)**: System and user prompts sent for inference.
*   **Web Dashboard (HTTP)**: Real-time display of agent actions and LLM call history, accessible via a web browser on port 5005.
*   **Console (stdout)**: Logs of operational status, agent thoughts, and action results.