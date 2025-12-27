//! nl command - number lines of files

use std::fs;
use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct NlCommand;

impl Command for NlCommand {
    fn name(&self) -> &'static str {
        "nl"
    }

    fn description(&self) -> &'static str {
        "Number lines of files"
    }

    fn usage(&self) -> &'static str {
        "nl [-b a|t|n] [file]"
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        self.execute_with_stdin(args, None, state)
    }

    fn execute_with_stdin(&self, args: &[String], stdin: Option<&str>, state: &mut TerminalState) -> Result<String> {
        let mut number_all = true; // -b a (number all lines)
        let mut file: Option<&String> = None;

        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "-b" if i + 1 < args.len() => {
                    match args[i + 1].as_str() {
                        "a" => number_all = true,
                        "t" => number_all = false, // only non-empty
                        "n" => number_all = false, // no numbering
                        _ => {}
                    }
                    i += 1;
                }
                "-h" | "--help" => {
                    return Ok("Usage: nl [OPTIONS] [FILE]\n\
                        Options:\n  \
                        -b a    Number all lines\n  \
                        -b t    Number only non-empty lines\n  \
                        -b n    No line numbering".to_string());
                }
                _ if !args[i].starts_with('-') => file = Some(&args[i]),
                _ => {}
            }
            i += 1;
        }

        let content = if let Some(f) = file {
            let path = state.resolve_path(f);
            fs::read_to_string(&path)
                .map_err(|e| anyhow::anyhow!("nl: {}: {}", f, e))?
        } else if let Some(input) = stdin {
            input.to_string()
        } else {
            return Err(anyhow::anyhow!("nl: no input"));
        };

        let mut line_number = 1;
        let mut output = Vec::new();

        for line in content.lines() {
            if number_all || !line.is_empty() {
                output.push(format!("{:6}\t{}", line_number, line));
                line_number += 1;
            } else {
                output.push(format!("      \t{}", line));
            }
        }

        Ok(output.join("\n"))
    }

    fn supports_stdin(&self) -> bool {
        true
    }
}
