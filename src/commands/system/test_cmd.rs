//! test command - evaluate conditional expressions

use std::fs;
use std::path::Path;

use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct TestCommand;

impl Command for TestCommand {
    fn name(&self) -> &'static str {
        "test"
    }

    fn description(&self) -> &'static str {
        "Evaluate conditional expression"
    }

    fn usage(&self) -> &'static str {
        "test expression\n  -e file    file exists\n  -f file    file is regular file\n  -d file    file is directory\n  -r file    file is readable\n  -w file    file is writable\n  -x file    file is executable\n  -s file    file size > 0\n  -z string  string is empty\n  -n string  string is not empty\n  s1 = s2    strings equal\n  s1 != s2   strings not equal\n  n1 -eq n2  numbers equal\n  n1 -ne n2  numbers not equal\n  n1 -lt n2  n1 < n2\n  n1 -le n2  n1 <= n2\n  n1 -gt n2  n1 > n2\n  n1 -ge n2  n1 >= n2"
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        if args.is_empty() {
            return Ok("false".to_string());
        }

        let result = evaluate_expression(args, state)?;

        Ok(if result { "true" } else { "false" }.to_string())
    }
}

fn evaluate_expression(args: &[String], state: &TerminalState) -> Result<bool> {
    if args.is_empty() {
        return Ok(false);
    }

    // Handle negation
    if args[0] == "!" {
        return Ok(!evaluate_expression(&args[1..].to_vec(), state)?);
    }

    // Unary operators
    if args.len() >= 2 {
        let op = &args[0];
        let operand = &args[1];

        match op.as_str() {
            "-e" => {
                let path = state.resolve_path(operand);
                return Ok(path.exists());
            }
            "-f" => {
                let path = state.resolve_path(operand);
                return Ok(path.is_file());
            }
            "-d" => {
                let path = state.resolve_path(operand);
                return Ok(path.is_dir());
            }
            "-r" => {
                let path = state.resolve_path(operand);
                return Ok(is_readable(&path));
            }
            "-w" => {
                let path = state.resolve_path(operand);
                return Ok(is_writable(&path));
            }
            "-x" => {
                let path = state.resolve_path(operand);
                return Ok(is_executable(&path));
            }
            "-s" => {
                let path = state.resolve_path(operand);
                if let Ok(meta) = fs::metadata(&path) {
                    return Ok(meta.len() > 0);
                }
                return Ok(false);
            }
            "-z" => return Ok(operand.is_empty()),
            "-n" => return Ok(!operand.is_empty()),
            _ => {}
        }
    }

    // Binary operators
    if args.len() >= 3 {
        let left = &args[0];
        let op = &args[1];
        let right = &args[2];

        match op.as_str() {
            "=" | "==" => return Ok(left == right),
            "!=" => return Ok(left != right),
            "-eq" => {
                let l: i64 = left.parse().unwrap_or(0);
                let r: i64 = right.parse().unwrap_or(0);
                return Ok(l == r);
            }
            "-ne" => {
                let l: i64 = left.parse().unwrap_or(0);
                let r: i64 = right.parse().unwrap_or(0);
                return Ok(l != r);
            }
            "-lt" => {
                let l: i64 = left.parse().unwrap_or(0);
                let r: i64 = right.parse().unwrap_or(0);
                return Ok(l < r);
            }
            "-le" => {
                let l: i64 = left.parse().unwrap_or(0);
                let r: i64 = right.parse().unwrap_or(0);
                return Ok(l <= r);
            }
            "-gt" => {
                let l: i64 = left.parse().unwrap_or(0);
                let r: i64 = right.parse().unwrap_or(0);
                return Ok(l > r);
            }
            "-ge" => {
                let l: i64 = left.parse().unwrap_or(0);
                let r: i64 = right.parse().unwrap_or(0);
                return Ok(l >= r);
            }
            _ => {}
        }
    }

    // Single argument - true if non-empty
    if args.len() == 1 {
        return Ok(!args[0].is_empty());
    }

    Ok(false)
}

fn is_readable(path: &Path) -> bool {
    fs::File::open(path).is_ok()
}

fn is_writable(path: &Path) -> bool {
    if path.exists() {
        fs::metadata(path)
            .map(|m| !m.permissions().readonly())
            .unwrap_or(false)
    } else {
        // Check if parent is writable
        path.parent()
            .map(|p| is_writable(p))
            .unwrap_or(false)
    }
}

fn is_executable(path: &Path) -> bool {
    // On Windows, check for executable extensions
    if let Some(ext) = path.extension() {
        let ext = ext.to_string_lossy().to_lowercase();
        return matches!(ext.as_str(), "exe" | "cmd" | "bat" | "com" | "ps1");
    }
    false
}
