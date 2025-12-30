//! matrix command - display Matrix-style digital rain
//!
//! The Matrix has you...

use anyhow::Result;

use crate::commands::traits::Command;
use crate::terminal::state::TerminalState;

pub struct MatrixCommand;

impl Command for MatrixCommand {
    fn name(&self) -> &'static str {
        "matrix"
    }

    fn description(&self) -> &'static str {
        "Display Matrix-style digital rain"
    }

    fn usage(&self) -> &'static str {
        "matrix"
    }

    fn execute(&self, args: &[String], _state: &mut TerminalState) -> Result<String> {
        for arg in args {
            if arg == "-h" || arg == "--help" {
                return Ok("Usage: matrix\n\
                    Display Matrix-style digital rain ASCII art.\n\
                    Wake up, Neo..."
                    .to_string());
            }
        }

        // Generate some matrix-style output
        let matrix_art = generate_matrix_frame();

        Ok(format!("{}\n\n  Wake up, Neo...\n  The Matrix has you...\n  Follow the white rabbit.\n\n  Knock, knock, Neo.", matrix_art))
    }
}

fn generate_matrix_frame() -> String {
    // Matrix-style characters (katakana-like symbols + numbers)
    let chars: &[char] = &[
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'ア', 'イ', 'ウ', 'エ', 'オ', 'カ', 'キ',
        'ク', 'ケ', 'コ', 'サ', 'シ', 'ス', 'セ', 'ソ', 'タ', 'チ', 'ツ', 'テ', 'ト', 'ナ', 'ニ',
        'ヌ', 'ネ', 'ノ', 'ハ', 'ヒ', 'フ', 'ヘ', 'ホ', '@', '#', '$', '%', '&', '*', '!', '?',
        '+', '=',
    ];

    let width = 60;
    let height = 15;
    let mut lines = Vec::new();

    // Use time-based seed for variety
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    let seed = now.as_nanos() as usize;

    for row in 0..height {
        let mut line = String::new();
        for col in 0..width {
            // Pseudo-random character selection
            let idx = (seed + row * 17 + col * 31) % chars.len();

            // Create "falling" effect with varying density
            let density = ((seed + col * 7) % 10) as i32;
            let row_offset = ((seed / 1000 + col * 3) % height) as i32;
            let dist = ((row as i32) - row_offset).abs();

            if dist < density / 2 {
                line.push(chars[idx]);
            } else if dist < density {
                // Use dots for dimmer effect
                line.push('.');
            } else {
                line.push(' ');
            }
        }
        lines.push(line);
    }

    lines.join("\n")
}
