//! Terminal grid for proper VT100/ANSI emulation
//!
//! Implements a 2D character grid with cursor positioning and ANSI escape sequence handling.

use std::collections::VecDeque;

/// A single cell in the terminal grid
#[derive(Debug, Clone)]
pub struct Cell {
    /// The character in this cell
    pub ch: char,
    /// Foreground color (ANSI color code, 0-255 or RGB)
    pub fg: Option<u32>,
    /// Background color
    pub bg: Option<u32>,
    /// Bold
    pub bold: bool,
    /// Dim
    pub dim: bool,
    /// Italic
    pub italic: bool,
    /// Underline
    pub underline: bool,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            ch: ' ',
            fg: None,
            bg: None,
            bold: false,
            dim: false,
            italic: false,
            underline: false,
        }
    }
}

impl Cell {
    pub fn with_char(ch: char) -> Self {
        Self { ch, ..Default::default() }
    }
}

/// Parser state for ANSI escape sequences
#[derive(Debug, Clone, PartialEq)]
enum ParserState {
    Normal,
    Escape,       // Just saw ESC
    Csi,          // ESC [
    CsiParam,     // Collecting CSI parameters
    Osc,          // ESC ]
    OscParam,     // Collecting OSC content
}

/// Terminal grid with VT100/ANSI emulation
pub struct TerminalGrid {
    /// The grid of cells (rows x cols)
    cells: Vec<Vec<Cell>>,
    /// Number of rows
    rows: usize,
    /// Number of columns
    cols: usize,
    /// Cursor row (0-indexed)
    cursor_row: usize,
    /// Cursor column (0-indexed)
    cursor_col: usize,
    /// Scrollback buffer (lines that scrolled off the top)
    scrollback: VecDeque<Vec<Cell>>,
    /// Maximum scrollback lines
    max_scrollback: usize,
    /// Parser state
    parser_state: ParserState,
    /// Accumulated escape sequence bytes
    escape_buffer: Vec<u8>,
    /// UTF-8 accumulator for multi-byte characters
    utf8_buffer: Vec<u8>,
    /// Current text attributes
    current_fg: Option<u32>,
    current_bg: Option<u32>,
    current_bold: bool,
    current_dim: bool,
    current_italic: bool,
    current_underline: bool,
    /// Alternate screen mode
    alternate_screen: bool,
    /// Saved main screen (when in alternate mode)
    saved_cells: Option<Vec<Vec<Cell>>>,
    saved_scrollback: Option<VecDeque<Vec<Cell>>>,
    saved_cursor: Option<(usize, usize)>,
}

impl TerminalGrid {
    pub fn new(rows: usize, cols: usize) -> Self {
        let cells = vec![vec![Cell::default(); cols]; rows];
        Self {
            cells,
            rows,
            cols,
            cursor_row: 0,
            cursor_col: 0,
            scrollback: VecDeque::new(),
            max_scrollback: 5000,
            parser_state: ParserState::Normal,
            escape_buffer: Vec::new(),
            utf8_buffer: Vec::new(),
            current_fg: None,
            current_bg: None,
            current_bold: false,
            current_dim: false,
            current_italic: false,
            current_underline: false,
            alternate_screen: false,
            saved_cells: None,
            saved_scrollback: None,
            saved_cursor: None,
        }
    }

    /// Process raw output bytes
    pub fn process(&mut self, data: &[u8]) {
        for &byte in data {
            self.process_byte(byte);
        }
    }

    /// Check if we're collecting a UTF-8 sequence
    fn is_collecting_utf8(&self) -> bool {
        !self.utf8_buffer.is_empty()
    }

    /// Get expected UTF-8 sequence length from first byte
    fn utf8_char_len(first_byte: u8) -> usize {
        if first_byte < 0x80 {
            1
        } else if first_byte < 0xE0 {
            2
        } else if first_byte < 0xF0 {
            3
        } else {
            4
        }
    }

    /// Try to decode the UTF-8 buffer and put the character
    fn try_decode_utf8(&mut self) {
        // Take the buffer to avoid borrow conflict
        let buffer = std::mem::take(&mut self.utf8_buffer);
        if let Ok(s) = std::str::from_utf8(&buffer) {
            for ch in s.chars() {
                self.put_char(ch);
            }
        }
        // Buffer is already cleared by take()
    }

