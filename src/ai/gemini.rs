//! Google Gemini Provider
//!
//! Access to Gemini models via Google's Generative AI API.

use super::provider::AiProvider;
use anyhow::{anyhow, Result};

const ENV_KEY: &str = "GEMINI_API_KEY";
const DEFAULT_MODEL: &str = "gemini-2.5-flash";

pub struct GeminiProvider;

impl GeminiProvider {
    pub fn new() -> Self {
        Self
    }

    fn api_url(&self, model: &str, api_key: &str) -> String {
        format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            model, api_key
        )
    }
}

impl Default for GeminiProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl AiProvider for GeminiProvider {
    fn name(&self) -> &'static str {
        "gemini"
    }

    fn display_name(&self) -> &'static str {
        "Google Gemini"
    }

    fn is_available(&self) -> bool {
        self.get_api_key().is_some()
    }

    fn api_key_env(&self) -> &'static str {
        ENV_KEY
    }

    fn signup_url(&self) -> &'static str {
        "https://aistudio.google.com/apikey"
    }

    fn default_model(&self) -> &'static str {
        DEFAULT_MODEL
    }

    fn models(&self) -> Vec<&'static str> {
        vec![
            "gemini-2.5-flash",
            "gemini-2.5-pro",
            "gemini-2.0-flash",
            "gemini-1.5-pro",
        ]
    }

    fn chat(&self, prompt: &str, model: Option<&str>) -> Result<String> {
        let api_key = self
            .get_or_prompt_api_key()
            .ok_or_else(|| anyhow!("{}", self.get_setup_instructions()))?;

        let model = model.unwrap_or(DEFAULT_MODEL);
        let url = self.api_url(model, &api_key);

        let client = reqwest::blocking::Client::new();
        let response = client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "contents": [{
                    "parts": [{
                        "text": prompt
                    }]
                }]
            }))
            .timeout(std::time::Duration::from_secs(120))
            .send()
            .map_err(|e| anyhow!("Failed to connect to Gemini: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            return Err(anyhow!("Gemini API error ({}): {}", status, body));
        }

        let json: serde_json::Value = response
            .json()
            .map_err(|e| anyhow!("Failed to parse Gemini response: {}", e))?;

        // Gemini returns candidates[0].content.parts[0].text
        json["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow!("No response content from Gemini"))
    }
}
