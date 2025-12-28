//! ANSI Escape Code Parser
//!
//! Parses ANSI escape codes and extracts styled text segments for rendering.

#![allow(dead_code)]

use regex::Regex;
use once_cell::sync::Lazy;

/// Regex to match ANSI SGR (color/style) escape codes
static ANSI_SGR_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\x1b\[([0-9;]*)m").unwrap()
});

/// Regex to match ALL ANSI escape sequences (CSI, OSC, etc.)
static ANSI_ALL_REGEX: Lazy<Regex> = Lazy::new(|| {
    // Matches:
    // - CSI sequences: \x1b[ ... (letter)
    // - OSC sequences: \x1b] ... (BEL or ST)
    // - Simple escapes: \x1b followed by single char
    // - DCS/PM/APC: \x1b P/^/_ ... ST
    Regex::new(concat!(
        r"\x1b\[[0-9;?]*[A-Za-z~]",   // CSI sequences (cursor, erase, etc.)
        r"|\x1b\][^\x07\x1b]*(?:\x07|\x1b\\)?",  // OSC sequences
        r"|\x1b[PX^_][^\x1b]*\x1b\\",  // DCS/PM/APC sequences
        r"|\x1b[NO].",                 // SS2/SS3
        r"|\x1b[78]",                  // Save/restore cursor
        r"|\x1b[=>c]",                 // Keypad/charset modes
        r"|\x1b[A-Za-z]",              // Simple escape sequences
    )).unwrap()
});

/// A text segment with optional ANSI styling
#[derive(Clone, Debug)]
pub struct StyledSegment {
    /// The text content
    pub text: String,
    /// Foreground color as RGB (None = default)
    pub fg_color: Option<(u8, u8, u8)>,
    /// Background color as RGB (None = default)
    pub bg_color: Option<(u8, u8, u8)>,
    /// Whether the text is bold
    pub bold: bool,
    /// Whether the text is italic
    pub italic: bool,
    /// Whether the text is underlined
    pub underline: bool,
}

impl Default for StyledSegment {
    fn default() -> Self {
        Self {
            text: String::new(),
            fg_color: None,
            bg_color: None,
            bold: false,
            italic: false,
            underline: false,
        }
    }
}

/// Current styling state while parsing
#[derive(Clone, Debug, Default)]
struct StyleState {
    fg_color: Option<(u8, u8, u8)>,
    bg_color: Option<(u8, u8, u8)>,
    bold: bool,
    italic: bool,
    underline: bool,
}

impl StyleState {
    fn reset(&mut self) {
        *self = Self::default();
    }

    fn to_segment(&self, text: String) -> StyledSegment {
        StyledSegment {
            text,
            fg_color: self.fg_color,
            bg_color: self.bg_color,
            bold: self.bold,
            italic: self.italic,
            underline: self.underline,
        }
    }
}

/// Parse a line of text containing ANSI codes into styled segments
pub fn parse_ansi(text: &str) -> Vec<StyledSegment> {
    let mut segments = Vec::new();
    let mut state = StyleState::default();
    let mut last_end = 0;

    for caps in ANSI_SGR_REGEX.captures_iter(text) {
        let whole_match = caps.get(0).unwrap();
        let codes_str = caps.get(1).map(|m| m.as_str()).unwrap_or("");

        // Add text before this escape sequence
        if whole_match.start() > last_end {
            let segment_text = &text[last_end..whole_match.start()];
            if !segment_text.is_empty() {
                segments.push(state.to_segment(segment_text.to_string()));
            }
        }

        // Parse the ANSI codes
        parse_codes(codes_str, &mut state);
        last_end = whole_match.end();
    }

    // Add remaining text
    if last_end < text.len() {
        let remaining = &text[last_end..];
        if !remaining.is_empty() {
            segments.push(state.to_segment(remaining.to_string()));
        }
    }

    // If no ANSI codes found, return the whole text as a single segment
    if segments.is_empty() && !text.is_empty() {
        segments.push(StyledSegment {
            text: text.to_string(),
            ..Default::default()
        });
    }

    segments
}

