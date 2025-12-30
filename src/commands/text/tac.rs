//! tac command - concatenate and print files in reverse

use anyhow::Result;
use std::fs;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct TacCommand;

impl Command for TacCommand {
    fn name(&self) -> &'static str {
        "tac"
    }

    fn description(&self) -> &'static str {
        "Print files in reverse (line by line)"
    }

    fn usage(&self) -> &'static str {
        "tac <file> [file2...]"
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        if args.is_empty() {
            return Err(anyhow::anyhow!("tac: missing file operand"));
        }

        let mut all_lines: Vec<String> = Vec::new();

        for arg in args {
            if arg == "-h" || arg == "--help" {
                return Ok("Usage: tac <file> [file2...]\n\
                    Print files in reverse order, line by line."
                    .to_string());
            }

            if arg.starts_with('-') {
                continue;
            }

            let path = state.resolve_path(arg);
            let content =
                fs::read_to_string(&path).map_err(|e| anyhow::anyhow!("tac: {}: {}", arg, e))?;

            all_lines.extend(content.lines().map(|s| s.to_string()));
        }

        all_lines.reverse();
        Ok(all_lines.join("\n"))
    }
}
