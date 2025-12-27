//! whoami command - print current user name

use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct WhoamiCommand;

impl Command for WhoamiCommand {
    fn name(&self) -> &'static str {
        "whoami"
    }

    fn description(&self) -> &'static str {
        "Print current user name"
    }

    fn usage(&self) -> &'static str {
        "whoami"
    }

    fn extended_help(&self) -> String {
        r#"whoami - Print current user name

USAGE:
  whoami

DESCRIPTION:
  Print the user name associated with the current session.
  Simple command with no options - just prints your username.

EXAMPLES:
  whoami                 Print current username
  echo "Hello, $(whoami)"  Use in scripts

OUTPUT:
  Just the username, no domain or extras.
  Example: john

COMMON USE CASES:
  • Verify which user you're logged in as
  • Use in scripts for user-specific paths
  • Debug permission issues
  • Personalize output messages

SIMILAR INFORMATION:
  $USER        Environment variable (Unix)
  $USERNAME    Environment variable (Windows)

RELATED COMMANDS:
  id         User and group IDs
  hostname   Computer name
  users      List logged in users (not available)
"#.to_string()
    }

    fn execute(&self, args: &[String], _state: &mut TerminalState) -> Result<String> {
        for arg in args {
            if arg == "-h" || arg == "--help" {
                return Ok("Usage: whoami\n\
                    Print the current user name.".to_string());
            }
        }

        // Try environment variable first
        if let Ok(user) = std::env::var("USERNAME") {
            return Ok(user);
        }

        // Fallback to PowerShell
        let output = std::process::Command::new("powershell")
            .args(["-NoProfile", "-Command", "[Environment]::UserName"])
            .output()?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            Err(anyhow::anyhow!("whoami: cannot determine user"))
        }
    }
}
