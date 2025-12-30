//! free command - display amount of free and used memory

use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct FreeCommand;

impl Command for FreeCommand {
    fn name(&self) -> &'static str {
        "free"
    }

    fn description(&self) -> &'static str {
        "Display amount of free and used memory"
    }

    fn usage(&self) -> &'static str {
        "free [-h] [-m] [-g]"
    }

    fn execute(&self, args: &[String], _state: &mut TerminalState) -> Result<String> {
        let mut human = false;
        let mut megabytes = false;
        let mut gigabytes = false;

        for arg in args {
            match arg.as_str() {
                "-h" | "--human" => human = true,
                "-m" | "--mega" => megabytes = true,
                "-g" | "--giga" => gigabytes = true,
                "--help" => {
                    return Ok("Usage: free [OPTIONS]\n\
                        Options:\n  \
                        -h    Human-readable output\n  \
                        -m    Show output in megabytes\n  \
                        -g    Show output in gigabytes"
                        .to_string());
                }
                _ => {}
            }
        }

        // Get memory info via PowerShell
        let output = std::process::Command::new("powershell")
            .args([
                "-NoProfile",
                "-Command",
                "$os = Get-CimInstance Win32_OperatingSystem; \
                 $total = $os.TotalVisibleMemorySize * 1024; \
                 $free = $os.FreePhysicalMemory * 1024; \
                 $used = $total - $free; \
                 \"$total,$used,$free\"",
            ])
            .output()?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let parts: Vec<&str> = stdout.trim().split(',').collect();

            if parts.len() >= 3 {
                let total: u64 = parts[0].parse().unwrap_or(0);
                let used: u64 = parts[1].parse().unwrap_or(0);
                let free: u64 = parts[2].parse().unwrap_or(0);

                let format_size = |bytes: u64| -> String {
                    if human {
                        format_human(bytes)
                    } else if gigabytes {
                        format!("{:.1}", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
                    } else if megabytes {
                        format!("{}", bytes / (1024 * 1024))
                    } else {
                        format!("{}", bytes / 1024) // KB by default
                    }
                };

                let unit = if human {
                    ""
                } else if gigabytes {
                    "Gi"
                } else if megabytes {
                    "Mi"
                } else {
                    "Ki"
                };

                let header = format!("{:>14} {:>14} {:>14}", "total", "used", "free");
                let mem_line = format!(
                    "Mem: {:>10}{} {:>10}{} {:>10}{}",
                    format_size(total),
                    unit,
                    format_size(used),
                    unit,
                    format_size(free),
                    unit
                );

                Ok(format!("{}\n{}", header, mem_line))
            } else {
                Err(anyhow::anyhow!("free: cannot parse memory info"))
            }
        } else {
            Err(anyhow::anyhow!("free: cannot get memory info"))
        }
    }
}

fn format_human(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;

    if bytes >= TB {
        format!("{:.1}T", bytes as f64 / TB as f64)
    } else if bytes >= GB {
        format!("{:.1}G", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1}M", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1}K", bytes as f64 / KB as f64)
    } else {
        format!("{}B", bytes)
    }
}
