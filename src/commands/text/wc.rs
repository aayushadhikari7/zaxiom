//! wc command - word, line, character count

use std::fs;
use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct WcCommand;

impl Command for WcCommand {
    fn name(&self) -> &'static str {
        "wc"
    }

    fn description(&self) -> &'static str {
        "Print newline, word, and byte counts"
    }

    fn usage(&self) -> &'static str {
        "wc [-lwc] [file...]"
    }

    fn extended_help(&self) -> String {
        r#"wc - Count lines, words, and characters

USAGE:
  wc [OPTIONS] <file...>
  command | wc [OPTIONS]

OPTIONS:
  -l    Count lines only
  -w    Count words only
  -c    Count bytes/characters only
  (no flags = show all three)

DESCRIPTION:
  Print newline, word, and byte counts for each file.
  With no file, or -, read standard input.

EXAMPLES:
  wc file.txt             Lines, words, chars
  wc -l file.txt          Lines only
  wc -w file.txt          Words only
  wc -c file.txt          Characters only
  wc *.txt                Count for multiple files
  cat file | wc -l        Count lines from pipe

OUTPUT FORMAT:
  <lines> <words> <chars> <filename>
  Example: 100 500 3000 document.txt

COMMON USE CASES:
  • Count lines in a log file
  • Measure document length
  • Verify file size
  • Count records in data files

RELATED COMMANDS:
  head     First N lines
  tail     Last N lines
  cat      Display file
"#.to_string()
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        self.execute_with_stdin(args, None, state)
    }

    fn execute_with_stdin(&self, args: &[String], stdin: Option<&str>, state: &mut TerminalState) -> Result<String> {
        let mut show_lines = false;
        let mut show_words = false;
        let mut show_chars = false;
        let mut show_bytes = false;
        let mut files: Vec<&String> = Vec::new();

        for arg in args {
            match arg.as_str() {
                "-l" | "--lines" => show_lines = true,
                "-w" | "--words" => show_words = true,
                "-c" | "--bytes" => show_bytes = true,
                "-m" | "--chars" => show_chars = true,
                "-h" | "--help" => {
                    return Ok("Usage: wc [OPTIONS] [FILE...]\n\
                        Options:\n  \
                        -l    Print line count\n  \
                        -w    Print word count\n  \
                        -c    Print byte count\n  \
                        -m    Print character count".to_string());
                }
                _ if !arg.starts_with('-') => files.push(arg),
                _ => {}
            }
        }

        // Default: show all
        if !show_lines && !show_words && !show_chars && !show_bytes {
            show_lines = true;
            show_words = true;
            show_bytes = true;
        }

        // Use stdin if no files specified
        if files.is_empty() {
            if let Some(input) = stdin {
                let lines = input.lines().count();
                let words = input.split_whitespace().count();
                let bytes = input.len();
                let chars = input.chars().count();

                let mut parts = Vec::new();
                if show_lines { parts.push(format!("{:8}", lines)); }
                if show_words { parts.push(format!("{:8}", words)); }
                if show_bytes { parts.push(format!("{:8}", bytes)); }
                if show_chars { parts.push(format!("{:8}", chars)); }

                return Ok(parts.join(" "));
            } else {
                return Err(anyhow::anyhow!("wc: missing file operand"));
            }
        }

        let mut output = Vec::new();
        let mut total_lines = 0usize;
        let mut total_words = 0usize;
        let mut total_bytes = 0usize;
        let mut total_chars = 0usize;

        for file in &files {
            let path = state.resolve_path(file);
            let content = fs::read_to_string(&path)
                .map_err(|e| anyhow::anyhow!("wc: {}: {}", file, e))?;

            let lines = content.lines().count();
            let words = content.split_whitespace().count();
            let bytes = content.len();
            let chars = content.chars().count();

            total_lines += lines;
            total_words += words;
            total_bytes += bytes;
            total_chars += chars;

            let mut parts = Vec::new();
            if show_lines { parts.push(format!("{:8}", lines)); }
            if show_words { parts.push(format!("{:8}", words)); }
            if show_bytes { parts.push(format!("{:8}", bytes)); }
            if show_chars { parts.push(format!("{:8}", chars)); }
            parts.push((*file).to_string());

            output.push(parts.join(" "));
        }

        // Show totals if multiple files
        if files.len() > 1 {
            let mut parts = Vec::new();
            if show_lines { parts.push(format!("{:8}", total_lines)); }
            if show_words { parts.push(format!("{:8}", total_words)); }
            if show_bytes { parts.push(format!("{:8}", total_bytes)); }
            if show_chars { parts.push(format!("{:8}", total_chars)); }
            parts.push("total".to_string());
            output.push(parts.join(" "));
        }

        Ok(output.join("\n"))
    }

    fn supports_stdin(&self) -> bool {
        true
    }
}
