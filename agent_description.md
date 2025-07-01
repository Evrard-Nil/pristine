### Pristine Agent

Pristine is an AI agent designed to autonomously manage GitHub issues within a specified repository. It continuously monitors the repository for new events, analyzes the current state, and takes proactive steps to maintain a clean and organized issue tracker.

**Main Functions:**
*   **GitHub Issue Management:** Creates, updates, closes, comments on, and manages labels for GitHub issues. It aims to detect and address documentation needs, code flaws, testing gaps, and feature enhancements. It also works to prevent duplicate issues.
*   **Repository Interaction:** Clones and keeps a local copy of the target GitHub repository updated, detecting new commits and allowing for file listing and reading.
*   **LLM Integration:** Utilizes a Large Language Model (LLM) to "think" about the repository's state and "decide" on a sequence of actions to perform.
*   **Context Management:** Maintains an internal memory to store and retrieve important information, ensuring continuity and informed decision-making.
*   **Monitoring Dashboard:** Provides a web-based interface to view a real-time history of all actions taken and LLM calls made by the agent.

**Key Features:**
*   **Autonomous Operation:** Runs in a continuous loop, independently identifying and addressing issues.
*   **Event-Driven:** Responds to new commits, issues, and updates on existing issues.
*   **Intelligent Prioritization:** Aims to prioritize issues based on importance and urgency.
*   **Duplicate Prevention:** Actively works to avoid creating redundant issues.
*   **Human Collaboration:** Designed to work alongside human maintainers, creating small, actionable issues and seeking human input when necessary.

**Inputs:**
*   **Environment Variables:**
    *   `OPENAI_API_KEY`: Required for LLM authentication.
    *   `OPENAI_API_BASE`: Optional, defaults to `https://api.openai.com`.
    *   `OPENAI_API_MODEL`: Optional, defaults to `gpt-3.5-turbo`.
    *   `GITHUB_REPOSITORY_OWNER`: Required, the owner of the target GitHub repository.
    *   `GITHUB_REPOSITORY_NAME`: Required, the name of the target GitHub repository.
    *   `GITHUB_REPOSITORY_ISSUES_BRANCH`: Optional, the branch where issues are managed (defaults to `issues`).
*   **GitHub Repository Events:** New commits, issues, comments, and pull requests.

**Outputs:**
*   **GitHub Issues:** Creates, updates, closes, comments on, and adds/removes labels from issues on the configured GitHub repository.
*   **Web Dashboard (HTTP on port 5005):** Provides a user interface to monitor the agent's action and LLM call history.
*   **Console Output:** Logs the agent's thinking process, decided actions, and execution results.