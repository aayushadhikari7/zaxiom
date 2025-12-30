//! uniq command - report or omit repeated lines

use anyhow::Result;
use std::fs;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct UniqCommand;

impl Command for UniqCommand {
    fn name(&self) -> &'static str {
        "uniq"
    }

    fn description(&self) -> &'static str {
        "Report or omit repeated lines"
    }

    fn usage(&self) -> &'static str {
        "uniq [-c] [-d] [-u] [file]"
    }

    fn extended_help(&self) -> String {
        r#"uniq - Report or omit repeated lines

USAGE:
  uniq [OPTIONS] [file]
  command | uniq [OPTIONS]

OPTIONS:
  -c, --count      Prefix lines with occurrence count
  -d, --repeated   Only print duplicate lines
  -u, --unique     Only print unique lines (appear once)

DESCRIPTION:
  Filter ADJACENT matching lines from input.

  ⚠️  IMPORTANT: uniq only detects ADJACENT duplicates!
  You usually need to SORT first!

EXAMPLES:
  uniq file.txt                Remove adjacent duplicates
  uniq -c file.txt             Count occurrences
  uniq -d file.txt             Show only duplicates
  uniq -u file.txt             Show only unique lines

COMMON PATTERNS (with sort):
  sort file.txt | uniq              Remove ALL duplicates
  sort file.txt | uniq -c           Count all occurrences
  sort file.txt | uniq -c | sort -n Sort by frequency
  sort file.txt | uniq -d           Find any duplicates

WHY SORT FIRST?
  Input:                    Without sort:    With sort:
  apple                     apple            apple
  banana                    banana           apple (removed)
  apple                     apple            banana
  banana                    banana           banana (removed)

  uniq only sees ADJACENT lines, so the repeated
  "apple" and "banana" aren't detected without sorting!

OUTPUT WITH -c:
      3 apple
      2 banana
      1 cherry

COMMON USE CASES:
  • Find duplicate entries in lists
  • Count word frequency
  • Clean up log files
  • Find unique items
  • Analyze data distributions

QUICK RECIPES:
  # Find most common lines
  sort file | uniq -c | sort -rn | head -10

  # Check for any duplicates
  sort file | uniq -d

  # Count unique lines
  sort file | uniq | wc -l

RELATED COMMANDS:
  sort     Sort lines (use before uniq!)
  wc       Count lines/words
  awk      More flexible deduplication
"#
        .to_string()
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        self.execute_with_stdin(args, None, state)
    }

    fn execute_with_stdin(
        &self,
        args: &[String],
        stdin: Option<&str>,
        state: &mut TerminalState,
    ) -> Result<String> {
        let mut count = false;
        let mut only_duplicates = false;
        let mut only_unique = false;
        let mut file: Option<&String> = None;

        for arg in args {
            match arg.as_str() {
                "-c" | "--count" => count = true,
                "-d" | "--repeated" => only_duplicates = true,
                "-u" | "--unique" => only_unique = true,
                "-h" | "--help" => {
                    return Ok("Usage: uniq [OPTIONS] [FILE]\n\
                        Options:\n  \
                        -c    Prefix lines with occurrence count\n  \
                        -d    Only print duplicate lines\n  \
                        -u    Only print unique lines"
                        .to_string());
                }
                _ if !arg.starts_with('-') => file = Some(arg),
                _ => {}
            }
        }

        let content = if let Some(file) = file {
            let path = state.resolve_path(file);
            fs::read_to_string(&path).map_err(|e| anyhow::anyhow!("uniq: {}: {}", file, e))?
        } else if let Some(input) = stdin {
            input.to_string()
        } else {
            return Err(anyhow::anyhow!("uniq: missing file operand"));
        };

        let lines: Vec<&str> = content.lines().collect();
        let mut result: Vec<String> = Vec::new();

        let mut i = 0;
        while i < lines.len() {
            let line = lines[i];
            let mut occurrence = 1;

            while i + occurrence < lines.len() && lines[i + occurrence] == line {
                occurrence += 1;
            }

            let is_duplicate = occurrence > 1;
            let should_include = (!only_duplicates && !only_unique)
                || (only_duplicates && is_duplicate)
                || (only_unique && !is_duplicate);

            if should_include {
                if count {
                    result.push(format!("{:7} {}", occurrence, line));
                } else {
                    result.push(line.to_string());
                }
            }

            i += occurrence;
        }

        Ok(result.join("\n"))
    }

    fn supports_stdin(&self) -> bool {
        true
    }
}
