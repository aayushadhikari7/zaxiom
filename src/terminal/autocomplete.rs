//! Autocomplete system
//!
//! Provides intelligent suggestions for commands, paths, git branches, and flags.

#![allow(dead_code)]

use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Autocomplete suggestion with metadata
#[derive(Clone, Debug)]
pub struct Suggestion {
    /// The suggestion text
    pub text: String,
    /// Type of suggestion for styling
    pub kind: SuggestionKind,
    /// Description or context
    pub description: Option<String>,
}

/// Type of autocomplete suggestion
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SuggestionKind {
    /// Command from history
    History,
    /// File path
    File,
    /// Directory path
    Directory,
    /// Git branch
    GitBranch,
    /// Command flag/option
    Flag,
    /// Built-in command
    Command,
    /// Environment variable
    EnvVar,
}

/// Autocomplete engine
pub struct Autocomplete {
    /// Command flag definitions
    flags: HashMap<&'static str, Vec<FlagDef>>,
    /// Built-in command names
    builtins: Vec<&'static str>,
}

/// Flag definition for a command
#[derive(Clone)]
struct FlagDef {
    short: Option<&'static str>,
    long: Option<&'static str>,
    description: &'static str,
    takes_value: bool,
}

impl Autocomplete {
    /// Create a new autocomplete engine
    pub fn new() -> Self {
        let mut ac = Self {
            flags: HashMap::new(),
            builtins: vec![
                "cd", "ls", "pwd", "echo", "cat", "head", "tail", "grep", "find", "cp", "mv", "rm",
                "mkdir", "rmdir", "touch", "chmod", "clear", "history", "alias", "unalias",
                "export", "env", "which", "help", "exit", "neofetch", "fortune", "cowsay",
                "matrix", "coffee", "wc", "sort", "uniq", "cut", "tr", "sed", "awk", "xargs",
                "tar", "gzip", "gunzip", "zip", "unzip", "curl", "wget", "ps", "kill", "top", "df",
                "du", "free", "uptime", "whoami", "date", "cal", "diff", "nano", "less", "more",
                "tree",
            ],
        };
        ac.init_flags();
        ac
    }

