use crate::config::Config;
use anyhow::{Context, Result};
use chrono::{DateTime, Utc}; // Use chrono directly
use git2::Repository;
use octocrab::Octocrab;
use octocrab::models::{
    AppId, InstallationToken, IssueState,
    issues::{Comment, Issue},
};
use octocrab::params::Direction;
use octocrab::params::apps::CreateInstallationAccessToken;
use octocrab::params::issues::Sort::Updated; // For sorting issues
use serde_json;
use std::sync::Arc;
use tempfile::TempDir;
use tokio::sync::Mutex;
use url::Url;

pub struct GitHubClient {
    pub octocrab: Arc<Mutex<Octocrab>>,
    repo_owner: String,
    repo_name: String,
    access_token: Arc<Mutex<String>>,
    app_id: AppId,
    app_private_key: String,
    installation_id: u64,
}

impl GitHubClient {
    pub async fn new(config: &Config) -> Result<Self> {
        let app_id = config
            .gh_app_id
            .parse::<u64>()
            .map(AppId)
            .map_err(|e| anyhow::anyhow!("Invalid GH_APP_ID: {}", e))?;

        let key = jsonwebtoken::EncodingKey::from_rsa_pem(config.gh_app_private_key.as_bytes())
            .map_err(|e| anyhow::anyhow!("Failed to parse GH_APP_PRIVATE_KEY: {}", e))?;

        let initial_octocrab = Octocrab::builder()
            .app(app_id, key)
            .build()
            .map_err(|e| anyhow::anyhow!("Failed to create initial Octocrab client: {}", e))?;

        println!(
            "Initial Octocrab client created successfully for App ID: {}",
            app_id.0
        );

        let installation = initial_octocrab
            .apps()
            .get_repository_installation(
                &config.github_repository_owner,
                &config.github_repository_name,
            )
            .await
            .map_err(|e| anyhow::anyhow!("Failed to get repository installation: {}", e))?;

        println!(
            "Repository installation retrieved successfully. Installation ID: {}",
            installation.id
        );

        let mut create_access_token = CreateInstallationAccessToken::default();
        create_access_token.repositories = vec![config.github_repository_name.clone()];

        let access_token_url_str = installation
            .access_tokens_url
            .ok_or_else(|| anyhow::anyhow!("Access tokens URL not found in installation"))?;
        let access_token_url = Url::parse(&access_token_url_str)
            .map_err(|e| anyhow::anyhow!("Failed to parse access_tokens_url: {}", e))?;

        let access: InstallationToken = initial_octocrab
            .post(access_token_url.path(), Some(&create_access_token))
            .await
            .map_err(|e| anyhow::anyhow!("Failed to create installation access token: {}", e))?;

        println!("Installation access token obtained successfully.");

        let octocrab_with_token = Octocrab::builder()
            .personal_token(access.token.clone())
            .build()
            .map_err(|e| anyhow::anyhow!("Failed to create Octocrab client with token: {}", e))?;

        println!("Octocrab client with installation token created successfully.");

        Ok(Self {
            octocrab: Arc::new(Mutex::new(octocrab_with_token)),
            repo_owner: config.github_repository_owner.clone(),
            repo_name: config.github_repository_name.clone(),
            access_token: Arc::new(Mutex::new(access.token)),
            app_id,
            app_private_key: config.gh_app_private_key.clone(),
            installation_id: installation.id.0,
        })
    }

