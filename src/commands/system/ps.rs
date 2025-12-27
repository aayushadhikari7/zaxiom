//! ps command - report process status

use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct PsCommand;

impl Command for PsCommand {
    fn name(&self) -> &'static str {
        "ps"
    }

    fn description(&self) -> &'static str {
        "Report process status"
    }

    fn usage(&self) -> &'static str {
        "ps [-a] [-e]"
    }

    fn extended_help(&self) -> String {
        r#"ps - Report process status

USAGE:
  ps [OPTIONS]

OPTIONS:
  -a, -e, --all    Show all processes (not just windowed)

DESCRIPTION:
  Display information about running processes.
  By default, shows only processes with windows.

EXAMPLES:
  ps                 List windowed processes
  ps -a              List ALL processes
  ps -e              Same as -a (Unix compatibility)

OUTPUT COLUMNS:
  PID      Process ID (use with kill command)
  CPU      CPU time used
  MEM(MB)  Memory usage in megabytes
  COMMAND  Process name

COMMON USE CASES:
  • Find process ID to terminate
  • Check memory usage
  • Monitor running applications
  • Debug hanging programs

FINDING SPECIFIC PROCESSES:
  ps -a | grep chrome    Find Chrome processes
  ps -a | grep node      Find Node.js processes

NOTE:
  Uses PowerShell internally on Windows.
  Output is formatted to look Unix-like.

RELATED COMMANDS:
  kill     Terminate processes
  top      Real-time process viewer (not available)
  htop     Interactive process viewer (not available)
"#.to_string()
    }

    fn execute(&self, args: &[String], _state: &mut TerminalState) -> Result<String> {
        let mut all = false;

        for arg in args {
            match arg.as_str() {
                "-a" | "-e" | "--all" => all = true,
                "-h" | "--help" => {
                    return Ok("Usage: ps [OPTIONS]\n\
                        Options:\n  \
                        -a, -e    Show all processes".to_string());
                }
                _ => {}
            }
        }

        // Use PowerShell to get process list
        let ps_cmd = if all {
            "Get-Process | Select-Object Id, ProcessName, CPU, @{N='Mem(MB)';E={[math]::Round($_.WorkingSet64/1MB,1)}} | Format-Table -AutoSize"
        } else {
            "Get-Process | Where-Object {$_.MainWindowHandle -ne 0} | Select-Object Id, ProcessName, CPU, @{N='Mem(MB)';E={[math]::Round($_.WorkingSet64/1MB,1)}} | Format-Table -AutoSize"
        };

        let output = std::process::Command::new("powershell")
            .args(["-NoProfile", "-Command", ps_cmd])
            .output()?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);

            // Format output to look more Unix-like
            let mut lines: Vec<String> = Vec::new();
            lines.push(format!("{:>8} {:>8} {:>8} {}", "PID", "CPU", "MEM(MB)", "COMMAND"));

            for line in stdout.lines().skip(3) { // Skip PowerShell headers
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    let pid = parts.first().unwrap_or(&"");
                    let name = parts.get(1).unwrap_or(&"");
                    let cpu = parts.get(2).unwrap_or(&"0");
                    let mem = parts.get(3).unwrap_or(&"0");

                    if !pid.is_empty() && pid.chars().all(|c| c.is_ascii_digit()) {
                        lines.push(format!("{:>8} {:>8} {:>8} {}", pid, cpu, mem, name));
                    }
                }
            }

            Ok(lines.join("\n"))
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(anyhow::anyhow!("ps: {}", stderr))
        }
    }
}
