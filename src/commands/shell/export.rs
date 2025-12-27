//! export command - set environment variables

use std::env;
use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct ExportCommand;

impl Command for ExportCommand {
    fn name(&self) -> &'static str {
        "export"
    }

    fn description(&self) -> &'static str {
        "Set environment variables"
    }

    fn usage(&self) -> &'static str {
        "export [name[=value] ...]"
    }

    fn extended_help(&self) -> String {
        r#"export - Set environment variables

USAGE:
  export                      List all environment variables
  export NAME=value           Set a variable
  export NAME="value"         Set with quotes (for spaces)

DESCRIPTION:
  Set environment variables that will be available to
  the current session and any child processes.

EXAMPLES:
  export                          List all variables
  export MY_VAR=hello             Set simple value
  export PATH="$PATH:/new/path"   Append to PATH
  export API_KEY="secret123"      Set secret

COMMON USE CASES:
  # Set custom paths
  export PATH="$PATH:$HOME/bin"

  # Configure applications
  export EDITOR=vim
  export NODE_ENV=development

  # Set API keys
  export API_KEY="your-key-here"

QUOTING VALUES:
  export VAR=simple              No quotes for simple values
  export VAR="has spaces"        Quotes needed for spaces
  export VAR='literal $dollar'   Single quotes = no expansion

VARIABLE EXPANSION:
  export NEW="$OLD/more"     Expands $OLD first
  export NEW='$OLD/more'     Literal $OLD (no expansion)

APPENDING TO PATH:
  export PATH="$PATH:/new/dir"   Add to end
  export PATH="/new/dir:$PATH"   Add to beginning

SESSION SCOPE:
  Variables set with export are temporary!
  They disappear when you close the terminal.

FOR PERMANENT VARIABLES:
  Add export commands to:
  ~/.bashrc or ~/.zshrc (Linux/Mac)
  System Environment Variables (Windows)

UNSETTING VARIABLES:
  unset VARIABLE_NAME

RELATED COMMANDS:
  env      View all environment variables
  unset    Remove a variable
  echo     Print variable: echo $VAR
"#.to_string()
    }

    fn execute(&self, args: &[String], _state: &mut TerminalState) -> Result<String> {
        if args.is_empty() {
            // List all exported variables
            let vars: Vec<String> = env::vars()
                .map(|(key, val)| format!("export {}=\"{}\"", key, val))
                .collect();
            return Ok(vars.join("\n"));
        }

        for arg in args {
            if arg == "-h" || arg == "--help" {
                return Ok("Usage: export [name[=value] ...]\n\
                    Without arguments, prints all environment variables.\n\
                    With name=value, sets the variable.\n\
                    With name only, marks variable for export.".to_string());
            }

            if let Some(eq_pos) = arg.find('=') {
                // Set variable: name=value
                let name = &arg[..eq_pos];
                let value = &arg[eq_pos + 1..];
                // Remove surrounding quotes if present
                let value = value.trim_matches('\'').trim_matches('"');
                env::set_var(name, value);
            }
            // If no =, we'd normally mark for export, but in this context
            // all env vars are already exported
        }

        Ok(String::new())
    }
}
