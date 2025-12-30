//! ifconfig command - display network interface configuration

use std::process::Command as ProcessCommand;

use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct IfconfigCommand;

impl Command for IfconfigCommand {
    fn name(&self) -> &'static str {
        "ifconfig"
    }

    fn description(&self) -> &'static str {
        "Display network interface configuration"
    }

    fn usage(&self) -> &'static str {
        "ifconfig"
    }

    fn execute(&self, _args: &[String], _state: &mut TerminalState) -> Result<String> {
        // On Windows, use ipconfig and format output to look more Unix-like
        let output = ProcessCommand::new("ipconfig")
            .output()
            .map_err(|e| anyhow::anyhow!("Failed to execute ipconfig: {}", e))?;

        if !output.status.success() {
            return Err(anyhow::anyhow!("ipconfig failed"));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);

        // Parse and reformat to look more like ifconfig
        let mut result = Vec::new();
        let mut current_interface;

        for line in stdout.lines() {
            let line = line.trim();

            if line.contains("adapter") {
                // New interface
                current_interface = line
                    .replace("Ethernet adapter ", "eth")
                    .replace("Wireless LAN adapter ", "wlan")
                    .replace(":", "")
                    .replace(" ", "_");
                result.push(format!(
                    "\n{}: flags=<UP,BROADCAST,RUNNING>",
                    current_interface
                ));
            } else if line.starts_with("IPv4 Address") {
                if let Some(ip) = line.split(':').nth(1) {
                    result.push(format!("        inet {}  netmask 255.255.255.0", ip.trim()));
                }
            } else if line.starts_with("IPv6 Address") {
                if let Some(ip) = line.split(':').nth(1) {
                    result.push(format!("        inet6 {}", ip.trim()));
                }
            } else if line.starts_with("Physical Address") {
                if let Some(mac) = line.split(':').nth(1) {
                    let mac = mac.trim().replace('-', ":");
                    result.push(format!("        ether {}", mac.to_lowercase()));
                }
            } else if line.starts_with("Subnet Mask") {
                // Already included in inet line
            } else if line.starts_with("Default Gateway") {
                if let Some(gw) = line.split(':').nth(1) {
                    let gw = gw.trim();
                    if !gw.is_empty() {
                        result.push(format!("        gateway {}", gw));
                    }
                }
            }
        }

        if result.is_empty() {
            Ok("No network interfaces found".to_string())
        } else {
            Ok(result.join("\n").trim_start().to_string())
        }
    }
}
