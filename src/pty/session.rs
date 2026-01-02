//! PTY session management
//!
//! Handles spawning and communicating with a PTY process (PowerShell on Windows).

use anyhow::{Context, Result};
use portable_pty::{native_pty_system, Child, CommandBuilder, MasterPty, PtySize};
use std::io::{Read, Write};
use std::path::Path;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{self, JoinHandle};

/// Output from the PTY process
#[derive(Debug)]
pub enum PtyOutput {
    /// Regular output data
    Data(Vec<u8>),
    /// Process exited with optional exit code
    Exited(Option<u32>),
    /// Error occurred
    Error(String),
}

/// A PTY session connected to a shell process
pub struct PtySession {
    /// The master PTY handle for resizing
    master: Box<dyn MasterPty + Send>,
    /// Child process handle
    child: Box<dyn Child + Send + Sync>,
    /// Writer to send input to PTY
    writer: Box<dyn Write + Send>,
    /// Channel receiver for output from reader thread
    output_rx: Receiver<PtyOutput>,
    /// Sender to signal shutdown to reader thread
    _shutdown_tx: Sender<()>,
    /// Handle to reader thread (for cleanup)
    _reader_handle: JoinHandle<()>,
    /// Current terminal size
    size: PtySize,
}

impl PtySession {
    /// Create a new PTY session with PowerShell
    #[allow(dead_code)]
    pub fn new(rows: u16, cols: u16, cwd: &Path) -> Result<Self> {
        Self::new_with_command("powershell.exe", &[], rows, cols, cwd)
    }

    /// Create a new PTY session with a specific command and arguments
    pub fn new_with_command(
        program: &str,
        args: &[String],
        rows: u16,
        cols: u16,
        cwd: &Path,
    ) -> Result<Self> {
        let size = PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        };

        // Get the native PTY system (ConPTY on Windows)
        let pty_system = native_pty_system();

        // Open a new PTY with the specified size
        let pair = pty_system.openpty(size).context("Failed to open PTY")?;

        // Build the command to spawn
        // Run commands directly via cmd.exe /c for proper PATH resolution and TTY handling
        let cmd = {
            #[cfg(windows)]
            {
                // Use cmd.exe /c to run the command - this:
                // 1. Properly resolves PATH (handles .exe, .cmd, .bat, etc.)
                // 2. Inherits the ConPTY properly for TTY detection
                // 3. Is fast (no extra process spawning for 'where' lookup)
                let mut full_command = program.to_string();
                for arg in args {
                    // Quote arguments that contain spaces
                    if arg.contains(' ') || arg.contains('"') {
                        full_command.push_str(&format!(" \"{}\"", arg.replace('"', "\\\"")));
                    } else {
                        full_command.push_str(&format!(" {}", arg));
                    }
                }

                let mut cmd = CommandBuilder::new("cmd.exe");
                cmd.arg("/c");
                cmd.arg(&full_command);

                // Set environment variables for proper TTY detection
                cmd.env("TERM", "xterm-256color");
                cmd.env("COLORTERM", "truecolor");
                cmd.env("FORCE_COLOR", "1");
                cmd.env("WT_SESSION", "1");

                cmd.cwd(cwd);
                cmd
            }

            #[cfg(not(windows))]
            {
                let mut cmd = CommandBuilder::new(program);
                for arg in args {
                    cmd.arg(arg);
                }
                cmd.env("TERM", "xterm-256color");
                cmd.env("COLORTERM", "truecolor");
                cmd.cwd(cwd);
                cmd
            }
        };

        // Spawn the process directly attached to PTY
        let child = pair
            .slave
            .spawn_command(cmd)
            .context("Failed to spawn process")?;

        // Get the reader and writer
        let reader = pair
            .master
            .try_clone_reader()
            .context("Failed to clone PTY reader")?;
        let writer = pair
            .master
            .take_writer()
            .context("Failed to take PTY writer")?;

        // Create channels for communication
        let (output_tx, output_rx) = mpsc::channel();
        let (shutdown_tx, shutdown_rx) = mpsc::channel();

        // Spawn the reader thread
        let reader_handle = thread::spawn(move || {
            Self::reader_loop(reader, output_tx, shutdown_rx);
        });

        Ok(Self {
            master: pair.master,
            child,
            writer,
            output_rx,
            _shutdown_tx: shutdown_tx,
            _reader_handle: reader_handle,
            size,
        })
    }

    /// Reader loop that runs in a separate thread
    fn reader_loop(
        mut reader: Box<dyn Read + Send>,
        tx: Sender<PtyOutput>,
        shutdown_rx: Receiver<()>,
    ) {
        let mut buf = [0u8; 4096];

        loop {
            // Check for shutdown signal (non-blocking)
            if shutdown_rx.try_recv().is_ok() {
                break;
            }

            // Try to read from PTY
            match reader.read(&mut buf) {
                Ok(0) => {
                    // EOF - process exited
                    let _ = tx.send(PtyOutput::Exited(None));
                    break;
                }
                Ok(n) => {
                    // Send the data
                    if tx.send(PtyOutput::Data(buf[..n].to_vec())).is_err() {
                        // Receiver dropped, exit
                        break;
                    }
                }
                Err(e) => {
                    // Check if it's a "would block" error (expected for non-blocking reads)
                    if e.kind() == std::io::ErrorKind::WouldBlock {
                        // Sleep briefly to avoid busy-waiting
                        thread::sleep(std::time::Duration::from_millis(10));
                        continue;
                    }
                    // Send error and exit
                    let _ = tx.send(PtyOutput::Error(e.to_string()));
                    break;
                }
            }
        }
    }

    /// Write input to the PTY
    pub fn write(&mut self, data: &[u8]) -> Result<()> {
        self.writer
            .write_all(data)
            .context("Failed to write to PTY")?;
        self.writer.flush().context("Failed to flush PTY writer")?;
        Ok(())
    }

    /// Write a string to the PTY
    #[allow(dead_code)]
    pub fn write_str(&mut self, s: &str) -> Result<()> {
        self.write(s.as_bytes())
    }

    /// Send a command to the PTY (appends \r\n)
    #[allow(dead_code)]
    pub fn send_command(&mut self, cmd: &str) -> Result<()> {
        self.write_str(&format!("{}\r\n", cmd))
    }

    /// Try to receive output (non-blocking)
    pub fn try_recv(&self) -> Option<PtyOutput> {
        self.output_rx.try_recv().ok()
    }

    /// Resize the PTY
    pub fn resize(&self, rows: u16, cols: u16) -> Result<()> {
        let new_size = PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        };
        self.master
            .resize(new_size)
            .context("Failed to resize PTY")?;
        Ok(())
    }

    /// Get the current size
    pub fn size(&self) -> (u16, u16) {
        (self.size.rows, self.size.cols)
    }

    /// Check if the child process is still running
    #[allow(dead_code)]
    pub fn is_running(&mut self) -> bool {
        self.child.try_wait().ok().flatten().is_none()
    }

    /// Kill the child process
    pub fn kill(&mut self) -> Result<()> {
        self.child.kill().context("Failed to kill child process")
    }

}

impl Drop for PtySession {
    fn drop(&mut self) {
        // Try to kill the child process gracefully
        let _ = self.kill();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_pty_session_creation() {
        let cwd = env::current_dir().unwrap();
        let session = PtySession::new(24, 80, &cwd);
        assert!(session.is_ok(), "Failed to create PTY session");
    }
}
