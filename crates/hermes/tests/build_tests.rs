//! Day-8 build tests: the full bounded loop end to end through the real
//! hermes binary — create → plan → approve → run — reaching CompletionProposed
//! with real gate execution (exit codes + captured logs) on a real git repo.

use std::path::Path;
use std::process::Command;
use wepld_contracts::command::{Command as WCommand, CommandOutcome};
use wepld_contracts::vocabulary::EventType;
use wepld_providers::{cassette_key, write_cassette_entry};
use wepld_runtime::{builder_pack, command_id_for, planner_pack, Core};

fn hermes_bin() -> String {
    env!("CARGO_BIN_EXE_hermes").to_owned()
}

fn git(dir: &Path, args: &[&str]) {
    let out = Command::new("git")
        .args(args)
        .current_dir(dir)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "git {args:?}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

/// A real git repo whose src/main.rs lacks the version handling the mission adds.
fn fixture_repo(root: &Path) -> std::path::PathBuf {
    let repo = root.join("notes-cli");
    std::fs::create_dir_all(repo.join("src")).unwrap();
    std::fs::write(repo.join("src/main.rs"), "fn main() {}\n").unwrap();
    git(&repo, &["init", "-q", "-b", "main"]);
    git(&repo, &["config", "user.name", "Fixture"]);
    git(&repo, &["config", "user.email", "fixture@local"]);
    git(&repo, &["add", "-A"]);
    git(&repo, &["commit", "-q", "-m", "initial"]);
    repo
}

fn brief(repo: &Path) -> serde_json::Value {
    serde_json::json!({
        "schema_version": 1,
        "mission_id": "mis_build",
        "title": "Add version flag",
        "outcome": "notes-cli prints its version with --version",
        "scope": {
            "repo": repo.to_string_lossy(),
            "base_branch": "main",
            "paths": ["src/**"],
            "forbidden_paths": []
        },
        "acceptance_criteria": [
            { "id": "AC1", "text": "VERSION constant present", "verify": "gate:build" },
            { "id": "AC2", "text": "--version handled", "verify": "gate:test" }
        ],
        "gates_required": ["build", "test"],
        "gate_commands": {
            "build": "grep -q VERSION src/main.rs",
            "test": "grep -q version src/main.rs"
        },
        "autonomy_mode": "bounded_auto",
        "envelope_declared": { "network": "deny", "dependency_install": "ask", "secrets": [] },
        "budget": { "max_cost_usd": 5.0, "max_wall_minutes": 90, "max_interrupts": 3 },
        "classification": "internal",
        "owner": "principal_local"
    })
}

fn plan_doc() -> serde_json::Value {
    serde_json::json!({
        "tasks": [
            { "id": "T1", "title": "add --version flag and VERSION constant", "satisfies": ["AC1", "AC2"] }
        ]
    })
}

const EDITED_MAIN: &str = "fn main() {\n    const VERSION: &str = \"0.1.0\";\n    if std::env::args().any(|a| a == \"--version\") {\n        println!(\"{VERSION}\");\n    }\n}\n";

fn builder_output() -> serde_json::Value {
    serde_json::json!({ "edits": [ { "path": "src/main.rs", "content": EDITED_MAIN } ] })
}

fn hash_of(dir: &Path, value: &serde_json::Value) -> String {
    let cas = wepld_artifacts::Cas::open(&dir.join("hashcalc")).unwrap();
    cas.put(&serde_json::to_vec(value).unwrap()).unwrap().hash
}

fn write_cassettes(store: &Path, brief: &serde_json::Value) {
    let cassette = store.join("cassettes/day8.jsonl");
    // Planner cassette.
    let plan_key = cassette_key(
        "plan",
        &hash_of(store, &planner_pack(brief)),
        "plan.v1",
        "fixture-model",
    );
    write_cassette_entry(&cassette, &plan_key, &plan_doc(), "fixture-model").unwrap();
    // Builder cassette (task spec matches the plan's task as stored).
    let task_spec = plan_doc()["tasks"][0].clone();
    let build_key = cassette_key(
        "build",
        &hash_of(store, &builder_pack(brief, &task_spec)),
        "builder_step.v1",
        "fixture-model",
    );
    write_cassette_entry(&cassette, &build_key, &builder_output(), "fixture-model").unwrap();
}

fn create(core: &mut Core, brief: &serde_json::Value) {
    let cmd = WCommand {
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
fn full_bounded_loop_reaches_completion_proposed() {
    let workdir = tempfile::tempdir().unwrap();
    let store = tempfile::tempdir().unwrap();
    let repo = fixture_repo(workdir.path());
    let brief = brief(&repo);
    write_cassettes(store.path(), &brief);

    let mut core = Core::open(store.path()).unwrap();
    core.set_worker_cmd(vec![hermes_bin()]);

    create(&mut core, &brief);
    assert!(matches!(
        core.plan_mission("mis_build").unwrap(),
        CommandOutcome::Accepted { .. }
    ));
    assert!(matches!(
        core.approve_plan("mis_build").unwrap(),
        CommandOutcome::Accepted { .. }
    ));

    let outcome = core.run_mission("mis_build").unwrap();
    assert!(
        matches!(&outcome, CommandOutcome::Accepted { detail } if detail["state"] == "completion_proposed"),
        "got {outcome:?}"
    );
    assert_eq!(
        core.mission_row("mis_build").unwrap().unwrap().1,
        "completion_proposed"
    );

    // Evidence on the timeline: snapshot, diff artifact, two passing gates,
    // task completed, completion proposed.
    let entries = core.all_entries().unwrap();
    let types: Vec<_> = entries.iter().map(|e| e.entry_type).collect();
    assert!(types.contains(&EventType::WorkspaceSnapshotRecorded));
    assert!(types.contains(&EventType::ArtifactRecorded));
    assert!(types.contains(&EventType::TaskCompleted));
    assert!(types.contains(&EventType::CompletionProposed));

    let gates: Vec<_> = entries
        .iter()
        .filter(|e| e.entry_type == EventType::GateEvaluated)
        .collect();
    assert_eq!(gates.len(), 2);
    assert!(gates.iter().all(|e| e.payload_json["status"] == "passed"));

    // The diff artifact is real and shows the edit.
    let diff_hash = entries
        .iter()
        .find(|e| e.entry_type == EventType::ArtifactRecorded)
        .unwrap()
        .payload_json["hash"]
        .as_str()
        .unwrap()
        .to_owned();
    let diff = String::from_utf8(core.artifact(&diff_hash).unwrap()).unwrap();
    assert!(
        diff.contains("VERSION"),
        "diff should show the added constant"
    );

    // The user's primary worktree was never touched.
    let status = Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(&repo)
        .output()
        .unwrap();
    assert!(String::from_utf8_lossy(&status.stdout).trim().is_empty());

    assert!(core.verify().unwrap().is_valid());

    let tasks = core.tasks("mis_build").unwrap();
    assert_eq!(tasks[0].state, "completed");
}

#[test]
fn failing_gate_keeps_mission_running() {
    let workdir = tempfile::tempdir().unwrap();
    let store = tempfile::tempdir().unwrap();
    let repo = fixture_repo(workdir.path());
    let mut brief = brief(&repo);
    // A gate that can never pass on the edited file.
    brief["gate_commands"]["test"] = serde_json::json!("grep -q NEVER_PRESENT src/main.rs");
    write_cassettes(store.path(), &brief);

    let mut core = Core::open(store.path()).unwrap();
    core.set_worker_cmd(vec![hermes_bin()]);
    create(&mut core, &brief);
    core.plan_mission("mis_build").unwrap();
    core.approve_plan("mis_build").unwrap();

    let outcome = core.run_mission("mis_build").unwrap();
    assert!(
        matches!(&outcome, CommandOutcome::Accepted { detail } if detail["state"] == "running"),
        "got {outcome:?}"
    );
    assert_eq!(core.mission_row("mis_build").unwrap().unwrap().1, "running");

    let entries = core.all_entries().unwrap();
    let gates: Vec<_> = entries
        .iter()
        .filter(|e| e.entry_type == EventType::GateEvaluated)
        .collect();
    assert!(gates.iter().any(|e| e.payload_json["status"] == "failed"));
    assert!(!entries
        .iter()
        .any(|e| e.entry_type == EventType::CompletionProposed));
    assert!(core.verify().unwrap().is_valid());
}
