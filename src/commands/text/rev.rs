//! rev command - reverse lines character by character

use anyhow::Result;
use std::fs;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct RevCommand;

impl Command for RevCommand {
    fn name(&self) -> &'static str {
        "rev"
    }

    fn description(&self) -> &'static str {
        "Reverse lines character by character"
    }

    fn usage(&self) -> &'static str {
        "rev [file]"
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        self.execute_with_stdin(args, None, state)
    }

    fn execute_with_stdin(
        &self,
        args: &[String],
        stdin: Option<&str>,
        state: &mut TerminalState,
    ) -> Result<String> {
        if !args.is_empty() && (args[0] == "-h" || args[0] == "--help") {
            return Ok("Usage: rev [FILE]\n\
                Reverse each line of input."
                .to_string());
        }

        let content = if args.is_empty() {
            if let Some(input) = stdin {
                input.to_string()
            } else {
                return Err(anyhow::anyhow!("rev: no input"));
            }
        } else {
            let path = state.resolve_path(&args[0]);
            fs::read_to_string(&path).map_err(|e| anyhow::anyhow!("rev: {}: {}", args[0], e))?
        };

        let reversed: Vec<String> = content
            .lines()
            .map(|line| line.chars().rev().collect())
            .collect();

        Ok(reversed.join("\n"))
    }

    fn supports_stdin(&self) -> bool {
        true
    }
}
