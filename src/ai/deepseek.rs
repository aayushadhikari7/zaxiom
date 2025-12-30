//! DeepSeek AI Provider
//!
//! Access to DeepSeek models - excellent for coding tasks.

use anyhow::{anyhow, Result};
use super::provider::AiProvider;

const DEEPSEEK_API_URL: &str = "https://api.deepseek.com/v1/chat/completions";
const ENV_KEY: &str = "DEEPSEEK_API_KEY";
const DEFAULT_MODEL: &str = "deepseek-chat";

pub struct DeepSeekProvider;

impl DeepSeekProvider {
    pub fn new() -> Self {
        Self
    }
}

impl Default for DeepSeekProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl AiProvider for DeepSeekProvider {
    fn name(&self) -> &'static str {
        "deepseek"
    }

    fn display_name(&self) -> &'static str {
        "DeepSeek"
    }

    fn is_available(&self) -> bool {
        self.get_api_key().is_some()
    }

    fn api_key_env(&self) -> &'static str {
        ENV_KEY
    }

    fn signup_url(&self) -> &'static str {
        "https://platform.deepseek.com"
    }

    fn default_model(&self) -> &'static str {
        DEFAULT_MODEL
    }

    fn models(&self) -> Vec<&'static str> {
        vec![
            "deepseek-chat",
            "deepseek-reasoner",
        ]
    }

    fn chat(&self, prompt: &str, model: Option<&str>) -> Result<String> {
        let api_key = self.get_or_prompt_api_key()
            .ok_or_else(|| anyhow!("{}", self.get_setup_instructions()))?;

        let model = model.unwrap_or(DEFAULT_MODEL);

        let client = reqwest::blocking::Client::new();
        let response = client
            .post(DEEPSEEK_API_URL)
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
            .map_err(|e| anyhow!("Failed to connect to DeepSeek: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            return Err(anyhow!("DeepSeek API error ({}): {}", status, body));
        }

        let json: serde_json::Value = response.json()
            .map_err(|e| anyhow!("Failed to parse DeepSeek response: {}", e))?;

        json["choices"][0]["message"]["content"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow!("No response content from DeepSeek"))
    }
}
