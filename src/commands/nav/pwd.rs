//! pwd command - print working directory

use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct PwdCommand;

impl Command for PwdCommand {
    fn name(&self) -> &'static str {
        "pwd"
    }

    fn description(&self) -> &'static str {
        "Print current working directory"
    }

    fn usage(&self) -> &'static str {
        "pwd"
    }

    fn extended_help(&self) -> String {
        r#"pwd - Print working directory

USAGE:
  pwd

DESCRIPTION:
  Print the full pathname of the current working directory.
  Shows where you are in the filesystem.

EXAMPLES:
  pwd              Show current directory

SAMPLE OUTPUT:
  D:/projects/myapp

COMMON USE CASES:
  • Verify your current location
  • Copy path for use elsewhere
  • Use in scripts to get location
  • Debug path-related issues

IN SCRIPTS:
  current_dir=$(pwd)
  echo "Working in: $current_dir"

THE PATH SHOWN:
  Uses forward slashes (/) for consistency,
  even on Windows where backslashes (\) are native.

RELATED COMMANDS:
  cd       Change directory
  ls       List directory contents
  tree     Show directory tree
"#
        .to_string()
    }

    fn execute(&self, _args: &[String], state: &mut TerminalState) -> Result<String> {
        let cwd = state.cwd();
        // Display with forward slashes for consistency
        Ok(cwd.display().to_string().replace('\\', "/"))
    }
}
