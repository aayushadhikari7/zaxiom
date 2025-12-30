//! Hints System
//!
//! Smart text extraction from terminal output - URLs, file paths, git hashes, etc.
//! Inspired by Kitty's hints kitten.

#![allow(dead_code)]

use regex::Regex;
use std::sync::LazyLock;

/// Types of hints that can be extracted
#[derive(Clone, Debug, PartialEq)]
pub enum HintType {
    /// URL (http, https, ftp, file)
    Url,
    /// File path (absolute or relative)
    Path,
    /// Git commit hash (short or long)
    GitHash,
    /// IP address (v4 or v6)
    IpAddress,
    /// Email address
    Email,
    /// Line number reference (file:line)
    LineRef,
    /// Hex color code
    HexColor,
    /// UUID
    Uuid,
    /// Docker container/image ID
    DockerId,
    /// Custom pattern match
    Custom,
}

/// A hint extracted from text
#[derive(Clone, Debug)]
pub struct Hint {
    /// The matched text
    pub text: String,
    /// Start position in the line
    pub start: usize,
    /// End position in the line
    pub end: usize,
    /// Type of hint
    pub hint_type: HintType,
    /// Display label (for overlay mode)
    pub label: String,
    /// Line number where found
    pub line: usize,
}

impl HintType {
    /// Get a display icon for the hint type
    pub fn icon(&self) -> &'static str {
        match self {
            HintType::Url => "🔗",
            HintType::Path => "📄",
            HintType::GitHash => "📝",
            HintType::IpAddress => "🌐",
            HintType::Email => "📧",
            HintType::LineRef => "📍",
            HintType::HexColor => "🎨",
            HintType::Uuid => "🔑",
            HintType::DockerId => "🐳",
            HintType::Custom => "✨",
        }
    }

    /// Get action description
    pub fn action_desc(&self) -> &'static str {
        match self {
            HintType::Url => "Open in browser",
            HintType::Path => "Open file",
            HintType::GitHash => "Copy hash",
            HintType::IpAddress => "Copy address",
            HintType::Email => "Copy email",
            HintType::LineRef => "Go to line",
            HintType::HexColor => "Copy color",
            HintType::Uuid => "Copy UUID",
            HintType::DockerId => "Copy ID",
            HintType::Custom => "Copy",
        }
    }
}

// Precompiled regexes for performance
static URL_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"https?://[^\s<>\[\]{}|\\^`\x00-\x1f\x7f]+|file://[^\s]+").unwrap()
});

static PATH_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    // Match common path patterns
    Regex::new(r#"(?:[A-Za-z]:\\|/)[^\s:*?"<>|]+\.[a-zA-Z0-9]+|\.{0,2}/[^\s:*?"<>|]+"#).unwrap()
});

static GIT_HASH_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    // Match 7-40 hex chars that look like git hashes
    Regex::new(r"\b[0-9a-f]{7,40}\b").unwrap()
});

static IP_V4_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\b").unwrap()
});

static IP_V6_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b(?:[0-9a-fA-F]{1,4}:){7}[0-9a-fA-F]{1,4}\b").unwrap());

static EMAIL_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}\b").unwrap());

static LINE_REF_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    // Match file:line or file:line:column patterns
    Regex::new(r"[a-zA-Z0-9_./\\-]+\.(rs|py|js|ts|go|c|cpp|h|java|rb|php|swift|kt):\d+(?::\d+)?")
        .unwrap()
});

static HEX_COLOR_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"#[0-9a-fA-F]{6}\b|#[0-9a-fA-F]{3}\b").unwrap());

static UUID_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}\b")
        .unwrap()
});

static DOCKER_ID_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    // 12 or 64 char hex strings typical for docker
    Regex::new(r"\b[0-9a-f]{12}\b|\b[0-9a-f]{64}\b").unwrap()
});

/// Hints extractor
pub struct HintsExtractor {
    /// Which hint types to extract
    pub enabled_types: Vec<HintType>,
    /// Custom regex patterns
    pub custom_patterns: Vec<Regex>,
}

impl Default for HintsExtractor {
    fn default() -> Self {
        Self {
            enabled_types: vec![
                HintType::Url,
                HintType::Path,
                HintType::GitHash,
                HintType::Email,
                HintType::LineRef,
            ],
            custom_patterns: Vec::new(),
        }
    }
}

