//! split command - split files into pieces

use std::fs::File;
use std::io::{BufRead, BufReader, Write};

use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct SplitCommand;

impl Command for SplitCommand {
    fn name(&self) -> &'static str {
        "split"
    }

    fn description(&self) -> &'static str {
        "Split a file into pieces"
    }

    fn usage(&self) -> &'static str {
        "split [-l lines] [-b bytes] [file [prefix]]"
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        let mut lines_per_file: Option<usize> = None;
        let mut _bytes_per_file: Option<usize> = None; // TODO: implement byte splitting
        let mut input_file: Option<&str> = None;
        let mut prefix = "x".to_string();

        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "-l" => {
                    if i + 1 < args.len() {
                        lines_per_file = args[i + 1].parse().ok();
                        i += 1;
                    }
                }
                "-b" => {
                    if i + 1 < args.len() {
                        _bytes_per_file = parse_size(&args[i + 1]);
                        i += 1;
                    }
                }
                arg if !arg.starts_with('-') => {
                    if input_file.is_none() {
                        input_file = Some(arg);
                    } else {
                        prefix = arg.to_string();
                    }
                }
                _ => {}
            }
            i += 1;
        }

        let input_file =
            input_file.ok_or_else(|| anyhow::anyhow!("split: missing file operand"))?;
        let path = state.resolve_path(input_file);
        let file = File::open(&path)?;
        let reader = BufReader::new(file);

        // Default to 1000 lines if no option specified
        let lines_per_file = lines_per_file.unwrap_or(1000);

        let mut file_num = 0;
        let mut current_lines = 0;
        let mut current_file: Option<File> = None;

        for line in reader.lines() {
            let line = line?;

            if current_file.is_none() || current_lines >= lines_per_file {
                // Start new file
                let suffix = generate_suffix(file_num);
                let output_path = state.resolve_path(&format!("{}{}", prefix, suffix));
                current_file = Some(File::create(&output_path)?);
                file_num += 1;
                current_lines = 0;
            }

            if let Some(ref mut f) = current_file {
                writeln!(f, "{}", line)?;
                current_lines += 1;
            }
        }

        Ok(format!("Split into {} files", file_num))
    }
}

fn generate_suffix(num: usize) -> String {
    // aa, ab, ac, ... az, ba, bb, ...
    let first = (b'a' + (num / 26) as u8) as char;
    let second = (b'a' + (num % 26) as u8) as char;
    format!("{}{}", first, second)
}

fn parse_size(s: &str) -> Option<usize> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }

    let (num_str, multiplier) = if s.ends_with('K') || s.ends_with('k') {
        (&s[..s.len() - 1], 1024)
    } else if s.ends_with('M') || s.ends_with('m') {
        (&s[..s.len() - 1], 1024 * 1024)
    } else if s.ends_with('G') || s.ends_with('g') {
        (&s[..s.len() - 1], 1024 * 1024 * 1024)
    } else {
        (s, 1)
    };

    num_str.parse::<usize>().ok().map(|n| n * multiplier)
}
