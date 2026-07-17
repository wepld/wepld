//! Spec-A/B golden trace: the Specification → Mission → Runtime → Hermes →
//! Evidence → Ledger vertical slice must produce exactly the recorded
//! entry-type sequence (IADR-0004 discipline, applied to the spec flow).
//! Deterministic: same canonical document → same mission → same trace.

use std::path::Path;
use std::process::Command;
use wepld_contracts::command::CommandOutcome;
use wepld_runtime::{builder_pack, Core};
use wepld_specification::{convert, ConvertInput, SpecAcceptanceCriterion, SpecificationDocument};

const EXPECTED_TRACE: &str = include_str!("../../../fixtures/golden/spec-to-mission.trace");
const EDITED_MAIN: &str = "fn main() {\n    const VERSION: &str = \"0.1.0\";\n    if std::env::args().any(|a| a == \"--version\") { println!(\"{VERSION}\"); }\n}\n";
const SLUG: &str = "version-flag";

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

fn spec_doc() -> SpecificationDocument {
    let mut d = SpecificationDocument {
        overview: "Add a --version flag".to_owned(),
        functional_requirements: vec!["Print the version".to_owned()],
        acceptance_criteria: vec![SpecAcceptanceCriterion {
            id: "AC1".to_owned(),
            text: "VERSION present".to_owned(),
            verify: "gate:build".to_owned(),
        }],
        ..Default::default()
    };
    d.verification
        .insert("build".to_owned(), "grep -q VERSION src/main.rs".to_owned());
    d
}

#[test]
fn spec_to_mission_matches_golden_trace() {
    let workdir = tempfile::tempdir().unwrap();
    let store = tempfile::tempdir().unwrap();
    let repo = fixture_repo(workdir.path());
    let repo_str = repo.to_string_lossy().into_owned();
    let doc = spec_doc();

    // Record the builder cassette to match the conversion's brief + task.
    // (brief/plan are independent of the document hash, so a dummy hash matches
    // what the Runtime computes internally.)
    let conv = convert(ConvertInput {
        doc: &doc,
        spec_id: "spec_version-flag",
        version: 1,
        document_hash: "unused-for-pack",
        slug: SLUG,
        repo: &repo_str,
        base_branch: "main",
        paths: vec!["src/**".to_owned()],
    })
    .unwrap();
    let brief_json = serde_json::to_value(&conv.brief).unwrap();
    let task_spec = serde_json::to_value(&conv.plan.tasks[0]).unwrap();
    let pack = builder_pack(&brief_json, &task_spec);
    let key = wepld_providers::cassette_key(
        "build",
        &wepld_artifacts::hash_hex(&serde_json::to_vec(&pack).unwrap()),
        "builder_step.v1",
        "fixture-model",
    );
    wepld_providers::write_cassette_entry(
        &store.path().join("cassettes/g.jsonl"),
        &key,
        &serde_json::json!({ "edits": [ { "path": "src/main.rs", "content": EDITED_MAIN } ] }),
        "fixture-model",
    )
    .unwrap();

    let mut core = Core::open(store.path()).unwrap();
    core.set_worker_cmd(vec![hermes_bin()]);
    core.set_fixtures_root(std::path::Path::new(&repo_str))
        .unwrap();

    let mission_id = format!("mis_{SLUG}_v1");
    assert!(ok(core
        .create_mission_from_spec(&doc, SLUG, &repo_str, "main", "principal_local")
        .unwrap()));
    assert!(ok(core
        .approve_plan(&mission_id, "principal_local")
        .unwrap()));
    assert!(ok(core.run_mission(&mission_id).unwrap()));
    assert!(ok(core
        .accept_mission(&mission_id, "principal_local")
        .unwrap()));

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
        "spec→mission trace diverged from the golden file"
    );

    assert_eq!(
        core.mission_row(&mission_id).unwrap().unwrap().1,
        "accepted"
    );
    assert!(core.verify().unwrap().is_valid());
    // V0: the spec-derived edit lands on a proposal ref, never on the primary
    // worktree, which stays clean.
    let main = std::fs::read_to_string(repo.join("src/main.rs")).unwrap();
    assert!(!main.contains("VERSION"), "primary worktree is untouched");
    let show = std::process::Command::new("git")
        .args([
            "show",
            &format!("refs/heads/wepld/mission-{mission_id}:src/main.rs"),
        ])
        .current_dir(&repo)
        .output()
        .unwrap();
    assert!(
        String::from_utf8_lossy(&show.stdout).contains("VERSION"),
        "the proposal ref carries the spec-derived edit"
    );
}

fn ok(o: CommandOutcome) -> bool {
    matches!(o, CommandOutcome::Accepted { .. })
}
