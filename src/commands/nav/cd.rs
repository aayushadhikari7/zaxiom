//! cd command - change directory

use std::path::PathBuf;
use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

/// Strip the Windows extended-length path prefix (\\?\) if present
#[cfg(windows)]
fn strip_unc_prefix(path: PathBuf) -> PathBuf {
    let path_str = path.to_string_lossy();
    if path_str.starts_with(r"\\?\") {
        PathBuf::from(&path_str[4..])
    } else {
        path
    }
}

#[cfg(not(windows))]
fn strip_unc_prefix(path: PathBuf) -> PathBuf {
    path
}

pub struct CdCommand;

impl Command for CdCommand {
    fn name(&self) -> &'static str {
        "cd"
    }

    fn description(&self) -> &'static str {
        "Change current directory"
    }

    fn usage(&self) -> &'static str {
        "cd [path]"
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        let target = if args.is_empty() {
            // No args = go to home directory
            state.home().clone()
        } else {
            state.resolve_path(&args[0])
        };

        // Canonicalize the path (resolve .., symlinks, etc.)
        let canonical = match target.canonicalize() {
            Ok(p) => strip_unc_prefix(p),
            Err(_) => {
                return Err(anyhow::anyhow!("No such directory: {}", target.display()));
            }
        };

        if !canonical.is_dir() {
            return Err(anyhow::anyhow!("Not a directory: {}", canonical.display()));
        }

        state.set_cwd(canonical);
        Ok(String::new()) // cd produces no output on success
    }
}
