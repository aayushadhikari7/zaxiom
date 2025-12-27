//! tr command - translate or delete characters

use std::fs;
use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct TrCommand;

impl Command for TrCommand {
    fn name(&self) -> &'static str {
        "tr"
    }

    fn description(&self) -> &'static str {
        "Translate or delete characters"
    }

    fn usage(&self) -> &'static str {
        "tr [-d] <set1> [set2] <file>"
    }

    fn extended_help(&self) -> String {
        r#"tr - Translate or delete characters

USAGE:
  tr [OPTIONS] <set1> <set2> <file>
  tr -d <set1> <file>
  command | tr <set1> <set2>

OPTIONS:
  -d, --delete           Delete characters in SET1
  -s, --squeeze-repeats  Squeeze repeated output characters

DESCRIPTION:
  Translate, squeeze, or delete characters. Each character
  in SET1 is replaced by the corresponding character in SET2.

EXAMPLES - TRANSLATION:
  tr 'a-z' 'A-Z' file.txt         Lowercase to uppercase
  tr 'A-Z' 'a-z' file.txt         Uppercase to lowercase
  tr ' ' '_' file.txt             Spaces to underscores
  tr '\t' ' ' file.txt            Tabs to spaces
  tr '0-9' 'X' file.txt           Replace digits with X

EXAMPLES - DELETE:
  tr -d ' ' file.txt              Remove all spaces
  tr -d '\n' file.txt             Remove newlines
  tr -d '0-9' file.txt            Remove all digits
  tr -d 'aeiou' file.txt          Remove vowels

EXAMPLES - SQUEEZE:
  tr -s ' ' ' ' file.txt          Collapse multiple spaces

CHARACTER SETS:
  a-z       Lowercase letters
  A-Z       Uppercase letters
  0-9       Digits
  [:alpha:] All letters
  [:digit:] All digits
  [:space:] Whitespace
  \n        Newline
  \t        Tab

COMMON USE CASES:
  • Change file case
  • Clean up whitespace
  • Remove unwanted characters
  • Convert delimiters
  • Normalize text

QUICK RECIPES:
  # Windows to Unix line endings
  tr -d '\r' file.txt

  # Make filename-safe (remove special chars)
  echo "My File!" | tr ' !' '_-'

  # ROT13 encoding
  tr 'A-Za-z' 'N-ZA-Mn-za-m' file.txt

RELATED COMMANDS:
  sed      Stream editing (more powerful)
  awk      Text processing
  cut      Extract columns
"#.to_string()
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        let mut delete = false;
        let mut squeeze = false;
        let mut positional: Vec<&String> = Vec::new();

        for arg in args {
            match arg.as_str() {
                "-d" | "--delete" => delete = true,
                "-s" | "--squeeze-repeats" => squeeze = true,
                "-h" | "--help" => {
                    return Ok("Usage: tr [OPTIONS] <set1> [set2] <file>\n\
                        Options:\n  \
                        -d    Delete characters in set1\n  \
                        -s    Squeeze repeated characters".to_string());
                }
                _ if !arg.starts_with('-') => positional.push(arg),
                _ => {}
            }
        }

        if delete {
            // tr -d <set> <file>
            if positional.len() < 2 {
                return Err(anyhow::anyhow!("tr: missing operand"));
            }
            let set1 = positional[0];
            let file = positional[1];
            let path = state.resolve_path(file);
            let content = fs::read_to_string(&path)
                .map_err(|e| anyhow::anyhow!("tr: {}: {}", file, e))?;

            let result: String = content
                .chars()
                .filter(|c| !set1.contains(*c))
                .collect();

            Ok(result)
        } else {
            // tr <set1> <set2> <file>
            if positional.len() < 3 {
                return Err(anyhow::anyhow!("tr: missing operand"));
            }
            let set1 = positional[0];
            let set2 = positional[1];
            let file = positional[2];
            let path = state.resolve_path(file);
            let content = fs::read_to_string(&path)
                .map_err(|e| anyhow::anyhow!("tr: {}: {}", file, e))?;

            let chars1: Vec<char> = expand_set(set1);
            let chars2: Vec<char> = expand_set(set2);

            let mut result: String = content
                .chars()
                .map(|c| {
                    if let Some(pos) = chars1.iter().position(|&ch| ch == c) {
                        chars2.get(pos).copied().unwrap_or(*chars2.last().unwrap_or(&c))
                    } else {
                        c
                    }
                })
                .collect();

            if squeeze {
                let mut squeezed = String::new();
                let mut last_char: Option<char> = None;
                for c in result.chars() {
                    if chars2.contains(&c) {
                        if Some(c) != last_char {
                            squeezed.push(c);
                        }
                    } else {
                        squeezed.push(c);
                    }
                    last_char = Some(c);
                }
                result = squeezed;
            }

            Ok(result)
        }
    }
}

fn expand_set(set: &str) -> Vec<char> {
    let mut result = Vec::new();
    let chars: Vec<char> = set.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if i + 2 < chars.len() && chars[i + 1] == '-' {
            // Range like a-z
            let start = chars[i];
            let end = chars[i + 2];
            for c in start..=end {
                result.push(c);
            }
            i += 3;
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }

    result
}
