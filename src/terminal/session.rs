//! Session persistence
//!
//! Saves and restores terminal sessions across restarts.

#![allow(dead_code)]

use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

/// Saved session data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedSession {
    /// Session name
    pub name: String,
    /// Tabs in this session
    pub tabs: Vec<SavedTab>,
    /// Active tab index
    pub active_tab: usize,
    /// When the session was last saved
    pub saved_at: u64,
}

/// Saved tab data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedTab {
    /// Tab title
    pub title: String,
    /// Working directory
    pub cwd: PathBuf,
    /// Command history (last N commands)
    pub history: Vec<String>,
    /// Scroll position (lines from top)
    pub scroll_position: usize,
}

/// Session manager for persistence
pub struct SessionManager {
    /// Directory to store session files
    session_dir: PathBuf,
    /// Maximum history entries to save per tab
    max_history: usize,
}

impl SessionManager {
    /// Create a new session manager
    pub fn new() -> Self {
        let session_dir = Self::default_session_dir();
        Self {
            session_dir,
            max_history: 100,
        }
    }

    /// Get the default session directory
    fn default_session_dir() -> PathBuf {
        dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("zaxiom")
            .join("sessions")
    }

    /// Ensure session directory exists
    fn ensure_dir(&self) -> std::io::Result<()> {
        fs::create_dir_all(&self.session_dir)
    }

    /// Save a session
    pub fn save_session(&self, session: &SavedSession) -> anyhow::Result<()> {
        self.ensure_dir()?;

        let filename = format!("{}.json", sanitize_filename(&session.name));
        let path = self.session_dir.join(filename);

        let json = serde_json::to_string_pretty(session)?;
        fs::write(path, json)?;

        Ok(())
    }

    /// Load a session by name
    pub fn load_session(&self, name: &str) -> anyhow::Result<SavedSession> {
        let filename = format!("{}.json", sanitize_filename(name));
        let path = self.session_dir.join(filename);

        let json = fs::read_to_string(path)?;
        let session: SavedSession = serde_json::from_str(&json)?;

        Ok(session)
    }

    /// Load the last session (most recently saved)
    pub fn load_last_session(&self) -> anyhow::Result<SavedSession> {
        self.ensure_dir()?;

        let mut sessions: Vec<_> = fs::read_dir(&self.session_dir)?
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map(|s| s == "json").unwrap_or(false))
            .collect();

        // Sort by modification time, most recent first
        sessions.sort_by_key(|e| {
            e.metadata()
                .and_then(|m| m.modified())
                .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
        });
        sessions.reverse();

        if let Some(entry) = sessions.first() {
            let json = fs::read_to_string(entry.path())?;
            let session: SavedSession = serde_json::from_str(&json)?;
            return Ok(session);
        }

        Err(anyhow::anyhow!("No saved sessions found"))
    }

    /// List all saved sessions
    pub fn list_sessions(&self) -> anyhow::Result<Vec<String>> {
        self.ensure_dir()?;

        let sessions: Vec<String> = fs::read_dir(&self.session_dir)?
            .filter_map(|e| e.ok())
            .filter_map(|e| {
                let path = e.path();
                if path.extension().map(|s| s == "json").unwrap_or(false) {
                    path.file_stem()
                        .map(|s| s.to_string_lossy().to_string())
                } else {
                    None
                }
            })
            .collect();

        Ok(sessions)
    }

    /// Delete a session
    pub fn delete_session(&self, name: &str) -> anyhow::Result<()> {
        let filename = format!("{}.json", sanitize_filename(name));
        let path = self.session_dir.join(filename);

        if path.exists() {
            fs::remove_file(path)?;
        }

        Ok(())
    }

    /// Save the current state as "autosave" for crash recovery
    pub fn autosave(&self, session: &SavedSession) -> anyhow::Result<()> {
        let mut autosave = session.clone();
        autosave.name = "autosave".to_string();
        self.save_session(&autosave)
    }

    /// Load autosave if it exists
    pub fn load_autosave(&self) -> anyhow::Result<SavedSession> {
        self.load_session("autosave")
    }

    /// Clear autosave after clean exit
    pub fn clear_autosave(&self) -> anyhow::Result<()> {
        self.delete_session("autosave")
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Sanitize a filename (remove invalid characters)
fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' || c == '.' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

impl SavedSession {
    /// Create a new session with a name
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            tabs: Vec::new(),
            active_tab: 0,
            saved_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
        }
    }

    /// Add a tab to the session
    pub fn add_tab(&mut self, tab: SavedTab) {
        self.tabs.push(tab);
    }
}

impl SavedTab {
    /// Create a new saved tab
    pub fn new(title: &str, cwd: PathBuf) -> Self {
        Self {
            title: title.to_string(),
            cwd,
            history: Vec::new(),
            scroll_position: 0,
        }
    }
}
