//! Environment variable management
//!
//! Handles loading, prompting, and saving API keys securely.

use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

/// Get the path to the .env file in the project/user directory
pub fn get_env_file_path() -> PathBuf {
    // Try current directory first, then fall back to home directory
    let current_dir = std::env::current_dir().unwrap_or_default();
    let env_path = current_dir.join(".env");

    if env_path.exists() || !current_dir.join("Cargo.toml").exists() {
        // Use current dir if .env exists or we're not in a cargo project
        env_path
    } else {
        // For installed app, use home directory
        dirs::home_dir()
            .map(|h| h.join(".zaxiom").join(".env"))
            .unwrap_or(env_path)
    }
}

/// Initialize environment - load .env and create if needed
pub fn init_env() {
    // Load from .env file
    let env_path = get_env_file_path();

    // Create parent directories if needed
    if let Some(parent) = env_path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    // Create .env with template if it doesn't exist
    if !env_path.exists() {
        let template = r#"# Zaxiom AI Provider Configuration
# Uncomment and add your API keys below

# Local AI (no key required - just run 'ollama serve')
# Ollama is checked first if running

# Cloud Providers (keys required)
# GROQ_API_KEY=           # Free tier: https://console.groq.com
# OPENAI_API_KEY=         # https://platform.openai.com
# ANTHROPIC_API_KEY=      # https://console.anthropic.com
# GEMINI_API_KEY=         # https://aistudio.google.com
# MISTRAL_API_KEY=        # https://console.mistral.ai
# DEEPSEEK_API_KEY=       # https://platform.deepseek.com
# XAI_API_KEY=            # https://console.x.ai
# COHERE_API_KEY=         # https://dashboard.cohere.com
# PERPLEXITY_API_KEY=     # https://www.perplexity.ai/settings/api

# Default provider (optional - auto-detected if not set)
# AI_PROVIDER=groq
"#;
        let _ = fs::write(&env_path, template);
    }

    // Load the .env file
    let _ = dotenvy::from_path(&env_path);
}

/// Show setup instructions for missing API key
/// Note: We can't use rpassword in a GUI terminal, so we show instructions instead
pub fn prompt_for_key(_provider_name: &str, _env_var: &str, _signup_url: &str) -> Option<String> {
    // In a GUI terminal like Zaxiom, we can't use stdin-based password prompts
    // Return None to let the caller show appropriate error message
    None
}

/// Generate setup instructions for a provider
pub fn get_setup_instructions(provider_name: &str, env_var: &str, signup_url: &str) -> String {
    let env_path = get_env_file_path();
    format!(
        r#"
┌─────────────────────────────────────────────────────────────┐
│  🔑 {} API Key Required
├─────────────────────────────────────────────────────────────┤
│
│  1. Get your API key at:
│     {}
│
│  2. Set it using ONE of these methods:
│
│     Option A - Command (this session):
│     $ export {}=your_key_here
│
│     Option B - Add to .env file (permanent):
│     File: {}
│     Add:  {}=your_key_here
│
└─────────────────────────────────────────────────────────────┘"#,
        provider_name,
        signup_url,
        env_var,
        env_path.display(),
        env_var
    )
}

/// Save an API key to the .env file
pub fn save_key_to_env(key_name: &str, key_value: &str) -> bool {
    let env_path = get_env_file_path();

    // Create parent directories if needed
    if let Some(parent) = env_path.parent() {
        if fs::create_dir_all(parent).is_err() {
            return false;
        }
    }

    // Read existing content
    let mut lines: Vec<String> = if env_path.exists() {
        let file = match fs::File::open(&env_path) {
            Ok(f) => f,
            Err(_) => return false,
        };
        BufReader::new(file).lines().map_while(Result::ok).collect()
    } else {
        vec!["# Zaxiom AI Configuration".to_string(), "".to_string()]
    };

    // Check if key already exists (commented or not)
    let key_prefix = format!("{}=", key_name);
    let commented_prefix = format!("# {}=", key_name);
    let mut found = false;

    for line in &mut lines {
        if line.starts_with(&key_prefix) || line.starts_with(&commented_prefix) {
            *line = format!("{}={}", key_name, key_value);
            found = true;
            break;
        }
    }

    // Add new key if not found
    if !found {
        lines.push(format!("{}={}", key_name, key_value));
    }

    // Write back
    let mut file = match OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&env_path)
    {
        Ok(f) => f,
        Err(_) => return false,
    };

    for line in lines {
        if writeln!(file, "{}", line).is_err() {
            return false;
        }
    }

    true
}

/// Check if a key exists in environment
#[allow(dead_code)]
pub fn has_key(env_var: &str) -> bool {
    std::env::var(env_var).is_ok()
}

/// Get a key, prompting if not available
pub fn get_or_prompt_key(provider_name: &str, env_var: &str, signup_url: &str) -> Option<String> {
    // Check if already set
    if let Ok(key) = std::env::var(env_var) {
        if !key.is_empty() {
            return Some(key);
        }
    }

    // Prompt for key
    prompt_for_key(provider_name, env_var, signup_url)
}