    async fn refresh_access_token(&self) -> Result<()> {
        println!("Refreshing GitHub access token...");

        let key = jsonwebtoken::EncodingKey::from_rsa_pem(self.app_private_key.as_bytes())
            .map_err(|e| anyhow::anyhow!("Failed to parse private key during refresh: {}", e))?;

        let app_octocrab = Octocrab::builder()
            .app(self.app_id, key)
            .build()
            .map_err(|e| {
                anyhow::anyhow!("Failed to create app Octocrab client during refresh: {}", e)
            })?;

        let mut create_access_token = CreateInstallationAccessToken::default();
        create_access_token.repositories = vec![self.repo_name.clone()];

        let access_token_url = format!("/app/installations/{}/access_tokens", self.installation_id);

        let access: InstallationToken = app_octocrab
            .post(&access_token_url, Some(&create_access_token))
            .await
            .map_err(|e| {
                anyhow::anyhow!("Failed to create new installation access token: {}", e)
            })?;

        println!("New installation access token obtained successfully.");

        // Update the stored access token
        let mut token_guard = self.access_token.lock().await;
        *token_guard = access.token.clone();

        // Create new octocrab client with the new token
        let new_octocrab = Octocrab::builder()
            .personal_token(access.token)
            .build()
            .map_err(|e| {
                anyhow::anyhow!("Failed to create Octocrab client with new token: {}", e)
            })?;

        // Update the octocrab client
        let mut octocrab_guard = self.octocrab.lock().await;
        *octocrab_guard = new_octocrab;

        println!("GitHub client refreshed with new access token.");
        Ok(())
    }

    async fn execute_with_retry<F, T, Fut>(&self, operation: F) -> Result<T>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<T>>,
    {
        // First attempt
        match operation().await {
            Ok(result) => Ok(result),
            Err(e) => {
                // Check if the error is due to authentication failure
                let error_str = e.to_string().to_lowercase();
                if error_str.contains("401")
                    || error_str.contains("unauthorized")
                    || error_str.contains("bad credentials")
                    || error_str.contains("authentication")
                {
                    println!("Authentication error detected: {}", e);

                    // Refresh the token
                    self.refresh_access_token().await?;

                    // Retry the operation
                    println!("Retrying operation after token refresh...");
                    operation().await
                } else {
                    // Not an authentication error, return the original error
                    Err(e)
                }
            }
        }
    }

    pub async fn list_open_issues(&self) -> Result<Vec<Issue>> {
        self.execute_with_retry(|| async {
            let octocrab = self.octocrab.lock().await;
            let issues_page = octocrab
                .issues(&self.repo_owner, &self.repo_name)
                .list()
                .state(octocrab::params::State::Open)
                .send()
                .await
                .map_err(|e| anyhow::anyhow!("Failed to list repository issues: {}", e))?;

            println!(
                "Repository issues retrieved successfully. Count: {}",
                issues_page.items.len()
            );
            Ok(issues_page.items)
        })
        .await
    }

    pub async fn clone_repository(&self) -> Result<(TempDir, Repository)> {
        let token = self.access_token.lock().await.clone();
        let clone_url = format!(
            "https://x-access-token:{}@github.com/{}/{}.git",
            token, self.repo_owner, self.repo_name
        );
        println!("Clone URL: {}", clone_url);

        let repo_dir = TempDir::new()
            .map_err(|e| anyhow::anyhow!("Failed to create temporary directory for repo: {}", e))?;

        println!(
            "Temporary repository directory created at: {}",
            repo_dir.path().display()
        );

        let repo = Repository::clone(&clone_url, repo_dir.path())
            .map_err(|e| anyhow::anyhow!("Failed to clone repository: {}", e))?;

        println!(
            "Repository cloned successfully into {}",
            repo_dir.path().display()
        );
        Ok((repo_dir, repo))
    }