impl HintsExtractor {
    /// Create a new hints extractor
    pub fn new() -> Self {
        Self::default()
    }

    /// Extract all hints from text
    pub fn extract(&self, text: &str, line_number: usize) -> Vec<Hint> {
        let mut hints = Vec::new();
        let mut label_counter = 0;

        for hint_type in &self.enabled_types {
            let matches = self.extract_type(text, hint_type);
            for (start, end, matched_text) in matches {
                // Generate label (a, b, c, ... aa, ab, etc.)
                let label = Self::generate_label(label_counter);
                label_counter += 1;

                hints.push(Hint {
                    text: matched_text,
                    start,
                    end,
                    hint_type: hint_type.clone(),
                    label,
                    line: line_number,
                });
            }
        }

        // Extract custom patterns
        for pattern in &self.custom_patterns {
            for m in pattern.find_iter(text) {
                let label = Self::generate_label(label_counter);
                label_counter += 1;

                hints.push(Hint {
                    text: m.as_str().to_string(),
                    start: m.start(),
                    end: m.end(),
                    hint_type: HintType::Custom,
                    label,
                    line: line_number,
                });
            }
        }

        // Sort by position
        hints.sort_by_key(|h| h.start);
        hints
    }

    /// Extract hints of a specific type
    fn extract_type(&self, text: &str, hint_type: &HintType) -> Vec<(usize, usize, String)> {
        let regex = match hint_type {
            HintType::Url => &*URL_REGEX,
            HintType::Path => &*PATH_REGEX,
            HintType::GitHash => &*GIT_HASH_REGEX,
            HintType::IpAddress => {
                // Try both v4 and v6
                let mut results: Vec<(usize, usize, String)> = IP_V4_REGEX
                    .find_iter(text)
                    .map(|m| (m.start(), m.end(), m.as_str().to_string()))
                    .collect();
                results.extend(
                    IP_V6_REGEX
                        .find_iter(text)
                        .map(|m| (m.start(), m.end(), m.as_str().to_string())),
                );
                return results;
            }
            HintType::Email => &*EMAIL_REGEX,
            HintType::LineRef => &*LINE_REF_REGEX,
            HintType::HexColor => &*HEX_COLOR_REGEX,
            HintType::Uuid => &*UUID_REGEX,
            HintType::DockerId => &*DOCKER_ID_REGEX,
            HintType::Custom => return Vec::new(),
        };

        regex
            .find_iter(text)
            .map(|m| (m.start(), m.end(), m.as_str().to_string()))
            .collect()
    }

    /// Generate a label (a-z, then aa-zz, etc.)
    fn generate_label(n: usize) -> String {
        if n < 26 {
            ((b'a' + n as u8) as char).to_string()
        } else if n < 26 * 27 {
            let first = (b'a' + ((n - 26) / 26) as u8) as char;
            let second = (b'a' + ((n - 26) % 26) as u8) as char;
            format!("{}{}", first, second)
        } else {
            format!("{}", n)
        }
    }

    /// Add a custom pattern
    pub fn add_pattern(&mut self, pattern: &str) -> Result<(), regex::Error> {
        let regex = Regex::new(pattern)?;
        self.custom_patterns.push(regex);
        Ok(())
    }

    /// Enable specific hint types
    pub fn enable_types(&mut self, types: Vec<HintType>) {
        self.enabled_types = types;
    }

    /// Enable all hint types
    pub fn enable_all(&mut self) {
        self.enabled_types = vec![
            HintType::Url,
            HintType::Path,
            HintType::GitHash,
            HintType::IpAddress,
            HintType::Email,
            HintType::LineRef,
            HintType::HexColor,
            HintType::Uuid,
            HintType::DockerId,
        ];
    }
}

/// Hints mode state for overlay display
#[derive(Default)]
pub struct HintsMode {
    /// Whether hints mode is active
    pub active: bool,
    /// Current hints to display
    pub hints: Vec<Hint>,
    /// Current typed filter
    pub filter: String,
    /// Selected hint index
    pub selected: Option<usize>,
    /// Filter by type (None = all)
    pub type_filter: Option<HintType>,
}

