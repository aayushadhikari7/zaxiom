//! realpath command - print resolved absolute path

use std::fs;
use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct RealpathCommand;

impl Command for RealpathCommand {
    fn name(&self) -> &'static str {
        "realpath"
    }

    fn description(&self) -> &'static str {
        "Print the resolved absolute path"
    }

    fn usage(&self) -> &'static str {
        "realpath <path...>"
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        if args.is_empty() {
            return Err(anyhow::anyhow!("realpath: missing operand"));
        }

        if args[0] == "-h" || args[0] == "--help" {
            return Ok("Usage: realpath PATH...\n\
                Print the resolved absolute pathname.".to_string());
        }

        let mut output = Vec::new();

        for arg in args {
            let path = state.resolve_path(arg);
            match fs::canonicalize(&path) {
                Ok(canonical) => {
                    output.push(canonical.to_string_lossy().to_string());
                }
                Err(_) => {
                    // If file doesn't exist, just return the resolved path
                    output.push(path.to_string_lossy().to_string());
                }
            }
        }

        Ok(output.join("\n"))
    }
}
