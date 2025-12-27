//! sha224sum command - compute SHA224 message digest

use std::fs;
use anyhow::Result;
use sha2::{Sha224, Digest};

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct Sha224sumCommand;

impl Command for Sha224sumCommand {
    fn name(&self) -> &'static str {
        "sha224sum"
    }

    fn description(&self) -> &'static str {
        "Compute SHA224 message digest"
    }

    fn usage(&self) -> &'static str {
        "sha224sum <file> [file2...]"
    }

    fn supports_stdin(&self) -> bool {
        true
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        if args.is_empty() {
            return Err(anyhow::anyhow!("sha224sum: missing file operand"));
        }

        let mut output = Vec::new();

        for arg in args {
            if arg == "-h" || arg == "--help" {
                return Ok("Usage: sha224sum <file> [file2...]\n\
                    Compute SHA224 message digest for files.".to_string());
            }

            if arg.starts_with('-') {
                continue;
            }

            let path = state.resolve_path(arg);
            let content = fs::read(&path)
                .map_err(|e| anyhow::anyhow!("sha224sum: {}: {}", arg, e))?;

            let mut hasher = Sha224::new();
            hasher.update(&content);
            let result = hasher.finalize();

            output.push(format!("{}  {}", hex::encode(result), arg));
        }

        Ok(output.join("\n"))
    }

    fn execute_with_stdin(&self, args: &[String], stdin: Option<&str>, state: &mut TerminalState) -> Result<String> {
        if let Some(input) = stdin {
            let mut hasher = Sha224::new();
            hasher.update(input.as_bytes());
            let result = hasher.finalize();
            Ok(format!("{}  -", hex::encode(result)))
        } else {
            self.execute(args, state)
        }
    }
}
