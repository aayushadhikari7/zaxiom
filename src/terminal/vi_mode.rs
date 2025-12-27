//! Vi Mode
//!
//! Vim-style navigation for terminal output, inspired by Alacritty's vi mode.
//! Allows navigating, selecting, and searching within terminal scrollback.

#![allow(dead_code)]

/// Vi mode state
#[derive(Clone, Debug, PartialEq)]
pub enum ViState {
    /// Normal mode - navigation
    Normal,
    /// Visual mode - selection
    Visual,
    /// Visual Line mode - line selection
    VisualLine,
    /// Visual Block mode - block selection
    VisualBlock,
    /// Search mode (forward)
    SearchForward,
    /// Search mode (backward)
    SearchBackward,
}

/// Vi mode cursor position
#[derive(Clone, Copy, Debug, Default)]
pub struct ViCursor {
    /// Line in scrollback (0 = bottom)
    pub line: usize,
    /// Column position
    pub col: usize,
}

/// Visual selection
#[derive(Clone, Debug)]
pub struct ViSelection {
    /// Start of selection
    pub start: ViCursor,
    /// End of selection (current cursor)
    pub end: ViCursor,
    /// Type of selection
    pub mode: ViState,
}

/// Vi Mode controller
pub struct ViMode {
    /// Whether vi mode is active
    pub active: bool,
    /// Current state
    pub state: ViState,
    /// Cursor position
    pub cursor: ViCursor,
    /// Visual selection (if any)
    pub selection: Option<ViSelection>,
    /// Search query
    pub search_query: String,
    /// Search matches (line, start_col, end_col)
    pub search_matches: Vec<(usize, usize, usize)>,
    /// Current match index
    pub current_match: usize,
    /// Pending operator (d, y, c)
    pub pending_op: Option<char>,
    /// Count prefix (e.g., 5j = move 5 lines)
    pub count: Option<usize>,
    /// Last search direction (true = forward)
    pub search_forward: bool,
    /// Yank register (clipboard)
    pub yank_register: String,
    /// Marks (a-z, A-Z)
    pub marks: std::collections::HashMap<char, ViCursor>,
    /// Jump list for Ctrl-O/Ctrl-I
    pub jump_list: Vec<ViCursor>,
    /// Current position in jump list
    pub jump_index: usize,
    /// Total lines in buffer (for bounds checking)
    pub total_lines: usize,
    /// Max column for current line
    pub max_col: usize,
}

impl Default for ViMode {
    fn default() -> Self {
        Self {
            active: false,
            state: ViState::Normal,
            cursor: ViCursor::default(),
            selection: None,
            search_query: String::new(),
            search_matches: Vec::new(),
            current_match: 0,
            pending_op: None,
            count: None,
            search_forward: true,
            yank_register: String::new(),
            marks: std::collections::HashMap::new(),
            jump_list: Vec::new(),
            jump_index: 0,
            total_lines: 0,
            max_col: 0,
        }
    }
}

impl ViMode {
    /// Create new vi mode
    pub fn new() -> Self {
        Self::default()
    }

    /// Enter vi mode
    pub fn enter(&mut self, total_lines: usize) {
        self.active = true;
        self.state = ViState::Normal;
        self.total_lines = total_lines;
        self.cursor = ViCursor {
            line: 0, // Start at bottom
            col: 0,
        };
        self.selection = None;
        self.search_query.clear();
        self.search_matches.clear();
        self.pending_op = None;
        self.count = None;
    }

    /// Exit vi mode
    pub fn exit(&mut self) {
        self.active = false;
        self.state = ViState::Normal;
        self.selection = None;
        self.pending_op = None;
        self.count = None;
    }

    /// Get effective count (default 1)
    fn get_count(&mut self) -> usize {
        self.count.take().unwrap_or(1)
    }

    /// Handle a key press, returns action to perform
    pub fn handle_key(&mut self, key: char) -> ViAction {
        // Handle digit input for count
        if key.is_ascii_digit() && key != '0' {
            let digit = key.to_digit(10).unwrap() as usize;
            self.count = Some(self.count.unwrap_or(0) * 10 + digit);
            return ViAction::None;
        }

        match self.state {
            ViState::Normal => self.handle_normal_key(key),
            ViState::Visual | ViState::VisualLine | ViState::VisualBlock => {
                self.handle_visual_key(key)
            }
            ViState::SearchForward | ViState::SearchBackward => self.handle_search_key(key),
        }
    }

