//! Fuzzy Finder
//!
//! fzf-like fuzzy search for history, files, and git branches.
//! Ctrl+R for history, Ctrl+Shift+F for files, Ctrl+G for git branches.

#![allow(dead_code)]

use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Fuzzy finder mode
#[derive(Clone, Debug, PartialEq)]
pub enum FuzzyMode {
    /// Search command history (Ctrl+R)
    History,
    /// Search files in current directory (Ctrl+Shift+F)
    Files,
    /// Search git branches (Ctrl+G)
    GitBranches,
}

/// A fuzzy finder result item
#[derive(Clone, Debug)]
pub struct FuzzyItem {
    /// Display text (what the user sees)
    pub display: String,
    /// Value to insert/execute
    pub value: String,
    /// Preview/description text
    pub preview: Option<String>,
    /// Match score (higher = better match)
    pub score: i32,
    /// Match positions for highlighting
    pub match_positions: Vec<usize>,
    /// Icon for the item type
    pub icon: &'static str,
}

/// Fuzzy finder result action
#[derive(Clone, Debug, PartialEq)]
pub enum FuzzyAction {
    /// No action (still searching)
    None,
    /// Insert result into command line
    Insert(String),
    /// Execute result immediately
    Execute(String),
    /// Cancelled by user
    Cancelled,
}

/// Fuzzy finder state
pub struct FuzzyFinder {
    /// Whether the fuzzy finder is active
    pub active: bool,
    /// Current mode
    pub mode: FuzzyMode,
    /// Search query
    pub query: String,
    /// All items (unfiltered)
    all_items: Vec<FuzzyItem>,
    /// Filtered/scored items
    pub items: Vec<FuzzyItem>,
    /// Currently selected index
    pub selected: usize,
    /// Maximum items to display
    pub max_display: usize,
    /// Scroll offset for long lists
    pub scroll_offset: usize,
    /// Working directory (for file/git search)
    cwd: PathBuf,
}

impl Default for FuzzyFinder {
    fn default() -> Self {
        Self {
            active: false,
            mode: FuzzyMode::History,
            query: String::new(),
            all_items: Vec::new(),
            items: Vec::new(),
            selected: 0,
            max_display: 10,
            scroll_offset: 0,
            cwd: PathBuf::new(),
        }
    }
}

impl FuzzyFinder {
    /// Create a new fuzzy finder
    pub fn new() -> Self {
        Self::default()
    }

    /// Activate fuzzy finder in a specific mode
    pub fn activate(&mut self, mode: FuzzyMode, cwd: &Path) {
        self.active = true;
        self.mode = mode.clone();
        self.query.clear();
        self.selected = 0;
        self.scroll_offset = 0;
        self.cwd = cwd.to_path_buf();

        // Load items based on mode
        self.all_items = match &mode {
            FuzzyMode::History => Vec::new(), // Will be populated externally
            FuzzyMode::Files => self.load_files(),
            FuzzyMode::GitBranches => self.load_git_branches(),
        };

        self.items = self.all_items.clone();
    }

    /// Set history items (called externally since SmartHistory is in app.rs)
    pub fn set_history_items(&mut self, history: Vec<(String, Option<String>)>) {
        self.all_items = history
            .into_iter()
            .map(|(cmd, preview)| FuzzyItem {
                display: cmd.clone(),
                value: cmd,
                preview,
                score: 0,
                match_positions: Vec::new(),
                icon: "📜",
            })
            .collect();
        self.items = self.all_items.clone();
    }

    /// Deactivate fuzzy finder
    pub fn deactivate(&mut self) {
        self.active = false;
        self.query.clear();
        self.all_items.clear();
        self.items.clear();
        self.selected = 0;
        self.scroll_offset = 0;
    }

    /// Add character to query
    pub fn push_char(&mut self, c: char) {
        self.query.push(c);
        self.filter_and_score();
        self.selected = 0;
        self.scroll_offset = 0;
    }

    /// Remove last character from query
    pub fn pop_char(&mut self) {
        self.query.pop();
        self.filter_and_score();
        self.selected = 0;
        self.scroll_offset = 0;
    }

