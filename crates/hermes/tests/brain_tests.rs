//! Day-6 brain round-trip tests: Hermes → Runtime → Provider Gateway →
//! fixture cassette → schema-validated result → invocation record.
//! Deterministic end to end; a cassette miss is loud, recorded, and honest.

use wepld_contracts::vocabulary::EventType;
use wepld_providers::{cassette_key, write_cassette_entry};
use wepld_runtime::{Core, PhaseOutcome, PhaseSpec};

fn hermes_cmd(mode: &str) -> Vec<String> {
    vec![
        "env".to_owned(),
        format!("WEPLD_HERMES_MODE={mode}"),
        "WEPLD_HEARTBEAT_MS=100".to_owned(),
        env!("CARGO_BIN_EXE_hermes").to_owned(),
    ]
}

fn pack() -> serde_json::Value {
    serde_json::json!({
        "schema_version": 1,
        "tiers": { "T0": { "mission_title": "brain test", "phase": "build" } }
    })
}

/// The pack hash the Runtime will compute (same bytes → same hash).
fn pack_hash(dir: &std::path::Path) -> String {
    let cas = wepld_artifacts::Cas::open(&dir.join("hashcalc")).unwrap();
    cas.put(&serde_json::to_vec(&pack()).unwrap()).unwrap().hash
}

fn brain_spec(repo: &str, attempt: &str) -> PhaseSpec {
    PhaseSpec {
        mission_id: "mis_b".to_owned(),
        task_id: "tsk_b".to_owned(),
        attempt_id: attempt.to_owned(),
        phase: "build".to_owned(),
        repo: repo.to_owned(),
        worker_cmd: hermes_cmd("brain"),
        pack: pack(),
        brain_profile: "fixture-default".to_owned(),
        workspace_path: None,
        max_brain_calls: 8,
        heartbeat_timeout_ms: 2000,
        deadline_ms: 10_000,
    }
}

#[test]
fn brain_round_trip_replays_cassette_and_records_invocation() {
    let dir = tempfile::tempdir().unwrap();
    let hash = pack_hash(dir.path());
    let key = cassette_key("stub_step", &hash, "phase_summary.v1", "fixture-model");
    write_cassette_entry(
        &dir.path().join("cassettes/day6.jsonl"),
        &key,
        &serde_json::json!({"schema": "phase_summary.v1", "what": "cassette says hi"}),
        "fixture-model",
    )
    .unwrap();

    let mut core = Core::open(dir.path()).unwrap();
    core.set_fixtures_root(dir.path());
    let repo = dir.path().to_string_lossy().into_owned();
    let outcome = core
        .run_phase_stub(&brain_spec(&repo, "att_brain"))
        .unwrap();
    assert_eq!(outcome, PhaseOutcome::Succeeded);

    // Invocation record: pack referenced by hash, response stored in CAS.
    let rows = core.brain_invocations("att_brain").unwrap();
    assert_eq!(rows.len(), 1);
    let row = &rows[0];
    assert_eq!(row.provider, "fixture");
    assert_eq!(row.status, "ok");
    assert_eq!(row.pack_hash, hash);
    let response = core
        .artifact(row.response_artifact.as_ref().unwrap())
        .unwrap();
    let response: serde_json::Value = serde_json::from_slice(&response).unwrap();
    assert_eq!(response["what"], "cassette says hi");

    // BrainInvoked fact on the timeline; phase summary grounded in the answer.
    let entries = core.all_entries().unwrap();
    assert!(entries
        .iter()
        .any(|e| e.entry_type == EventType::BrainInvoked
            && e.payload_json["pack_hash"] == serde_json::json!(hash)));
    let completed = entries
        .iter()
        .find(|e| e.entry_type == EventType::PhaseCompleted)
        .unwrap();
    assert_eq!(
        completed.payload_json["summary"]["what"],
        "cassette says hi"
    );
    assert!(core.verify().unwrap().is_valid());
}

#[test]
fn cassette_miss_is_loud_recorded_and_fails_the_phase() {
    let dir = tempfile::tempdir().unwrap();
    // No cassette written: the gateway must miss loudly, never improvise.
    let mut core = Core::open(dir.path()).unwrap();
    core.set_fixtures_root(dir.path());
    let repo = dir.path().to_string_lossy().into_owned();
    let outcome = core.run_phase_stub(&brain_spec(&repo, "att_miss")).unwrap();
    assert_eq!(outcome, PhaseOutcome::Failed);

    let rows = core.brain_invocations("att_miss").unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].status, "provider_error");

    // The failure is on the timeline in both the invocation and the summary.
    let entries = core.all_entries().unwrap();
    let completed = entries
        .iter()
        .find(|e| e.entry_type == EventType::PhaseCompleted)
        .unwrap();
    assert!(completed.payload_json["summary"]["what"]
        .as_str()
        .unwrap()
        .contains("reasoning unavailable"));
    assert!(core.verify().unwrap().is_valid());
}

#[test]
fn deterministic_phase_makes_zero_brain_calls() {
    // IADR-0007 §1: echo mode completes with an empty invocation table —
    // a first-class execution, not a degraded one.
    let dir = tempfile::tempdir().unwrap();
    let mut core = Core::open(dir.path()).unwrap();
    core.set_fixtures_root(dir.path());
    let repo = dir.path().to_string_lossy().into_owned();
    let mut spec = brain_spec(&repo, "att_zero");
    spec.worker_cmd = hermes_cmd("echo");
    let outcome = core.run_phase_stub(&spec).unwrap();
    assert_eq!(outcome, PhaseOutcome::Succeeded);
    assert!(core.brain_invocations("att_zero").unwrap().is_empty());
}
