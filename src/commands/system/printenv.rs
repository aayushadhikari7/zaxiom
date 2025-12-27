//! printenv command - print environment variables

use std::env;

use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct PrintenvCommand;

impl Command for PrintenvCommand {
    fn name(&self) -> &'static str {
        "printenv"
    }

    fn description(&self) -> &'static str {
        "Print environment variables"
    }

    fn usage(&self) -> &'static str {
        "printenv [variable...]"
    }

    fn execute(&self, args: &[String], _state: &mut TerminalState) -> Result<String> {
        if args.is_empty() {
            // Print all environment variables
            let mut vars: Vec<_> = env::vars().collect();
            vars.sort_by(|a, b| a.0.cmp(&b.0));

            let output: Vec<String> = vars
                .into_iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect();

            Ok(output.join("\n"))
        } else {
            // Print specific variables
            let mut output = Vec::new();

            for var in args {
                if let Ok(value) = env::var(var) {
                    output.push(value);
                }
            }

            if output.is_empty() && args.len() == 1 {
                return Err(anyhow::anyhow!("printenv: '{}' not set", args[0]));
            }

            Ok(output.join("\n"))
        }
    }
}
