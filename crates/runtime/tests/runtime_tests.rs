//! Day-3 command-pipeline tests: acceptance, idempotency, rejection paths,
//! and the DEV tier disclosure fact.

use wepld_contracts::command::{Command, CommandOutcome};
use wepld_contracts::vocabulary::EventType;
use wepld_runtime::{command_id_for, Core};

const BRIEF: &str = include_str!("../../contracts/tests/fixtures/mission_rate_limiting.json");

fn open_core() -> (tempfile::TempDir, Core) {
    let dir = tempfile::tempdir().unwrap();
    let core = Core::open(dir.path()).unwrap();
    (dir, core)
}

fn create_cmd(payload: serde_json::Value) -> Command {
    Command {
        command_id: command_id_for("create_mission", &payload),
        command_type: "create_mission".to_owned(),
        actor: "principal_local".to_owned(),
        payload,
    }
}

#[test]
fn tier_fact_is_recorded_once_at_init() {
    let (dir, core) = open_core();
    let entries = core.all_entries().unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].entry_type, EventType::SandboxTierDetected);
    assert_eq!(entries[0].payload_json["tier"], "DEV");
    drop(core);

    // Re-open: no duplicate tier fact.
    let core = Core::open(dir.path()).unwrap();
    assert_eq!(core.all_entries().unwrap().len(), 1);
}

#[test]
fn create_mission_accepted_and_recorded() {
    let (_dir, mut core) = open_core();
    let payload: serde_json::Value = serde_json::from_str(BRIEF).unwrap();
    let outcome = core.submit(&create_cmd(payload)).unwrap();

    assert!(matches!(outcome, CommandOutcome::Accepted { .. }));
    let timeline = core.timeline("mis_01J8QZ3F0000000000000000").unwrap();
    assert_eq!(timeline.len(), 1);
    assert_eq!(timeline[0].entry_type, EventType::MissionCreated);
    assert_eq!(
        timeline[0].causation_ref.as_deref(),
        Some(
            create_cmd(serde_json::from_str(BRIEF).unwrap())
                .command_id
                .as_str()
        )
    );
    let (_, state) = core
        .mission_row("mis_01J8QZ3F0000000000000000")
        .unwrap()
        .unwrap();
    assert_eq!(state, "draft");
    assert!(core.verify().unwrap().is_valid());
}

#[test]
fn duplicate_command_is_idempotent() {
    let (_dir, mut core) = open_core();
    let payload: serde_json::Value = serde_json::from_str(BRIEF).unwrap();
    let first = core.submit(&create_cmd(payload.clone())).unwrap();
    let second = core.submit(&create_cmd(payload)).unwrap();

    assert_eq!(first, second, "replay must return the stored outcome");
    // Exactly one MissionCreated despite two submissions.
    let created: Vec<_> = core
        .all_entries()
        .unwrap()
        .into_iter()
        .filter(|e| e.entry_type == EventType::MissionCreated)
        .collect();
    assert_eq!(created.len(), 1);
}

#[test]
fn command_id_reuse_with_different_payload_is_rejected() {
    let (_dir, mut core) = open_core();
    let payload: serde_json::Value = serde_json::from_str(BRIEF).unwrap();
    let cmd = create_cmd(payload.clone());
    core.submit(&cmd).unwrap();

    let mut altered = payload;
    altered["title"] = serde_json::json!("Different title");
    let reused = Command {
        payload: altered,
        ..cmd
    };
    let outcome = core.submit(&reused).unwrap();
    assert!(
        matches!(outcome, CommandOutcome::Rejected { ref reason } if reason.contains("reused")),
        "got {outcome:?}"
    );
}

#[test]
fn missing_verify_method_is_rejected_without_ledger_noise() {
    let (_dir, mut core) = open_core();
    let mut payload: serde_json::Value = serde_json::from_str(BRIEF).unwrap();
    payload["acceptance_criteria"][0]["verify"] = serde_json::json!("");
    let outcome = core.submit(&create_cmd(payload)).unwrap();

    assert!(
        matches!(outcome, CommandOutcome::Rejected { ref reason } if reason.contains("verify")),
        "got {outcome:?}"
    );
    // Only the tier fact exists — rejections append nothing.
    assert_eq!(core.all_entries().unwrap().len(), 1);
}

#[test]
fn duplicate_mission_and_unknown_command_are_rejected() {
    let (_dir, mut core) = open_core();
    let payload: serde_json::Value = serde_json::from_str(BRIEF).unwrap();
    core.submit(&create_cmd(payload.clone())).unwrap();

    // Same mission_id, different command_id (title tweak changes the hash).
    let mut second = payload.clone();
    second["title"] = serde_json::json!("Same mission, second brief");
    let outcome = core.submit(&create_cmd(second)).unwrap();
    assert!(
        matches!(outcome, CommandOutcome::Rejected { ref reason } if reason.contains("exists"))
    );

    let unknown = Command {
        command_id: "cmd_x".to_owned(),
        command_type: "launch_rockets".to_owned(),
        actor: "principal_local".to_owned(),
        payload: serde_json::json!({}),
    };
    let outcome = core.submit(&unknown).unwrap();
    assert!(
        matches!(outcome, CommandOutcome::Rejected { ref reason } if reason.contains("unknown command"))
    );
}
