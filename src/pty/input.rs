//! PTY input handling
//!
//! Converts keyboard input to PTY escape sequences.

use egui::Key;

/// Input mode for the terminal
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputMode {
    /// Normal line editing mode - input goes to TextEdit
    #[default]
    Normal,
    /// Raw mode - keypresses go directly to PTY
    Raw,
}

/// Convert an egui key event to PTY bytes
pub fn key_to_bytes(key: Key, modifiers: &egui::Modifiers) -> Option<Vec<u8>> {
    // Handle Ctrl combinations first
    if modifiers.ctrl && !modifiers.alt {
        return ctrl_key_to_bytes(key);
    }

    // Handle Alt combinations
    if modifiers.alt && !modifiers.ctrl {
        return alt_key_to_bytes(key);
    }

    // Regular keys
    match key {
        Key::Enter => Some(vec![b'\r']),
        Key::Backspace => Some(vec![0x7f]),
        Key::Delete => Some(vec![0x1b, b'[', b'3', b'~']),
        Key::Escape => Some(vec![0x1b]),
        Key::Tab => Some(vec![b'\t']),

        // Arrow keys
        Key::ArrowUp => Some(vec![0x1b, b'[', b'A']),
        Key::ArrowDown => Some(vec![0x1b, b'[', b'B']),
        Key::ArrowRight => Some(vec![0x1b, b'[', b'C']),
        Key::ArrowLeft => Some(vec![0x1b, b'[', b'D']),

        // Navigation keys
        Key::Home => Some(vec![0x1b, b'[', b'H']),
        Key::End => Some(vec![0x1b, b'[', b'F']),
        Key::PageUp => Some(vec![0x1b, b'[', b'5', b'~']),
        Key::PageDown => Some(vec![0x1b, b'[', b'6', b'~']),
        Key::Insert => Some(vec![0x1b, b'[', b'2', b'~']),

        // Function keys
        Key::F1 => Some(vec![0x1b, b'O', b'P']),
        Key::F2 => Some(vec![0x1b, b'O', b'Q']),
        Key::F3 => Some(vec![0x1b, b'O', b'R']),
        Key::F4 => Some(vec![0x1b, b'O', b'S']),
        Key::F5 => Some(vec![0x1b, b'[', b'1', b'5', b'~']),
        Key::F6 => Some(vec![0x1b, b'[', b'1', b'7', b'~']),
        Key::F7 => Some(vec![0x1b, b'[', b'1', b'8', b'~']),
        Key::F8 => Some(vec![0x1b, b'[', b'1', b'9', b'~']),
        Key::F9 => Some(vec![0x1b, b'[', b'2', b'0', b'~']),
        Key::F10 => Some(vec![0x1b, b'[', b'2', b'1', b'~']),
        Key::F11 => Some(vec![0x1b, b'[', b'2', b'3', b'~']),
        Key::F12 => Some(vec![0x1b, b'[', b'2', b'4', b'~']),

        _ => None,
    }
}

/// Convert Ctrl+key combinations to control codes
fn ctrl_key_to_bytes(key: Key) -> Option<Vec<u8>> {
    match key {
        // Ctrl+A through Ctrl+Z map to 0x01-0x1A
        Key::A => Some(vec![0x01]),
        Key::B => Some(vec![0x02]),
        Key::C => Some(vec![0x03]), // SIGINT
        Key::D => Some(vec![0x04]), // EOF
        Key::E => Some(vec![0x05]),
        Key::F => Some(vec![0x06]),
        Key::G => Some(vec![0x07]), // Bell
        Key::H => Some(vec![0x08]), // Backspace
        Key::I => Some(vec![0x09]), // Tab
        Key::J => Some(vec![0x0A]), // Newline
        Key::K => Some(vec![0x0B]),
        Key::L => Some(vec![0x0C]), // Form feed (clear screen)
        Key::M => Some(vec![0x0D]), // Carriage return
        Key::N => Some(vec![0x0E]),
        Key::O => Some(vec![0x0F]),
        Key::P => Some(vec![0x10]),
        Key::Q => Some(vec![0x11]), // XON
        Key::R => Some(vec![0x12]),
        Key::S => Some(vec![0x13]), // XOFF
        Key::T => Some(vec![0x14]),
        Key::U => Some(vec![0x15]), // Kill line
        Key::V => Some(vec![0x16]),
        Key::W => Some(vec![0x17]), // Kill word
        Key::X => Some(vec![0x18]),
        Key::Y => Some(vec![0x19]),
        Key::Z => Some(vec![0x1A]), // SIGTSTP (suspend)

        // Ctrl+[ is Escape
        Key::OpenBracket => Some(vec![0x1b]),
        // Ctrl+\ is SIGQUIT
        Key::Backslash => Some(vec![0x1c]),
        // Ctrl+] is 0x1D
        Key::CloseBracket => Some(vec![0x1d]),

        _ => None,
    }
}

/// Convert Alt+key combinations to escape sequences
fn alt_key_to_bytes(key: Key) -> Option<Vec<u8>> {
    // Alt+key sends ESC followed by the key
    match key {
        Key::A => Some(vec![0x1b, b'a']),
        Key::B => Some(vec![0x1b, b'b']),
        Key::C => Some(vec![0x1b, b'c']),
        Key::D => Some(vec![0x1b, b'd']),
        Key::E => Some(vec![0x1b, b'e']),
        Key::F => Some(vec![0x1b, b'f']),
        Key::G => Some(vec![0x1b, b'g']),
        Key::H => Some(vec![0x1b, b'h']),
        Key::I => Some(vec![0x1b, b'i']),
        Key::J => Some(vec![0x1b, b'j']),
        Key::K => Some(vec![0x1b, b'k']),
        Key::L => Some(vec![0x1b, b'l']),
        Key::M => Some(vec![0x1b, b'm']),
        Key::N => Some(vec![0x1b, b'n']),
        Key::O => Some(vec![0x1b, b'o']),
        Key::P => Some(vec![0x1b, b'p']),
        Key::Q => Some(vec![0x1b, b'q']),
        Key::R => Some(vec![0x1b, b'r']),
        Key::S => Some(vec![0x1b, b's']),
        Key::T => Some(vec![0x1b, b't']),
        Key::U => Some(vec![0x1b, b'u']),
        Key::V => Some(vec![0x1b, b'v']),
        Key::W => Some(vec![0x1b, b'w']),
        Key::X => Some(vec![0x1b, b'x']),
        Key::Y => Some(vec![0x1b, b'y']),
        Key::Z => Some(vec![0x1b, b'z']),
        _ => None,
    }
}

/// Convert a character to PTY bytes
pub fn char_to_bytes(ch: char) -> Vec<u8> {
    let mut buf = [0u8; 4];
    let s = ch.encode_utf8(&mut buf);
    s.as_bytes().to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ctrl_c() {
        let mods = egui::Modifiers {
            ctrl: true,
            ..Default::default()
        };
        assert_eq!(key_to_bytes(Key::C, &mods), Some(vec![0x03]));
    }

    #[test]
    fn test_arrow_keys() {
        let mods = egui::Modifiers::default();
        assert_eq!(
            key_to_bytes(Key::ArrowUp, &mods),
            Some(vec![0x1b, b'[', b'A'])
        );
        assert_eq!(
            key_to_bytes(Key::ArrowDown, &mods),
            Some(vec![0x1b, b'[', b'B'])
        );
    }

    #[test]
    fn test_enter() {
        let mods = egui::Modifiers::default();
        assert_eq!(key_to_bytes(Key::Enter, &mods), Some(vec![b'\r']));
    }
}
