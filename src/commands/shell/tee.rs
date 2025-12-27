//! tee command - read from stdin and write to stdout and files

use std::fs::{File, OpenOptions};
use std::io::Write;
use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct TeeCommand;

impl Command for TeeCommand {
    fn name(&self) -> &'static str {
        "tee"
    }

    fn description(&self) -> &'static str {
        "Read from stdin and write to stdout and files"
    }

    fn usage(&self) -> &'static str {
        "tee [-a] <file...>"
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        self.execute_with_stdin(args, None, state)
    }

    fn execute_with_stdin(&self, args: &[String], stdin: Option<&str>, state: &mut TerminalState) -> Result<String> {
        let mut append = false;
        let mut files: Vec<&String> = Vec::new();

        for arg in args {
            match arg.as_str() {
                "-a" | "--append" => append = true,
                "-h" | "--help" => {
                    return Ok("Usage: tee [OPTIONS] FILE...\n\
                        Copy stdin to each FILE and also to stdout.\n\n\
                        Options:\n  \
                        -a    Append to files instead of overwriting".to_string());
                }
                _ if !arg.starts_with('-') => files.push(arg),
                _ => {}
            }
        }

        let input = stdin.ok_or_else(|| anyhow::anyhow!("tee: no input"))?;

        // Write to all files
        for file in &files {
            let path = state.resolve_path(file);
            let mut handle = if append {
                OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&path)
                    .map_err(|e| anyhow::anyhow!("tee: {}: {}", file, e))?
            } else {
                File::create(&path)
                    .map_err(|e| anyhow::anyhow!("tee: {}: {}", file, e))?
            };

            writeln!(handle, "{}", input)
                .map_err(|e| anyhow::anyhow!("tee: {}: {}", file, e))?;
        }

        // Also output to stdout
        Ok(input.to_string())
    }

    fn supports_stdin(&self) -> bool {
        true
    }
}
