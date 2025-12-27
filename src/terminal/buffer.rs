//! Output buffer with scrollback
//!
//! Stores terminal output lines with a configurable scrollback limit.
//! Supports block-based output grouping and URL detection.

#![allow(dead_code)]

use std::collections::VecDeque;
use regex::Regex;
use once_cell::sync::Lazy;

/// Regex for detecting URLs in output
static URL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"https?://[^\s<>"{}|\\^\[\]`]+"#).unwrap()
});

/// Output buffer for terminal display
pub struct OutputBuffer {
    /// Lines of output
    lines: VecDeque<OutputLine>,
    /// Maximum number of lines to keep
    max_lines: usize,
    /// Command blocks (for block-based navigation)
    blocks: Vec<CommandBlock>,
    /// Currently selected block index (for block navigation)
    selected_block: Option<usize>,
    /// Search query for filtering
    search_query: Option<String>,
}

/// A command block groups a command with its output
#[derive(Clone)]
pub struct CommandBlock {
    /// Block ID
    pub id: usize,
    /// The command that was executed
    pub command: String,
    /// Starting line index
    pub start_line: usize,
    /// Ending line index (exclusive)
    pub end_line: usize,
    /// Whether the command succeeded
    pub success: bool,
    /// Timestamp when the command was executed
    pub timestamp: std::time::Instant,
    /// Duration of command execution (set when block ends)
    pub duration: Option<std::time::Duration>,
}

/// A single line of output
#[derive(Clone)]
pub struct OutputLine {
    /// The text content
    pub text: String,
    /// Line type for styling
    pub line_type: LineType,
    /// Block ID this line belongs to (if any)
    pub block_id: Option<usize>,
    /// Detected URLs in this line
    pub urls: Vec<UrlSpan>,
}

/// A URL span in a line
#[derive(Clone)]
pub struct UrlSpan {
    /// Start character index
    pub start: usize,
    /// End character index
    pub end: usize,
    /// The URL text
    pub url: String,
}

/// Type of output line (for styling)
#[derive(Clone, Copy, PartialEq)]
pub enum LineType {
    /// Normal output
    Normal,
    /// Error message
    Error,
    /// Command echo (prompt + command)
    Command,
    /// Success message
    Success,
}

impl OutputBuffer {
    /// Create a new output buffer with specified max lines
    pub fn new(max_lines: usize) -> Self {
        Self {
            lines: VecDeque::with_capacity(max_lines),
            max_lines,
            blocks: Vec::new(),
            selected_block: None,
            search_query: None,
        }
    }

    /// Detect URLs in a text string
    fn detect_urls(text: &str) -> Vec<UrlSpan> {
        URL_REGEX.find_iter(text)
            .map(|m| UrlSpan {
                start: m.start(),
                end: m.end(),
                url: m.as_str().to_string(),
            })
            .collect()
    }

    /// Push a normal line
    pub fn push_line(&mut self, text: &str) {
        let urls = Self::detect_urls(text);
        let current_block_id = self.blocks.last().map(|b| b.id);
        self.push(OutputLine {
            text: text.to_string(),
            line_type: LineType::Normal,
            block_id: current_block_id,
            urls,
        });
    }

    /// Push an error line
    pub fn push_error(&mut self, text: &str) {
        let urls = Self::detect_urls(text);
        let current_block_id = self.blocks.last().map(|b| b.id);
        self.push(OutputLine {
            text: text.to_string(),
            line_type: LineType::Error,
            block_id: current_block_id,
            urls,
        });
    }

    /// Push a success line
    pub fn push_success(&mut self, text: &str) {
        let urls = Self::detect_urls(text);
        let current_block_id = self.blocks.last().map(|b| b.id);
        self.push(OutputLine {
            text: text.to_string(),
            line_type: LineType::Success,
            block_id: current_block_id,
            urls,
        });
    }

    /// Start a new command block
    pub fn start_block(&mut self, command: &str) {
        let block_id = self.blocks.len();
        let start_line = self.lines.len();
        self.blocks.push(CommandBlock {
            id: block_id,
            command: command.to_string(),
            start_line,
            end_line: start_line,
            success: true,
            timestamp: std::time::Instant::now(),
            duration: None,
        });
    }

    /// End the current command block
    pub fn end_block(&mut self, success: bool) {
        if let Some(block) = self.blocks.last_mut() {
            block.end_line = self.lines.len();
            block.success = success;
            block.duration = Some(block.timestamp.elapsed());
        }
    }

    /// Get the last block's duration formatted as a string
    pub fn last_block_duration(&self) -> Option<String> {
        self.blocks.last().and_then(|b| b.duration).map(|d| format_duration(d))
    }

    /// Get all command blocks
    pub fn blocks(&self) -> &[CommandBlock] {
        &self.blocks
    }

    /// Get selected block index
    pub fn selected_block(&self) -> Option<usize> {
        self.selected_block
    }

    /// Select next block
    pub fn select_next_block(&mut self) {
        if self.blocks.is_empty() {
            return;
        }
        self.selected_block = Some(match self.selected_block {
            Some(idx) if idx + 1 < self.blocks.len() => idx + 1,
            Some(_) => self.blocks.len() - 1,
            None => self.blocks.len() - 1,
        });
    }

    /// Select previous block
    pub fn select_prev_block(&mut self) {
        if self.blocks.is_empty() {
            return;
        }
        self.selected_block = Some(match self.selected_block {
            Some(idx) if idx > 0 => idx - 1,
            Some(_) => 0,
            None => 0,
        });
    }

    /// Clear block selection
    pub fn clear_block_selection(&mut self) {
        self.selected_block = None;
    }

    /// Set search query for filtering
    pub fn set_search(&mut self, query: Option<String>) {
        self.search_query = query;
    }

    /// Get current search query
    pub fn search_query(&self) -> Option<&str> {
        self.search_query.as_deref()
    }

    /// Search within buffer and return matching line indices
    pub fn search(&self, query: &str) -> Vec<usize> {
        let query_lower = query.to_lowercase();
        self.lines.iter()
            .enumerate()
            .filter(|(_, line)| line.text.to_lowercase().contains(&query_lower))
            .map(|(idx, _)| idx)
            .collect()
    }

    /// Push a line
    fn push(&mut self, line: OutputLine) {
        if self.lines.len() >= self.max_lines {
            self.lines.pop_front();
            // Adjust block start/end indices
            for block in &mut self.blocks {
                if block.start_line > 0 {
                    block.start_line -= 1;
                }
                if block.end_line > 0 {
                    block.end_line -= 1;
                }
            }
            // Remove blocks that are now entirely out of bounds
            self.blocks.retain(|b| b.end_line > 0);
        }
        self.lines.push_back(line);
        // Update current block's end line
        if let Some(block) = self.blocks.last_mut() {
            block.end_line = self.lines.len();
        }
    }

    /// Clear all lines
    pub fn clear(&mut self) {
        self.lines.clear();
        self.blocks.clear();
        self.selected_block = None;
        self.search_query = None;
    }

    /// Get all lines as strings (for simple rendering)
    pub fn lines(&self) -> impl Iterator<Item = &str> {
        self.lines.iter().map(|l| l.text.as_str())
    }

    /// Get all output lines with metadata
    pub fn output_lines(&self) -> impl Iterator<Item = &OutputLine> {
        self.lines.iter()
    }

    /// Get number of lines
    pub fn len(&self) -> usize {
        self.lines.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    /// Get a specific line by index
    pub fn get_line(&self, idx: usize) -> Option<&OutputLine> {
        self.lines.get(idx)
    }

    /// Get block by ID
    pub fn get_block(&self, block_id: usize) -> Option<&CommandBlock> {
        self.blocks.get(block_id)
    }

    /// Get all text content of a command block (for copying)
    pub fn get_block_content(&self, block_id: usize) -> Option<String> {
        let block = self.blocks.get(block_id)?;
        let mut content = String::new();

        // Get lines that belong to this block (skip the command line itself)
        for i in (block.start_line + 1)..block.end_line.min(self.lines.len()) {
            if let Some(line) = self.lines.get(i) {
                content.push_str(&line.text);
                content.push('\n');
            }
        }

        // Remove trailing newline
        if content.ends_with('\n') {
            content.pop();
        }

        Some(content)
    }

    /// Get command from a block
    pub fn get_block_command(&self, block_id: usize) -> Option<&str> {
        self.blocks.get(block_id).map(|b| b.command.as_str())
    }
}

/// Format a duration for display
pub fn format_duration(d: std::time::Duration) -> String {
    let secs = d.as_secs();
    let millis = d.as_millis();

    if secs >= 60 {
        let mins = secs / 60;
        let secs = secs % 60;
        format!("{}m {}s", mins, secs)
    } else if secs >= 1 {
        format!("{}.{:02}s", secs, (millis % 1000) / 10)
    } else if millis >= 1 {
        format!("{}ms", millis)
    } else {
        format!("{}µs", d.as_micros())
    }
}
