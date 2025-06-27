# Pristine

Pristine is a sophisticated Rust-based agent application designed for autonomous repository management and intelligent issue handling on GitHub. It operates with a modular and extensible architecture, integrating Large Language Models (LLMs) for decision-making and providing a real-time monitoring dashboard.

## Features

-   **Autonomous Agent Core**: The `Agent` module (`src/agent.rs`) forms the brain of the application. It continuously monitors for new events (commits, issues, comments) in a GitHub repository, processes them, and decides on subsequent actions. This core loop involves:
    -   **Event Detection**: Automatically identifies new commits, issues, and comments.
    -   **Contextual Thinking**: Utilizes an LLM to analyze the current state, past actions, and events to formulate a "thought" process.
    -   **Action Decision**: Based on the "thought," the LLM decides on a sequence of actions to perform.
    -   **Action Execution**: Executes the chosen actions, which can range from interacting with GitHub to managing internal memories or running further LLM inferences.

-   **GitHub Integration**: The `GitHubClient` (`src/github.rs`) provides comprehensive interaction with the GitHub API.
    -   **Secure Authentication**: Uses GitHub Apps for secure authentication, handling JWT and installation access tokens, with automatic token refreshing.
    -   **Repository Management**: Clones and pulls the target repository, ensuring the agent always works with the latest code.
    -   **Issue Management**: Full CRUD operations for GitHub issues (create, list, get, close), including adding/removing labels, commenting, and editing issue titles/bodies.

-   **Large Language Model (LLM) Integration**: The `LlmClient` (`src/llm.rs`) facilitates communication with LLMs (e.g., OpenAI).
    -   **Prompt Engineering**: Constructs system and user prompts based on the agent's responsibilities and current context.
    -   **Inference Execution**: Sends prompts to the LLM and processes the generated responses, which drive the agent's thinking and decision-making.

-   **Modular Actions System**: The `Actions` enum (`src/actions.rs`) defines a rich set of capabilities the agent can perform. These include:
    -   Repository I/O (reading code, listing files).
    -   Context Management (storing and retrieving memories).
    -   GitHub operations (issue creation, listing, commenting, labeling).
    -   LLM inference calls.
    -   Utility actions like `Sleep`.
    This modularity allows for easy expansion of the agent's capabilities.

-   **Configuration Management**: The `Config` module (`src/config.rs`) handles all application settings.
    -   **Environment-based**: Loads configurations from environment variables, making deployment flexible and secure.
    -   Includes GitHub App credentials, OpenAI API settings, and target repository details.

-   **Real-time Monitoring Dashboard**: The `WebServer` (`src/web_server.rs`) and `Monitor` (`src/monitoring.rs`) modules provide visibility into the agent's operations.
    -   **Action Logging**: Records every action taken by the agent, including its result and duration.
    -   **LLM Call Logging**: Logs all interactions with the LLM, capturing prompts, responses, and duration.
    -   **Web Interface**: A simple HTTP server serves an HTML dashboard that displays the action and LLM call history in real-time, aiding in debugging and understanding agent behavior.

-   **Repository Management**: The `RepositoryManager` (`src/repository.rs`) handles local Git operations.
    -   **Branch Management**: Ensures the agent operates on a specified issues branch, or the default branch if none is specified.
    -   **Commit Detection**: Detects new commits to trigger agent re-evaluation.
    -   **Code Reading**: Provides functionality to read all code or specific files from the cloned repository for LLM context.

## Implementation Design

The Pristine agent follows a clear separation of concerns, with distinct modules handling specific functionalities:

-   **`main.rs` (in `bin/run.rs`)**: The entry point, responsible for initializing the `Config`, `Agent`, and `WebServer`, then starting their respective loops.
-   **`Agent` (`src/agent.rs`)**: Orchestrates the entire process. It maintains the agent's internal state (memories, past actions/events), interacts with `GitHubClient`, `LlmClient`, and `RepositoryManager`, and implements the core `think-decide-act` loop. It's designed to be event-driven, reacting to changes in the GitHub repository.
-   **`Actions` (`src/actions.rs`)**: A central enum that defines the agent's vocabulary. This enum is serialized to JSON for LLM output, allowing the LLM to directly specify the actions to be taken. This design provides a clear interface between the LLM's reasoning and the agent's execution capabilities.
-   **`GitHubClient` (`src/github.rs`)**: Encapsulates all GitHub API logic, including authentication, token refreshing, and error handling with retry mechanisms. It abstracts away the complexities of GitHub interactions from the core agent logic.
-   **`LlmClient` (`src/llm.rs`)**: Manages communication with the LLM provider. It's responsible for formatting prompts, making API calls, and parsing responses. It also integrates with the `Monitor` for logging LLM interactions.
-   **`RepositoryManager` (`src/repository.rs`)**: Handles all local Git operations, ensuring the agent has access to an up-to-date copy of the repository and can read its contents efficiently.
-   **`Monitor` (`src/monitoring.rs`)**: A thread-safe data store for logging agent actions and LLM calls. It uses `Arc<Mutex>` to allow multiple parts of the application to log data concurrently.
-   **`WebServer` (`src/web_server.rs`)**: A lightweight HTTP server that exposes the `Monitor`'s data through a simple web dashboard, providing a visual interface for observing the agent's behavior.

The application leverages Rust's strong type system and concurrency features (via `tokio`) to build a robust and performant autonomous agent. Error handling is managed using `anyhow` for consistent error propagation.
