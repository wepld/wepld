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
    spec_for("Add a --version flag to notes-cli", "VERSION")
}

/// A minimal spec whose single gate greps for `token` in `src/main.rs`.
fn spec_for(overview: &str, token: &str) -> SpecificationDocument {
    let mut d = SpecificationDocument {
        overview: overview.to_owned(),
        functional_requirements: vec![format!("Ensure {token} is present")],
        acceptance_criteria: vec![SpecAcceptanceCriterion {
            id: "AC1".to_owned(),
            text: format!("{token} present"),
            verify: "gate:build".to_owned(),
        }],
        ..Default::default()
    };
    d.verification
        .insert("build".to_owned(), format!("grep -q {token} src/main.rs"));
    d
}

fn write_cassettes(store: &Path, repo: &str) {
    let doc = reasoned_spec();
    let specify_pack = wepld_runtime::specify_pack(REQUEST, vec![]);
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

/// Write specify+build cassettes for one feature run, parameterized so tests
/// can drive missions that fail a gate or belong to a second repository.
fn write_feature_cassettes(
    store: &Path,
    request: &str,
    slug: &str,
    repo: &str,
    doc: &SpecificationDocument,
    prior_memory: &[serde_json::Value],
    edited_main: &str,
) {
    let cassette = store.join("cassettes/r.jsonl");
    let specify_pack = wepld_runtime::specify_pack(request, prior_memory.to_vec());
    let specify_key = wepld_providers::cassette_key(
        "specify",
        &wepld_artifacts::hash_hex(&serde_json::to_vec(&specify_pack).unwrap()),
        "specification.v1",
        "fixture-model",
    );
    wepld_providers::write_cassette_entry(
        &cassette,
        &specify_key,
        &serde_json::to_value(doc).unwrap(),
        "fixture-model",
    )
    .unwrap();

    let conv = convert(ConvertInput {
        doc,
        spec_id: &format!("spec_{slug}"),
        version: 1,
        document_hash: "x",
        slug,
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
        &serde_json::json!({ "edits": [ { "path": "src/main.rs", "content": edited_main } ] }),
        "fixture-model",
    )
    .unwrap();
}

/// Test driver: three explicit, distinct governed calls (start → approve plan
/// & execute → decide completion). The test is the caller making separate
/// decisions — there is no public one-shot recipe method (Blocker 1).
fn drive(
    core: &mut Core,
    request: &str,
    slug: &str,
    repo: &str,
    base: &str,
    principal: &str,
) -> Result<RecipeOutcome, wepld_runtime::RuntimeError> {
    match core.start_build_feature(request, slug, repo, base, principal)? {
        RecipeOutcome::NeedsPlanApproval { mission_id, .. } => {
            match core.approve_plan_and_execute(&mission_id, principal)? {
                RecipeOutcome::NeedsCompletionApproval { mission_id, .. } => {
                    core.decide_completion(&mission_id, principal, true)
                }
                other => Ok(other),
            }
        }
        other => Ok(other),
    }
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
    core.set_fixtures_root(workdir.path()).unwrap();

    let outcome = drive(
        &mut core,
        REQUEST,
        SLUG,
        &repo_str,
        "main",
        "principal_local",
    )
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
    // Provenance: the lesson points back to the ledger fact that created it.
    assert!(
        lessons[0].created_seq > 0,
        "lesson carries its creation seq"
    );
    assert_eq!(lessons[0].mission_id, report.mission_id);

    // Idempotent acceptance: re-recording the same mission's experience records
    // nothing new — no second lesson, no duplicate InsightRecorded event.
    let again = core
        .record_engineering_experience(&report.mission_id)
        .unwrap();
    assert!(
        again.is_none(),
        "a replayed acceptance yields no new lesson"
    );
    assert_eq!(core.lessons_for_repo(&repo_str).unwrap().len(), 1);
    let insights = core
        .all_entries()
        .unwrap()
        .iter()
        .filter(|e| e.entry_type.code() == "InsightRecorded")
        .count();
    assert_eq!(insights, 1, "no duplicate InsightRecorded on replay");

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

    // Blocker 2: acceptance never mutates the primary worktree or base branch —
    // the feature lands on a proposal ref, not on main.
    let primary = std::fs::read_to_string(repo.join("src/main.rs")).unwrap();
    assert_eq!(
        primary, "fn main() {}\n",
        "primary worktree is never mutated"
    );
    let base = Command::new("git")
        .args(["rev-parse", "main"])
        .current_dir(&repo)
        .output()
        .unwrap();
    let base_head = String::from_utf8_lossy(&base.stdout).trim().to_owned();
    // The proposal ref exists and carries the change; base HEAD != proposal.
    let show = Command::new("git")
        .args([
            "show",
            "refs/heads/wepld/mission-mis_version-flag_v1:src/main.rs",
        ])
        .current_dir(&repo)
        .output()
        .unwrap();
    assert!(show.status.success(), "proposal ref exists");
    assert!(
        String::from_utf8_lossy(&show.stdout).contains("VERSION"),
        "the proposal ref carries the feature"
    );
    let proposal = Command::new("git")
        .args(["rev-parse", "refs/heads/wepld/mission-mis_version-flag_v1"])
        .current_dir(&repo)
        .output()
        .unwrap();
    assert_ne!(
        base_head,
        String::from_utf8_lossy(&proposal.stdout).trim(),
        "base branch was not advanced to the proposal"
    );
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
    let specify_pack = wepld_runtime::specify_pack(REQUEST, vec![]);
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
    let outcome = drive(
        &mut core,
        REQUEST,
        SLUG,
        &repo_str,
        "main",
        "principal_local",
    )
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
    core.set_fixtures_root(workdir.path()).unwrap();

    // Feature 1 records a lesson.
    let one = drive(
        &mut core,
        REQUEST,
        SLUG,
        &repo_str,
        "main",
        "principal_local",
    )
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

    // Build the pack exactly as production does — through the same bounded,
    // structured selection + pack helpers — so the cassette key matches.
    let memory = wepld_runtime::specify_memory_entries(&lessons);
    let specify_pack2 = wepld_runtime::specify_pack(request2, memory);
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
    core.set_fixtures_root(workdir.path()).unwrap();

    // Restart durability: a brand-new process loads feature 1's lesson from the
    // persistent store — proof beyond a single continuously-alive process.
    let reloaded = core.lessons_for_repo(&repo_str).unwrap();
    assert_eq!(reloaded.len(), 1, "the lesson survived a full store reopen");
    assert_eq!(reloaded[0].lesson_id, "lesson_mis_version-flag_v1");

    let two = drive(
        &mut core,
        request2,
        slug2,
        &repo_str,
        "main",
        "principal_local",
    )
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

/// A mission that fails its gate is never accepted and leaves no Active lesson:
/// memory records verified success, not attempts.
#[test]
fn a_non_accepted_mission_records_no_lesson() {
    let workdir = tempfile::tempdir().unwrap();
    let store = tempfile::tempdir().unwrap();
    let repo = fixture_repo(workdir.path());
    let repo_str = repo.to_string_lossy().into_owned();

    // The spec requires VERSION; the build writes a file WITHOUT it, so the gate
    // fails, completion is never proposed, and the mission is never accepted.
    write_feature_cassettes(
        store.path(),
        REQUEST,
        "noversion",
        &repo_str,
        &reasoned_spec(),
        &[],
        "fn main() { println!(\"no version here\"); }\n",
    );

    let mut core = Core::open(store.path()).unwrap();
    core.set_worker_cmd(vec![hermes_bin()]);
    core.set_fixtures_root(workdir.path()).unwrap();
    let outcome = drive(
        &mut core,
        REQUEST,
        "noversion",
        &repo_str,
        "main",
        "principal_local",
    )
    .unwrap();

    if let RecipeOutcome::Completed(bf) = &outcome {
        assert_ne!(bf.report.state, "accepted", "gate failed → not accepted");
        assert_eq!(bf.lessons_learned, 0);
        assert_eq!(bf.total_memory, 0);
    }
    assert!(
        core.lessons_for_repo(&repo_str).unwrap().is_empty(),
        "no lesson from an unaccepted mission"
    );
    // A direct extraction attempt also refuses a non-accepted mission.
    assert!(core
        .record_engineering_experience("mis_noversion_v1")
        .unwrap()
        .is_none());
}

/// A lesson learned in one repository is never supplied to another: Engineering
/// Memory is scoped to its project, and cross-repo counters never mislead.
#[test]
fn lessons_do_not_leak_across_repositories() {
    let work_a = tempfile::tempdir().unwrap();
    let work_b = tempfile::tempdir().unwrap();
    let store = tempfile::tempdir().unwrap();
    let repo_a = fixture_repo(work_a.path());
    let repo_b = fixture_repo(work_b.path());
    let a = repo_a.to_string_lossy().into_owned();
    let b = repo_b.to_string_lossy().into_owned();

    // Both features' cassettes are written before open (the adapter loads once).
    // Each is a first run on its own repo, so each carries empty prior memory.
    write_feature_cassettes(
        store.path(),
        REQUEST,
        "version-flag",
        &a,
        &reasoned_spec(),
        &[],
        EDITED_MAIN,
    );
    let doc_b = spec_for("Add a --quiet flag to notes-cli", "QUIET");
    write_feature_cassettes(
        store.path(),
        "Add a --quiet flag to notes-cli",
        "quiet-flag",
        &b,
        &doc_b,
        &[],
        "fn main() { const QUIET: &str = \"q\"; let _ = QUIET; }\n",
    );

    let mut core = Core::open(store.path()).unwrap();
    core.set_worker_cmd(vec![hermes_bin()]);

    // Feature on repo A records a lesson scoped to A.
    core.set_fixtures_root(work_a.path()).unwrap();
    let ra = drive(
        &mut core,
        REQUEST,
        "version-flag",
        &a,
        "main",
        "principal_local",
    )
    .unwrap();
    assert!(matches!(ra, RecipeOutcome::Completed(_)));
    assert_eq!(core.lessons_for_repo(&a).unwrap().len(), 1);
    assert!(
        core.lessons_for_repo(&b).unwrap().is_empty(),
        "A's lesson must not appear under B"
    );
    assert!(wepld_runtime::specify_memory_entries(&core.lessons_for_repo(&b).unwrap()).is_empty());

    // Feature on repo B sees no memory from A. (If A's lesson had leaked into
    // B's pack, the cassette key would differ and this run would fail outright.)
    core.set_fixtures_root(work_b.path()).unwrap();
    match drive(
        &mut core,
        "Add a --quiet flag to notes-cli",
        "quiet-flag",
        &b,
        "main",
        "principal_local",
    )
    .unwrap()
    {
        RecipeOutcome::Completed(bf) => {
            assert_eq!(bf.prior_lessons_applied, 0, "no cross-repo memory applied");
            assert_eq!(bf.lessons_learned, 1);
            assert_eq!(bf.total_memory, 1, "B's memory counts only B's own lessons");
        }
        other => panic!(
            "expected completion, got {:?}",
            std::mem::discriminant(&other)
        ),
    }
    assert_eq!(core.lessons_for_repo(&a).unwrap().len(), 1, "A unchanged");
    assert_eq!(core.lessons_for_repo(&b).unwrap().len(), 1);
}
