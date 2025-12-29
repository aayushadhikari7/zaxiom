//! PTY output buffer
//!
//! Handles streaming output from PTY with ANSI escape sequence processing.

use std::collections::VecDeque;

/// A line in the PTY buffer
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct PtyLine {
    /// The text content (may contain ANSI codes)
    pub text: String,
    /// Whether this is an error/stderr line
    pub is_error: bool,
}

/// Buffer for PTY output with streaming support
#[allow(dead_code)]
pub struct PtyBuffer {
    /// Lines of output
    lines: VecDeque<PtyLine>,
    /// Current incomplete line being built
    current_line: String,
    /// Maximum number of lines (scrollback limit)
    max_lines: usize,
    /// Current cursor column (for cursor movement within line)
    cursor_col: usize,
    /// Whether alternate screen mode is active
    alternate_screen: bool,
    /// Saved primary screen content (when in alternate screen)
    saved_lines: Option<VecDeque<PtyLine>>,
    /// Terminal dimensions
    rows: usize,
    cols: usize,
}

#[allow(dead_code)]
impl PtyBuffer {
    /// Create a new PTY buffer
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            lines: VecDeque::new(),
            current_line: String::new(),
            max_lines: 10_000,
            cursor_col: 0,
            alternate_screen: false,
            saved_lines: None,
            rows,
            cols,
        }
    }

    /// Process raw output bytes from PTY
    pub fn process_output(&mut self, data: &[u8]) {
        // Convert to UTF-8 (lossy for invalid sequences)
        let text = String::from_utf8_lossy(data);

        for ch in text.chars() {
            match ch {
                '\n' => {
                    // Newline: push current line and start new one
                    self.push_current_line();
                }
                '\r' => {
                    // Carriage return: move cursor to start of line
                    self.cursor_col = 0;
                }
                '\x08' => {
                    // Backspace: move cursor left
                    if self.cursor_col > 0 {
                        self.cursor_col -= 1;
                    }
                }
                '\x1b' => {
                    // ESC - start of escape sequence, just append for now
                    // The ANSI parser will handle it during rendering
                    self.append_char(ch);
                }
                '\t' => {
                    // Tab: expand to spaces (8-column tabs)
                    let spaces = 8 - (self.cursor_col % 8);
                    for _ in 0..spaces {
                        self.append_char(' ');
                    }
                }
                '\x07' => {
                    // Bell: ignore for now
                }
                _ if ch.is_control() => {
                    // Ignore other control characters
                }
                _ => {
                    // Regular character
                    self.append_char(ch);
                }
            }
        }
    }

    /// Append a character at the current cursor position
    fn append_char(&mut self, ch: char) {
        // Ensure the line is long enough
        while self.current_line.chars().count() < self.cursor_col {
            self.current_line.push(' ');
        }

        // If we're at the end, just append
        if self.cursor_col >= self.current_line.chars().count() {
            self.current_line.push(ch);
        } else {
            // Replace character at cursor position
            let mut chars: Vec<char> = self.current_line.chars().collect();
            if self.cursor_col < chars.len() {
                chars[self.cursor_col] = ch;
            } else {
                chars.push(ch);
            }
            self.current_line = chars.into_iter().collect();
        }

        self.cursor_col += 1;
    }

    /// Push the current line to the buffer
    fn push_current_line(&mut self) {
        let line = PtyLine {
            text: std::mem::take(&mut self.current_line),
            is_error: false,
        };
        self.lines.push_back(line);
        self.cursor_col = 0;

        // Enforce scrollback limit
        while self.lines.len() > self.max_lines {
            self.lines.pop_front();
        }
    }

    /// Push a line directly (for native command output)
    pub fn push_line(&mut self, text: &str) {
        // First, flush any partial line
        if !self.current_line.is_empty() {
            self.push_current_line();
        }

        let line = PtyLine {
            text: text.to_string(),
            is_error: false,
        };
        self.lines.push_back(line);

        while self.lines.len() > self.max_lines {
            self.lines.pop_front();
        }
    }

    /// Push an error line
    pub fn push_error(&mut self, text: &str) {
        // First, flush any partial line
        if !self.current_line.is_empty() {
            self.push_current_line();
        }

        let line = PtyLine {
            text: text.to_string(),
            is_error: true,
        };
        self.lines.push_back(line);

        while self.lines.len() > self.max_lines {
            self.lines.pop_front();
        }
    }

    /// Get all lines for rendering
    pub fn lines(&self) -> impl Iterator<Item = &PtyLine> {
        self.lines.iter()
    }

    /// Get the current incomplete line (for rendering with cursor)
    pub fn current_line(&self) -> &str {
        &self.current_line
    }

    /// Get cursor column position
    pub fn cursor_col(&self) -> usize {
        self.cursor_col
    }

    /// Clear the buffer
    pub fn clear(&mut self) {
        self.lines.clear();
        self.current_line.clear();
        self.cursor_col = 0;
    }

    /// Enter alternate screen mode (for vim, less, etc.)
    pub fn enter_alternate_screen(&mut self) {
        if !self.alternate_screen {
            self.alternate_screen = true;
            // Save current screen content
            self.saved_lines = Some(std::mem::take(&mut self.lines));
            self.current_line.clear();
            self.cursor_col = 0;
        }
    }

    /// Exit alternate screen mode
    pub fn exit_alternate_screen(&mut self) {
        if self.alternate_screen {
            self.alternate_screen = false;
            // Restore saved screen content
            if let Some(saved) = self.saved_lines.take() {
                self.lines = saved;
            }
            self.current_line.clear();
            self.cursor_col = 0;
        }
    }

    /// Check if alternate screen is active
    pub fn is_alternate_screen(&self) -> bool {
        self.alternate_screen
    }

    /// Resize the buffer dimensions
    pub fn resize(&mut self, rows: usize, cols: usize) {
        self.rows = rows;
        self.cols = cols;
    }

    /// Get the number of lines
    pub fn len(&self) -> usize {
        self.lines.len() + if self.current_line.is_empty() { 0 } else { 1 }
    }

    /// Check if buffer is empty
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty() && self.current_line.is_empty()
    }
}

impl Default for PtyBuffer {
    fn default() -> Self {
        Self::new(24, 80)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_simple_output() {
        let mut buffer = PtyBuffer::new(24, 80);
        buffer.process_output(b"Hello, World!\n");
        assert_eq!(buffer.len(), 1);
        assert_eq!(buffer.lines().next().unwrap().text, "Hello, World!");
    }

    #[test]
    fn test_process_multiline() {
        let mut buffer = PtyBuffer::new(24, 80);
        buffer.process_output(b"Line 1\nLine 2\nLine 3\n");
        assert_eq!(buffer.len(), 3);
    }

    #[test]
    fn test_carriage_return() {
        let mut buffer = PtyBuffer::new(24, 80);
        buffer.process_output(b"XXXX\rHello\n");
        assert_eq!(buffer.lines().next().unwrap().text, "Hello");
    }
}
