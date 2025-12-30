//! Command Palette
//!
//! Ctrl+P quick access to all commands, similar to VS Code

#![allow(dead_code)]

use crate::commands::registry::CommandRegistry;

/// A command palette entry
#[derive(Clone)]
pub struct PaletteEntry {
    /// Command name
    pub name: String,
    /// Command description
    pub description: String,
    /// Category (nav, files, text, etc.)
    pub category: String,
    /// Keyboard shortcut (if any)
    pub shortcut: Option<String>,
    /// Match score for fuzzy search
    pub score: i32,
}

/// Command palette state
pub struct CommandPalette {
    /// Whether the palette is open
    pub is_open: bool,
    /// Current search query
    pub query: String,
    /// Filtered entries
    pub entries: Vec<PaletteEntry>,
    /// All available entries
    all_entries: Vec<PaletteEntry>,
    /// Selected index
    pub selected: usize,
}

impl CommandPalette {
    /// Create a new command palette
    pub fn new() -> Self {
        let all_entries = Self::build_entries();
        Self {
            is_open: false,
            query: String::new(),
            entries: all_entries.clone(),
            all_entries,
            selected: 0,
        }
    }

    /// Build all palette entries from the command registry
    fn build_entries() -> Vec<PaletteEntry> {
        let registry = CommandRegistry::new();
        let mut entries: Vec<PaletteEntry> = registry
            .list()
            .into_iter()
            .map(|(name, desc)| {
                let category = Self::categorize_command(name);
                PaletteEntry {
                    name: name.to_string(),
                    description: desc.to_string(),
                    category,
                    shortcut: Self::get_shortcut(name),
                    score: 0,
                }
            })
            .collect();

        // Add special actions
        entries.push(PaletteEntry {
            name: "New Tab".to_string(),
            description: "Open a new terminal tab".to_string(),
            category: "Actions".to_string(),
            shortcut: Some("Ctrl+T".to_string()),
            score: 0,
        });
        entries.push(PaletteEntry {
            name: "Close Tab".to_string(),
            description: "Close current terminal tab".to_string(),
            category: "Actions".to_string(),
            shortcut: Some("Ctrl+W".to_string()),
            score: 0,
        });
        entries.push(PaletteEntry {
            name: "Search".to_string(),
            description: "Search in terminal output".to_string(),
            category: "Actions".to_string(),
            shortcut: Some("Ctrl+F".to_string()),
            score: 0,
        });
        entries.push(PaletteEntry {
            name: "Clear".to_string(),
            description: "Clear the terminal screen".to_string(),
            category: "Actions".to_string(),
            shortcut: Some("Ctrl+L".to_string()),
            score: 0,
        });

        // AI-related actions
        entries.push(PaletteEntry {
            name: "ai status".to_string(),
            description: "Show AI provider status and configuration".to_string(),
            category: "AI".to_string(),
            shortcut: None,
            score: 0,
        });
        entries.push(PaletteEntry {
            name: "ai providers".to_string(),
            description: "List all available AI providers".to_string(),
            category: "AI".to_string(),
            shortcut: None,
            score: 0,
        });
        entries.push(PaletteEntry {
            name: "ollama list".to_string(),
            description: "List installed Ollama models".to_string(),
            category: "AI".to_string(),
            shortcut: None,
            score: 0,
        });
        entries.push(PaletteEntry {
            name: "ollama status".to_string(),
            description: "Check if Ollama server is running".to_string(),
            category: "AI".to_string(),
            shortcut: None,
            score: 0,
        });
        entries.push(PaletteEntry {
            name: "ollama models".to_string(),
            description: "Show recommended Ollama models to download".to_string(),
            category: "AI".to_string(),
            shortcut: None,
            score: 0,
        });
        entries.push(PaletteEntry {
            name: "ollama serve".to_string(),
            description: "Start the Ollama server".to_string(),
            category: "AI".to_string(),
            shortcut: None,
            score: 0,
        });

        // Split pane actions
        entries.push(PaletteEntry {
            name: "Split Horizontal".to_string(),
            description: "Split current pane horizontally".to_string(),
            category: "Actions".to_string(),
            shortcut: Some("Ctrl+Shift+D".to_string()),
            score: 0,
        });
        entries.push(PaletteEntry {
            name: "Split Vertical".to_string(),
            description: "Split current pane vertically".to_string(),
            category: "Actions".to_string(),
            shortcut: Some("Ctrl+Shift+E".to_string()),
            score: 0,
        });
        entries.push(PaletteEntry {
            name: "Vi Mode".to_string(),
            description: "Toggle vim-style navigation mode".to_string(),
            category: "Actions".to_string(),
            shortcut: Some("Ctrl+Shift+M".to_string()),
            score: 0,
        });
        entries.push(PaletteEntry {
            name: "Hints Mode".to_string(),
            description: "Extract URLs and paths from output".to_string(),
            category: "Actions".to_string(),
            shortcut: Some("Ctrl+Shift+H".to_string()),
            score: 0,
        });
        entries.push(PaletteEntry {
            name: "History Search".to_string(),
            description: "Fuzzy search command history".to_string(),
            category: "Actions".to_string(),
            shortcut: Some("Ctrl+R".to_string()),
            score: 0,
        });

        entries.sort_by(|a, b| a.name.cmp(&b.name));
        entries
    }

