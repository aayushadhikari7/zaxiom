//! join command - join lines of two files on a common field

use std::collections::HashMap;
use std::fs;

use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct JoinCommand;

impl Command for JoinCommand {
    fn name(&self) -> &'static str {
        "join"
    }

    fn description(&self) -> &'static str {
        "Join lines of two files on a common field"
    }

    fn usage(&self) -> &'static str {
        "join [-1 field] [-2 field] [-t sep] file1 file2"
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        let mut field1 = 1usize;
        let mut field2 = 1usize;
        let mut separator = ' ';
        let mut files: Vec<&str> = Vec::new();

        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "-1" => {
                    if i + 1 < args.len() {
                        field1 = args[i + 1].parse().unwrap_or(1);
                        i += 1;
                    }
                }
                "-2" => {
                    if i + 1 < args.len() {
                        field2 = args[i + 1].parse().unwrap_or(1);
                        i += 1;
                    }
                }
                "-t" => {
                    if i + 1 < args.len() {
                        separator = args[i + 1].chars().next().unwrap_or(' ');
                        i += 1;
                    }
                }
                arg if !arg.starts_with('-') => files.push(arg),
                _ => {}
            }
            i += 1;
        }

        if files.len() < 2 {
            return Err(anyhow::anyhow!("join: requires two files"));
        }

        let path1 = state.resolve_path(files[0]);
        let path2 = state.resolve_path(files[1]);

        let content1 = fs::read_to_string(&path1)?;
        let content2 = fs::read_to_string(&path2)?;

        // Build hash map from first file
        let mut map1: HashMap<String, Vec<String>> = HashMap::new();
        for line in content1.lines() {
            let fields: Vec<&str> = if separator == ' ' {
                line.split_whitespace().collect()
            } else {
                line.split(separator).collect()
            };
            if let Some(key) = fields.get(field1.saturating_sub(1)) {
                map1.entry(key.to_string())
                    .or_default()
                    .push(line.to_string());
            }
        }

        // Join with second file
        let mut output = Vec::new();
        for line in content2.lines() {
            let fields: Vec<&str> = if separator == ' ' {
                line.split_whitespace().collect()
            } else {
                line.split(separator).collect()
            };
            if let Some(key) = fields.get(field2.saturating_sub(1)) {
                if let Some(lines1) = map1.get(*key) {
                    for line1 in lines1 {
                        // Remove key from line2 to avoid duplication
                        let other_fields: Vec<&str> = fields
                            .iter()
                            .enumerate()
                            .filter(|(i, _)| *i != field2.saturating_sub(1))
                            .map(|(_, f)| *f)
                            .collect();
                        let sep_str = if separator == ' ' {
                            " "
                        } else {
                            &separator.to_string()
                        };
                        output.push(format!(
                            "{}{}{}",
                            line1,
                            sep_str,
                            other_fields.join(sep_str)
                        ));
                    }
                }
            }
        }

        Ok(output.join("\n"))
    }
}
