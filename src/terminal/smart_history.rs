//! Smart Command History
//!
//! Enhanced command history with context tracking, fuzzy search, and persistence.
//! Inspired by Warp's AI-powered history and atuin shell history.

#![allow(dead_code)]

use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

use crate::terminal::project::ProjectType;

/// A command entry with full context
#[derive(Clone, Debug)]
pub struct HistoryEntry {
    /// The command that was executed
    pub command: String,
    /// Working directory when command was run
    pub cwd: PathBuf,
    /// Exit code (None if not yet completed)
    pub exit_code: Option<i32>,
    /// Timestamp when command was executed
    pub timestamp: SystemTime,
    /// Duration of command execution
    pub duration: Option<Duration>,
    /// Project type detected at time of execution
    pub project_type: Option<ProjectType>,
    /// Session ID (to group commands from same session)
    pub session_id: u64,
    /// Tags (user-defined or auto-detected)
    pub tags: Vec<String>,
    /// Output snippet (first few lines, for context)
    pub output_snippet: Option<String>,
}

impl HistoryEntry {
    /// Create a new history entry
    pub fn new(command: String, cwd: PathBuf, session_id: u64) -> Self {
        Self {
            command,
            cwd,
            exit_code: None,
            timestamp: SystemTime::now(),
            duration: None,
            project_type: None,
            session_id,
            tags: Vec::new(),
            output_snippet: None,
        }
    }

    /// Mark command as completed with exit code and duration
    pub fn complete(&mut self, exit_code: i32, duration: Duration) {
        self.exit_code = Some(exit_code);
        self.duration = Some(duration);
    }

    /// Add output snippet
    pub fn set_output(&mut self, output: &str) {
        // Keep first 3 lines or first 200 chars, whichever is shorter
        let snippet: String = output
            .lines()
            .take(3)
            .collect::<Vec<_>>()
            .join("\n");
        let snippet = if snippet.len() > 200 {
            format!("{}...", &snippet[..200])
        } else {
            snippet
        };
        self.output_snippet = Some(snippet);
    }

    /// Check if command was successful
    pub fn is_success(&self) -> bool {
        self.exit_code == Some(0)
    }

    /// Get a relevance score for fuzzy matching
    pub fn relevance_score(&self, query: &str, current_cwd: &PathBuf) -> i32 {
        let mut score = 0;
        let query_lower = query.to_lowercase();
        let cmd_lower = self.command.to_lowercase();

        // Exact match is best
        if cmd_lower == query_lower {
            score += 1000;
        }
        // Starts with query
        else if cmd_lower.starts_with(&query_lower) {
            score += 500;
        }
        // Contains query
        else if cmd_lower.contains(&query_lower) {
            score += 200;
        }
        // Fuzzy match (all chars in order)
        else if Self::fuzzy_match(&query_lower, &cmd_lower) {
            score += 100;
        } else {
            // No match at all
            return 0;
        }

        // Boost for same directory
        if self.cwd == *current_cwd {
            score += 100;
        }
        // Boost for parent/child directory
        else if current_cwd.starts_with(&self.cwd) || self.cwd.starts_with(current_cwd) {
            score += 50;
        }

        // Boost for recent commands (last hour)
        if let Ok(elapsed) = self.timestamp.elapsed() {
            if elapsed < Duration::from_secs(3600) {
                score += 50;
            } else if elapsed < Duration::from_secs(86400) {
                score += 25;
            }
        }

        // Boost for successful commands
        if self.is_success() {
            score += 20;
        }

        // Slight penalty for very long commands (likely one-off)
        if self.command.len() > 100 {
            score -= 10;
        }

        score
    }

    /// Simple fuzzy matching (all chars in order)
    fn fuzzy_match(needle: &str, haystack: &str) -> bool {
        let mut needle_chars = needle.chars().peekable();
        for c in haystack.chars() {
            if needle_chars.peek() == Some(&c) {
                needle_chars.next();
            }
        }
        needle_chars.peek().is_none()
    }
}

