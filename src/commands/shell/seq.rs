//! seq command - print a sequence of numbers

use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct SeqCommand;

impl Command for SeqCommand {
    fn name(&self) -> &'static str {
        "seq"
    }

    fn description(&self) -> &'static str {
        "Print a sequence of numbers"
    }

    fn usage(&self) -> &'static str {
        "seq [-s separator] [-w] [first [incr]] last"
    }

    fn execute(&self, args: &[String], _state: &mut TerminalState) -> Result<String> {
        let mut separator = "\n".to_string();
        let mut equal_width = false;
        let mut numbers: Vec<f64> = Vec::new();

        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "-s" | "--separator" => {
                    if i + 1 < args.len() {
                        separator = args[i + 1].clone();
                        i += 1;
                    }
                }
                "-w" | "--equal-width" => equal_width = true,
                "-h" | "--help" => {
                    return Ok("Usage: seq [OPTIONS] [first [incr]] last\n\
                        Options:\n  \
                        -s <sep>    Use separator instead of newline\n  \
                        -w          Equalize width by padding with zeros\n\n\
                        Examples:\n  \
                        seq 5           → 1 2 3 4 5\n  \
                        seq 2 5         → 2 3 4 5\n  \
                        seq 1 2 10      → 1 3 5 7 9"
                        .to_string());
                }
                _ if !args[i].starts_with('-') => {
                    if let Ok(n) = args[i].parse::<f64>() {
                        numbers.push(n);
                    }
                }
                _ => {}
            }
            i += 1;
        }

        if numbers.is_empty() {
            return Err(anyhow::anyhow!("seq: missing operand"));
        }

        let (first, increment, last) = match numbers.len() {
            1 => (1.0, 1.0, numbers[0]),
            2 => (numbers[0], 1.0, numbers[1]),
            _ => (numbers[0], numbers[1], numbers[2]),
        };

        // Generate sequence
        let mut sequence: Vec<f64> = Vec::new();
        let mut current = first;

        if increment > 0.0 {
            while current <= last + f64::EPSILON {
                sequence.push(current);
                current += increment;
            }
        } else if increment < 0.0 {
            while current >= last - f64::EPSILON {
                sequence.push(current);
                current += increment;
            }
        }

        if sequence.is_empty() {
            return Ok(String::new());
        }

        // Format numbers
        let output: Vec<String> = if equal_width {
            // Find max width
            let max_width = sequence
                .iter()
                .map(|n| format_number(*n).len())
                .max()
                .unwrap_or(1);

            sequence
                .iter()
                .map(|n| {
                    let s = format_number(*n);
                    format!("{:0>width$}", s, width = max_width)
                })
                .collect()
        } else {
            sequence.iter().map(|n| format_number(*n)).collect()
        };

        Ok(output.join(&separator))
    }
}

fn format_number(n: f64) -> String {
    if n.fract() == 0.0 {
        format!("{}", n as i64)
    } else {
        format!("{}", n)
    }
}