    #[allow(dead_code)]
    pub async fn find_archivable_issues(
        &self,
        inactivity_days: u64,
        stale_label: Option<String>,
    ) -> Result<Vec<Issue>> {
        let mut archivable_issues = Vec::new();
        let mut page = 1u32;

        let now: DateTime<Utc> = Utc::now();
        let inactivity_threshold_date: DateTime<Utc> =
            now - chrono::Duration::days(inactivity_days as i64);

        println!(
            "Searching for issues with no activity since {} ({} days ago)",
            inactivity_threshold_date.to_rfc3339(),
            inactivity_days
        );

        loop {
            let current_page = page;
            let label_clone = stale_label.clone();

            let issue_page = self
                .execute_with_retry(|| async {
                    let octocrab = self.octocrab.lock().await;
                    if let Some(label_str) = &label_clone {
                        println!("Filtering by label: {}", label_str);
                        let labels_param = vec![label_str.clone()];
                        octocrab
                            .issues(&self.repo_owner, &self.repo_name)
                            .list()
                            .state(octocrab::params::State::Open)
                            .sort(Updated)
                            .direction(Direction::Ascending)
                            .per_page(100)
                            .page(current_page)
                            .labels(&labels_param)
                            .send()
                            .await
                    } else {
                        octocrab
                            .issues(&self.repo_owner, &self.repo_name)
                            .list()
                            .state(octocrab::params::State::Open)
                            .sort(Updated)
                            .direction(Direction::Ascending)
                            .per_page(100)
                            .page(current_page)
                            .send()
                            .await
                    }
                    .context(format!(
                        "Failed to list open issues for archival check (page {})",
                        current_page
                    ))
                })
                .await?;

            if issue_page.items.is_empty() {
                break;
            }

            for issue in issue_page.items {
                let updated_at_value = issue.updated_at;

                if updated_at_value < inactivity_threshold_date {
                    println!(
                        "  Found potential candidate for archival: Issue #{} - '{}', last updated: {}",
                        issue.number,
                        issue.title,
                        updated_at_value.to_rfc3339()
                    );
                    archivable_issues.push(issue);
                }
            }

            if issue_page.next.is_none() {
                break;
            }
            page += 1;
        }

        println!(
            "Found {} potential issues for archival.",
            archivable_issues.len()
        );
        Ok(archivable_issues)
    }

    pub async fn delete_issue(&self, issue_number: u64) -> Result<()> {
        self.execute_with_retry(|| async {
            let octocrab = self.octocrab.lock().await;
            octocrab
                .issues(&self.repo_owner, &self.repo_name)
                .update(issue_number)
                .state(IssueState::Closed)
                .send()
                .await
                .context(format!("Failed to close issue #{}", issue_number))?;
            Ok(())
        })
        .await
    }

    pub async fn get_issue_comments(&self, issue_number: u64) -> Result<Vec<Comment>> {
        let mut all_comments = Vec::new();
        let mut page = 1u32;

        loop {
            let current_page = page;
            let comments_page = self
                .execute_with_retry(|| async {
                    let octocrab = self.octocrab.lock().await;
                    octocrab
                        .issues(&self.repo_owner, &self.repo_name)
                        .list_comments(issue_number)
                        .per_page(100)
                        .page(current_page)
                        .send()
                        .await
                        .context(format!(
                            "Failed to list comments for issue #{}",
                            issue_number
                        ))
                })
                .await?;

            if comments_page.items.is_empty() {
                break;
            }

            all_comments.extend(comments_page.items);

            if comments_page.next.is_none() {
                break;
            }
            page += 1;
        }

        Ok(all_comments)
    }

    pub async fn add_comment_to_issue(&self, issue_number: u64, body: &str) -> Result<()> {
        let body_clone = body.to_string();
        self.execute_with_retry(|| async {
            let octocrab = self.octocrab.lock().await;
            octocrab
                .issues(&self.repo_owner, &self.repo_name)
                .create_comment(issue_number, &body_clone)
                .await
                .context(format!("Failed to add comment to issue #{}", issue_number))?;
            println!("Added comment to issue #{}", issue_number);
            Ok(())
        })
        .await
    }

    pub async fn update_issue(
        &self,
        issue_number: u64,
        title: Option<String>,
        body: Option<String>,
        labels: Option<Vec<String>>,
    ) -> Result<()> {
        // Build the update request with the provided fields
        let mut update_request = serde_json::json!({});

        if let Some(title) = title {
            update_request["title"] = serde_json::json!(title);
        }

        if let Some(body) = body {
            update_request["body"] = serde_json::json!(body);
        }

        if let Some(labels) = labels {
            update_request["labels"] = serde_json::json!(labels);
        }

        // Use the PATCH endpoint directly
        let route = format!(
            "/repos/{}/{}/issues/{}",
            self.repo_owner, self.repo_name, issue_number
        );

        self.execute_with_retry(|| async {
            let octocrab = self.octocrab.lock().await;
            octocrab
                ._patch(route.clone(), Some(&update_request))
                .await
                .context(format!("Failed to update issue #{}", issue_number))?;

            println!("Updated issue #{}", issue_number);
            Ok(())
        })
        .await
    }

