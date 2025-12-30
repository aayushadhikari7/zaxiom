//! df command - report file system disk space usage

use anyhow::Result;
use std::path::Path;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct DfCommand;

impl Command for DfCommand {
    fn name(&self) -> &'static str {
        "df"
    }

    fn description(&self) -> &'static str {
        "Report file system disk space usage"
    }

    fn usage(&self) -> &'static str {
        "df [-h]"
    }

    fn extended_help(&self) -> String {
        r#"df - Report file system disk space usage

USAGE:
  df [OPTIONS]

OPTIONS:
  -h, --human-readable    Print sizes in human format (K, M, G)

DESCRIPTION:
  Display amount of disk space available on mounted drives.
  Shows total, used, and free space for each drive.

EXAMPLES:
  df                 Show disk usage (raw bytes)
  df -h              Human-readable sizes (recommended!)

OUTPUT COLUMNS:
  Filesystem   Drive letter (C:, D:, etc.)
  Size         Total drive capacity
  Used         Space used
  Avail        Space available
  Use%         Percentage used

SAMPLE OUTPUT (with -h):
  Filesystem       Size      Used     Avail  Use%
  C:              500.0G    320.5G    179.5G    64%
  D:              1.0T      756.2G    267.8G    74%

COMMON USE CASES:
  • Check available disk space
  • Monitor drive usage
  • Find drives running low
  • Plan storage needs

LOW DISK WARNING:
  When Use% > 90%, consider:
  • Clearing temp files
  • Uninstalling unused programs
  • Moving files to another drive

RELATED COMMANDS:
  du       Estimate file/directory space usage
  ls -l    List files with sizes
"#
        .to_string()
    }

    fn execute(&self, args: &[String], _state: &mut TerminalState) -> Result<String> {
        let mut human_readable = false;

        for arg in args {
            match arg.as_str() {
                "-h" | "--human-readable" => human_readable = true,
                "--help" => {
                    return Ok("Usage: df [OPTIONS]\n\
                        Options:\n  \
                        -h    Human-readable sizes"
                        .to_string());
                }
                _ => {}
            }
        }

        // Use PowerShell to get disk info
        let output = std::process::Command::new("powershell")
            .args([
                "-NoProfile",
                "-Command",
                "Get-PSDrive -PSProvider FileSystem | Select-Object Name, @{N='Used';E={$_.Used}}, @{N='Free';E={$_.Free}}, @{N='Size';E={$_.Used + $_.Free}} | Format-Table -AutoSize"
            ])
            .output()?;

        if output.status.success() {
            let raw = String::from_utf8_lossy(&output.stdout);

            if human_readable {
                // Parse and reformat with human-readable sizes
                let mut lines: Vec<String> = Vec::new();
                lines.push(format!(
                    "{:<12} {:>10} {:>10} {:>10} {:>6}",
                    "Filesystem", "Size", "Used", "Avail", "Use%"
                ));

                for line in raw.lines().skip(3) {
                    // Skip headers
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 4 {
                        let name = parts[0];
                        let used: u64 = parts[1].parse().unwrap_or(0);
                        let free: u64 = parts[2].parse().unwrap_or(0);
                        let total: u64 = parts[3].parse().unwrap_or(1);

                        let use_pct = if total > 0 {
                            (used as f64 / total as f64 * 100.0) as u8
                        } else {
                            0
                        };

                        lines.push(format!(
                            "{:<12} {:>10} {:>10} {:>10} {:>5}%",
                            format!("{}:", name),
                            format_size(total),
                            format_size(used),
                            format_size(free),
                            use_pct
                        ));
                    }
                }

                Ok(lines.join("\n"))
            } else {
                Ok(raw.to_string())
            }
        } else {
            // Fallback: basic disk info
            let mut output_lines = vec![format!(
                "{:<12} {:>15} {:>15} {:>15}",
                "Filesystem", "1K-blocks", "Used", "Available"
            )];

            // Check common drive letters
            for letter in ['C', 'D', 'E', 'F'] {
                let path = format!("{}:\\", letter);
                if Path::new(&path).exists() {
                    output_lines.push(format!(
                        "{:<12} {:>15} {:>15} {:>15}",
                        format!("{}:", letter),
                        "N/A",
                        "N/A",
                        "N/A"
                    ));
                }
            }

            Ok(output_lines.join("\n"))
        }
    }
}

fn format_size(bytes: u64) -> String {
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
