//! Pipeline execution
//!
//! Handles piped commands (cmd1 | cmd2 | cmd3).

#![allow(dead_code)]

use anyhow::Result;

/// Execute a pipeline of commands
/// Currently delegates to PowerShell for proper pipe handling
pub fn execute_pipeline(commands: &str) -> Result<String> {
    // For MVP, we delegate pipelines to PowerShell
    // In the future, we could implement native pipe handling
    // by passing stdout of one command as stdin to the next

    use std::process::Command;

    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", commands])
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() && !stderr.is_empty() {
        return Err(anyhow::anyhow!("{}", stderr.trim()));
    }

    Ok(stdout.trim().to_string())
}
