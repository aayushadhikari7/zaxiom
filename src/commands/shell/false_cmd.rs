//! false command - do nothing, unsuccessfully

use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct FalseCommand;

impl Command for FalseCommand {
    fn name(&self) -> &'static str {
        "false"
    }

    fn description(&self) -> &'static str {
        "Do nothing, unsuccessfully"
    }

    fn usage(&self) -> &'static str {
        "false"
    }

    fn execute(&self, _args: &[String], _state: &mut TerminalState) -> Result<String> {
        Err(anyhow::anyhow!(""))
    }
}
