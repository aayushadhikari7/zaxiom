//! Ollama AI Integration
//!
//! Provides AI chat via Ollama's local API.
//! Use `#` prefix for quick AI queries or `ollama` command for full control.

#![allow(dead_code)]

use std::process::Command;
use std::io::{BufRead, BufReader};
use anyhow::Result;

use crate::terminal::state::TerminalState;
use super::traits::Command as CommandTrait;

/// Ollama API base URL
const OLLAMA_API: &str = "http://localhost:11434";

/// Default model to use
const DEFAULT_MODEL: &str = "llama3.2";

/// Check if Ollama server is running
pub fn is_ollama_running() -> bool {
    match reqwest::blocking::Client::new()
        .get(format!("{}/api/tags", OLLAMA_API))
        .timeout(std::time::Duration::from_secs(2))
        .send()
    {
        Ok(resp) => resp.status().is_success(),
        Err(_) => false,
    }
}

/// Start Ollama server in background
pub fn start_ollama_server() -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args(["/C", "start", "/B", "ollama", "serve"])
            .spawn()
            .map_err(|e| format!("Failed to start Ollama: {}", e))?;
    }

    #[cfg(not(target_os = "windows"))]
    {
        Command::new("ollama")
            .arg("serve")
            .spawn()
            .map_err(|e| format!("Failed to start Ollama: {}", e))?;
    }

    // Wait a bit for server to start
    std::thread::sleep(std::time::Duration::from_secs(2));

    if is_ollama_running() {
        Ok(())
    } else {
        Err("Ollama server failed to start".to_string())
    }
}

/// Ensure Ollama is running, start if needed
pub fn ensure_ollama_running() -> Result<(), String> {
    if is_ollama_running() {
        Ok(())
    } else {
        start_ollama_server()
    }
}

/// List available models
pub fn list_models() -> Result<Vec<String>, String> {
    let resp = reqwest::blocking::Client::new()
        .get(format!("{}/api/tags", OLLAMA_API))
        .timeout(std::time::Duration::from_secs(10))
        .send()
        .map_err(|e| format!("Failed to connect to Ollama: {}", e))?;

    let json: serde_json::Value = resp.json()
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    let models: Vec<String> = json["models"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .filter_map(|m| m["name"].as_str().map(|s| s.to_string()))
        .collect();

    Ok(models)
}

/// Get the default or first available model
pub fn get_default_model() -> Result<String, String> {
    let models = list_models()?;

    // Prefer these models in order
    let preferred = ["llama3.2", "llama3.1", "llama3", "mistral", "codellama", "llama2"];

    for pref in preferred {
        if models.iter().any(|m| m.starts_with(pref)) {
            return Ok(models.iter().find(|m| m.starts_with(pref)).unwrap().clone());
        }
    }

    // Return first available model
    models.into_iter().next().ok_or_else(|| {
        "No models installed. Run: ollama pull llama3.2".to_string()
    })
}

/// Send a chat message to Ollama and get response (blocking, for simple queries)
pub fn chat_simple(prompt: &str, model: Option<&str>) -> Result<String, String> {
    ensure_ollama_running()?;

    let model = match model {
        Some(m) => m.to_string(),
        None => get_default_model()?,
    };

    let client = reqwest::blocking::Client::new();
    let resp = client
        .post(format!("{}/api/generate", OLLAMA_API))
        .json(&serde_json::json!({
            "model": model,
            "prompt": prompt,
            "stream": false
        }))
        .timeout(std::time::Duration::from_secs(120))
        .send()
        .map_err(|e| format!("Failed to send request: {}", e))?;

    let json: serde_json::Value = resp.json()
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    json["response"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| "No response from model".to_string())
}

/// Send a chat message with streaming (returns chunks via callback)
pub fn chat_stream<F>(prompt: &str, model: Option<&str>, mut on_chunk: F) -> Result<(), String>
where
    F: FnMut(&str),
{
    ensure_ollama_running()?;

    let model = match model {
        Some(m) => m.to_string(),
        None => get_default_model()?,
    };

    let client = reqwest::blocking::Client::new();
    let resp = client
        .post(format!("{}/api/generate", OLLAMA_API))
        .json(&serde_json::json!({
            "model": model,
            "prompt": prompt,
            "stream": true
        }))
        .timeout(std::time::Duration::from_secs(300))
        .send()
        .map_err(|e| format!("Failed to send request: {}", e))?;

    let reader = BufReader::new(resp);

    for line in reader.lines() {
        let line = line.map_err(|e| format!("Failed to read response: {}", e))?;
        if line.is_empty() {
            continue;
        }

        let json: serde_json::Value = serde_json::from_str(&line)
            .map_err(|e| format!("Failed to parse JSON: {}", e))?;

        if let Some(text) = json["response"].as_str() {
            on_chunk(text);
        }

        if json["done"].as_bool().unwrap_or(false) {
            break;
        }
    }

    Ok(())
}

/// Pull a model from Ollama registry
pub fn pull_model(model: &str) -> Result<String, String> {
    ensure_ollama_running()?;

    let output = Command::new("ollama")
        .args(["pull", model])
        .output()
        .map_err(|e| format!("Failed to run ollama pull: {}", e))?;

    if output.status.success() {
        Ok(format!("Successfully pulled {}", model))
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("Failed to pull {}: {}", model, stderr))
    }
}

