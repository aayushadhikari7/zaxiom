//! netstat command - network statistics

use std::process::Command as ProcessCommand;
use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct NetstatCommand;

impl Command for NetstatCommand {
    fn name(&self) -> &'static str {
        "netstat"
    }

    fn description(&self) -> &'static str {
        "Display network connections and statistics"
    }

    fn usage(&self) -> &'static str {
        "netstat [-a] [-n] [-o] [-p protocol]"
    }

    fn execute(&self, args: &[String], _state: &mut TerminalState) -> Result<String> {
        let mut show_all = false;
        let mut numeric = false;
        let mut show_pid = false;
        let mut protocol: Option<&String> = None;

        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "-a" | "--all" => show_all = true,
                "-n" | "--numeric" => numeric = true,
                "-o" | "--timers" => show_pid = true,
                "-p" | "--protocol" => {
                    if i + 1 < args.len() {
                        protocol = Some(&args[i + 1]);
                        i += 1;
                    }
                }
                "-h" | "--help" => {
                    return Ok("Usage: netstat [OPTIONS]\n\
                        Options:\n  \
                        -a           Show all connections and listening ports\n  \
                        -n           Show addresses and port numbers numerically\n  \
                        -o           Show process ID for each connection\n  \
                        -p <proto>   Show connections for specified protocol (tcp/udp)".to_string());
                }
                _ => {}
            }
            i += 1;
        }

        // Build Windows netstat command
        let mut cmd = ProcessCommand::new("netstat");

        if show_all {
            cmd.arg("-a");
        }
        if numeric {
            cmd.arg("-n");
        }
        if show_pid {
            cmd.arg("-o");
        }
        if let Some(proto) = protocol {
            cmd.arg("-p").arg(proto);
        }

        let output = cmd.output()
            .map_err(|e| anyhow::anyhow!("netstat: {}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if !output.status.success() && !stderr.is_empty() {
            return Err(anyhow::anyhow!("netstat: {}", stderr.trim()));
        }

        Ok(stdout.to_string())
    }
}
