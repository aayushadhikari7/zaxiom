//! sort command - sort lines of text

use anyhow::Result;
use std::fs;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct SortCommand;

impl Command for SortCommand {
    fn name(&self) -> &'static str {
        "sort"
    }

    fn description(&self) -> &'static str {
        "Sort lines of text"
    }

    fn usage(&self) -> &'static str {
        "sort [-r] [-n] [-u] [file...]"
    }

    fn extended_help(&self) -> String {
        r#"sort - Sort lines of text

USAGE:
  sort [OPTIONS] <file...>
  command | sort [OPTIONS]

OPTIONS:
  -r, --reverse      Reverse sort order (descending)
  -n, --numeric      Sort numerically
  -u, --unique       Output only unique lines
  -f, --ignore-case  Case-insensitive sort

DESCRIPTION:
  Write sorted concatenation of all files to output.
  By default, sorts alphabetically in ascending order.

EXAMPLES:
  sort file.txt              Alphabetical sort
  sort -r file.txt           Reverse order (Z to A)
  sort -n numbers.txt        Numeric sort (1, 2, 10, not 1, 10, 2)
  sort -u file.txt           Remove duplicates
  sort -rn scores.txt        Numeric, highest first
  cat *.log | sort           Sort piped input
  sort -f names.txt          Ignore case

COMBINING OPTIONS:
  sort -rnu file.txt         Reverse numeric unique

COMMON USE CASES:
  • Sort log files by timestamp
  • Find unique entries
  • Order numeric data
  • Alphabetize lists

RELATED COMMANDS:
  uniq     Remove duplicates (requires sorted input)
  wc       Count lines
  head     First N lines
  tail     Last N lines
"#
        .to_string()
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        self.execute_with_stdin(args, None, state)
    }

    fn execute_with_stdin(
        &self,
        args: &[String],
        stdin: Option<&str>,
        state: &mut TerminalState,
    ) -> Result<String> {
        let mut reverse = false;
        let mut numeric = false;
        let mut unique = false;
        let mut files: Vec<&String> = Vec::new();

        for arg in args {
            match arg.as_str() {
                "-r" | "--reverse" => reverse = true,
                "-n" | "--numeric-sort" => numeric = true,
                "-u" | "--unique" => unique = true,
                "-h" | "--help" => {
                    return Ok("Usage: sort [OPTIONS] [FILE...]\n\
                        Options:\n  \
                        -r    Reverse the result\n  \
                        -n    Compare according to string numerical value\n  \
                        -u    Output only unique lines"
                        .to_string());
                }
                _ if !arg.starts_with('-') => files.push(arg),
                _ => {}
            }
        }

        let mut all_lines: Vec<String> = Vec::new();

        // Use stdin if no files specified
        if files.is_empty() {
            if let Some(input) = stdin {
                all_lines.extend(input.lines().map(|s| s.to_string()));
            } else {
                return Err(anyhow::anyhow!("sort: missing file operand"));
            }
        } else {
            for file in &files {
                let path = state.resolve_path(file);
                let content = fs::read_to_string(&path)
                    .map_err(|e| anyhow::anyhow!("sort: {}: {}", file, e))?;
                all_lines.extend(content.lines().map(|s| s.to_string()));
            }
        }

        if numeric {
            all_lines.sort_by(|a, b| {
                let a_num: f64 = a
                    .split_whitespace()
                    .next()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(f64::MAX);
                let b_num: f64 = b
                    .split_whitespace()
                    .next()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(f64::MAX);
                a_num
                    .partial_cmp(&b_num)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
        } else {
            all_lines.sort();
        }

        if reverse {
            all_lines.reverse();
        }

        if unique {
            all_lines.dedup();
        }

        Ok(all_lines.join("\n"))
    }

    fn supports_stdin(&self) -> bool {
        true
    }
}
