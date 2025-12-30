//! OpenAI Provider (GPT)
//!
//! Access to GPT models via OpenAI's API.

use super::provider::AiProvider;
use anyhow::{anyhow, Result};

const OPENAI_API_URL: &str = "https://api.openai.com/v1/chat/completions";
const ENV_KEY: &str = "OPENAI_API_KEY";
const DEFAULT_MODEL: &str = "gpt-5.2";

pub struct OpenAIProvider;

impl OpenAIProvider {
    pub fn new() -> Self {
        Self
    }
}

impl Default for OpenAIProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl AiProvider for OpenAIProvider {
    fn name(&self) -> &'static str {
        "openai"
    }

    fn display_name(&self) -> &'static str {
        "OpenAI GPT"
    }

    fn is_available(&self) -> bool {
        self.get_api_key().is_some()
    }

    fn api_key_env(&self) -> &'static str {
        ENV_KEY
    }

    fn signup_url(&self) -> &'static str {
        "https://platform.openai.com/api-keys"
    }

    fn default_model(&self) -> &'static str {
        DEFAULT_MODEL
    }

    fn models(&self) -> Vec<&'static str> {
        vec![
            "gpt-5.2",
            "gpt-5.2-pro",
            "gpt-5-mini",
            "gpt-4o",
            "gpt-4o-mini",
            "o1",
            "o1-mini",
        ]
    }

    fn chat(&self, prompt: &str, model: Option<&str>) -> Result<String> {
        let api_key = self
            .get_or_prompt_api_key()
            .ok_or_else(|| anyhow!("{}", self.get_setup_instructions()))?;

        let model = model.unwrap_or(DEFAULT_MODEL);

        let client = reqwest::blocking::Client::new();
        let response = client
            .post(OPENAI_API_URL)
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
            .map_err(|e| anyhow!("Failed to connect to OpenAI: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            return Err(anyhow!("OpenAI API error ({}): {}", status, body));
        }

        let json: serde_json::Value = response
            .json()
            .map_err(|e| anyhow!("Failed to parse OpenAI response: {}", e))?;

        json["choices"][0]["message"]["content"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow!("No response content from OpenAI"))
    }
}
