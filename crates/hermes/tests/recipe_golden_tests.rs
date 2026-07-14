//! Build Feature recipe golden: the one-command flow (request → Hermes reasons
//! a spec → mission → execution → evidence → accept → report) must produce
//! exactly the recorded trace and a full-confidence, evidence-derived report.
//! Deterministic under cassettes.

use std::path::Path;
use std::process::Command;
use wepld_runtime::{builder_pack, Core, RecipeOutcome};
use wepld_specification::{convert, ConvertInput, SpecAcceptanceCriterion, SpecificationDocument};

const EXPECTED_TRACE: &str = include_str!("../../../fixtures/golden/build-feature.trace");
const REQUEST: &str = "Add a --version flag to notes-cli";
const SLUG: &str = "version-flag";
const EDITED_MAIN: &str = "fn main() {\n    const VERSION: &str = \"0.1.0\";\n    if std::env::args().any(|a| a == \"--version\") { println!(\"{VERSION}\"); }\n}\n";

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

fn reasoned_spec() -> SpecificationDocument {
    let mut d = SpecificationDocument {
        overview: "Add a --version flag to notes-cli".to_owned(),
        functional_requirements: vec!["Print the version on --version".to_owned()],
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

fn write_cassettes(store: &Path, repo: &str) {
    let doc = reasoned_spec();
    let specify_pack = serde_json::json!({
        "schema_version": 1, "intent": "specify", "request": REQUEST, "engineering_memory": []
    });
    let specify_key = wepld_providers::cassette_key(
        "specify",
        &wepld_artifacts::hash_hex(&serde_json::to_vec(&specify_pack).unwrap()),
        "specification.v1",
        "fixture-model",
    );
    let cassette = store.join("cassettes/r.jsonl");
    wepld_providers::write_cassette_entry(
        &cassette,
        &specify_key,
        &serde_json::to_value(&doc).unwrap(),
        "fixture-model",
    )
    .unwrap();

    let conv = convert(ConvertInput {
        doc: &doc,
        spec_id: "spec_version-flag",
        version: 1,
        document_hash: "x",
        slug: SLUG,
        repo,
        base_branch: "main",
        paths: vec!["src/**".to_owned()],
    })
    .unwrap();
    let pack = builder_pack(
        &serde_json::to_value(&conv.brief).unwrap(),
        &serde_json::to_value(&conv.plan.tasks[0]).unwrap(),
    );
    let build_key = wepld_providers::cassette_key(
        "build",
        &wepld_artifacts::hash_hex(&serde_json::to_vec(&pack).unwrap()),
        "builder_step.v1",
        "fixture-model",
    );
    wepld_providers::write_cassette_entry(
        &cassette,
        &build_key,
        &serde_json::json!({ "edits": [ { "path": "src/main.rs", "content": EDITED_MAIN } ] }),
        "fixture-model",
    )
    .unwrap();
}

#[test]
fn build_feature_recipe_matches_golden_and_reports_full_confidence() {
    let workdir = tempfile::tempdir().unwrap();
    let store = tempfile::tempdir().unwrap();
    let repo = fixture_repo(workdir.path());
    let repo_str = repo.to_string_lossy().into_owned();
    write_cassettes(store.path(), &repo_str);

    let mut core = Core::open(store.path()).unwrap();
    core.set_worker_cmd(vec![hermes_bin()]);

    let outcome = core
        .run_build_feature(REQUEST, SLUG, &repo_str, "main")
        .unwrap();
    let bf = match outcome {
        RecipeOutcome::Completed(r) => *r,
        other => panic!(
            "expected completion, got {:?}",
            std::mem::discriminant(&other)
        ),
    };
    let report = &bf.report;

    // Evidence-derived report.
    assert_eq!(report.state, "accepted");
    assert_eq!(report.gates_passed(), 1);
    assert_eq!(report.criteria_met(), 1);
    assert!(report.chain_verified);
    assert_eq!(report.uncertain_attempts, 0);
    // Mission-scoped: the build-phase reasoning. Spec generation is a separate
    // spec-scoped fact (still on the ledger, seen by the golden trace below).
    assert_eq!(report.brain_calls, 1);
    assert_eq!(
        report.confidence, 1.0,
        "all evidence green → full confidence"
    );
    assert_eq!(report.spec_id.as_deref(), Some("spec_version-flag"));

    // Engineering Memory: this mission left a lesson; none applied (first run).
    assert_eq!(bf.lessons_learned, 1);
    assert_eq!(bf.prior_lessons_applied, 0);
    assert_eq!(bf.total_memory, 1);
    let lessons = core.lessons_for_repo(&repo_str).unwrap();
    assert_eq!(lessons.len(), 1);
    assert!(
        lessons[0].body.contains("src/main.rs"),
        "lesson cites the file it touched"
    );
    assert!(
        lessons[0].body.contains("build"),
        "lesson cites the verifying gate"
    );

    // Golden trace.
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
        "recipe trace diverged from the golden file"
    );

    // The feature landed on main.
    let main = std::fs::read_to_string(repo.join("src/main.rs")).unwrap();
    assert!(main.contains("VERSION"));
    assert!(core.verify().unwrap().is_valid());
}

#[test]
fn recipe_asks_for_clarification_when_the_spec_has_open_questions() {
    // The specify cassette returns a spec with an unresolved question.
    let workdir = tempfile::tempdir().unwrap();
    let store = tempfile::tempdir().unwrap();
    let repo = fixture_repo(workdir.path());
    let repo_str = repo.to_string_lossy().into_owned();

    let mut doc = reasoned_spec();
    doc.open_questions
        .push("Which version string source?".to_owned());
    let specify_pack = serde_json::json!({
        "schema_version": 1, "intent": "specify", "request": REQUEST, "engineering_memory": []
    });
    let key = wepld_providers::cassette_key(
        "specify",
        &wepld_artifacts::hash_hex(&serde_json::to_vec(&specify_pack).unwrap()),
        "specification.v1",
        "fixture-model",
    );
    wepld_providers::write_cassette_entry(
        &store.path().join("cassettes/r.jsonl"),
        &key,
        &serde_json::to_value(&doc).unwrap(),
        "fixture-model",
    )
    .unwrap();

    let mut core = Core::open(store.path()).unwrap();
    core.set_worker_cmd(vec![hermes_bin()]);
    let outcome = core
        .run_build_feature(REQUEST, SLUG, &repo_str, "main")
        .unwrap();
    assert!(matches!(outcome, RecipeOutcome::NeedsClarification { .. }));
    // No mission was created — the recipe stopped to ask.
    assert!(core.mission_row("mis_version-flag_v1").unwrap().is_none());
}

/// The Engineering Memory loop closes: a lesson learned by one mission is
/// applied by the next mission on the same repo. This is "Hermes remembers".
#[test]
fn a_second_feature_applies_the_first_lesson() {
    let workdir = tempfile::tempdir().unwrap();
    let store = tempfile::tempdir().unwrap();
    let repo = fixture_repo(workdir.path());
    let repo_str = repo.to_string_lossy().into_owned();
    write_cassettes(store.path(), &repo_str);

    let mut core = Core::open(store.path()).unwrap();
    core.set_worker_cmd(vec![hermes_bin()]);

    // Feature 1 records a lesson.
    let one = core
        .run_build_feature(REQUEST, SLUG, &repo_str, "main")
        .unwrap();
    assert!(matches!(one, RecipeOutcome::Completed(_)));
    let lessons = core.lessons_for_repo(&repo_str).unwrap();
    assert_eq!(lessons.len(), 1);

    // Feature 2 on the same repo: its specify pack now carries the prior lesson
    // (the actual recorded one), so its cassette key includes that memory.
    let request2 = "Add a --help flag to notes-cli";
    let slug2 = "help-flag";
    let mut doc2 = reasoned_spec();
    doc2.overview = "Add a --help flag".to_owned();
    doc2.acceptance_criteria[0].text = "HELP present".to_owned();
    doc2.verification.clear();
    doc2.verification
        .insert("build".to_owned(), "grep -q HELP src/main.rs".to_owned());

    let memory: Vec<serde_json::Value> = lessons
        .iter()
        .map(|l| serde_json::json!({ "title": l.title, "body": l.body }))
        .collect();
    let specify_pack2 = serde_json::json!({
        "schema_version": 1, "intent": "specify", "request": request2, "engineering_memory": memory
    });
    let specify_key2 = wepld_providers::cassette_key(
        "specify",
        &wepld_artifacts::hash_hex(&serde_json::to_vec(&specify_pack2).unwrap()),
        "specification.v1",
        "fixture-model",
    );
    let cassette = store.path().join("cassettes/r.jsonl");
    wepld_providers::write_cassette_entry(
        &cassette,
        &specify_key2,
        &serde_json::to_value(&doc2).unwrap(),
        "fixture-model",
    )
    .unwrap();

    // Build cassette for feature 2.
    let conv2 = convert(ConvertInput {
        doc: &doc2,
        spec_id: "spec_help-flag",
        version: 1,
        document_hash: "x",
        slug: slug2,
        repo: &repo_str,
        base_branch: "main",
        paths: vec!["src/**".to_owned()],
    })
    .unwrap();
    let pack2 = builder_pack(
        &serde_json::to_value(&conv2.brief).unwrap(),
        &serde_json::to_value(&conv2.plan.tasks[0]).unwrap(),
    );
    let build_key2 = wepld_providers::cassette_key(
        "build",
        &wepld_artifacts::hash_hex(&serde_json::to_vec(&pack2).unwrap()),
        "builder_step.v1",
        "fixture-model",
    );
    wepld_providers::write_cassette_entry(
        &cassette,
        &build_key2,
        &serde_json::json!({ "edits": [ { "path": "src/main.rs", "content": "fn main() { const HELP: &str = \"usage\"; const VERSION: &str = \"0.1.0\"; let _ = (HELP, VERSION); }\n" } ] }),
        "fixture-model",
    )
    .unwrap();

    // Reopen so the fixture adapter reloads the new cassettes (the ledger,
    // incl. lesson 1, persists across the reopen).
    drop(core);
    let mut core = Core::open(store.path()).unwrap();
    core.set_worker_cmd(vec![hermes_bin()]);

    let two = core
        .run_build_feature(request2, slug2, &repo_str, "main")
        .unwrap();
    match two {
        RecipeOutcome::Completed(bf) => {
            assert_eq!(
                bf.prior_lessons_applied, 1,
                "the first lesson informed the second feature"
            );
            assert_eq!(bf.lessons_learned, 1);
            assert_eq!(bf.total_memory, 2, "memory grew across missions");
        }
        other => panic!(
            "expected completion, got {:?}",
            std::mem::discriminant(&other)
        ),
    }
}