    /// Handle key in normal mode
    fn handle_normal_key(&mut self, key: char) -> ViAction {
        let count = self.get_count();

        match key {
            // Exit vi mode
            'q' | '\x1b' => {
                self.exit();
                ViAction::Exit
            }

            // Motion: left
            'h' => {
                for _ in 0..count {
                    if self.cursor.col > 0 {
                        self.cursor.col -= 1;
                    }
                }
                ViAction::MoveCursor
            }

            // Motion: down
            'j' => {
                for _ in 0..count {
                    if self.cursor.line + 1 < self.total_lines {
                        self.cursor.line += 1;
                    }
                }
                ViAction::MoveCursor
            }

            // Motion: up
            'k' => {
                for _ in 0..count {
                    if self.cursor.line > 0 {
                        self.cursor.line -= 1;
                    }
                }
                ViAction::MoveCursor
            }

            // Motion: right
            'l' => {
                for _ in 0..count {
                    if self.cursor.col < self.max_col {
                        self.cursor.col += 1;
                    }
                }
                ViAction::MoveCursor
            }

            // Motion: word forward
            'w' => {
                for _ in 0..count {
                    // Simplified: just move 5 chars
                    self.cursor.col = (self.cursor.col + 5).min(self.max_col);
                }
                ViAction::MoveCursor
            }

            // Motion: word backward
            'b' => {
                for _ in 0..count {
                    self.cursor.col = self.cursor.col.saturating_sub(5);
                }
                ViAction::MoveCursor
            }

            // Motion: end of word
            'e' => {
                for _ in 0..count {
                    self.cursor.col = (self.cursor.col + 5).min(self.max_col);
                }
                ViAction::MoveCursor
            }

            // Motion: start of line
            '0' => {
                self.cursor.col = 0;
                ViAction::MoveCursor
            }

            // Motion: first non-blank
            '^' => {
                self.cursor.col = 0; // Simplified
                ViAction::MoveCursor
            }

            // Motion: end of line
            '$' => {
                self.cursor.col = self.max_col;
                ViAction::MoveCursor
            }

            // Motion: top of screen
            'H' => {
                self.cursor.line = self.total_lines.saturating_sub(1);
                ViAction::MoveCursor
            }

            // Motion: middle of screen
            'M' => {
                self.cursor.line = self.total_lines / 2;
                ViAction::MoveCursor
            }

            // Motion: bottom of screen
            'L' => {
                self.cursor.line = 0;
                ViAction::MoveCursor
            }

            // Motion: go to top
            'g' => {
                if self.pending_op == Some('g') {
                    self.pending_op = None;
                    self.cursor.line = self.total_lines.saturating_sub(count);
                    self.cursor.col = 0;
                    ViAction::MoveCursor
                } else {
                    self.pending_op = Some('g');
                    ViAction::None
                }
            }

            // Motion: go to bottom/line number
            'G' => {
                if let Some(n) = self.count.take() {
                    // Go to specific line
                    self.cursor.line = self.total_lines.saturating_sub(n);
                } else {
                    // Go to bottom
                    self.cursor.line = 0;
                }
                self.cursor.col = 0;
                ViAction::MoveCursor
            }

            // Page down
            '\x06' => {
                // Ctrl+F
                self.cursor.line = self.cursor.line.saturating_sub(20);
                ViAction::ScrollDown(20)
            }

            // Page up
            '\x02' => {
                // Ctrl+B
                self.cursor.line = (self.cursor.line + 20).min(self.total_lines.saturating_sub(1));
                ViAction::ScrollUp(20)
            }

            // Half page down
            '\x04' => {
                // Ctrl+D
                self.cursor.line = self.cursor.line.saturating_sub(10);
                ViAction::ScrollDown(10)
            }

            // Half page up
            '\x15' => {
                // Ctrl+U
                self.cursor.line = (self.cursor.line + 10).min(self.total_lines.saturating_sub(1));
                ViAction::ScrollUp(10)
            }

            // Visual mode
            'v' => {
                self.state = ViState::Visual;
                self.selection = Some(ViSelection {
                    start: self.cursor,
                    end: self.cursor,
                    mode: ViState::Visual,
                });
                ViAction::StartSelection
            }

            // Visual line mode
            'V' => {
                self.state = ViState::VisualLine;
                self.selection = Some(ViSelection {
                    start: self.cursor,
                    end: self.cursor,
                    mode: ViState::VisualLine,
                });
                ViAction::StartSelection
            }

            // Visual block mode
            '\x16' => {
                // Ctrl+V
                self.state = ViState::VisualBlock;
                self.selection = Some(ViSelection {
                    start: self.cursor,
                    end: self.cursor,
                    mode: ViState::VisualBlock,
                });
                ViAction::StartSelection
            }

            // Search forward
            '/' => {
                self.state = ViState::SearchForward;
                self.search_forward = true;
                self.search_query.clear();
                ViAction::StartSearch
            }

            // Search backward
            '?' => {
                self.state = ViState::SearchBackward;
                self.search_forward = false;
                self.search_query.clear();
                ViAction::StartSearch
            }

            // Next search match
            'n' => {
                if !self.search_matches.is_empty() {
                    if self.search_forward {
                        self.current_match = (self.current_match + 1) % self.search_matches.len();
                    } else {
                        self.current_match = self
                            .current_match
                            .checked_sub(1)
                            .unwrap_or(self.search_matches.len() - 1);
                    }
                    if let Some(&(line, col, _)) = self.search_matches.get(self.current_match) {
                        self.cursor.line = line;
                        self.cursor.col = col;
                    }
                }
                ViAction::JumpToMatch
            }

            // Previous search match
            'N' => {
                if !self.search_matches.is_empty() {
                    if self.search_forward {
                        self.current_match = self
                            .current_match
                            .checked_sub(1)
                            .unwrap_or(self.search_matches.len() - 1);
                    } else {
                        self.current_match = (self.current_match + 1) % self.search_matches.len();
                    }
                    if let Some(&(line, col, _)) = self.search_matches.get(self.current_match) {
                        self.cursor.line = line;
                        self.cursor.col = col;
                    }
                }
                ViAction::JumpToMatch
            }

            // Yank (copy) line
            'y' => {
                if self.pending_op == Some('y') {
                    self.pending_op = None;
                    ViAction::YankLine
                } else {
                    self.pending_op = Some('y');
                    ViAction::None
                }
            }

            // Yank to end of line
            'Y' => ViAction::YankToEnd,

            // Set mark
            'm' => ViAction::SetMark,

            // Go to mark
            '\'' | '`' => ViAction::GoToMark,

            // Open URL/path under cursor
            'o' => ViAction::OpenUnderCursor,

            // Copy to clipboard
            'p' => ViAction::Paste,

            _ => ViAction::None,
        }
    }

