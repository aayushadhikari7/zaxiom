//! find command - search for files

use anyhow::Result;
use walkdir::WalkDir;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct FindCommand;

impl Command for FindCommand {
    fn name(&self) -> &'static str {
        "find"
    }

    fn description(&self) -> &'static str {
        "Search for files by name"
    }

    fn usage(&self) -> &'static str {
        "find [path] -name <pattern>"
    }

    fn extended_help(&self) -> String {
        r#"find - Search for files and directories

USAGE:
  find [path] [OPTIONS]

OPTIONS:
  -name <pattern>    Search by filename pattern (supports wildcards)
  -type f            Find only files
  -type d            Find only directories

DESCRIPTION:
  Recursively search for files matching the given criteria.
  Patterns support * and ? wildcards.

EXAMPLES:
  find . -name "*.rs"           Find all Rust files
  find . -name "*.js" -type f   Find JS files only (not dirs)
  find ~/projects -name "README*"   Find README files
  find . -type d -name "src"    Find directories named "src"
  find . -name "test*"          Find files starting with "test"

PATTERNS:
  *        Match any characters
  ?        Match single character
  [abc]    Match a, b, or c

RELATED COMMANDS:
  grep     Search file contents
  ls       List directory
  tree     Show directory tree
"#.to_string()
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        let mut search_path = None;
        let mut name_pattern = None;
        let mut file_type = None; // "f" for file, "d" for directory

        let mut iter = args.iter().peekable();
        while let Some(arg) = iter.next() {
            match arg.as_str() {
                "-name" => {
                    name_pattern = iter.next().map(|s| s.as_str());
                }
                "-type" => {
                    file_type = iter.next().map(|s| s.as_str());
                }
                _ if !arg.starts_with('-') && search_path.is_none() => {
                    search_path = Some(arg.as_str());
                }
                _ => {}
            }
        }

        let base_path = match search_path {
            Some(p) => state.resolve_path(p),
            None => state.cwd().clone(),
        };

        if !base_path.exists() {
            return Err(anyhow::anyhow!("No such directory: {}", base_path.display()));
        }

        let mut results = Vec::new();

        for entry in WalkDir::new(&base_path).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();

            // Check type filter
            if let Some(t) = file_type {
                let is_match = match t {
                    "f" => path.is_file(),
                    "d" => path.is_dir(),
                    _ => true,
                };
                if !is_match {
                    continue;
                }
            }

            // Check name pattern
            if let Some(pattern) = name_pattern {
                let name = path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("");

                if !matches_glob(name, pattern) {
                    continue;
                }
            }

            // Format path with forward slashes
            let display_path = path.strip_prefix(&base_path)
                .unwrap_or(path)
                .display()
                .to_string()
                .replace('\\', "/");

            if display_path.is_empty() {
                results.push(".".to_string());
            } else {
                results.push(format!("./{}", display_path));
            }
        }

        Ok(results.join("\n"))
    }
}

/// Simple glob matching (supports * and ?)
fn matches_glob(name: &str, pattern: &str) -> bool {
    let pattern = pattern.to_lowercase();
    let name = name.to_lowercase();

    let mut pattern_chars = pattern.chars().peekable();
    let mut name_chars = name.chars().peekable();

    while let Some(p) = pattern_chars.next() {
        match p {
            '*' => {
                // * matches any sequence
                if pattern_chars.peek().is_none() {
                    // Trailing * matches everything
                    return true;
                }

                // Try matching rest of pattern at each position
                let rest_pattern: String = pattern_chars.collect();
                let mut remaining: String = name_chars.collect();

                while !remaining.is_empty() {
                    if matches_glob(&remaining, &rest_pattern) {
                        return true;
                    }
                    remaining = remaining.chars().skip(1).collect();
                }

                return matches_glob("", &rest_pattern);
            }
            '?' => {
                // ? matches any single character
                if name_chars.next().is_none() {
                    return false;
                }
            }
            c => {
                // Literal match
                if name_chars.next() != Some(c) {
                    return false;
                }
            }
        }
    }

    // Pattern consumed, name should also be consumed
    name_chars.next().is_none()
}
