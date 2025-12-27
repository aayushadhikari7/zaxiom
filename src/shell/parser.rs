//! Command parser using nom
//!
//! Tokenizes and parses command line input.

#![allow(dead_code)]

use nom::{
    IResult,
    Parser,
    branch::alt,
    bytes::complete::{tag, take_while, take_while1},
    character::complete::{char, multispace0, space0},
    multi::{many0, separated_list0},
    sequence::{delimited, preceded},
};

/// Redirection type
#[derive(Debug, Clone, PartialEq)]
pub enum RedirectType {
    /// > file (overwrite)
    Output,
    /// >> file (append)
    Append,
    /// < file (input)
    Input,
}

/// A redirection specification
#[derive(Debug, Clone)]
pub struct Redirection {
    pub redirect_type: RedirectType,
    pub target: String,
}

/// A parsed command with arguments
#[derive(Debug, Clone)]
pub struct ParsedCommand {
    /// Command name
    pub command: String,
    /// Arguments
    pub args: Vec<String>,
    /// Input/output redirections
    pub redirections: Vec<Redirection>,
}

/// A pipeline of commands (cmd1 | cmd2 | cmd3)
#[derive(Debug, Clone)]
pub struct Pipeline {
    pub commands: Vec<ParsedCommand>,
}

impl Pipeline {
    /// Check if this is a single command
    pub fn is_single(&self) -> bool {
        self.commands.len() == 1
    }

    /// Get the first (or only) command
    pub fn first(&self) -> Option<&ParsedCommand> {
        self.commands.first()
    }
}

/// Parse a command line into a pipeline
pub fn parse_command_line(input: &str) -> Result<Pipeline, String> {
    match pipeline(input.trim()) {
        Ok((remaining, pipeline)) => {
            if remaining.trim().is_empty() {
                Ok(pipeline)
            } else {
                Err(format!("Unexpected input: {}", remaining))
            }
        }
        Err(e) => Err(format!("Parse error: {:?}", e)),
    }
}

/// Parse a pipeline (commands separated by |)
fn pipeline(input: &str) -> IResult<&str, Pipeline> {
    let pipe_sep = delimited(space0, char('|'), space0);
    let (input, commands) = separated_list0(pipe_sep, single_command).parse(input)?;
    Ok((input, Pipeline { commands }))
}

/// Parse a single command with arguments and redirections
fn single_command(input: &str) -> IResult<&str, ParsedCommand> {
    let (input, _) = multispace0(input)?;
    let (input, tokens) = many0(preceded(space0, token)).parse(input)?;

    if tokens.is_empty() {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::NonEmpty,
        )));
    }

    // Separate arguments from redirections
    let mut args: Vec<String> = Vec::new();
    let mut redirections: Vec<Redirection> = Vec::new();
    let mut i = 0;

    while i < tokens.len() {
        let token = &tokens[i];

        if token == ">>" {
            // Append redirection
            if i + 1 < tokens.len() {
                redirections.push(Redirection {
                    redirect_type: RedirectType::Append,
                    target: tokens[i + 1].clone(),
                });
                i += 2;
                continue;
            }
        } else if token == ">" {
            // Output redirection
            if i + 1 < tokens.len() {
                redirections.push(Redirection {
                    redirect_type: RedirectType::Output,
                    target: tokens[i + 1].clone(),
                });
                i += 2;
                continue;
            }
        } else if token == "<" {
            // Input redirection
            if i + 1 < tokens.len() {
                redirections.push(Redirection {
                    redirect_type: RedirectType::Input,
                    target: tokens[i + 1].clone(),
                });
                i += 2;
                continue;
            }
        }

        args.push(token.clone());
        i += 1;
    }

    if args.is_empty() {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::NonEmpty,
        )));
    }

    let command = args[0].clone();
    let args = args[1..].to_vec();

    Ok((input, ParsedCommand { command, args, redirections }))
}

