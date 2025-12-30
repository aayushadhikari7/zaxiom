//! ping command - send ICMP echo requests

use anyhow::Result;
use std::process::Command as ProcessCommand;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct PingCommand;

impl Command for PingCommand {
    fn name(&self) -> &'static str {
        "ping"
    }

    fn description(&self) -> &'static str {
        "Send ICMP echo requests to network hosts"
    }

    fn usage(&self) -> &'static str {
        "ping [-c count] [-t] <host>"
    }

    fn extended_help(&self) -> String {
        r#"ping - Test network connectivity

USAGE:
  ping [OPTIONS] <host>

OPTIONS:
  -c <count>    Number of pings to send (default: 4)
  -t            Continuous ping (Ctrl+C to stop)

DESCRIPTION:
  Send ICMP echo requests to test if a host is reachable
  and measure round-trip time (latency).

EXAMPLES:
  ping google.com           Ping Google (4 times)
  ping -c 10 8.8.8.8        Ping 10 times
  ping -t localhost         Continuous ping
  ping 192.168.1.1          Ping local IP

OUTPUT EXPLAINED:
  Reply from X: bytes=32 time=15ms TTL=64
  - bytes: Packet size
  - time: Round-trip time (latency)
  - TTL: Time To Live (hop count)

COMMON USES:
  • Check if a server is online
  • Measure network latency
  • Diagnose connection issues
  • Test DNS resolution

RELATED COMMANDS:
  traceroute   Trace packet route
  netstat      Network statistics
  nslookup     DNS lookup
  curl         HTTP requests
"#
        .to_string()
    }

    fn execute(&self, args: &[String], _state: &mut TerminalState) -> Result<String> {
        let mut count: Option<u32> = None;
        let mut continuous = false;
        let mut host: Option<&String> = None;

        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "-c" | "--count" => {
                    if i + 1 < args.len() {
                        count = args[i + 1].parse().ok();
                        i += 1;
                    }
                }
                "-t" => continuous = true,
                "-h" | "--help" => {
                    return Ok("Usage: ping [OPTIONS] <host>\n\
                        Options:\n  \
                        -c <count>   Number of pings to send (default: 4)\n  \
                        -t           Ping continuously until stopped"
                        .to_string());
                }
                _ if !args[i].starts_with('-') => host = Some(&args[i]),
                _ => {}
            }
            i += 1;
        }

        let host = host.ok_or_else(|| anyhow::anyhow!("ping: missing host"))?;

        // Build Windows ping command
        let mut cmd = ProcessCommand::new("ping");

        if continuous {
            cmd.arg("-t");
        } else {
            let n = count.unwrap_or(4);
            cmd.arg("-n").arg(n.to_string());
        }

        cmd.arg(host);

        let output = cmd.output().map_err(|e| anyhow::anyhow!("ping: {}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if !output.status.success() && !stderr.is_empty() {
            return Err(anyhow::anyhow!("ping: {}", stderr.trim()));
        }

        Ok(stdout.to_string())
    }
}
