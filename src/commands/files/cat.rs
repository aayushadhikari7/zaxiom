//! cat command - print file contents

use std::fs;

use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::img::{is_image_file, image_to_ascii, format_image_info};
use crate::terminal::state::TerminalState;
use crate::terminal::syntax;

pub struct CatCommand;

impl Command for CatCommand {
    fn name(&self) -> &'static str {
        "cat"
    }

    fn description(&self) -> &'static str {
        "Print file contents with optional syntax highlighting"
    }

    fn usage(&self) -> &'static str {
        "cat [-n] [-s|--syntax] [file...]"
    }

    fn extended_help(&self) -> String {
        r#"cat - Print file contents with syntax highlighting

USAGE:
  cat [OPTIONS] <file...>
  command | cat

OPTIONS:
  -n, --number    Show line numbers
  -s, --syntax    Force syntax highlighting
  -p, --plain     Disable syntax highlighting

DESCRIPTION:
  Concatenate and display files. Automatically detects file type
  and applies syntax highlighting for 40+ languages.
  Can also display images as ASCII art!

SUPPORTED LANGUAGES:
  Rust, Python, JavaScript, TypeScript, Go, C, C++, Java,
  Ruby, PHP, Swift, Kotlin, Scala, Haskell, Elixir, and more...

EXAMPLES:
  cat file.txt           Display file contents
  cat -n script.py       Show with line numbers
  cat -s main.rs         Force syntax highlighting
  cat *.md               Concatenate multiple files
  cat image.png          Display image as ASCII art!
  echo "hello" | cat     Read from stdin

IMAGE SUPPORT:
  cat supports PNG, JPG, GIF, BMP, ICO, WEBP
  Images are converted to colored ASCII art

RELATED COMMANDS:
  head     Show first lines
  tail     Show last lines
  less     Page through file (coming soon)
"#.to_string()
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        self.execute_with_stdin(args, None, state)
    }

    fn execute_with_stdin(&self, args: &[String], stdin: Option<&str>, state: &mut TerminalState) -> Result<String> {
        let mut show_line_numbers = false;
        let mut force_syntax = false;
        let mut no_syntax = false;
        let mut files: Vec<&String> = Vec::new();

        for arg in args {
            match arg.as_str() {
                "-n" | "--number" => show_line_numbers = true,
                "-s" | "--syntax" => force_syntax = true,
                "-p" | "--plain" => no_syntax = true,
                "-h" | "--help" => {
                    return Ok("Usage: cat [OPTIONS] [FILE...]\n\
                        Options:\n  \
                        -n, --number   Number all output lines\n  \
                        -s, --syntax   Force syntax highlighting\n  \
                        -p, --plain    Disable syntax highlighting".to_string());
                }
                _ if !arg.starts_with('-') => files.push(arg),
                _ => {}
            }
        }

        // If no files and we have stdin, pass through stdin
        if files.is_empty() {
            if let Some(input) = stdin {
                if show_line_numbers {
                    let numbered: Vec<String> = input
                        .lines()
                        .enumerate()
                        .map(|(i, line)| format!("{:6}\t{}", i + 1, line))
                        .collect();
                    return Ok(numbered.join("\n"));
                }
                return Ok(input.to_string());
            } else {
                return Err(anyhow::anyhow!("cat: missing file operand"));
            }
        }

        let mut output = String::new();
        let mut line_number = 1;

        for file in &files {
            let path = state.resolve_path(file);

            if !path.exists() {
                return Err(anyhow::anyhow!("cat: {}: No such file or directory", file));
            }

            if path.is_dir() {
                return Err(anyhow::anyhow!("cat: {}: Is a directory", file));
            }

            // Check if it's an image file
            if is_image_file(&path) {
                // Display image info
                if let Some(info) = format_image_info(&path) {
                    output.push_str(&format!(" {} - {}\n\n", path.display(), info));
                } else {
                    output.push_str(&format!(" {} (image)\n\n", path.display()));
                }

                // Display ASCII art representation (width based on terminal-ish size)
                if let Some(ascii) = image_to_ascii(&path, 60) {
                    output.push_str(&ascii);
                } else {
                    output.push_str("[Unable to render image as ASCII]\n");
                }
                continue;
            }

            let contents = fs::read_to_string(&path)
                .map_err(|e| anyhow::anyhow!("cat: {}: {}", file, e))?;

            // Check for syntax highlighting
            let should_highlight = !no_syntax && (force_syntax || {
                path.extension()
                    .and_then(|e| e.to_str())
                    .map(|ext| syntax::is_supported(ext))
                    .unwrap_or(false)
            });

            if should_highlight {
                // Get the file extension for syntax detection
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    if let Some(highlighted) = syntax::highlight_code(&contents, ext) {
                        // Show syntax name in header
                        if let Some(syntax_name) = syntax::get_syntax_name(ext) {
                            output.push_str(&format!(" {} [{}]\n", path.display(), syntax_name));
                            output.push_str(&"─".repeat(60));
                            output.push('\n');
                        }

                        // Output highlighted lines with color markers
                        for (line_idx, line_segments) in highlighted.iter().enumerate() {
                            if show_line_numbers {
                                output.push_str(&format!("{:6} │ ", line_number));
                                line_number += 1;
                            }

                            for segment in line_segments {
                                // Use ANSI color codes for terminal display
                                let r = segment.fg.0;
                                let g = segment.fg.1;
                                let b = segment.fg.2;

                                // Clean up the text (remove trailing newline as we add our own)
                                let text = segment.text.trim_end_matches('\n');

                                if segment.bold {
                                    output.push_str(&format!("\x1b[1;38;2;{};{};{}m{}\x1b[0m", r, g, b, text));
                                } else if segment.italic {
                                    output.push_str(&format!("\x1b[3;38;2;{};{};{}m{}\x1b[0m", r, g, b, text));
                                } else {
                                    output.push_str(&format!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, text));
                                }
                            }

                            // Only add newline if it's not the last line or if original had newline
                            if line_idx < highlighted.len() - 1 {
                                output.push('\n');
                            }
                        }
                        output.push('\n');
                        continue;
                    }
                }
            }

            // Fall back to plain text
            if show_line_numbers {
                for line in contents.lines() {
                    output.push_str(&format!("{:6}\t{}\n", line_number, line));
                    line_number += 1;
                }
            } else {
                output.push_str(&contents);
            }
        }

        // Remove trailing newline for cleaner output
        if output.ends_with('\n') {
            output.pop();
        }

        Ok(output)
    }

    fn supports_stdin(&self) -> bool {
        true
    }
}
