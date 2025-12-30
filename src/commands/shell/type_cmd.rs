//! type command - display information about command type

use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct TypeCommand;

// List of builtin commands (should match registry)
const BUILTINS: &[&str] = &[
    "ls", "cd", "pwd", "clear", "tree", "help",
    "cat", "touch", "rm", "mkdir", "cp", "mv", "ln", "stat", "file", "basename", "dirname", "realpath",
    "echo", "head", "tail", "wc", "sort", "uniq", "tac", "cut", "paste", "diff", "tr", "sed", "awk", "rev", "nl", "printf",
    "grep", "find",
    "exit", "quit", "which", "du", "df", "ps", "kill", "whoami", "hostname", "uname", "uptime", "free", "date", "cal", "id", "neofetch",
    "curl", "wget", "ping", "netstat", "traceroute",
    "md5sum", "sha1sum", "sha256sum", "base64", "xxd",
    "tar", "zip", "unzip", "gzip", "gunzip",
    "alias", "env", "export", "sleep", "watch", "seq", "yes", "true", "false", "expr", "bc", "tee", "timeout",
    "type", "command", "printenv", "lscpu", "history", "test",
    "chmod", "readlink", "mktemp",
    "xargs", "column", "strings", "split", "join", "comm",
    "pushd", "popd", "dirs",
    "nslookup", "host", "ifconfig",
];

impl Command for TypeCommand {
    fn name(&self) -> &'static str {
        "type"
    }

    fn description(&self) -> &'static str {
        "Display information about command type"
    }

    fn usage(&self) -> &'static str {
        "type [-a] [-t] name..."
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        let mut show_all = false;
        let mut type_only = false;
        let mut names: Vec<&str> = Vec::new();

        for arg in args {
            match arg.as_str() {
                "-a" | "--all" => show_all = true,
                "-t" | "--type" => type_only = true,
                arg if !arg.starts_with('-') => names.push(arg),
                _ => {}
            }
        }

        if names.is_empty() {
            return Err(anyhow::anyhow!("type: missing argument"));
        }

        let mut output = Vec::new();

        for name in names {
            // Check if builtin
            if BUILTINS.contains(&name) {
                if type_only {
                    output.push("builtin".to_string());
                } else {
                    output.push(format!("{} is a shell builtin", name));
                }
                if !show_all {
                    continue;
                }
            }

            // Check if alias
            if let Some(alias_value) = state.get_alias(name) {
                if type_only {
                    output.push("alias".to_string());
                } else {
                    output.push(format!("{} is aliased to `{}`", name, alias_value));
                }
                if !show_all {
                    continue;
                }
            }

            // Check if external command
            if let Some(path) = find_in_path(name) {
                if type_only {
                    output.push("file".to_string());
                } else {
                    output.push(format!("{} is {}", name, path));
                }
            } else if (output.is_empty() || output.last().map(|s| !s.contains(name)).unwrap_or(true))
                && !type_only {
                    output.push(format!("{}: not found", name));
                }
        }

        Ok(output.join("\n"))
    }
}

fn find_in_path(name: &str) -> Option<String> {
    let path_var = std::env::var("PATH").ok()?;
    let extensions = ["", ".exe", ".cmd", ".bat", ".com"];

    for dir in path_var.split(';') {
        for ext in &extensions {
            let full_path = std::path::Path::new(dir).join(format!("{}{}", name, ext));
            if full_path.exists() {
                return Some(full_path.display().to_string());
            }
        }
    }

    None
}