    /// Move selection up
    pub fn select_up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
        self.adjust_scroll();
    }

    /// Move selection down
    pub fn select_down(&mut self) {
        if !self.items.is_empty() && self.selected + 1 < self.items.len() {
            self.selected += 1;
        }
        self.adjust_scroll();
    }

    /// Adjust scroll offset to keep selection visible
    fn adjust_scroll(&mut self) {
        if self.selected < self.scroll_offset {
            self.scroll_offset = self.selected;
        } else if self.selected >= self.scroll_offset + self.max_display {
            self.scroll_offset = self.selected - self.max_display + 1;
        }
    }

    /// Get the selected item
    pub fn get_selected(&self) -> Option<&FuzzyItem> {
        self.items.get(self.selected)
    }

    /// Get visible items (for rendering)
    pub fn visible_items(&self) -> impl Iterator<Item = (usize, &FuzzyItem)> {
        self.items
            .iter()
            .enumerate()
            .skip(self.scroll_offset)
            .take(self.max_display)
    }

    /// Filter and score items based on query
    fn filter_and_score(&mut self) {
        if self.query.is_empty() {
            self.items = self.all_items.clone();
            return;
        }

        let query_lower = self.query.to_lowercase();
        self.items = self
            .all_items
            .iter()
            .filter_map(|item| {
                let (score, positions) =
                    self.fuzzy_score(&query_lower, &item.display.to_lowercase());
                if score > 0 {
                    let mut new_item = item.clone();
                    new_item.score = score;
                    new_item.match_positions = positions;
                    Some(new_item)
                } else {
                    None
                }
            })
            .collect();

        // Sort by score (highest first)
        self.items.sort_by(|a, b| b.score.cmp(&a.score));
    }

    /// Calculate fuzzy match score and positions
    fn fuzzy_score(&self, query: &str, target: &str) -> (i32, Vec<usize>) {
        // Exact match
        if target == query {
            return (1000, (0..query.len()).collect());
        }

        // Starts with query
        if target.starts_with(query) {
            return (500, (0..query.len()).collect());
        }

        // Contains query as substring
        if let Some(pos) = target.find(query) {
            return (200, (pos..pos + query.len()).collect());
        }

        // Fuzzy match (all chars in order)
        let mut score = 0;
        let mut positions = Vec::new();
        let mut query_chars = query.chars().peekable();
        let target_chars: Vec<char> = target.chars().collect();

        let mut target_idx = 0;
        while let Some(&query_char) = query_chars.peek() {
            if target_idx >= target_chars.len() {
                break;
            }

            if query_char == target_chars[target_idx] {
                positions.push(target_idx);
                query_chars.next();
                score += 10;

                // Bonus for consecutive matches
                if positions.len() > 1 {
                    let prev = positions[positions.len() - 2];
                    if target_idx == prev + 1 {
                        score += 5;
                    }
                }

                // Bonus for match at word boundary
                if target_idx == 0 {
                    score += 10;
                } else {
                    let prev_char = target_chars[target_idx - 1];
                    if prev_char == '/' || prev_char == ' ' || prev_char == '_' || prev_char == '-'
                    {
                        score += 10;
                    }
                }
            }

            target_idx += 1;
        }

        // All query chars must be matched
        if query_chars.peek().is_some() {
            return (0, Vec::new());
        }

        (score, positions)
    }

    /// Load files from current directory (recursive, limited depth)
    fn load_files(&self) -> Vec<FuzzyItem> {
        let mut items = Vec::new();

        let walker = WalkDir::new(&self.cwd)
            .max_depth(4)
            .follow_links(false)
            .into_iter()
            .filter_entry(|e| {
                // Skip hidden directories and common ignore patterns
                let name = e.file_name().to_string_lossy();
                !name.starts_with('.')
                    && name != "node_modules"
                    && name != "target"
                    && name != "__pycache__"
                    && name != "dist"
                    && name != "build"
                    && name != ".git"
            });

        for entry in walker.filter_map(|e| e.ok()).take(1000) {
            let path = entry.path();
            if path == self.cwd {
                continue;
            }

            let relative = path
                .strip_prefix(&self.cwd)
                .unwrap_or(path)
                .to_string_lossy()
                .replace('\\', "/");

            let is_dir = entry.file_type().is_dir();
            let icon = if is_dir { "📁" } else { "📄" };

            items.push(FuzzyItem {
                display: relative.clone(),
                value: relative,
                preview: None,
                score: 0,
                match_positions: Vec::new(),
                icon,
            });
        }

        items
    }

    /// Load git branches from repository
    fn load_git_branches(&self) -> Vec<FuzzyItem> {
        let mut items = Vec::new();

        // Find .git directory
        let mut current = self.cwd.clone();
        loop {
            let git_dir = current.join(".git");
            if git_dir.is_dir() {
                // Read local branches
                let heads_dir = git_dir.join("refs").join("heads");
                if let Ok(entries) = std::fs::read_dir(&heads_dir) {
                    for entry in entries.filter_map(|e| e.ok()) {
                        let name = entry.file_name().to_string_lossy().to_string();
                        items.push(FuzzyItem {
                            display: name.clone(),
                            value: name,
                            preview: Some("local".to_string()),
                            score: 0,
                            match_positions: Vec::new(),
                            icon: "🌿",
                        });
                    }
                }

                // Read remote branches
                let remotes_dir = git_dir.join("refs").join("remotes");
                if let Ok(remotes) = std::fs::read_dir(&remotes_dir) {
                    for remote in remotes.filter_map(|e| e.ok()) {
                        if let Ok(branches) = std::fs::read_dir(remote.path()) {
                            let remote_name = remote.file_name().to_string_lossy().to_string();
                            for branch in branches.filter_map(|e| e.ok()) {
                                let branch_name = branch.file_name().to_string_lossy().to_string();
                                if branch_name == "HEAD" {
                                    continue;
                                }
                                let full_name = format!("{}/{}", remote_name, branch_name);
                                items.push(FuzzyItem {
                                    display: full_name.clone(),
                                    value: full_name,
                                    preview: Some("remote".to_string()),
                                    score: 0,
                                    match_positions: Vec::new(),
                                    icon: "🌐",
                                });
                            }
                        }
                    }
                }
                break;
            }

            if !current.pop() {
                break;
            }
        }

        // If no branches found, show message
        if items.is_empty() {
            items.push(FuzzyItem {
                display: "(not a git repository)".to_string(),
                value: String::new(),
                preview: None,
                score: 0,
                match_positions: Vec::new(),
                icon: "⚠️",
            });
        }

        items
    }

    /// Get mode display name
    pub fn mode_name(&self) -> &'static str {
        match self.mode {
            FuzzyMode::History => "History",
            FuzzyMode::Files => "Files",
            FuzzyMode::GitBranches => "Branches",
        }
    }

    /// Get mode icon
    pub fn mode_icon(&self) -> &'static str {
        match self.mode {
            FuzzyMode::History => "📜",
            FuzzyMode::Files => "📂",
            FuzzyMode::GitBranches => "🌿",
        }
    }

    /// Get status text
    pub fn status_text(&self) -> String {
        format!("{}/{}", self.items.len(), self.all_items.len())
    }
}
