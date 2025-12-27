//! stat command - display file status

use std::fs;
use anyhow::Result;
use chrono::{DateTime, Local};

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct StatCommand;

impl Command for StatCommand {
    fn name(&self) -> &'static str {
        "stat"
    }

    fn description(&self) -> &'static str {
        "Display file or filesystem status"
    }

    fn usage(&self) -> &'static str {
        "stat <file> [file2...]"
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        if args.is_empty() {
            return Err(anyhow::anyhow!("stat: missing operand"));
        }

        let mut output = Vec::new();

        for arg in args {
            if arg.starts_with('-') {
                if arg == "--help" || arg == "-h" {
                    return Ok("Usage: stat <file> [file2...]\n\
                        Display file metadata including size, type, and timestamps.".to_string());
                }
                continue;
            }

            let path = state.resolve_path(arg);
            if !path.exists() {
                output.push(format!("stat: cannot stat '{}': No such file or directory", arg));
                continue;
            }

            let metadata = fs::metadata(&path)?;
            let file_type = if metadata.is_dir() {
                "directory"
            } else if metadata.is_symlink() {
                "symbolic link"
            } else {
                "regular file"
            };

            let size = metadata.len();

            let modified: DateTime<Local> = metadata.modified()?.into();
            let accessed: DateTime<Local> = metadata.accessed()?.into();
            let created: DateTime<Local> = metadata.created()?.into();

            let permissions = if metadata.permissions().readonly() {
                "r--r--r--"
            } else {
                "rw-rw-rw-"
            };

            output.push(format!(
                "  File: {}\n  \
                 Size: {} bytes\t\tType: {}\n  \
                 Access: {}\n  \
                 Modify: {}\n  \
                 Create: {}\n  \
                 Permissions: {}",
                path.display(),
                size,
                file_type,
                accessed.format("%Y-%m-%d %H:%M:%S"),
                modified.format("%Y-%m-%d %H:%M:%S"),
                created.format("%Y-%m-%d %H:%M:%S"),
                permissions
            ));
        }

        Ok(output.join("\n\n"))
    }
}
