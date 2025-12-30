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
        // Try to resolve the full path to the executable for better TTY handling
        let cmd = {
            #[cfg(windows)]
            {
                // On Windows, try to find the actual executable
                // For .cmd/.bat files (like npm packages), we need special handling
                let resolved = Self::resolve_executable(program);

                match resolved {
                    Some((exe, wrapper_args)) => {
                        // Found the actual executable
                        let mut cmd = CommandBuilder::new(&exe);
                        for arg in &wrapper_args {
                            cmd.arg(arg);
                        }
                        for arg in args {
                            cmd.arg(arg);
                        }
                        cmd.cwd(cwd);
                        cmd
                    }
                    None => {
                        // Fallback: use PowerShell to run the command
                        let mut full_cmd = program.to_string();
                        for arg in args {
                            let escaped = arg.replace("'", "''");
                            full_cmd.push_str(&format!(" '{}'", escaped));
                        }
                        let mut cmd = CommandBuilder::new("powershell.exe");
                        cmd.arg("-NoLogo");
                        cmd.arg("-NoProfile");
                        cmd.arg("-Command");
                        cmd.arg(format!("& {{ {} }}", full_cmd));
                        cmd.cwd(cwd);
                        cmd
                    }
                }
            }

            #[cfg(not(windows))]
            {
                let mut cmd = CommandBuilder::new(program);
                for arg in args {
                    cmd.arg(arg);
                }
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

    /// Resolve a command to its actual executable
    /// For npm packages (.cmd files), extracts the node.exe + script path
    /// Returns (executable, args_to_prepend) or None if can't resolve
    #[cfg(windows)]
    fn resolve_executable(program: &str) -> Option<(String, Vec<String>)> {
        use std::process::Command;

        // Try to find the command using 'where'
        let output = Command::new("where").arg(program).output().ok()?;

        if !output.status.success() {
            return None;
        }

        let paths = String::from_utf8_lossy(&output.stdout);
        let first_path = paths.lines().next()?.trim();

        // If it's a .cmd or .bat file, parse it to find the actual executable
        if first_path.ends_with(".cmd") || first_path.ends_with(".bat") {
            let cmd_dir = std::path::Path::new(first_path).parent()?;

            // Read the batch file and look for the node invocation
            if let Ok(contents) = std::fs::read_to_string(first_path) {
                // npm packages typically have a pattern like:
                // "%_prog%" "%dp0%\node_modules\package\dist\index.js" %*
                // The %dp0% is the directory of the batch file

                for line in contents.lines() {
                    // Look for lines that contain a path to node_modules
                    if line.contains("node_modules") {
                        // Find the script path pattern: either %dp0%\... or just node_modules\...
                        // Pattern: "%dp0%\node_modules\pkg\dist\index.js"

                        // Try to extract the path between quotes after %dp0%
                        if let Some(dp0_idx) = line.find("%dp0%") {
                            let after_dp0 = &line[dp0_idx + 5..]; // Skip "%dp0%"
                                                                  // Skip leading backslash if present
                            let after_dp0 = after_dp0.strip_prefix('\\').unwrap_or(after_dp0);

                            // Find the end of the path (quote, space before %*, or end of line)
                            let end = after_dp0
                                .find('"')
                                .or_else(|| after_dp0.find(" %"))
                                .unwrap_or(after_dp0.len());
                            let script_rel = &after_dp0[..end];

                            // Construct full path
                            let script_path = cmd_dir.join(script_rel);
                            if script_path.exists() {
                                return Some((
                                    "node".to_string(),
                                    vec![script_path.to_string_lossy().to_string()],
                                ));
                            }
                        }
                    }
                }
            }
            // Couldn't parse .cmd file - return None to fall back to PowerShell
            None
        } else if first_path.ends_with(".exe") {
            // Direct executable - use it directly
            Some((first_path.to_string(), vec![]))
        } else {
            None
        }
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
