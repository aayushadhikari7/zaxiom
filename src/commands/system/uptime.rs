//! uptime command - tell how long the system has been running

use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct UptimeCommand;

impl Command for UptimeCommand {
    fn name(&self) -> &'static str {
        "uptime"
    }

    fn description(&self) -> &'static str {
        "Tell how long the system has been running"
    }

    fn usage(&self) -> &'static str {
        "uptime [-p]"
    }

    fn extended_help(&self) -> String {
        r#"uptime - Tell how long the system has been running

USAGE:
  uptime [OPTIONS]

OPTIONS:
  -p, --pretty    Show uptime in human-friendly format

DESCRIPTION:
  Display how long the system has been running since
  the last boot. Also shows current time.

EXAMPLES:
  uptime              Standard format
  uptime -p           Pretty format

OUTPUT FORMATS:
  Standard:   14:30:45  up 3 days, 07:22
  Pretty:     up 3 days, 7 hours, 22 minutes

WHAT IT SHOWS:
  • Current system time
  • Days/hours/minutes since last boot

COMMON USE CASES:
  • Check system stability
  • Verify after restart
  • Monitor server uptime
  • Troubleshoot issues

UPTIME MILESTONES:
  1 day     Fresh start
  7 days    Stable system
  30 days   Good reliability
  90+ days  Excellent! (but update security patches)

NOTE:
  Long uptime isn't always good - make sure
  you're applying security updates regularly!

RELATED COMMANDS:
  date       Current date and time
  hostname   System name
  uname      System information
"#.to_string()
    }

    fn execute(&self, args: &[String], _state: &mut TerminalState) -> Result<String> {
        let mut pretty = false;

        for arg in args {
            match arg.as_str() {
                "-p" | "--pretty" => pretty = true,
                "-h" | "--help" => {
                    return Ok("Usage: uptime [OPTIONS]\n\
                        Options:\n  \
                        -p    Show uptime in pretty format".to_string());
                }
                _ => {}
            }
        }

        // Get uptime via PowerShell
        let output = std::process::Command::new("powershell")
            .args(["-NoProfile", "-Command",
                "(Get-Date) - (Get-CimInstance Win32_OperatingSystem).LastBootUpTime | Select-Object Days, Hours, Minutes, Seconds | Format-List"])
            .output()?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);

            let mut days = 0u64;
            let mut hours = 0u64;
            let mut minutes = 0u64;

            for line in stdout.lines() {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() == 2 {
                    let key = parts[0].trim();
                    let value: u64 = parts[1].trim().parse().unwrap_or(0);
                    match key {
                        "Days" => days = value,
                        "Hours" => hours = value,
                        "Minutes" => minutes = value,
                        _ => {}
                    }
                }
            }

            if pretty {
                let mut parts = Vec::new();
                if days > 0 {
                    parts.push(format!("{} day{}", days, if days == 1 { "" } else { "s" }));
                }
                if hours > 0 {
                    parts.push(format!("{} hour{}", hours, if hours == 1 { "" } else { "s" }));
                }
                if minutes > 0 {
                    parts.push(format!("{} minute{}", minutes, if minutes == 1 { "" } else { "s" }));
                }
                Ok(format!("up {}", parts.join(", ")))
            } else {
                let now = chrono::Local::now();
                let time_str = now.format("%H:%M:%S").to_string();

                let uptime_str = if days > 0 {
                    format!("{} days, {:02}:{:02}", days, hours, minutes)
                } else {
                    format!("{:02}:{:02}", hours, minutes)
                };

                Ok(format!(" {}  up {}", time_str, uptime_str))
            }
        } else {
            Err(anyhow::anyhow!("uptime: cannot determine uptime"))
        }
    }
}
