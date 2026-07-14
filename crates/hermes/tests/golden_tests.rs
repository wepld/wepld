//! Day-9 golden trace (IADR-0004): the normative `m0-first-mission` flow —
//! create → plan → approve → run → accept --merge — must produce exactly the
//! recorded ledger entry-type sequence. The trace is normalized to entry types
//! (ids, timestamps, hashes elided); a behavior change shows as a diff in
//! fixtures/golden/m0-first-mission.trace. Also the crash micro-drill.

use std::path::Path;
use std::process::Command;
use wepld_contracts::command::{Command as WCommand, CommandOutcome};
use wepld_contracts::vocabulary::EventType;
use wepld_providers::{cassette_key, write_cassette_entry};
use wepld_runtime::{builder_pack, command_id_for, planner_pack, Core};

const EXPECTED_TRACE: &str = include_str!("../../../fixtures/golden/m0-first-mission.trace");

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
        "mission_id": "mis_first",
        "title": "Add version flag",
        "outcome": "notes-cli prints its version with --version",
        "scope": { "repo": repo.to_string_lossy(), "base_branch": "main", "paths": ["src/**"], "forbidden_paths": [] },
        "acceptance_criteria": [
            { "id": "AC1", "text": "VERSION constant present", "verify": "gate:build" },
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

fn plan_doc() -> serde_json::Value {
    serde_json::json!({ "tasks": [ { "id": "T1", "title": "add version flag", "satisfies": ["AC1", "AC2"] } ] })
}

const EDITED_MAIN: &str = "fn main() {\n    const VERSION: &str = \"0.1.0\";\n    if std::env::args().any(|a| a == \"--version\") { println!(\"{VERSION}\"); }\n}\n";

fn hash_of(dir: &Path, value: &serde_json::Value) -> String {
    let cas = wepld_artifacts::Cas::open(&dir.join("hashcalc")).unwrap();
    cas.put(&serde_json::to_vec(value).unwrap()).unwrap().hash
}

fn write_cassettes(store: &Path, brief: &serde_json::Value) {
    let cassette = store.join("cassettes/golden.jsonl");
    let plan_key = cassette_key(
        "plan",
        &hash_of(store, &planner_pack(brief)),
        "plan.v1",
        "fixture-model",
    );
    write_cassette_entry(&cassette, &plan_key, &plan_doc(), "fixture-model").unwrap();
    let task_spec = plan_doc()["tasks"][0].clone();
    let build_key = cassette_key(
        "build",
        &hash_of(store, &builder_pack(brief, &task_spec)),
        "builder_step.v1",
        "fixture-model",
    );
    write_cassette_entry(
        &cassette,
        &build_key,
        &serde_json::json!({ "edits": [ { "path": "src/main.rs", "content": EDITED_MAIN } ] }),
        "fixture-model",
    )
    .unwrap();
}

fn open(store: &Path) -> Core {
    let mut core = Core::open(store).unwrap();
    core.set_worker_cmd(vec![hermes_bin()]);
    core
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
fn m0_first_mission_matches_golden_trace() {
    let workdir = tempfile::tempdir().unwrap();
    let store = tempfile::tempdir().unwrap();
    let repo = fixture_repo(workdir.path());
    let brief = brief(&repo);
    write_cassettes(store.path(), &brief);

    let mut core = open(store.path());
    core.set_fixtures_root(workdir.path());
    create(&mut core, &brief);
    assert!(accepted(core.plan_mission("mis_first").unwrap()));
    assert!(accepted(
        core.approve_plan("mis_first", "principal_local").unwrap()
    ));
    assert!(accepted(core.run_mission("mis_first").unwrap()));
    assert!(accepted(
        core.accept_mission("mis_first", "principal_local").unwrap()
    ));

    // Normalized trace: the entry-type sequence.
    let actual: Vec<String> = core
        .all_entries()
        .unwrap()
        .iter()
        .map(|e| e.entry_type.code().to_owned())
        .collect();
    let expected: Vec<String> = EXPECTED_TRACE
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(str::to_owned)
        .collect();
    assert_eq!(
        actual, expected,
        "ledger trace diverged from the golden file"
    );

    // The mission is accepted, the chain verifies, and the edit is on main.
    assert_eq!(
        core.mission_row("mis_first").unwrap().unwrap().1,
        "accepted"
    );
    assert!(core.verify().unwrap().is_valid());
    // V0 never mutates the primary worktree: the edit lands on a proposal ref,
    // and the working tree stays clean.
    let main = std::fs::read_to_string(repo.join("src/main.rs")).unwrap();
    assert_eq!(main, "fn main() {}\n", "primary worktree is untouched");
    let show = Command::new("git")
        .args(["show", "refs/heads/wepld/mission-mis_first:src/main.rs"])
        .current_dir(&repo)
        .output()
        .unwrap();
    assert!(
        String::from_utf8_lossy(&show.stdout).contains("VERSION"),
        "the proposal ref carries the edit"
    );
}

#[test]
fn crash_during_build_is_uncertain_and_recoverable() {
    let workdir = tempfile::tempdir().unwrap();
    let store = tempfile::tempdir().unwrap();
    let repo = fixture_repo(workdir.path());
    let brief = brief(&repo);
    write_cassettes(store.path(), &brief);

    let mut core = open(store.path());
    core.set_fixtures_root(workdir.path());
    create(&mut core, &brief);
    core.plan_mission("mis_first").unwrap();
    core.approve_plan("mis_first", "principal_local").unwrap();

    // The build worker crashes (exit before producing a result).
    core.set_worker_cmd(vec![
        "env".to_owned(),
        "WEPLD_HERMES_MODE=die".to_owned(),
        hermes_bin(),
    ]);
    let outcome = core.run_mission("mis_first").unwrap();
    assert!(
        matches!(outcome, CommandOutcome::Rejected { .. }),
        "got {outcome:?}"
    );

    // The crash is classified UNCERTAIN — never assumed failed cleanly —
    // the mission stays running, and the ledger still verifies.
    let entries = core.all_entries().unwrap();
    assert!(entries
        .iter()
        .any(|e| e.entry_type == EventType::AttemptUncertain));
    assert!(!entries
        .iter()
        .any(|e| e.entry_type == EventType::CompletionProposed));
    assert_eq!(core.mission_row("mis_first").unwrap().unwrap().1, "running");
    assert!(core.verify().unwrap().is_valid());
}

fn accepted(outcome: CommandOutcome) -> bool {
    matches!(outcome, CommandOutcome::Accepted { .. })
}
