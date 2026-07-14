//! M0 Release Gate — adversarial tests. Each attempts to prove incorrectness;
//! the assertions encode the defended behavior. Run against the real hermes
//! binary and a real store/git so nothing is mocked away.

use std::path::Path;
use wepld_contracts::command::{Command as WCommand, CommandOutcome};
use wepld_contracts::vocabulary::EventType;
use wepld_providers::{cassette_key, write_cassette_entry};
use wepld_runtime::{command_id_for, planner_pack, Core};

fn hermes_bin() -> String {
    env!("CARGO_BIN_EXE_hermes").to_owned()
}

fn hermes_mode(mode: &str) -> Vec<String> {
    vec![
        "env".to_owned(),
        format!("WEPLD_HERMES_MODE={mode}"),
        "WEPLD_HEARTBEAT_MS=50".to_owned(),
        hermes_bin(),
    ]
}

fn brief(repo: &str, id: &str) -> serde_json::Value {
    serde_json::json!({
        "schema_version": 1,
        "mission_id": id,
        "title": "Add version flag",
        "outcome": "print version with --version",
        "scope": { "repo": repo, "base_branch": "main", "paths": ["src/**"], "forbidden_paths": [] },
        "acceptance_criteria": [ { "id": "AC1", "text": "x", "verify": "gate:build" } ],
        "gates_required": ["build"],
        "gate_commands": { "build": "true" },
        "autonomy_mode": "bounded_auto",
        "envelope_declared": { "network": "deny", "dependency_install": "ask", "secrets": [] },
        "budget": { "max_cost_usd": 5.0, "max_wall_minutes": 90, "max_interrupts": 3 },
        "classification": "internal",
        "owner": "principal_local"
    })
}

fn create(core: &mut Core, brief: &serde_json::Value) -> CommandOutcome {
    let cmd = WCommand {
        command_id: command_id_for("create_mission", brief),
        command_type: "create_mission".to_owned(),
        actor: "principal_local".to_owned(),
        payload: brief.clone(),
    };
    core.submit(&cmd).unwrap()
}

fn hash_of(dir: &Path, value: &serde_json::Value) -> String {
    let cas = wepld_artifacts::Cas::open(&dir.join("hashcalc")).unwrap();
    cas.put(&serde_json::to_vec(value).unwrap()).unwrap().hash
}

fn write_plan_cassette(store: &Path, brief: &serde_json::Value) {
    let key = cassette_key(
        "plan",
        &hash_of(store, &planner_pack(brief)),
        "plan.v1",
        "fixture-model",
    );
    let plan =
        serde_json::json!({ "tasks": [ { "id": "T1", "title": "t", "satisfies": ["AC1"] } ] });
    write_cassette_entry(
        &store.join("cassettes/p.jsonl"),
        &key,
        &plan,
        "fixture-model",
    )
    .unwrap();
}

// ── 1. Brain-call budget is enforced (worker request spam is bounded) ────────
#[test]
fn brain_call_budget_is_enforced() {
    let store = tempfile::tempdir().unwrap();
    let mut core = Core::open(store.path()).unwrap();
    core.set_worker_cmd(hermes_mode("brainspam"));
    create(&mut core, &brief("/tmp/x", "mis_spam"));

    // The planner floods brain.requests; the Core caps them and fails the phase.
    let outcome = core.plan_mission("mis_spam").unwrap();
    assert!(
        matches!(outcome, CommandOutcome::Rejected { .. }),
        "got {outcome:?}"
    );

    let invocations = core.brain_invocations("att_mis_spam_plan").unwrap();
    assert!(
        invocations.len() <= 8,
        "budget bounded recorded calls, got {}",
        invocations.len()
    );
    assert!(core.all_entries().unwrap().iter().any(|e| {
        e.entry_type == EventType::AttemptCompleted
            && e.payload_json["reason"]
                .as_str()
                .unwrap_or("")
                .contains("budget")
    }));
    assert_eq!(core.mission_row("mis_spam").unwrap().unwrap().1, "draft");
    assert!(core.verify().unwrap().is_valid());
}

// ── 2. Invalid state transitions are rejected, state unchanged ───────────────
#[test]
fn invalid_transitions_are_rejected() {
    let store = tempfile::tempdir().unwrap();
    let b = brief("/tmp/x", "mis_x");
    write_plan_cassette(store.path(), &b); // cassettes load at Core::open
    let mut core = Core::open(store.path()).unwrap();
    core.set_worker_cmd(vec![hermes_bin()]);
    create(&mut core, &b);

    // run before approve; approve before plan; accept before completion.
    assert!(matches!(
        core.run_mission("mis_x").unwrap(),
        CommandOutcome::Rejected { .. }
    ));
    assert!(matches!(
        core.approve_plan("mis_x", "principal_local").unwrap(),
        CommandOutcome::Rejected { .. }
    ));
    assert!(matches!(
        core.accept_mission("mis_x", "principal_local").unwrap(),
        CommandOutcome::Rejected { .. }
    ));
    assert_eq!(core.mission_row("mis_x").unwrap().unwrap().1, "draft");

    // Plan, then a second plan is rejected (not in draft anymore).
    assert!(matches!(
        core.plan_mission("mis_x").unwrap(),
        CommandOutcome::Accepted { .. }
    ));
    assert!(matches!(
        core.plan_mission("mis_x").unwrap(),
        CommandOutcome::Rejected { .. }
    ));
    // Approve, then double approve is rejected.
    assert!(matches!(
        core.approve_plan("mis_x", "principal_local").unwrap(),
        CommandOutcome::Accepted { .. }
    ));
    assert!(matches!(
        core.approve_plan("mis_x", "principal_local").unwrap(),
        CommandOutcome::Rejected { .. }
    ));
    assert!(core.verify().unwrap().is_valid());
}

