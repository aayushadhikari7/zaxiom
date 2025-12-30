//! cp command - copy files/directories

use std::fs;
use std::path::Path;

use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct CpCommand;

impl Command for CpCommand {
    fn name(&self) -> &'static str {
        "cp"
    }

    fn description(&self) -> &'static str {
        "Copy files or directories"
    }

    fn usage(&self) -> &'static str {
        "cp [-r] <source> <dest>"
    }

    fn extended_help(&self) -> String {
        r#"cp - Copy files and directories

USAGE:
  cp [OPTIONS] <source> <destination>
  cp [OPTIONS] <source...> <directory>

OPTIONS:
  -r, -R, --recursive    Copy directories recursively

DESCRIPTION:
  Copy SOURCE to DEST, or multiple SOURCE(s) to DIRECTORY.
  Use -r to copy directories and their contents.

EXAMPLES:
  cp file.txt backup.txt         Copy file to new name
  cp file.txt /backup/           Copy file to directory
  cp -r folder/ backup/          Copy directory recursively
  cp *.txt documents/            Copy multiple files
  cp -r src/ tests/ backup/      Copy multiple dirs to backup/

COMMON USE CASES:
  • Create backup copies of files
  • Duplicate project folders
  • Copy files to different locations
  • Batch copy with wildcards

NOTES:
  • Existing files will be overwritten
  • Use -r for directories (required)
  • Target must be a directory when copying multiple sources

RELATED COMMANDS:
  mv       Move/rename files
  rm       Remove files
  ln       Create links
  rsync    Advanced sync (not available)
"#.to_string()
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        let mut recursive = false;
        let mut paths = Vec::new();

        for arg in args {
            match arg.as_str() {
                "-r" | "-R" | "--recursive" => recursive = true,
                _ if !arg.starts_with('-') => paths.push(arg.as_str()),
                _ => {}
            }
        }

        if paths.len() < 2 {
            return Err(anyhow::anyhow!("Usage: cp [-r] <source> <dest>"));
        }

        let dest = state.resolve_path(paths.pop().unwrap());
        let sources: Vec<_> = paths.iter().map(|p| state.resolve_path(p)).collect();

        // Multiple sources -> dest must be a directory
        if sources.len() > 1
            && !dest.is_dir() {
                return Err(anyhow::anyhow!("Target must be a directory when copying multiple files"));
            }

        for source in sources {
            if !source.exists() {
                return Err(anyhow::anyhow!("No such file or directory: {}", source.display()));
            }

            let target = if dest.is_dir() {
                dest.join(source.file_name().unwrap_or_default())
            } else {
                dest.clone()
            };

            if source.is_dir() {
                if !recursive {
                    return Err(anyhow::anyhow!("Cannot copy directory (use -r): {}", source.display()));
                }
                copy_dir_all(&source, &target)?;
            } else {
                fs::copy(&source, &target)
                    .map_err(|e| anyhow::anyhow!("Cannot copy: {}", e))?;
            }
        }

        Ok(String::new())
    }
}

/// Recursively copy a directory
fn copy_dir_all(src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if file_type.is_dir() {
            copy_dir_all(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}
