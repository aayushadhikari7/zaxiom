//! host command - DNS lookup utility

use std::net::ToSocketAddrs;

use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct HostCommand;

impl Command for HostCommand {
    fn name(&self) -> &'static str {
        "host"
    }

    fn description(&self) -> &'static str {
        "DNS lookup utility"
    }

    fn usage(&self) -> &'static str {
        "host hostname"
    }

    fn execute(&self, args: &[String], _state: &mut TerminalState) -> Result<String> {
        if args.is_empty() {
            return Err(anyhow::anyhow!("host: missing hostname"));
        }

        let hostname = &args[0];
        let mut output = Vec::new();

        // Try to resolve the hostname
        let lookup = format!("{}:80", hostname);
        match lookup.to_socket_addrs() {
            Ok(addrs) => {
                let addresses: Vec<_> = addrs.collect();
                if addresses.is_empty() {
                    output.push(format!("Host {} not found", hostname));
                } else {
                    for addr in addresses {
                        let ip = addr.ip();
                        if ip.is_ipv4() {
                            output.push(format!("{} has address {}", hostname, ip));
                        } else {
                            output.push(format!("{} has IPv6 address {}", hostname, ip));
                        }
                    }
                }
            }
            Err(e) => {
                output.push(format!("Host {} not found: {}", hostname, e));
            }
        }

        Ok(output.join("\n"))
    }
}