    /// Process a single byte
    fn process_byte(&mut self, byte: u8) {
        // Handle UTF-8 continuation bytes
        if self.is_collecting_utf8() {
            if byte >= 0x80 && byte < 0xC0 {
                // Continuation byte
                self.utf8_buffer.push(byte);
                let expected_len = Self::utf8_char_len(self.utf8_buffer[0]);
                if self.utf8_buffer.len() >= expected_len {
                    self.try_decode_utf8();
                }
                return;
            } else {
                // Not a continuation - decode what we have and process this byte
                self.try_decode_utf8();
            }
        }

        match self.parser_state {
            ParserState::Normal => {
                match byte {
                    0x1b => {
                        self.parser_state = ParserState::Escape;
                        self.escape_buffer.clear();
                    }
                    0x07 => {
                        // Bell - ignore
                    }
                    0x08 => {
                        // Backspace
                        if self.cursor_col > 0 {
                            self.cursor_col -= 1;
                        }
                    }
                    0x09 => {
                        // Tab
                        let next_tab = (self.cursor_col / 8 + 1) * 8;
                        self.cursor_col = next_tab.min(self.cols - 1);
                    }
                    0x0a => {
                        // Line feed
                        self.line_feed();
                    }
                    0x0d => {
                        // Carriage return
                        self.cursor_col = 0;
                    }
                    // ASCII printable (0x20-0x7F)
                    0x20..=0x7F => {
                        self.put_char(byte as char);
                    }
                    // UTF-8 start bytes (0xC0-0xFD)
                    0xC0..=0xFD => {
                        self.utf8_buffer.clear();
                        self.utf8_buffer.push(byte);
                    }
                    _ => {
                        // Ignore other bytes (invalid UTF-8 starts, control chars, etc.)
                    }
                }
            }
            ParserState::Escape => {
                match byte {
                    b'[' => {
                        self.parser_state = ParserState::Csi;
                        self.escape_buffer.clear();
                    }
                    b']' => {
                        self.parser_state = ParserState::Osc;
                        self.escape_buffer.clear();
                    }
                    b'7' => {
                        // Save cursor
                        self.saved_cursor = Some((self.cursor_row, self.cursor_col));
                        self.parser_state = ParserState::Normal;
                    }
                    b'8' => {
                        // Restore cursor
                        if let Some((row, col)) = self.saved_cursor {
                            self.cursor_row = row.min(self.rows - 1);
                            self.cursor_col = col.min(self.cols - 1);
                        }
                        self.parser_state = ParserState::Normal;
                    }
                    b'M' => {
                        // Reverse index (scroll down)
                        if self.cursor_row == 0 {
                            self.scroll_down();
                        } else {
                            self.cursor_row -= 1;
                        }
                        self.parser_state = ParserState::Normal;
                    }
                    _ => {
                        // Unknown escape, ignore
                        self.parser_state = ParserState::Normal;
                    }
                }
            }
            ParserState::Csi | ParserState::CsiParam => {
                self.escape_buffer.push(byte);

                // Check if this is the final byte of the sequence
                if byte >= 0x40 && byte <= 0x7e {
                    self.handle_csi();
                    self.parser_state = ParserState::Normal;
                } else {
                    self.parser_state = ParserState::CsiParam;
                }
            }
            ParserState::Osc | ParserState::OscParam => {
                // OSC sequences end with BEL (0x07) or ST (ESC \)
                if byte == 0x07 {
                    // End of OSC
                    self.parser_state = ParserState::Normal;
                } else if byte == 0x1b {
                    // Could be ST, check next byte
                    self.escape_buffer.push(byte);
                    self.parser_state = ParserState::OscParam;
                } else if self.parser_state == ParserState::OscParam && byte == b'\\' {
                    // ST (String Terminator)
                    self.parser_state = ParserState::Normal;
                } else {
                    self.escape_buffer.push(byte);
                    self.parser_state = ParserState::OscParam;
                }
            }
        }
    }

