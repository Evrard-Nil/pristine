### Pristine Agent

The Pristine Agent is an autonomous AI agent designed to manage GitHub issues for a specified repository. It continuously monitors the repository for changes, leverages a Large Language Model (LLM) for decision-making, and executes various GitHub actions to maintain issue hygiene and project health. It also provides a web-based dashboard for real-time monitoring of its activities.

#### Key Features

*   **GitHub Issue Management**: Creates, retrieves, updates (title, body, labels, comments), and closes GitHub issues. It aims to create small, actionable, and focused issues related to documentation, bugs, or missing tests.
*   **Repository Monitoring**: Periodically checks the configured GitHub repository for new commits and updates to existing issues.
*   **LLM Integration**: Utilizes an LLM to analyze the current repository state, past events, and internal memories to formulate thoughts and decide on the next sequence of actions.
*   **Internal Memory**: Stores and retrieves key-value pair memories to maintain context and continuity across its operations, allowing it to remember important information like code snippets, issue summaries, or user preferences.
*   **Codebase Interaction**: Can list all files in the repository, read the content of specific files, and execute arbitrary shell commands within the cloned repository's directory.
*   **Self-Management**: Capable of pausing its execution for a specified duration (`Sleep` action) or marking its current task as complete (`MarkComplete`), after which it will await new external events to resume activity.
*   **Monitoring Dashboard**: Hosts a local web server to provide a real-time dashboard displaying logs of all executed actions and LLM calls, including their inputs, outputs, and durations.

#### Inputs

*   **GitHub Repository**: Reads code content, commit history, and issue data (title, body, state, comments, labels) from the specified GitHub repository.
*   **Large Language Model (LLM)**: Receives text responses from the configured LLM based on system and user prompts.
*   **Environment Variables**: Configuration parameters are loaded from environment variables, potentially supplemented by a `.env` file.

#### Outputs

*   **GitHub Actions**: Performs actions such as creating new issues, updating existing issues (title, body, labels), adding comments, and closing issues.
*   **Shell Command Results**: Provides standard output and error from executed shell commands.
*   **LLM Interactions**: Sends detailed prompts to the LLM and processes the generated text responses.
*   **Internal Memory**: Updates its internal key-value memory store.
*   **Monitoring Dashboard (HTTP)**: Serves a web interface accessible via HTTP on port 5005, displaying logs of its operations.

#### Environment Variables

*   **`GITHUB_PERSONAL_ACCESS_TOKEN`**: (Required) A GitHub Personal Access Token with necessary permissions to interact with the repository.
*   **`OPENAI_API_KEY`** / **`OPENAI_KEY`**: (Required) An API key for authenticating with the OpenAI API.
*   **`OPENAI_API_BASE`**: (Optional) The base URL for the OpenAI API. Defaults to `https://api.openai.com`.
*   **`OPENAI_API_MODEL`**: (Optional) The name of the OpenAI model to use for LLM inference. Defaults to `gpt-3.5-turbo`.
*   **`GITHUB_REPOSITORY_OWNER`**: (Required) The GitHub username or organization name that owns the target repository.
*   **`GITHUB_REPOSITORY_NAME`**: (Required) The name of the GitHub repository the agent will manage.
*   **`GITHUB_REPOSITORY_ISSUES_BRANCH`**: (Optional) The name of the Git branch where the agent will manage issues. Defaults to `issues`.