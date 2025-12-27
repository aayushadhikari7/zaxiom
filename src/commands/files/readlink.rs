//! readlink command - print resolved symbolic links

use std::fs;

use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct ReadlinkCommand;

impl Command for ReadlinkCommand {
    fn name(&self) -> &'static str {
        "readlink"
    }

    fn description(&self) -> &'static str {
        "Print value of a symbolic link"
    }

    fn usage(&self) -> &'static str {
        "readlink [-f] file..."
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        let mut canonicalize = false;
        let mut files: Vec<&str> = Vec::new();

        for arg in args {
            match arg.as_str() {
                "-f" | "--canonicalize" => canonicalize = true,
                "-e" | "--canonicalize-existing" => canonicalize = true,
                "-m" | "--canonicalize-missing" => canonicalize = true,
                arg if !arg.starts_with('-') => files.push(arg),
                _ => {}
            }
        }

        if files.is_empty() {
            return Err(anyhow::anyhow!("readlink: missing file operand"));
        }

        let mut output = Vec::new();

        for file in files {
            let path = state.resolve_path(file);

            if canonicalize {
                match fs::canonicalize(&path) {
                    Ok(resolved) => output.push(resolved.display().to_string()),
                    Err(e) => return Err(anyhow::anyhow!("readlink: {}: {}", file, e)),
                }
            } else {
                match fs::read_link(&path) {
                    Ok(target) => output.push(target.display().to_string()),
                    Err(e) => return Err(anyhow::anyhow!("readlink: {}: {}", file, e)),
                }
            }
        }

        Ok(output.join("\n"))
    }
}
