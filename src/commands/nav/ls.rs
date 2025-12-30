//! ls command - directory listing
//!
//! Lists directory contents with icons and colors.

use std::fs;
use std::path::Path;

use anyhow::Result;
use chrono::{DateTime, Local};

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct LsCommand;

impl Command for LsCommand {
    fn name(&self) -> &'static str {
        "ls"
    }

    fn description(&self) -> &'static str {
        "List directory contents"
    }

    fn usage(&self) -> &'static str {
        "ls [-l] [-a] [path]"
    }

    fn extended_help(&self) -> String {
        r#"ls - List directory contents

USAGE:
  ls [OPTIONS] [path]

OPTIONS:
  -a, --all     Show hidden files (starting with .)
  -l, --long    Long format with details (size, date, permissions)
  -la, -al      Combine -l and -a

DESCRIPTION:
  Lists files and directories with colorful icons.
  Files are color-coded by type and sorted alphabetically.

ICONS:
  📁  Directory       📄  File           🔗  Symlink
  🦀  Rust (.rs)      🐍  Python (.py)   📜  JavaScript (.js)
  ⚙️   Config          📝  Markdown       🖼️   Image

EXAMPLES:
  ls                   List current directory
  ls -l                Long format with details
  ls -a                Show hidden files
  ls -la               Long format + hidden files
  ls ~/projects        List specific directory
  ls *.rs              List matching files (glob)

RELATED COMMANDS:
  tree     Show directory tree
  cd       Change directory
  pwd      Print working directory
"#
        .to_string()
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        let mut show_hidden = false;
        let mut long_format = false;
        let mut path_arg: Option<&str> = None;

        // Parse arguments
        for arg in args {
            match arg.as_str() {
                "-a" | "--all" => show_hidden = true,
                "-l" | "--long" => long_format = true,
                "-la" | "-al" => {
                    show_hidden = true;
                    long_format = true;
                }
                _ if !arg.starts_with('-') => path_arg = Some(arg),
                _ => {} // Ignore unknown flags
            }
        }

        let target_path = match path_arg {
            Some(p) => state.resolve_path(p),
            None => state.cwd().clone(),
        };

        if !target_path.exists() {
            return Err(anyhow::anyhow!(
                "No such file or directory: {}",
                target_path.display()
            ));
        }

        if target_path.is_file() {
            return Ok(format_entry(&target_path, long_format));
        }

        let mut entries: Vec<_> = fs::read_dir(&target_path)?.filter_map(|e| e.ok()).collect();

        // Sort entries: directories first, then alphabetically
        entries.sort_by(|a, b| {
            let a_is_dir = a.file_type().map(|t| t.is_dir()).unwrap_or(false);
            let b_is_dir = b.file_type().map(|t| t.is_dir()).unwrap_or(false);

            match (a_is_dir, b_is_dir) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.file_name().cmp(&b.file_name()),
            }
        });

        let mut output = String::new();

        for entry in entries {
            let name = entry.file_name().to_string_lossy().to_string();

            // Skip hidden files unless -a
            if !show_hidden && name.starts_with('.') {
                continue;
            }

            let path = entry.path();
            let line = format_entry(&path, long_format);
            output.push_str(&line);
            output.push('\n');
        }

        // Remove trailing newline
        if output.ends_with('\n') {
            output.pop();
        }

        Ok(output)
    }
}

/// Format a single directory entry
fn format_entry(path: &Path, long_format: bool) -> String {
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| path.display().to_string());

    let is_dir = path.is_dir();
    let icon = get_icon(path);

    if long_format {
        let metadata = path.metadata().ok();

        let size = metadata
            .as_ref()
            .map(|m| format_size(m.len()))
            .unwrap_or_else(|| "    -".to_string());

        let modified = metadata
            .as_ref()
            .and_then(|m| m.modified().ok())
            .map(|t| {
                let dt: DateTime<Local> = t.into();
                dt.format("%b %d %H:%M").to_string()
            })
            .unwrap_or_else(|| "            ".to_string());

        let type_char = if is_dir { "d" } else { "-" };

        format!("{} {:>8} {} {} {}", type_char, size, modified, icon, name)
    } else if is_dir {
        format!("{} {}/", icon, name)
    } else {
        format!("{} {}", icon, name)
    }
}

/// Get icon for file/directory (Nerd Font icons)
fn get_icon(path: &Path) -> &'static str {
    if path.is_dir() {
        return "\u{f07b}"; //  folder
    }

    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

    let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");

    // Special files
    match name {
        "Cargo.toml" | "Cargo.lock" => return "\u{e7a8}", //  Rust
        "package.json" | "package-lock.json" => return "\u{e718}", //  npm
        ".gitignore" | ".gitattributes" => return "\u{f1d3}", //  git
        "Dockerfile" => return "\u{f308}",                //  docker
        "Makefile" => return "\u{f489}",                  //  terminal
        "README.md" | "README" => return "\u{f48a}",      //  book
        "LICENSE" => return "\u{f718}",                   //  certificate
        _ => {}
    }

    // By extension
    match extension.to_lowercase().as_str() {
        "rs" => "\u{e7a8}",                                            //  Rust
        "js" => "\u{e74e}",                                            //  JavaScript
        "ts" => "\u{e628}",                                            //  TypeScript
        "jsx" | "tsx" => "\u{e7ba}",                                   //  React
        "py" => "\u{e73c}",                                            //  Python
        "go" => "\u{e627}",                                            //  Go
        "java" => "\u{e738}",                                          //  Java
        "c" | "h" => "\u{e61e}",                                       //  C
        "cpp" | "hpp" | "cc" => "\u{e61d}",                            //  C++
        "cs" => "\u{f81a}",                                            //  C#
        "rb" => "\u{e791}",                                            //  Ruby
        "php" => "\u{e73d}",                                           //  PHP
        "swift" => "\u{e755}",                                         //  Swift
        "kt" => "\u{e634}",                                            //  Kotlin
        "json" => "\u{e60b}",                                          //  JSON
        "toml" => "\u{e60b}",                                          //  config
        "yaml" | "yml" => "\u{e60b}",                                  //  config
        "xml" => "\u{e619}",                                           //  XML
        "html" | "htm" => "\u{e736}",                                  //  HTML
        "css" => "\u{e749}",                                           //  CSS
        "scss" | "sass" => "\u{e74b}",                                 //  Sass
        "md" => "\u{e73e}",                                            //  Markdown
        "txt" => "\u{f15c}",                                           //  text
        "pdf" => "\u{f1c1}",                                           //  PDF
        "zip" | "tar" | "gz" | "rar" | "7z" => "\u{f1c6}",             //  archive
        "png" | "jpg" | "jpeg" | "gif" | "svg" | "webp" => "\u{f1c5}", //  image
        "mp3" | "wav" | "flac" | "ogg" => "\u{f1c7}",                  //  audio
        "mp4" | "mkv" | "avi" | "mov" => "\u{f1c8}",                   //  video
        "exe" | "msi" => "\u{f17a}",                                   //  Windows
        "sh" | "bash" | "zsh" => "\u{f489}",                           //  terminal
        "ps1" => "\u{e70f}",                                           //  PowerShell
        _ => "\u{f15b}",                                               //  generic file
    }
}

/// Format file size in human-readable form
fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.1}G", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1}M", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1}K", bytes as f64 / KB as f64)
    } else {
        format!("{}B", bytes)
    }
}
