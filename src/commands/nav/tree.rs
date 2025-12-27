//! tree command - list directory contents in tree format

use std::fs;
use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct TreeCommand;

impl Command for TreeCommand {
    fn name(&self) -> &'static str {
        "tree"
    }

    fn description(&self) -> &'static str {
        "List directory contents in tree format"
    }

    fn usage(&self) -> &'static str {
        "tree [-L n] [-a] [-d] [directory]"
    }

    fn extended_help(&self) -> String {
        r#"tree - List directory contents in tree format

USAGE:
  tree [OPTIONS] [directory]

OPTIONS:
  -L <n>    Limit display to n levels deep
  -a        Show hidden files (starting with .)
  -d        Show directories only (no files)

DESCRIPTION:
  Display directory structure as an indented tree.
  Great for visualizing project layouts!

EXAMPLES:
  tree                    Current directory tree
  tree src/               Tree of src folder
  tree -L 2               Only 2 levels deep
  tree -a                 Include hidden files
  tree -d                 Directories only
  tree -L 3 -d project/   3 levels, dirs only

SAMPLE OUTPUT:
  project
  ├── src
  │   ├── main.rs
  │   └── lib.rs
  ├── tests
  │   └── test.rs
  └── Cargo.toml

  2 directories, 4 files

COMMON USE CASES:
  • Visualize project structure
  • Document codebase layout
  • Explore unfamiliar directories
  • Find deeply nested files

LIMITING DEPTH:
  Large projects can be overwhelming!
  Use -L to limit depth:

  tree -L 1    Just immediate children
  tree -L 2    Two levels deep
  tree -L 3    Three levels (usually enough)

IGNORING FILES:
  Use -d to focus on structure:
  tree -d -L 2     Show folder layout only

THE SYMBOLS:
  ├──    Item with siblings below
  └──    Last item in directory
  │      Continuing line from above

RELATED COMMANDS:
  ls       List directory contents
  find     Search for files
  pwd      Current directory
"#.to_string()
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        let mut max_depth: Option<usize> = None;
        let mut show_hidden = false;
        let mut dirs_only = false;
        let mut target_path = state.cwd().to_path_buf();

        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "-L" => {
                    if i + 1 < args.len() {
                        max_depth = args[i + 1].parse().ok();
                        i += 1;
                    }
                }
                "-a" => show_hidden = true,
                "-d" => dirs_only = true,
                "-h" | "--help" => {
                    return Ok("Usage: tree [OPTIONS] [DIRECTORY]\n\
                        Options:\n  \
                        -L <n>    Descend only n levels deep\n  \
                        -a        Show hidden files\n  \
                        -d        List directories only".to_string());
                }
                _ if !args[i].starts_with('-') => {
                    target_path = state.resolve_path(&args[i]);
                }
                _ => {}
            }
            i += 1;
        }

        if !target_path.exists() {
            return Err(anyhow::anyhow!("tree: '{}' does not exist", target_path.display()));
        }

        let mut output = vec![target_path.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| ".".to_string())];

        let mut dir_count = 0;
        let mut file_count = 0;

        print_tree(
            &target_path,
            "",
            max_depth,
            0,
            show_hidden,
            dirs_only,
            &mut output,
            &mut dir_count,
            &mut file_count,
        )?;

        output.push(format!("\n{} directories, {} files", dir_count, file_count));

        Ok(output.join("\n"))
    }
}

fn print_tree(
    path: &std::path::Path,
    prefix: &str,
    max_depth: Option<usize>,
    current_depth: usize,
    show_hidden: bool,
    dirs_only: bool,
    output: &mut Vec<String>,
    dir_count: &mut usize,
    file_count: &mut usize,
) -> Result<()> {
    if let Some(max) = max_depth {
        if current_depth >= max {
            return Ok(());
        }
    }

    let mut entries: Vec<_> = fs::read_dir(path)?
        .filter_map(|e| e.ok())
        .filter(|e| {
            let name = e.file_name().to_string_lossy().to_string();
            show_hidden || !name.starts_with('.')
        })
        .filter(|e| {
            if dirs_only {
                e.file_type().map(|ft| ft.is_dir()).unwrap_or(false)
            } else {
                true
            }
        })
        .collect();

    entries.sort_by(|a, b| {
        a.file_name().to_string_lossy().to_lowercase()
            .cmp(&b.file_name().to_string_lossy().to_lowercase())
    });

    let total = entries.len();

    for (i, entry) in entries.iter().enumerate() {
        let is_last = i == total - 1;
        let connector = if is_last { "└── " } else { "├── " };
        let name = entry.file_name().to_string_lossy().to_string();
        let is_dir = entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false);

        let icon = if is_dir { " " } else { "" };

        output.push(format!("{}{}{}{}", prefix, connector, icon, name));

        if is_dir {
            *dir_count += 1;
            let new_prefix = if is_last {
                format!("{}    ", prefix)
            } else {
                format!("{}│   ", prefix)
            };
            print_tree(
                &entry.path(),
                &new_prefix,
                max_depth,
                current_depth + 1,
                show_hidden,
                dirs_only,
                output,
                dir_count,
                file_count,
            )?;
        } else {
            *file_count += 1;
        }
    }

    Ok(())
}