impl HintsMode {
    /// Create new hints mode
    pub fn new() -> Self {
        Self::default()
    }

    /// Activate hints mode with extracted hints
    pub fn activate(&mut self, hints: Vec<Hint>) {
        self.active = true;
        self.hints = hints;
        self.filter.clear();
        self.selected = if self.hints.is_empty() { None } else { Some(0) };
        self.type_filter = None;
    }

    /// Deactivate hints mode
    pub fn deactivate(&mut self) {
        self.active = false;
        self.hints.clear();
        self.filter.clear();
        self.selected = None;
    }

    /// Filter hints by typed characters
    pub fn update_filter(&mut self, c: char) {
        self.filter.push(c);
        self.apply_filter();
    }

    /// Backspace in filter
    pub fn backspace(&mut self) {
        self.filter.pop();
        self.apply_filter();
    }

    /// Apply current filter
    fn apply_filter(&mut self) {
        // Find hints matching the filter
        let matches: Vec<usize> = self
            .hints
            .iter()
            .enumerate()
            .filter(|(_, h)| h.label.starts_with(&self.filter))
            .map(|(i, _)| i)
            .collect();

        if matches.len() == 1 {
            // Exact match - auto-select
            self.selected = Some(matches[0]);
        } else if !matches.is_empty() {
            self.selected = Some(matches[0]);
        } else {
            self.selected = None;
        }
    }

    /// Get the selected hint
    pub fn get_selected(&self) -> Option<&Hint> {
        self.selected.and_then(|i| self.hints.get(i))
    }

    /// Filter by hint type
    pub fn filter_by_type(&mut self, hint_type: HintType) {
        self.type_filter = Some(hint_type);
    }

    /// Get filtered hints
    pub fn get_filtered_hints(&self) -> Vec<&Hint> {
        self.hints
            .iter()
            .filter(|h| {
                let type_ok = self.type_filter.as_ref().is_none_or(|t| &h.hint_type == t);
                let label_ok = self.filter.is_empty() || h.label.starts_with(&self.filter);
                type_ok && label_ok
            })
            .collect()
    }
}

/// Extract all URLs from a block of text
pub fn extract_urls(text: &str) -> Vec<String> {
    URL_REGEX
        .find_iter(text)
        .map(|m| m.as_str().to_string())
        .collect()
}

/// Extract file paths from text
pub fn extract_paths(text: &str) -> Vec<String> {
    PATH_REGEX
        .find_iter(text)
        .map(|m| m.as_str().to_string())
        .collect()
}

/// Extract line references (file:line) from text
pub fn extract_line_refs(text: &str) -> Vec<(String, usize, Option<usize>)> {
    LINE_REF_REGEX
        .find_iter(text)
        .filter_map(|m| {
            let s = m.as_str();
            let parts: Vec<&str> = s.rsplitn(3, ':').collect();
            match parts.len() {
                2 => {
                    let line: usize = parts[0].parse().ok()?;
                    let file = parts[1].to_string();
                    Some((file, line, None))
                }
                3 => {
                    let col: usize = parts[0].parse().ok()?;
                    let line: usize = parts[1].parse().ok()?;
                    let file = parts[2].to_string();
                    Some((file, line, Some(col)))
                }
                _ => None,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_extraction() {
        let text = "Check out https://example.com and http://test.org/path?query=1";
        let urls = extract_urls(text);
        assert_eq!(urls.len(), 2);
        assert!(urls[0].starts_with("https://"));
        assert!(urls[1].starts_with("http://"));
    }

    #[test]
    fn test_line_ref_extraction() {
        let text = "Error at src/main.rs:42:10 and warning at lib.py:100";
        let refs = extract_line_refs(text);
        assert_eq!(refs.len(), 2);
        assert_eq!(refs[0], ("src/main.rs".to_string(), 42, Some(10)));
    }

    #[test]
    fn test_label_generation() {
        assert_eq!(HintsExtractor::generate_label(0), "a");
        assert_eq!(HintsExtractor::generate_label(25), "z");
        assert_eq!(HintsExtractor::generate_label(26), "aa");
        assert_eq!(HintsExtractor::generate_label(27), "ab");
    }
}
