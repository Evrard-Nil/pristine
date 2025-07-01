### Pristine Agent

The Pristine Agent is an AI-powered GitHub issue management bot designed to autonomously monitor, analyze, and interact with a specified GitHub repository. It operates in a continuous "Think-Decide-Act" loop, leveraging large language models (LLMs) to manage issues, respond to events, and maintain an organized project state.

#### Key Capabilities

*   **GitHub Issue Management:** Creates, updates, closes, labels, and comments on GitHub issues. It can detect new issues, updates to existing issues (like new comments), and new commits in the repository.
*   **Repository Interaction:** Clones and periodically pulls the target GitHub repository to stay up-to-date with code changes and detect new commits. It can list all files and read the content of specific files.
*   **Contextual Awareness:** Maintains an internal "memory" to store important information, allowing it to build a comprehensive context for its decision-making, including current time, known open/closed issues, past events, and previous actions/thoughts.
*   **LLM Integration:** Utilizes a configurable Large Language Model (e.g., OpenAI's GPT models) for sophisticated reasoning and action planning. It generates "thoughts" to analyze the current situation and "decides" on a sequence of actions to perform.
*   **Self-Monitoring Dashboard:** Provides a built-in web server with a dashboard to visualize its action history and LLM call logs, offering transparency into its operations.

#### Interaction Flow

The agent continuously performs the following loop:
1.  **Event Check:** Monitors the configured GitHub repository for new commits, new issues, and updates to existing issues.
2.  **Think:** Uses an LLM to analyze the current state, including its internal memories, known issues, and recent events, to formulate a strategic "thought" on what needs to be done.
3.  **Decide:** Based on its "thought" and the current context, it uses the LLM to generate a structured list of actions to execute.
4.  **Act:** Executes the decided actions, which can involve interacting with GitHub (e.g., creating an issue, adding a comment), managing its internal memories, reading repository files, or performing custom LLM inferences.

#### Configuration (Inputs)

The agent is configured via environment variables:

*   **OPENAI_API_KEY**: Your OpenAI API key (required).
*   **OPENAI_API_BASE**: The base URL for the OpenAI API (defaults to `https://api.openai.com`).
*   **OPENAI_API_MODEL**: The specific LLM model to use (defaults to `gpt-3.5-turbo`).
*   **GITHUB_REPOSITORY_OWNER**: The owner (user or organization) of the target GitHub repository (required).
*   **GITHUB_REPOSITORY_NAME**: The name of the target GitHub repository (required).
*   **GITHUB_REPOSITORY_ISSUES_BRANCH**: The branch used for issue management (defaults to `issues`).
*   **GitHub App Credentials**: The agent uses hardcoded GitHub App credentials (`GH_APP_ID`, `GH_APP_CLIENT_ID`, `GH_APP_CLIENT_SECRET`) and loads the private key from the `.gh_pk` file for authentication with GitHub.

#### Outputs

*   **GitHub Actions:** Creates, updates, closes issues, adds/removes labels, and posts comments on the configured GitHub repository.
*   **Local Repository Changes:** Clones and pulls the target repository to a temporary directory.
*   **LLM Inferences:** Sends prompts to and receives responses from the configured LLM.
*   **Monitoring Dashboard (HTTP):** Exposes a web interface on port `5005` to display real-time logs of agent actions and LLM calls.
*   **Console Output (stdout):** Provides detailed logs of its operations, thoughts, and action outputs.