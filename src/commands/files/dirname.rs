//! dirname command - strip last component from file name

use anyhow::Result;
use std::path::Path;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct DirnameCommand;

impl Command for DirnameCommand {
    fn name(&self) -> &'static str {
        "dirname"
    }

    fn description(&self) -> &'static str {
        "Strip last component from file name"
    }

    fn usage(&self) -> &'static str {
        "dirname <path>"
    }

    fn execute(&self, args: &[String], _state: &mut TerminalState) -> Result<String> {
        if args.is_empty() {
            return Err(anyhow::anyhow!("dirname: missing operand"));
        }

        if args[0] == "-h" || args[0] == "--help" {
            return Ok("Usage: dirname NAME\n\
                Output NAME with its last component removed."
                .to_string());
        }

        let path = Path::new(&args[0]);

        let parent = path
            .parent()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| ".".to_string());

        // Handle empty parent
        if parent.is_empty() {
            Ok(".".to_string())
        } else {
            Ok(parent)
        }
    }
}
