//! clear command - clear the terminal screen

use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct ClearCommand;

impl Command for ClearCommand {
    fn name(&self) -> &'static str {
        "clear"
    }

    fn description(&self) -> &'static str {
        "Clear the terminal screen"
    }

    fn usage(&self) -> &'static str {
        "clear"
    }

    fn extended_help(&self) -> String {
        r#"clear - Clear the terminal screen

USAGE:
  clear

DESCRIPTION:
  Clear all text from the terminal screen and move
  cursor to top-left. History is preserved.

EXAMPLES:
  clear            Clear the screen

KEYBOARD SHORTCUT:
  Ctrl+L           Same as typing 'clear'

WHAT IT DOES:
  • Removes all visible text
  • Moves prompt to top
  • Does NOT clear command history
  • Does NOT reset the terminal

COMMAND HISTORY:
  After clearing, you can still:
  • Press Up arrow for previous commands
  • Use 'history' to see past commands

RELATED COMMANDS:
  reset    Reset terminal (more thorough)
  history  View command history
"#
        .to_string()
    }

    fn execute(&self, _args: &[String], _state: &mut TerminalState) -> Result<String> {
        // Return a special marker that the app will interpret as "clear screen"
        // The actual clearing is handled by the app
        Ok("\x1b[CLEAR]".to_string())
    }
}
