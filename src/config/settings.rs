//! Application settings
//!
//! Global settings for Zaxiom.

#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use super::theme::ThemeConfig;
use super::aliases::AliasConfig;

/// Main configuration
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Config {
    #[serde(default)]
    pub theme: ThemeConfig,

    #[serde(default)]
    pub font: FontConfig,

    #[serde(default)]
    pub prompt: PromptConfig,

    #[serde(default)]
    pub aliases: AliasConfig,

    #[serde(default)]
    pub terminal: TerminalConfig,

    #[serde(default)]
    pub ai: AiConfig,

    /// Kawaii mode - cuter UI elements when enabled
    #[serde(default)]
    pub kawaii_mode: bool,
}

/// AI provider configuration
#[derive(Debug, Deserialize, Serialize)]
#[derive(Default)]
pub struct AiConfig {
    /// Default provider: "groq", "anthropic", "openai", "gemini", "ollama"
    pub default_provider: Option<String>,

    /// Default model override (uses provider's default if not set)
    pub default_model: Option<String>,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct FontConfig {
    pub family: String,
    pub size: f32,
}

impl Default for FontConfig {
    fn default() -> Self {
        Self {
            family: "JetBrainsMono Nerd Font".to_string(),
            size: 14.0,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PromptConfig {
    pub format: String,
}

impl Default for PromptConfig {
    fn default() -> Self {
        Self {
            format: "{cwd} {git_branch} → ".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TerminalConfig {
    pub scrollback_lines: usize,
    pub history_size: usize,
}

impl Default for TerminalConfig {
    fn default() -> Self {
        Self {
            scrollback_lines: 10_000,
            history_size: 1_000,
        }
    }
}

impl Config {
    /// Load config from file
    pub fn load() -> Self {
        let config_path = Self::config_path();

        if config_path.exists() {
            match std::fs::read_to_string(&config_path) {
                Ok(contents) => {
                    match toml::from_str(&contents) {
                        Ok(config) => return config,
                        Err(e) => {
                            eprintln!("Failed to parse config: {}", e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to read config: {}", e);
                }
            }
        }

        Self::default()
    }

    /// Save config to file
    pub fn save(&self) -> std::io::Result<()> {
        Self::ensure_config_dir()?;
        let config_path = Self::config_path();
        let contents = toml::to_string_pretty(self)
            .map_err(std::io::Error::other)?;
        std::fs::write(config_path, contents)
    }

    /// Get config file path
    pub fn config_path() -> PathBuf {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("zaxiom");

        config_dir.join("config.toml")
    }

    /// Ensure config directory exists
    pub fn ensure_config_dir() -> std::io::Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("zaxiom");

        std::fs::create_dir_all(&config_dir)?;
        Ok(config_dir)
    }

    /// Set theme name and save
    pub fn set_theme(&mut self, theme_name: &str) -> std::io::Result<()> {
        self.theme.name = Some(theme_name.to_string());
        self.save()
    }

    /// Set kawaii mode and save
    pub fn set_kawaii_mode(&mut self, enabled: bool) -> std::io::Result<()> {
        self.kawaii_mode = enabled;
        self.save()
    }
}