    /// Initialize flag definitions for common commands
    fn init_flags(&mut self) {
        // ls flags
        self.flags.insert(
            "ls",
            vec![
                FlagDef {
                    short: Some("-l"),
                    long: Some("--long"),
                    description: "Long listing format",
                    takes_value: false,
                },
                FlagDef {
                    short: Some("-a"),
                    long: Some("--all"),
                    description: "Show hidden files",
                    takes_value: false,
                },
                FlagDef {
                    short: Some("-h"),
                    long: Some("--human-readable"),
                    description: "Human readable sizes",
                    takes_value: false,
                },
                FlagDef {
                    short: Some("-R"),
                    long: Some("--recursive"),
                    description: "List recursively",
                    takes_value: false,
                },
                FlagDef {
                    short: Some("-S"),
                    long: None,
                    description: "Sort by size",
                    takes_value: false,
                },
                FlagDef {
                    short: Some("-t"),
                    long: None,
                    description: "Sort by time",
                    takes_value: false,
                },
                FlagDef {
                    short: Some("-r"),
                    long: Some("--reverse"),
                    description: "Reverse sort order",
                    takes_value: false,
                },
            ],
        );

        // grep flags
        self.flags.insert(
            "grep",
            vec![
                FlagDef {
                    short: Some("-i"),
                    long: Some("--ignore-case"),
                    description: "Case insensitive",
                    takes_value: false,
                },
                FlagDef {
                    short: Some("-r"),
                    long: Some("--recursive"),
                    description: "Search recursively",
                    takes_value: false,
                },
                FlagDef {
                    short: Some("-n"),
                    long: Some("--line-number"),
                    description: "Show line numbers",
                    takes_value: false,
                },
                FlagDef {
                    short: Some("-v"),
                    long: Some("--invert-match"),
                    description: "Invert match",
                    takes_value: false,
                },
                FlagDef {
                    short: Some("-c"),
                    long: Some("--count"),
                    description: "Count matches",
                    takes_value: false,
                },
                FlagDef {
                    short: Some("-l"),
                    long: Some("--files-with-matches"),
                    description: "Show only filenames",
                    takes_value: false,
                },
                FlagDef {
                    short: Some("-A"),
                    long: Some("--after-context"),
                    description: "Lines after match",
                    takes_value: true,
                },
                FlagDef {
                    short: Some("-B"),
                    long: Some("--before-context"),
                    description: "Lines before match",
                    takes_value: true,
                },
            ],
        );

        // git flags
        self.flags.insert(
            "git",
            vec![
                FlagDef {
                    short: None,
                    long: Some("status"),
                    description: "Show working tree status",
                    takes_value: false,
                },
                FlagDef {
                    short: None,
                    long: Some("add"),
                    description: "Add files to staging",
                    takes_value: false,
                },
                FlagDef {
                    short: None,
                    long: Some("commit"),
                    description: "Commit changes",
                    takes_value: false,
                },
                FlagDef {
                    short: None,
                    long: Some("push"),
                    description: "Push to remote",
                    takes_value: false,
                },
                FlagDef {
                    short: None,
                    long: Some("pull"),
                    description: "Pull from remote",
                    takes_value: false,
                },
                FlagDef {
                    short: None,
                    long: Some("checkout"),
                    description: "Switch branches",
                    takes_value: false,
                },
                FlagDef {
                    short: None,
                    long: Some("branch"),
                    description: "List/create branches",
                    takes_value: false,
                },
                FlagDef {
                    short: None,
                    long: Some("merge"),
                    description: "Merge branches",
                    takes_value: false,
                },
                FlagDef {
                    short: None,
                    long: Some("log"),
                    description: "Show commit history",
                    takes_value: false,
                },
                FlagDef {
                    short: None,
                    long: Some("diff"),
                    description: "Show changes",
                    takes_value: false,
                },
                FlagDef {
                    short: None,
                    long: Some("stash"),
                    description: "Stash changes",
                    takes_value: false,
                },
            ],
        );

        // find flags
        self.flags.insert(
            "find",
            vec![
                FlagDef {
                    short: None,
                    long: Some("-name"),
                    description: "Search by name pattern",
                    takes_value: true,
                },
                FlagDef {
                    short: None,
                    long: Some("-type"),
                    description: "Filter by type (f/d)",
                    takes_value: true,
                },
                FlagDef {
                    short: None,
                    long: Some("-size"),
                    description: "Filter by size",
                    takes_value: true,
                },
                FlagDef {
                    short: None,
                    long: Some("-mtime"),
                    description: "Filter by modification time",
                    takes_value: true,
                },
            ],
        );

        // cp/mv flags
        for cmd in &["cp", "mv"] {
            self.flags.insert(
                *cmd,
                vec![
                    FlagDef {
                        short: Some("-r"),
                        long: Some("--recursive"),
                        description: "Copy/move recursively",
                        takes_value: false,
                    },
                    FlagDef {
                        short: Some("-f"),
                        long: Some("--force"),
                        description: "Force overwrite",
                        takes_value: false,
                    },
                    FlagDef {
                        short: Some("-i"),
                        long: Some("--interactive"),
                        description: "Prompt before overwrite",
                        takes_value: false,
                    },
                    FlagDef {
                        short: Some("-v"),
                        long: Some("--verbose"),
                        description: "Verbose output",
                        takes_value: false,
                    },
                ],
            );
        }

        // rm flags
        self.flags.insert(
            "rm",
            vec![
                FlagDef {
                    short: Some("-r"),
                    long: Some("--recursive"),
                    description: "Remove recursively",
                    takes_value: false,
                },
                FlagDef {
                    short: Some("-f"),
                    long: Some("--force"),
                    description: "Force removal",
                    takes_value: false,
                },
                FlagDef {
                    short: Some("-i"),
                    long: Some("--interactive"),
                    description: "Prompt before removal",
                    takes_value: false,
                },
                FlagDef {
                    short: Some("-v"),
                    long: Some("--verbose"),
                    description: "Verbose output",
                    takes_value: false,
                },
            ],
        );

        // cat flags
        self.flags.insert(
            "cat",
            vec![
                FlagDef {
                    short: Some("-n"),
                    long: Some("--number"),
                    description: "Number all lines",
                    takes_value: false,
                },
                FlagDef {
                    short: Some("-b"),
                    long: Some("--number-nonblank"),
                    description: "Number non-blank lines",
                    takes_value: false,
                },
            ],
        );

        // tar flags
        self.flags.insert(
            "tar",
            vec![
                FlagDef {
                    short: Some("-c"),
                    long: Some("--create"),
                    description: "Create archive",
                    takes_value: false,
                },
                FlagDef {
                    short: Some("-x"),
                    long: Some("--extract"),
                    description: "Extract archive",
                    takes_value: false,
                },
                FlagDef {
                    short: Some("-t"),
                    long: Some("--list"),
                    description: "List archive contents",
                    takes_value: false,
                },
                FlagDef {
                    short: Some("-v"),
                    long: Some("--verbose"),
                    description: "Verbose output",
                    takes_value: false,
                },
                FlagDef {
                    short: Some("-z"),
                    long: Some("--gzip"),
                    description: "Use gzip compression",
                    takes_value: false,
                },
                FlagDef {
                    short: Some("-f"),
                    long: Some("--file"),
                    description: "Archive file name",
                    takes_value: true,
                },
            ],
        );
    }

