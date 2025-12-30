//! xAI Provider (Grok)
//!
//! Access to Grok models via xAI's API.

use super::provider::AiProvider;
use anyhow::{anyhow, Result};

const XAI_API_URL: &str = "https://api.x.ai/v1/chat/completions";
const ENV_KEY: &str = "XAI_API_KEY";
const DEFAULT_MODEL: &str = "grok-2-latest";

pub struct XaiProvider;

impl XaiProvider {
    pub fn new() -> Self {
        Self
    }
}

impl Default for XaiProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl AiProvider for XaiProvider {
    fn name(&self) -> &'static str {
        "xai"
    }

    fn display_name(&self) -> &'static str {
        "xAI Grok"
    }

    fn is_available(&self) -> bool {
        self.get_api_key().is_some()
    }

    fn api_key_env(&self) -> &'static str {
        ENV_KEY
    }

    fn signup_url(&self) -> &'static str {
        "https://console.x.ai"
    }

    fn default_model(&self) -> &'static str {
        DEFAULT_MODEL
    }

    fn models(&self) -> Vec<&'static str> {
        vec!["grok-2-latest", "grok-2-vision-latest", "grok-beta"]
    }

    fn chat(&self, prompt: &str, model: Option<&str>) -> Result<String> {
        let api_key = self
            .get_or_prompt_api_key()
            .ok_or_else(|| anyhow!("{}", self.get_setup_instructions()))?;

        let model = model.unwrap_or(DEFAULT_MODEL);

        let client = reqwest::blocking::Client::new();
        let response = client
            .post(XAI_API_URL)
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
            .map_err(|e| anyhow!("Failed to connect to xAI: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            return Err(anyhow!("xAI API error ({}): {}", status, body));
        }

        let json: serde_json::Value = response
            .json()
            .map_err(|e| anyhow!("Failed to parse xAI response: {}", e))?;

        json["choices"][0]["message"]["content"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow!("No response content from xAI"))
    }
}
