//! Day-5 Runtime ↔ Hermes lifecycle tests against the real hermes binary:
//! happy path, crashed worker, silent worker (watchdog), cooperative cancel.

use std::time::Duration;
use wepld_contracts::vocabulary::EventType;
use wepld_contracts::wwp::{
    ArtifactRef, AttemptStart, PhaseBudget, PhaseStatus, RoleProfile, WwpMessage,
};
use wepld_runtime::{Core, PhaseOutcome, PhaseSpec};
use wepld_wwp::{spawn_worker, WorkerEvent};

/// Env is process-global and tests run in parallel — modes are passed per
/// spawned worker through `env(1)`, never via `set_var`.
fn hermes_cmd(mode: &str, hb_ms: u64) -> Vec<String> {
    vec![
        "env".to_owned(),
        format!("WEPLD_HERMES_MODE={mode}"),
        format!("WEPLD_HEARTBEAT_MS={hb_ms}"),
        env!("CARGO_BIN_EXE_hermes").to_owned(),
    ]
}

fn spec(attempt: &str, mode: &str, hb_ms: u64, heartbeat_timeout_ms: u64) -> PhaseSpec {
    PhaseSpec {
        mission_id: "mis_t".to_owned(),
        task_id: "tsk_t".to_owned(),
        attempt_id: attempt.to_owned(),
        phase: "build".to_owned(),
        worker_cmd: hermes_cmd(mode, hb_ms),
        pack: serde_json::json!({ "schema_version": 1, "tiers": {} }),
        brain_profile: "fixture-default".to_owned(),
        workspace_path: None,
        heartbeat_timeout_ms,
        deadline_ms: 10_000,
    }
}

fn entry_types(core: &Core, attempt: &str) -> Vec<EventType> {
    core.all_entries()
        .unwrap()
        .into_iter()
        .filter(|e| e.aggregate_id == attempt)
        .map(|e| e.entry_type)
        .collect()
}

#[test]
fn happy_path_records_full_lifecycle() {
    let dir = tempfile::tempdir().unwrap();
    let mut core = Core::open(dir.path()).unwrap();

    let outcome = core
        .run_phase_stub(&spec("att_ok", "echo", 100, 2000))
        .unwrap();
    assert_eq!(outcome, PhaseOutcome::Succeeded);

    assert_eq!(
        entry_types(&core, "att_ok"),
        vec![
            EventType::AttemptSpawned,
            EventType::PhaseStarted,
            EventType::PhaseCompleted,
            EventType::AttemptCompleted,
        ]
    );
    assert_eq!(core.attempt_state("att_ok").unwrap().unwrap(), "succeeded");
    assert!(core.verify().unwrap().is_valid());
}

#[test]
fn killed_worker_is_recorded_uncertain_never_failed() {
    let dir = tempfile::tempdir().unwrap();
    let mut core = Core::open(dir.path()).unwrap();

    let outcome = core
        .run_phase_stub(&spec("att_die", "die", 100, 2000))
        .unwrap();
    assert!(matches!(outcome, PhaseOutcome::Uncertain(_)));

    let types = entry_types(&core, "att_die");
    assert_eq!(*types.last().unwrap(), EventType::AttemptUncertain);
    assert_eq!(core.attempt_state("att_die").unwrap().unwrap(), "uncertain");

    let entries = core.all_entries().unwrap();
    let uncertain = entries
        .iter()
        .find(|e| e.entry_type == EventType::AttemptUncertain)
        .unwrap();
    assert!(uncertain.payload_json["reason"]
        .as_str()
        .unwrap()
        .contains("without phase.result"));
    assert!(core.verify().unwrap().is_valid());
}

#[test]
fn silent_worker_trips_the_watchdog() {
    let dir = tempfile::tempdir().unwrap();
    let mut core = Core::open(dir.path()).unwrap();

    let outcome = core
        .run_phase_stub(&spec("att_mute", "mute", 100, 300))
        .unwrap();
    match outcome {
        PhaseOutcome::Uncertain(reason) => assert!(reason.contains("heartbeat")),
        other => panic!("expected uncertain, got {other:?}"),
    }
    assert_eq!(
        core.attempt_state("att_mute").unwrap().unwrap(),
        "uncertain"
    );
}

#[test]
fn cancel_is_honored_cooperatively() {
    let start = AttemptStart {
        attempt_id: "att_cxl".to_owned(),
        task_id: "tsk_t".to_owned(),
        phase: "build".to_owned(),
        role_profile: RoleProfile {
            name: "stub".to_owned(),
            version: 0,
            brain_profile: "none".to_owned(),
            skills: vec![],
        },
        context_pack_ref: ArtifactRef {
            artifact: "none".to_owned(),
            hash: "0".repeat(64),
        },
        envelope: serde_json::from_value(serde_json::json!({
            "envelope_id": "env_t", "attempt_id": "att_cxl", "sandbox_tier": "DEV",
            "fs": {"write": [], "read": [], "deny": ["*"]},
            "network": {"mode": "deny"},
            "process": {"max_procs": 4, "max_mem_mb": 128, "cpu_share": 0.5, "timeout_s": 30},
            "secrets": [], "expires_at": ""
        }))
        .unwrap(),
        gates: vec![],
        budget: PhaseBudget {
            max_brain_calls: 0,
            max_wall_minutes: 1,
        },
        idempotency_key: "att_cxl:1".to_owned(),
    };

    let mut handle =
        spawn_worker(&hermes_cmd("hang", 50), &start, Duration::from_millis(2000)).unwrap();
    // Wait for first heartbeat (worker is alive), then cancel.
    loop {
        match handle.events.recv_timeout(Duration::from_secs(5)).unwrap() {
            WorkerEvent::Message(f) => match f.msg {
                WwpMessage::Heartbeat(_) => break,
                _ => continue,
            },
            other => panic!("unexpected before heartbeat: {other:?}"),
        }
    }
    handle.cancel("att_cxl").unwrap();
    loop {
        match handle.events.recv_timeout(Duration::from_secs(5)).unwrap() {
            WorkerEvent::Message(f) => match f.msg {
                WwpMessage::PhaseResult(r) => {
                    assert_eq!(r.status, PhaseStatus::Cancelled);
                    break;
                }
                _ => continue,
            },
            other => panic!("expected cancelled result, got {other:?}"),
        }
    }
}
