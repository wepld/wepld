//! OpenAI-compatible reasoning adapter. Speaks the `/v1/chat/completions`
//! JSON shape used by Ollama, LM Studio, vLLM, llama.cpp's server, and the
//! OpenAI API itself — so WePLD depends on no single provider (charter LLM
//! philosophy; "Hermes + Ollama" is a first-class local mode).
//!
//! This build targets HTTP endpoints (local-first: Ollama & friends). Hosted
//! HTTPS endpoints require a TLS-enabled build, added in a later slice; the
//! adapter contract is unchanged by that.

use crate::{Adapter, AdapterError, AdapterRequest, AdapterResponse};
use std::time::{Duration, Instant};
use wepld_contracts::brain::Usage;

pub struct OpenAiCompatAdapter {
    name: String,
    base_url: String,
    api_key: Option<String>,
    timeout: Duration,
}

impl OpenAiCompatAdapter {
    /// `name` is the adapter id a profile routes to (e.g. "ollama").
    /// `base_url` is the server root, e.g. `http://127.0.0.1:11434`.
    pub fn new(name: &str, base_url: &str, api_key: Option<String>, timeout: Duration) -> Self {
        Self {
            name: name.to_owned(),
            base_url: base_url.trim_end_matches('/').to_owned(),
            api_key,
            timeout,
        }
    }
}

impl Adapter for OpenAiCompatAdapter {
    fn name(&self) -> &str {
        &self.name
    }

    fn invoke(&self, req: &AdapterRequest) -> Result<AdapterResponse, AdapterError> {
        let url = format!("{}/v1/chat/completions", self.base_url);
        let body = serde_json::json!({
            "model": req.model,
            "messages": [
                { "role": "system", "content": system_prompt(&req.output_schema_id) },
                { "role": "user", "content": user_prompt(&req.intent, &req.pack) }
            ],
            "temperature": 0,
            "stream": false
        });

        let cfg = ureq::Agent::config_builder()
            .timeout_global(Some(self.timeout))
            .build();
        let agent: ureq::Agent = cfg.into();

        let started = Instant::now();
        let mut builder = agent.post(&url);
        if let Some(key) = &self.api_key {
            builder = builder.header("Authorization", format!("Bearer {key}"));
        }
        let mut response = builder
            .send_json(&body)
            .map_err(|e| AdapterError::Provider(format!("request failed: {e}")))?;
        let parsed: serde_json::Value = response
            .body_mut()
            .read_json()
            .map_err(|e| AdapterError::Provider(format!("invalid response body: {e}")))?;
        let latency_ms = started.elapsed().as_millis() as u64;

        let content = parsed["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| {
                AdapterError::Provider("response missing choices[0].message.content".to_owned())
            })?;
        let output = extract_json(content).ok_or_else(|| {
            AdapterError::Provider("model output is not parseable JSON".to_owned())
        })?;

        Ok(AdapterResponse {
            output,
            usage: Usage {
                provider: self.name.clone(),
                model: req.model.clone(),
                tokens_in: parsed["usage"]["prompt_tokens"].as_u64().unwrap_or(0),
                tokens_out: parsed["usage"]["completion_tokens"].as_u64().unwrap_or(0),
                cost_usd: 0.0, // local endpoints are free; hosted cost estimation is a later slice
                latency_ms,
            },
        })
    }

    fn supports_reformat_retry(&self) -> bool {
        true
    }
}

fn system_prompt(schema_id: &str) -> String {
    format!(
        "You are a WePLD engineering worker. Respond with ONLY a single JSON object \
         that conforms to the '{schema_id}' output schema. No prose, no code fences."
    )
}

fn user_prompt(intent: &str, pack: &serde_json::Value) -> String {
    format!(
        "Intent: {intent}\n\nContext pack (JSON):\n{}\n\nReturn only the JSON result.",
        serde_json::to_string_pretty(pack).unwrap_or_else(|_| pack.to_string())
    )
}

/// Extract a JSON object from model content that may include code fences or
/// stray prose. Tries a direct parse, then a fenced block, then the widest
/// brace-delimited span.
fn extract_json(content: &str) -> Option<serde_json::Value> {
    let trimmed = content.trim();
    if let Ok(v) = serde_json::from_str::<serde_json::Value>(trimmed) {
        return Some(v);
    }
    let unfenced = trimmed
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();
    if let Ok(v) = serde_json::from_str::<serde_json::Value>(unfenced) {
        return Some(v);
    }
    let start = content.find('{')?;
    let end = content.rfind('}')?;
    if end > start {
        serde_json::from_str(&content[start..=end]).ok()
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::extract_json;

    #[test]
    fn extracts_plain_fenced_and_noisy_json() {
        assert_eq!(extract_json(r#"{"a":1}"#).unwrap()["a"], 1);
        assert_eq!(extract_json("```json\n{\"a\":2}\n```").unwrap()["a"], 2);
        assert_eq!(
            extract_json("Sure! Here you go:\n{\"a\":3}\nHope that helps.").unwrap()["a"],
            3
        );
        assert!(extract_json("no json here").is_none());
    }
}
