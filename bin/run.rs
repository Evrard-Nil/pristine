#[tokio::main]
async fn main() {
    dotenvy::dotenv_override().expect("Failed to load environment variables from .env file");
    let config = pristine::config::Config::from_env()
        .expect("Failed to load configuration from environment variables");
    let agent = pristine::Agent::new(config)
        .await
        .expect("Failed to create agent");
    agent.start().await
}
