use crate::config::Config;
use anyhow::{Context, Result};
use chrono::{DateTime, Utc}; // Use chrono directly
use git2::Repository;
use octocrab::Octocrab;
use octocrab::models::{IssueState, issues::Comment};
use octocrab::params::Direction;
use octocrab::params::issues::Sort::{self, Updated}; // For sorting issues
use std::fmt::Display;
use tempfile::TempDir;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Issue {
    pub number: u64,
    pub title: String,
    pub body: String,
    pub state: String,
    pub updated_at: DateTime<Utc>,
    pub labels: Vec<String>,
    pub comments: Vec<(String, String)>, // (author, comment_body)
    pub comments_count: usize,           // Added to track the number of comments
}

impl Display for Issue {
    /// Formats the issue for adding to the context prompt.
    /// We add all the relevant fields to the prompt.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted_comments: Vec<String> = self
            .comments
            .iter()
            .map(|(author, body)| format!("  - {}: {}", author, body))
            .collect();

        write!(
            f,
            "Issue #{}: {}\nState: {}\nUpdated at: {}\nLabels: {:?}\nBody: {}\nComments and Updates:\n{}",
            self.number,
            self.title,
            self.state,
            self.updated_at.to_rfc3339(),
            self.labels,
            self.body,
            formatted_comments.join("\n")
        )
    }
}

pub(crate) struct GitHubClient {
    pub octocrab: Octocrab,
    repo_owner: String,
    repo_name: String,
    access_token: String,
}

impl GitHubClient {
    pub(crate) async fn new(config: &Config) -> Result<Self> {
        let octocrab_with_token = Octocrab::builder()
            .personal_token(config.github_personal_access_token.clone())
            .build()
            .map_err(|e| anyhow::anyhow!("Failed to create Octocrab client with token: {}", e))?;

        println!("Octocrab client with personal access token created successfully.");

        Ok(Self {
            octocrab: octocrab_with_token,
            repo_owner: config.github_repository_owner.clone(),
            repo_name: config.github_repository_name.clone(),
            access_token: config.github_personal_access_token.clone(),
        })
    }

