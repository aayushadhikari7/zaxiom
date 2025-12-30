//! Git prompt integration
//!
//! Extracts git info for the prompt.

#![allow(dead_code)]

use std::path::Path;

/// Get current git branch name, if in a git repo
pub fn get_git_branch(path: &Path) -> Option<String> {
    // Try to find .git directory
    let mut current = path.to_path_buf();

    loop {
        let git_dir = current.join(".git");
        if git_dir.exists() {
            // Found a git repo, get the branch
            return read_git_branch(&git_dir);
        }

        if !current.pop() {
            break;
        }
    }

    None
}

/// Read branch name from .git directory
fn read_git_branch(git_dir: &Path) -> Option<String> {
    // First try reading HEAD
    let head_path = git_dir.join("HEAD");
    let head_content = std::fs::read_to_string(head_path).ok()?;
    let head = head_content.trim();

    // Check if it's a ref
    if let Some(branch) = head.strip_prefix("ref: refs/heads/") {
        // Skip "ref: refs/heads/"
        return Some(branch.to_string());
    }

    // If not a ref, it might be a detached HEAD (commit hash)
    if head.len() >= 7 {
        return Some(format!(":{}", &head[..7])); // Show short hash with : prefix
    }

    None
}

/// Get git status indicators
pub fn get_git_status(path: &Path) -> Option<GitStatus> {
    // This could be expanded to show:
    // - Number of modified files
    // - Staged changes
    // - Ahead/behind remote
    // For MVP, we just check if there are changes

    let output = std::process::Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(path)
        .output()
        .ok()?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let has_changes = !stdout.trim().is_empty();

        Some(GitStatus {
            has_changes,
            staged: 0,
            modified: 0,
            untracked: 0,
        })
    } else {
        None
    }
}

/// Git status info
#[derive(Debug, Clone)]
pub struct GitStatus {
    pub has_changes: bool,
    pub staged: usize,
    pub modified: usize,
    pub untracked: usize,
}
