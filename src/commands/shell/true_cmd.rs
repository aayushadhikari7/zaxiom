//! true command - do nothing, successfully

use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct TrueCommand;

impl Command for TrueCommand {
    fn name(&self) -> &'static str {
        "true"
    }

    fn description(&self) -> &'static str {
        "Do nothing, successfully"
    }

    fn usage(&self) -> &'static str {
        "true"
    }

    fn execute(&self, _args: &[String], _state: &mut TerminalState) -> Result<String> {
        Ok(String::new())
    }
}
