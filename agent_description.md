### Agent Name
Pristine Agent

### Description
The Pristine Agent is an autonomous AI system designed to manage GitHub repository issues. It continuously monitors a specified GitHub repository for new events, leverages a Large Language Model (LLM) to analyze the repository's state and decide on appropriate actions, and then executes those actions on GitHub. The agent maintains an internal memory for contextual information and provides a real-time web-based monitoring dashboard to track its activities.

### Key Features
*   **Issue Management**: Automatically creates, updates, closes, labels, and comments on GitHub issues.
*   **Codebase Monitoring**: Monitors the connected GitHub repository for new commits and pulls the latest changes.
*   **Intelligent Decision-Making**: Uses a Large Language Model to process repository context, past events, and internal memories to formulate a "thought" and generate a sequence of actions.
*   **Contextual Memory**: Stores and retrieves key-value pair memories to maintain continuity and track important information relevant to its tasks.
*   **Proactive Issue Identification**: Identifies potential issues such as documentation gaps, code TODOs, logic flaws, and areas for testing improvements, creating new issues as needed.
*   **Issue Prioritization & Deduplication**: Prioritizes issues based on importance and urgency, and aims to prevent the creation of duplicate issues.
*   **Real-time Monitoring Dashboard**: Provides a web interface on port `5005` to view the history of all executed actions and LLM calls.

### Inputs
*   **GitHub Repository Events**: Automatically detects and processes new commits, new issues, and updates/comments on existing issues.
*   **Environment Variables**: Configuration is loaded from environment variables.

### Outputs
*   **GitHub Actions**: Modifies the GitHub repository by creating, updating, or closing issues; adding or removing labels; and posting comments.
*   **LLM Interactions**: Sends system and user prompts to the configured Large Language Model and receives generated text responses.
*   **Web Dashboard (HTTP)**: Serves a monitoring dashboard on `http://0.0.0.0:5005` to display logs of agent actions and LLM calls.

### Configuration
The agent requires the following environment variables to be set:
*   `OPENAI_API_KEY`: Your API key for authenticating with the OpenAI API.
*   `GITHUB_REPOSITORY_OWNER`: The GitHub username or organization that owns the repository.
*   `GITHUB_REPOSITORY_NAME`: The name of the GitHub repository the agent will manage.

Optional environment variables:
*   `OPENAI_API_BASE`: The base URL for the OpenAI API (defaults to `https://api.openai.com`).
*   `OPENAI_API_MODEL`: The OpenAI model to use for LLM inference (defaults to `gpt-3.5-turbo`).
*   `GITHUB_REPOSITORY_ISSUES_BRANCH`: The name of the branch the agent will use for managing issues (defaults to `issues`).