    /// Get suggestions for the current input
    pub fn suggest(
        &self,
        input: &str,
        cursor_pos: usize,
        cwd: &Path,
        history: &[String],
    ) -> Vec<Suggestion> {
        let input = &input[..cursor_pos.min(input.len())];
        let parts: Vec<&str> = input.split_whitespace().collect();

        if parts.is_empty() || (parts.len() == 1 && !input.ends_with(' ')) {
            // Suggesting command names
            let prefix = parts.first().copied().unwrap_or("");
            return self.suggest_commands(prefix, history);
        }

        let cmd = parts[0];
        let current_word = if input.ends_with(' ') {
            ""
        } else {
            parts.last().copied().unwrap_or("")
        };

        // Check if we're completing a flag
        if current_word.starts_with('-') {
            return self.suggest_flags(cmd, current_word);
        }

        // Check for git branch completion
        if cmd == "git"
            && (parts.contains(&"checkout")
                || parts.contains(&"merge")
                || parts.contains(&"branch"))
        {
            return self.suggest_git_branches(current_word, cwd);
        }

        // Default to path completion
        self.suggest_paths(current_word, cwd)
    }

    /// Suggest command names from history and builtins
    fn suggest_commands(&self, prefix: &str, history: &[String]) -> Vec<Suggestion> {
        let mut suggestions = Vec::new();
        let prefix_lower = prefix.to_lowercase();

        // History suggestions (most recent first, deduplicated)
        let mut seen = std::collections::HashSet::new();
        for cmd in history.iter().rev() {
            let first_word = cmd.split_whitespace().next().unwrap_or("");
            if first_word.to_lowercase().starts_with(&prefix_lower) && seen.insert(cmd.clone()) {
                suggestions.push(Suggestion {
                    text: cmd.clone(),
                    kind: SuggestionKind::History,
                    description: Some("from history".to_string()),
                });
                if suggestions.len() >= 5 {
                    break;
                }
            }
        }

        // Built-in commands
        for builtin in &self.builtins {
            if builtin.starts_with(&prefix_lower) {
                suggestions.push(Suggestion {
                    text: builtin.to_string(),
                    kind: SuggestionKind::Command,
                    description: None,
                });
            }
        }

        // Common external commands
        let externals = [
            "git", "python", "node", "npm", "cargo", "rustc", "code", "vim",
        ];
        for ext in externals {
            if ext.starts_with(&prefix_lower) && !self.builtins.contains(&ext) {
                suggestions.push(Suggestion {
                    text: ext.to_string(),
                    kind: SuggestionKind::Command,
                    description: Some("external".to_string()),
                });
            }
        }

        suggestions.truncate(10);
        suggestions
    }

    /// Suggest flags for a command
    fn suggest_flags(&self, cmd: &str, prefix: &str) -> Vec<Suggestion> {
        let mut suggestions = Vec::new();

        if let Some(flags) = self.flags.get(cmd) {
            for flag in flags {
                if let Some(short) = flag.short {
                    if short.starts_with(prefix) {
                        suggestions.push(Suggestion {
                            text: short.to_string(),
                            kind: SuggestionKind::Flag,
                            description: Some(flag.description.to_string()),
                        });
                    }
                }
                if let Some(long) = flag.long {
                    let long_flag = if long.starts_with('-') {
                        long.to_string()
                    } else {
                        format!("--{}", long)
                    };
                    if long_flag.starts_with(prefix) || (prefix == "-" && !long.starts_with('-')) {
                        suggestions.push(Suggestion {
                            text: long.to_string(),
                            kind: SuggestionKind::Flag,
                            description: Some(flag.description.to_string()),
                        });
                    }
                }
            }
        }

        suggestions
    }

