//! rm command - remove files/directories

use std::fs;

use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct RmCommand;

impl Command for RmCommand {
    fn name(&self) -> &'static str {
        "rm"
    }

    fn description(&self) -> &'static str {
        "Remove files or directories"
    }

    fn usage(&self) -> &'static str {
        "rm [-r] [-f] <file> [file2...]"
    }

    fn extended_help(&self) -> String {
        r#"rm - Remove files or directories

USAGE:
  rm [OPTIONS] <file...>

OPTIONS:
  -r, -R, --recursive    Remove directories and contents
  -f, --force            Ignore nonexistent files, no errors
  -rf, -fr               Combine recursive and force

DESCRIPTION:
  Remove (unlink) files. Use -r for directories.
  ⚠️  WARNING: Deleted files cannot be recovered!

EXAMPLES:
  rm file.txt                 Remove a single file
  rm *.log                    Remove all .log files
  rm -r folder/               Remove directory recursively
  rm -f missing.txt           No error if doesn't exist
  rm -rf node_modules/        Force remove directory

COMMON USE CASES:
  • Delete temporary files
  • Clean up build artifacts
  • Remove old log files
  • Clear cache directories

⚠️  DANGER ZONE:
  rm -rf /                    NEVER DO THIS!
  rm -rf *                    Be very careful!

  Always double-check paths before using -rf

NOTES:
  • Directories require -r flag
  • -f suppresses errors for missing files
  • No recycle bin - files are gone!

RELATED COMMANDS:
  rmdir    Remove empty directories
  mv       Move to trash (manual)
  find     Find and delete files
"#.to_string()
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        let mut recursive = false;
        let mut force = false;
        let mut files = Vec::new();

        for arg in args {
            match arg.as_str() {
                "-r" | "-R" | "--recursive" => recursive = true,
                "-f" | "--force" => force = true,
                "-rf" | "-fr" => {
                    recursive = true;
                    force = true;
                }
                _ if !arg.starts_with('-') => files.push(arg.as_str()),
                _ => {} // Ignore unknown flags
            }
        }

        if files.is_empty() {
            return Err(anyhow::anyhow!("Usage: rm [-r] [-f] <file>"));
        }

        for file in files {
            let path = state.resolve_path(file);

            if !path.exists() {
                if force {
                    continue;
                }
                return Err(anyhow::anyhow!("No such file or directory: {}", path.display()));
            }

            if path.is_dir() {
                if !recursive {
                    return Err(anyhow::anyhow!("Cannot remove directory (use -r): {}", path.display()));
                }
                fs::remove_dir_all(&path)
                    .map_err(|e| anyhow::anyhow!("Cannot remove {}: {}", path.display(), e))?;
            } else {
                fs::remove_file(&path)
                    .map_err(|e| anyhow::anyhow!("Cannot remove {}: {}", path.display(), e))?;
            }
        }

        Ok(String::new())
    }
}
