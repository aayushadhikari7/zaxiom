//! sleep command - delay for a specified time

use anyhow::Result;
use std::thread;
use std::time::Duration;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct SleepCommand;

impl Command for SleepCommand {
    fn name(&self) -> &'static str {
        "sleep"
    }

    fn description(&self) -> &'static str {
        "Delay for a specified amount of time"
    }

    fn usage(&self) -> &'static str {
        "sleep <seconds>"
    }

    fn execute(&self, args: &[String], _state: &mut TerminalState) -> Result<String> {
        if args.is_empty() {
            return Err(anyhow::anyhow!("sleep: missing operand"));
        }

        for arg in args {
            if arg == "-h" || arg == "--help" {
                return Ok("Usage: sleep <number>[suffix]\n\
                    Suffixes:\n  \
                    s    seconds (default)\n  \
                    m    minutes\n  \
                    h    hours\n  \
                    d    days"
                    .to_string());
            }
        }

        let mut total_secs: f64 = 0.0;

        for arg in args {
            let (num_str, suffix) = if arg.ends_with('s') {
                (&arg[..arg.len() - 1], 1.0)
            } else if arg.ends_with('m') {
                (&arg[..arg.len() - 1], 60.0)
            } else if arg.ends_with('h') {
                (&arg[..arg.len() - 1], 3600.0)
            } else if arg.ends_with('d') {
                (&arg[..arg.len() - 1], 86400.0)
            } else {
                (arg.as_str(), 1.0)
            };

            let num: f64 = num_str
                .parse()
                .map_err(|_| anyhow::anyhow!("sleep: invalid time interval '{}'", arg))?;

            total_secs += num * suffix;
        }

        let duration = Duration::from_secs_f64(total_secs);
        thread::sleep(duration);

        Ok(String::new())
    }
}
