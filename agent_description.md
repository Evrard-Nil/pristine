### Pristine Agent

Pristine is an AI agent designed to autonomously manage GitHub repository issues. It continuously monitors a specified GitHub repository for new events, analyzes the current state, and makes decisions to create, update, or close issues.

**Main Functions:**

*   **Issue Management**: Creates new issues, updates existing ones (title, body, labels), adds comments, and closes issues based on its analysis of the repository and ongoing events.
*   **Codebase Monitoring**: Detects new commits, scans the codebase to identify potential documentation needs, logic flaws, areas for testing improvements, and feature suggestions.
*   **Contextual Reasoning**: Utilizes an internal memory system to retain important information, past actions, and LLM outputs, enabling it to make informed decisions and maintain continuity.
*   **LLM Integration**: Leverages a Large Language Model (LLM) for "thinking" (analyzing the situation and formulating a plan) and "deciding" (generating a sequence of specific actions to take).
*   **Event-Driven Operation**: Regularly checks for new commits and updates to GitHub issues, reacting to changes in the repository's state.
*   **Real-time Monitoring Dashboard**: Provides a web interface to view a history of executed actions and LLM calls, including their duration and results.

**Key Features:**

*   **Dynamic Issue Handling**: Can detect various issue types (documentation, bugs, enhancements, tests), prioritize them, and manage their lifecycle.
*   **Duplicate Prevention**: Aims to avoid creating redundant issues by leveraging historical data and current open/closed issues.
*   **Repository Interaction**: Capable of listing all files and reading the content of specific files within the cloned repository.
*   **Self-Correction**: Can log and potentially react to errors encountered during action execution.

**Inputs:**

*   **GitHub Repository**: Reads repository content (files, commits) and issue data (issues, comments, labels).
*   **Environment Variables**: Configuration parameters for GitHub and OpenAI API access.

**Outputs:**

*   **GitHub Actions**: Creates, updates, comments on, adds/removes labels from, and closes issues on the configured GitHub repository.
*   **Internal State**: Stores and manages key-value memories within its context.
*   **Monitoring Dashboard (HTTP)**: Serves a web dashboard on port 5005, providing real-time logs of agent actions and LLM interactions.

**Configuration:**

The agent requires the following environment variables to be set:

*   **`OPENAI_API_KEY`**: Your API key for authenticating with the OpenAI service.
*   **`GITHUB_REPOSITORY_OWNER`**: The GitHub username or organization that owns the target repository.
*   **`GITHUB_REPOSITORY_NAME`**: The name of the GitHub repository the agent will manage.

Optional environment variables:

*   **`OPENAI_API_BASE`**: The base URL for the OpenAI API (defaults to `https://api.openai.com`).
*   **`OPENAI_API_MODEL`**: The specific OpenAI model to use for LLM inference (defaults to `gpt-3.5-turbo`).
*   **`GITHUB_REPOSITORY_ISSUES_BRANCH`**: The name of the branch where the agent will manage issues (defaults to `issues`).

Note: GitHub App credentials (ID, client ID, client secret, and private key) are pre-configured within the agent's code, with the private key loaded from a local file named `.gh_pk`.