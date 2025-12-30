//! Cohere AI Provider
//!
//! Access to Cohere's Command models.

use super::provider::AiProvider;
use anyhow::{anyhow, Result};

const COHERE_API_URL: &str = "https://api.cohere.com/v2/chat";
const ENV_KEY: &str = "COHERE_API_KEY";
const DEFAULT_MODEL: &str = "command-r-plus";

pub struct CohereProvider;

impl CohereProvider {
    pub fn new() -> Self {
        Self
    }
}

impl Default for CohereProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl AiProvider for CohereProvider {
    fn name(&self) -> &'static str {
        "cohere"
    }

    fn display_name(&self) -> &'static str {
        "Cohere"
    }

    fn is_available(&self) -> bool {
        self.get_api_key().is_some()
    }

    fn api_key_env(&self) -> &'static str {
        ENV_KEY
    }

    fn signup_url(&self) -> &'static str {
        "https://dashboard.cohere.com"
    }

    fn default_model(&self) -> &'static str {
        DEFAULT_MODEL
    }

    fn models(&self) -> Vec<&'static str> {
        vec!["command-r-plus", "command-r", "command", "command-light"]
    }

    fn chat(&self, prompt: &str, model: Option<&str>) -> Result<String> {
        let api_key = self
            .get_or_prompt_api_key()
            .ok_or_else(|| anyhow!("{}", self.get_setup_instructions()))?;

        let model = model.unwrap_or(DEFAULT_MODEL);

        let client = reqwest::blocking::Client::new();
        let response = client
            .post(COHERE_API_URL)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "model": model,
                "messages": [
                    {"role": "user", "content": prompt}
                ]
            }))
            .timeout(std::time::Duration::from_secs(120))
            .send()
            .map_err(|e| anyhow!("Failed to connect to Cohere: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            return Err(anyhow!("Cohere API error ({}): {}", status, body));
        }

        let json: serde_json::Value = response
            .json()
            .map_err(|e| anyhow!("Failed to parse Cohere response: {}", e))?;

        // Cohere v2 returns message.content[0].text
        json["message"]["content"][0]["text"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow!("No response content from Cohere"))
    }
}
