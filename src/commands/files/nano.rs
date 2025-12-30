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
    /// Scroll offset (first visible line)
    pub scroll_offset: usize,
    /// Visible lines (set by renderer)
    pub visible_lines: usize,
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
            status: String::new(),
            scroll_offset: 0,
            visible_lines: 30,
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

    /// Ensure cursor is visible by adjusting scroll offset
    pub fn ensure_cursor_visible(&mut self) {
        if self.cursor_line < self.scroll_offset {
            self.scroll_offset = self.cursor_line;
        } else if self.cursor_line >= self.scroll_offset + self.visible_lines {
            self.scroll_offset = self.cursor_line.saturating_sub(self.visible_lines - 1);
        }
    }

    /// Move cursor up
    pub fn cursor_up(&mut self) {
        if self.cursor_line > 0 {
            self.cursor_line -= 1;
            let lines: Vec<&str> = self.content.lines().collect();
            if let Some(line) = lines.get(self.cursor_line) {
                self.cursor_col = self.cursor_col.min(line.len());
            }
            self.ensure_cursor_visible();
        }
    }

    /// Move cursor down
    pub fn cursor_down(&mut self) {
        let line_count = self.content.lines().count();
        if self.cursor_line < line_count.saturating_sub(1) {
            self.cursor_line += 1;
            let lines: Vec<&str> = self.content.lines().collect();
            if let Some(line) = lines.get(self.cursor_line) {
                self.cursor_col = self.cursor_col.min(line.len());
            }
            self.ensure_cursor_visible();
        }
    }

    /// Page up
    pub fn page_up(&mut self) {
        let jump = self.visible_lines.saturating_sub(2);
        self.cursor_line = self.cursor_line.saturating_sub(jump);
        self.scroll_offset = self.scroll_offset.saturating_sub(jump);
        let lines: Vec<&str> = self.content.lines().collect();
        if let Some(line) = lines.get(self.cursor_line) {
            self.cursor_col = self.cursor_col.min(line.len());
        }
    }

    /// Page down
    pub fn page_down(&mut self) {
        let line_count = self.content.lines().count();
        let jump = self.visible_lines.saturating_sub(2);
        self.cursor_line = (self.cursor_line + jump).min(line_count.saturating_sub(1));
        self.scroll_offset =
            (self.scroll_offset + jump).min(line_count.saturating_sub(self.visible_lines));
        let lines: Vec<&str> = self.content.lines().collect();
        if let Some(line) = lines.get(self.cursor_line) {
            self.cursor_col = self.cursor_col.min(line.len());
        }
    }

    /// Go to start of file
    pub fn go_to_start(&mut self) {
        self.cursor_line = 0;
        self.cursor_col = 0;
        self.scroll_offset = 0;
    }

    /// Go to end of file
    pub fn go_to_end(&mut self) {
        let lines: Vec<&str> = self.content.lines().collect();
        self.cursor_line = lines.len().saturating_sub(1);
        self.cursor_col = lines.last().map(|l| l.len()).unwrap_or(0);
        self.ensure_cursor_visible();
    }

    /// Go to start of line
    pub fn go_to_line_start(&mut self) {
        self.cursor_col = 0;
    }

    /// Go to end of line
    pub fn go_to_line_end(&mut self) {
        let lines: Vec<&str> = self.content.lines().collect();
        if let Some(line) = lines.get(self.cursor_line) {
            self.cursor_col = line.len();
        }
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
