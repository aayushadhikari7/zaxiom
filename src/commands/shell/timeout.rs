//! timeout command - run a command with a time limit

use std::process::Command as ProcessCommand;
use std::time::Duration;
use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct TimeoutCommand;

impl Command for TimeoutCommand {
    fn name(&self) -> &'static str {
        "timeout"
    }

    fn description(&self) -> &'static str {
        "Run a command with a time limit"
    }

    fn usage(&self) -> &'static str {
        "timeout <duration> <command>"
    }

    fn execute(&self, args: &[String], _state: &mut TerminalState) -> Result<String> {
        if args.len() < 2 {
            return Err(anyhow::anyhow!("timeout: missing arguments"));
        }

        if args[0] == "-h" || args[0] == "--help" {
            return Ok("Usage: timeout DURATION COMMAND [ARGS...]\n\
                Run COMMAND with a time limit.\n\n\
                DURATION can be:\n  \
                N      N seconds\n  \
                Ns     N seconds\n  \
                Nm     N minutes\n  \
                Nh     N hours".to_string());
        }

        let duration_str = &args[0];
        let duration = parse_duration(duration_str)?;

        let command = &args[1];
        let command_args = &args[2..];

        // Run via PowerShell with timeout
        let full_command = if command_args.is_empty() {
            command.clone()
        } else {
            format!("{} {}", command, command_args.join(" "))
        };

        // Use PowerShell Start-Process with timeout
        let ps_command = format!(
            "$proc = Start-Process -FilePath 'powershell' -ArgumentList '-NoProfile', '-Command', '{}' -PassThru -NoNewWindow; \
             if (!$proc.WaitForExit({})) {{ $proc.Kill(); Write-Error 'timeout: timed out' }} \
             else {{ $proc.ExitCode }}",
            full_command.replace("'", "''"),
            duration.as_millis()
        );

        let output = ProcessCommand::new("powershell")
            .args(["-NoProfile", "-Command", &ps_command])
            .output()
            .map_err(|e| anyhow::anyhow!("timeout: {}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if stderr.contains("timed out") {
            return Err(anyhow::anyhow!("timeout: command timed out after {}", duration_str));
        }

        Ok(format!("{}{}", stdout.trim(), stderr.trim()))
    }
}

fn parse_duration(s: &str) -> Result<Duration> {
    let s = s.trim();

    if let Some(n) = s.strip_suffix('s') {
        let n: f64 = n.parse()
            .map_err(|_| anyhow::anyhow!("timeout: invalid duration"))?;
        return Ok(Duration::from_secs_f64(n));
    }

    if let Some(n) = s.strip_suffix('m') {
        let n: f64 = n.parse()
            .map_err(|_| anyhow::anyhow!("timeout: invalid duration"))?;
        return Ok(Duration::from_secs_f64(n * 60.0));
    }

    if let Some(n) = s.strip_suffix('h') {
        let n: f64 = n.parse()
            .map_err(|_| anyhow::anyhow!("timeout: invalid duration"))?;
        return Ok(Duration::from_secs_f64(n * 3600.0));
    }

    // Default: seconds
    let n: f64 = s.parse()
        .map_err(|_| anyhow::anyhow!("timeout: invalid duration"))?;
    Ok(Duration::from_secs_f64(n))
}
