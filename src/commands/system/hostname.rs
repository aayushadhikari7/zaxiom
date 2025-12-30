//! hostname command - show or set the system's host name

use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct HostnameCommand;

impl Command for HostnameCommand {
    fn name(&self) -> &'static str {
        "hostname"
    }

    fn description(&self) -> &'static str {
        "Show the system's host name"
    }

    fn usage(&self) -> &'static str {
        "hostname"
    }

    fn extended_help(&self) -> String {
        r#"hostname - Show the system's host name

USAGE:
  hostname

DESCRIPTION:
  Print the name of the current host system (computer name).
  This is the name your computer uses on the network.

EXAMPLES:
  hostname              Print computer name

OUTPUT:
  Just the hostname, e.g.: DESKTOP-ABC123

WHAT IS A HOSTNAME?
  • Your computer's name on the network
  • Used for identification in networks
  • Set during Windows setup or later

COMMON USE CASES:
  • Verify which machine you're on
  • Include in scripts/logs
  • Network troubleshooting
  • Remote connection verification

VIEWING FULL NETWORK NAME:
  On Windows, full name might be:
  DESKTOP-ABC123.local or
  DESKTOP-ABC123.domain.com

CHANGING HOSTNAME:
  Windows: System Settings > About > Rename this PC
  (Requires restart)

RELATED COMMANDS:
  whoami     Current username
  uname      System information
  neofetch   System info with style
"#
        .to_string()
    }

    fn execute(&self, args: &[String], _state: &mut TerminalState) -> Result<String> {
        for arg in args {
            if arg == "-h" || arg == "--help" {
                return Ok("Usage: hostname\n\
                    Show the system's host name."
                    .to_string());
            }
        }

        // Try environment variable first
        if let Ok(hostname) = std::env::var("COMPUTERNAME") {
            return Ok(hostname);
        }

        // Fallback to PowerShell
        let output = std::process::Command::new("powershell")
            .args(["-NoProfile", "-Command", "[Environment]::MachineName"])
            .output()?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            Err(anyhow::anyhow!("hostname: cannot determine hostname"))
        }
    }
}
