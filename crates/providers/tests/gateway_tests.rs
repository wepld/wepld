//! Day-6 gateway tests: deterministic replay, loud cassette miss, schema
//! validation fail-closed, profile resolution.

use wepld_contracts::brain::BrainStatus;
use wepld_providers::{
    cassette_key, write_cassette_entry, FixtureAdapter, Gateway, Profile, SchemaRegistry,
};

fn gateway_with_cassette(dir: &std::path::Path, entries: &[(&str, serde_json::Value)]) -> Gateway {
    let path = dir.join("test.jsonl");
    for (key, output) in entries {
        write_cassette_entry(&path, key, output, "fixture-model").unwrap();
    }
    let mut gw = Gateway::new(SchemaRegistry::default());
    gw.register_adapter(Box::new(FixtureAdapter::load(&[dir]).unwrap()));
    gw.register_profile(Profile {
        name: "fixture-default".to_owned(),
        adapter: "fixture".to_owned(),
        model: "fixture-model".to_owned(),
        timeout_ms: 1000,
    })
    .unwrap();
    gw
}

const PACK_HASH: &str = "abc123";

#[test]
fn replay_is_deterministic() {
    let dir = tempfile::tempdir().unwrap();
    let key = cassette_key("stub_step", PACK_HASH, "phase_summary.v1", "fixture-model");
    let gw = gateway_with_cassette(
        dir.path(),
        &[(
            key.as_str(),
            serde_json::json!({"schema": "phase_summary.v1", "what": "recorded answer"}),
        )],
    );

    let a = gw
        .invoke(
            "brn_1",
            "fixture-default",
            "stub_step",
            &serde_json::json!({}),
            PACK_HASH,
            "phase_summary.v1",
        )
        .unwrap();
    let b = gw
        .invoke(
            "brn_2",
            "fixture-default",
            "stub_step",
            &serde_json::json!({}),
            PACK_HASH,
            "phase_summary.v1",
        )
        .unwrap();

    assert_eq!(a.status, BrainStatus::Ok);
    assert_eq!(a.output, b.output, "same key must replay identically");
    assert_eq!(a.usage.provider, "fixture");
}

#[test]
fn cassette_miss_fails_loudly_never_silently() {
    let dir = tempfile::tempdir().unwrap();
    let gw = gateway_with_cassette(dir.path(), &[]);

    let r = gw
        .invoke(
            "brn_1",
            "fixture-default",
            "stub_step",
            &serde_json::json!({}),
            PACK_HASH,
            "phase_summary.v1",
        )
        .unwrap();
    assert_eq!(r.status, BrainStatus::ProviderError);
    assert!(r.reason.unwrap().contains("cassette miss"));
    assert_eq!(
        r.output,
        serde_json::json!({}),
        "no improvised output, ever"
    );
}

#[test]
fn schema_validation_fails_closed() {
    let dir = tempfile::tempdir().unwrap();
    let key = cassette_key("stub_step", PACK_HASH, "phase_summary.v1", "fixture-model");
    // Recorded output is missing the required "what" field.
    let gw = gateway_with_cassette(
        dir.path(),
        &[(
            key.as_str(),
            serde_json::json!({"schema": "phase_summary.v1"}),
        )],
    );

    let r = gw
        .invoke(
            "brn_1",
            "fixture-default",
            "stub_step",
            &serde_json::json!({}),
            PACK_HASH,
            "phase_summary.v1",
        )
        .unwrap();
    assert_eq!(r.status, BrainStatus::SchemaInvalid);
    assert!(r.reason.unwrap().contains("what"));

    // Unknown schema ids also fail closed.
    let r = gw
        .invoke(
            "brn_2",
            "fixture-default",
            "stub_step",
            &serde_json::json!({}),
            PACK_HASH,
            "no_such_schema.v9",
        )
        .unwrap();
    assert_eq!(r.status, BrainStatus::SchemaInvalid);
}

#[test]
fn unknown_profile_is_an_error_not_a_guess() {
    let dir = tempfile::tempdir().unwrap();
    let gw = gateway_with_cassette(dir.path(), &[]);
    assert!(gw
        .invoke(
            "brn_1",
            "gpt-please",
            "x",
            &serde_json::json!({}),
            PACK_HASH,
            "phase_summary.v1"
        )
        .is_err());
}
