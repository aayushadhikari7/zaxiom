//! tail command - print last N lines

use std::fs;
use std::io::{BufRead, BufReader};
use std::collections::VecDeque;

use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct TailCommand;

impl Command for TailCommand {
    fn name(&self) -> &'static str {
        "tail"
    }

    fn description(&self) -> &'static str {
        "Print last N lines of file"
    }

    fn usage(&self) -> &'static str {
        "tail [-n lines] [file]"
    }

    fn extended_help(&self) -> String {
        r#"tail - Print last lines of a file

USAGE:
  tail [OPTIONS] <file>
  command | tail [OPTIONS]

OPTIONS:
  -n <lines>    Number of lines to show (default: 10)
  -<number>     Shorthand for -n (e.g., tail -5 file)
  -f            Follow file (coming soon)

DESCRIPTION:
  Output the last part of files. By default, prints
  the last 10 lines of each file.

EXAMPLES:
  tail file.txt           Last 10 lines
  tail -n 5 file.txt      Last 5 lines
  tail -20 file.txt       Last 20 lines (shorthand)
  tail -n 1 file.txt      Last line only
  cat log.txt | tail      Last 10 lines of piped input

COMMON USE CASES:
  • View recent log entries
  • Check end of large files
  • Monitor file updates
  • Extract final results

RELATED COMMANDS:
  head     Print first lines
  cat      Print entire file
  wc       Count lines/words
"#.to_string()
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        self.execute_with_stdin(args, None, state)
    }

    fn execute_with_stdin(&self, args: &[String], stdin: Option<&str>, state: &mut TerminalState) -> Result<String> {
        let mut lines = 10usize;
        let mut file_path = None;

        let mut iter = args.iter().peekable();
        while let Some(arg) = iter.next() {
            match arg.as_str() {
                "-n" => {
                    if let Some(n) = iter.next() {
                        lines = n.parse().map_err(|_| anyhow::anyhow!("tail: invalid line count"))?;
                    }
                }
                "-h" | "--help" => {
                    return Ok("Usage: tail [-n lines] [file]\n\
                        Print last N lines (default: 10)".to_string());
                }
                _ if arg.starts_with("-n") => {
                    lines = arg[2..].parse().map_err(|_| anyhow::anyhow!("tail: invalid line count"))?;
                }
                _ if arg.starts_with('-') && arg.len() > 1 && arg[1..].chars().all(|c| c.is_ascii_digit()) => {
                    // Handle -N shorthand
                    lines = arg[1..].parse().map_err(|_| anyhow::anyhow!("tail: invalid line count"))?;
                }
                _ if !arg.starts_with('-') => {
                    file_path = Some(arg.as_str());
                }
                _ => {}
            }
        }

        // Use stdin if no file specified
        if file_path.is_none() {
            if let Some(input) = stdin {
                let all_lines: Vec<&str> = input.lines().collect();
                let start = all_lines.len().saturating_sub(lines);
                return Ok(all_lines[start..].join("\n"));
            } else {
                return Err(anyhow::anyhow!("tail: no input file"));
            }
        }

        let file = file_path.unwrap();
        let path = state.resolve_path(file);

        if !path.exists() {
            return Err(anyhow::anyhow!("tail: {}: No such file", file));
        }

        let file = fs::File::open(&path)?;
        let reader = BufReader::new(file);

        // Keep last N lines in a ring buffer
        let mut ring: VecDeque<String> = VecDeque::with_capacity(lines);

        for line in reader.lines() {
            if let Ok(l) = line {
                if ring.len() >= lines {
                    ring.pop_front();
                }
                ring.push_back(l);
            }
        }

        let output: Vec<String> = ring.into_iter().collect();
        Ok(output.join("\n"))
    }

    fn supports_stdin(&self) -> bool {
        true
    }
}
