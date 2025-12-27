//! env command - display environment variables

use std::env;
use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct EnvCommand;

impl Command for EnvCommand {
    fn name(&self) -> &'static str {
        "env"
    }

    fn description(&self) -> &'static str {
        "Display environment variables"
    }

    fn usage(&self) -> &'static str {
        "env [-0] [name]"
    }

    fn extended_help(&self) -> String {
        r#"env - Display environment variables

USAGE:
  env                 Show all environment variables
  env NAME            Show specific variable value

OPTIONS:
  -0, --null    Separate entries with null instead of newline

DESCRIPTION:
  Display environment variables. These are system-wide
  settings that programs use for configuration.

EXAMPLES:
  env                 List all environment variables
  env PATH            Show PATH variable
  env HOME            Show home directory
  env | grep USER     Search for USER-related vars

COMMON ENVIRONMENT VARIABLES:
  PATH        Where to find executable programs
  HOME        User's home directory
  USER        Current username
  SHELL       Default shell
  PWD         Current working directory
  TEMP/TMP    Temporary files directory
  LANG        Language setting

WINDOWS-SPECIFIC:
  USERNAME          Current user
  USERPROFILE       Home directory
  COMPUTERNAME      Computer name
  SystemRoot        Windows directory
  ProgramFiles      Program files location

USING VARIABLES:
  In commands:
  echo $PATH         Show PATH value
  cd $HOME           Go to home directory

SETTING VARIABLES:
  Use 'export' to set variables:
  export MY_VAR=value

RELATED COMMANDS:
  export     Set environment variables
  printenv   Print environment (alias for env)
  echo $VAR  Print specific variable
"#.to_string()
    }

    fn execute(&self, args: &[String], _state: &mut TerminalState) -> Result<String> {
        let mut null_sep = false;
        let mut name_filter: Option<&String> = None;

        for arg in args {
            match arg.as_str() {
                "-0" | "--null" => null_sep = true,
                "-h" | "--help" => {
                    return Ok("Usage: env [OPTIONS] [name]\n\
                        Options:\n  \
                        -0    Use null separators\n\
                        Without arguments, prints all environment variables.".to_string());
                }
                _ if !arg.starts_with('-') => name_filter = Some(arg),
                _ => {}
            }
        }

        let separator = if null_sep { "\0" } else { "\n" };

        if let Some(name) = name_filter {
            // Show specific variable
            match env::var(name) {
                Ok(val) => Ok(val),
                Err(_) => Ok(String::new()),
            }
        } else {
            // Show all variables
            let vars: Vec<String> = env::vars()
                .map(|(key, val)| format!("{}={}", key, val))
                .collect();
            Ok(vars.join(separator))
        }
    }
}
