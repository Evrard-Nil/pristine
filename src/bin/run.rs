use pristine::agent::Agent;
use pristine::config::Config;
use pristine::web_server::WebServer;

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv_override();
    let config =
        Config::from_env().expect("Failed to load configuration from environment variables");
    let agent = Agent::new(&config).await.expect("Failed to create agent");

    // Get the monitor from the agent and start the web server
    let monitor = agent.get_monitor();
    let web_server = WebServer::new(monitor, 5000);

    // Start the web server in the background
    web_server
        .start()
        .await
        .expect("Failed to start web server");

    // Start the agent
    agent.start().await
}
