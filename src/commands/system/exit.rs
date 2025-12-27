//! exit command - exit the terminal

use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct ExitCommand;

impl Command for ExitCommand {
    fn name(&self) -> &'static str {
        "exit"
    }

    fn description(&self) -> &'static str {
        "Exit the terminal"
    }

    fn usage(&self) -> &'static str {
        "exit [code]"
    }

    fn execute(&self, args: &[String], _state: &mut TerminalState) -> Result<String> {
        let code = args.first()
            .and_then(|s| s.parse::<i32>().ok())
            .unwrap_or(0);

        // Return a special marker that the app will interpret as "exit"
        Ok(format!("\x1b[EXIT:{}]", code))
    }
}