/// Parse ANSI code parameters and update state
fn parse_codes(codes: &str, state: &mut StyleState) {
    if codes.is_empty() || codes == "0" {
        state.reset();
        return;
    }

    let parts: Vec<u8> = codes
        .split(';')
        .filter_map(|s| s.parse().ok())
        .collect();

    let mut i = 0;
    while i < parts.len() {
        match parts[i] {
            0 => state.reset(),
            1 => state.bold = true,
            3 => state.italic = true,
            4 => state.underline = true,
            22 => state.bold = false,
            23 => state.italic = false,
            24 => state.underline = false,

            // Standard foreground colors (30-37)
            30 => state.fg_color = Some((0, 0, 0)),       // Black
            31 => state.fg_color = Some((205, 49, 49)),   // Red
            32 => state.fg_color = Some((13, 188, 121)),  // Green
            33 => state.fg_color = Some((229, 229, 16)),  // Yellow
            34 => state.fg_color = Some((36, 114, 200)),  // Blue
            35 => state.fg_color = Some((188, 63, 188)),  // Magenta
            36 => state.fg_color = Some((17, 168, 205)),  // Cyan
            37 => state.fg_color = Some((229, 229, 229)), // White
            39 => state.fg_color = None,                  // Default

            // Bright foreground colors (90-97)
            90 => state.fg_color = Some((102, 102, 102)), // Bright Black
            91 => state.fg_color = Some((241, 76, 76)),   // Bright Red
            92 => state.fg_color = Some((35, 209, 139)),  // Bright Green
            93 => state.fg_color = Some((245, 245, 67)),  // Bright Yellow
            94 => state.fg_color = Some((59, 142, 234)),  // Bright Blue
            95 => state.fg_color = Some((214, 112, 214)), // Bright Magenta
            96 => state.fg_color = Some((41, 184, 219)),  // Bright Cyan
            97 => state.fg_color = Some((255, 255, 255)), // Bright White

            // 256-color and 24-bit color (38;2;r;g;b or 38;5;n)
            38 => {
                if i + 1 < parts.len() {
                    match parts[i + 1] {
                        2 if i + 4 < parts.len() => {
                            // 24-bit color
                            state.fg_color = Some((parts[i + 2], parts[i + 3], parts[i + 4]));
                            i += 4;
                        }
                        5 if i + 2 < parts.len() => {
                            // 256-color palette
                            state.fg_color = Some(color_256_to_rgb(parts[i + 2]));
                            i += 2;
                        }
                        _ => {}
                    }
                }
            }

            // Standard background colors (40-47)
            40 => state.bg_color = Some((0, 0, 0)),
            41 => state.bg_color = Some((205, 49, 49)),
            42 => state.bg_color = Some((13, 188, 121)),
            43 => state.bg_color = Some((229, 229, 16)),
            44 => state.bg_color = Some((36, 114, 200)),
            45 => state.bg_color = Some((188, 63, 188)),
            46 => state.bg_color = Some((17, 168, 205)),
            47 => state.bg_color = Some((229, 229, 229)),
            49 => state.bg_color = None,

            // 256-color and 24-bit background (48;2;r;g;b or 48;5;n)
            48 => {
                if i + 1 < parts.len() {
                    match parts[i + 1] {
                        2 if i + 4 < parts.len() => {
                            state.bg_color = Some((parts[i + 2], parts[i + 3], parts[i + 4]));
                            i += 4;
                        }
                        5 if i + 2 < parts.len() => {
                            state.bg_color = Some(color_256_to_rgb(parts[i + 2]));
                            i += 2;
                        }
                        _ => {}
                    }
                }
            }

            _ => {}
        }
        i += 1;
    }
}

/// Convert 256-color palette index to RGB
fn color_256_to_rgb(n: u8) -> (u8, u8, u8) {
    match n {
        // Standard colors (0-15)
        0 => (0, 0, 0),
        1 => (128, 0, 0),
        2 => (0, 128, 0),
        3 => (128, 128, 0),
        4 => (0, 0, 128),
        5 => (128, 0, 128),
        6 => (0, 128, 128),
        7 => (192, 192, 192),
        8 => (128, 128, 128),
        9 => (255, 0, 0),
        10 => (0, 255, 0),
        11 => (255, 255, 0),
        12 => (0, 0, 255),
        13 => (255, 0, 255),
        14 => (0, 255, 255),
        15 => (255, 255, 255),

        // 216-color cube (16-231)
        16..=231 => {
            let n = n - 16;
            let r = (n / 36) % 6;
            let g = (n / 6) % 6;
            let b = n % 6;
            (
                if r > 0 { 55 + r * 40 } else { 0 },
                if g > 0 { 55 + g * 40 } else { 0 },
                if b > 0 { 55 + b * 40 } else { 0 },
            )
        }

        // Grayscale (232-255)
        232..=255 => {
            let gray = 8 + (n - 232) * 10;
            (gray, gray, gray)
        }
    }
}

/// Strip all ANSI escape sequences from text (comprehensive)
pub fn strip_ansi(text: &str) -> String {
    ANSI_ALL_REGEX.replace_all(text, "").to_string()
}

/// Strip only SGR (color/style) codes, keep other sequences
pub fn strip_ansi_colors(text: &str) -> String {
    ANSI_SGR_REGEX.replace_all(text, "").to_string()
}

/// Check if text contains ANSI codes
pub fn has_ansi(text: &str) -> bool {
    ANSI_ALL_REGEX.is_match(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_colors() {
        let text = "\x1b[31mRed\x1b[0m Normal";
        let segments = parse_ansi(text);
        assert_eq!(segments.len(), 2);
        assert_eq!(segments[0].text, "Red");
        assert_eq!(segments[0].fg_color, Some((205, 49, 49)));
        assert_eq!(segments[1].text, " Normal");
        assert_eq!(segments[1].fg_color, None);
    }

    #[test]
    fn test_24bit_colors() {
        let text = "\x1b[38;2;255;128;64mOrange\x1b[0m";
        let segments = parse_ansi(text);
        assert_eq!(segments.len(), 1);
        assert_eq!(segments[0].fg_color, Some((255, 128, 64)));
    }

    #[test]
    fn test_strip_ansi() {
        let text = "\x1b[31mRed\x1b[0m and \x1b[32mGreen\x1b[0m";
        assert_eq!(strip_ansi(text), "Red and Green");
    }

    #[test]
    fn test_no_ansi() {
        let text = "Plain text without colors";
        let segments = parse_ansi(text);
        assert_eq!(segments.len(), 1);
        assert_eq!(segments[0].text, text);
        assert_eq!(segments[0].fg_color, None);
    }
}
