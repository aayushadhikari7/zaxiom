//! User-defined aliases
//!
//! Custom command shortcuts.

#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Alias configuration
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct AliasConfig {
    /// User-defined aliases (alias -> expansion)
    #[serde(default)]
    pub commands: HashMap<String, String>,
}

impl AliasConfig {
    /// Create new empty alias config
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
        }
    }

    /// Add a default set of useful aliases
    pub fn with_defaults() -> Self {
        let mut aliases = HashMap::new();

        // Common shortcuts
        aliases.insert("ll".to_string(), "ls -la".to_string());
        aliases.insert("la".to_string(), "ls -a".to_string());
        aliases.insert("..".to_string(), "cd ..".to_string());
        aliases.insert("...".to_string(), "cd ../..".to_string());

        // Git shortcuts are handled separately in executor
        // but can also be defined here for customization

        Self { commands: aliases }
    }

    /// Look up an alias
    pub fn get(&self, alias: &str) -> Option<&str> {
        self.commands.get(alias).map(|s| s.as_str())
    }

    /// Add an alias
    pub fn add(&mut self, alias: String, expansion: String) {
        self.commands.insert(alias, expansion);
    }

    /// Remove an alias
    pub fn remove(&mut self, alias: &str) -> bool {
        self.commands.remove(alias).is_some()
    }

    /// List all aliases
    pub fn list(&self) -> impl Iterator<Item = (&str, &str)> {
        self.commands.iter().map(|(k, v)| (k.as_str(), v.as_str()))
    }
}
