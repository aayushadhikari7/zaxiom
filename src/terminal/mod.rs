//! Terminal emulator layer
//!
//! Handles terminal state, output buffering, input, and rendering.

pub mod ansi;
pub mod autocomplete;
pub mod buffer;
pub mod format;
pub mod fuzzy;
pub mod hints;
pub mod history;
pub mod img;
pub mod input;
pub mod palette;
pub mod project;
pub mod render;
pub mod session;
pub mod smart_history;
pub mod split;
pub mod state;
pub mod syntax;
pub mod vi_mode;
