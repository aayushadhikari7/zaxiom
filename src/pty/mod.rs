//! PTY (Pseudo-Terminal) support for Zaxiom
//!
//! Uses portable-pty for cross-platform PTY support.
//! On Windows, this uses ConPTY for full interactive terminal support.

pub mod buffer;
pub mod grid;
pub mod input;
pub mod session;

pub use buffer::PtyBuffer;
pub use grid::TerminalGrid;
pub use input::InputMode;
pub use session::{PtyOutput, PtySession};
