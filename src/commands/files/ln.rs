//! ln command - create links

use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct LnCommand;

impl Command for LnCommand {
    fn name(&self) -> &'static str {
        "ln"
    }

    fn description(&self) -> &'static str {
        "Create hard or symbolic links"
    }

    fn usage(&self) -> &'static str {
        "ln [-s] <target> <link_name>"
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        let mut symbolic = false;
        let mut paths: Vec<&String> = Vec::new();

        for arg in args {
            match arg.as_str() {
                "-s" | "--symbolic" => symbolic = true,
                "-h" | "--help" => {
                    return Ok("Usage: ln [-s] <target> <link_name>\n\
                        Options:\n  \
                        -s    Create symbolic link".to_string());
                }
                _ if !arg.starts_with('-') => paths.push(arg),
                _ => {}
            }
        }

        if paths.len() < 2 {
            return Err(anyhow::anyhow!("ln: missing operands\nUsage: ln [-s] <target> <link_name>"));
        }

        let target = state.resolve_path(paths[0]);
        let link_name = state.resolve_path(paths[1]);

        if symbolic {
            #[cfg(windows)]
            {
                if target.is_dir() {
                    std::os::windows::fs::symlink_dir(&target, &link_name)?;
                } else {
                    std::os::windows::fs::symlink_file(&target, &link_name)?;
                }
            }
            #[cfg(not(windows))]
            {
                std::os::unix::fs::symlink(&target, &link_name)?;
            }
            Ok(format!("Created symbolic link: {} -> {}", link_name.display(), target.display()))
        } else {
            std::fs::hard_link(&target, &link_name)?;
            Ok(format!("Created hard link: {} -> {}", link_name.display(), target.display()))
        }
    }
}
