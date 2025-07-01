### Pristine Agent

The Pristine Agent is an autonomous AI agent designed to manage and maintain GitHub repositories by intelligently interacting with issues and codebase. It operates in a continuous loop, leveraging a Large Language Model (LLM) for reasoning and decision-making, and integrates directly with GitHub and the local repository.

#### Key Capabilities
*   **Intelligent GitHub Issue Management**: Autonomously creates, updates, and closes issues; manages labels; comments on issues; and prioritizes tasks based on predefined categories (e.g., `bug`, `enhancement`, `documentation`, `p0`, `p1`, `p2`).
*   **Codebase Analysis**: Reads and analyzes repository files to identify potential bugs, areas for improvement, and documentation needs, mapping these findings to new or existing issues.
*   **Contextual Learning & Memory**: Stores and retrieves important information (memories) in its internal context to maintain continuity and inform future decisions.
*   **Event Monitoring**: Continuously monitors the configured GitHub repository for new commits, issues, and updates (including comments).
*   **LLM-Powered Reasoning**: Utilizes a large language model (defaulting to OpenAI's GPT-3.5-turbo) to generate thoughts and decide on appropriate actions based on the current context and predefined system prompts.
*   **Real-time Monitoring Dashboard**: Provides a local web interface to visualize its action history and LLM call logs, offering transparency into its operations.

#### Operation
The agent operates in a persistent loop, performing the following steps:
1.  **Event Checking**: Pulls the latest changes from the GitHub repository and fetches recent issue updates to detect new events (e.g., new commits, new issues, comments on existing issues).
2.  **Thinking**: Analyzes the current state, including its internal memories, known open/closed issues, and recent events, using an LLM to formulate a "thought" on what needs to be done.
3.  **Decision Making**: Based on its thought process, it instructs the LLM to generate a structured list of specific actions to perform.
4.  **Action Execution**: Executes the decided actions, which can involve interacting with GitHub (creating issues, commenting), reading repository files, or managing its internal memory.
5.  **Logging**: All LLM calls and executed actions are logged internally and made available through the monitoring dashboard.

#### Inputs
*   **Environment Variables**: Configuration parameters for GitHub App credentials, OpenAI API, and target repository details.
*   **GitHub API (HTTP)**: Receives information about repository commits, issues, labels, and comments.
*   **LLM Responses (HTTP)**: Receives generated text and structured action plans from the configured Large Language Model.

#### Outputs
*   **GitHub API (HTTP)**: Sends requests to create, update, close issues, add/remove labels, and post comments.
*   **Local Filesystem**: Clones and manages a local copy of the target GitHub repository.
*   **Monitoring Dashboard (HTTP)**: Serves a web interface on port `5005` displaying action and LLM call logs.
*   **Console (stdout)**: Logs its operational status, thoughts, and action outputs.

#### Environment Variables
The agent requires the following environment variables for configuration:
*   `OPENAI_API_KEY`: Your OpenAI API key (required).
*   `OPENAI_API_BASE`: The base URL for the OpenAI API (optional, defaults to `https://api.openai.com`).
*   `OPENAI_API_MODEL`: The LLM model to use for inference (optional, defaults to `gpt-3.5-turbo`).
*   `GITHUB_REPOSITORY_OWNER`: The GitHub username or organization that owns the target repository (required).
*   `GITHUB_REPOSITORY_NAME`: The name of the GitHub repository the agent will manage (required).
*   `GITHUB_REPOSITORY_ISSUES_BRANCH`: The branch name where the agent will manage issues (optional, defaults to `issues`).
*   Additionally, it reads a GitHub App private key from the `.gh_pk` file in its root directory.