    /// Get keyboard shortcut for a command
    fn get_shortcut(name: &str) -> Option<String> {
        match name {
            "clear" => Some("Ctrl+L".to_string()),
            "exit" => Some("Ctrl+D".to_string()),
            _ => None,
        }
    }

    /// Categorize a command
    fn categorize_command(name: &str) -> String {
        match name {
            "ls" | "cd" | "pwd" | "tree" | "clear" | "help" => "Navigation",
            "cat" | "touch" | "rm" | "mkdir" | "cp" | "mv" | "ln" | "stat" | "file" | "chmod"
            | "readlink" | "mktemp" | "nano" | "vim" | "vi" | "edit" => "Files",
            "echo" | "head" | "tail" | "wc" | "sort" | "uniq" | "grep" | "find" | "cut"
            | "paste" | "diff" | "tr" | "sed" | "awk" | "rev" | "nl" | "printf" | "xargs"
            | "column" | "strings" | "split" | "join" | "comm" => "Text",
            "exit" | "which" | "du" | "df" | "ps" | "kill" | "whoami" | "hostname" | "uname"
            | "uptime" | "free" | "date" | "cal" | "id" | "neofetch" | "printenv" | "lscpu"
            | "history" | "test" | "man" | "theme" => "System",
            "curl" | "wget" | "ping" | "netstat" | "traceroute" | "nslookup" | "host"
            | "ifconfig" => "Network",
            "md5sum" | "sha1sum" | "sha224sum" | "sha256sum" | "sha384sum" | "sha512sum"
            | "blake3sum" | "b3sum" | "crc32" | "base64" | "xxd" => "Hash",
            "tar" | "zip" | "unzip" | "gzip" | "gunzip" => "Compress",
            "alias" | "env" | "export" | "sleep" | "seq" | "yes" | "true" | "false" | "expr"
            | "bc" | "tee" | "timeout" | "type" | "command" | "pushd" | "popd" | "dirs" => "Shell",
            "fortune" | "cowsay" | "coffee" | "matrix" | "pet" => "Fun",
            "ai" | "ollama" => "AI",
            _ => "Other",
        }
        .to_string()
    }

    /// Toggle the palette
    pub fn toggle(&mut self) {
        self.is_open = !self.is_open;
        if self.is_open {
            self.query.clear();
            self.entries = self.all_entries.clone();
            self.selected = 0;
        }
    }

    /// Open the palette
    pub fn open(&mut self) {
        self.is_open = true;
        self.query.clear();
        self.entries = self.all_entries.clone();
        self.selected = 0;
    }

    /// Close the palette
    pub fn close(&mut self) {
        self.is_open = false;
        self.query.clear();
    }

    /// Update search results based on query
    pub fn update_search(&mut self) {
        if self.query.is_empty() {
            self.entries = self.all_entries.clone();
        } else {
            let query_lower = self.query.to_lowercase();
            self.entries = self
                .all_entries
                .iter()
                .filter_map(|entry| {
                    let name_lower = entry.name.to_lowercase();
                    let desc_lower = entry.description.to_lowercase();

                    // Calculate fuzzy match score
                    let score = Self::fuzzy_score(&query_lower, &name_lower, &desc_lower);
                    if score > 0 {
                        let mut e = entry.clone();
                        e.score = score;
                        Some(e)
                    } else {
                        None
                    }
                })
                .collect();

            // Sort by score (highest first)
            self.entries.sort_by(|a, b| b.score.cmp(&a.score));
        }

        // Reset selection
        self.selected = 0;
    }

    /// Calculate fuzzy match score
    fn fuzzy_score(query: &str, name: &str, desc: &str) -> i32 {
        let mut score = 0;

        // Exact match in name (highest priority)
        if name == query {
            score += 1000;
        }
        // Starts with query
        else if name.starts_with(query) {
            score += 500;
        }
        // Contains query in name
        else if name.contains(query) {
            score += 200;
        }
        // Contains query in description
        else if desc.contains(query) {
            score += 50;
        }
        // Fuzzy character match
        else {
            let mut query_chars = query.chars().peekable();
            for c in name.chars() {
                if query_chars.peek() == Some(&c) {
                    query_chars.next();
                    score += 10;
                }
            }
            // Only count if all query chars were found
            if query_chars.peek().is_some() {
                score = 0;
            }
        }

        score
    }

    /// Move selection up
    pub fn select_up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    /// Move selection down
    pub fn select_down(&mut self) {
        if self.selected + 1 < self.entries.len() {
            self.selected += 1;
        }
    }

    /// Get the selected entry
    pub fn get_selected(&self) -> Option<&PaletteEntry> {
        self.entries.get(self.selected)
    }

    /// Get selected command name
    pub fn get_selected_command(&self) -> Option<String> {
        self.get_selected().map(|e| e.name.clone())
    }
}

impl Default for CommandPalette {
    fn default() -> Self {
        Self::new()
    }
}
