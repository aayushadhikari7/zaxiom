//! Perplexity AI Provider
//!
//! Search-augmented AI with real-time information.

use anyhow::{anyhow, Result};
use super::provider::AiProvider;

const PERPLEXITY_API_URL: &str = "https://api.perplexity.ai/chat/completions";
const ENV_KEY: &str = "PERPLEXITY_API_KEY";
const DEFAULT_MODEL: &str = "llama-3.1-sonar-large-128k-online";

pub struct PerplexityProvider;

impl PerplexityProvider {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PerplexityProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl AiProvider for PerplexityProvider {
    fn name(&self) -> &'static str {
        "perplexity"
    }

    fn display_name(&self) -> &'static str {
        "Perplexity"
    }

    fn is_available(&self) -> bool {
        self.get_api_key().is_some()
    }

    fn api_key_env(&self) -> &'static str {
        ENV_KEY
    }

    fn signup_url(&self) -> &'static str {
        "https://www.perplexity.ai/settings/api"
    }

    fn default_model(&self) -> &'static str {
        DEFAULT_MODEL
    }

    fn models(&self) -> Vec<&'static str> {
        vec![
            "llama-3.1-sonar-large-128k-online",
            "llama-3.1-sonar-small-128k-online",
            "llama-3.1-sonar-huge-128k-online",
        ]
    }

    fn chat(&self, prompt: &str, model: Option<&str>) -> Result<String> {
        let api_key = self.get_or_prompt_api_key()
            .ok_or_else(|| anyhow!("{}", self.get_setup_instructions()))?;

        let model = model.unwrap_or(DEFAULT_MODEL);

        let client = reqwest::blocking::Client::new();
        let response = client
            .post(PERPLEXITY_API_URL)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "model": model,
                "messages": [
                    {"role": "user", "content": prompt}
                ],
                "max_tokens": 2048
            }))
            .timeout(std::time::Duration::from_secs(120))
            .send()
            .map_err(|e| anyhow!("Failed to connect to Perplexity: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            return Err(anyhow!("Perplexity API error ({}): {}", status, body));
        }

        let json: serde_json::Value = response.json()
            .map_err(|e| anyhow!("Failed to parse Perplexity response: {}", e))?;

        json["choices"][0]["message"]["content"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow!("No response content from Perplexity"))
    }
}
