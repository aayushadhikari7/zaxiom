//! awk command - pattern scanning and processing language

use std::fs;
use anyhow::Result;
use regex::Regex;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct AwkCommand;

impl Command for AwkCommand {
    fn name(&self) -> &'static str {
        "awk"
    }

    fn description(&self) -> &'static str {
        "Pattern scanning and processing language"
    }

    fn usage(&self) -> &'static str {
        "awk [-F sep] 'pattern {action}' [file]"
    }

    fn extended_help(&self) -> String {
        r#"awk - Pattern scanning and processing language

USAGE:
  awk [OPTIONS] 'program' [file]
  command | awk [OPTIONS] 'program'

OPTIONS:
  -F <sep>    Set field separator (default: whitespace)

DESCRIPTION:
  AWK is a powerful text processing tool. It reads input
  line by line, splits each line into fields, and lets you
  perform actions based on patterns.

BASIC SYNTAX:
  awk 'pattern { action }' file
  awk '{ action }' file         # No pattern = all lines
  awk '/regex/' file            # No action = print line

FIELD VARIABLES:
  $0    Entire line
  $1    First field (column)
  $2    Second field
  $N    Nth field
  NR    Current line number
  NF    Number of fields in current line

EXAMPLES - PRINT COLUMNS:
  awk '{print $1}' file.txt           First column
  awk '{print $1, $3}' file.txt       Columns 1 and 3
  awk '{print $NF}' file.txt          Last column
  awk '{print NR, $0}' file.txt       Line numbers

EXAMPLES - WITH PATTERN:
  awk '/error/' log.txt               Lines containing "error"
  awk '/^#/' config.txt               Lines starting with #
  awk 'NR==5' file.txt                Only line 5
  awk 'NR>10' file.txt                Lines after line 10

EXAMPLES - FIELD SEPARATOR:
  awk -F: '{print $1}' /etc/passwd    Colon-separated
  awk -F, '{print $2}' data.csv       CSV second column
  awk -F'\t' '{print $1}' file.tsv    Tab-separated

COMMON USE CASES:
  • Extract specific columns from files
  • Filter lines by pattern
  • Process CSV/TSV data
  • Calculate statistics
  • Transform text data

QUICK RECIPES:
  # Sum a column
  awk '{sum += $1} END {print sum}' file

  # Count lines matching pattern
  awk '/pattern/ {count++} END {print count}' file

  # Print unique values in column 1
  awk '!seen[$1]++' file

RELATED COMMANDS:
  cut      Extract columns (simpler)
  sed      Stream editing
  grep     Pattern matching
"#.to_string()
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        self.execute_with_stdin(args, None, state)
    }

    fn execute_with_stdin(&self, args: &[String], stdin: Option<&str>, state: &mut TerminalState) -> Result<String> {
        let mut field_separator = " ".to_string();
        let mut program: Option<&String> = None;
        let mut file: Option<&String> = None;

        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "-F" => {
                    if i + 1 < args.len() {
                        field_separator = args[i + 1].clone();
                        i += 1;
                    }
                }
                "-h" | "--help" => {
                    return Ok("Usage: awk [OPTIONS] 'program' [file]\n\
                        Options:\n  \
                        -F sep    Set field separator\n\n\
                        Program Examples:\n  \
                        '{print $1}'           Print first column\n  \
                        '{print $1, $3}'       Print columns 1 and 3\n  \
                        '{print $0}'           Print entire line\n  \
                        '{print NR, $0}'       Print line number and line\n  \
                        '/pattern/'            Print lines matching pattern\n  \
                        '/pattern/ {print $1}' Print column 1 of matching lines\n\n\
                        Special variables:\n  \
                        $0    Entire line\n  \
                        $1-$n Individual fields\n  \
                        NR    Line number\n  \
                        NF    Number of fields".to_string());
                }
                _ if args[i].starts_with("-F") => {
                    field_separator = args[i][2..].to_string();
                }
                _ if !args[i].starts_with('-') => {
                    if program.is_none() {
                        program = Some(&args[i]);
                    } else {
                        file = Some(&args[i]);
                    }
                }
                _ => {}
            }
            i += 1;
        }

        let program = program.ok_or_else(|| anyhow::anyhow!("awk: missing program"))?;

        // Get content
        let content = if let Some(f) = file {
            let path = state.resolve_path(f);
            fs::read_to_string(&path)
                .map_err(|e| anyhow::anyhow!("awk: {}: {}", f, e))?
        } else if let Some(input) = stdin {
            input.to_string()
        } else {
            return Err(anyhow::anyhow!("awk: no input file"));
        };

        // Parse the awk program
        let (pattern, action) = parse_awk_program(program)?;

        // Compile pattern regex if present
        let pattern_regex = if let Some(p) = &pattern {
            Some(Regex::new(p).map_err(|e| anyhow::anyhow!("awk: invalid pattern: {}", e))?)
        } else {
            None
        };

        let mut output = Vec::new();

        for (line_num, line) in content.lines().enumerate() {
            let nr = line_num + 1;

            // Check pattern match
            if let Some(ref regex) = pattern_regex {
                if !regex.is_match(line) {
                    continue;
                }
            }

            // Split into fields
            let fields: Vec<&str> = if field_separator == " " {
                line.split_whitespace().collect()
            } else {
                line.split(&field_separator).collect()
            };

            let nf = fields.len();

            // Execute action
            if let Some(ref act) = action {
                let result = execute_awk_action(act, line, &fields, nr, nf)?;
                if !result.is_empty() {
                    output.push(result);
                }
            } else {
                // Default action: print line
                output.push(line.to_string());
            }
        }

        Ok(output.join("\n"))
    }

    fn supports_stdin(&self) -> bool {
        true
    }
}

