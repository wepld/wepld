//! OpenAI-compatible reasoning adapter. Speaks the `/v1/chat/completions`
//! JSON shape used by Ollama, LM Studio, vLLM, llama.cpp's server, and the
//! OpenAI API itself — so WePLD depends on no single provider (charter LLM
//! philosophy; "Hermes + Ollama" is a first-class local mode).
//!
//! **This build is local-loopback-only.** It supports credential-free HTTP to
//! verified loopback hosts (`127.0.0.1`, `localhost`, `::1`) and nothing else:
//! non-loopback HTTP is refused, any API key over HTTP is refused, and HTTPS is
//! refused because no TLS is built in yet. Hosted / API-key support is deferred
//! until a verified-TLS build lands and is tested; `new` returns a typed
//! [`AdapterConfigError`] rather than silently downgrading or leaking a key.

use crate::{Adapter, AdapterError, AdapterRequest, AdapterResponse};
use std::time::{Duration, Instant};
use wepld_contracts::brain::Usage;

/// Why an adapter configuration was refused. None of these variants ever embed
/// the API key value — configuration errors are safe to log.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum AdapterConfigError {
    #[error("malformed provider URL (expected http://host[:port]): {0}")]
    MalformedUrl(String),
    #[error("unsupported URL scheme '{0}' (only http:// to loopback is supported in this build)")]
    UnsupportedScheme(String),
    #[error(
        "HTTPS is not supported in this local-loopback-only build (verified-TLS build is deferred); \
         refusing to reach {0}"
    )]
    HttpsUnsupported(String),
    #[error(
        "plaintext HTTP is only permitted to loopback hosts (127.0.0.1, localhost, ::1); refusing {0}"
    )]
    NonLoopbackHttp(String),
    #[error(
        "an API key must never be sent over plaintext HTTP; use a keyless loopback endpoint \
         (a verified-TLS build is required for hosted/keyed providers)"
    )]
    KeyOverHttp,
}

pub struct OpenAiCompatAdapter {
    name: String,
    base_url: String,
    timeout: Duration,
}

// Manual Debug so a future keyed build can never leak a credential through
// `{:?}`. (This build holds no key, but the guarantee is structural.)
impl std::fmt::Debug for OpenAiCompatAdapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OpenAiCompatAdapter")
            .field("name", &self.name)
            .field("base_url", &self.base_url)
            .field("api_key", &"<redacted>")
            .finish()
    }
}

impl OpenAiCompatAdapter {
    /// `name` is the adapter id a profile routes to (e.g. "ollama").
    /// `base_url` is the server root, e.g. `http://127.0.0.1:11434`.
    ///
    /// Validates the endpoint up front: loopback HTTP only, no key over HTTP,
    /// no HTTPS in this build. Returns a typed error rather than constructing an
    /// adapter that could reach a remote host or transmit a credential in clear.
    pub fn new(
        name: &str,
        base_url: &str,
        api_key: Option<String>,
        timeout: Duration,
    ) -> Result<Self, AdapterConfigError> {
        validate_endpoint(base_url, api_key.is_some())?;
        Ok(Self {
            name: name.to_owned(),
            base_url: base_url.trim_end_matches('/').to_owned(),
            timeout,
        })
    }
}

/// The one place transport policy is decided. Local-loopback-only: parse the
/// scheme and host, then apply the rules. Never inspects or echoes the key.
fn validate_endpoint(base_url: &str, has_key: bool) -> Result<(), AdapterConfigError> {
    let (scheme, rest) = base_url
        .split_once("://")
        .ok_or_else(|| AdapterConfigError::MalformedUrl(base_url.to_owned()))?;
    let scheme = scheme.to_ascii_lowercase();

    // Authority is everything before the first '/', '?', or '#'.
    let authority = rest.split(['/', '?', '#']).next().unwrap_or("").to_string();
    if authority.is_empty() {
        return Err(AdapterConfigError::MalformedUrl(base_url.to_owned()));
    }

    match scheme.as_str() {
        "https" => Err(AdapterConfigError::HttpsUnsupported(base_url.to_owned())),
        "http" => {
            if has_key {
                // Refuse before we ever consider the host — a key never rides HTTP.
                return Err(AdapterConfigError::KeyOverHttp);
            }
            if is_loopback_authority(&authority) {
                Ok(())
            } else {
                Err(AdapterConfigError::NonLoopbackHttp(base_url.to_owned()))
            }
        }
        other => Err(AdapterConfigError::UnsupportedScheme(other.to_owned())),
    }
}

/// True only for verified loopback authorities: `127.0.0.1`, `localhost`, `::1`
/// (bracketed or not), with an optional `:port`. Any other host is remote.
fn is_loopback_authority(authority: &str) -> bool {
    // Strip an IPv6 bracket form first: [::1]:8080 → ::1
    if let Some(after) = authority.strip_prefix('[') {
        let host = after.split(']').next().unwrap_or("");
        return host == "::1";
    }
    // host[:port] — take the host (IPv4/hostname have a single colon at most).
    let host = authority
        .rsplit_once(':')
        .map(|(h, _)| h)
        .unwrap_or(authority);
    matches!(host, "127.0.0.1" | "localhost" | "::1")
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
        // Local-loopback-only: no Authorization header exists in this build, so
        // a credential can never appear in a request or in an error string.
        let mut response = agent
            .post(&url)
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
