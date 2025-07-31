### Agent Name
Pristine

### Description
Pristine is an autonomous AI agent designed to manage and maintain issues within a GitHub repository. It continuously monitors the specified repository for new commits, issue updates, and other relevant events. Leveraging a Large Language Model (LLM) for reasoning, Pristine analyzes the repository's state to identify potential issues such as missing documentation, code bugs, or lacking unit tests. Based on its analysis, it can create, update, or close GitHub issues, manage labels, prioritize tasks, and respond to human comments.

### Key Features
*   **Intelligent Issue Management:** Detects and creates focused, actionable issues for documentation gaps, code bugs, and missing tests.
*   **GitHub Integration:** Directly interacts with the GitHub API to perform comprehensive issue management, including creation, retrieval, updates, labeling, commenting, and closing. It also clones and pulls the target repository to access code and commit history.
*   **LLM-Powered Reasoning:** Utilizes an OpenAI-compatible LLM to process contextual information, generate "thoughts" on necessary actions, and formulate detailed issue descriptions or comments.
*   **Contextual Memory:** Maintains an internal memory to store and retrieve important information, such as code snippets, issue summaries, and user preferences, ensuring continuity and informed decision-making across its operations.
*   **Autonomous Operation:** Runs continuously in the background, automatically checking for events and executing actions based on its LLM-driven reasoning.
*   **Web Monitoring Dashboard:** Provides a local web interface accessible on port 5005 to visualize its action history and LLM call logs, offering transparency into its operations.

### Inputs
*   **GitHub Repository:** Codebase, commit history, existing issues, comments, and pull requests (accessed via cloning and GitHub API).
*   **Internal Memory:** Stored key-value pairs representing learned context and persistent information.
*   **Previous Action Outputs:** Results and feedback from previously executed actions, influencing subsequent decisions.

### Outputs
*   **GitHub API Calls:** Creates new issues, updates existing issue titles/bodies, adds/removes labels, posts comments, and closes issues.
*   **LLM Inference Calls:** Sends prompts and receives generated text from the configured Large Language Model.
*   **Command Execution Results:** Returns the standard output and error from shell commands executed within the cloned repository.
*   **Internal Memory Updates:** Stores new or updates existing contextual information for future use.
*   **Web Dashboard (HTTP):** Serves a monitoring interface on port 5005, displaying action and LLM call logs.

### Configuration
The agent is configured using environment variables, which can be loaded from a `.env` file:
*   **`GITHUB_PERSONAL_ACCESS_TOKEN`**: Required for authenticating with the GitHub API.
*   **`OPENAI_API_KEY`**: Required for authenticating with the OpenAI API.
*   **`OPENAI_API_BASE`**: (Optional) Specifies the base URL for the OpenAI API. Defaults to `https://api.openai.com`.
*   **`OPENAI_API_MODEL`**: (Optional) Specifies the name of the OpenAI model to use for LLM inference. Defaults to `gpt-3.5-turbo`.
*   **`GITHUB_REPOSITORY_OWNER`**: Required. Specifies the owner (username or organization) of the GitHub repository to be managed.
*   **`GITHUB_REPOSITORY_NAME`**: Required. Specifies the name of the GitHub repository to be managed.
*   **`GITHUB_REPOSITORY_ISSUES_BRANCH`**: (Optional) Specifies the branch within the GitHub repository used for issues. Defaults to `issues`.
*   **`OPENAI_KEY`**: (Alternative/Fallback) An additional environment variable checked for the OpenAI API key.