    /// Handle a complete CSI sequence
    fn handle_csi(&mut self) {
        if self.escape_buffer.is_empty() {
            return;
        }

        let final_byte = *self.escape_buffer.last().unwrap();
        let params_str = String::from_utf8_lossy(&self.escape_buffer[..self.escape_buffer.len() - 1]);

        // Parse parameters (semicolon-separated numbers)
        let params: Vec<usize> = params_str
            .split(|c| c == ';' || c == ':')
            .filter_map(|s| s.trim_start_matches('?').parse().ok())
            .collect();

        let param = |i: usize, default: usize| -> usize {
            params.get(i).copied().unwrap_or(default).max(1)
        };

        match final_byte {
            b'A' => {
                // Cursor Up
                let n = param(0, 1);
                self.cursor_row = self.cursor_row.saturating_sub(n);
            }
            b'B' => {
                // Cursor Down
                let n = param(0, 1);
                self.cursor_row = (self.cursor_row + n).min(self.rows - 1);
            }
            b'C' => {
                // Cursor Forward
                let n = param(0, 1);
                self.cursor_col = (self.cursor_col + n).min(self.cols - 1);
            }
            b'D' => {
                // Cursor Back
                let n = param(0, 1);
                self.cursor_col = self.cursor_col.saturating_sub(n);
            }
            b'E' => {
                // Cursor Next Line
                let n = param(0, 1);
                self.cursor_row = (self.cursor_row + n).min(self.rows - 1);
                self.cursor_col = 0;
            }
            b'F' => {
                // Cursor Previous Line
                let n = param(0, 1);
                self.cursor_row = self.cursor_row.saturating_sub(n);
                self.cursor_col = 0;
            }
            b'G' => {
                // Cursor Horizontal Absolute (column)
                let col = param(0, 1).saturating_sub(1);
                self.cursor_col = col.min(self.cols - 1);
            }
            b'H' | b'f' => {
                // Cursor Position
                let row = param(0, 1).saturating_sub(1);
                let col = params.get(1).copied().unwrap_or(1).saturating_sub(1);
                self.cursor_row = row.min(self.rows - 1);
                self.cursor_col = col.min(self.cols - 1);
            }
            b'J' => {
                // Erase in Display
                let mode = params.first().copied().unwrap_or(0);
                self.erase_in_display(mode);
            }
            b'K' => {
                // Erase in Line
                let mode = params.first().copied().unwrap_or(0);
                self.erase_in_line(mode);
            }
            b'L' => {
                // Insert Lines
                let n = param(0, 1);
                self.insert_lines(n);
            }
            b'M' => {
                // Delete Lines
                let n = param(0, 1);
                self.delete_lines(n);
            }
            b'P' => {
                // Delete Characters
                let n = param(0, 1);
                self.delete_chars(n);
            }
            b'S' => {
                // Scroll Up
                let n = param(0, 1);
                for _ in 0..n {
                    self.scroll_up();
                }
            }
            b'T' => {
                // Scroll Down
                let n = param(0, 1);
                for _ in 0..n {
                    self.scroll_down();
                }
            }
            b'X' => {
                // Erase Characters
                let n = param(0, 1);
                for i in 0..n {
                    let col = self.cursor_col + i;
                    if col < self.cols {
                        self.cells[self.cursor_row][col] = Cell::default();
                    }
                }
            }
            b'd' => {
                // Cursor Vertical Absolute (row)
                let row = param(0, 1).saturating_sub(1);
                self.cursor_row = row.min(self.rows - 1);
            }
            b'm' => {
                // SGR (Select Graphic Rendition)
                self.handle_sgr(&params);
            }
            b'h' => {
                // Set Mode
                if params_str.starts_with('?') {
                    // DEC Private Mode Set
                    if params.contains(&1049) || params.contains(&47) {
                        // Alternate screen buffer
                        self.enter_alternate_screen();
                    }
                }
            }
            b'l' => {
                // Reset Mode
                if params_str.starts_with('?') {
                    // DEC Private Mode Reset
                    if params.contains(&1049) || params.contains(&47) {
                        // Exit alternate screen
                        self.exit_alternate_screen();
                    }
                }
            }
            b'r' => {
                // Set Scrolling Region - ignore for now
            }
            b's' => {
                // Save Cursor Position
                self.saved_cursor = Some((self.cursor_row, self.cursor_col));
            }
            b'u' => {
                // Restore Cursor Position
                if let Some((row, col)) = self.saved_cursor {
                    self.cursor_row = row.min(self.rows - 1);
                    self.cursor_col = col.min(self.cols - 1);
                }
            }
            _ => {
                // Unknown CSI sequence, ignore
            }
        }
    }

