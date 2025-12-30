//! AI Provider System
//!
//! Multi-provider AI integration supporting:
//! - Ollama (local, no key required)
//! - Groq (free tier available)
//! - Anthropic (Claude)
//! - OpenAI (GPT)
//! - Google (Gemini)
//! - Mistral AI
//! - DeepSeek
//! - xAI (Grok)
//! - Cohere
//! - Perplexity

use anyhow::{anyhow, Result};

pub mod provider;
pub mod groq;
pub mod anthropic;
pub mod openai;
pub mod gemini;
pub mod mistral;
pub mod deepseek;
pub mod xai;
pub mod cohere;
pub mod perplexity;
pub mod ollama;

pub use provider::{AiProvider, ProviderChoice};
pub use groq::GroqProvider;
pub use anthropic::AnthropicProvider;
pub use openai::OpenAIProvider;
pub use gemini::GeminiProvider;
pub use mistral::MistralProvider;
pub use deepseek::DeepSeekProvider;
pub use xai::XaiProvider;
pub use cohere::CohereProvider;
pub use perplexity::PerplexityProvider;
pub use ollama::OllamaProvider;

/// Null provider - used when no AI provider is configured
/// Returns helpful setup instructions instead of actual AI responses
struct NullProvider;

impl AiProvider for NullProvider {
    fn name(&self) -> &'static str { "none" }
    fn display_name(&self) -> &'static str { "No Provider" }
    fn is_available(&self) -> bool { false }
    fn api_key_env(&self) -> &'static str { "" }
    fn signup_url(&self) -> &'static str { "" }
    fn default_model(&self) -> &'static str { "" }
    fn models(&self) -> Vec<&'static str> { vec![] }

    fn chat(&self, _prompt: &str, _model: Option<&str>) -> Result<String> {
        Err(anyhow!(
            "No AI provider configured.\n\n\
            Quick setup options:\n\n\
            1. Local (no API key):\n\
               Install Ollama: https://ollama.ai\n\
               Then run: ollama serve\n\n\
            2. Cloud (free tier):\n\
               Get a key at: https://console.groq.com\n\
               Then set: export GROQ_API_KEY=your_key\n\n\
            Run '# help' to see all available providers."
        ))
    }
}

/// Get a provider by name
pub fn get_provider(name: &str) -> Option<Box<dyn AiProvider>> {
    match name.to_lowercase().as_str() {
        "groq" => Some(Box::new(GroqProvider::new())),
        "anthropic" | "claude" => Some(Box::new(AnthropicProvider::new())),
        "openai" | "gpt" => Some(Box::new(OpenAIProvider::new())),
        "gemini" | "google" => Some(Box::new(GeminiProvider::new())),
        "mistral" => Some(Box::new(MistralProvider::new())),
        "deepseek" => Some(Box::new(DeepSeekProvider::new())),
        "xai" | "grok" => Some(Box::new(XaiProvider::new())),
        "cohere" => Some(Box::new(CohereProvider::new())),
        "perplexity" | "pplx" => Some(Box::new(PerplexityProvider::new())),
        "ollama" | "local" => Some(Box::new(OllamaProvider::new())),
        _ => None,
    }
}

/// Get provider from choice enum
pub fn get_provider_from_choice(choice: &ProviderChoice) -> Box<dyn AiProvider> {
    match choice {
        ProviderChoice::Groq => Box::new(GroqProvider::new()),
        ProviderChoice::Anthropic => Box::new(AnthropicProvider::new()),
        ProviderChoice::OpenAI => Box::new(OpenAIProvider::new()),
        ProviderChoice::Gemini => Box::new(GeminiProvider::new()),
        ProviderChoice::Mistral => Box::new(MistralProvider::new()),
        ProviderChoice::DeepSeek => Box::new(DeepSeekProvider::new()),
        ProviderChoice::Xai => Box::new(XaiProvider::new()),
        ProviderChoice::Cohere => Box::new(CohereProvider::new()),
        ProviderChoice::Perplexity => Box::new(PerplexityProvider::new()),
        ProviderChoice::Ollama => Box::new(OllamaProvider::new()),
        ProviderChoice::Default => get_default_provider(),
    }
}

