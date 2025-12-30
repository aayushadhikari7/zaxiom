//! du command - estimate file space usage

use anyhow::Result;
use walkdir::WalkDir;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct DuCommand;

impl Command for DuCommand {
    fn name(&self) -> &'static str {
        "du"
    }

    fn description(&self) -> &'static str {
        "Estimate file space usage"
    }

    fn usage(&self) -> &'static str {
        "du [-h] [-s] [-d n] [file...]"
    }

    fn extended_help(&self) -> String {
        r#"du - Estimate file space usage

USAGE:
  du [OPTIONS] [FILE...]

OPTIONS:
  -h, --human-readable   Print sizes in human format (K, M, G)
  -s, --summarize        Display only total for each argument
  -d, --max-depth N      Print totals for directories N levels deep

DESCRIPTION:
  Summarize disk usage of files and directories recursively.
  Useful for finding large folders eating up disk space.

EXAMPLES:
  du                     Current directory breakdown
  du -h                  Human-readable sizes
  du -sh *               Summary of each item in current dir
  du -sh Downloads/      Total size of Downloads folder
  du -h -d 1             One level deep breakdown
  du -sh node_modules/   Check node_modules size

COMMON USE CASES:
  • Find what's using disk space
  • Identify large folders
  • Check project sizes
  • Compare directory sizes

FINDING LARGE DIRECTORIES:
  du -sh * | sort -h     Sort directories by size
  du -h -d 1 | sort -h   Sorted one-level breakdown

OUTPUT FORMAT:
  <size>    <path>
  4.5G      ./node_modules
  256M      ./target
  12K       ./src

TIPS:
  • Use -h for readability
  • Use -s for quick totals
  • Combine with sort for ranking

RELATED COMMANDS:
  df       Disk free space
  ls -lh   List files with sizes
  find     Find files by size
"#
        .to_string()
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        let mut human_readable = false;
        let mut summarize = false;
        let mut max_depth: Option<usize> = None;
        let mut paths: Vec<String> = Vec::new();

        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "-h" | "--human-readable" => human_readable = true,
                "-s" | "--summarize" => summarize = true,
                "-d" | "--max-depth" => {
                    if i + 1 < args.len() {
                        max_depth = args[i + 1].parse().ok();
                        i += 1;
                    }
                }
                "--help" => {
                    return Ok("Usage: du [OPTIONS] [FILE...]\n\
                        Options:\n  \
                        -h    Human-readable sizes\n  \
                        -s    Display only total for each argument\n  \
                        -d N  Print total for directory only if N or fewer levels deep"
                        .to_string());
                }
                _ if !args[i].starts_with('-') => paths.push(args[i].clone()),
                _ => {}
            }
            i += 1;
        }

        if paths.is_empty() {
            paths.push(".".to_string());
        }

        let mut output = Vec::new();

        for path_str in &paths {
            let path = state.resolve_path(path_str);

            if summarize {
                let size = calculate_dir_size(&path)?;
                output.push(format!(
                    "{}\t{}",
                    format_size(size, human_readable),
                    path.display()
                ));
            } else {
                for entry in WalkDir::new(&path)
                    .min_depth(0)
                    .max_depth(max_depth.unwrap_or(usize::MAX))
                    .into_iter()
                    .filter_map(|e| e.ok())
                {
                    if entry.file_type().is_dir() {
                        let size = calculate_dir_size(entry.path())?;
                        output.push(format!(
                            "{}\t{}",
                            format_size(size, human_readable),
                            entry.path().display()
                        ));
                    }
                }
            }
        }

        Ok(output.join("\n"))
    }
}

fn calculate_dir_size(path: &std::path::Path) -> Result<u64> {
    let mut total = 0u64;

    if path.is_file() {
        return Ok(std::fs::metadata(path)?.len());
    }

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            total += entry.metadata().map(|m| m.len()).unwrap_or(0);
        }
    }

    Ok(total)
}

fn format_size(bytes: u64, human_readable: bool) -> String {
    if !human_readable {
        return format!("{:>8}", bytes / 1024); // KB by default
    }

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