    /// Handle SGR (color/attribute) sequences
    fn handle_sgr(&mut self, params: &[usize]) {
        if params.is_empty() {
            // Reset all attributes
            self.reset_attributes();
            return;
        }

        let mut i = 0;
        while i < params.len() {
            match params[i] {
                0 => self.reset_attributes(),
                1 => self.current_bold = true,
                2 => self.current_dim = true,
                3 => self.current_italic = true,
                4 => self.current_underline = true,
                22 => { self.current_bold = false; self.current_dim = false; }
                23 => self.current_italic = false,
                24 => self.current_underline = false,
                30..=37 => self.current_fg = Some((params[i] - 30) as u32),
                38 => {
                    // Extended foreground color
                    if i + 2 < params.len() && params[i + 1] == 5 {
                        // 256 color
                        self.current_fg = Some(params[i + 2] as u32);
                        i += 2;
                    } else if i + 4 < params.len() && params[i + 1] == 2 {
                        // RGB
                        let r = params[i + 2] as u32;
                        let g = params[i + 3] as u32;
                        let b = params[i + 4] as u32;
                        self.current_fg = Some(0x1000000 | (r << 16) | (g << 8) | b);
                        i += 4;
                    }
                }
                39 => self.current_fg = None,
                40..=47 => self.current_bg = Some((params[i] - 40) as u32),
                48 => {
                    // Extended background color
                    if i + 2 < params.len() && params[i + 1] == 5 {
                        self.current_bg = Some(params[i + 2] as u32);
                        i += 2;
                    } else if i + 4 < params.len() && params[i + 1] == 2 {
                        let r = params[i + 2] as u32;
                        let g = params[i + 3] as u32;
                        let b = params[i + 4] as u32;
                        self.current_bg = Some(0x1000000 | (r << 16) | (g << 8) | b);
                        i += 4;
                    }
                }
                49 => self.current_bg = None,
                90..=97 => self.current_fg = Some((params[i] - 90 + 8) as u32), // Bright colors
                100..=107 => self.current_bg = Some((params[i] - 100 + 8) as u32),
                _ => {}
            }
            i += 1;
        }
    }

    /// Reset text attributes
    fn reset_attributes(&mut self) {
        self.current_fg = None;
        self.current_bg = None;
        self.current_bold = false;
        self.current_dim = false;
        self.current_italic = false;
        self.current_underline = false;
    }

    /// Put a character at current cursor position
    fn put_char(&mut self, ch: char) {
        if self.cursor_col >= self.cols {
            // Wrap to next line
            self.cursor_col = 0;
            self.line_feed();
        }

        self.cells[self.cursor_row][self.cursor_col] = Cell {
            ch,
            fg: self.current_fg,
            bg: self.current_bg,
            bold: self.current_bold,
            dim: self.current_dim,
            italic: self.current_italic,
            underline: self.current_underline,
        };
        self.cursor_col += 1;
    }

    /// Line feed (move cursor down, scroll if needed)
    fn line_feed(&mut self) {
        if self.cursor_row < self.rows - 1 {
            self.cursor_row += 1;
        } else {
            self.scroll_up();
        }
    }

    /// Scroll the screen up (move content up, new line at bottom)
    fn scroll_up(&mut self) {
        if !self.cells.is_empty() {
            let top_row = self.cells.remove(0);
            if !self.alternate_screen {
                self.scrollback.push_back(top_row);
                while self.scrollback.len() > self.max_scrollback {
                    self.scrollback.pop_front();
                }
            }
            self.cells.push(vec![Cell::default(); self.cols]);
        }
    }

    /// Scroll the screen down (move content down, new line at top)
    fn scroll_down(&mut self) {
        if !self.cells.is_empty() {
            self.cells.pop();
            self.cells.insert(0, vec![Cell::default(); self.cols]);
        }
    }

    /// Erase in display
    fn erase_in_display(&mut self, mode: usize) {
        match mode {
            0 => {
                // Erase from cursor to end of screen
                self.erase_in_line(0);
                for row in (self.cursor_row + 1)..self.rows {
                    self.cells[row] = vec![Cell::default(); self.cols];
                }
            }
            1 => {
                // Erase from start to cursor
                for row in 0..self.cursor_row {
                    self.cells[row] = vec![Cell::default(); self.cols];
                }
                self.erase_in_line(1);
            }
            2 | 3 => {
                // Erase entire screen (3 also clears scrollback)
                for row in 0..self.rows {
                    self.cells[row] = vec![Cell::default(); self.cols];
                }
                if mode == 3 {
                    self.scrollback.clear();
                }
            }
            _ => {}
        }
    }

