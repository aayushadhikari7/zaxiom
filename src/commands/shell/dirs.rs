//! dirs command - display directory stack

use anyhow::Result;

use super::pushd::DIR_STACK;
use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct DirsCommand;

impl Command for DirsCommand {
    fn name(&self) -> &'static str {
        "dirs"
    }

    fn description(&self) -> &'static str {
        "Display directory stack"
    }

    fn usage(&self) -> &'static str {
        "dirs [-c] [-l] [-v]"
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        let mut clear = false;
        let mut long_format = false;
        let mut vertical = false;

        for arg in args {
            match arg.as_str() {
                "-c" | "--clear" => clear = true,
                "-l" | "--long" => long_format = true,
                "-v" | "--vertical" => vertical = true,
                "-p" => vertical = true,
                _ => {}
            }
        }

        if clear {
            DIR_STACK.lock().unwrap().clear();
            return Ok(String::new());
        }

        let stack = DIR_STACK.lock().unwrap();
        let home = state.home();

        // Current directory is always at "top" of displayed stack
        let mut dirs: Vec<String> = vec![state.cwd().display().to_string()];
        dirs.extend(
            stack
                .iter()
                .rev()
                .map(|p: &std::path::PathBuf| p.display().to_string()),
        );

        // Format paths
        let formatted: Vec<String> = dirs
            .iter()
            .enumerate()
            .map(|(i, d)| {
                let display = if !long_format && d.starts_with(&home.display().to_string()) {
                    format!("~{}", &d[home.display().to_string().len()..])
                } else {
                    d.clone()
                }
                .replace('\\', "/");

                if vertical {
                    format!(" {} {}", i, display)
                } else {
                    display
                }
            })
            .collect();

        if vertical {
            Ok(formatted.join("\n"))
        } else {
            Ok(formatted.join(" "))
        }
    }
}
