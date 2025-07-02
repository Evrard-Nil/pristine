use std::fs;

use tempfile::TempDir;
use walkdir::WalkDir;

use crate::config::Config;

pub struct RepositoryManager {
    directory: TempDir,
    repository: git2::Repository,
    branch: String,
    last_commit_hash: String,
}
impl RepositoryManager {
    pub(crate) fn new(
        repo_dir: TempDir,
        repo: git2::Repository,
        config: &Config,
    ) -> anyhow::Result<Self> {
        let branch = if !config.github_repository_issues_branch.is_empty() {
            // Ensure the issues branch exists
            let branch_name = format!("refs/heads/{}", config.github_repository_issues_branch);
            let _branch_ref = repo.find_reference(&branch_name)?;
            repo.set_head(&branch_name)?;
            println!(
                "Switched to issues branch: {}",
                config.github_repository_issues_branch
            );
            let mut checkout_options = git2::build::CheckoutBuilder::new();
            checkout_options.force();

            repo.checkout_head(Some(&mut checkout_options))?;
            println!("Successfully checked out branch '{}'.", branch_name);
            config.github_repository_issues_branch.clone()
        } else {
            // check default branch
            let default_branch = repo.head()?.shorthand().unwrap_or("main").to_string();
            repo.set_head(&format!("refs/heads/{}", default_branch))?;
            println!("Switched to default branch: {}", default_branch);
            default_branch
        };
        let last_commit = repo.head()?.peel_to_commit()?;
        let last_commit_hash = last_commit.id().to_string();
        drop(last_commit);

        Ok(Self {
            directory: repo_dir,
            repository: repo,
            branch,
            last_commit_hash,
        })
    }

    pub fn pull(&self) -> anyhow::Result<()> {
        let mut remote = self.repository.find_remote("origin")?;

        let mut fetch_options = git2::FetchOptions::new();
        fetch_options.download_tags(git2::AutotagOption::All);
        remote.fetch(&[self.branch.clone()], Some(&mut fetch_options), None)?;

        let fetch_head = self.repository.find_reference("FETCH_HEAD")?;
        let fetch_commit = self.repository.reference_to_annotated_commit(&fetch_head)?;
        let (analysis, _) = self.repository.merge_analysis(&[&fetch_commit])?;

        if analysis.is_up_to_date() {
            println!("Already up-to-date.");
            return Ok(());
        } else if analysis.is_fast_forward() {
            println!("Performing fast-forward merge.");
            let mut reference = self
                .repository
                .find_reference(&format!("refs/heads/{}", self.branch))?;
            reference.set_target(fetch_commit.id(), "Fast-forward merge")?;
            self.repository
                .set_head(&format!("refs/heads/{}", self.branch))?;
            self.repository.checkout_head(Some(
                git2::build::CheckoutBuilder::new()
                    .force()
                    .allow_conflicts(true),
            ))?;
        } else if analysis.is_normal() {
            // Use is_normal() for a true merge
            println!("Performing merge.");
            let local_commit = self.repository.head()?.peel_to_commit()?;
            // Perform the merge
            self.repository
                .merge(&[&fetch_commit], Some(&mut git2::MergeOptions::new()), None)?;

            // Get the index after merge
            let mut merge_index = self.repository.index()?;

            if merge_index.has_conflicts() {
                return Err(anyhow::anyhow!(
                    "Conflicts detected during merge. Please resolve them manually."
                ));
            }

            let merge_tree = self
                .repository
                .find_tree(merge_index.write_tree_to(&self.repository)?)?;
            let message = format!(
                "Merge remote-tracking branch 'origin/{}' into {}",
                self.branch, self.branch
            );
            let sig = self.repository.signature()?;
            let _merge_commit = self.repository.commit(
                Some("HEAD"),
                &sig,
                &sig,
                &message,
                &merge_tree,
                &[
                    &local_commit,
                    &self.repository.find_commit(fetch_commit.id())?,
                ],
            )?;

            self.repository.checkout_head(Some(
                git2::build::CheckoutBuilder::new()
                    .force()
                    .allow_conflicts(true),
            ))?;
        } else {
            return Err(anyhow::anyhow!("Unknown merge analysis result."));
        }

        println!("Successfully pulled branch '{}'.", self.branch);
        Ok(())
    }

    pub fn new_commit(&mut self) -> bool {
        let head = match self.repository.head() {
            Ok(head) => head,
            Err(_) => return false, // No HEAD reference found
        };

        let commit = match head.peel_to_commit() {
            Ok(commit) => commit,
            Err(_) => return false, // HEAD is not a commit
        };

        let current_commit_hash = commit.id().to_string();
        if current_commit_hash == self.last_commit_hash {
            return false; // No new commit since last check
        }

        self.last_commit_hash = current_commit_hash;
        true // New commit detected
    }

    pub(crate) fn get_latest_commit(&self) -> anyhow::Result<git2::Commit> {
        let head = self.repository.head()?;
        let commit = head.peel_to_commit()?;
        Ok(commit)
    }

    pub(crate) async fn list_all_files(&self) -> anyhow::Result<Vec<String>> {
        let mut files = Vec::new();
        for entry in WalkDir::new(&self.directory.path())
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();
            if path.starts_with(self.directory.path().join(".git")) {
                continue;
            }
            if let Some(file_name) = path.file_name() {
                files.push(file_name.to_string_lossy().to_string());
            }
        }
        Ok(files)
    }

    pub(crate) async fn read_file(&self, path: &str) -> anyhow::Result<String> {
        let full_path = self.directory.path().join(path);
        if !full_path.exists() {
            return Err(anyhow::anyhow!("File not found: {}", path));
        }
        match fs::read_to_string(&full_path) {
            Ok(content) => Ok(content),
            Err(e) => Err(anyhow::anyhow!("Failed to read file {}: {}", path, e)),
        }
    }
}
