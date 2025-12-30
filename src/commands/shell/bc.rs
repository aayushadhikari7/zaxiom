//! bc command - basic calculator

use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct BcCommand;

impl Command for BcCommand {
    fn name(&self) -> &'static str {
        "bc"
    }

    fn description(&self) -> &'static str {
        "Basic calculator"
    }

    fn usage(&self) -> &'static str {
        "bc <expression>"
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        self.execute_with_stdin(args, None, state)
    }

    fn execute_with_stdin(
        &self,
        args: &[String],
        stdin: Option<&str>,
        _state: &mut TerminalState,
    ) -> Result<String> {
        if !args.is_empty() && (args[0] == "-h" || args[0] == "--help") {
            return Ok("Usage: bc [EXPRESSION]\n\
                A basic calculator.\n\n\
                Operators:\n  \
                +, -, *, /   Arithmetic\n  \
                ^            Power\n  \
                %            Modulo\n  \
                ( )          Grouping\n\n\
                Examples:\n  \
                echo '2 + 2' | bc\n  \
                bc '3.14 * 2'"
                .to_string());
        }

        let expr = if !args.is_empty() {
            args.join(" ")
        } else if let Some(input) = stdin {
            input.trim().to_string()
        } else {
            return Err(anyhow::anyhow!("bc: no expression"));
        };

        evaluate_bc(&expr)
    }

    fn supports_stdin(&self) -> bool {
        true
    }
}

fn evaluate_bc(expr: &str) -> Result<String> {
    // Simple expression evaluator
    let expr = expr.trim();

    // Remove any scale setting
    let expr = if expr.starts_with("scale") {
        expr.split(';').next_back().unwrap_or(expr).trim()
    } else {
        expr
    };

    // Tokenize and evaluate
    match parse_and_eval(expr) {
        Ok(result) => {
            // Format result
            if result.fract() == 0.0 {
                Ok(format!("{}", result as i64))
            } else {
                Ok(format!("{:.6}", result)
                    .trim_end_matches('0')
                    .trim_end_matches('.')
                    .to_string())
            }
        }
        Err(e) => Err(e),
    }
}

fn parse_and_eval(expr: &str) -> Result<f64> {
    let expr = expr.replace(" ", "");
    eval_expr(&expr, &mut 0)
}

fn eval_expr(expr: &str, pos: &mut usize) -> Result<f64> {
    let mut result = eval_term(expr, pos)?;

    while *pos < expr.len() {
        let c = expr.chars().nth(*pos).unwrap();
        match c {
            '+' => {
                *pos += 1;
                result += eval_term(expr, pos)?;
            }
            '-' => {
                *pos += 1;
                result -= eval_term(expr, pos)?;
            }
            ')' => break,
            _ => break,
        }
    }

    Ok(result)
}

fn eval_term(expr: &str, pos: &mut usize) -> Result<f64> {
    let mut result = eval_power(expr, pos)?;

    while *pos < expr.len() {
        let c = expr.chars().nth(*pos).unwrap();
        match c {
            '*' => {
                *pos += 1;
                result *= eval_power(expr, pos)?;
            }
            '/' => {
                *pos += 1;
                let divisor = eval_power(expr, pos)?;
                if divisor == 0.0 {
                    return Err(anyhow::anyhow!("bc: division by zero"));
                }
                result /= divisor;
            }
            '%' => {
                *pos += 1;
                let divisor = eval_power(expr, pos)?;
                if divisor == 0.0 {
                    return Err(anyhow::anyhow!("bc: division by zero"));
                }
                result %= divisor;
            }
            _ => break,
        }
    }

    Ok(result)
}

fn eval_power(expr: &str, pos: &mut usize) -> Result<f64> {
    let base = eval_factor(expr, pos)?;

    if *pos < expr.len() && expr.chars().nth(*pos) == Some('^') {
        *pos += 1;
        let exp = eval_power(expr, pos)?;
        return Ok(base.powf(exp));
    }

    Ok(base)
}

fn eval_factor(expr: &str, pos: &mut usize) -> Result<f64> {
    // Skip whitespace
    while *pos < expr.len() && expr.chars().nth(*pos) == Some(' ') {
        *pos += 1;
    }

    if *pos >= expr.len() {
        return Err(anyhow::anyhow!("bc: unexpected end of expression"));
    }

    let c = expr.chars().nth(*pos).unwrap();

    // Handle negative
    if c == '-' {
        *pos += 1;
        return Ok(-eval_factor(expr, pos)?);
    }

    // Handle parentheses
    if c == '(' {
        *pos += 1;
        let result = eval_expr(expr, pos)?;
        if *pos < expr.len() && expr.chars().nth(*pos) == Some(')') {
            *pos += 1;
        }
        return Ok(result);
    }

    // Parse number
    let start = *pos;
    while *pos < expr.len() {
        let c = expr.chars().nth(*pos).unwrap();
        if c.is_ascii_digit() || c == '.' {
            *pos += 1;
        } else {
            break;
        }
    }

    if start == *pos {
        return Err(anyhow::anyhow!("bc: syntax error"));
    }

    let num_str: String = expr.chars().skip(start).take(*pos - start).collect();
    num_str
        .parse::<f64>()
        .map_err(|_| anyhow::anyhow!("bc: invalid number"))
}