/// Parse a single token (redirection operator, quoted string, or word)
fn token(input: &str) -> IResult<&str, String> {
    alt((
        redirection_operator,
        double_quoted_string,
        single_quoted_string,
        unquoted_word,
    )).parse(input)
}

/// Parse redirection operators
fn redirection_operator(input: &str) -> IResult<&str, String> {
    alt((
        tag(">>").map(|s: &str| s.to_string()),
        tag(">").map(|s: &str| s.to_string()),
        tag("<").map(|s: &str| s.to_string()),
    )).parse(input)
}

/// Parse a single argument (quoted or unquoted) - kept for compatibility
fn argument(input: &str) -> IResult<&str, String> {
    alt((
        double_quoted_string,
        single_quoted_string,
        unquoted_word,
    )).parse(input)
}

/// Parse a double-quoted string
fn double_quoted_string(input: &str) -> IResult<&str, String> {
    let (input, _) = char('"')(input)?;
    let (input, content) = take_while(|c| c != '"').parse(input)?;
    let (input, _) = char('"')(input)?;

    // Basic unescaping
    let unescaped = content
        .replace("\\n", "\n")
        .replace("\\t", "\t")
        .replace("\\\"", "\"")
        .replace("\\\\", "\\");

    Ok((input, unescaped))
}

/// Parse a single-quoted string (no escaping)
fn single_quoted_string(input: &str) -> IResult<&str, String> {
    let (input, _) = char('\'')(input)?;
    let (input, content) = take_while(|c| c != '\'').parse(input)?;
    let (input, _) = char('\'')(input)?;
    Ok((input, content.to_string()))
}

/// Parse an unquoted word (stops at whitespace, pipe, quotes, and redirection operators)
fn unquoted_word(input: &str) -> IResult<&str, String> {
    let (input, s) = take_while1(|c: char| {
        !c.is_whitespace() && c != '|' && c != '"' && c != '\'' && c != '>' && c != '<'
    }).parse(input)?;

    Ok((input, s.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_command() {
        let result = parse_command_line("ls -la").unwrap();
        assert_eq!(result.commands.len(), 1);
        assert_eq!(result.commands[0].command, "ls");
        assert_eq!(result.commands[0].args, vec!["-la"]);
    }

    #[test]
    fn test_quoted_args() {
        let result = parse_command_line(r#"echo "hello world""#).unwrap();
        assert_eq!(result.commands[0].command, "echo");
        assert_eq!(result.commands[0].args, vec!["hello world"]);
    }

    #[test]
    fn test_pipeline() {
        let result = parse_command_line("ls | grep foo").unwrap();
        assert_eq!(result.commands.len(), 2);
        assert_eq!(result.commands[0].command, "ls");
        assert_eq!(result.commands[1].command, "grep");
    }

    #[test]
    fn test_output_redirect() {
        let result = parse_command_line("ls > output.txt").unwrap();
        assert_eq!(result.commands.len(), 1);
        assert_eq!(result.commands[0].command, "ls");
        assert_eq!(result.commands[0].redirections.len(), 1);
        assert_eq!(result.commands[0].redirections[0].redirect_type, RedirectType::Output);
        assert_eq!(result.commands[0].redirections[0].target, "output.txt");
    }

    #[test]
    fn test_append_redirect() {
        let result = parse_command_line("echo hello >> log.txt").unwrap();
        assert_eq!(result.commands[0].command, "echo");
        assert_eq!(result.commands[0].args, vec!["hello"]);
        assert_eq!(result.commands[0].redirections[0].redirect_type, RedirectType::Append);
    }

    #[test]
    fn test_input_redirect() {
        let result = parse_command_line("sort < data.txt").unwrap();
        assert_eq!(result.commands[0].command, "sort");
        assert_eq!(result.commands[0].redirections[0].redirect_type, RedirectType::Input);
        assert_eq!(result.commands[0].redirections[0].target, "data.txt");
    }
}
