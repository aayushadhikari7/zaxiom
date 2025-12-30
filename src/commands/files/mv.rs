//! mv command - move/rename files

use std::fs;

use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct MvCommand;

impl Command for MvCommand {
    fn name(&self) -> &'static str {
        "mv"
    }

    fn description(&self) -> &'static str {
        "Move or rename files"
    }

    fn usage(&self) -> &'static str {
        "mv <source> <dest>"
    }

    fn extended_help(&self) -> String {
        r#"mv - Move or rename files and directories

USAGE:
  mv <source> <destination>
  mv <source...> <directory>

DESCRIPTION:
  Move or rename SOURCE to DEST, or move multiple SOURCE(s)
  to DIRECTORY. Works for both files and directories.

EXAMPLES:
  mv old.txt new.txt             Rename a file
  mv file.txt /other/dir/        Move file to directory
  mv folder/ new_folder/         Rename a directory
  mv *.txt documents/            Move multiple files
  mv file1 file2 dir/            Move multiple to directory

COMMON USE CASES:
  • Rename files and folders
  • Organize files into directories
  • Move projects between locations
  • Batch relocate files

NOTES:
  • Overwrites existing destination files
  • Faster than cp + rm (same filesystem)
  • Works across directories

RELATED COMMANDS:
  cp       Copy files
  rm       Remove files
  rename   Batch rename (not available)
"#
        .to_string()
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        let mut paths: Vec<&str> = args
            .iter()
            .filter(|a| !a.starts_with('-'))
            .map(|s| s.as_str())
            .collect();

        if paths.len() < 2 {
            return Err(anyhow::anyhow!("Usage: mv <source> <dest>"));
        }

        let dest = state.resolve_path(paths.pop().unwrap());
        let sources: Vec<_> = paths.iter().map(|p| state.resolve_path(p)).collect();

        // Multiple sources -> dest must be a directory
        if sources.len() > 1 && !dest.is_dir() {
            return Err(anyhow::anyhow!(
                "Target must be a directory when moving multiple files"
            ));
        }

        for source in sources {
            if !source.exists() {
                return Err(anyhow::anyhow!(
                    "No such file or directory: {}",
                    source.display()
                ));
            }

            let target = if dest.is_dir() {
                dest.join(source.file_name().unwrap_or_default())
            } else {
                dest.clone()
            };

            fs::rename(&source, &target).map_err(|e| anyhow::anyhow!("Cannot move: {}", e))?;
        }

        Ok(String::new())
    }
}
