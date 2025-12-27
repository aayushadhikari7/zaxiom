//! Git shortcuts
//!
//! Quick commands for common git operations.

#![allow(dead_code)]

use std::collections::HashMap;

/// Git shortcut definitions
pub struct GitShortcuts {
    shortcuts: HashMap<&'static str, ShortcutDef>,
}

struct ShortcutDef {
    /// The git command template
    template: &'static str,
    /// Whether it requires arguments
    requires_args: bool,
    /// Description
    description: &'static str,
}

impl GitShortcuts {
    pub fn new() -> Self {
        let mut shortcuts = HashMap::new();

        shortcuts.insert("gs", ShortcutDef {
            template: "git status",
            requires_args: false,
            description: "Show git status",
        });

        shortcuts.insert("gd", ShortcutDef {
            template: "git diff",
            requires_args: false,
            description: "Show git diff",
        });

        shortcuts.insert("gds", ShortcutDef {
            template: "git diff --staged",
            requires_args: false,
            description: "Show staged changes",
        });

        shortcuts.insert("gl", ShortcutDef {
            template: "git log --oneline -20",
            requires_args: false,
            description: "Show recent commits",
        });

        shortcuts.insert("gp", ShortcutDef {
            template: "git push",
            requires_args: false,
            description: "Push to remote",
        });

        shortcuts.insert("gpl", ShortcutDef {
            template: "git pull",
            requires_args: false,
            description: "Pull from remote",
        });

        shortcuts.insert("ga", ShortcutDef {
            template: "git add",
            requires_args: false, // Defaults to . if no args
            description: "Stage files",
        });

        shortcuts.insert("gc", ShortcutDef {
            template: "git commit -m",
            requires_args: true,
            description: "Commit with message",
        });

        shortcuts.insert("gco", ShortcutDef {
            template: "git checkout",
            requires_args: true,
            description: "Checkout branch/file",
        });

        shortcuts.insert("gb", ShortcutDef {
            template: "git branch",
            requires_args: false,
            description: "List branches",
        });

        shortcuts.insert("gba", ShortcutDef {
            template: "git branch -a",
            requires_args: false,
            description: "List all branches",
        });

        shortcuts.insert("gcb", ShortcutDef {
            template: "git checkout -b",
            requires_args: true,
            description: "Create and checkout branch",
        });

        shortcuts.insert("grh", ShortcutDef {
            template: "git reset --hard HEAD",
            requires_args: false,
            description: "Hard reset to HEAD",
        });

        shortcuts.insert("gst", ShortcutDef {
            template: "git stash",
            requires_args: false,
            description: "Stash changes",
        });

        shortcuts.insert("gstp", ShortcutDef {
            template: "git stash pop",
            requires_args: false,
            description: "Pop stashed changes",
        });

        Self { shortcuts }
    }

    /// Expand a shortcut to its full command
    pub fn expand(&self, shortcut: &str, args: &[String]) -> Option<String> {
        let def = self.shortcuts.get(shortcut)?;

        if def.requires_args && args.is_empty() {
            return None; // Needs args but none provided
        }

        let mut command = def.template.to_string();

        if !args.is_empty() {
            // Special handling for gc (commit message needs quotes)
            if shortcut == "gc" {
                command = format!("{} \"{}\"", command, args.join(" "));
            } else {
                command = format!("{} {}", command, args.join(" "));
            }
        } else if shortcut == "ga" {
            // Default to adding all
            command = format!("{} .", command);
        }

        Some(command)
    }

    /// List all shortcuts with descriptions
    pub fn list(&self) -> Vec<(&'static str, &'static str)> {
        let mut list: Vec<_> = self.shortcuts
            .iter()
            .map(|(k, v)| (*k, v.description))
            .collect();
        list.sort_by_key(|(k, _)| *k);
        list
    }
}

impl Default for GitShortcuts {
    fn default() -> Self {
        Self::new()
    }
}
