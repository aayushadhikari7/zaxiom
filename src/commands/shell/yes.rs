//! yes command - output a string repeatedly

use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct YesCommand;

impl Command for YesCommand {
    fn name(&self) -> &'static str {
        "yes"
    }

    fn description(&self) -> &'static str {
        "Output a string repeatedly"
    }

    fn usage(&self) -> &'static str {
        "yes [string]"
    }

    fn execute(&self, args: &[String], _state: &mut TerminalState) -> Result<String> {
        if !args.is_empty() && (args[0] == "-h" || args[0] == "--help") {
            return Ok("Usage: yes [STRING]\n\
                Repeatedly output STRING, or 'y' by default.\n\
                Note: Limited to 100 lines in this terminal."
                .to_string());
        }

        let text = if args.is_empty() {
            "y".to_string()
        } else {
            args.join(" ")
        };

        // Limit output to prevent hanging
        let lines: Vec<&str> = std::iter::repeat_n(text.as_str(), 100).collect();
        Ok(lines.join("\n"))
    }
}
