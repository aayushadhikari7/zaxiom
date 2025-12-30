//! Mistral AI Provider
//!
//! Access to Mistral models via their API.

use super::provider::AiProvider;
use anyhow::{anyhow, Result};

const MISTRAL_API_URL: &str = "https://api.mistral.ai/v1/chat/completions";
const ENV_KEY: &str = "MISTRAL_API_KEY";
const DEFAULT_MODEL: &str = "mistral-large-latest";

pub struct MistralProvider;

impl MistralProvider {
    pub fn new() -> Self {
        Self
    }
}

impl Default for MistralProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl AiProvider for MistralProvider {
    fn name(&self) -> &'static str {
        "mistral"
    }

    fn display_name(&self) -> &'static str {
        "Mistral AI"
    }

    fn is_available(&self) -> bool {
        self.get_api_key().is_some()
    }

    fn api_key_env(&self) -> &'static str {
        ENV_KEY
    }

    fn signup_url(&self) -> &'static str {
        "https://console.mistral.ai"
    }

    fn default_model(&self) -> &'static str {
        DEFAULT_MODEL
    }

    fn models(&self) -> Vec<&'static str> {
        vec![
            "mistral-large-latest",
            "mistral-medium-latest",
            "mistral-small-latest",
            "codestral-latest",
            "open-mistral-nemo",
        ]
    }

    fn chat(&self, prompt: &str, model: Option<&str>) -> Result<String> {
        let api_key = self
            .get_or_prompt_api_key()
            .ok_or_else(|| anyhow!("{}", self.get_setup_instructions()))?;

        let model = model.unwrap_or(DEFAULT_MODEL);

        let client = reqwest::blocking::Client::new();
        let response = client
            .post(MISTRAL_API_URL)
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
            .map_err(|e| anyhow!("Failed to connect to Mistral: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            return Err(anyhow!("Mistral API error ({}): {}", status, body));
        }

        let json: serde_json::Value = response
            .json()
            .map_err(|e| anyhow!("Failed to parse Mistral response: {}", e))?;

        json["choices"][0]["message"]["content"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow!("No response content from Mistral"))
    }
}