    /// Suggest git branches
    fn suggest_git_branches(&self, prefix: &str, cwd: &Path) -> Vec<Suggestion> {
        let mut suggestions = Vec::new();

        // Try to find git directory and read branches
        let mut current = cwd.to_path_buf();
        loop {
            let git_dir = current.join(".git");
            if git_dir.is_dir() {
                // Read local branches
                let heads_dir = git_dir.join("refs/heads");
                if let Ok(entries) = fs::read_dir(&heads_dir) {
                    for entry in entries.filter_map(|e| e.ok()) {
                        let name = entry.file_name().to_string_lossy().to_string();
                        if name.to_lowercase().starts_with(&prefix.to_lowercase()) {
                            suggestions.push(Suggestion {
                                text: name,
                                kind: SuggestionKind::GitBranch,
                                description: Some("branch".to_string()),
                            });
                        }
                    }
                }
                break;
            }

            if !current.pop() {
                break;
            }
        }

        suggestions
    }

    /// Suggest file/directory paths
    fn suggest_paths(&self, prefix: &str, cwd: &Path) -> Vec<Suggestion> {
        let mut suggestions = Vec::new();

        // Determine the directory to search and the file prefix
        let (search_dir, file_prefix) = if prefix.contains('/') || prefix.contains('\\') {
            let path = Path::new(prefix);
            if prefix.ends_with('/') || prefix.ends_with('\\') {
                (cwd.join(prefix), String::new())
            } else {
                let parent = path.parent().unwrap_or(Path::new("."));
                let file_part = path
                    .file_name()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_default();
                (cwd.join(parent), file_part)
            }
        } else {
            (cwd.to_path_buf(), prefix.to_string())
        };

        // Read directory entries
        if let Ok(entries) = fs::read_dir(&search_dir) {
            for entry in entries.filter_map(|e| e.ok()) {
                let name = entry.file_name().to_string_lossy().to_string();

                // Skip hidden files unless prefix starts with .
                if name.starts_with('.') && !file_prefix.starts_with('.') {
                    continue;
                }

                if name.to_lowercase().starts_with(&file_prefix.to_lowercase()) {
                    let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);

                    // Build the completion text
                    let completion = if prefix.contains('/') || prefix.contains('\\') {
                        let parent = Path::new(prefix).parent().unwrap_or(Path::new(""));
                        let mut path = parent.join(&name).to_string_lossy().to_string();
                        path = path.replace('\\', "/");
                        if is_dir {
                            path.push('/');
                        }
                        path
                    } else if is_dir {
                        format!("{}/", name)
                    } else {
                        name.clone()
                    };

                    suggestions.push(Suggestion {
                        text: completion,
                        kind: if is_dir {
                            SuggestionKind::Directory
                        } else {
                            SuggestionKind::File
                        },
                        description: if is_dir {
                            Some("directory".to_string())
                        } else {
                            None
                        },
                    });
                }
            }
        }

        // Sort: directories first, then alphabetically
        suggestions.sort_by(|a, b| match (a.kind, b.kind) {
            (SuggestionKind::Directory, SuggestionKind::File) => std::cmp::Ordering::Less,
            (SuggestionKind::File, SuggestionKind::Directory) => std::cmp::Ordering::Greater,
            _ => a.text.to_lowercase().cmp(&b.text.to_lowercase()),
        });

        suggestions.truncate(15);
        suggestions
    }

    /// Apply a suggestion to the input
    pub fn apply_suggestion(
        &self,
        input: &str,
        cursor_pos: usize,
        suggestion: &Suggestion,
    ) -> (String, usize) {
        let before_cursor = &input[..cursor_pos.min(input.len())];
        let after_cursor = &input[cursor_pos.min(input.len())..];

        // Find the word being completed
        let word_start = before_cursor
            .rfind(|c: char| c.is_whitespace())
            .map(|i| i + 1)
            .unwrap_or(0);

        let new_input = format!(
            "{}{}{}",
            &before_cursor[..word_start],
            suggestion.text,
            after_cursor
        );

        let new_cursor = word_start + suggestion.text.len();

        (new_input, new_cursor)
    }
}

impl Default for Autocomplete {
    fn default() -> Self {
        Self::new()
    }
}
