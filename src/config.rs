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

    pub github_repository_owner: String,
    pub github_repository_name: String,
    pub github_repository_issues_branch: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            gh_app_id: "1429200".to_string(),
            gh_app_client_id: "Iv23livtKwE8vUQDqAPB".to_string(),
            gh_app_client_secret: "7dc2d035f4b25ec58049dfde4ab59334a8463f1e".to_string(),
            gh_app_private_key: include_str!("../.gh_pk").trim().to_string(),

            openai_api_key: std::env::var("OPENAI_API_KEY")
                .map_err(|e| anyhow::anyhow!("Failed to read OPENAI_API_KEY: {}", e))?,
            openai_api_base: std::env::var("OPENAI_API_BASE")
                .unwrap_or_else(|_| "https://api.openai.com".to_string()),
            openai_api_type: "openai".to_string(),
            openai_api_model: std::env::var("OPENAI_API_MODEL")
                .unwrap_or_else(|_| "gpt-3.5-turbo".to_string()),

            github_repository_owner: std::env::var("GITHUB_REPOSITORY_OWNER")
                .map_err(|e| anyhow::anyhow!("Failed to read GITHUB_REPOSITORY_OWNER: {}", e))?,
            github_repository_name: std::env::var("GITHUB_REPOSITORY_NAME")
                .map_err(|e| anyhow::anyhow!("Failed to read GITHUB_REPOSITORY_NAME: {}", e))?,
            github_repository_issues_branch: std::env::var("GITHUB_REPOSITORY_ISSUES_BRANCH")
                .unwrap_or_else(|_| "issues".to_string()),
        })
    }
}
