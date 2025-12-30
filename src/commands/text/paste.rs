//! paste command - merge lines of files

use anyhow::Result;
use std::fs;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct PasteCommand;

impl Command for PasteCommand {
    fn name(&self) -> &'static str {
        "paste"
    }

    fn description(&self) -> &'static str {
        "Merge lines of files"
    }

    fn usage(&self) -> &'static str {
        "paste [-d <delim>] <file1> <file2>"
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        let mut delimiter = "\t".to_string();
        let mut files: Vec<&String> = Vec::new();

        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "-d" | "--delimiters" => {
                    if i + 1 < args.len() {
                        delimiter = args[i + 1].clone();
                        i += 1;
                    }
                }
                "-h" | "--help" => {
                    return Ok("Usage: paste [OPTIONS] <file1> <file2> ...\n\
                        Options:\n  \
                        -d <chars>    Use characters from <chars> as delimiters"
                        .to_string());
                }
                _ if !args[i].starts_with('-') => files.push(&args[i]),
                _ => {}
            }
            i += 1;
        }

        if files.len() < 2 {
            return Err(anyhow::anyhow!("paste: need at least two files"));
        }

        // Read all files into vectors of lines
        let mut file_lines: Vec<Vec<String>> = Vec::new();
        let mut max_lines = 0;

        for file in &files {
            let path = state.resolve_path(file);
            let content =
                fs::read_to_string(&path).map_err(|e| anyhow::anyhow!("paste: {}: {}", file, e))?;
            let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
            max_lines = max_lines.max(lines.len());
            file_lines.push(lines);
        }

        // Merge lines
        let mut result: Vec<String> = Vec::new();
        for line_idx in 0..max_lines {
            let merged: Vec<&str> = file_lines
                .iter()
                .map(|lines| lines.get(line_idx).map(|s| s.as_str()).unwrap_or(""))
                .collect();
            result.push(merged.join(&delimiter));
        }

        Ok(result.join("\n"))
    }
}
