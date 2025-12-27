//! Command history
//!
//! Stores and navigates through previously executed commands.

#![allow(dead_code)]

use std::collections::VecDeque;

/// Command history manager
pub struct CommandHistory {
    /// List of commands (oldest first)
    commands: VecDeque<String>,
    /// Maximum number of commands to keep
    max_commands: usize,
    /// Current position when navigating (None = at end, typing new command)
    position: Option<usize>,
}

impl CommandHistory {
    /// Create a new command history with specified capacity
    pub fn new(max_commands: usize) -> Self {
        Self {
            commands: VecDeque::with_capacity(max_commands),
            max_commands,
            position: None,
        }
    }

    /// Add a command to history
    pub fn add(&mut self, command: &str) {
        let command = command.trim();
        if command.is_empty() {
            return;
        }

        // Don't add duplicates of the last command
        if self.commands.back().map(|s| s.as_str()) == Some(command) {
            self.position = None;
            return;
        }

        // Remove oldest if at capacity
        if self.commands.len() >= self.max_commands {
            self.commands.pop_front();
        }

        self.commands.push_back(command.to_string());
        self.position = None;
    }

    /// Get previous command (up arrow)
    pub fn previous(&mut self) -> Option<&str> {
        if self.commands.is_empty() {
            return None;
        }

        let new_pos = match self.position {
            None => self.commands.len().saturating_sub(1),
            Some(0) => 0,
            Some(pos) => pos - 1,
        };

        self.position = Some(new_pos);
        self.commands.get(new_pos).map(|s| s.as_str())
    }

    /// Get next command (down arrow)
    pub fn next(&mut self) -> Option<&str> {
        match self.position {
            None => None,
            Some(pos) => {
                let new_pos = pos + 1;
                if new_pos >= self.commands.len() {
                    self.position = None;
                    None
                } else {
                    self.position = Some(new_pos);
                    self.commands.get(new_pos).map(|s| s.as_str())
                }
            }
        }
    }

    /// Reset navigation position
    pub fn reset_position(&mut self) {
        self.position = None;
    }

    /// Get all commands
    pub fn all(&self) -> impl Iterator<Item = &str> {
        self.commands.iter().map(|s| s.as_str())
    }

    /// Get number of commands in history
    pub fn len(&self) -> usize {
        self.commands.len()
    }

    /// Check if history is empty
    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }
}
