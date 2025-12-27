//! Output Formatting
//!
//! Smart formatting for command output including JSON pretty printing,
//! syntax highlighting hints, and more.

#![allow(dead_code)]

/// Detect if a string is likely JSON
pub fn is_json(s: &str) -> bool {
    let trimmed = s.trim();
    (trimmed.starts_with('{') && trimmed.ends_with('}'))
        || (trimmed.starts_with('[') && trimmed.ends_with(']'))
}

/// Pretty print JSON string
pub fn pretty_json(s: &str) -> Option<String> {
    // Try to parse and re-format JSON
    let value: serde_json::Value = serde_json::from_str(s).ok()?;
    serde_json::to_string_pretty(&value).ok()
}

/// Detect output type for syntax highlighting hints
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OutputType {
    /// Plain text
    Plain,
    /// JSON data
    Json,
    /// Error message
    Error,
    /// Warning message
    Warning,
    /// Success message
    Success,
    /// Code/script
    Code,
    /// Log output
    Log,
    /// Table/structured data
    Table,
}

/// Detect the type of output for a line
pub fn detect_output_type(line: &str) -> OutputType {
    let lower = line.to_lowercase();

    // Error patterns
    if lower.starts_with("error")
        || lower.starts_with("err:")
        || lower.contains("error:")
        || lower.contains("failed")
        || lower.contains("exception")
        || lower.contains("panic")
    {
        return OutputType::Error;
    }

    // Warning patterns
    if lower.starts_with("warning")
        || lower.starts_with("warn:")
        || lower.contains("warning:")
        || lower.contains("deprecated")
    {
        return OutputType::Warning;
    }

    // Success patterns
    if lower.contains("success")
        || lower.contains("completed")
        || lower.contains("finished")
        || lower.starts_with("ok")
        || lower.contains("✓")
        || lower.contains("✔")
    {
        return OutputType::Success;
    }

    // JSON
    let trimmed = line.trim();
    if (trimmed.starts_with('{') && trimmed.ends_with('}'))
        || (trimmed.starts_with('[') && trimmed.ends_with(']'))
        || trimmed.starts_with("\"") && trimmed.ends_with("\"")
    {
        return OutputType::Json;
    }

    // Log patterns (timestamp prefixed)
    if line.len() > 20 {
        let start = &line[..20];
        if start.contains(':') && (start.contains('-') || start.contains('/')) {
            return OutputType::Log;
        }
    }

    OutputType::Plain
}

/// Format a size in bytes to human-readable string
pub fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;

    if bytes >= TB {
        format!("{:.1}T", bytes as f64 / TB as f64)
    } else if bytes >= GB {
        format!("{:.1}G", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1}M", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1}K", bytes as f64 / KB as f64)
    } else {
        format!("{}B", bytes)
    }
}

/// Format a duration in seconds to human-readable string
pub fn format_duration(seconds: u64) -> String {
    if seconds < 60 {
        format!("{}s", seconds)
    } else if seconds < 3600 {
        format!("{}m {}s", seconds / 60, seconds % 60)
    } else if seconds < 86400 {
        let hours = seconds / 3600;
        let mins = (seconds % 3600) / 60;
        format!("{}h {}m", hours, mins)
    } else {
        let days = seconds / 86400;
        let hours = (seconds % 86400) / 3600;
        format!("{}d {}h", days, hours)
    }
}

/// Truncate a string with ellipsis
pub fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else if max_len <= 3 {
        "...".to_string()
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}

/// Highlight matching text in a string (returns segments)
pub fn highlight_matches(text: &str, query: &str) -> Vec<(String, bool)> {
    if query.is_empty() {
        return vec![(text.to_string(), false)];
    }

    let lower_text = text.to_lowercase();
    let lower_query = query.to_lowercase();

    let mut result = Vec::new();
    let mut last_end = 0;

    for (start, _) in lower_text.match_indices(&lower_query) {
        // Add non-matching segment
        if start > last_end {
            result.push((text[last_end..start].to_string(), false));
        }
        // Add matching segment
        let end = start + query.len();
        result.push((text[start..end].to_string(), true));
        last_end = end;
    }

    // Add remaining non-matching segment
    if last_end < text.len() {
        result.push((text[last_end..].to_string(), false));
    }

    if result.is_empty() {
        result.push((text.to_string(), false));
    }

    result
}