/// Smart history manager
pub struct SmartHistory {
    /// All history entries
    entries: Vec<HistoryEntry>,
    /// Maximum entries to keep
    max_entries: usize,
    /// Current session ID
    session_id: u64,
    /// Index for navigation
    position: Option<usize>,
    /// Filtered entries for current search
    filtered: Vec<usize>,
    /// Current search query
    search_query: String,
    /// Command frequency map (for suggestions)
    frequency: HashMap<String, usize>,
    /// Directory-specific command frequency
    dir_frequency: HashMap<PathBuf, HashMap<String, usize>>,
}

impl Default for SmartHistory {
    fn default() -> Self {
        Self::new(10000)
    }
}

impl SmartHistory {
    /// Create a new smart history
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: Vec::with_capacity(max_entries),
            max_entries,
            session_id: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
            position: None,
            filtered: Vec::new(),
            search_query: String::new(),
            frequency: HashMap::new(),
            dir_frequency: HashMap::new(),
        }
    }

    /// Add a command to history
    pub fn add(&mut self, command: &str, cwd: PathBuf, project_type: Option<ProjectType>) {
        let command = command.trim();
        if command.is_empty() {
            return;
        }

        // Don't add if identical to last command
        if let Some(last) = self.entries.last() {
            if last.command == command && last.cwd == cwd {
                self.position = None;
                return;
            }
        }

        // Create entry
        let mut entry = HistoryEntry::new(command.to_string(), cwd.clone(), self.session_id);
        entry.project_type = project_type;

        // Auto-tag based on command
        entry.tags = Self::auto_tag(command);

        // Update frequency maps
        *self.frequency.entry(command.to_string()).or_insert(0) += 1;
        *self
            .dir_frequency
            .entry(cwd)
            .or_default()
            .entry(command.to_string())
            .or_insert(0) += 1;

        // Remove oldest if at capacity
        if self.entries.len() >= self.max_entries {
            self.entries.remove(0);
        }

        self.entries.push(entry);
        self.position = None;
        self.filtered.clear();
    }

    /// Update the last entry with completion info
    pub fn complete_last(&mut self, exit_code: i32, duration: Duration, output: Option<&str>) {
        if let Some(entry) = self.entries.last_mut() {
            entry.complete(exit_code, duration);
            if let Some(out) = output {
                entry.set_output(out);
            }
        }
    }

    /// Auto-generate tags based on command content
    fn auto_tag(command: &str) -> Vec<String> {
        let mut tags = Vec::new();
        let cmd = command.to_lowercase();

        // Git commands
        if cmd.starts_with("git ") {
            tags.push("git".to_string());
            if cmd.contains("commit") {
                tags.push("commit".to_string());
            }
            if cmd.contains("push") || cmd.contains("pull") {
                tags.push("sync".to_string());
            }
        }

        // Package managers
        if cmd.starts_with("npm ") || cmd.starts_with("yarn ") || cmd.starts_with("pnpm ") {
            tags.push("npm".to_string());
        }
        if cmd.starts_with("cargo ") {
            tags.push("cargo".to_string());
        }
        if cmd.starts_with("pip ") || cmd.starts_with("pip3 ") {
            tags.push("pip".to_string());
        }

        // Docker
        if cmd.starts_with("docker ") || cmd.starts_with("docker-compose ") {
            tags.push("docker".to_string());
        }

        // Build/test
        if cmd.contains("build") || cmd.contains("compile") {
            tags.push("build".to_string());
        }
        if cmd.contains("test") {
            tags.push("test".to_string());
        }

        // Navigation
        if cmd.starts_with("cd ") || cmd.starts_with("ls") || cmd.starts_with("pwd") {
            tags.push("nav".to_string());
        }

        tags
    }

    /// Search history with fuzzy matching and context
    pub fn search(&mut self, query: &str, current_cwd: &PathBuf) -> Vec<&HistoryEntry> {
        self.search_query = query.to_string();

        if query.is_empty() {
            // Return recent unique commands
            self.filtered = (0..self.entries.len()).rev().collect();
        } else {
            // Score and filter
            let mut scored: Vec<(usize, i32)> = self
                .entries
                .iter()
                .enumerate()
                .filter_map(|(i, entry)| {
                    let score = entry.relevance_score(query, current_cwd);
                    if score > 0 {
                        Some((i, score))
                    } else {
                        None
                    }
                })
                .collect();

            // Sort by score (highest first)
            scored.sort_by(|a, b| b.1.cmp(&a.1));

            // Deduplicate by command (keep highest scored)
            let mut seen = std::collections::HashSet::new();
            self.filtered = scored
                .into_iter()
                .filter(|(i, _)| seen.insert(self.entries[*i].command.clone()))
                .map(|(i, _)| i)
                .collect();
        }

        self.filtered
            .iter()
            .map(|&i| &self.entries[i])
            .collect()
    }

    /// Get previous command (up arrow)
    pub fn previous(&mut self) -> Option<&str> {
        if self.entries.is_empty() {
            return None;
        }

        let list = if self.filtered.is_empty() {
            // Use all entries
            let new_pos = match self.position {
                None => self.entries.len().saturating_sub(1),
                Some(0) => 0,
                Some(pos) => pos - 1,
            };
            self.position = Some(new_pos);
            self.entries.get(new_pos).map(|e| e.command.as_str())
        } else {
            // Use filtered entries
            let new_pos = match self.position {
                None => 0,
                Some(pos) if pos + 1 < self.filtered.len() => pos + 1,
                Some(pos) => pos,
            };
            self.position = Some(new_pos);
            self.filtered
                .get(new_pos)
                .and_then(|&i| self.entries.get(i))
                .map(|e| e.command.as_str())
        };

        list
    }

    /// Get next command (down arrow)
    pub fn next(&mut self) -> Option<&str> {
        match self.position {
            None => None,
            Some(pos) => {
                if self.filtered.is_empty() {
                    // Use all entries
                    let new_pos = pos + 1;
                    if new_pos >= self.entries.len() {
                        self.position = None;
                        None
                    } else {
                        self.position = Some(new_pos);
                        self.entries.get(new_pos).map(|e| e.command.as_str())
                    }
                } else {
                    // Use filtered entries
                    if pos == 0 {
                        self.position = None;
                        None
                    } else {
                        let new_pos = pos - 1;
                        self.position = Some(new_pos);
                        self.filtered
                            .get(new_pos)
                            .and_then(|&i| self.entries.get(i))
                            .map(|e| e.command.as_str())
                    }
                }
            }
        }
    }

    /// Reset navigation position
    pub fn reset_position(&mut self) {
        self.position = None;
        self.filtered.clear();
        self.search_query.clear();
    }

    /// Get suggestions for autocomplete based on current input and directory
    pub fn suggest(&self, prefix: &str, cwd: &PathBuf, limit: usize) -> Vec<String> {
        let prefix_lower = prefix.to_lowercase();

        // Get directory-specific suggestions first
        let mut suggestions: Vec<(String, usize)> = self
            .dir_frequency
            .get(cwd)
            .map(|freq| {
                freq.iter()
                    .filter(|(cmd, _)| cmd.to_lowercase().starts_with(&prefix_lower))
                    .map(|(cmd, count)| (cmd.clone(), *count * 2)) // Boost dir-specific
                    .collect()
            })
            .unwrap_or_default();

        // Add global suggestions
        for (cmd, count) in &self.frequency {
            if cmd.to_lowercase().starts_with(&prefix_lower) {
                // Only add if not already present
                if !suggestions.iter().any(|(s, _)| s == cmd) {
                    suggestions.push((cmd.clone(), *count));
                }
            }
        }

        // Sort by frequency
        suggestions.sort_by(|a, b| b.1.cmp(&a.1));

        suggestions
            .into_iter()
            .take(limit)
            .map(|(s, _)| s)
            .collect()
    }

    /// Get most frequent commands overall
    pub fn top_commands(&self, limit: usize) -> Vec<(&str, usize)> {
        let mut freq: Vec<_> = self.frequency.iter().collect();
        freq.sort_by(|a, b| b.1.cmp(a.1));
        freq.into_iter()
            .take(limit)
            .map(|(k, v)| (k.as_str(), *v))
            .collect()
    }

    /// Get commands by tag
    pub fn by_tag(&self, tag: &str) -> Vec<&HistoryEntry> {
        self.entries
            .iter()
            .filter(|e| e.tags.iter().any(|t| t == tag))
            .collect()
    }

    /// Get failed commands (for debugging patterns)
    pub fn failed_commands(&self, limit: usize) -> Vec<&HistoryEntry> {
        self.entries
            .iter()
            .rev()
            .filter(|e| e.exit_code.map_or(false, |c| c != 0))
            .take(limit)
            .collect()
    }

    /// Get history for a specific directory
    pub fn for_directory(&self, cwd: &PathBuf) -> Vec<&HistoryEntry> {
        self.entries
            .iter()
            .filter(|e| e.cwd == *cwd)
            .collect()
    }

    /// Get commands from current session
    pub fn current_session(&self) -> Vec<&HistoryEntry> {
        self.entries
            .iter()
            .filter(|e| e.session_id == self.session_id)
            .collect()
    }

    /// Get recent command strings (for AI context)
    pub fn recent_commands(&self, limit: usize) -> Vec<String> {
        self.entries
            .iter()
            .rev()
            .take(limit)
            .map(|e| e.command.clone())
            .collect()
    }

    /// Get all entries
    pub fn all(&self) -> impl Iterator<Item = &HistoryEntry> {
        self.entries.iter()
    }

    /// Get number of entries
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Get statistics
    pub fn stats(&self) -> HistoryStats<'_> {
        let total = self.entries.len();
        let successful = self
            .entries
            .iter()
            .filter(|e| e.is_success())
            .count();
        let failed = self
            .entries
            .iter()
            .filter(|e| e.exit_code.map_or(false, |c| c != 0))
            .count();
        let unique = self.frequency.len();

        HistoryStats {
            total_commands: total,
            unique_commands: unique,
            successful,
            failed,
            most_used: self.top_commands(5),
        }
    }
}

