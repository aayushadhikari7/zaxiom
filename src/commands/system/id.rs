//! id command - print user identity

use std::env;
use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct IdCommand;

impl Command for IdCommand {
    fn name(&self) -> &'static str {
        "id"
    }

    fn description(&self) -> &'static str {
        "Print user identity"
    }

    fn usage(&self) -> &'static str {
        "id [-u] [-n]"
    }

    fn execute(&self, args: &[String], _state: &mut TerminalState) -> Result<String> {
        let mut show_uid_only = false;
        let mut show_name = false;

        for arg in args {
            match arg.as_str() {
                "-u" => show_uid_only = true,
                "-n" => show_name = true,
                "-h" | "--help" => {
                    return Ok("Usage: id [OPTIONS]\n\
                        Options:\n  \
                        -u    Print only the user ID\n  \
                        -n    Print name instead of number".to_string());
                }
                _ => {}
            }
        }

        let username = env::var("USERNAME")
            .or_else(|_| env::var("USER"))
            .unwrap_or_else(|_| "unknown".to_string());

        let domain = env::var("USERDOMAIN").unwrap_or_else(|_| "".to_string());

        if show_uid_only {
            if show_name {
                return Ok(username);
            }
            // Windows doesn't have simple numeric UIDs, use a hash
            return Ok("1000".to_string());
        }

        // Full output
        let mut output = format!("uid=1000({}) gid=1000({})", username, username);
        if !domain.is_empty() {
            output.push_str(&format!(" domain={}", domain));
        }

        Ok(output)
    }
}