/// Get the default provider based on config or availability
pub fn get_default_provider() -> Box<dyn AiProvider> {
    // Check config first
    if let Some(config_provider) = get_config_provider() {
        return config_provider;
    }

    // Fall back to first available provider
    // Priority: Ollama (local, no key) -> Groq (free tier) -> others
    let providers: Vec<Box<dyn AiProvider>> = vec![
        Box::new(OllamaProvider::new()),
        Box::new(GroqProvider::new()),
        Box::new(DeepSeekProvider::new()),
        Box::new(MistralProvider::new()),
        Box::new(OpenAIProvider::new()),
        Box::new(GeminiProvider::new()),
        Box::new(AnthropicProvider::new()),
        Box::new(XaiProvider::new()),
        Box::new(CohereProvider::new()),
        Box::new(PerplexityProvider::new()),
    ];

    for provider in providers {
        if provider.is_available() {
            return provider;
        }
    }

    // No provider available - return a placeholder that will show setup instructions
    Box::new(NullProvider)
}

/// Get provider from config
fn get_config_provider() -> Option<Box<dyn AiProvider>> {
    // Check AI_PROVIDER env var
    if let Ok(provider_name) = std::env::var("AI_PROVIDER") {
        return get_provider(&provider_name);
    }
    None
}

/// List all available providers with their status
pub fn list_providers() -> Vec<ProviderStatus> {
    vec![
        ProviderStatus::new("groq", "Groq", &GroqProvider::new()),
        ProviderStatus::new("anthropic", "Anthropic Claude", &AnthropicProvider::new()),
        ProviderStatus::new("openai", "OpenAI GPT", &OpenAIProvider::new()),
        ProviderStatus::new("gemini", "Google Gemini", &GeminiProvider::new()),
        ProviderStatus::new("mistral", "Mistral AI", &MistralProvider::new()),
        ProviderStatus::new("deepseek", "DeepSeek", &DeepSeekProvider::new()),
        ProviderStatus::new("xai", "xAI Grok", &XaiProvider::new()),
        ProviderStatus::new("cohere", "Cohere", &CohereProvider::new()),
        ProviderStatus::new("perplexity", "Perplexity", &PerplexityProvider::new()),
        ProviderStatus::new("ollama", "Ollama (Local)", &OllamaProvider::new()),
    ]
}

/// Provider status information
pub struct ProviderStatus {
    pub name: &'static str,
    pub display_name: &'static str,
    pub available: bool,
    pub env_var: &'static str,
    pub default_model: &'static str,
}

impl ProviderStatus {
    fn new(name: &'static str, display_name: &'static str, provider: &dyn AiProvider) -> Self {
        Self {
            name,
            display_name,
            available: provider.is_available(),
            env_var: provider.api_key_env(),
            default_model: provider.default_model(),
        }
    }
}

/// Parse provider flag from input and return (provider_choice, remaining_input)
pub fn parse_provider_flag(input: &str) -> (ProviderChoice, String) {
    let input = input.trim();
    let parts: Vec<&str> = input.splitn(2, ' ').collect();

    if parts.is_empty() {
        return (ProviderChoice::Default, String::new());
    }

    // Check if first part is a flag
    if let Some(choice) = ProviderChoice::from_flag(parts[0]) {
        let remaining = if parts.len() > 1 { parts[1].to_string() } else { String::new() };
        return (choice, remaining);
    }

    // No flag found, return default with full input
    (ProviderChoice::Default, input.to_string())
}

/// Handle AI chat with provider selection
#[allow(dead_code)]
pub fn handle_ai_chat(input: &str) -> String {
    let input = input.trim_start_matches('#').trim();

    if input.is_empty() {
        return get_help();
    }

    let (choice, prompt) = parse_provider_flag(input);

    if prompt.is_empty() {
        return get_help();
    }

    let provider = get_provider_from_choice(&choice);

    match provider.chat(&prompt, None) {
        Ok(response) => format!("{} {}:\n\n{}", get_provider_emoji(&choice), provider.display_name(), response),
        Err(e) => format!("Error: {}", e),
    }
}

/// Set the default AI provider
pub fn set_default_provider(choice: &ProviderChoice) -> String {
    let provider_name = choice.name();

    // Save to .env file
    if crate::config::env::save_key_to_env("AI_PROVIDER", provider_name) {
        // Also set in current environment
        std::env::set_var("AI_PROVIDER", provider_name);

        let provider = get_provider_from_choice(choice);
        format!(
            "{} Default provider set to: {}\n\n\
            All future # commands will use this provider.\n\
            Use # --<provider> <msg> to override for a single command.",
            get_provider_emoji(choice),
            provider.display_name()
        )
    } else {
        format!("Failed to save default provider. Using {} for this session only.", provider_name)
    }
}

