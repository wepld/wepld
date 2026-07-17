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
        "plaintext HTTP is only permitted to loopback hosts (127.0.0.1, ::1, or a loopback-resolving localhost); refusing {0}"
    )]
    NonLoopbackHttp(String),
    #[error("provider URL must not contain a username or password: {0}")]
    UrlContainsCredentials(String),
    #[error("provider base URL must not contain a query or fragment: {0}")]
    UrlHasQueryOrFragment(String),
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
        // Validate and normalize: store the parsed authority, never the original
        // ambiguous string, so `invoke` cannot be tricked by userinfo/host tricks.
        let normalized = validate_endpoint(base_url, api_key.is_some())?;
        Ok(Self {
            name: name.to_owned(),
            base_url: normalized,
            timeout,
        })
    }
}

/// The one place transport policy is decided, using a standards-compliant URL
/// parser (`url::Url`) so userinfo cannot masquerade as a loopback host
/// (`http://127.0.0.1:80@evil.example`). Returns the normalized `scheme://host[:port]`
/// base (no path/query/fragment). Never inspects or echoes the API key.
fn validate_endpoint(base_url: &str, has_key: bool) -> Result<String, AdapterConfigError> {
    use url::{Host, Url};

    let url =
        Url::parse(base_url).map_err(|_| AdapterConfigError::MalformedUrl(base_url.to_owned()))?;

    match url.scheme() {
        "http" => {}
        "https" => return Err(AdapterConfigError::HttpsUnsupported(base_url.to_owned())),
        other => return Err(AdapterConfigError::UnsupportedScheme(other.to_owned())),
    }
    // A credential never rides plaintext HTTP.
    if has_key {
        return Err(AdapterConfigError::KeyOverHttp);
    }
    // Reject embedded userinfo — the classic loopback-spoofing vector.
    if !url.username().is_empty() || url.password().is_some() {
        return Err(AdapterConfigError::UrlContainsCredentials(
            base_url.to_owned(),
        ));
    }
    // The base endpoint carries no query or fragment.
    if url.query().is_some() || url.fragment().is_some() {
        return Err(AdapterConfigError::UrlHasQueryOrFragment(
            base_url.to_owned(),
        ));
    }

    // The host must be a real loopback address (numeric IPs preferred), or a
    // `localhost` that resolves to loopback only.
    let host = url
        .host()
        .ok_or_else(|| AdapterConfigError::MalformedUrl(base_url.to_owned()))?;
    let host_ok = match &host {
        Host::Ipv4(ip) => ip.is_loopback(),
        Host::Ipv6(ip) => ip.is_loopback(),
        Host::Domain(d) => d.eq_ignore_ascii_case("localhost") && localhost_is_loopback_only(),
    };
    if !host_ok {
        return Err(AdapterConfigError::NonLoopbackHttp(base_url.to_owned()));
    }

    // Normalized base from the parsed components — not the original string.
    let host_str = url
        .host_str()
        .ok_or_else(|| AdapterConfigError::MalformedUrl(base_url.to_owned()))?;
    let port = url.port().map(|p| format!(":{p}")).unwrap_or_default();
    Ok(format!("http://{host_str}{port}"))
}

/// Documented `localhost` policy: it is accepted only if every address it
/// resolves to is a loopback address (no split-horizon surprise). If resolution
/// fails or yields any non-loopback address, `localhost` is refused.
fn localhost_is_loopback_only() -> bool {
    use std::net::ToSocketAddrs;
    match ("localhost", 0u16).to_socket_addrs() {
        Ok(addrs) => {
            let mut any = false;
            for a in addrs {
                any = true;
                if !a.ip().is_loopback() {
                    return false;
                }
            }
            any
        }
        Err(_) => false,
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
