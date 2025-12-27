//! nano command - simple text editor
//!
//! Opens files for editing in a modal text editor interface.

#![allow(dead_code)]

use std::fs;
use std::path::PathBuf;

use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

/// Editor state for nano command
#[derive(Clone, Debug)]
pub struct EditorState {
    /// File being edited
    pub file_path: PathBuf,
    /// File content
    pub content: String,
    /// Whether the content has been modified
    pub modified: bool,
    /// Cursor line position
    pub cursor_line: usize,
    /// Cursor column position
    pub cursor_col: usize,
    /// Whether the editor is active
    pub active: bool,
    /// Status message
    pub status: String,
}

impl EditorState {
    /// Create a new editor state for a file
    pub fn new(path: PathBuf) -> Result<Self> {
        let content = if path.exists() {
            fs::read_to_string(&path)?
        } else {
            String::new()
        };

        Ok(Self {
            file_path: path,
            content,
            modified: false,
            cursor_line: 0,
            cursor_col: 0,
            active: true,
            status: "^X Exit  ^S Save  ^G Help".to_string(),
        })
    }

    /// Save the file
    pub fn save(&mut self) -> Result<()> {
        fs::write(&self.file_path, &self.content)?;
        self.modified = false;
        self.status = format!("Saved: {}", self.file_path.display());
        Ok(())
    }

    /// Get line count
    pub fn line_count(&self) -> usize {
        self.content.lines().count().max(1)
    }

    /// Insert text at cursor
    pub fn insert(&mut self, text: &str) {
        self.content.push_str(text);
        self.modified = true;
    }
}

pub struct NanoCommand;

impl Command for NanoCommand {
    fn name(&self) -> &'static str {
        "nano"
    }

    fn description(&self) -> &'static str {
        "Simple text editor"
    }

    fn usage(&self) -> &'static str {
        "nano <file>"
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        if args.is_empty() {
            return Err(anyhow::anyhow!("Usage: nano <file>"));
        }

        let path = state.resolve_path(&args[0]);

        // Return a special marker that the app will handle to open the editor
        // Format: \x1b[EDIT]filepath
        Ok(format!("\x1b[EDIT]{}", path.display()))
    }
}

/// vim command - alias to nano for basic editing
pub struct VimCommand;

impl Command for VimCommand {
    fn name(&self) -> &'static str {
        "vim"
    }

    fn description(&self) -> &'static str {
        "Text editor (opens nano)"
    }

    fn usage(&self) -> &'static str {
        "vim <file>"
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        if args.is_empty() {
            return Err(anyhow::anyhow!("Usage: vim <file>"));
        }

        let path = state.resolve_path(&args[0]);
        Ok(format!("\x1b[EDIT]{}", path.display()))
    }
}

/// vi command - alias to nano
pub struct ViCommand;

impl Command for ViCommand {
    fn name(&self) -> &'static str {
        "vi"
    }

    fn description(&self) -> &'static str {
        "Text editor (opens nano)"
    }

    fn usage(&self) -> &'static str {
        "vi <file>"
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        if args.is_empty() {
            return Err(anyhow::anyhow!("Usage: vi <file>"));
        }

        let path = state.resolve_path(&args[0]);
        Ok(format!("\x1b[EDIT]{}", path.display()))
    }
}

/// edit command - Windows-style alias to nano
pub struct EditCommand;

impl Command for EditCommand {
    fn name(&self) -> &'static str {
        "edit"
    }

    fn description(&self) -> &'static str {
        "Text editor (opens nano)"
    }

    fn usage(&self) -> &'static str {
        "edit <file>"
    }

    fn execute(&self, args: &[String], state: &mut TerminalState) -> Result<String> {
        if args.is_empty() {
            return Err(anyhow::anyhow!("Usage: edit <file>"));
        }

        let path = state.resolve_path(&args[0]);
        Ok(format!("\x1b[EDIT]{}", path.display()))
    }
}
