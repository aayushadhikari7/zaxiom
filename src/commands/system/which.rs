//! which command - find command location

use std::env;

use anyhow::Result;

use crate::commands::registry::CommandRegistry;
use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct WhichCommand;

impl Command for WhichCommand {
    fn name(&self) -> &'static str {
        "which"
    }

    fn description(&self) -> &'static str {
        "Show command location or type"
    }

    fn usage(&self) -> &'static str {
        "which <command>"
    }

    fn execute(&self, args: &[String], _state: &mut TerminalState) -> Result<String> {
        if args.is_empty() {
            return Err(anyhow::anyhow!("Usage: which <command>"));
        }

        let command = &args[0];
        let registry = CommandRegistry::new();

        // Check if it's a built-in
        if registry.has_command(command) {
            return Ok(format!("{}: zaxiom built-in command", command));
        }

        // Check git shortcuts
        let git_shortcuts = [
            "gs", "gd", "gl", "gp", "gpl", "ga", "gc", "gco", "gb", "gba", "gcb", "grh", "gst",
            "gstp",
        ];
        if git_shortcuts.contains(&command.as_str()) {
            return Ok(format!("{}: zaxiom git shortcut", command));
        }

        // Search in PATH
        if let Ok(path_var) = env::var("PATH") {
            let paths: Vec<&str> = path_var.split(';').collect();

            for path in paths {
                let exe_path = std::path::Path::new(path).join(command);

                // Check common extensions on Windows
                for ext in &["", ".exe", ".cmd", ".bat", ".com", ".ps1"] {
                    let full_path = if ext.is_empty() {
                        exe_path.clone()
                    } else {
                        exe_path.with_extension(&ext[1..])
                    };

                    if full_path.exists() {
                        return Ok(full_path.display().to_string().replace('\\', "/"));
                    }
                }
            }
        }

        Err(anyhow::anyhow!("{} not found", command))
    }
}
