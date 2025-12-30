//! AI Commands
//!
//! Commands for managing AI providers and interacting with LLMs.
//! The actual provider implementations are in src/ai/.

use anyhow::Result;

use super::traits::Command as CommandTrait;
use crate::ai::{get_help, list_providers, OllamaProvider};
use crate::terminal::state::TerminalState;

/// AI command - manage providers and settings
pub struct AiCommand;

impl CommandTrait for AiCommand {
    fn name(&self) -> &'static str {
        "ai"
    }

    fn description(&self) -> &'static str {
        "Manage AI providers and settings"
    }

    fn usage(&self) -> &'static str {
        "ai [status|providers|help] [args...]"
    }

    fn extended_help(&self) -> String {
        get_help()
    }

    fn execute(&self, args: &[String], _state: &mut TerminalState) -> Result<String> {
        if args.is_empty() {
            return Ok(get_help());
        }

        match args[0].as_str() {
            "status" => Ok(get_status()),
            "providers" | "list" => Ok(get_providers_list()),
            "help" | "--help" | "-h" => Ok(get_help()),
            _ => Ok(get_help()),
        }
    }
}

/// Get provider status
fn get_status() -> String {
    let providers = list_providers();

    let mut output = String::from("\n🤖 AI Provider Status\n");
    output.push_str("═══════════════════════════════════════\n\n");

    for p in providers {
        let status = if p.available { "✓" } else { "✗" };
        let available = if p.available {
            "ready"
        } else {
            "not configured"
        };
        output.push_str(&format!(
            "  {} {} ({})\n    Model: {}\n    Status: {}\n\n",
            status, p.display_name, p.name, p.default_model, available
        ));
    }

    // Show which env vars to set
    output.push_str("Environment Variables:\n");
    output.push_str("  GROQ_API_KEY       - Groq (free tier)\n");
    output.push_str("  ANTHROPIC_API_KEY  - Anthropic Claude\n");
    output.push_str("  OPENAI_API_KEY     - OpenAI GPT\n");
    output.push_str("  GEMINI_API_KEY     - Google Gemini\n");
    output.push_str("\nOllama runs locally - no API key needed.\n");

    output
}

/// Get formatted providers list
fn get_providers_list() -> String {
    let providers = list_providers();

    let mut output = String::from("\n📋 Available AI Providers\n");
    output.push_str("═══════════════════════════════════════\n\n");

    for p in providers {
        let status = if p.available {
            "✓ Ready"
        } else {
            "✗ Need setup"
        };
        output.push_str(&format!(
            "  {:<20} {}\n    Flag: --{}\n    Model: {}\n\n",
            p.display_name, status, p.name, p.default_model
        ));
    }

    output.push_str("Usage: # --<provider> <prompt>\n");
    output.push_str("Example: # --claude explain rust ownership\n");

    output
}

/// Ollama command - Ollama-specific management
pub struct OllamaCommand;

impl CommandTrait for OllamaCommand {
    fn name(&self) -> &'static str {
        "ollama"
    }

    fn description(&self) -> &'static str {
        "Manage local Ollama models"
    }

    fn usage(&self) -> &'static str {
        "ollama [list|pull|serve|status|models|--help] [args...]"
    }

    fn extended_help(&self) -> String {
        get_ollama_help()
    }

    fn execute(&self, args: &[String], _state: &mut TerminalState) -> Result<String> {
        if args.is_empty() || args[0] == "--help" || args[0] == "-h" || args[0] == "help" {
            return Ok(get_ollama_help());
        }

        match args[0].as_str() {
            "list" | "ls" => match OllamaProvider::list_models() {
                Ok(models) => {
                    if models.is_empty() {
                        Ok("No models installed.\n\nRun: ollama pull llama3.2".to_string())
                    } else {
                        let mut output = String::from("📦 Installed models:\n\n");
                        for model in models {
                            output.push_str(&format!("  • {}\n", model));
                        }
                        Ok(output)
                    }
                }
                Err(e) => Ok(format!("❌ {}", e)),
            },

            "pull" => {
                if args.len() < 2 {
                    Ok("Usage: ollama pull <model>\n\nExample: ollama pull llama3.2".to_string())
                } else {
                    match OllamaProvider::pull_model(&args[1]) {
                        Ok(msg) => Ok(format!("✅ {}", msg)),
                        Err(e) => Ok(format!("❌ {}", e)),
                    }
                }
            }

            "serve" | "start" => {
                if OllamaProvider::is_server_running() {
                    Ok("✅ Ollama server is already running".to_string())
                } else {
                    match OllamaProvider::start_server() {
                        Ok(()) => Ok("✅ Ollama server started".to_string()),
                        Err(e) => Ok(format!("❌ {}", e)),
                    }
                }
            }

            "status" => {
                if OllamaProvider::is_server_running() {
                    match OllamaProvider::get_best_model() {
                        Ok(model) => {
                            Ok(format!("✅ Ollama is running\n📦 Default model: {}", model))
                        }
                        Err(_) => Ok("✅ Ollama is running\n⚠️  No models installed".to_string()),
                    }
                } else {
                    Ok("❌ Ollama is not running\n\nRun: ollama serve".to_string())
                }
            }

            "models" => Ok(get_recommended_models()),

            _ => {
                // Pass through to ollama CLI
                match std::process::Command::new("ollama").args(args).output() {
                    Ok(output) => {
                        let stdout = String::from_utf8_lossy(&output.stdout);
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        if output.status.success() {
                            Ok(stdout.to_string())
                        } else {
                            Ok(format!("{}\n{}", stdout, stderr))
                        }
                    }
                    Err(e) => Ok(format!("❌ Failed to run ollama: {}", e)),
                }
            }
        }
    }
}

fn get_ollama_help() -> String {
    r#"
┌─────────────────────────────────────────────────────────────┐
│  🦙 Ollama - Local LLM Management                           │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  COMMANDS:                                                  │
│    ollama list          List installed models               │
│    ollama pull <model>  Download a model                    │
│    ollama serve         Start Ollama server                 │
│    ollama status        Check if Ollama is running          │
│    ollama models        Show recommended models             │
│    ollama --help        Show this help                      │
│                                                             │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  CHAT WITH OLLAMA:                                          │
│    # --ollama <prompt>  Chat using local models             │
│                                                             │
│  Example:                                                   │
│    # --ollama explain what a hashmap is                     │
│                                                             │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  FIRST TIME SETUP:                                          │
│    1. Install Ollama: https://ollama.com                    │
│    2. Pull a model: ollama pull llama3.2                    │
│    3. Chat: # --ollama hello                                │
│                                                             │
└─────────────────────────────────────────────────────────────┘
"#
    .to_string()
}

fn get_recommended_models() -> String {
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
"#
    .to_string()
}