/// Get Ollama help text
pub fn get_help() -> String {
    r#"
┌─────────────────────────────────────────────────────────────┐
│  🦙 Ollama AI Integration                                   │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  QUICK AI CHAT (use # prefix):                              │
│    # explain what a hashmap is                              │
│    # write a rust function to reverse a string              │
│    # fix this error: cannot borrow as mutable               │
│                                                             │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  OLLAMA COMMANDS:                                           │
│    ollama list          List installed models               │
│    ollama pull <model>  Download a model                    │
│    ollama run <model>   Chat with a specific model          │
│    ollama serve         Start Ollama server                 │
│    ollama status        Check if Ollama is running          │
│    ollama models        Show recommended models             │
│    ollama --help        Show this help                      │
│                                                             │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  RECOMMENDED MODELS:                                        │
│    llama3.2      Latest Llama (8B, fast, good quality)      │
│    mistral       Great for code and reasoning               │
│    codellama     Specialized for programming                │
│    deepseek-coder Best for code generation                  │
│    phi3          Small but capable (3.8B)                   │
│                                                             │
│  First time? Run: ollama pull llama3.2                      │
│                                                             │
└─────────────────────────────────────────────────────────────┘
"#.to_string()
}

/// Execute ollama command
pub fn execute_ollama_command(args: &[String]) -> String {
    if args.is_empty() || args[0] == "--help" || args[0] == "-h" || args[0] == "help" {
        return get_help();
    }

    match args[0].as_str() {
        "list" | "ls" => {
            match list_models() {
                Ok(models) => {
                    if models.is_empty() {
                        "No models installed.\n\nRun: ollama pull llama3.2".to_string()
                    } else {
                        let mut output = String::from("📦 Installed models:\n\n");
                        for model in models {
                            output.push_str(&format!("  • {}\n", model));
                        }
                        output
                    }
                }
                Err(e) => format!("❌ {}", e),
            }
        }

        "pull" => {
            if args.len() < 2 {
                "Usage: ollama pull <model>\n\nExample: ollama pull llama3.2".to_string()
            } else {
                match pull_model(&args[1]) {
                    Ok(msg) => format!("✅ {}", msg),
                    Err(e) => format!("❌ {}", e),
                }
            }
        }

        "run" | "chat" => {
            if args.len() < 2 {
                "Usage: ollama run <model> [prompt]\n\nExample: ollama run llama3.2 hello".to_string()
            } else {
                let model = &args[1];
                if args.len() > 2 {
                    // Direct prompt provided
                    let prompt = args[2..].join(" ");
                    match chat_simple(&prompt, Some(model)) {
                        Ok(resp) => format!("🦙 {}\n\n{}", model, resp),
                        Err(e) => format!("❌ {}", e),
                    }
                } else {
                    format!("🦙 Model '{}' ready. Use # prefix to chat.\n\nExample: # hello", model)
                }
            }
        }

        "serve" | "start" => {
            if is_ollama_running() {
                "✅ Ollama server is already running".to_string()
            } else {
                match start_ollama_server() {
                    Ok(()) => "✅ Ollama server started".to_string(),
                    Err(e) => format!("❌ {}", e),
                }
            }
        }

        "status" => {
            if is_ollama_running() {
                match get_default_model() {
                    Ok(model) => format!("✅ Ollama is running\n📦 Default model: {}", model),
                    Err(_) => "✅ Ollama is running\n⚠️  No models installed".to_string(),
                }
            } else {
                "❌ Ollama is not running\n\nRun: ollama serve".to_string()
            }
        }

        "models" => {
            r#"📦 Recommended models:

  General purpose:
    • llama3.2        - Latest Llama, 8B params, fast & smart
    • llama3.1        - Previous gen, very capable
    • mistral         - Great reasoning, 7B params
    • phi3            - Small but mighty, 3.8B params

  Coding:
    • codellama       - Meta's code model
    • deepseek-coder  - Excellent for code
    • starcoder2      - Code completion

  Small & Fast:
    • tinyllama       - 1.1B, very fast
    • phi3:mini       - Tiny but useful

Download with: ollama pull <model>
"#.to_string()
        }

        _ => {
            // Pass through to ollama CLI
            match Command::new("ollama").args(args).output() {
                Ok(output) => {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    if output.status.success() {
                        stdout.to_string()
                    } else {
                        format!("{}\n{}", stdout, stderr)
                    }
                }
                Err(e) => format!("❌ Failed to run ollama: {}", e),
            }
        }
    }
}

/// Build terminal context for AI
fn build_terminal_context(state: &crate::terminal::state::TerminalState, history: Option<&[String]>) -> String {
    let mut context = String::new();

    // Current working directory
    context.push_str(&format!("Current directory: {}\n", state.cwd().display()));

    // Operating system
    #[cfg(target_os = "windows")]
    context.push_str("OS: Windows\n");
    #[cfg(target_os = "linux")]
    context.push_str("OS: Linux\n");
    #[cfg(target_os = "macos")]
    context.push_str("OS: macOS\n");

    // Terminal name
    context.push_str("Terminal: Zaxiom (Rust-based terminal emulator)\n");

    // Recent command history (last 5 commands)
    if let Some(hist) = history {
        if !hist.is_empty() {
            context.push_str("\nRecent commands:\n");
            for (i, cmd) in hist.iter().rev().take(5).enumerate() {
                context.push_str(&format!("  {}. {}\n", i + 1, cmd));
            }
        }
    }

    context
}

/// Handle # prefix AI chat with terminal context
pub fn handle_ai_chat_with_context(prompt: &str, state: &crate::terminal::state::TerminalState, history: Option<&[String]>) -> String {
    // Remove # prefix and trim
    let prompt = prompt.trim_start_matches('#').trim();

    if prompt.is_empty() {
        return get_help();
    }

    // Check if ollama is running
    if !is_ollama_running() {
        match start_ollama_server() {
            Ok(()) => {}
            Err(e) => return format!("❌ {}\n\nMake sure Ollama is installed: https://ollama.com", e),
        }
    }

    // Build context-aware prompt
    let context = build_terminal_context(state, history);
    let full_prompt = format!(
        "You are a helpful terminal assistant. Here's the user's terminal context:\n\n{}\n\nUser question: {}\n\nProvide a concise, helpful response. If the user asks about commands, show examples. Keep responses brief but useful.",
        context,
        prompt
    );

    // Get response
    match chat_simple(&full_prompt, None) {
        Ok(response) => {
            format!("🦙 AI:\n\n{}", response)
        }
        Err(e) => {
            if e.contains("No models") {
                format!("❌ {}\n\nInstall a model first:\n  ollama pull llama3.2", e)
            } else {
                format!("❌ {}", e)
            }
        }
    }
}

/// Handle # prefix AI chat (simple version without context)
pub fn handle_ai_chat(prompt: &str) -> String {
    // Remove # prefix and trim
    let prompt = prompt.trim_start_matches('#').trim();

    if prompt.is_empty() {
        return get_help();
    }

    // Check if ollama is running
    if !is_ollama_running() {
        match start_ollama_server() {
            Ok(()) => {}
            Err(e) => return format!("❌ {}\n\nMake sure Ollama is installed: https://ollama.com", e),
        }
    }

    // Get response
    match chat_simple(prompt, None) {
        Ok(response) => {
            format!("🦙 AI:\n\n{}", response)
        }
        Err(e) => {
            if e.contains("No models") {
                format!("❌ {}\n\nInstall a model first:\n  ollama pull llama3.2", e)
            } else {
                format!("❌ {}", e)
            }
        }
    }
}

/// Ollama command implementation
pub struct OllamaCommand;

impl CommandTrait for OllamaCommand {
    fn name(&self) -> &'static str {
        "ollama"
    }

    fn description(&self) -> &'static str {
        "AI chat and model management via Ollama"
    }

    fn usage(&self) -> &'static str {
        "ollama [list|pull|run|serve|status|models|--help] [args...]"
    }

    fn extended_help(&self) -> String {
        get_help()
    }

    fn execute(&self, args: &[String], _state: &mut TerminalState) -> Result<String> {
        Ok(execute_ollama_command(args))
    }
}