    /// Handle key in visual mode
    fn handle_visual_key(&mut self, key: char) -> ViAction {
        match key {
            // Exit visual mode
            '\x1b' | 'v' | 'V' => {
                self.state = ViState::Normal;
                self.selection = None;
                ViAction::CancelSelection
            }

            // Yank selection
            'y' => {
                let action = ViAction::YankSelection;
                self.state = ViState::Normal;
                self.selection = None;
                action
            }

            // Switch visual modes
            '\x16' => {
                // Ctrl+V - toggle block mode
                if self.state == ViState::VisualBlock {
                    self.state = ViState::Visual;
                } else {
                    self.state = ViState::VisualBlock;
                }
                if let Some(ref mut sel) = self.selection {
                    sel.mode = self.state.clone();
                }
                ViAction::UpdateSelection
            }

            // Navigation keys update selection
            'h' | 'j' | 'k' | 'l' | 'w' | 'b' | 'e' | '0' | '$' | 'G' | 'g' => {
                // First do the movement
                let action = self.handle_normal_key(key);
                // Update selection end
                if let Some(ref mut sel) = self.selection {
                    sel.end = self.cursor;
                }
                action
            }

            _ => ViAction::None,
        }
    }

    /// Handle key in search mode
    fn handle_search_key(&mut self, key: char) -> ViAction {
        match key {
            // Cancel search
            '\x1b' => {
                self.state = ViState::Normal;
                self.search_query.clear();
                ViAction::CancelSearch
            }

            // Execute search
            '\r' | '\n' => {
                self.state = ViState::Normal;
                ViAction::ExecuteSearch(self.search_query.clone())
            }

            // Backspace
            '\x7f' | '\x08' => {
                self.search_query.pop();
                ViAction::UpdateSearch(self.search_query.clone())
            }

            // Add character to search
            c if c.is_ascii_graphic() || c == ' ' => {
                self.search_query.push(c);
                ViAction::UpdateSearch(self.search_query.clone())
            }

            _ => ViAction::None,
        }
    }

