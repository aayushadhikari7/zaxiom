//! strings command - extract printable strings from binary files

use std::fs::File;
use std::io::{BufReader, Read};

use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct StringsCommand;

impl Command for StringsCommand {
    fn name(&self) -> &'static str {
        "strings"
    }

    fn description(&self) -> &'static str {
        "Print printable strings from files"
    }

    fn usage(&self) -> &'static str {
        "strings [-n min_length] [file...]"
    }

    fn supports_stdin(&self) -> bool {
        true
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        let mut min_length = 4usize;
        let mut files: Vec<&str> = Vec::new();

        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "-n" => {
                    if i + 1 < args.len() {
                        min_length = args[i + 1].parse().unwrap_or(4);
                        i += 1;
                    }
                }
                arg if !arg.starts_with('-') => files.push(arg),
                _ => {}
            }
            i += 1;
        }

        if files.is_empty() {
            return Err(anyhow::anyhow!("strings: no input files"));
        }

        let mut output = Vec::new();

        for file in files {
            let path = state.resolve_path(file);
            let f = File::open(&path)?;
            let mut reader = BufReader::new(f);
            let mut buffer = Vec::new();
            reader.read_to_end(&mut buffer)?;

            let strings = extract_strings(&buffer, min_length);
            output.extend(strings);
        }

        Ok(output.join("\n"))
    }

    fn execute_with_stdin(
        &self,
        args: &[String],
        stdin: Option<&str>,
        state: &mut TerminalState,
    ) -> Result<String> {
        if let Some(input) = stdin {
            let mut min_length = 4usize;
            for i in 0..args.len() {
                if args[i] == "-n" && i + 1 < args.len() {
                    min_length = args[i + 1].parse().unwrap_or(4);
                }
            }
            let strings = extract_strings(input.as_bytes(), min_length);
            Ok(strings.join("\n"))
        } else {
            self.execute(args, state)
        }
    }
}

fn extract_strings(data: &[u8], min_length: usize) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = String::new();

    for &byte in data {
        if (0x20..0x7f).contains(&byte) {
            current.push(byte as char);
        } else {
            if current.len() >= min_length {
                result.push(current.clone());
            }
            current.clear();
        }
    }

    if current.len() >= min_length {
        result.push(current);
    }

    result
}
