### Pristine Agent

The Pristine Agent is an AI-driven system designed to autonomously manage GitHub issues and interact with a specified code repository. It leverages a Large Language Model (LLM) for intelligent decision-making, continuously monitors repository events, and maintains an internal memory to inform its actions. The agent also provides a web-based dashboard for real-time monitoring of its activities.

#### Features

*   **GitHub Issue Management**: Creates, retrieves, updates (title, body), closes, comments on, and manages labels for GitHub issues. It aims to detect documentation issues, scope human-created issues, address TODOs in code, identify logic flaws, suggest test improvements, and propose enhancements.
*   **Code Repository Interaction**: Clones and periodically pulls the latest changes from the target GitHub repository, lists all files, and reads the content of specific files to gather context.
*   **Intelligent Decision-Making**: Utilizes an LLM to "think" about the current state of the repository and "decide" on a sequence of actions. Its decision-making process is informed by current context, past actions, and detected events.
*   **Contextual Memory**: Stores and retrieves key-value pairs as internal memories, allowing the agent to maintain continuity and remember important information (e.g., code snippets, issue summaries, user preferences).
*   **Event Monitoring**: Continuously checks the GitHub repository for new commits, newly created issues, and updates or comments on existing issues.
*   **Self-Correction**: Includes built-in logic to automatically refresh its GitHub application access token and retry operations if authentication errors occur.
*   **Monitoring Dashboard**: Provides a web interface for real-time visualization of the agent's executed actions and LLM call history.

#### Inputs

*   **Environment Variables**:
    *   `OPENAI_API_KEY`: Required for LLM functionality.
    *   `OPENAI_API_BASE`: Optional, defaults to `https://api.openai.com`.
    *   `OPENAI_API_MODEL`: Optional, defaults to `gpt-3.5-turbo`.
    *   `GITHUB_REPOSITORY_OWNER`: Required, specifies the GitHub user or organization.
    *   `GITHUB_REPOSITORY_NAME`: Required, specifies the target repository name.
    *   `GITHUB_REPOSITORY_ISSUES_BRANCH`: Optional, defaults to `issues`, specifies the branch for issue-related changes.
    *   The agent also supports loading environment variables from a `.env` file.
*   **GitHub Repository State**: The agent reads the current state of the configured GitHub repository, including file contents, commit history, and all issue-related data (titles, bodies, labels, comments, states, and update times).

#### Outputs

*   **GitHub Repository**: The agent performs actions that modify the GitHub repository, such as creating new issues, updating existing issues (title, body, state), adding comments, and managing labels.
*   **LLM API Calls**: Sends structured prompts to the configured OpenAI Large Language Model.
*   **Monitoring Dashboard (HTTP)**: Serves an interactive web dashboard on port `5005` (accessible via `http://0.0.0.0:5005`) that displays logs of its actions and LLM interactions.
*   **Console Logs**: Outputs detailed logs of its thinking process, detected events, executed actions, and their outcomes to standard output.