//! M1-A tests: the OpenAI-compatible adapter against an in-process mock
//! server (no network, no API key — deterministic in CI), plus record mode
//! producing a cassette that replays identically through the fixture adapter.

use std::io::{Read, Write};
use std::net::TcpListener;
use std::time::Duration;
use wepld_providers::{
    cassette_key, Adapter, AdapterConfigError, AdapterRequest, FixtureAdapter, Gateway,
    OpenAiCompatAdapter, Profile, RecordingAdapter, SchemaRegistry,
};

fn loopback(base: &str) -> OpenAiCompatAdapter {
    OpenAiCompatAdapter::new("ollama", base, None, Duration::from_secs(5)).unwrap()
}

/// A tiny OpenAI-compatible server that returns a fixed chat-completion whose
/// message content is `content_json`. Loops accepting connections until the
/// test process exits. Returns its base URL.
fn mock_server(content_json: &'static str) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    std::thread::spawn(move || {
        for stream in listener.incoming() {
            let mut stream = match stream {
                Ok(s) => s,
                Err(_) => continue,
            };
            // Read the request head (enough to consume the client's write).
            let mut buf = [0u8; 4096];
            let _ = stream.read(&mut buf);
            let body = serde_json::json!({
                "id": "chatcmpl-mock",
                "object": "chat.completion",
                "choices": [ {
                    "index": 0,
                    "message": { "role": "assistant", "content": content_json },
                    "finish_reason": "stop"
                } ],
                "usage": { "prompt_tokens": 42, "completion_tokens": 17, "total_tokens": 59 }
            })
            .to_string();
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                body.len(),
                body
            );
            let _ = stream.write_all(response.as_bytes());
            let _ = stream.flush();
        }
    });
    format!("http://{addr}")
}

fn request() -> AdapterRequest {
    AdapterRequest {
        model: "llama3".to_owned(),
        intent: "plan".to_owned(),
        pack: serde_json::json!({ "schema_version": 1, "mission": { "title": "x" } }),
        pack_hash: "abc123".to_owned(),
        output_schema_id: "plan.v1".to_owned(),
    }
}

const PLAN_CONTENT: &str = r#"{"tasks":[{"id":"T1","title":"add flag","satisfies":["AC1"]}]}"#;

#[test]
fn adapter_calls_endpoint_and_parses_structured_output() {
    let base = mock_server(PLAN_CONTENT);
    let adapter = loopback(&base);

    let resp = adapter.invoke(&request()).unwrap();
    assert_eq!(resp.output["tasks"][0]["id"], "T1");
    assert_eq!(resp.usage.provider, "ollama");
    assert_eq!(resp.usage.model, "llama3");
    assert_eq!(resp.usage.tokens_in, 42);
    assert_eq!(resp.usage.tokens_out, 17);
}

#[test]
fn adapter_tolerates_fenced_and_noisy_output() {
    let base = mock_server("Here is your plan:\n```json\n{\"tasks\":[]}\n```\n");
    let adapter = loopback(&base);
    let resp = adapter.invoke(&request()).unwrap();
    assert!(resp.output["tasks"].is_array());
}

#[test]
fn record_mode_produces_a_replayable_cassette() {
    let dir = tempfile::tempdir().unwrap();
    let cassette = dir.path().join("recorded.jsonl");
    let base = mock_server(PLAN_CONTENT);

    // Record: a real adapter wrapped in the recorder.
    let real = loopback(&base);
    let recorder = RecordingAdapter::new(Box::new(real), cassette.clone());
    let recorded = recorder.invoke(&request()).unwrap();
    assert_eq!(recorded.output["tasks"][0]["id"], "T1");
    assert!(cassette.exists(), "record mode wrote a cassette");

    // Replay: the fixture adapter serves the recorded response for the same key,
    // preserving the real provider/model/usage — no server needed.
    let fixture = FixtureAdapter::load(&[dir.path()]).unwrap();
    let replayed = fixture.invoke(&request()).unwrap();
    assert_eq!(
        replayed.output, recorded.output,
        "replay reproduces the recorded output"
    );
    assert_eq!(
        replayed.usage.provider, "ollama",
        "recorded provider preserved"
    );
    assert_eq!(replayed.usage.tokens_in, 42);

    // The cassette key is the deterministic replay key.
    let key = cassette_key("plan", "abc123", "plan.v1", "llama3");
    let line = std::fs::read_to_string(&cassette).unwrap();
    assert!(line.contains(&key));
}

#[test]
fn recorded_cassette_flows_through_the_gateway() {
    let dir = tempfile::tempdir().unwrap();
    let cassette = dir.path().join("g.jsonl");
    let base = mock_server(PLAN_CONTENT);

    // Record against the mock, then serve the cassette through the gateway.
    let recorder = RecordingAdapter::new(Box::new(loopback(&base)), cassette);
    recorder.invoke(&request()).unwrap();

    // The fixture adapter (named "fixture") replays a recording that was made
    // by "ollama"; the recorded provider is preserved in the result.
    let mut gw = Gateway::new(SchemaRegistry::default());
    gw.register_adapter(Box::new(FixtureAdapter::load(&[dir.path()]).unwrap()));
    gw.register_profile(Profile {
        name: "local".to_owned(),
        adapter: "fixture".to_owned(),
        model: "llama3".to_owned(),
        timeout_ms: 5000,
    })
    .unwrap();

    let result = gw
        .invoke(
            "brn_1",
            "local",
            "plan",
            &serde_json::json!({}),
            "abc123",
            "plan.v1",
        )
        .unwrap();
    assert_eq!(result.status, wepld_contracts::brain::BrainStatus::Ok);
    assert_eq!(result.output["tasks"][0]["id"], "T1");
    assert_eq!(result.usage.provider, "ollama");
}

