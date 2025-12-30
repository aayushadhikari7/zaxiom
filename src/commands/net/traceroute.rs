//! traceroute command - trace packet route to host

use anyhow::Result;
use std::process::Command as ProcessCommand;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct TracerouteCommand;

impl Command for TracerouteCommand {
    fn name(&self) -> &'static str {
        "traceroute"
    }

    fn description(&self) -> &'static str {
        "Trace the route packets take to a host"
    }

    fn usage(&self) -> &'static str {
        "traceroute [-m maxhops] <host>"
    }

    fn execute(&self, args: &[String], _state: &mut TerminalState) -> Result<String> {
        let mut max_hops: Option<u32> = None;
        let mut host: Option<&String> = None;

        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "-m" | "--max-hops" => {
                    if i + 1 < args.len() {
                        max_hops = args[i + 1].parse().ok();
                        i += 1;
                    }
                }
                "-h" | "--help" => {
                    return Ok("Usage: traceroute [OPTIONS] <host>\n\
                        Options:\n  \
                        -m <hops>    Maximum number of hops (default: 30)"
                        .to_string());
                }
                _ if !args[i].starts_with('-') => host = Some(&args[i]),
                _ => {}
            }
            i += 1;
        }

        let host = host.ok_or_else(|| anyhow::anyhow!("traceroute: missing host"))?;

        // Windows uses 'tracert' instead of 'traceroute'
        let mut cmd = ProcessCommand::new("tracert");

        if let Some(hops) = max_hops {
            cmd.arg("-h").arg(hops.to_string());
        }

        cmd.arg(host);

        let output = cmd
            .output()
            .map_err(|e| anyhow::anyhow!("traceroute: {}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if !output.status.success() && !stderr.is_empty() {
            return Err(anyhow::anyhow!("traceroute: {}", stderr.trim()));
        }

        Ok(stdout.to_string())
    }
}
