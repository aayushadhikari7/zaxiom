//! Command trait definition
//!
//! All built-in commands implement this trait.

#![allow(dead_code)]

use anyhow::Result;

use crate::terminal::state::TerminalState;

/// Trait for built-in commands
pub trait Command: Send + Sync {
    /// Get the command name
    fn name(&self) -> &'static str;

    /// Get command description
    fn description(&self) -> &'static str;

    /// Get usage information
    fn usage(&self) -> &'static str;

    /// Get extended help with examples (override for detailed help)
    fn extended_help(&self) -> String {
        format!(
            "{} - {}\n\nUsage:\n  {}\n",
            self.name(),
            self.description(),
            self.usage()
        )
    }

    /// Execute the command
    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String>;

    /// Execute the command with stdin input (for piping)
    /// Default implementation ignores stdin and calls execute
    fn execute_with_stdin(&self, args: &[String], stdin: Option<&str>, state: &mut TerminalState) -> Result<String> {
        // Default: ignore stdin, just call execute
        let _ = stdin;
        self.execute(args, state)
    }

    /// Whether this command supports reading from stdin
    fn supports_stdin(&self) -> bool {
        false
    }
}

/// Result of command execution
#[derive(Debug)]
pub struct CommandResult {
    /// Output text
    pub output: String,
    /// Whether the command succeeded
    pub success: bool,
    /// Exit code (0 = success)
    pub exit_code: i32,
}

impl CommandResult {
    /// Create a successful result
    pub fn success(output: String) -> Self {
        Self {
            output,
            success: true,
            exit_code: 0,
        }
    }

    /// Create a failed result
    pub fn error(message: String) -> Self {
        Self {
            output: message,
            success: false,
            exit_code: 1,
        }
    }
}
