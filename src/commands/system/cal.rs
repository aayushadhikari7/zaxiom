//! cal command - display a calendar

use anyhow::Result;
use chrono::{Datelike, Local, NaiveDate};

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct CalCommand;

impl Command for CalCommand {
    fn name(&self) -> &'static str {
        "cal"
    }

    fn description(&self) -> &'static str {
        "Display a calendar"
    }

    fn usage(&self) -> &'static str {
        "cal [[month] year]"
    }

    fn execute(&self, args: &[String], _state: &mut TerminalState) -> Result<String> {
        let today = Local::now().date_naive();

        let (year, month) = match args.len() {
            0 => (today.year(), today.month()),
            1 => {
                if args[0] == "-h" || args[0] == "--help" {
                    return Ok("Usage: cal [[month] year]\n\
                        Display a calendar for the given month/year.\n\
                        Without arguments, shows current month."
                        .to_string());
                }
                let y: i32 = args[0]
                    .parse()
                    .map_err(|_| anyhow::anyhow!("cal: invalid year"))?;
                (y, today.month())
            }
            _ => {
                let m: u32 = args[0]
                    .parse()
                    .map_err(|_| anyhow::anyhow!("cal: invalid month"))?;
                let y: i32 = args[1]
                    .parse()
                    .map_err(|_| anyhow::anyhow!("cal: invalid year"))?;
                if !(1..=12).contains(&m) {
                    return Err(anyhow::anyhow!("cal: month must be 1-12"));
                }
                (y, m)
            }
        };

        Ok(render_calendar(year, month, today))
    }
}

fn render_calendar(year: i32, month: u32, today: NaiveDate) -> String {
    let month_names = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let first_day = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let days_in_month = days_in_month(year, month);
    let start_weekday = first_day.weekday().num_days_from_sunday();

    let mut output = Vec::new();

    // Header
    let header = format!("{} {}", month_names[(month - 1) as usize], year);
    let padding = (20 - header.len()) / 2;
    output.push(format!("{:>width$}{}", "", header, width = padding));
    output.push("Su Mo Tu We Th Fr Sa".to_string());

    // Days
    let mut line = String::new();
    for _ in 0..start_weekday {
        line.push_str("   ");
    }

    for day in 1..=days_in_month {
        let is_today = year == today.year() && month == today.month() && day == today.day();

        if is_today {
            // Highlight today with ANSI inverse
            line.push_str(&format!("\x1b[7m{:>2}\x1b[0m ", day));
        } else {
            line.push_str(&format!("{:>2} ", day));
        }

        if (start_weekday + day).is_multiple_of(7) {
            output.push(line.trim_end().to_string());
            line = String::new();
        }
    }

    if !line.is_empty() {
        output.push(line.trim_end().to_string());
    }

    output.join("\n")
}

fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
                29
            } else {
                28
            }
        }
        _ => 30,
    }
}
