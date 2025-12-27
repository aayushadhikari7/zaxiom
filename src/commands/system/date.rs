//! date command - display or set the system date and time

use chrono::{Local, Utc};
use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct DateCommand;

impl Command for DateCommand {
    fn name(&self) -> &'static str {
        "date"
    }

    fn description(&self) -> &'static str {
        "Display the current date and time"
    }

    fn usage(&self) -> &'static str {
        "date [-u] [+format]"
    }

    fn extended_help(&self) -> String {
        r#"date - Display the current date and time

USAGE:
  date [OPTIONS] [+FORMAT]

OPTIONS:
  -u, --utc    Display time in UTC (not local time)

DESCRIPTION:
  Display the current date and time. Use format
  specifiers to customize the output.

EXAMPLES:
  date                      Default format
  date -u                   UTC time
  date +%Y-%m-%d            2024-01-15
  date +%H:%M:%S            14:30:45
  date "+%Y-%m-%d %H:%M"    2024-01-15 14:30
  date +%A                  Monday

FORMAT SPECIFIERS:
  %Y    Year (4 digits)       2024
  %y    Year (2 digits)       24
  %m    Month (01-12)         01
  %d    Day of month (01-31)  15
  %H    Hour 24h (00-23)      14
  %I    Hour 12h (01-12)      02
  %M    Minute (00-59)        30
  %S    Second (00-59)        45
  %p    AM/PM                 PM
  %A    Weekday name          Monday
  %a    Weekday short         Mon
  %B    Month name            January
  %b    Month short           Jan
  %Z    Timezone              EST

COMMON FORMATS:
  +%Y-%m-%d           ISO date: 2024-01-15
  +%Y%m%d_%H%M%S      Timestamp: 20240115_143045
  +%s                 Unix epoch seconds

USE CASES:
  • Timestamp for filenames
  • Log entries
  • Backup naming
  • Script automation

RELATED COMMANDS:
  cal      Calendar
  uptime   System uptime
"#.to_string()
    }

    fn execute(&self, args: &[String], _state: &mut TerminalState) -> Result<String> {
        let mut use_utc = false;
        let mut format: Option<&str> = None;

        for arg in args {
            match arg.as_str() {
                "-u" | "--utc" => use_utc = true,
                "-h" | "--help" => {
                    return Ok("Usage: date [OPTIONS] [+FORMAT]\n\
                        Options:\n  \
                        -u    Use UTC time\n\n\
                        Format specifiers:\n  \
                        %Y    Year (4 digits)\n  \
                        %m    Month (01-12)\n  \
                        %d    Day (01-31)\n  \
                        %H    Hour (00-23)\n  \
                        %M    Minute (00-59)\n  \
                        %S    Second (00-59)\n  \
                        %A    Weekday name\n  \
                        %B    Month name\n  \
                        %Z    Timezone".to_string());
                }
                _ if arg.starts_with('+') => format = Some(&arg[1..]),
                _ => {}
            }
        }

        let fmt = format.unwrap_or("%a %b %d %H:%M:%S %Z %Y");

        if use_utc {
            Ok(Utc::now().format(fmt).to_string())
        } else {
            Ok(Local::now().format(fmt).to_string())
        }
    }
}
