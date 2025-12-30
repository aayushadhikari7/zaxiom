//! touch command - create empty file or update timestamp

use std::fs::{File, OpenOptions};

use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct TouchCommand;

impl Command for TouchCommand {
    fn name(&self) -> &'static str {
        "touch"
    }

    fn description(&self) -> &'static str {
        "Create empty file or update timestamp"
    }

    fn usage(&self) -> &'static str {
        "touch <file> [file2...]"
    }

    fn extended_help(&self) -> String {
        r#"touch - Create empty files or update timestamps

USAGE:
  touch <file...>

DESCRIPTION:
  Update the access and modification times of each FILE
  to the current time. Create empty files if they don't exist.

EXAMPLES:
  touch newfile.txt             Create empty file
  touch file1 file2 file3       Create multiple files
  touch existing.txt            Update timestamp
  touch src/*.rs                Update all Rust files

COMMON USE CASES:
  • Create empty placeholder files
  • Update file modification times
  • Create marker/lock files
  • Trigger file watchers
  • Initialize configuration files

COMMON PATTERNS:
  touch .gitkeep                Keep empty dir in git
  touch .env                    Create env file
  touch build.lock              Create lock file

NOTES:
  • Creates file if it doesn't exist
  • Only updates timestamp if file exists
  • Doesn't modify file contents

RELATED COMMANDS:
  mkdir    Create directories
  cat      Create file with content
  stat     View file timestamps
  ls -l    List with timestamps
"#
        .to_string()
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        if args.is_empty() {
            return Err(anyhow::anyhow!("Usage: touch <file>"));
        }

        for arg in args {
            let path = state.resolve_path(arg);

            if path.exists() {
                // Update modification time
                let file = OpenOptions::new()
                    .write(true)
                    .open(&path)
                    .map_err(|e| anyhow::anyhow!("Cannot touch {}: {}", path.display(), e))?;
                drop(file);
            } else {
                // Create new empty file
                File::create(&path)
                    .map_err(|e| anyhow::anyhow!("Cannot create {}: {}", path.display(), e))?;
            }
        }

        Ok(String::new())
    }
}
