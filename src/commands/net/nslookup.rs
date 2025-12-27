//! nslookup command - query DNS servers

use std::net::ToSocketAddrs;

use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct NslookupCommand;

impl Command for NslookupCommand {
    fn name(&self) -> &'static str {
        "nslookup"
    }

    fn description(&self) -> &'static str {
        "Query Internet name servers"
    }

    fn usage(&self) -> &'static str {
        "nslookup hostname"
    }

    fn execute(&self, args: &[String], _state: &mut TerminalState) -> Result<String> {
        if args.is_empty() {
            return Err(anyhow::anyhow!("nslookup: missing hostname"));
        }

        let hostname = &args[0];
        let mut output = Vec::new();

        output.push(format!("Server:  (system resolver)"));
        output.push(String::new());
        output.push(format!("Name:    {}", hostname));

        // Try to resolve the hostname
        let lookup = format!("{}:80", hostname);
        match lookup.to_socket_addrs() {
            Ok(addrs) => {
                let addresses: Vec<_> = addrs.collect();
                if addresses.is_empty() {
                    output.push("Address: (no addresses found)".to_string());
                } else {
                    for addr in addresses {
                        output.push(format!("Address: {}", addr.ip()));
                    }
                }
            }
            Err(e) => {
                output.push(format!("** Unable to find {}: {}", hostname, e));
            }
        }

        Ok(output.join("\n"))
    }
}
