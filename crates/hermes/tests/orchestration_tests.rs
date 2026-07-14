//! Day-7 orchestration tests: planner phase → PlanProposed, then approval →
//! task rows + PlanApproved. End to end through the real hermes binary and
//! deterministic plan cassette.

use wepld_contracts::command::{Command, CommandOutcome};
use wepld_contracts::vocabulary::EventType;
use wepld_providers::{cassette_key, write_cassette_entry};
use wepld_runtime::{command_id_for, planner_pack, Core};

fn hermes_bin() -> String {
    env!("CARGO_BIN_EXE_hermes").to_owned()
}

fn brief(repo: &str) -> serde_json::Value {
    serde_json::json!({
        "schema_version": 1,
        "mission_id": "mis_orch",
        "title": "Add version flag",
        "outcome": "notes-cli prints its version with --version",
        "scope": { "repo": repo, "base_branch": "main", "paths": ["src/**"], "forbidden_paths": [] },
        "acceptance_criteria": [
            { "id": "AC1", "text": "version constant present", "verify": "gate:build" },
            { "id": "AC2", "text": "--version handled", "verify": "gate:test" }
        ],
        "gates_required": ["build", "test"],
        "gate_commands": { "build": "grep -q VERSION src/main.rs", "test": "grep -q version src/main.rs" },
        "autonomy_mode": "manual",
        "envelope_declared": { "network": "deny", "dependency_install": "ask", "secrets": [] },
        "budget": { "max_cost_usd": 5.0, "max_wall_minutes": 90, "max_interrupts": 3 },
        "classification": "internal",
        "owner": "principal_local"
    })
}

/// A real git repo under `dir` so DEV mission-creation authorization can pass
/// (the preflight requires a canonicalizable repository within the fixtures
/// root). Returns its path string.
fn real_repo(dir: &std::path::Path) -> String {
    let repo = dir.join("repo");
    std::fs::create_dir_all(repo.join("src")).unwrap();
    std::fs::write(repo.join("src/main.rs"), "fn main() {}\n").unwrap();
    for a in [
        &["init", "-q", "-b", "main"][..],
        &["config", "user.name", "t"],
        &["config", "user.email", "t@l"],
        &["add", "-A"],
        &["commit", "-q", "-m", "i"],
    ] {
        std::process::Command::new("git")
            .args(a)
            .current_dir(&repo)
            .output()
            .unwrap();
    }
    repo.to_string_lossy().into_owned()
}

fn plan_doc() -> serde_json::Value {
    serde_json::json!({
        "tasks": [
            { "id": "T1", "title": "add --version flag and VERSION constant", "satisfies": ["AC1", "AC2"] }
        ]
    })
}

fn hash_of(dir: &std::path::Path, value: &serde_json::Value) -> String {
    let cas = wepld_artifacts::Cas::open(&dir.join("hashcalc")).unwrap();
    cas.put(&serde_json::to_vec(value).unwrap()).unwrap().hash
}

fn create(core: &mut Core, brief: &serde_json::Value) {
    let cmd = Command {
        command_id: command_id_for("create_mission", brief),
        command_type: "create_mission".to_owned(),
        actor: "principal_local".to_owned(),
        payload: brief.clone(),
    };
    assert!(matches!(
        core.submit(&cmd).unwrap(),
        CommandOutcome::Accepted { .. }
    ));
}

#[test]
fn plan_then_approve_produces_tasks() {
    let dir = tempfile::tempdir().unwrap();
    let brief = brief(&real_repo(dir.path()));

    // Record the plan the planner's brain will "return".
    let pack_hash = hash_of(dir.path(), &planner_pack(&brief));
    let key = cassette_key("plan", &pack_hash, "plan.v1", "fixture-model");
    write_cassette_entry(
        &dir.path().join("cassettes/plan.jsonl"),
        &key,
        &plan_doc(),
        "fixture-model",
    )
    .unwrap();

    let mut core = Core::open(dir.path()).unwrap();
    core.set_worker_cmd(vec![hermes_bin()]);
    core.set_fixtures_root(dir.path());
    create(&mut core, &brief);

    // Plan.
    let outcome = core.plan_mission("mis_orch").unwrap();
    assert!(
        matches!(&outcome, CommandOutcome::Accepted { detail } if detail["task_count"] == 1),
        "got {outcome:?}"
    );
    assert_eq!(
        core.mission_row("mis_orch").unwrap().unwrap().1,
        "plan_review"
    );
    let entries = core.all_entries().unwrap();
    assert!(entries
        .iter()
        .any(|e| e.entry_type == EventType::PlanProposed));

    // Approve.
    let outcome = core.approve_plan("mis_orch", "principal_local").unwrap();
    assert!(matches!(outcome, CommandOutcome::Accepted { .. }));
    assert_eq!(core.mission_row("mis_orch").unwrap().unwrap().1, "running");

    let tasks = core.tasks("mis_orch").unwrap();
    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].task_id, "mis_orch_T1");
    assert_eq!(tasks[0].state, "ready");

    let entries = core.all_entries().unwrap();
    assert!(entries
        .iter()
        .any(|e| e.entry_type == EventType::PlanApproved));
    assert!(core.verify().unwrap().is_valid());
}

#[test]
fn plan_rejected_when_not_draft() {
    let dir = tempfile::tempdir().unwrap();
    let mut core = Core::open(dir.path()).unwrap();
    core.set_worker_cmd(vec![hermes_bin()]);
    core.set_fixtures_root(dir.path());

    // Missing mission.
    assert!(matches!(
        core.plan_mission("mis_ghost").unwrap(),
        CommandOutcome::Rejected { .. }
    ));

    // Approve before plan.
    create(&mut core, &brief(&real_repo(dir.path())));
    assert!(matches!(
        core.approve_plan("mis_orch", "principal_local").unwrap(),
        CommandOutcome::Rejected { .. }
    ));
}

#[test]
fn plan_rejected_on_cassette_miss() {
    let dir = tempfile::tempdir().unwrap();
    let mut core = Core::open(dir.path()).unwrap();
    core.set_worker_cmd(vec![hermes_bin()]);
    core.set_fixtures_root(dir.path());
    create(&mut core, &brief(&real_repo(dir.path())));

    // No cassette: planner phase fails loudly, mission stays draft.
    let outcome = core.plan_mission("mis_orch").unwrap();
    assert!(
        matches!(outcome, CommandOutcome::Rejected { .. }),
        "got {outcome:?}"
    );
    assert_eq!(core.mission_row("mis_orch").unwrap().unwrap().1, "draft");
}
