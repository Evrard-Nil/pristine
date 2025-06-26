#[derive(Clone)] // Add Clone trait
#[allow(dead_code)]
pub struct Config {
    pub gh_app_id: String,
    pub gh_app_client_id: String,
    pub gh_app_client_secret: String,
    pub gh_app_private_key: String,

    pub openai_api_key: String,
    pub openai_api_base: String,
    pub openai_api_type: String,
    pub openai_api_model: String,

    pub issues_per_day: u32,
    pub github_repository_owner: String,
    pub github_repository_name: String,
    pub github_repository_issues_branch: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            gh_app_id: std::env::var("GH_APP_ID")
                .map_err(|e| anyhow::anyhow!("Failed to read GH_APP_ID: {}", e))?,
            gh_app_client_id: std::env::var("GH_APP_CLIENT_ID")
                .map_err(|e| anyhow::anyhow!("Failed to read GH_APP_CLIENT_ID: {}", e))?,
            gh_app_client_secret: std::env::var("GH_APP_CLIENT_SECRET")
                .map_err(|e| anyhow::anyhow!("Failed to read GH_APP_CLIENT_SECRET: {}", e))?,
            gh_app_private_key: std::env::var("GH_APP_PRIVATE_KEY")
                .map_err(|e| anyhow::anyhow!("Failed to read GH_APP_PRIVATE_KEY: {}", e))?,

            openai_api_key: std::env::var("OPENAI_API_KEY")
                .map_err(|e| anyhow::anyhow!("Failed to read OPENAI_API_KEY: {}", e))?,
            openai_api_base: std::env::var("OPENAI_API_BASE")
                .unwrap_or_else(|_| "https://api.openai.com".to_string()),
            openai_api_type: std::env::var("OPENAI_API_TYPE")
                .unwrap_or_else(|_| "openai".to_string()),
            openai_api_model: std::env::var("OPENAI_API_MODEL")
                .unwrap_or_else(|_| "gpt-3.5-turbo".to_string()),

            issues_per_day: std::env::var("ISSUES_PER_DAY")
                .map_err(|e| anyhow::anyhow!("Failed to read ISSUES_PER_DAY: {}", e))
                .and_then(|s| {
                    s.parse::<u32>().map_err(|e| {
                        anyhow::anyhow!("Failed to parse ISSUES_PER_DAY as u32: {}", e)
                    })
                })
                .unwrap_or(10),
            github_repository_owner: std::env::var("GITHUB_REPOSITORY_OWNER")
                .map_err(|e| anyhow::anyhow!("Failed to read GITHUB_REPOSITORY_OWNER: {}", e))?,
            github_repository_name: std::env::var("GITHUB_REPOSITORY_NAME")
                .map_err(|e| anyhow::anyhow!("Failed to read GITHUB_REPOSITORY_NAME: {}", e))?,
            github_repository_issues_branch: std::env::var("GITHUB_REPOSITORY_ISSUES_BRANCH")
                .unwrap_or_else(|_| "issues".to_string()),
        })
    }
}