    pub async fn list_all_issues(&self, state: Option<String>) -> Result<Vec<Issue>> {
        let mut all_issues = Vec::new();

        match state.as_deref() {
            Some("open") => {
                let mut page = 1u32;
                loop {
                    let current_page = page;
                    let issue_page = self
                        .execute_with_retry(|| async {
                            let octocrab = self.octocrab.lock().await;
                            octocrab
                                .issues(&self.repo_owner, &self.repo_name)
                                .list()
                                .state(octocrab::params::State::Open)
                                .sort(Updated)
                                .direction(Direction::Descending)
                                .per_page(100)
                                .page(current_page)
                                .send()
                                .await
                                .context(format!(
                                    "Failed to list open issues (page {})",
                                    current_page
                                ))
                        })
                        .await?;

                    if issue_page.items.is_empty() {
                        break;
                    }

                    all_issues.extend(issue_page.items);

                    if issue_page.next.is_none() {
                        break;
                    }
                    page += 1;
                }
            }
            Some("closed") => {
                let mut page = 1u32;
                loop {
                    let current_page = page;
                    let issue_page = self
                        .execute_with_retry(|| async {
                            let octocrab = self.octocrab.lock().await;
                            octocrab
                                .issues(&self.repo_owner, &self.repo_name)
                                .list()
                                .state(octocrab::params::State::Closed)
                                .sort(Updated)
                                .direction(Direction::Descending)
                                .per_page(100)
                                .page(current_page)
                                .send()
                                .await
                                .context(format!(
                                    "Failed to list closed issues (page {})",
                                    current_page
                                ))
                        })
                        .await?;

                    if issue_page.items.is_empty() {
                        break;
                    }

                    all_issues.extend(issue_page.items);

                    if issue_page.next.is_none() {
                        break;
                    }
                    page += 1;
                }
            }
            Some("all") | None => {
                // Fetch open issues
                let mut page = 1u32;
                loop {
                    let current_page = page;
                    let issue_page = self
                        .execute_with_retry(|| async {
                            let octocrab = self.octocrab.lock().await;
                            octocrab
                                .issues(&self.repo_owner, &self.repo_name)
                                .list()
                                .state(octocrab::params::State::Open)
                                .sort(Updated)
                                .direction(Direction::Descending)
                                .per_page(100)
                                .page(current_page)
                                .send()
                                .await
                                .context(format!(
                                    "Failed to list open issues (page {})",
                                    current_page
                                ))
                        })
                        .await?;

                    if issue_page.items.is_empty() {
                        break;
                    }

                    all_issues.extend(issue_page.items);

                    if issue_page.next.is_none() {
                        break;
                    }
                    page += 1;
                }

                // Fetch closed issues
                page = 1;
                loop {
                    let current_page = page;
                    let issue_page = self
                        .execute_with_retry(|| async {
                            let octocrab = self.octocrab.lock().await;
                            octocrab
                                .issues(&self.repo_owner, &self.repo_name)
                                .list()
                                .state(octocrab::params::State::Closed)
                                .sort(Updated)
                                .direction(Direction::Descending)
                                .per_page(100)
                                .page(current_page)
                                .send()
                                .await
                                .context(format!(
                                    "Failed to list closed issues (page {})",
                                    current_page
                                ))
                        })
                        .await?;

                    if issue_page.items.is_empty() {
                        break;
                    }

                    all_issues.extend(issue_page.items);

                    if issue_page.next.is_none() {
                        break;
                    }
                    page += 1;
                }
            }
            _ => return Err(anyhow::anyhow!("Invalid state parameter")),
        }

        println!("Fetched {} issues", all_issues.len());
        Ok(all_issues)
    }

