//! basename command - strip directory from filenames

use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct BasenameCommand;

impl Command for BasenameCommand {
    fn name(&self) -> &'static str {
        "basename"
    }

    fn description(&self) -> &'static str {
        "Strip directory and suffix from filenames"
    }

    fn usage(&self) -> &'static str {
        "basename <path> [suffix]"
    }

    fn execute(&self, args: &[String], _state: &mut TerminalState) -> Result<String> {
        if args.is_empty() {
            return Err(anyhow::anyhow!("basename: missing operand"));
        }

        if args[0] == "-h" || args[0] == "--help" {
            return Ok("Usage: basename NAME [SUFFIX]\n\
                Print NAME with any leading directory components removed.\n\
                If SUFFIX is specified, also remove trailing SUFFIX."
                .to_string());
        }

        let path = &args[0];
        let suffix = args.get(1).map(|s| s.as_str());

        // Get the last component
        let name = path
            .trim_end_matches(['/', '\\'])
            .rsplit(['/', '\\'])
            .next()
            .unwrap_or(path);

        // Remove suffix if specified
        let result = if let Some(suf) = suffix {
            name.strip_suffix(suf).unwrap_or(name)
        } else {
            name
        };

        Ok(result.to_string())
    }
}
