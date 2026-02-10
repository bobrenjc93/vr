use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VcsType {
    Git,
    Mercurial,
}

impl VcsType {
    /// Auto-detect which VCS is being used in the current directory
    pub fn detect() -> Result<Self> {
        if Path::new(".git").exists() {
            Ok(VcsType::Git)
        } else if Path::new(".hg").exists() {
            Ok(VcsType::Mercurial)
        } else {
            anyhow::bail!("Not a git or mercurial repository")
        }
    }

    /// Get diff text from the detected VCS
    /// - None: uncommitted changes
    /// - Some(ref): specific commit/changeset or branch comparison
    pub fn get_diff(&self, vcs_ref: Option<&str>) -> Result<String> {
        match self {
            VcsType::Git => get_git_diff(vcs_ref),
            VcsType::Mercurial => get_hg_diff(vcs_ref),
        }
    }
}

/// Get diff text from git
/// - None: uncommitted changes (git diff HEAD)
/// - Some("HEAD"): last commit (git show HEAD)
/// - Some("abc123"): specific commit
/// - Some("main..feature"): branch diff
fn get_git_diff(git_ref: Option<&str>) -> Result<String> {
    // Verify we're in a git repository
    if !Path::new(".git").exists() {
        anyhow::bail!("Not a git repository");
    }

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

/// Get diff text from mercurial
/// - None: uncommitted changes (hg diff)
/// - Some("."): current changeset (hg export .)
/// - Some("123"): specific changeset (hg export 123)
/// - Some("branch1::branch2"): range of changesets
fn get_hg_diff(hg_ref: Option<&str>) -> Result<String> {
    // Verify we're in a mercurial repository
    if !Path::new(".hg").exists() {
        anyhow::bail!("Not a mercurial repository");
    }

    let output = match hg_ref {
        None => {
            // Get uncommitted changes
            Command::new("hg")
                .args(&["diff"])
                .output()
                .context("Failed to run hg diff")?
        }
        Some(ref_str) if ref_str.contains("::") => {
            // Range of changesets (e.g., "branch1::branch2")
            Command::new("hg")
                .args(&["log", "-p", "-r", ref_str])
                .output()
                .context("Failed to run hg log")?
        }
        Some(ref_str) => {
            // Specific changeset (export shows it as a diff)
            Command::new("hg")
                .args(&["export", ref_str])
                .output()
                .context("Failed to run hg export")?
        }
    };

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Mercurial command failed: {}", error);
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