    pub(crate) async fn get_issue(&self, issue_number: u64) -> anyhow::Result<Issue> {
        self.execute_with_retry(|| async {
            let octocrab = self.octocrab.lock().await;
            octocrab
                .issues(&self.repo_owner, &self.repo_name)
                .get(issue_number)
                .await
                .context(format!("Failed to get issue #{}", issue_number))
        })
        .await
    }

    pub(crate) async fn create_issue(
        &self,
        title: String,
        body: String,
        labels: Vec<String>,
    ) -> Result<Issue> {
        let title_clone = title.clone();
        let body_clone = body.clone();
        let labels_clone = labels.clone();

        self.execute_with_retry(|| async {
            let octocrab = self.octocrab.lock().await;
            octocrab
                .issues(&self.repo_owner, &self.repo_name)
                .create(title_clone.clone())
                .body(body_clone.clone())
                .labels(labels_clone.clone())
                .send()
                .await
                .context("Failed to create new issue")
        })
        .await
    }

    pub(crate) async fn add_label_to_issue(
        &self,
        issue_number: u64,
        label: &str,
    ) -> Result<Vec<octocrab::models::Label>> {
        let label_vec = vec![label.to_string()];
        self.execute_with_retry(|| async {
            let octocrab = self.octocrab.lock().await;
            octocrab
                .issues(&self.repo_owner, &self.repo_name)
                .add_labels(issue_number, &label_vec)
                .await
                .context(format!("Failed to add label to issue #{}", issue_number))
        })
        .await
    }

    pub(crate) async fn remove_label_from_issue(
        &self,
        issue_number: u64,
        label: &str,
    ) -> Result<Vec<octocrab::models::Label>> {
        let label_str = label.to_string();
        self.execute_with_retry(|| async {
            let octocrab = self.octocrab.lock().await;
            octocrab
                .issues(&self.repo_owner, &self.repo_name)
                .remove_label(issue_number, &label_str)
                .await
                .context(format!(
                    "Failed to remove label '{}' from issue #{}",
                    label_str, issue_number
                ))
        })
        .await
    }

    pub(crate) async fn close_issue(&self, issue_number: u64) -> Result<()> {
        self.execute_with_retry(|| async {
            let octocrab = self.octocrab.lock().await;
            octocrab
                .issues(&self.repo_owner, &self.repo_name)
                .update(issue_number)
                .state(IssueState::Closed)
                .send()
                .await
                .context(format!("Failed to close issue #{}", issue_number))?;

            Ok(())
        })
        .await
    }

    pub(crate) async fn comment_on_issue(&self, issue_number: u64, body: &str) -> Result<Comment> {
        let body_str = body.to_string();
        self.execute_with_retry(|| async {
            let octocrab = self.octocrab.lock().await;
            octocrab
                .issues(&self.repo_owner, &self.repo_name)
                .create_comment(issue_number, &body_str)
                .await
                .context(format!("Failed to comment on issue #{}", issue_number))
                .and_then(|comment| {
                    println!("Commented on issue #{}", issue_number);
                    Ok(comment)
                })
        })
        .await
    }

    pub(crate) async fn edit_issue_body(&self, issue_number: u64, body: &str) -> Result<()> {
        let body_str = body.to_string();
        self.execute_with_retry(|| async {
            let octocrab = self.octocrab.lock().await;
            octocrab
                .issues(&self.repo_owner, &self.repo_name)
                .update(issue_number)
                .body(&body_str)
                .send()
                .await
                .context(format!("Failed to edit body of issue #{}", issue_number))?;
            println!("Edited body of issue #{}", issue_number);
            Ok(())
        })
        .await
    }

    pub(crate) async fn edit_issue_title(&self, issue_number: u64, title: &str) -> Result<()> {
        let title_str = title.to_string();
        self.execute_with_retry(|| async {
            let octocrab = self.octocrab.lock().await;
            octocrab
                .issues(&self.repo_owner, &self.repo_name)
                .update(issue_number)
                .title(&title_str)
                .send()
                .await
                .context(format!("Failed to edit title of issue #{}", issue_number))?;
            println!("Edited title of issue #{}", issue_number);
            Ok(())
        })
        .await
    }
}
