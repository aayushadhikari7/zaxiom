//! expr command - evaluate expressions

use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct ExprCommand;

impl Command for ExprCommand {
    fn name(&self) -> &'static str {
        "expr"
    }

    fn description(&self) -> &'static str {
        "Evaluate expressions"
    }

    fn usage(&self) -> &'static str {
        "expr <expression>"
    }

    fn execute(&self, args: &[String], _state: &mut TerminalState) -> Result<String> {
        if args.is_empty() {
            return Err(anyhow::anyhow!("expr: missing operand"));
        }

        if args[0] == "-h" || args[0] == "--help" {
            return Ok("Usage: expr EXPRESSION\n\
                Evaluate EXPRESSION and print result.\n\n\
                Operators:\n  \
                +, -, *, /, %   Arithmetic\n  \
                <, <=, =, !=, >=, >   Comparison\n  \
                |, &   Logical OR/AND\n\n\
                Examples:\n  \
                expr 1 + 2\n  \
                expr 10 \\* 5\n  \
                expr 10 / 3"
                .to_string());
        }

        // Join args and evaluate
        let expr = args.join(" ");
        evaluate_expression(&expr)
    }
}

fn evaluate_expression(expr: &str) -> Result<String> {
    let tokens: Vec<&str> = expr.split_whitespace().collect();

    if tokens.is_empty() {
        return Err(anyhow::anyhow!("expr: missing operand"));
    }

    // Handle string operations
    if tokens.len() == 1 {
        return Ok(tokens[0].to_string());
    }

    // Handle length
    if tokens.len() == 2 && tokens[0] == "length" {
        return Ok(tokens[1].len().to_string());
    }

    // Handle binary operations
    if tokens.len() >= 3 {
        let left = tokens[0];
        let op = tokens[1];
        let right = tokens[2];

        // Try numeric operations
        if let (Ok(l), Ok(r)) = (left.parse::<i64>(), right.parse::<i64>()) {
            let result = match op {
                "+" => l + r,
                "-" => l - r,
                "*" => l * r,
                "/" => {
                    if r == 0 {
                        return Err(anyhow::anyhow!("expr: division by zero"));
                    }
                    l / r
                }
                "%" => {
                    if r == 0 {
                        return Err(anyhow::anyhow!("expr: division by zero"));
                    }
                    l % r
                }
                "<" => {
                    if l < r {
                        1
                    } else {
                        0
                    }
                }
                "<=" => {
                    if l <= r {
                        1
                    } else {
                        0
                    }
                }
                "=" => {
                    if l == r {
                        1
                    } else {
                        0
                    }
                }
                "!=" => {
                    if l != r {
                        1
                    } else {
                        0
                    }
                }
                ">=" => {
                    if l >= r {
                        1
                    } else {
                        0
                    }
                }
                ">" => {
                    if l > r {
                        1
                    } else {
                        0
                    }
                }
                "|" => {
                    if l != 0 {
                        l
                    } else {
                        r
                    }
                }
                "&" => {
                    if l != 0 && r != 0 {
                        l
                    } else {
                        0
                    }
                }
                _ => return Err(anyhow::anyhow!("expr: unknown operator: {}", op)),
            };
            return Ok(result.to_string());
        }

        // String comparison
        match op {
            "=" => return Ok(if left == right { "1" } else { "0" }.to_string()),
            "!=" => return Ok(if left != right { "1" } else { "0" }.to_string()),
            "<" => return Ok(if left < right { "1" } else { "0" }.to_string()),
            ">" => return Ok(if left > right { "1" } else { "0" }.to_string()),
            ":" => {
                // Match operation (simplified)
                if right == ".*" {
                    return Ok(left.len().to_string());
                }
            }
            _ => {}
        }
    }

    Err(anyhow::anyhow!("expr: syntax error"))
}