/// Parse an awk program into pattern and action
fn parse_awk_program(program: &str) -> Result<(Option<String>, Option<String>)> {
    let program = program.trim();

    // Pattern only: /regex/
    if program.starts_with('/') && program.ends_with('/') && !program.contains('{') {
        let pattern = &program[1..program.len()-1];
        return Ok((Some(pattern.to_string()), None));
    }

    // Action only: {action}
    if program.starts_with('{') && program.ends_with('}') {
        let action = &program[1..program.len()-1];
        return Ok((None, Some(action.to_string())));
    }

    // Pattern and action: /regex/ {action}
    if program.starts_with('/') {
        if let Some(end_pattern) = program[1..].find('/') {
            let pattern = &program[1..end_pattern+1];
            let rest = program[end_pattern+2..].trim();

            if rest.starts_with('{') && rest.ends_with('}') {
                let action = &rest[1..rest.len()-1];
                return Ok((Some(pattern.to_string()), Some(action.to_string())));
            }
        }
    }

    // Assume it's just an action if nothing else matches
    Ok((None, Some(program.to_string())))
}

/// Execute an awk action and return the result
fn execute_awk_action(action: &str, line: &str, fields: &[&str], nr: usize, nf: usize) -> Result<String> {
    let action = action.trim();

    // Handle print statements
    if action.starts_with("print") {
        let args = action[5..].trim();

        if args.is_empty() {
            return Ok(line.to_string());
        }

        // Parse print arguments
        let mut output_parts = Vec::new();

        for part in args.split(',') {
            let part = part.trim();
            let value = evaluate_awk_expression(part, line, fields, nr, nf)?;
            output_parts.push(value);
        }

        return Ok(output_parts.join(" "));
    }

    // Default: return empty (no output for this line)
    Ok(String::new())
}

/// Evaluate an awk expression
fn evaluate_awk_expression(expr: &str, line: &str, fields: &[&str], nr: usize, nf: usize) -> Result<String> {
    let expr = expr.trim();

    // String literal
    if (expr.starts_with('"') && expr.ends_with('"')) ||
       (expr.starts_with('\'') && expr.ends_with('\'')) {
        return Ok(expr[1..expr.len()-1].to_string());
    }

    // Field reference $N
    if expr.starts_with('$') {
        let field_num = &expr[1..];
        if field_num == "0" {
            return Ok(line.to_string());
        }
        if let Ok(n) = field_num.parse::<usize>() {
            if n > 0 && n <= fields.len() {
                return Ok(fields[n - 1].to_string());
            }
            return Ok(String::new()); // Out of range field
        }
    }

    // Special variables
    match expr {
        "NR" => return Ok(nr.to_string()),
        "NF" => return Ok(nf.to_string()),
        _ => {}
    }

    // Return as literal
    Ok(expr.to_string())
}
