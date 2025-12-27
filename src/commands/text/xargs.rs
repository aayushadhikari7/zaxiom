//! xargs command - build and execute commands from stdin

use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct XargsCommand;

impl Command for XargsCommand {
    fn name(&self) -> &'static str {
        "xargs"
    }

    fn description(&self) -> &'static str {
        "Build and execute commands from stdin"
    }

    fn usage(&self) -> &'static str {
        "xargs [-n num] [-d delim] [command [args...]]"
    }

    fn supports_stdin(&self) -> bool {
        true
    }

    fn execute(&self, _args: &[String], _state: &mut TerminalState) -> Result<String> {
        // Without stdin, just show usage
        Ok("xargs: requires stdin input (use with pipe)".to_string())
    }

    fn execute_with_stdin(&self, args: &[String], stdin: Option<&str>, _state: &mut TerminalState) -> Result<String> {
        let input = stdin.unwrap_or("");
        if input.is_empty() {
            return Ok(String::new());
        }

        let mut max_args: Option<usize> = None;
        let mut delimiter = ' ';
        let mut cmd_start = 0;

        // Parse options
        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "-n" => {
                    if i + 1 < args.len() {
                        max_args = args[i + 1].parse().ok();
                        i += 2;
                        cmd_start = i;
                        continue;
                    }
                }
                "-d" => {
                    if i + 1 < args.len() {
                        delimiter = args[i + 1].chars().next().unwrap_or(' ');
                        i += 2;
                        cmd_start = i;
                        continue;
                    }
                }
                "-0" => {
                    delimiter = '\0';
                    i += 1;
                    cmd_start = i;
                    continue;
                }
                _ => break,
            }
            i += 1;
        }

        // Get command and args (default to echo)
        let base_cmd = if cmd_start < args.len() {
            args[cmd_start..].to_vec()
        } else {
            vec!["echo".to_string()]
        };

        // Split input by delimiter
        let items: Vec<&str> = if delimiter == ' ' {
            input.split_whitespace().collect()
        } else {
            input.split(delimiter).filter(|s| !s.is_empty()).collect()
        };

        // Build output showing what would be executed
        let mut output = Vec::new();

        match max_args {
            Some(n) => {
                for chunk in items.chunks(n) {
                    let cmd_line = format!("{} {}", base_cmd.join(" "), chunk.join(" "));
                    output.push(cmd_line);
                }
            }
            None => {
                let cmd_line = format!("{} {}", base_cmd.join(" "), items.join(" "));
                output.push(cmd_line);
            }
        }

        Ok(output.join("\n"))
    }
}