    /// Set mark at current position
    pub fn set_mark(&mut self, mark: char) {
        if mark.is_ascii_alphabetic() {
            self.marks.insert(mark, self.cursor);
        }
    }

    /// Go to mark
    pub fn go_to_mark(&mut self, mark: char) -> bool {
        if let Some(&pos) = self.marks.get(&mark) {
            // Add current position to jump list
            self.jump_list.push(self.cursor);
            self.jump_index = self.jump_list.len();
            self.cursor = pos;
            true
        } else {
            false
        }
    }

    /// Update search matches
    pub fn set_search_matches(&mut self, matches: Vec<(usize, usize, usize)>) {
        self.search_matches = matches;
        self.current_match = 0;
    }

    /// Get current selection bounds (normalized)
    pub fn get_selection_bounds(&self) -> Option<(ViCursor, ViCursor)> {
        self.selection.as_ref().map(|sel| {
            let (start, end) = if sel.start.line > sel.end.line
                || (sel.start.line == sel.end.line && sel.start.col > sel.end.col)
            {
                (sel.end, sel.start)
            } else {
                (sel.start, sel.end)
            };
            (start, end)
        })
    }

    /// Get status line text
    pub fn status_text(&self) -> String {
        let mode = match self.state {
            ViState::Normal => "NORMAL",
            ViState::Visual => "VISUAL",
            ViState::VisualLine => "V-LINE",
            ViState::VisualBlock => "V-BLOCK",
            ViState::SearchForward => "SEARCH",
            ViState::SearchBackward => "SEARCH?",
        };

        let count = self
            .count
            .map_or(String::new(), |c| format!("{}", c));
        let op = self.pending_op.map_or(String::new(), |o| format!("{}", o));

        if matches!(
            self.state,
            ViState::SearchForward | ViState::SearchBackward
        ) {
            let prefix = if self.state == ViState::SearchForward {
                "/"
            } else {
                "?"
            };
            format!("-- {} -- {}{}", mode, prefix, self.search_query)
        } else {
            format!("-- {} -- {}{}  {}:{}", mode, count, op, self.cursor.line, self.cursor.col)
        }
    }
}

/// Actions that vi mode can trigger
#[derive(Clone, Debug)]
pub enum ViAction {
    /// No action
    None,
    /// Exit vi mode
    Exit,
    /// Move cursor (cursor position updated in ViMode)
    MoveCursor,
    /// Scroll down by N lines
    ScrollDown(usize),
    /// Scroll up by N lines
    ScrollUp(usize),
    /// Start visual selection
    StartSelection,
    /// Update visual selection
    UpdateSelection,
    /// Cancel visual selection
    CancelSelection,
    /// Start search mode
    StartSearch,
    /// Update search as typing
    UpdateSearch(String),
    /// Execute search
    ExecuteSearch(String),
    /// Cancel search
    CancelSearch,
    /// Jump to current match
    JumpToMatch,
    /// Yank current line
    YankLine,
    /// Yank selection
    YankSelection,
    /// Yank to end of line
    YankToEnd,
    /// Set a mark
    SetMark,
    /// Go to a mark
    GoToMark,
    /// Open URL/path under cursor
    OpenUnderCursor,
    /// Paste from yank register
    Paste,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vi_mode_navigation() {
        let mut vi = ViMode::new();
        vi.enter(100);
        vi.max_col = 80;

        // Move right
        vi.handle_key('l');
        assert_eq!(vi.cursor.col, 1);

        // Move down
        vi.handle_key('j');
        assert_eq!(vi.cursor.line, 1);

        // Move up
        vi.handle_key('k');
        assert_eq!(vi.cursor.line, 0);

        // Move left
        vi.handle_key('h');
        assert_eq!(vi.cursor.col, 0);
    }

    #[test]
    fn test_count_prefix() {
        let mut vi = ViMode::new();
        vi.enter(100);
        vi.max_col = 80;

        // 5j should move 5 lines
        vi.handle_key('5');
        vi.handle_key('j');
        assert_eq!(vi.cursor.line, 5);
    }

    #[test]
    fn test_visual_mode() {
        let mut vi = ViMode::new();
        vi.enter(100);

        // Enter visual mode
        vi.handle_key('v');
        assert_eq!(vi.state, ViState::Visual);
        assert!(vi.selection.is_some());

        // Exit visual mode
        vi.handle_key('\x1b');
        assert_eq!(vi.state, ViState::Normal);
        assert!(vi.selection.is_none());
    }
}
