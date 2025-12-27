//! mkdir command - create directories

use std::fs;

use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct MkdirCommand;

impl Command for MkdirCommand {
    fn name(&self) -> &'static str {
        "mkdir"
    }

    fn description(&self) -> &'static str {
        "Create directories"
    }

    fn usage(&self) -> &'static str {
        "mkdir [-p] <dir> [dir2...]"
    }

    fn extended_help(&self) -> String {
        r#"mkdir - Create directories

USAGE:
  mkdir [OPTIONS] <directory...>

OPTIONS:
  -p, --parents    Create parent directories as needed
                   No error if directory exists

DESCRIPTION:
  Create the DIRECTORY(ies), if they do not already exist.
  Use -p to create nested directory structures.

EXAMPLES:
  mkdir projects                Create single directory
  mkdir dir1 dir2 dir3          Create multiple directories
  mkdir -p a/b/c/d              Create nested structure
  mkdir -p src/{lib,bin,test}   Create multiple subdirs

COMMON USE CASES:
  • Set up project structures
  • Create organized folder hierarchies
  • Prepare directories for file operations
  • Initialize workspace layouts

COMMON PATTERNS:
  mkdir -p project/{src,tests,docs}
  mkdir -p ~/.config/myapp
  mkdir -p build/release

NOTES:
  • Without -p, parent must exist
  • With -p, no error if dir exists
  • Creates with default permissions

RELATED COMMANDS:
  rmdir    Remove empty directories
  rm -r    Remove directories with contents
  ls       List directories
  tree     Show directory tree
"#.to_string()
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        let mut parents = false;
        let mut dirs = Vec::new();

        for arg in args {
            match arg.as_str() {
                "-p" | "--parents" => parents = true,
                _ if !arg.starts_with('-') => dirs.push(arg.as_str()),
                _ => {}
            }
        }

        if dirs.is_empty() {
            return Err(anyhow::anyhow!("Usage: mkdir [-p] <dir>"));
        }

        for dir in dirs {
            let path = state.resolve_path(dir);

            if parents {
                fs::create_dir_all(&path)
                    .map_err(|e| anyhow::anyhow!("Cannot create {}: {}", path.display(), e))?;
            } else {
                fs::create_dir(&path)
                    .map_err(|e| anyhow::anyhow!("Cannot create {}: {}", path.display(), e))?;
            }
        }

        Ok(String::new())
    }
}
