//! command command - run a command bypassing shell functions

use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct CommandCommand;

impl Command for CommandCommand {
    fn name(&self) -> &'static str {
        "command"
    }

    fn description(&self) -> &'static str {
        "Run a command, bypassing functions"
    }

    fn usage(&self) -> &'static str {
        "command [-v] [-V] command [args...]"
    }

    fn execute(&self, args: &[String], _state: &mut TerminalState) -> Result<String> {
        let mut verbose = false;
        let mut very_verbose = false;
        let mut cmd_start = 0;

        for (i, arg) in args.iter().enumerate() {
            match arg.as_str() {
                "-v" => {
                    verbose = true;
                    cmd_start = i + 1;
                }
                "-V" => {
                    very_verbose = true;
                    cmd_start = i + 1;
                }
                _ if !arg.starts_with('-') => {
                    cmd_start = i;
                    break;
                }
                _ => {}
            }
        }

        if cmd_start >= args.len() {
            return Err(anyhow::anyhow!("command: missing command argument"));
        }

        let cmd_name = &args[cmd_start];

        if verbose || very_verbose {
            // Just print where the command is
            if let Some(path) = find_in_path(cmd_name) {
                if very_verbose {
                    Ok(format!("{} is {}", cmd_name, path))
                } else {
                    Ok(path)
                }
            } else {
                Err(anyhow::anyhow!("{}: not found", cmd_name))
            }
        } else {
            // In a real shell, this would execute the command
            // For now, just indicate what would be run
            let cmd_args: Vec<&str> = args[cmd_start..].iter().map(|s| s.as_str()).collect();
            Ok(format!("Would execute: {}", cmd_args.join(" ")))
        }
    }
}

fn find_in_path(name: &str) -> Option<String> {
    let path_var = std::env::var("PATH").ok()?;
    let extensions = ["", ".exe", ".cmd", ".bat", ".com"];

    for dir in path_var.split(';') {
        for ext in &extensions {
            let full_path = std::path::Path::new(dir).join(format!("{}{}", name, ext));
            if full_path.exists() {
                return Some(full_path.display().to_string());
            }
        }
    }

    None
}
