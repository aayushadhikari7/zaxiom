//! kill command - terminate processes

use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct KillCommand;

impl Command for KillCommand {
    fn name(&self) -> &'static str {
        "kill"
    }

    fn description(&self) -> &'static str {
        "Terminate processes"
    }

    fn usage(&self) -> &'static str {
        "kill [-9] <pid> [pid2...]"
    }

    fn extended_help(&self) -> String {
        r#"kill - Terminate processes

USAGE:
  kill [OPTIONS] <pid> [pid2...]

OPTIONS:
  -9, -KILL, --force    Force kill (no graceful shutdown)

DESCRIPTION:
  Terminate processes by their process ID (PID).
  Use 'ps' command to find process IDs.

EXAMPLES:
  kill 1234             Gracefully terminate process
  kill -9 1234          Force kill process
  kill 1234 5678 9012   Kill multiple processes

SIGNALS (Linux-style, Windows compatible):
  (default)  Request graceful termination
  -9         Force immediate termination (SIGKILL)

COMMON USE CASES:
  • Stop hung applications
  • Free up system resources
  • Terminate background processes
  • Stop runaway scripts

WORKFLOW:
  1. ps -a               Find process ID
  2. kill <pid>          Try graceful stop
  3. kill -9 <pid>       Force if still running

ERROR MESSAGES:
  "No such process"      PID doesn't exist
  "Permission denied"    Need admin rights

NOTE:
  Some system processes require administrator
  privileges to terminate.

RELATED COMMANDS:
  ps       List processes
  taskkill Windows native kill command
"#.to_string()
    }

    fn execute(&self, args: &[String], _state: &mut TerminalState) -> Result<String> {
        let mut force = false;
        let mut pids: Vec<&String> = Vec::new();

        for arg in args {
            match arg.as_str() {
                "-9" | "-KILL" | "--force" => force = true,
                "-h" | "--help" => {
                    return Ok("Usage: kill [OPTIONS] <pid> [pid2...]\n\
                        Options:\n  \
                        -9    Force kill (SIGKILL)".to_string());
                }
                _ if !arg.starts_with('-') => pids.push(arg),
                _ => {}
            }
        }

        if pids.is_empty() {
            return Err(anyhow::anyhow!("kill: missing process ID"));
        }

        let mut results = Vec::new();

        for pid in pids {
            let cmd = if force {
                format!("Stop-Process -Id {} -Force -ErrorAction Stop", pid)
            } else {
                format!("Stop-Process -Id {} -ErrorAction Stop", pid)
            };

            let output = std::process::Command::new("powershell")
                .args(["-NoProfile", "-Command", &cmd])
                .output()?;

            if output.status.success() {
                results.push(format!("Killed process {}", pid));
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                if stderr.contains("Cannot find a process") {
                    results.push(format!("kill: ({}): No such process", pid));
                } else if stderr.contains("Access is denied") {
                    results.push(format!("kill: ({}): Permission denied", pid));
                } else {
                    results.push(format!("kill: ({}): {}", pid, stderr.trim()));
                }
            }
        }

        Ok(results.join("\n"))
    }
}