/// Handle AI chat with terminal context
pub fn handle_ai_chat_with_context(
    input: &str,
    state: &crate::terminal::state::TerminalState,
    history: Option<&[String]>,
) -> String {
    let input = input.trim_start_matches('#').trim();

    if input.is_empty() {
        return get_help();
    }

    let (choice, prompt) = parse_provider_flag(input);

    // If just a flag with no message, set as default provider
    if prompt.is_empty() {
        if choice != ProviderChoice::Default {
            return set_default_provider(&choice);
        } else {
            return get_help();
        }
    }

    let context = build_terminal_context(state, history);
    let full_prompt = format!(
        "You are a helpful terminal assistant. Here's the user's terminal context:\n\n{}\n\nUser question: {}\n\nProvide a concise, helpful response. If the user asks about commands, show examples. Keep responses brief but useful.",
        context,
        prompt
    );

    let provider = get_provider_from_choice(&choice);

    match provider.chat(&full_prompt, None) {
        Ok(response) => format!("{} {}:\n\n{}", get_provider_emoji(&choice), provider.display_name(), response),
        Err(e) => format!("Error: {}", e),
    }
}

/// Build terminal context for AI
fn build_terminal_context(state: &crate::terminal::state::TerminalState, history: Option<&[String]>) -> String {
    let mut context = String::new();

    context.push_str(&format!("Current directory: {}\n", state.cwd().display()));

    #[cfg(target_os = "windows")]
    context.push_str("OS: Windows\n");
    #[cfg(target_os = "linux")]
    context.push_str("OS: Linux\n");
    #[cfg(target_os = "macos")]
    context.push_str("OS: macOS\n");

    context.push_str("Terminal: Zaxiom (Rust-based terminal emulator)\n");

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

/// Get emoji for provider
fn get_provider_emoji(choice: &ProviderChoice) -> &'static str {
    match choice {
        ProviderChoice::Groq => "⚡",
        ProviderChoice::Anthropic => "🎭",
        ProviderChoice::OpenAI => "💬",
        ProviderChoice::Gemini => "✨",
        ProviderChoice::Mistral => "🌀",
        ProviderChoice::DeepSeek => "🔍",
        ProviderChoice::Xai => "𝕏",
        ProviderChoice::Cohere => "🔷",
        ProviderChoice::Perplexity => "🔮",
        ProviderChoice::Ollama => "🦙",
        ProviderChoice::Default => "🧠",
    }
}

/// Get help text
pub fn get_help() -> String {
    let providers = list_providers();
    let mut available = Vec::new();
    let mut unavailable = Vec::new();

    for p in &providers {
        if p.available {
            available.push(format!("  ✓ {} ({})", p.display_name, p.name));
        } else {
            let hint = if p.env_var.is_empty() {
                "run 'ollama serve'".to_string()
            } else {
                format!("set {}", p.env_var)
            };
            unavailable.push(format!("  ✗ {} - {}", p.display_name, hint));
        }
    }

    format!(r#"
┌───────────────────────────────────────────────────────────────┐
│  🤖 AI Chat - Multi-Provider Support                         │
├───────────────────────────────────────────────────────────────┤
│                                                               │
│  USAGE: # [--provider] <question>                             │
│                                                               │
│  FLAGS:                                                       │
│    --groq       Groq (free tier, fast)                        │
│    --claude     Anthropic Claude                              │
│    --gpt        OpenAI GPT                                    │
│    --gemini     Google Gemini                                 │
│    --mistral    Mistral AI                                    │
│    --deepseek   DeepSeek (great for code)                     │
│    --grok       xAI Grok                                      │
│    --cohere     Cohere                                        │
│    --pplx       Perplexity (search-augmented)                 │
│    --ollama     Local Ollama                                  │
│                                                               │
├───────────────────────────────────────────────────────────────┤
│  AVAILABLE:                                                   │
{}│                                                               │
│  NEED SETUP:                                                  │
{}│                                                               │
├───────────────────────────────────────────────────────────────┤
│  COMMANDS: ai status | ai providers                           │
└───────────────────────────────────────────────────────────────┘
"#,
        if available.is_empty() { "│    (none configured)\n".to_string() } else { available.iter().map(|s| format!("│{}\n", s)).collect::<String>() },
        if unavailable.is_empty() { "│    (all configured!)\n".to_string() } else { unavailable.iter().map(|s| format!("│{}\n", s)).collect::<String>() }
    )
}