/// History statistics
pub struct HistoryStats<'a> {
    pub total_commands: usize,
    pub unique_commands: usize,
    pub successful: usize,
    pub failed: usize,
    pub most_used: Vec<(&'a str, usize)>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_retrieve() {
        let mut history = SmartHistory::new(100);
        let cwd = PathBuf::from("/home/user/project");

        history.add("ls -la", cwd.clone(), None);
        history.add("git status", cwd.clone(), None);
        history.add("cargo build", cwd.clone(), None);

        assert_eq!(history.len(), 3);
    }

    #[test]
    fn test_search() {
        let mut history = SmartHistory::new(100);
        let cwd = PathBuf::from("/home/user/project");

        history.add("git status", cwd.clone(), None);
        history.add("git commit -m 'test'", cwd.clone(), None);
        history.add("git push", cwd.clone(), None);
        history.add("cargo build", cwd.clone(), None);

        let results = history.search("git", &cwd);
        assert_eq!(results.len(), 3);
    }

    #[test]
    fn test_suggestions() {
        let mut history = SmartHistory::new(100);
        let cwd = PathBuf::from("/home/user/project");

        history.add("cargo build", cwd.clone(), None);
        history.add("cargo build", cwd.clone(), None); // Add twice for frequency
        history.add("cargo test", cwd.clone(), None);
        history.add("cargo run", cwd.clone(), None);

        let suggestions = history.suggest("cargo", &cwd, 5);
        assert!(!suggestions.is_empty());
        assert!(suggestions[0].contains("cargo"));
    }
}
