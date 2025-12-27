//! printf command - format and print data

use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct PrintfCommand;

impl Command for PrintfCommand {
    fn name(&self) -> &'static str {
        "printf"
    }

    fn description(&self) -> &'static str {
        "Format and print data"
    }

    fn usage(&self) -> &'static str {
        "printf <format> [arguments...]"
    }

    fn execute(&self, args: &[String], _state: &mut TerminalState) -> Result<String> {
        if args.is_empty() {
            return Ok(String::new());
        }

        if args[0] == "-h" || args[0] == "--help" {
            return Ok("Usage: printf FORMAT [ARGUMENT...]\n\
                Format specifiers:\n  \
                %s    String\n  \
                %d    Integer\n  \
                %f    Float\n  \
                %x    Hex\n  \
                %o    Octal\n  \
                %%    Literal %\n\n\
                Escape sequences:\n  \
                \\n   Newline\n  \
                \\t   Tab\n  \
                \\\\   Backslash".to_string());
        }

        let format = &args[0];
        let arguments: Vec<&str> = args[1..].iter().map(|s| s.as_str()).collect();
        let mut arg_index = 0;

        let mut result = String::new();
        let mut chars = format.chars().peekable();

        while let Some(c) = chars.next() {
            if c == '\\' {
                // Escape sequence
                match chars.next() {
                    Some('n') => result.push('\n'),
                    Some('t') => result.push('\t'),
                    Some('r') => result.push('\r'),
                    Some('\\') => result.push('\\'),
                    Some('0') => result.push('\0'),
                    Some(other) => {
                        result.push('\\');
                        result.push(other);
                    }
                    None => result.push('\\'),
                }
            } else if c == '%' {
                // Format specifier
                match chars.next() {
                    Some('%') => result.push('%'),
                    Some('s') => {
                        if arg_index < arguments.len() {
                            result.push_str(arguments[arg_index]);
                            arg_index += 1;
                        }
                    }
                    Some('d') | Some('i') => {
                        if arg_index < arguments.len() {
                            if let Ok(n) = arguments[arg_index].parse::<i64>() {
                                result.push_str(&n.to_string());
                            } else {
                                result.push_str(arguments[arg_index]);
                            }
                            arg_index += 1;
                        }
                    }
                    Some('f') => {
                        if arg_index < arguments.len() {
                            if let Ok(n) = arguments[arg_index].parse::<f64>() {
                                result.push_str(&format!("{:.6}", n));
                            } else {
                                result.push_str(arguments[arg_index]);
                            }
                            arg_index += 1;
                        }
                    }
                    Some('x') => {
                        if arg_index < arguments.len() {
                            if let Ok(n) = arguments[arg_index].parse::<i64>() {
                                result.push_str(&format!("{:x}", n));
                            } else {
                                result.push_str(arguments[arg_index]);
                            }
                            arg_index += 1;
                        }
                    }
                    Some('X') => {
                        if arg_index < arguments.len() {
                            if let Ok(n) = arguments[arg_index].parse::<i64>() {
                                result.push_str(&format!("{:X}", n));
                            } else {
                                result.push_str(arguments[arg_index]);
                            }
                            arg_index += 1;
                        }
                    }
                    Some('o') => {
                        if arg_index < arguments.len() {
                            if let Ok(n) = arguments[arg_index].parse::<i64>() {
                                result.push_str(&format!("{:o}", n));
                            } else {
                                result.push_str(arguments[arg_index]);
                            }
                            arg_index += 1;
                        }
                    }
                    Some('c') => {
                        if arg_index < arguments.len() && !arguments[arg_index].is_empty() {
                            result.push(arguments[arg_index].chars().next().unwrap());
                            arg_index += 1;
                        }
                    }
                    Some(other) => {
                        result.push('%');
                        result.push(other);
                    }
                    None => result.push('%'),
                }
            } else {
                result.push(c);
            }
        }

        Ok(result)
    }
}
