//! head command - print first N lines

use std::fs;
use std::io::{BufRead, BufReader};

use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct HeadCommand;

impl Command for HeadCommand {
    fn name(&self) -> &'static str {
        "head"
    }

    fn description(&self) -> &'static str {
        "Print first N lines of file"
    }

    fn usage(&self) -> &'static str {
        "head [-n lines] [file]"
    }

    fn extended_help(&self) -> String {
        r#"head - Print first lines of a file

USAGE:
  head [OPTIONS] <file>
  command | head [OPTIONS]

OPTIONS:
  -n <lines>    Number of lines to show (default: 10)
  -<number>     Shorthand for -n (e.g., head -5 file)

DESCRIPTION:
  Output the first part of files. By default, prints
  the first 10 lines of each file.

EXAMPLES:
  head file.txt           First 10 lines
  head -n 5 file.txt      First 5 lines
  head -20 file.txt       First 20 lines (shorthand)
  head -n 1 file.txt      First line only
  cat log.txt | head      First 10 lines of piped input
  head *.log              First 10 lines of each file

COMMON USE CASES:
  • Preview file contents
  • Get file headers (CSV, logs)
  • Quick file inspection
  • Limit output in pipelines

RELATED COMMANDS:
  tail     Print last lines
  cat      Print entire file
  wc       Count lines/words
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
        let mut lines = 10usize;
        let mut file_path = None;

        let mut iter = args.iter().peekable();
        while let Some(arg) = iter.next() {
            match arg.as_str() {
                "-n" => {
                    if let Some(n) = iter.next() {
                        lines = n
                            .parse()
                            .map_err(|_| anyhow::anyhow!("head: invalid line count"))?;
                    }
                }
                "-h" | "--help" => {
                    return Ok("Usage: head [-n lines] [file]\n\
                        Print first N lines (default: 10)"
                        .to_string());
                }
                _ if arg.starts_with("-n") => {
                    lines = arg[2..]
                        .parse()
                        .map_err(|_| anyhow::anyhow!("head: invalid line count"))?;
                }
                _ if arg.starts_with('-')
                    && arg.len() > 1
                    && arg[1..].chars().all(|c| c.is_ascii_digit()) =>
                {
                    // Handle -N shorthand (e.g., -5 for first 5 lines)
                    lines = arg[1..]
                        .parse()
                        .map_err(|_| anyhow::anyhow!("head: invalid line count"))?;
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
                let output: Vec<&str> = input.lines().take(lines).collect();
                return Ok(output.join("\n"));
            } else {
                return Err(anyhow::anyhow!("head: no input file"));
            }
        }

        let file = file_path.unwrap();
        let path = state.resolve_path(file);

        if !path.exists() {
            return Err(anyhow::anyhow!("head: {}: No such file", file));
        }

        let file = fs::File::open(&path)?;
        let reader = BufReader::new(file);

        let output: Vec<String> = reader.lines().take(lines).filter_map(|l| l.ok()).collect();

        Ok(output.join("\n"))
    }

    fn supports_stdin(&self) -> bool {
        true
    }
}
