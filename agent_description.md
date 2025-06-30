### Pristine Agent

The Pristine Agent is an autonomous AI agent designed to manage GitHub repository issues. It continuously monitors a specified GitHub repository for new commits and issue updates, leveraging a Large Language Model (LLM) to analyze the repository's state, make decisions, and execute actions. Its primary goal is to maintain the health and organization of the repository's issues by creating, updating, commenting on, and closing issues as needed, while also identifying areas for improvement like documentation, testing, and potential bugs.

### Key Features

*   **Continuous Monitoring:** Automatically pulls repository changes and fetches GitHub issue updates (new issues, comments, state changes).
*   **LLM-Powered Decision Making:** Utilizes a Large Language Model for both "thinking" (analyzing context and formulating strategies) and "deciding" (selecting specific actions to execute) based on the current repository state, past events, and its internal memory.
*   **GitHub Issue Management:** Capable of a wide range of GitHub operations including:
    *   Creating new issues with specified titles, bodies, and labels.
    *   Retrieving details of existing issues.
    *   Adding and removing labels from issues.
    *   Closing issues.
    *   Adding comments to issues.
    *   Editing the title and body of existing issues.
*   **Repository Interaction:** Can list all files and read the content of specific files within the cloned repository to gather context.
*   **Internal Memory:** Maintains a persistent key-value memory store to retain important information and context across its operational cycles.
*   **Self-Correction/Adaptation:** Incorporates past actions and events into its context to inform future decisions and aims to avoid creating duplicate issues.
*   **Monitoring Dashboard:** Provides a web-based dashboard to visualize its action history and LLM call logs in real-time.

### Actions

The agent can perform the following types of actions:

*   **Repository I/O:**
    *   `ListAllFiles`: Lists all files in the repository.
    *   `ReadASingleFile`: Reads the content of a specified file.
*   **Context Management:**
    *   `StoreOrUpdateMemoryInContext`: Stores or updates a key-value memory.
    *   `RemoveMemoryFromContext`: Removes a memory by its key.
*   **GitHub Operations:**
    *   `GithubCreateIssue`: Creates a new GitHub issue.
    *   `GithubGetIssue`: Retrieves details of a specific GitHub issue.
    *   `GithubAddLabelToIssue`: Adds a label to a GitHub issue.
    *   `GithubRemoveLabelFromIssue`: Removes a label from a GitHub issue.
    *   `GithubCloseIssue`: Closes a GitHub issue.
    *   `GithubCommentOnIssue`: Adds a comment to a GitHub issue.
    *   `GithubEditBodyOfIssue`: Edits the body of a GitHub issue.
    *   `GithubEditTitleOfIssue`: Edits the title of a GitHub issue.
*   **LLM Inference:**
    *   `RunLLMInference`: Executes a general LLM inference with custom system and user prompts.
*   **Utility:**
    *   `Sleep`: Pauses the agent's execution for a specified duration.

### Inputs

*   **GitHub Repository Events:** New commits, new issues, updates to existing issues (comments, state changes, label changes).
*   **Environment Variables:** Configuration for GitHub App authentication and OpenAI API.

### Outputs

*   **GitHub Actions:** Creates, updates, comments on, and closes issues on the configured GitHub repository.
*   **Internal State:** Updates its internal memory and context based on observations and actions.
*   **Monitoring Dashboard (HTTP):** Provides a real-time web interface on port `5000` displaying action and LLM call logs.

### Environment Variables

The agent requires the following environment variables for configuration. These can also be loaded from a `.env` file.

*   `OPENAI_API_KEY`: Your OpenAI API key (mandatory).
*   `OPENAI_API_BASE`: (Optional) Base URL for the OpenAI API (defaults to `https://api.openai.com`).
*   `OPENAI_API_MODEL`: (Optional) OpenAI model name (defaults to `gpt-3.5-turbo`).
*   `OPENAI_KEY`: (Alternative to `OPENAI_API_KEY`) If set, the LLM functionality will be enabled.
*   `GITHUB_REPOSITORY_OWNER`: The owner (user or organization) of the GitHub repository (mandatory).
*   `GITHUB_REPOSITORY_NAME`: The name of the GitHub repository to manage (mandatory).
*   `GITHUB_REPOSITORY_ISSUES_BRANCH`: (Optional) The branch used for issue management (defaults to `issues`).