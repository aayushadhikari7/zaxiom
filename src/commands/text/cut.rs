//! cut command - remove sections from each line

use anyhow::Result;
use std::fs;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct CutCommand;

impl Command for CutCommand {
    fn name(&self) -> &'static str {
        "cut"
    }

    fn description(&self) -> &'static str {
        "Remove sections from each line"
    }

    fn usage(&self) -> &'static str {
        "cut -d <delim> -f <fields> <file>"
    }

    fn extended_help(&self) -> String {
        r#"cut - Remove sections from each line of files

USAGE:
  cut [OPTIONS] <file>
  command | cut [OPTIONS]

OPTIONS:
  -d <char>     Use <char> as field delimiter (default: TAB)
  -f <list>     Select only these fields (columns)
  -c <list>     Select only these character positions

DESCRIPTION:
  Extract sections from each line of a file. Perfect for
  working with delimited data like CSV or TSV files.

EXAMPLES - FIELD EXTRACTION:
  cut -f1 file.tsv                First column (tab-delimited)
  cut -f1,3 file.tsv              Columns 1 and 3
  cut -f1-3 file.tsv              Columns 1 through 3
  cut -f2- file.tsv               Column 2 to end

EXAMPLES - WITH DELIMITER:
  cut -d: -f1 /etc/passwd         First field, colon-separated
  cut -d, -f2 data.csv            Second column of CSV
  cut -d' ' -f1 file.txt          First word (space-delimited)
  cut -d'|' -f1,3 data.txt        Pipe-delimited fields 1,3

EXAMPLES - CHARACTER POSITIONS:
  cut -c1-10 file.txt             First 10 characters
  cut -c5 file.txt                Only character 5
  cut -c1,5,10 file.txt           Characters 1, 5, and 10

FIELD LIST SYNTAX:
  N       Single field N
  N-      Field N to end of line
  N-M     Fields N through M
  -M      Fields 1 through M
  N,M,O   Fields N, M, and O

COMMON USE CASES:
  • Extract columns from CSV/TSV files
  • Parse colon-separated files (/etc/passwd)
  • Get first/last N characters
  • Extract data from logs

QUICK RECIPES:
  # Get usernames from /etc/passwd
  cut -d: -f1 /etc/passwd

  # Extract first word of each line
  cut -d' ' -f1 file.txt

  # Get file extensions
  ls | cut -d. -f2

CUT vs AWK:
  cut is simpler and faster for basic extraction
  awk is more powerful for complex processing

RELATED COMMANDS:
  awk      More powerful field processing
  paste    Merge lines of files
  tr       Character translation
"#
        .to_string()
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        let mut delimiter = '\t';
        let mut fields: Option<String> = None;
        let mut chars: Option<String> = None;
        let mut file: Option<&String> = None;

        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "-d" | "--delimiter" => {
                    if i + 1 < args.len() {
                        delimiter = args[i + 1].chars().next().unwrap_or('\t');
                        i += 1;
                    }
                }
                "-f" | "--fields" => {
                    if i + 1 < args.len() {
                        fields = Some(args[i + 1].clone());
                        i += 1;
                    }
                }
                "-c" | "--characters" => {
                    if i + 1 < args.len() {
                        chars = Some(args[i + 1].clone());
                        i += 1;
                    }
                }
                "-h" | "--help" => {
                    return Ok("Usage: cut [OPTIONS] <file>\n\
                        Options:\n  \
                        -d <char>    Use <char> as delimiter\n  \
                        -f <list>    Select only these fields\n  \
                        -c <list>    Select only these characters"
                        .to_string());
                }
                _ if !args[i].starts_with('-') => file = Some(&args[i]),
                _ => {}
            }
            i += 1;
        }

        let file = file.ok_or_else(|| anyhow::anyhow!("cut: missing file operand"))?;
        let path = state.resolve_path(file);
        let content =
            fs::read_to_string(&path).map_err(|e| anyhow::anyhow!("cut: {}: {}", file, e))?;

        let mut result: Vec<String> = Vec::new();

        for line in content.lines() {
            if let Some(ref field_spec) = fields {
                let field_indices = parse_range(field_spec)?;
                let parts: Vec<&str> = line.split(delimiter).collect();
                let selected: Vec<&str> = field_indices
                    .iter()
                    .filter_map(|&idx| parts.get(idx.saturating_sub(1)))
                    .copied()
                    .collect();
                result.push(selected.join(&delimiter.to_string()));
            } else if let Some(ref char_spec) = chars {
                let char_indices = parse_range(char_spec)?;
                let line_chars: Vec<char> = line.chars().collect();
                let selected: String = char_indices
                    .iter()
                    .filter_map(|&idx| line_chars.get(idx.saturating_sub(1)))
                    .collect();
                result.push(selected);
            } else {
                result.push(line.to_string());
            }
        }

        Ok(result.join("\n"))
    }
}

fn parse_range(spec: &str) -> Result<Vec<usize>> {
    let mut indices = Vec::new();

    for part in spec.split(',') {
        if part.contains('-') {
            let range_parts: Vec<&str> = part.split('-').collect();
            if range_parts.len() == 2 {
                let start: usize = range_parts[0].parse().unwrap_or(1);
                let end: usize = range_parts[1].parse().unwrap_or(1000);
                indices.extend(start..=end);
            }
        } else if let Ok(n) = part.parse::<usize>() {
            indices.push(n);
        }
    }

    Ok(indices)
}
