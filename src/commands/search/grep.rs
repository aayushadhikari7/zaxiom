//! grep command - search file contents

use std::fs;
use std::io::{BufRead, BufReader};

use anyhow::Result;
use regex::Regex;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct GrepCommand;

impl Command for GrepCommand {
    fn name(&self) -> &'static str {
        "grep"
    }

    fn description(&self) -> &'static str {
        "Search for pattern in files"
    }

    fn usage(&self) -> &'static str {
        "grep [-i] [-n] [-v] <pattern> [file...]"
    }

    fn extended_help(&self) -> String {
        r#"grep - Search for patterns in files

USAGE:
  grep [OPTIONS] <pattern> [file...]
  command | grep <pattern>

OPTIONS:
  -i, --ignore-case     Case-insensitive matching
  -n, --line-number     Show line numbers
  -v, --invert-match    Show lines that DON'T match
  -in, -ni              Combine -i and -n

DESCRIPTION:
  Search for PATTERN in each FILE or standard input.
  PATTERN is a regular expression.

EXAMPLES:
  grep "error" log.txt           Find "error" in log.txt
  grep -i "error" log.txt        Case-insensitive search
  grep -n "TODO" *.rs            Show line numbers, search all .rs files
  grep -v "^#" config.txt        Show lines not starting with #
  cat file.txt | grep "pattern"  Search in piped input
  grep "func.*\(" *.js           Regex: find function definitions

REGEX PATTERNS:
  .        Any character
  *        Zero or more of previous
  +        One or more of previous
  ?        Zero or one of previous
  ^        Start of line
  $        End of line
  [abc]    Character class
  [^abc]   Negated character class
  \d       Digit
  \w       Word character
  \s       Whitespace

RELATED COMMANDS:
  find     Find files by name
  cat      Display file contents
  sed      Stream editor
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
        let mut case_insensitive = false;
        let mut show_line_numbers = false;
        let mut invert_match = false;
        let mut pattern_str = None;
        let mut files = Vec::new();

        for arg in args {
            match arg.as_str() {
                "-i" | "--ignore-case" => case_insensitive = true,
                "-n" | "--line-number" => show_line_numbers = true,
                "-v" | "--invert-match" => invert_match = true,
                "-h" | "--help" => {
                    return Ok("Usage: grep [OPTIONS] <pattern> [file...]\n\
                        Options:\n  \
                        -i    Ignore case\n  \
                        -n    Show line numbers\n  \
                        -v    Invert match (show non-matching lines)"
                        .to_string());
                }
                "-in" | "-ni" => {
                    case_insensitive = true;
                    show_line_numbers = true;
                }
                _ if !arg.starts_with('-') => {
                    if pattern_str.is_none() {
                        pattern_str = Some(arg.as_str());
                    } else {
                        files.push(arg.as_str());
                    }
                }
                _ => {}
            }
        }

        let pattern = pattern_str.ok_or_else(|| anyhow::anyhow!("grep: missing pattern"))?;

        // Build regex
        let regex_pattern = if case_insensitive {
            format!("(?i){}", pattern)
        } else {
            pattern.to_string()
        };

        let regex = Regex::new(&regex_pattern)
            .map_err(|e| anyhow::anyhow!("grep: invalid pattern: {}", e))?;

        let mut output = Vec::new();

        // If no files specified but we have stdin, use stdin
        if files.is_empty() {
            if let Some(input) = stdin {
                for (line_num, line) in input.lines().enumerate() {
                    let matches = regex.is_match(line);
                    if matches != invert_match {
                        let mut result = String::new();
                        if show_line_numbers {
                            result.push_str(&format!("{}:", line_num + 1));
                        }
                        result.push_str(line);
                        output.push(result);
                    }
                }
                return Ok(output.join("\n"));
            } else {
                return Err(anyhow::anyhow!("grep: no input files"));
            }
        }

        let show_filename = files.len() > 1;

        for file in files {
            let path = state.resolve_path(file);

            if !path.exists() {
                output.push(format!("grep: {}: No such file", file));
                continue;
            }

            if path.is_dir() {
                output.push(format!("grep: {}: Is a directory", file));
                continue;
            }

            let file_handle = match fs::File::open(&path) {
                Ok(f) => f,
                Err(e) => {
                    output.push(format!("grep: {}: {}", file, e));
                    continue;
                }
            };

            let reader = BufReader::new(file_handle);

            for (line_num, line) in reader.lines().enumerate() {
                if let Ok(line) = line {
                    let matches = regex.is_match(&line);
                    if matches != invert_match {
                        let mut result = String::new();

                        if show_filename {
                            result.push_str(file);
                            result.push(':');
                        }

                        if show_line_numbers {
                            result.push_str(&format!("{}:", line_num + 1));
                        }

                        result.push_str(&line);
                        output.push(result);
                    }
                }
            }
        }

        Ok(output.join("\n"))
    }

    fn supports_stdin(&self) -> bool {
        true
    }
}
