//! AI Provider trait definition
//!
//! Defines the common interface for all AI providers.

#![allow(dead_code)]

use anyhow::Result;

/// Common trait for all AI providers
pub trait AiProvider: Send + Sync {
    /// Provider name (e.g., "groq", "anthropic", "openai")
    fn name(&self) -> &'static str;

    /// Display name for UI (e.g., "Groq", "Anthropic Claude")
    fn display_name(&self) -> &'static str;

    /// Check if provider is available (has API key or is running)
    fn is_available(&self) -> bool;

    /// Get the environment variable name for the API key
    fn api_key_env(&self) -> &'static str;

    /// Get the signup URL for obtaining an API key
    fn signup_url(&self) -> &'static str;

    /// Get the default model for this provider
    fn default_model(&self) -> &'static str;

    /// List available models
    fn models(&self) -> Vec<&'static str>;

    /// Send a chat message and get response (blocking)
    fn chat(&self, prompt: &str, model: Option<&str>) -> Result<String>;

    /// Get API key from environment
    fn get_api_key(&self) -> Option<String> {
        std::env::var(self.api_key_env()).ok()
    }

    /// Get API key, prompting user if not available
    fn get_or_prompt_api_key(&self) -> Option<String> {
        crate::config::env::get_or_prompt_key(
            self.display_name(),
            self.api_key_env(),
            self.signup_url(),
        )
    }

    /// Get setup instructions for this provider
    fn get_setup_instructions(&self) -> String {
        crate::config::env::get_setup_instructions(
            self.display_name(),
            self.api_key_env(),
            self.signup_url(),
        )
    }
}

/// Provider selection from command flags or config
#[derive(Debug, Clone, PartialEq)]
pub enum ProviderChoice {
    Groq,
    Anthropic,
    OpenAI,
    Gemini,
    Mistral,
    DeepSeek,
    Xai,
    Cohere,
    Perplexity,
    Ollama,
    Default,
}

impl ProviderChoice {
    /// Parse provider from flag string
    pub fn from_flag(flag: &str) -> Option<Self> {
        match flag.to_lowercase().as_str() {
            "--groq" | "-g" => Some(Self::Groq),
            "--claude" | "--anthropic" | "-c" => Some(Self::Anthropic),
            "--gpt" | "--openai" | "-o" => Some(Self::OpenAI),
            "--gemini" | "--google" => Some(Self::Gemini),
            "--mistral" => Some(Self::Mistral),
            "--deepseek" | "--ds" => Some(Self::DeepSeek),
            "--grok" | "--xai" => Some(Self::Xai),
            "--cohere" => Some(Self::Cohere),
            "--perplexity" | "--pplx" => Some(Self::Perplexity),
            "--ollama" | "--local" | "-l" => Some(Self::Ollama),
            _ => None,
        }
    }

    /// Get provider name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Groq => "groq",
            Self::Anthropic => "anthropic",
            Self::OpenAI => "openai",
            Self::Gemini => "gemini",
            Self::Mistral => "mistral",
            Self::DeepSeek => "deepseek",
            Self::Xai => "xai",
            Self::Cohere => "cohere",
            Self::Perplexity => "perplexity",
            Self::Ollama => "ollama",
            Self::Default => "default",
        }
    }
}
