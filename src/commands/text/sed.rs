//! sed command - stream editor for text transformation

use std::fs;
use anyhow::Result;
use regex::Regex;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct SedCommand;

impl Command for SedCommand {
    fn name(&self) -> &'static str {
        "sed"
    }

    fn description(&self) -> &'static str {
        "Stream editor for filtering and transforming text"
    }

    fn usage(&self) -> &'static str {
        "sed [-i] 's/pattern/replacement/[flags]' [file]"
    }

    fn extended_help(&self) -> String {
        r#"sed - Stream editor for text transformation

USAGE:
  sed [OPTIONS] 's/pattern/replacement/[flags]' [file]
  command | sed 's/pattern/replacement/'

OPTIONS:
  -i    Edit file in-place (modify the file)

DESCRIPTION:
  Transform text using pattern substitution.
  Uses the s/old/new/ syntax for find and replace.

SYNTAX:
  s/pattern/replacement/flags

  Flags:
    g    Replace ALL occurrences (global)
    i    Case-insensitive matching
    (none) Replace first occurrence only

EXAMPLES:
  sed 's/foo/bar/' file.txt        Replace first "foo" with "bar"
  sed 's/foo/bar/g' file.txt       Replace ALL "foo" with "bar"
  sed 's/old/new/gi' file.txt      Global, case-insensitive
  sed -i 's/error/fixed/' log.txt  Edit file in place
  echo "hello" | sed 's/l/L/g'     Output: heLLo

REGEX IN PATTERNS:
  sed 's/[0-9]+/NUM/g'             Replace numbers
  sed 's/^/prefix: /'              Add prefix to lines
  sed 's/$/ suffix/'               Add suffix to lines
  sed 's/  */ /g'                  Collapse multiple spaces

COMMON USE CASES:
  • Find and replace in files
  • Clean up text data
  • Transform log files
  • Batch text editing

RELATED COMMANDS:
  awk      Pattern scanning
  tr       Character translation
  grep     Pattern search
"#.to_string()
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        self.execute_with_stdin(args, None, state)
    }

    fn execute_with_stdin(&self, args: &[String], stdin: Option<&str>, state: &mut TerminalState) -> Result<String> {
        let mut in_place = false;
        let mut expression: Option<&String> = None;
        let mut file: Option<&String> = None;

        for arg in args {
            match arg.as_str() {
                "-i" | "--in-place" => in_place = true,
                "-h" | "--help" => {
                    return Ok("Usage: sed [OPTIONS] 's/pattern/replacement/[flags]' [file]\n\
                        Options:\n  \
                        -i    Edit file in place\n\n\
                        Flags:\n  \
                        g     Global (replace all occurrences)\n  \
                        i     Case insensitive\n\n\
                        Examples:\n  \
                        sed 's/foo/bar/' file.txt\n  \
                        sed 's/foo/bar/g' file.txt\n  \
                        sed 's/foo/bar/gi' file.txt".to_string());
                }
                _ if !arg.starts_with('-') => {
                    if expression.is_none() {
                        expression = Some(arg);
                    } else {
                        file = Some(arg);
                    }
                }
                _ => {}
            }
        }

        let expression = expression.ok_or_else(|| anyhow::anyhow!("sed: missing expression"))?;

        // Parse the s/pattern/replacement/flags expression
        let (pattern, replacement, global, case_insensitive) = parse_sed_expression(expression)?;

        // Build regex
        let regex_pattern = if case_insensitive {
            format!("(?i){}", pattern)
        } else {
            pattern.to_string()
        };

        let regex = Regex::new(&regex_pattern)
            .map_err(|e| anyhow::anyhow!("sed: invalid pattern: {}", e))?;

        // Get content
        let content = if let Some(f) = file {
            let path = state.resolve_path(f);
            fs::read_to_string(&path)
                .map_err(|e| anyhow::anyhow!("sed: {}: {}", f, e))?
        } else if let Some(input) = stdin {
            input.to_string()
        } else {
            return Err(anyhow::anyhow!("sed: no input file"));
        };

        // Perform replacement
        let result: String = content
            .lines()
            .map(|line| {
                if global {
                    regex.replace_all(line, replacement.as_str()).to_string()
                } else {
                    regex.replace(line, replacement.as_str()).to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("\n");

        // Handle in-place editing
        if in_place {
            if let Some(f) = file {
                let path = state.resolve_path(f);
                fs::write(&path, &result)
                    .map_err(|e| anyhow::anyhow!("sed: {}: {}", f, e))?;
                return Ok(String::new());
            }
        }

        Ok(result)
    }

    fn supports_stdin(&self) -> bool {
        true
    }
}

/// Parse a sed s/pattern/replacement/flags expression
fn parse_sed_expression(expr: &str) -> Result<(String, String, bool, bool)> {
    // Must start with s
    if !expr.starts_with('s') {
        return Err(anyhow::anyhow!("sed: only 's' command is supported"));
    }

    let rest = &expr[1..];
    if rest.is_empty() {
        return Err(anyhow::anyhow!("sed: invalid expression"));
    }

    // Get delimiter (first character after 's')
    let delimiter = rest.chars().next().unwrap();
    let parts: Vec<&str> = rest[1..].split(delimiter).collect();

    if parts.len() < 2 {
        return Err(anyhow::anyhow!("sed: invalid substitution expression"));
    }

    let pattern = parts[0].to_string();
    let replacement = parts[1].to_string();

    // Parse flags
    let flags = if parts.len() > 2 { parts[2] } else { "" };
    let global = flags.contains('g');
    let case_insensitive = flags.contains('i') || flags.contains('I');

    Ok((pattern, replacement, global, case_insensitive))
}
