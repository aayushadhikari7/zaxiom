//! comm command - compare two sorted files line by line

use std::fs;

use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct CommCommand;

impl Command for CommCommand {
    fn name(&self) -> &'static str {
        "comm"
    }

    fn description(&self) -> &'static str {
        "Compare two sorted files line by line"
    }

    fn usage(&self) -> &'static str {
        "comm [-1] [-2] [-3] file1 file2"
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        let mut suppress1 = false;
        let mut suppress2 = false;
        let mut suppress3 = false;
        let mut files: Vec<&str> = Vec::new();

        for arg in args {
            match arg.as_str() {
                "-1" => suppress1 = true,
                "-2" => suppress2 = true,
                "-3" => suppress3 = true,
                "-12" => { suppress1 = true; suppress2 = true; }
                "-23" => { suppress2 = true; suppress3 = true; }
                "-13" => { suppress1 = true; suppress3 = true; }
                "-123" => { suppress1 = true; suppress2 = true; suppress3 = true; }
                arg if !arg.starts_with('-') => files.push(arg),
                _ => {}
            }
        }

        if files.len() < 2 {
            return Err(anyhow::anyhow!("comm: requires two files"));
        }

        let path1 = state.resolve_path(files[0]);
        let path2 = state.resolve_path(files[1]);

        let content1 = fs::read_to_string(&path1)?;
        let content2 = fs::read_to_string(&path2)?;

        let lines1: Vec<&str> = content1.lines().collect();
        let lines2: Vec<&str> = content2.lines().collect();

        let mut i = 0;
        let mut j = 0;
        let mut output = Vec::new();

        while i < lines1.len() || j < lines2.len() {
            if i >= lines1.len() {
                // Only lines from file2 left
                if !suppress2 {
                    let prefix = if suppress1 { "" } else { "\t" };
                    output.push(format!("{}{}", prefix, lines2[j]));
                }
                j += 1;
            } else if j >= lines2.len() {
                // Only lines from file1 left
                if !suppress1 {
                    output.push(lines1[i].to_string());
                }
                i += 1;
            } else {
                match lines1[i].cmp(lines2[j]) {
                    std::cmp::Ordering::Less => {
                        // Line only in file1
                        if !suppress1 {
                            output.push(lines1[i].to_string());
                        }
                        i += 1;
                    }
                    std::cmp::Ordering::Greater => {
                        // Line only in file2
                        if !suppress2 {
                            let prefix = if suppress1 { "" } else { "\t" };
                            output.push(format!("{}{}", prefix, lines2[j]));
                        }
                        j += 1;
                    }
                    std::cmp::Ordering::Equal => {
                        // Line in both files
                        if !suppress3 {
                            let prefix = match (suppress1, suppress2) {
                                (true, true) => "",
                                (true, false) => "\t",
                                (false, true) => "\t",
                                (false, false) => "\t\t",
                            };
                            output.push(format!("{}{}", prefix, lines1[i]));
                        }
                        i += 1;
                        j += 1;
                    }
                }
            }
        }

        Ok(output.join("\n"))
    }
}