    pub(crate) async fn clone_repository(&self) -> Result<(TempDir, Repository)> {
        let token = self.access_token.clone();
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

    pub async fn get_issue_comments(&self, issue_number: u64) -> Result<Vec<Comment>> {
        let mut all_comments = Vec::new();
        let mut page = 1u32;

        loop {
            println!("Fetching comments for issue #{} on page {}", issue_number, page);
            let current_page = page;
            let comments_page = self.
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
                        ))?;

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

    pub async fn list_all_issues(&self, state: Option<String>) -> Result<Vec<Issue>> {
        let mut all_issues = Vec::new();

        match state.as_deref() {
            Some("open") => {
                let mut page = 1u32;
                loop {
                    let current_page = page;
                    let issue_page = self.octocrab
                                .issues(&self.repo_owner, &self.repo_name)
                                .list()
                                .state(octocrab::params::State::Open)
                                .sort(Sort::Created)
                                .direction(Direction::Descending)
                                .per_page(100)
                                .page(current_page)
                                .send()
                                .await
                                .context(format!(
                                    "Failed to list open issues (page {})",
                                    current_page
                                ))?;

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
                    let issue_page = self.octocrab
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
                                ))?;

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
                    let issue_page = self.octocrab
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
                                ))?;

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
                    let issue_page = self.octocrab
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
                                ))?;

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

        println!("Fetched {} items (issues and pull requests)", all_issues.len());
        // Filter out pull requests and convert octocrab::models::Issue to our Issue struct
        let filtered_issues = all_issues
            .into_iter()
            .filter(|item| item.pull_request.is_none()) // Filter out pull requests
            .map(|item| Issue {
                number: item.number,
                title: item.title,
                body: item.body.unwrap_or_default(),
                state: match item.state {
                    IssueState::Open => "open".to_string(),
                    IssueState::Closed => "closed".to_string(),
                    _ => "unknown".to_string(),
                },
                updated_at: item.updated_at,
                labels: item.labels.iter().map(|l| l.name.clone()).collect(),
                comments: vec![],
                comments_count: item.comments as usize,
            })
            .collect::<Vec<Issue>>();

        println!("Parsed {} issues from the response.", filtered_issues.len());

        Ok(filtered_issues)
    }

    pub(crate) async fn get_issue(&self, issue_number: u64) -> anyhow::Result<Issue> {
        let issue = self.octocrab
                .issues(&self.repo_owner, &self.repo_name)
                .get(issue_number)
                .await
                .context(format!("Failed to get issue #{}", issue_number))?;
            let comments = self.get_issue_comments(issue_number).await?;
            let comments_and_updates = comments
                .iter()
                .map(|c| (c.user.login.clone(), c.body.clone().unwrap_or_default()))
                .collect::<Vec<(String, String)>>();

            Ok(Issue {
                number: issue.number,
                title: issue.title,
                body: issue.body.unwrap_or_default(),
                state: match issue.state {
                    IssueState::Open => "open".to_string(),
                    IssueState::Closed => "closed".to_string(),
                    _ => "unknown".to_string(),
                },
                updated_at: issue.updated_at,
                labels: issue.labels.iter().map(|l| l.name.clone()).collect(),
                comments: comments_and_updates,
                comments_count: issue.comments as usize,
            })
    }

    pub(crate) async fn create_issue(
        &self,
        title: String,
        body: String,
        labels: Vec<String>,
    ) -> Result<u64> {
        let title_clone = title.clone();
        let body_clone = body.clone();
        let labels_clone = labels.clone();

            let i = self.octocrab
                .issues(&self.repo_owner, &self.repo_name)
                .create(title_clone.clone())
                .body(body_clone.clone())
                .labels(labels_clone.clone())
                .send()
                .await
                .context("Failed to create new issue")?;

            Ok(i.number)
    }

    pub(crate) async fn add_label_to_issue(
        &self,
        issue_number: u64,
        label: &str,
    ) -> Result<Vec<octocrab::models::Label>> {
        let label_vec = vec![label.to_string()];
            self.octocrab
                .issues(&self.repo_owner, &self.repo_name)
                .add_labels(issue_number, &label_vec)
                .await
                .context(format!("Failed to add label to issue #{}", issue_number))
    }

    pub(crate) async fn remove_label_from_issue(
        &self,
        issue_number: u64,
        label: &str,
    ) -> Result<Vec<octocrab::models::Label>> {
        let label_str = label.to_string();
            self.octocrab
                .issues(&self.repo_owner, &self.repo_name)
                .remove_label(issue_number, &label_str)
                .await
                .context(format!(
                    "Failed to remove label '{}' from issue #{}",
                    label_str, issue_number
                ))
    }

    pub(crate) async fn close_issue(&self, issue_number: u64) -> Result<()> {
            self.octocrab
                .issues(&self.repo_owner, &self.repo_name)
                .update(issue_number)
                .state(IssueState::Closed)
                .send()
                .await
                .context(format!("Failed to close issue #{}", issue_number))?;

            Ok(())
    }

    pub(crate) async fn comment_on_issue(&self, issue_number: u64, body: &str) -> Result<Comment> {
        let body_str = body.to_string();
            self.octocrab
                .issues(&self.repo_owner, &self.repo_name)
                .create_comment(issue_number, &body_str)
                .await
                .context(format!("Failed to comment on issue #{}", issue_number))
                .and_then(|comment| {
                    println!("Commented on issue #{}", issue_number);
                    Ok(comment)
                })
    }

    pub(crate) async fn edit_issue_body(&self, issue_number: u64, body: &str) -> Result<()> {
        let body_str = body.to_string();
            self.octocrab
                .issues(&self.repo_owner, &self.repo_name)
                .update(issue_number)
                .body(&body_str)
                .send()
                .await
                .context(format!("Failed to edit body of issue #{}", issue_number))?;
            println!("Edited body of issue #{}", issue_number);
            Ok(())
    }

    pub(crate) async fn edit_issue_title(&self, issue_number: u64, title: &str) -> Result<()> {
        let title_str = title.to_string();
            self.octocrab
                .issues(&self.repo_owner, &self.repo_name)
                .update(issue_number)
                .title(&title_str)
                .send()
                .await
                .context(format!("Failed to edit title of issue #{}", issue_number))?;
            println!("Edited title of issue #{}", issue_number);
            Ok(())
    }
}
