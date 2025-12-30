//! Ollama Provider (Local LLMs)
//!
//! Runs LLMs locally via Ollama server.
//! No API key needed - uses localhost:11434.

use super::provider::AiProvider;
use anyhow::{anyhow, Result};
use std::io::{BufRead, BufReader};
use std::process::Command;

const OLLAMA_API: &str = "http://localhost:11434";
const DEFAULT_MODEL: &str = "llama3.2";

pub struct OllamaProvider;

impl OllamaProvider {
    pub fn new() -> Self {
        Self
    }

    /// Check if Ollama server is running
    pub fn is_server_running() -> bool {
        match reqwest::blocking::Client::new()
            .get(format!("{}/api/tags", OLLAMA_API))
            .timeout(std::time::Duration::from_secs(2))
            .send()
        {
            Ok(resp) => resp.status().is_success(),
            Err(_) => false,
        }
    }

    /// Start Ollama server in background (silently, no window)
    pub fn start_server() -> Result<()> {
        #[cfg(target_os = "windows")]
        {
            use std::os::windows::process::CommandExt;
            const CREATE_NO_WINDOW: u32 = 0x08000000;
            const DETACHED_PROCESS: u32 = 0x00000008;

            Command::new("ollama")
                .arg("serve")
                .creation_flags(CREATE_NO_WINDOW | DETACHED_PROCESS)
                .spawn()
                .map_err(|e| anyhow!("Failed to start Ollama: {}", e))?;
        }

        #[cfg(not(target_os = "windows"))]
        {
            Command::new("ollama")
                .arg("serve")
                .spawn()
                .map_err(|e| anyhow!("Failed to start Ollama: {}", e))?;
        }

        // Wait for server to start
        std::thread::sleep(std::time::Duration::from_secs(2));

        if Self::is_server_running() {
            Ok(())
        } else {
            Err(anyhow!("Ollama server failed to start"))
        }
    }

    /// Ensure Ollama is running
    pub fn ensure_running() -> Result<()> {
        if Self::is_server_running() {
            Ok(())
        } else {
            Self::start_server()
        }
    }

    /// List installed models
    pub fn list_models() -> Result<Vec<String>> {
        let resp = reqwest::blocking::Client::new()
            .get(format!("{}/api/tags", OLLAMA_API))
            .timeout(std::time::Duration::from_secs(10))
            .send()
            .map_err(|e| anyhow!("Failed to connect to Ollama: {}", e))?;

        let json: serde_json::Value = resp
            .json()
            .map_err(|e| anyhow!("Failed to parse response: {}", e))?;

        let models: Vec<String> = json["models"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|m| m["name"].as_str().map(|s| s.to_string()))
            .collect();

        Ok(models)
    }

    /// Get the best available model
    pub fn get_best_model() -> Result<String> {
        let models = Self::list_models()?;

        // Prefer these models in order
        let preferred = [
            "llama3.2",
            "llama3.1",
            "llama3",
            "mistral",
            "codellama",
            "llama2",
        ];

        for pref in preferred {
            if let Some(model) = models.iter().find(|m| m.starts_with(pref)) {
                return Ok(model.clone());
            }
        }

        models
            .into_iter()
            .next()
            .ok_or_else(|| anyhow!("No models installed. Run: ollama pull llama3.2"))
    }

    /// Pull a model
    pub fn pull_model(model: &str) -> Result<String> {
        Self::ensure_running()?;

        let output = Command::new("ollama")
            .args(["pull", model])
            .output()
            .map_err(|e| anyhow!("Failed to run ollama pull: {}", e))?;

        if output.status.success() {
            Ok(format!("Successfully pulled {}", model))
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(anyhow!("Failed to pull {}: {}", model, stderr))
        }
    }

    /// Chat with streaming (for future use)
    #[allow(dead_code)]
    pub fn chat_stream<F>(prompt: &str, model: Option<&str>, mut on_chunk: F) -> Result<()>
    where
        F: FnMut(&str),
    {
        Self::ensure_running()?;

        let model = match model {
            Some(m) => m.to_string(),
            None => Self::get_best_model()?,
        };

        let client = reqwest::blocking::Client::new();
        let resp = client
            .post(format!("{}/api/generate", OLLAMA_API))
            .json(&serde_json::json!({
                "model": model,
                "prompt": prompt,
                "stream": true
            }))
            .timeout(std::time::Duration::from_secs(300))
            .send()
            .map_err(|e| anyhow!("Failed to send request: {}", e))?;

        let reader = BufReader::new(resp);

        for line in reader.lines() {
            let line = line.map_err(|e| anyhow!("Failed to read response: {}", e))?;
            if line.is_empty() {
                continue;
            }

            let json: serde_json::Value =
                serde_json::from_str(&line).map_err(|e| anyhow!("Failed to parse JSON: {}", e))?;

            if let Some(text) = json["response"].as_str() {
                on_chunk(text);
            }

            if json["done"].as_bool().unwrap_or(false) {
                break;
            }
        }

        Ok(())
    }
}

impl Default for OllamaProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl AiProvider for OllamaProvider {
    fn name(&self) -> &'static str {
        "ollama"
    }

    fn display_name(&self) -> &'static str {
        "Ollama (Local)"
    }

    fn is_available(&self) -> bool {
        // Only check if server is running, don't try to start it
        // Starting should only happen when actually chatting
        Self::is_server_running()
    }

    fn api_key_env(&self) -> &'static str {
        "" // No API key needed
    }

    fn signup_url(&self) -> &'static str {
        "https://ollama.ai"
    }

    fn default_model(&self) -> &'static str {
        DEFAULT_MODEL
    }

    fn models(&self) -> Vec<&'static str> {
        vec![
            "llama3.2",
            "llama3.1",
            "mistral",
            "codellama",
            "deepseek-coder",
            "phi3",
        ]
    }

    fn chat(&self, prompt: &str, model: Option<&str>) -> Result<String> {
        Self::ensure_running()?;

        let model = match model {
            Some(m) => m.to_string(),
            None => Self::get_best_model()?,
        };

        let client = reqwest::blocking::Client::new();
        let resp = client
            .post(format!("{}/api/generate", OLLAMA_API))
            .json(&serde_json::json!({
                "model": model,
                "prompt": prompt,
                "stream": false
            }))
            .timeout(std::time::Duration::from_secs(120))
            .send()
            .map_err(|e| anyhow!("Failed to send request: {}", e))?;

        let json: serde_json::Value = resp
            .json()
            .map_err(|e| anyhow!("Failed to parse response: {}", e))?;

        json["response"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow!("No response from Ollama"))
    }
}
