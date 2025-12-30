//! echo command - print text

use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct EchoCommand;

impl Command for EchoCommand {
    fn name(&self) -> &'static str {
        "echo"
    }

    fn description(&self) -> &'static str {
        "Print text to output"
    }

    fn usage(&self) -> &'static str {
        "echo [text...]"
    }

    fn extended_help(&self) -> String {
        r#"echo - Print text to output

USAGE:
  echo [text...]

DESCRIPTION:
  Display a line of text. Simple but powerful when
  combined with pipes and redirections.

EXAMPLES:
  echo Hello World            Print "Hello World"
  echo "Hello World"          Same, with quotes
  echo $HOME                  Print environment variable
  echo "Line 1\nLine 2"       Print with newline
  echo one two three          Print with spaces

WITH REDIRECTION:
  echo "Hello" > file.txt     Write to file
  echo "More" >> file.txt     Append to file

WITH PIPES:
  echo "search term" | grep term
  echo "3 + 4" | bc

ENVIRONMENT VARIABLES:
  echo $PATH                  Show PATH
  echo $HOME                  Show home directory
  echo $USER                  Show username

RELATED COMMANDS:
  printf   Formatted output
  cat      Print file contents
"#
        .to_string()
    }

    fn execute(&self, args: &[String], _state: &mut TerminalState) -> Result<String> {
        Ok(args.join(" "))
    }
}