    /// Erase in line
    fn erase_in_line(&mut self, mode: usize) {
        match mode {
            0 => {
                // Erase from cursor to end of line
                for col in self.cursor_col..self.cols {
                    self.cells[self.cursor_row][col] = Cell::default();
                }
            }
            1 => {
                // Erase from start to cursor
                for col in 0..=self.cursor_col.min(self.cols - 1) {
                    self.cells[self.cursor_row][col] = Cell::default();
                }
            }
            2 => {
                // Erase entire line
                self.cells[self.cursor_row] = vec![Cell::default(); self.cols];
            }
            _ => {}
        }
    }

    /// Insert blank lines at cursor
    fn insert_lines(&mut self, n: usize) {
        for _ in 0..n {
            if self.cursor_row < self.rows {
                self.cells.pop();
                self.cells.insert(self.cursor_row, vec![Cell::default(); self.cols]);
            }
        }
    }

    /// Delete lines at cursor
    fn delete_lines(&mut self, n: usize) {
        for _ in 0..n {
            if self.cursor_row < self.rows {
                self.cells.remove(self.cursor_row);
                self.cells.push(vec![Cell::default(); self.cols]);
            }
        }
    }

    /// Delete characters at cursor
    fn delete_chars(&mut self, n: usize) {
        let row = &mut self.cells[self.cursor_row];
        for _ in 0..n {
            if self.cursor_col < row.len() {
                row.remove(self.cursor_col);
                row.push(Cell::default());
            }
        }
    }

    /// Enter alternate screen mode
    fn enter_alternate_screen(&mut self) {
        if !self.alternate_screen {
            self.alternate_screen = true;
            self.saved_cells = Some(std::mem::replace(
                &mut self.cells,
                vec![vec![Cell::default(); self.cols]; self.rows],
            ));
            self.saved_scrollback = Some(std::mem::take(&mut self.scrollback));
            self.saved_cursor = Some((self.cursor_row, self.cursor_col));
            self.cursor_row = 0;
            self.cursor_col = 0;
        }
    }

    /// Exit alternate screen mode
    fn exit_alternate_screen(&mut self) {
        if self.alternate_screen {
            self.alternate_screen = false;
            if let Some(cells) = self.saved_cells.take() {
                self.cells = cells;
            }
            if let Some(scrollback) = self.saved_scrollback.take() {
                self.scrollback = scrollback;
            }
            if let Some((row, col)) = self.saved_cursor.take() {
                self.cursor_row = row;
                self.cursor_col = col;
            }
        }
    }

    /// Resize the grid
    pub fn resize(&mut self, new_rows: usize, new_cols: usize) {
        // Ensure minimum dimensions to avoid empty grid panics
        let new_rows = new_rows.max(1);
        let new_cols = new_cols.max(1);

        // Resize each existing row
        for row in &mut self.cells {
            row.resize(new_cols, Cell::default());
        }

        // Add or remove rows
        while self.cells.len() < new_rows {
            self.cells.push(vec![Cell::default(); new_cols]);
        }
        while self.cells.len() > new_rows {
            let removed = self.cells.remove(0);
            if !self.alternate_screen {
                self.scrollback.push_back(removed);
            }
        }

        // Resize scrollback rows
        for row in &mut self.scrollback {
            row.resize(new_cols, Cell::default());
        }

        self.rows = new_rows;
        self.cols = new_cols;

        // Clamp cursor
        self.cursor_row = self.cursor_row.min(new_rows.saturating_sub(1));
        self.cursor_col = self.cursor_col.min(new_cols.saturating_sub(1));
    }

    /// Get the visible grid as lines of text with ANSI colors preserved
    pub fn get_lines(&self) -> Vec<String> {
        let mut result = Vec::new();

        // Add scrollback
        for row in &self.scrollback {
            result.push(self.row_to_string(row));
        }

        // Add visible cells
        for row in &self.cells {
            result.push(self.row_to_string(row));
        }

        result
    }

    /// Get only the visible screen (no scrollback)
    pub fn get_visible_lines(&self) -> Vec<String> {
        self.cells.iter().map(|row| self.row_to_string(row)).collect()
    }

