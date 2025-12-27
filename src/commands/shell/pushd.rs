//! pushd command - push directory onto stack

use std::path::PathBuf;
use std::sync::Mutex;

use anyhow::Result;
use once_cell::sync::Lazy;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

// Global directory stack
pub static DIR_STACK: Lazy<Mutex<Vec<PathBuf>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub struct PushdCommand;

impl Command for PushdCommand {
    fn name(&self) -> &'static str {
        "pushd"
    }

    fn description(&self) -> &'static str {
        "Push directory onto stack and cd to it"
    }

    fn usage(&self) -> &'static str {
        "pushd [directory]"
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        let current = state.cwd().clone();

        let target = if args.is_empty() {
            // Swap top two directories
            let mut stack = DIR_STACK.lock().unwrap();
            if stack.is_empty() {
                return Err(anyhow::anyhow!("pushd: no other directory"));
            }
            let top = stack.pop().unwrap();
            stack.push(current.clone());
            top
        } else {
            let target = state.resolve_path(&args[0]);
            if !target.exists() {
                return Err(anyhow::anyhow!("pushd: {}: No such directory", args[0]));
            }
            if !target.is_dir() {
                return Err(anyhow::anyhow!("pushd: {}: Not a directory", args[0]));
            }

            // Push current directory onto stack
            DIR_STACK.lock().unwrap().push(current.clone());
            target
        };

        // Change to new directory
        state.set_cwd(target.clone());

        // Show directory stack
        let stack = DIR_STACK.lock().unwrap();
        let mut dirs: Vec<String> = vec![target.display().to_string()];
        dirs.extend(stack.iter().rev().map(|p: &PathBuf| p.display().to_string()));

        Ok(dirs.join(" "))
    }
}