// ── Local-loopback-only transport policy (Blocker 5) ───────────────────────

const SECRET: &str = "sk-super-secret-key-value-do-not-leak";

#[test]
fn loopback_http_without_a_key_is_accepted() {
    for base in [
        "http://127.0.0.1:11434",
        "http://localhost:8080",
        "http://[::1]:1234",
    ] {
        assert!(
            OpenAiCompatAdapter::new("ollama", base, None, Duration::from_secs(5)).is_ok(),
            "loopback keyless HTTP must be accepted: {base}"
        );
    }
}

#[test]
fn loopback_http_with_a_key_is_rejected() {
    let err = OpenAiCompatAdapter::new(
        "ollama",
        "http://127.0.0.1:11434",
        Some(SECRET.to_owned()),
        Duration::from_secs(5),
    )
    .unwrap_err();
    assert_eq!(err, AdapterConfigError::KeyOverHttp);
}

#[test]
fn remote_http_is_rejected() {
    let err = OpenAiCompatAdapter::new(
        "ollama",
        "http://198.51.100.10:11434",
        None,
        Duration::from_secs(5),
    )
    .unwrap_err();
    assert!(matches!(err, AdapterConfigError::NonLoopbackHttp(_)));
    // A hostname that merely contains "localhost" is still remote.
    assert!(matches!(
        OpenAiCompatAdapter::new(
            "x",
            "http://localhost.evil.com",
            None,
            Duration::from_secs(1)
        )
        .unwrap_err(),
        AdapterConfigError::NonLoopbackHttp(_)
    ));
}

#[test]
fn https_is_rejected_without_silent_downgrade() {
    let err = OpenAiCompatAdapter::new("x", "https://api.openai.com", None, Duration::from_secs(1))
        .unwrap_err();
    assert!(matches!(err, AdapterConfigError::HttpsUnsupported(_)));
}

#[test]
fn malformed_and_unsupported_urls_are_rejected() {
    assert!(matches!(
        OpenAiCompatAdapter::new("x", "not a url", None, Duration::from_secs(1)).unwrap_err(),
        AdapterConfigError::MalformedUrl(_)
    ));
    assert!(matches!(
        OpenAiCompatAdapter::new("x", "http://", None, Duration::from_secs(1)).unwrap_err(),
        AdapterConfigError::MalformedUrl(_)
    ));
    assert!(matches!(
        OpenAiCompatAdapter::new("x", "ftp://127.0.0.1", None, Duration::from_secs(1)).unwrap_err(),
        AdapterConfigError::UnsupportedScheme(_)
    ));
}

#[test]
fn adversarial_urls_are_rejected() {
    // userinfo/host confusion, credentials, malformed ports/hosts, remote IPs,
    // unsupported schemes, and query/fragment on the base endpoint.
    let cases = [
        "http://127.0.0.1:80@evil.example",
        "http://localhost@evil.example",
        "http://[::1]@evil.example",
        "http://[::1]evil.example",
        "http://user:pass@127.0.0.1",
        "http://127.0.0.1:notaport",
        "http://198.51.100.7:11434",
        "http://8.8.8.8",
        "ftp://127.0.0.1",
        "gopher://localhost",
        "http://127.0.0.1:11434/v1?x=1",
        "http://127.0.0.1:11434/#frag",
    ];
    for c in cases {
        assert!(
            OpenAiCompatAdapter::new("x", c, None, Duration::from_secs(1)).is_err(),
            "must reject adversarial url: {c}"
        );
    }
}

#[test]
fn userinfo_spoof_is_a_credentials_error_not_treated_as_loopback() {
    let err = OpenAiCompatAdapter::new(
        "x",
        "http://127.0.0.1:80@evil.example",
        None,
        Duration::from_secs(1),
    )
    .unwrap_err();
    assert!(
        matches!(err, AdapterConfigError::UrlContainsCredentials(_)),
        "got {err:?}"
    );
}

#[test]
fn query_or_fragment_on_the_base_is_rejected() {
    assert!(matches!(
        OpenAiCompatAdapter::new(
            "x",
            "http://127.0.0.1:11434/?a=b",
            None,
            Duration::from_secs(1)
        )
        .unwrap_err(),
        AdapterConfigError::UrlHasQueryOrFragment(_)
    ));
}

#[test]
fn an_error_never_contains_the_key() {
    // The key-over-HTTP error, its Debug, and Display must not echo the secret.
    let err = OpenAiCompatAdapter::new(
        "ollama",
        "http://127.0.0.1:11434",
        Some(SECRET.to_owned()),
        Duration::from_secs(5),
    )
    .unwrap_err();
    assert!(!format!("{err}").contains(SECRET));
    assert!(!format!("{err:?}").contains(SECRET));
    // The adapter's own Debug never prints a credential either.
    let adapter = loopback("http://127.0.0.1:11434");
    assert!(format!("{adapter:?}").contains("<redacted>"));
    assert!(!format!("{adapter:?}").contains(SECRET));
}
