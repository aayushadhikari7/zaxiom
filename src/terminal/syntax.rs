//! Syntax Highlighting Module
//!
//! Provides syntax highlighting for source code files using syntect.

#![allow(dead_code)]

use once_cell::sync::Lazy;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

/// Lazy-loaded syntax set
static SYNTAX_SET: Lazy<SyntaxSet> = Lazy::new(SyntaxSet::load_defaults_newlines);

/// Lazy-loaded theme set
static THEME_SET: Lazy<ThemeSet> = Lazy::new(ThemeSet::load_defaults);

/// A highlighted line segment with color info
#[derive(Clone, Debug)]
pub struct HighlightedSegment {
    /// The text content
    pub text: String,
    /// Foreground color (RGBA)
    pub fg: (u8, u8, u8, u8),
    /// Whether the text is bold
    pub bold: bool,
    /// Whether the text is italic
    pub italic: bool,
}

/// A fully highlighted line
pub type HighlightedLine = Vec<HighlightedSegment>;

/// Get supported file extensions for syntax highlighting
pub fn supported_extensions() -> Vec<&'static str> {
    vec![
        "rs",
        "py",
        "js",
        "ts",
        "jsx",
        "tsx",
        "c",
        "cpp",
        "h",
        "hpp",
        "java",
        "go",
        "rb",
        "php",
        "swift",
        "kt",
        "scala",
        "cs",
        "html",
        "css",
        "scss",
        "sass",
        "less",
        "json",
        "yaml",
        "yml",
        "toml",
        "xml",
        "md",
        "sh",
        "bash",
        "zsh",
        "fish",
        "ps1",
        "sql",
        "vim",
        "lua",
        "perl",
        "r",
        "dart",
        "ex",
        "exs",
        "hs",
        "ml",
        "fs",
        "clj",
        "lisp",
        "el",
        "asm",
        "s",
        "dockerfile",
        "makefile",
        "cmake",
        "gradle",
    ]
}

/// Check if a file extension is supported for highlighting
pub fn is_supported(extension: &str) -> bool {
    let ext_lower = extension.to_lowercase();
    SYNTAX_SET.find_syntax_by_extension(&ext_lower).is_some()
}

/// Highlight source code and return colored segments
pub fn highlight_code(code: &str, extension: &str) -> Option<Vec<HighlightedLine>> {
    let syntax = SYNTAX_SET.find_syntax_by_extension(extension)?;

    // Use a dark theme for terminal look
    let theme = &THEME_SET.themes["base16-ocean.dark"];
    let mut highlighter = HighlightLines::new(syntax, theme);

    let mut result = Vec::new();

    for line in LinesWithEndings::from(code) {
        let ranges = highlighter.highlight_line(line, &SYNTAX_SET).ok()?;
        let mut highlighted_line = Vec::new();

        for (style, text) in ranges {
            highlighted_line.push(style_to_segment(style, text));
        }

        result.push(highlighted_line);
    }

    Some(result)
}

/// Convert syntect Style to our HighlightedSegment
fn style_to_segment(style: Style, text: &str) -> HighlightedSegment {
    HighlightedSegment {
        text: text.to_string(),
        fg: (
            style.foreground.r,
            style.foreground.g,
            style.foreground.b,
            style.foreground.a,
        ),
        bold: style
            .font_style
            .contains(syntect::highlighting::FontStyle::BOLD),
        italic: style
            .font_style
            .contains(syntect::highlighting::FontStyle::ITALIC),
    }
}

/// Highlight code and format as ANSI colored string (for terminal output)
pub fn highlight_to_ansi(code: &str, extension: &str) -> Option<String> {
    let syntax = SYNTAX_SET.find_syntax_by_extension(extension)?;
    let theme = &THEME_SET.themes["base16-ocean.dark"];
    let mut highlighter = HighlightLines::new(syntax, theme);

    let mut output = String::new();

    for line in LinesWithEndings::from(code) {
        let ranges = highlighter.highlight_line(line, &SYNTAX_SET).ok()?;

        for (style, text) in ranges {
            // Convert to ANSI escape codes
            let r = style.foreground.r;
            let g = style.foreground.g;
            let b = style.foreground.b;

            // Use 24-bit true color ANSI codes
            output.push_str(&format!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, text));
        }
    }

    Some(output)
}

/// Highlight code and return as plain text with inline color markers
/// Format: [[rgb:RRGGBB]]text[[/rgb]]
pub fn highlight_with_markers(code: &str, extension: &str) -> Option<String> {
    let highlighted = highlight_code(code, extension)?;

    let mut output = String::new();

    for line in highlighted {
        for segment in line {
            let color_hex = format!(
                "{:02x}{:02x}{:02x}",
                segment.fg.0, segment.fg.1, segment.fg.2
            );
            output.push_str(&format!("[[rgb:{}]]{}[[/rgb]]", color_hex, segment.text));
        }
    }

    Some(output)
}

/// Get syntax name for a file extension
pub fn get_syntax_name(extension: &str) -> Option<String> {
    SYNTAX_SET
        .find_syntax_by_extension(extension)
        .map(|s| s.name.clone())
}

/// List all available syntax names
pub fn list_syntaxes() -> Vec<String> {
    SYNTAX_SET
        .syntaxes()
        .iter()
        .map(|s| s.name.clone())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rust_highlighting() {
        let code = r#"fn main() {
    println!("Hello, world!");
}
"#;
        let result = highlight_code(code, "rs");
        assert!(result.is_some());
        let lines = result.unwrap();
        assert!(!lines.is_empty());
    }

    #[test]
    fn test_is_supported() {
        assert!(is_supported("rs"));
        assert!(is_supported("py"));
        assert!(is_supported("js"));
    }

    #[test]
    fn test_get_syntax_name() {
        assert_eq!(get_syntax_name("rs"), Some("Rust".to_string()));
        assert_eq!(get_syntax_name("py"), Some("Python".to_string()));
    }
}
