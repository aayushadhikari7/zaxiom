//! popd command - pop directory from stack

use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;
use super::pushd::DIR_STACK;

pub struct PopdCommand;

impl Command for PopdCommand {
    fn name(&self) -> &'static str {
        "popd"
    }

    fn description(&self) -> &'static str {
        "Pop directory from stack and cd to it"
    }

    fn usage(&self) -> &'static str {
        "popd"
    }

    fn execute(&self, _args: &[String], state: &mut TerminalState) -> Result<String> {
        let mut stack = DIR_STACK.lock().unwrap();

        if stack.is_empty() {
            return Err(anyhow::anyhow!("popd: directory stack empty"));
        }

        let target = stack.pop().unwrap();
        drop(stack); // Release lock before set_cwd

        state.set_cwd(target.clone());

        // Show remaining directory stack
        let stack = DIR_STACK.lock().unwrap();
        let mut dirs: Vec<String> = vec![target.display().to_string()];
        dirs.extend(stack.iter().rev().map(|p: &std::path::PathBuf| p.display().to_string()));

        Ok(dirs.join(" "))
    }
}
