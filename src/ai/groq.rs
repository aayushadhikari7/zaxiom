//! Groq AI Provider
//!
//! Fast inference with free tier (30 requests/minute).
//! Uses OpenAI-compatible API format.

use super::provider::AiProvider;
use anyhow::{anyhow, Result};

const GROQ_API_URL: &str = "https://api.groq.com/openai/v1/chat/completions";
const ENV_KEY: &str = "GROQ_API_KEY";
const DEFAULT_MODEL: &str = "llama-3.3-70b-versatile";

pub struct GroqProvider;

impl GroqProvider {
    pub fn new() -> Self {
        Self
    }
}

impl Default for GroqProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl AiProvider for GroqProvider {
    fn name(&self) -> &'static str {
        "groq"
    }

    fn display_name(&self) -> &'static str {
        "Groq"
    }

    fn is_available(&self) -> bool {
        self.get_api_key().is_some()
    }

    fn api_key_env(&self) -> &'static str {
        ENV_KEY
    }

    fn signup_url(&self) -> &'static str {
        "https://console.groq.com"
    }

    fn default_model(&self) -> &'static str {
        DEFAULT_MODEL
    }

    fn models(&self) -> Vec<&'static str> {
        vec![
            "llama-3.3-70b-versatile",
            "llama-3.3-70b-specdec",
            "llama-3.1-8b-instant",
            "mixtral-8x7b-32768",
            "gemma2-9b-it",
        ]
    }

    fn chat(&self, prompt: &str, model: Option<&str>) -> Result<String> {
        let api_key = self
            .get_or_prompt_api_key()
            .ok_or_else(|| anyhow!("{}", self.get_setup_instructions()))?;

        let model = model.unwrap_or(DEFAULT_MODEL);

        let client = reqwest::blocking::Client::new();
        let response = client
            .post(GROQ_API_URL)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "model": model,
                "messages": [
                    {"role": "user", "content": prompt}
                ],
                "max_tokens": 2048
            }))
            .timeout(std::time::Duration::from_secs(60))
            .send()
            .map_err(|e| anyhow!("Failed to connect to Groq: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            return Err(anyhow!("Groq API error ({}): {}", status, body));
        }

        let json: serde_json::Value = response
            .json()
            .map_err(|e| anyhow!("Failed to parse Groq response: {}", e))?;

        json["choices"][0]["message"]["content"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow!("No response content from Groq"))
    }
}
