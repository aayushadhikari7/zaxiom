//! Anthropic AI Provider (Claude)
//!
//! Access to Claude models via Anthropic's API.

use anyhow::{anyhow, Result};
use super::provider::AiProvider;

const ANTHROPIC_API_URL: &str = "https://api.anthropic.com/v1/messages";
const ENV_KEY: &str = "ANTHROPIC_API_KEY";
const DEFAULT_MODEL: &str = "claude-sonnet-4-5-20250929";
const API_VERSION: &str = "2023-06-01";

pub struct AnthropicProvider;

impl AnthropicProvider {
    pub fn new() -> Self {
        Self
    }
}

impl Default for AnthropicProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl AiProvider for AnthropicProvider {
    fn name(&self) -> &'static str {
        "anthropic"
    }

    fn display_name(&self) -> &'static str {
        "Anthropic Claude"
    }

    fn is_available(&self) -> bool {
        self.get_api_key().is_some()
    }

    fn api_key_env(&self) -> &'static str {
        ENV_KEY
    }

    fn signup_url(&self) -> &'static str {
        "https://console.anthropic.com"
    }

    fn default_model(&self) -> &'static str {
        DEFAULT_MODEL
    }

    fn models(&self) -> Vec<&'static str> {
        vec![
            "claude-sonnet-4-5-20250929",
            "claude-opus-4-5-20251101",
            "claude-sonnet-4-20250514",
            "claude-3-5-haiku-20241022",
        ]
    }

    fn chat(&self, prompt: &str, model: Option<&str>) -> Result<String> {
        let api_key = self.get_or_prompt_api_key()
            .ok_or_else(|| anyhow!("{}", self.get_setup_instructions()))?;

        let model = model.unwrap_or(DEFAULT_MODEL);

        let client = reqwest::blocking::Client::new();
        let response = client
            .post(ANTHROPIC_API_URL)
            .header("x-api-key", &api_key)
            .header("anthropic-version", API_VERSION)
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "model": model,
                "max_tokens": 2048,
                "messages": [
                    {"role": "user", "content": prompt}
                ]
            }))
            .timeout(std::time::Duration::from_secs(120))
            .send()
            .map_err(|e| anyhow!("Failed to connect to Anthropic: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            return Err(anyhow!("Anthropic API error ({}): {}", status, body));
        }

        let json: serde_json::Value = response.json()
            .map_err(|e| anyhow!("Failed to parse Anthropic response: {}", e))?;

        // Anthropic returns content as an array of content blocks
        json["content"][0]["text"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow!("No response content from Anthropic"))
    }
}
