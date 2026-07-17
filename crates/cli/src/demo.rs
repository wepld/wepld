//! `wepld demo` — a self-contained showcase of the full M0 bounded loop on a
//! bundled fixture repository. It sets up a scratch git repo and store, records
//! the deterministic reasoning it will replay, then drives create → plan →
//! approve → run → accept in front of the user. No API keys, no network.

use std::error::Error;
use std::path::Path;
use std::process::Command;
use wepld_contracts::command::{Command as WCommand, CommandOutcome};
use wepld_providers::{cassette_key, write_cassette_entry};
use wepld_runtime::{builder_pack, command_id_for, planner_pack, Core};

const FIXTURE_MAIN: &str = include_str!("../../../fixtures/repos/notes-cli/src/main.rs");
const FIXTURE_BRIEF: &str = include_str!("../../../fixtures/missions/add-version-flag.json");

/// The plan the planner's brain "returns" and the edit the builder's brain
/// "returns" — deterministic, so the demo never depends on a live provider.
const PLAN: &str =
    r#"{"tasks":[{"id":"T1","title":"add a --version flag","satisfies":["AC1","AC2"]}]}"#;
const EDITED_MAIN: &str = "fn main() {\n    const VERSION: &str = \"0.1.0\";\n    if std::env::args().any(|a| a == \"--version\") {\n        println!(\"{VERSION}\");\n        return;\n    }\n    println!(\"notes-cli\");\n}\n";

pub fn run(worker_cmd: Vec<String>) -> Result<(), Box<dyn Error>> {
    let scratch = std::env::temp_dir().join(format!("wepld-demo-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&scratch);
    let repo = scratch.join("notes-cli");
    let store = scratch.join("store");
    std::fs::create_dir_all(repo.join("src"))?;
    std::fs::write(repo.join("src/main.rs"), FIXTURE_MAIN)?;
    git(&repo, &["init", "-q", "-b", "main"])?;
    git(&repo, &["config", "user.name", "notes-cli dev"])?;
    git(&repo, &["config", "user.email", "dev@local"])?;
    git(&repo, &["add", "-A"])?;
    git(&repo, &["commit", "-q", "-m", "initial"])?;

    // The brief, pointed at the scratch repo.
    let mut brief: serde_json::Value = serde_json::from_str(FIXTURE_BRIEF)?;
    brief["scope"]["repo"] = serde_json::json!(repo.to_string_lossy());
    let mission_id = brief["mission_id"].as_str().unwrap().to_owned();

    record_cassettes(&store, &brief)?;

    let mut core = Core::open(&store)?;
    core.set_worker_cmd(worker_cmd);
    // The scratch repo is this demo's fixtures root (DEV-tier: fixture repos
    // only). No OS containment exists — disclosed up front.
    core.set_fixtures_root(&scratch)?;

    println!("── WePLD demo ──  (Operating System for Autonomous Engineering)");
    println!("scratch: {}", scratch.display());
    println!("{}\n", wepld_runtime::DEV_TIER_WARNING);

    let create = WCommand {
        command_id: command_id_for("create_mission", &brief),
        command_type: "create_mission".to_owned(),
        actor: "principal_local".to_owned(),
        payload: brief.clone(),
    };
    step("create mission", &core.submit(&create)?);
    step("plan", &core.plan_mission(&mission_id)?);
    step(
        "approve plan",
        &core.approve_plan(&mission_id, "principal_local")?,
    );
    step("run (build + gates)", &core.run_mission(&mission_id)?);
    step(
        "accept (proposal)",
        &core.accept_mission(&mission_id, "principal_local")?,
    );

    println!("\n── timeline ──");
    for e in core.timeline(&mission_id)? {
        println!("{:>4}  {:<26} {}", e.seq, e.entry_type.code(), e.actor_id);
    }

    let report = core.verify()?;
    println!(
        "\nchain {} — {} entries",
        if report.is_valid() {
            "VERIFIED"
        } else {
            "BROKEN"
        },
        report.total
    );
    // V0 acceptance never mutates the primary worktree or base branch: it
    // creates a reviewable proposal ref for an external protected merge.
    let primary = std::fs::read_to_string(repo.join("src/main.rs"))?;
    println!(
        "primary worktree unchanged (no --version in the working tree): {}",
        !primary.contains("--version")
    );
    println!("proposal ref: refs/heads/wepld/mission-{mission_id} — review and merge externally");
    println!("\nThis is WePLD: a mission became an evidence-backed, replayable merge proposal.");
    Ok(())
}

fn record_cassettes(store: &Path, brief: &serde_json::Value) -> Result<(), Box<dyn Error>> {
    let cassette = store.join("cassettes/demo.jsonl");
    let plan: serde_json::Value = serde_json::from_str(PLAN)?;
    let plan_key = cassette_key(
        "plan",
        &wepld_artifacts::hash_hex(&serde_json::to_vec(&planner_pack(brief))?),
        "plan.v1",
        "fixture-model",
    );
    write_cassette_entry(&cassette, &plan_key, &plan, "fixture-model")?;

    let task_spec = plan["tasks"][0].clone();
    let build_key = cassette_key(
        "build",
        &wepld_artifacts::hash_hex(&serde_json::to_vec(&builder_pack(brief, &task_spec))?),
        "builder_step.v1",
        "fixture-model",
    );
    let edits =
        serde_json::json!({ "edits": [ { "path": "src/main.rs", "content": EDITED_MAIN } ] });
    write_cassette_entry(&cassette, &build_key, &edits, "fixture-model")?;
    Ok(())
}

fn step(label: &str, outcome: &CommandOutcome) {
    match outcome {
        CommandOutcome::Accepted { detail } => {
            let state = detail["state"].as_str().unwrap_or("");
            println!("✓ {label:<22} → {state}");
        }
        CommandOutcome::Rejected { reason } => println!("✗ {label:<22} REJECTED: {reason}"),
        other => println!("· {label:<22} {other:?}"),
    }
}

fn git(dir: &Path, args: &[&str]) -> Result<(), Box<dyn Error>> {
    let out = Command::new("git").args(args).current_dir(dir).output()?;
    if !out.status.success() {
        return Err(format!("git {args:?}: {}", String::from_utf8_lossy(&out.stderr)).into());
    }
    Ok(())
}