// ── 3. Duplicate mission id cannot create a second mission ───────────────────
#[test]
fn duplicate_mission_id_is_rejected() {
    let store = tempfile::tempdir().unwrap();
    let mut core = Core::open(store.path()).unwrap();

    let first = brief("/tmp/x", "mis_dup");
    let mut second = first.clone();
    second["title"] = serde_json::json!("different title, same id");

    assert!(matches!(
        create(&mut core, &first),
        CommandOutcome::Accepted { .. }
    ));
    assert!(matches!(
        create(&mut core, &second),
        CommandOutcome::Rejected { .. }
    ));

    let created = core
        .all_entries()
        .unwrap()
        .into_iter()
        .filter(|e| e.entry_type == EventType::MissionCreated)
        .count();
    assert_eq!(created, 1, "exactly one mission created");
    assert!(core.verify().unwrap().is_valid());
}

// ── 4. A worker that emits non-protocol bytes is classified UNCERTAIN ────────
#[test]
fn garbage_worker_is_uncertain() {
    let store = tempfile::tempdir().unwrap();
    let mut core = Core::open(store.path()).unwrap();
    core.set_worker_cmd(hermes_mode("garbage"));
    create(&mut core, &brief("/tmp/x", "mis_g"));

    let outcome = core.plan_mission("mis_g").unwrap();
    assert!(
        matches!(outcome, CommandOutcome::Rejected { .. }),
        "got {outcome:?}"
    );
    assert!(core
        .all_entries()
        .unwrap()
        .iter()
        .any(|e| e.entry_type == EventType::AttemptUncertain));
    assert_eq!(core.mission_row("mis_g").unwrap().unwrap().1, "draft");
    assert!(core.verify().unwrap().is_valid());
}

// ── 5. A bad repository path is a clean rejection, not corruption ────────────
#[test]
fn bad_repo_path_is_clean_rejection() {
    let store = tempfile::tempdir().unwrap();
    let mut brief = brief("/nonexistent/not-a-repo", "mis_b");
    // Manual + allow the (bad) path past the DEV gate so we exercise the
    // workspace-open failure specifically, not the tier caps.
    brief["autonomy_mode"] = serde_json::json!("manual");
    write_plan_cassette(store.path(), &brief); // cassettes load at Core::open
    let mut core = Core::open(store.path()).unwrap();
    core.set_worker_cmd(vec![hermes_bin()]);
    core.set_fixtures_root(Path::new("/"));
    create(&mut core, &brief);
    core.plan_mission("mis_b").unwrap();
    core.approve_plan("mis_b", "principal_local").unwrap();

    let outcome = core.run_mission("mis_b").unwrap();
    assert!(
        matches!(&outcome, CommandOutcome::Rejected { reason } if reason.contains("repository")),
        "got {outcome:?}"
    );
    assert_eq!(core.mission_row("mis_b").unwrap().unwrap().1, "running");
    assert!(core.verify().unwrap().is_valid());
}

// ── 6. Tampering with any entry after a real flow is detected ────────────────
#[test]
fn tampering_after_flow_is_detected() {
    let store = tempfile::tempdir().unwrap();
    let mut core = Core::open(store.path()).unwrap();
    core.set_worker_cmd(vec![hermes_bin()]);
    let brief = brief("/tmp/x", "mis_t");
    create(&mut core, &brief);
    write_plan_cassette(store.path(), &brief);
    core.plan_mission("mis_t").unwrap();
    assert!(core.verify().unwrap().is_valid());
    drop(core);

    // Out-of-band tamper: drop the guard trigger, mutate a middle entry.
    let raw = rusqlite::Connection::open(store.path().join("wepld.db")).unwrap();
    raw.execute_batch("DROP TRIGGER ledger_no_update;").unwrap();
    raw.execute(
        "UPDATE ledger SET payload_json = '{\"forged\":true}' WHERE seq = 2",
        [],
    )
    .unwrap();
    drop(raw);

    let core = Core::open(store.path()).unwrap();
    let report = core.verify().unwrap();
    assert_eq!(report.broken_at, Some(2), "tamper at seq 2 detected");
}

// ── 7. A second Core can open the store after the first is dropped ───────────
#[test]
fn store_reopens_cleanly() {
    let store = tempfile::tempdir().unwrap();
    {
        let mut core = Core::open(store.path()).unwrap();
        create(&mut core, &brief("/tmp/x", "mis_r"));
    }
    let core = Core::open(store.path()).unwrap();
    assert_eq!(core.mission_row("mis_r").unwrap().unwrap().1, "draft");
    assert!(core.verify().unwrap().is_valid());
    // The tier fact is not duplicated on reopen.
    let tiers = core
        .all_entries()
        .unwrap()
        .into_iter()
        .filter(|e| e.entry_type == EventType::SandboxTierDetected)
        .count();
    assert_eq!(tiers, 1);
}
