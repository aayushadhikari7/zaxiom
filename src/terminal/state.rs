//! Terminal state management
//!
//! Tracks current working directory, environment variables, and other state.

#![allow(dead_code)]

use std::collections::HashMap;
use std::env;
use std::path::PathBuf;

use crate::config::settings::Config;
use crate::config::theme::{icons, kawaii_icons, ThemeName};
use crate::git::prompt::get_git_branch;

/// Terminal state
pub struct TerminalState {
    /// Current working directory
    cwd: PathBuf,
    /// Home directory for ~ expansion
    home: PathBuf,
    /// User-defined aliases
    aliases: HashMap<String, String>,
    /// Previous directory for `cd -`
    prev_cwd: Option<PathBuf>,
    /// Requested theme change (checked by app after command execution)
    pub requested_theme: Option<ThemeName>,
    /// Current active theme
    pub current_theme: ThemeName,
    /// Kawaii mode - cuter UI elements
    pub kawaii_mode: bool,
}

impl TerminalState {
    /// Create new terminal state starting in home directory
    pub fn new() -> Self {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        // Start in home directory like a normal terminal
        let cwd = home.clone();
        // Set the process cwd as well
        let _ = env::set_current_dir(&cwd);

        // Load kawaii mode from config
        let config = Config::load();
        let kawaii_mode = config.kawaii_mode;

        Self {
            cwd,
            home,
            aliases: HashMap::new(),
            prev_cwd: None,
            requested_theme: None,
            current_theme: ThemeName::CatppuccinMocha, // Default theme
            kawaii_mode,
        }
    }

    /// Set kawaii mode and persist to config
    pub fn set_kawaii_mode(&mut self, enabled: bool) {
        self.kawaii_mode = enabled;
        // Persist to config
        let mut config = Config::load();
        let _ = config.set_kawaii_mode(enabled);
    }

    /// Get current working directory
    pub fn cwd(&self) -> &PathBuf {
        &self.cwd
    }

    /// Set current working directory
    pub fn set_cwd(&mut self, path: PathBuf) {
        // Store current as previous before changing
        self.prev_cwd = Some(self.cwd.clone());
        self.cwd = path;
        // Also update the actual process cwd
        let _ = env::set_current_dir(&self.cwd);
    }

    /// Get previous working directory
    pub fn prev_cwd(&self) -> Option<&PathBuf> {
        self.prev_cwd.as_ref()
    }

    /// Get home directory
    pub fn home(&self) -> &PathBuf {
        &self.home
    }

    /// Format prompt string with iTerm/Warp-style icons
    /// Format:  ~/path/to/dir  branch ❯  (or ♡ in kawaii mode)
    pub fn format_prompt(&self) -> String {
        let (icon, display_path) = self.format_path_display();
        let git_branch = get_git_branch(&self.cwd);

        // Use kawaii icons when kawaii mode is enabled
        let (prompt_icon, git_icon) = if self.kawaii_mode {
            (kawaii_icons::PROMPT, kawaii_icons::GIT_BRANCH)
        } else {
            (icons::PROMPT, icons::GIT_BRANCH)
        };

        match git_branch {
            Some(branch) => format!(
                "{} {}  {} {} ",
                icon,
                display_path,
                git_icon,
                branch
            ),
            None => format!("{} {} {} ", icon, display_path, prompt_icon),
        }
    }

    /// Format path for display (replace home with ~, use forward slashes)
    /// Returns (folder_icon, path_string)
    fn format_path_display(&self) -> (&'static str, String) {
        let path_str = self.cwd.to_string_lossy();
        let home_str = self.home.to_string_lossy();

        // Check if we're at home directory
        if path_str == home_str {
            return (icons::HOME, "~".to_string());
        }

        // Check if we're in a subdirectory of home
        if path_str.starts_with(home_str.as_ref()) {
            // Make sure it's actually a subdirectory (has a separator after home path)
            let rest = &path_str[home_str.len()..];
            if rest.starts_with('\\') || rest.starts_with('/') {
                let display = format!("~{}", rest.replace('\\', "/"));
                return (icons::FOLDER, display);
            }
        }

        // Check if we're at a drive root (e.g., C:\)
        let path_clean = path_str.replace('\\', "/");
        if path_clean.len() <= 3 && path_clean.ends_with(":/") || path_clean.ends_with(':') {
            // Drive root like "C:/" or "C:"
            let drive = path_clean.chars().next().unwrap_or('C');
            return (icons::ROOT, format!("{}:", drive));
        }

        // Check if we're at filesystem root
        if path_str == "/" || path_str == "\\" {
            return (icons::ROOT, "/".to_string());
        }

        // Regular path outside home - convert backslashes to forward slashes
        (icons::FOLDER, path_clean)
    }

    /// Resolve a path (handle ~, relative paths, /c/ style)
    pub fn resolve_path(&self, path: &str) -> PathBuf {
        let path = path.trim();

        // Handle empty path
        if path.is_empty() || path == "~" {
            return self.home.clone();
        }

        // Handle ~ prefix
        if path.starts_with("~/") {
            return self.home.join(&path[2..]);
        }

        // Handle Git Bash style /c/path -> C:\path
        if path.len() >= 3 && path.starts_with('/') && path.chars().nth(2) == Some('/') {
            let drive = path.chars().nth(1).unwrap().to_ascii_uppercase();
            let rest = &path[3..];
            return PathBuf::from(format!("{}:\\{}", drive, rest.replace('/', "\\")));
        }

        // Handle - (previous directory)
        if path == "-" {
            return self.prev_cwd.clone().unwrap_or_else(|| self.cwd.clone());
        }

        // Handle .. and .
        let path_buf = PathBuf::from(path.replace('/', "\\"));

        if path_buf.is_absolute() {
            path_buf
        } else {
            self.cwd.join(path_buf)
        }
    }

    /// Get an alias by name
    pub fn get_alias(&self, name: &str) -> Option<String> {
        self.aliases.get(name).cloned()
    }

    /// Set an alias
    pub fn set_alias(&mut self, name: String, value: String) {
        self.aliases.insert(name, value);
    }

    /// Remove an alias
    pub fn remove_alias(&mut self, name: &str) -> bool {
        self.aliases.remove(name).is_some()
    }

    /// List all aliases
    pub fn list_aliases(&self) -> Vec<(&String, &String)> {
        let mut aliases: Vec<_> = self.aliases.iter().collect();
        aliases.sort_by_key(|(name, _)| *name);
        aliases
    }
}

impl Default for TerminalState {
    fn default() -> Self {
        Self::new()
    }
}
