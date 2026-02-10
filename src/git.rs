use anyhow::{Context, Result};
use git2::Repository;
use std::process::Command;

/// Get diff text from git
/// - None: uncommitted changes (git diff HEAD)
/// - Some("HEAD"): last commit (git show HEAD)
/// - Some("abc123"): specific commit
/// - Some("main..feature"): branch diff
pub fn get_diff(git_ref: Option<&str>) -> Result<String> {
    let _repo = Repository::open(".").context("Not a git repository")?;

    let output = match git_ref {
        None => {
            // Get uncommitted changes
            Command::new("git")
                .args(&["diff", "HEAD"])
                .output()
                .context("Failed to run git diff")?
        }
        Some(ref_str) if ref_str.contains("..") => {
            // Branch diff
            Command::new("git")
                .args(&["diff", ref_str])
                .output()
                .context("Failed to run git diff")?
        }
        Some(ref_str) => {
            // Specific commit
            Command::new("git")
                .args(&["show", ref_str])
                .output()
                .context("Failed to run git show")?
        }
    };

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Git command failed: {}", error);
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
