//! Input handling
//!
//! Handles keyboard input and line editing.

/// Input handler for terminal
#[allow(dead_code)]
pub struct InputHandler {
    /// Current input buffer
    buffer: String,
    /// Cursor position within buffer
    cursor: usize,
}

#[allow(dead_code)]
impl InputHandler {
    /// Create a new input handler
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
            cursor: 0,
        }
    }

    /// Get current input
    pub fn text(&self) -> &str {
        &self.buffer
    }

    /// Set input text
    pub fn set_text(&mut self, text: &str) {
        self.buffer = text.to_string();
        self.cursor = self.buffer.len();
    }

    /// Clear input
    pub fn clear(&mut self) {
        self.buffer.clear();
        self.cursor = 0;
    }

    /// Insert character at cursor
    pub fn insert(&mut self, c: char) {
        self.buffer.insert(self.cursor, c);
        self.cursor += c.len_utf8();
    }

    /// Delete character before cursor (backspace)
    pub fn backspace(&mut self) {
        if self.cursor > 0 {
            let prev_char_boundary = self.buffer[..self.cursor]
                .char_indices()
                .last()
                .map(|(i, _)| i)
                .unwrap_or(0);
            self.buffer.remove(prev_char_boundary);
            self.cursor = prev_char_boundary;
        }
    }

    /// Delete character at cursor (delete)
    pub fn delete(&mut self) {
        if self.cursor < self.buffer.len() {
            self.buffer.remove(self.cursor);
        }
    }

    /// Move cursor left
    pub fn move_left(&mut self) {
        if self.cursor > 0 {
            self.cursor = self.buffer[..self.cursor]
                .char_indices()
                .last()
                .map(|(i, _)| i)
                .unwrap_or(0);
        }
    }

    /// Move cursor right
    pub fn move_right(&mut self) {
        if self.cursor < self.buffer.len() {
            self.cursor = self.buffer[self.cursor..]
                .char_indices()
                .nth(1)
                .map(|(i, _)| self.cursor + i)
                .unwrap_or(self.buffer.len());
        }
    }

    /// Move cursor to start
    pub fn home(&mut self) {
        self.cursor = 0;
    }

    /// Move cursor to end
    pub fn end(&mut self) {
        self.cursor = self.buffer.len();
    }

    /// Get cursor position
    pub fn cursor(&self) -> usize {
        self.cursor
    }

    /// Take the current input and clear
    pub fn take(&mut self) -> String {
        self.cursor = 0;
        std::mem::take(&mut self.buffer)
    }
}

impl Default for InputHandler {
    fn default() -> Self {
        Self::new()
    }
}
