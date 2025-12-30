//! watch command - execute a program periodically

use anyhow::Result;
use std::process::Command as ProcessCommand;
use std::thread;
use std::time::Duration;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct WatchCommand;

impl Command for WatchCommand {
    fn name(&self) -> &'static str {
        "watch"
    }

    fn description(&self) -> &'static str {
        "Execute a program periodically, showing output"
    }

    fn usage(&self) -> &'static str {
        "watch [-n seconds] <command>"
    }

    fn execute(&self, args: &[String], _state: &mut TerminalState) -> Result<String> {
        let mut interval: f64 = 2.0;
        let mut iterations: Option<u32> = None;
        let mut command_start = 0;

        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "-n" | "--interval" => {
                    if i + 1 < args.len() {
                        interval = args[i + 1].parse().unwrap_or(2.0);
                        i += 1;
                        command_start = i + 1;
                    }
                }
                "-c" | "--count" => {
                    if i + 1 < args.len() {
                        iterations = args[i + 1].parse().ok();
                        i += 1;
                        command_start = i + 1;
                    }
                }
                "-h" | "--help" => {
                    return Ok("Usage: watch [OPTIONS] <command>\n\
                        Options:\n  \
                        -n <secs>    Seconds between updates (default: 2)\n  \
                        -c <count>   Number of iterations (default: infinite)\n\n\
                        Note: In this terminal, watch runs for limited iterations.\n\
                        Use -c to specify how many times to run."
                        .to_string());
                }
                _ if !args[i].starts_with('-') => {
                    command_start = i;
                    break;
                }
                _ => {
                    command_start = i + 1;
                }
            }
            i += 1;
        }

        if command_start >= args.len() {
            return Err(anyhow::anyhow!("watch: no command specified"));
        }

        let command_args = &args[command_start..];
        let command_str = command_args.join(" ");

        // For non-interactive use, default to 3 iterations if not specified
        let max_iterations = iterations.unwrap_or(3);
        let mut outputs = Vec::new();

        for iter in 0..max_iterations {
            // Run command via PowerShell
            let output = ProcessCommand::new("powershell")
                .args(["-NoProfile", "-Command", &command_str])
                .output();

            let result = match output {
                Ok(out) => {
                    let stdout = String::from_utf8_lossy(&out.stdout);
                    let stderr = String::from_utf8_lossy(&out.stderr);
                    if !stderr.is_empty() {
                        format!("{}{}", stdout, stderr)
                    } else {
                        stdout.to_string()
                    }
                }
                Err(e) => format!("Error: {}", e),
            };

            outputs.push(format!("--- Iteration {} ---\n{}", iter + 1, result.trim()));

            // Sleep between iterations (but not after the last one)
            if iter + 1 < max_iterations {
                thread::sleep(Duration::from_secs_f64(interval));
            }
        }

        Ok(outputs.join("\n\n"))
    }
}