    /// Convert a row of cells to a string with ANSI colors
    fn row_to_string(&self, row: &[Cell]) -> String {
        let mut result = String::new();
        let mut last_fg: Option<u32> = None;
        let mut last_bg: Option<u32> = None;
        let mut last_bold = false;
        let mut last_dim = false;
        let mut last_italic = false;
        let mut last_underline = false;

        for cell in row {
            // Check if we need to change attributes
            let need_reset = (cell.fg != last_fg) || (cell.bg != last_bg) ||
                            (cell.bold != last_bold) || (cell.dim != last_dim) ||
                            (cell.italic != last_italic) || (cell.underline != last_underline);

            if need_reset {
                // Build SGR sequence
                let mut sgr_parts = vec!["0".to_string()]; // Reset first

                if cell.bold { sgr_parts.push("1".to_string()); }
                if cell.dim { sgr_parts.push("2".to_string()); }
                if cell.italic { sgr_parts.push("3".to_string()); }
                if cell.underline { sgr_parts.push("4".to_string()); }

                if let Some(fg) = cell.fg {
                    if fg >= 0x1000000 {
                        // RGB color
                        let r = (fg >> 16) & 0xff;
                        let g = (fg >> 8) & 0xff;
                        let b = fg & 0xff;
                        sgr_parts.push(format!("38;2;{};{};{}", r, g, b));
                    } else if fg < 8 {
                        sgr_parts.push(format!("{}", 30 + fg));
                    } else if fg < 16 {
                        sgr_parts.push(format!("{}", 90 + fg - 8));
                    } else {
                        sgr_parts.push(format!("38;5;{}", fg));
                    }
                }

                if let Some(bg) = cell.bg {
                    if bg >= 0x1000000 {
                        let r = (bg >> 16) & 0xff;
                        let g = (bg >> 8) & 0xff;
                        let b = bg & 0xff;
                        sgr_parts.push(format!("48;2;{};{};{}", r, g, b));
                    } else if bg < 8 {
                        sgr_parts.push(format!("{}", 40 + bg));
                    } else if bg < 16 {
                        sgr_parts.push(format!("{}", 100 + bg - 8));
                    } else {
                        sgr_parts.push(format!("48;5;{}", bg));
                    }
                }

                result.push_str(&format!("\x1b[{}m", sgr_parts.join(";")));

                last_fg = cell.fg;
                last_bg = cell.bg;
                last_bold = cell.bold;
                last_dim = cell.dim;
                last_italic = cell.italic;
                last_underline = cell.underline;
            }

            result.push(cell.ch);
        }

        // Reset at end of line
        if last_fg.is_some() || last_bg.is_some() || last_bold || last_dim || last_italic || last_underline {
            result.push_str("\x1b[0m");
        }

        // Trim trailing spaces
        result.trim_end().to_string()
    }

    /// Get cursor position (row, col)
    pub fn cursor_position(&self) -> (usize, usize) {
        (self.cursor_row, self.cursor_col)
    }

    /// Get dimensions (rows, cols)
    pub fn dimensions(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    /// Check if in alternate screen mode
    pub fn is_alternate_screen(&self) -> bool {
        self.alternate_screen
    }

    /// Clear the grid and scrollback
    pub fn clear(&mut self) {
        self.cells = vec![vec![Cell::default(); self.cols]; self.rows];
        self.scrollback.clear();
        self.cursor_row = 0;
        self.cursor_col = 0;
        self.utf8_buffer.clear();
        self.parser_state = ParserState::Normal;
        self.escape_buffer.clear();
    }
}

impl Default for TerminalGrid {
    fn default() -> Self {
        Self::new(24, 80)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_text() {
        let mut grid = TerminalGrid::new(24, 80);
        grid.process(b"Hello, World!");
        let lines = grid.get_visible_lines();
        assert!(lines[0].starts_with("Hello, World!"));
    }

    #[test]
    fn test_cursor_movement() {
        let mut grid = TerminalGrid::new(24, 80);
        grid.process(b"AAAA\x1b[2GXXX");
        let lines = grid.get_visible_lines();
        assert!(lines[0].starts_with("AXXX"));
    }

    #[test]
    fn test_newline() {
        let mut grid = TerminalGrid::new(24, 80);
        grid.process(b"Line 1\nLine 2");
        let lines = grid.get_visible_lines();
        assert!(lines[0].starts_with("Line 1"));
        assert!(lines[1].starts_with("Line 2"));
    }

    #[test]
    fn test_carriage_return() {
        let mut grid = TerminalGrid::new(24, 80);
        grid.process(b"XXXX\rHello");
        let lines = grid.get_visible_lines();
        assert!(lines[0].starts_with("Hello"));
    }

    #[test]
    fn test_erase_in_line() {
        let mut grid = TerminalGrid::new(24, 80);
        grid.process(b"Hello World\x1b[6G\x1b[K");
        let lines = grid.get_visible_lines();
        assert_eq!(lines[0].trim(), "Hello");
    }
}
