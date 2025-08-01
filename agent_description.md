### Pristine Agent

The Pristine Agent is an autonomous AI agent designed to manage and maintain GitHub repository issues. It continuously monitors a designated GitHub repository for new commits and issue updates, leveraging a Large Language Model (LLM) to "think" and decide on appropriate actions. Its primary goal is to assist in keeping the repository's issues organized, relevant, and actionable.

**Key Functionalities**:

*   **GitHub Issue Management**: Creates, retrieves, updates, labels, comments on, and closes GitHub issues. It aims to create small, focused, and actionable issues.
*   **Repository Monitoring**: Clones and monitors a specified GitHub repository, detecting new commits and changes to issues.
*   **LLM Integration**: Uses an OpenAI-compatible Large Language Model to process context, generate thoughts, and decide on a sequence of actions.
*   **Internal Memory**: Maintains an internal memory (key-value store) to retain important information and ensure continuity across its operations.
*   **Codebase Interaction**: Can list files, read file contents, and execute shell commands within the cloned repository to gather information.
*   **Self-Monitoring Dashboard**: Provides a web-based dashboard to monitor its action history and LLM call logs in real-time.
*   **Event-Driven Operation**: Reacts to new events (commits, issue changes) by re-evaluating its state and taking new actions, or marks itself complete and waits for external events if no further actions are needed.

**Purpose**:

The agent's responsibilities include identifying and creating issues for:
*   Missing, outdated, or incomplete documentation.
*   Bugs or logic flaws in the code.
*   Missing unit tests.

It also handles answering human comments, mapping TODOs to issues, closing resolved issues, and prioritizing tasks using predefined labels such as `documentation`, `bug`, `enhancement`, `test`, `needs-human-input`, `ready-for-approval`, `ready-for-implementation-by-ai`, `duplicate`, `p0`, `p1`, and `p2`.

**Inputs**:

*   **Environment Variables**: Configured via environment variables including `GITHUB_PERSONAL_ACCESS_TOKEN`, `OPENAI_API_KEY`, `OPENAI_API_BASE` (optional, defaults to `https://api.openai.com`), `OPENAI_API_MODEL` (optional, defaults to `gpt-3.5-turbo`), `GITHUB_REPOSITORY_OWNER`, `GITHUB_REPOSITORY_NAME`, and `GITHUB_REPOSITORY_ISSUES_BRANCH` (optional, defaults to `issues`).
*   **GitHub Repository (HTTP/Git)**: Reads repository content (files, commits) and issue data (titles, bodies, labels, comments, states).
*   **LLM Responses (HTTP)**: Receives text responses from the configured Large Language Model.

**Outputs**:

*   **GitHub Actions (HTTP)**: Creates, updates, or comments on GitHub issues.
*   **Command Line (stdout)**: Prints its thoughts, actions, and execution outputs to the console.
*   **Web Dashboard (HTTP)**: Exposes a real-time monitoring dashboard on port 5005, displaying logs of executed actions and LLM calls.