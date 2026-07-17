//! Final integrity & resource-safety tests (final remediation, Blockers 1–4).
//! Proves: DEV override activation is ledger-atomic and reconstructed from
//! durable state at `Core::open`; an explicit completion return is its own
//! terminal outcome (never `Completed`, no proposal ref, no lesson); and
//! model-produced plans are bounded (titles, satisfies, coverage, serialized
//! size) before any persistence.

use std::path::Path;
use std::process::Command;
use wepld_contracts::command::{Command as WCommand, CommandOutcome};
use wepld_contracts::validation::{MAX_SATISFIES_PER_TASK, MAX_TASK_TITLE_BYTES};
use wepld_contracts::vocabulary::EventType;
use wepld_runtime::{builder_pack, command_id_for, planner_pack, Core, RecipeOutcome};
use wepld_specification::{convert, ConvertInput, SpecAcceptanceCriterion, SpecificationDocument};

const REQUEST: &str = "Add a --version flag to notes-cli";
const SLUG: &str = "version-flag";
const MID: &str = "mis_version-flag_v1";
const EDITED: &str = "fn main() {\n    const VERSION: &str = \"0.1.0\";\n}\n";

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

fn spec() -> SpecificationDocument {
    let mut d = SpecificationDocument {
        overview: REQUEST.to_owned(),
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

fn write_cassettes(store: &Path, repo: &str) {
    write_cassettes_with_edits(
        store,
        repo,
        serde_json::json!({ "edits": [ { "path": "src/main.rs", "content": EDITED } ] }),
    );
}

fn write_cassettes_with_edits(store: &Path, repo: &str, build_output: serde_json::Value) {
    let doc = spec();
    let cassette = store.join("cassettes/r.jsonl");
    let specify_key = wepld_providers::cassette_key(
        "specify",
        &wepld_artifacts::hash_hex(
            &serde_json::to_vec(&wepld_runtime::specify_pack(REQUEST, vec![])).unwrap(),
        ),
        "specification.v1",
        "fixture-model",
    );
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
    wepld_providers::write_cassette_entry(&cassette, &build_key, &build_output, "fixture-model")
        .unwrap();
}

/// Drive a mission to `completion_proposed` via the staged recipe API.
fn to_completion_proposed(store: &Path, repo: &str) -> Core {
    let mut core = Core::open(store).unwrap();
    core.set_worker_cmd(vec![hermes_bin()]);
    core.set_fixtures_root(Path::new(repo)).unwrap();
    assert!(matches!(
        core.start_build_feature(REQUEST, SLUG, repo, "main", "principal_local")
            .unwrap(),
        RecipeOutcome::NeedsPlanApproval { .. }
    ));
    assert!(matches!(
        core.approve_plan_and_execute(MID, "principal_local")
            .unwrap(),
        RecipeOutcome::NeedsCompletionApproval { .. }
    ));
    core
}

fn override_facts(core: &Core) -> Vec<(String, String)> {
    core.all_entries()
        .unwrap()
        .into_iter()
        .filter(|e| {
            e.entry_type == EventType::SandboxTierDetected
                && e.payload_json["override"] == "allow_uncontained_repo"
        })
        .map(|e| {
            (
                e.actor_id.clone(),
                e.payload_json["repo"].as_str().unwrap_or("").to_owned(),
            )
        })
        .collect()
}

fn proposal_ref_exists(repo: &Path) -> bool {
    Command::new("git")
        .args([
            "rev-parse",
            "--verify",
            "--quiet",
            &format!("refs/heads/wepld/mission-{MID}"),
        ])
        .current_dir(repo)
        .output()
        .unwrap()
        .status
        .success()
}

// ── Blocker 1: DEV override activation is ledger-atomic ────────────────────

#[test]
fn override_ledger_failure_leaves_the_repo_denied_with_no_partial_fact() {
    let workdir = tempfile::tempdir().unwrap();
    let store = tempfile::tempdir().unwrap();
    let repo = fixture_repo(workdir.path());
    let repo_str = repo.to_string_lossy().into_owned();
    let mut core = Core::open(store.path()).unwrap();

    // Forced storage failure during the durable write → error out.
    assert!(
        core.allow_uncontained_repo_at(&repo_str, "principal_local", true)
            .is_err(),
        "a failed ledger transaction must surface as an error"
    );
    // The live authorization state is unchanged: the repo stays denied.
    assert!(
        !core.is_repo_allowed_under_dev(&repo_str),
        "a failed activation must leave the repository denied"
    );
    // No partial override fact was committed.
    assert!(
        override_facts(&core).is_empty(),
        "no partial override fact may exist after a rolled-back transaction"
    );
    // The ledger chain stays valid (the rollback left no torn state).
    assert!(core.verify().unwrap().is_valid());

    // Retry after "storage recovery" → exactly one effective authorization fact.
    core.allow_uncontained_repo(&repo_str, "principal_local")
        .unwrap();
    assert!(core.is_repo_allowed_under_dev(&repo_str));
    assert_eq!(
        override_facts(&core).len(),
        1,
        "the retry creates exactly one authorization fact"
    );
}

#[test]
fn override_ledger_failure_does_not_replace_a_prior_valid_override() {
    let workdir = tempfile::tempdir().unwrap();
    let store = tempfile::tempdir().unwrap();
    let repo_a = fixture_repo(workdir.path());
    let repo_b = fixture_repo(&workdir.path().join("other"));
    let a = repo_a.to_string_lossy().into_owned();
    let b = repo_b.to_string_lossy().into_owned();
    let mut core = Core::open(store.path()).unwrap();

    core.allow_uncontained_repo(&a, "principal_local").unwrap();
    assert!(core.is_repo_allowed_under_dev(&a));

    // A failed grant for B must not disturb A's standing authorization.
    assert!(core
        .allow_uncontained_repo_at(&b, "principal_local", true)
        .is_err());
    assert!(
        core.is_repo_allowed_under_dev(&a),
        "the prior valid override must remain in force"
    );
    assert!(
        !core.is_repo_allowed_under_dev(&b),
        "the failed grant must not authorize the new repository"
    );
    assert_eq!(override_facts(&core).len(), 1, "only A's fact exists");
}

#[test]
fn override_is_reconstructed_from_the_ledger_at_open() {
    let workdir = tempfile::tempdir().unwrap();
    let store = tempfile::tempdir().unwrap();
    let repo = fixture_repo(workdir.path());
    let repo_str = repo.to_string_lossy().into_owned();

    {
        let mut core = Core::open(store.path()).unwrap();
        core.allow_uncontained_repo(&repo_str, "principal_local")
            .unwrap();
        assert!(core.is_repo_allowed_under_dev(&repo_str));
    } // "restart": the process state is dropped; only the ledger survives.

    let core = Core::open(store.path()).unwrap();
    assert!(
        core.is_repo_allowed_under_dev(&repo_str),
        "durable authorization: the recorded override survives a restart"
    );
    // Reconstruction is read-only — it appends no new fact.
    assert_eq!(override_facts(&core).len(), 1);
    // Exact canonical matching still applies after restore: a sibling repo
    // is NOT authorized by the restored override.
    let other = fixture_repo(&workdir.path().join("sibling"));
    assert!(!core.is_repo_allowed_under_dev(&other.to_string_lossy()));
}

// ── Blocker 2: a return decision is preserved, never Completed ─────────────

#[test]
fn an_explicit_return_is_a_terminal_returned_outcome_with_no_ref_or_lesson() {
    let workdir = tempfile::tempdir().unwrap();
    let store = tempfile::tempdir().unwrap();
    let repo = fixture_repo(workdir.path());
    let repo_str = repo.to_string_lossy().into_owned();
    write_cassettes(store.path(), &repo_str);
    let mut core = to_completion_proposed(store.path(), &repo_str);

    match core
        .decide_completion(MID, "principal_local", false)
        .unwrap()
    {
        RecipeOutcome::Returned {
            mission_id,
            state,
            returned_by,
            reason,
        } => {
            assert_eq!(mission_id, MID);
            assert_eq!(state, "returned");
            assert_eq!(returned_by, "principal_local");
            assert!(!reason.is_empty());
        }
        other => panic!(
            "a return decision must surface as Returned, got {:?}",
            std::mem::discriminant(&other)
        ),
    }

    // The durable record: MissionReturned by the real reviewer; state returned.
    let entries = core.timeline(MID).unwrap();
    let ret = entries
        .iter()
        .find(|e| e.entry_type == EventType::MissionReturned)
        .expect("MissionReturned recorded");
    assert_eq!(ret.actor_id, "principal_local");
    assert_eq!(core.mission_row(MID).unwrap().unwrap().1, "returned");

    // Returned ≠ accepted: no MissionAccepted, no proposal ref, no lesson.
    assert!(!entries
        .iter()
        .any(|e| e.entry_type == EventType::MissionAccepted));
    assert!(
        !proposal_ref_exists(&repo),
        "a return decision must not create a proposal ref"
    );
    assert!(!core
        .all_entries()
        .unwrap()
        .iter()
        .any(|e| e.entry_type == EventType::InsightRecorded));
    assert!(
        core.record_engineering_experience(MID).unwrap().is_none(),
        "an unaccepted (returned) mission leaves no Engineering Memory lesson"
    );
    assert!(core.verify().unwrap().is_valid());
}

#[test]
fn an_unauthenticated_return_is_rejected_and_changes_nothing() {
    let workdir = tempfile::tempdir().unwrap();
    let store = tempfile::tempdir().unwrap();
    let repo = fixture_repo(workdir.path());
    let repo_str = repo.to_string_lossy().into_owned();
    write_cassettes(store.path(), &repo_str);
    let mut core = to_completion_proposed(store.path(), &repo_str);

    let outcome = core.decide_completion(MID, "intruder", false).unwrap();
    assert!(
        matches!(outcome, RecipeOutcome::Rejected(_)),
        "an unauthenticated return must be Rejected"
    );
    assert_eq!(
        core.mission_row(MID).unwrap().unwrap().1,
        "completion_proposed",
        "the mission state is unchanged"
    );
    assert!(!core
        .timeline(MID)
        .unwrap()
        .iter()
        .any(|e| e.entry_type == EventType::MissionReturned));
}

#[test]
fn a_return_from_the_wrong_state_is_rejected_never_completed() {
    let workdir = tempfile::tempdir().unwrap();
    let store = tempfile::tempdir().unwrap();
    let repo = fixture_repo(workdir.path());
    let repo_str = repo.to_string_lossy().into_owned();
    write_cassettes(store.path(), &repo_str);

    let mut core = Core::open(store.path()).unwrap();
    core.set_worker_cmd(vec![hermes_bin()]);
    core.set_fixtures_root(Path::new(&repo_str)).unwrap();
    assert!(matches!(
        core.start_build_feature(REQUEST, SLUG, &repo_str, "main", "principal_local")
            .unwrap(),
        RecipeOutcome::NeedsPlanApproval { .. }
    ));

    // The mission is in plan_review — a completion return is impossible here.
    let outcome = core
        .decide_completion(MID, "principal_local", false)
        .unwrap();
    match outcome {
        RecipeOutcome::Rejected(reason) => {
            assert!(reason.contains("completion_proposed"), "{reason}")
        }
        RecipeOutcome::Completed(_) => {
            panic!("a rejected return must NEVER surface as Completed")
        }
        other => panic!(
            "expected Rejected, got {:?}",
            std::mem::discriminant(&other)
        ),
    }
    assert_eq!(core.mission_row(MID).unwrap().unwrap().1, "plan_review");
}

// ── Blocker 4: plans are bounded before persistence ────────────────────────

fn brief_with_acs(repo: &str, acs: &[&str]) -> serde_json::Value {
    let criteria: Vec<_> = acs
        .iter()
        .map(|id| serde_json::json!({ "id": id, "text": "x", "verify": "gate:build" }))
        .collect();
    serde_json::json!({
        "schema_version": 1, "mission_id": "mis_dev", "title": "t", "outcome": "o",
        "scope": { "repo": repo, "base_branch": "main", "paths": ["src/**"], "forbidden_paths": [] },
        "acceptance_criteria": criteria,
        "gates_required": ["build"], "gate_commands": { "build": "true" },
        "autonomy_mode": "manual",
        "envelope_declared": { "network": "deny", "dependency_install": "ask", "secrets": [] },
        "budget": { "max_cost_usd": 5.0, "max_wall_minutes": 90, "max_interrupts": 3 },
        "classification": "internal", "owner": "principal_local"
    })
}

/// Create a mission whose planner "returns" `plan`, then run planning.
/// Returns the Core and the planning outcome.
fn plan_with(
    store: &Path,
    workdir: &Path,
    acs: &[&str],
    plan: serde_json::Value,
) -> (Core, CommandOutcome) {
    let repo = fixture_repo(workdir);
    let repo_str = repo.to_string_lossy().into_owned();
    let brief = brief_with_acs(&repo_str, acs);
    let key = wepld_providers::cassette_key(
        "plan",
        &wepld_artifacts::hash_hex(&serde_json::to_vec(&planner_pack(&brief)).unwrap()),
        "plan.v1",
        "fixture-model",
    );
    wepld_providers::write_cassette_entry(
        &store.join("cassettes/p.jsonl"),
        &key,
        &plan,
        "fixture-model",
    )
    .unwrap();

    let mut core = Core::open(store).unwrap();
    core.set_worker_cmd(vec![hermes_bin()]);
    core.set_fixtures_root(workdir).unwrap();
    let cmd = WCommand {
        command_id: command_id_for("create_mission", &brief),
        command_type: "create_mission".to_owned(),
        actor: "principal_local".to_owned(),
        payload: brief,
    };
    assert!(matches!(
        core.submit(&cmd).unwrap(),
        CommandOutcome::Accepted { .. }
    ));
    let outcome = core.plan_mission("mis_dev").unwrap();
    (core, outcome)
}

fn assert_plan_rejected(core: &Core, outcome: &CommandOutcome, needle: &str) {
    assert!(
        matches!(outcome, CommandOutcome::Rejected { reason } if reason.contains(needle)),
        "expected rejection containing {needle:?}, got {outcome:?}"
    );
    // Rejected before persistence: no task rows, mission still draft.
    assert!(core.tasks("mis_dev").unwrap().is_empty());
    assert_eq!(core.mission_row("mis_dev").unwrap().unwrap().1, "draft");
}

#[test]
fn an_overlong_task_title_is_rejected_before_persistence() {
    let workdir = tempfile::tempdir().unwrap();
    let store = tempfile::tempdir().unwrap();
    let plan = serde_json::json!({
        "tasks": [ { "id": "T1", "title": "t".repeat(MAX_TASK_TITLE_BYTES + 1), "satisfies": ["AC1"] } ]
    });
    let (core, outcome) = plan_with(store.path(), workdir.path(), &["AC1"], plan);
    assert_plan_rejected(&core, &outcome, "title too long");
}

#[test]
fn excessive_satisfies_are_rejected_before_persistence() {
    let workdir = tempfile::tempdir().unwrap();
    let store = tempfile::tempdir().unwrap();
    let sat: Vec<String> = (0..=MAX_SATISFIES_PER_TASK)
        .map(|_| "AC1".to_owned())
        .collect();
    let plan = serde_json::json!({
        "tasks": [ { "id": "T1", "title": "t", "satisfies": sat } ]
    });
    let (core, outcome) = plan_with(store.path(), workdir.path(), &["AC1"], plan);
    assert_plan_rejected(&core, &outcome, "too many satisfies");
}

#[test]
fn duplicate_satisfies_are_rejected_before_persistence() {
    let workdir = tempfile::tempdir().unwrap();
    let store = tempfile::tempdir().unwrap();
    let plan = serde_json::json!({
        "tasks": [ { "id": "T1", "title": "t", "satisfies": ["AC1", "AC1"] } ]
    });
    let (core, outcome) = plan_with(store.path(), workdir.path(), &["AC1"], plan);
    assert_plan_rejected(&core, &outcome, "duplicate satisfies");
}

#[test]
fn an_uncovered_acceptance_criterion_is_rejected_before_persistence() {
    let workdir = tempfile::tempdir().unwrap();
    let store = tempfile::tempdir().unwrap();
    // AC2 exists in the brief but no task satisfies it.
    let plan = serde_json::json!({
        "tasks": [ { "id": "T1", "title": "t", "satisfies": ["AC1"] } ]
    });
    let (core, outcome) = plan_with(store.path(), workdir.path(), &["AC1", "AC2"], plan);
    assert_plan_rejected(&core, &outcome, "not covered");
}

#[test]
fn an_oversized_serialized_plan_is_rejected_before_persistence() {
    let workdir = tempfile::tempdir().unwrap();
    let store = tempfile::tempdir().unwrap();
    // One task whose title alone exceeds the whole-plan byte bound — the
    // serialized-size check fires before any per-field check.
    let plan = serde_json::json!({
        "tasks": [ { "id": "T1", "title": "t".repeat(100 << 10), "satisfies": ["AC1"] } ]
    });
    let (core, outcome) = plan_with(store.path(), workdir.path(), &["AC1"], plan);
    assert_plan_rejected(&core, &outcome, "plan too large");
}

// ── Contract B: a failed edit attempt is contained, never promoted ─────────

/// A builder step whose SECOND edit fails at runtime (its target is a
/// pre-existing directory — deterministic, no timing race) after the first
/// edit was written. Proves the full failed-attempt containment lifecycle:
/// the phase is durably Failed, no snapshot fact exists, the mission never
/// reaches completion, no proposal ref or lesson can result, the failed
/// worktree is destroyed, and the failed task is never offered for rerun.
#[test]
fn a_mid_batch_runtime_failure_cannot_be_promoted_snapshotted_or_reused() {
    let workdir = tempfile::tempdir().unwrap();
    let store = tempfile::tempdir().unwrap();
    let repo = fixture_repo(workdir.path());
    let repo_str = repo.to_string_lossy().into_owned();
    // First edit valid; second edit targets "src" — a directory that exists in
    // the worktree — so prevalidation passes and execution fails on edit 2.
    write_cassettes_with_edits(
        store.path(),
        &repo_str,
        serde_json::json!({ "edits": [
            { "path": "src/main.rs", "content": EDITED },
            { "path": "src", "content": "cannot-write-a-directory" }
        ] }),
    );

    let mut core = Core::open(store.path()).unwrap();
    core.set_worker_cmd(vec![hermes_bin()]);
    core.set_fixtures_root(Path::new(&repo_str)).unwrap();
    assert!(matches!(
        core.start_build_feature(REQUEST, SLUG, &repo_str, "main", "principal_local")
            .unwrap(),
        RecipeOutcome::NeedsPlanApproval { .. }
    ));
    let outcome = core
        .approve_plan_and_execute(MID, "principal_local")
        .unwrap();
    match outcome {
        RecipeOutcome::Rejected(reason) => {
            assert!(reason.contains("did not complete"), "{reason}")
        }
        other => panic!(
            "a failed build must reject, got {:?}",
            std::mem::discriminant(&other)
        ),
    }

    // Durably failed: attempt and task both record the failure.
    let task = &core.tasks(MID).unwrap()[0];
    assert_eq!(task.state, "failed");
    let attempt_id = format!("att_{}_build", task.task_id);
    assert_eq!(core.attempt_state(&attempt_id).unwrap().unwrap(), "failed");

    // No promotion basis exists: promotion runs through a recorded snapshot
    // fact, and none was appended for the failed phase.
    let entries = core.all_entries().unwrap();
    assert!(!entries
        .iter()
        .any(|e| e.entry_type == EventType::WorkspaceSnapshotRecorded));
    assert!(!entries
        .iter()
        .any(|e| e.entry_type == EventType::CompletionProposed));
    assert!(!entries
        .iter()
        .any(|e| e.entry_type == EventType::GateEvaluated));

    // Completion cannot be accepted and no proposal ref can appear.
    assert!(matches!(
        core.accept_mission(MID, "principal_local").unwrap(),
        CommandOutcome::Rejected { .. }
    ));
    assert!(
        !proposal_ref_exists(&repo),
        "a failed edit attempt must never produce a proposal ref"
    );

    // No Engineering Memory lesson from a failed attempt.
    assert!(core.record_engineering_experience(MID).unwrap().is_none());
    assert!(!entries
        .iter()
        .any(|e| e.entry_type == EventType::InsightRecorded));

    // The failed attempt worktree was destroyed (git worktree remove --force),
    // so its partial contents cannot be reused as a retry basis.
    assert!(
        !store.path().join("worktrees").join(&attempt_id).exists(),
        "the failed worktree must be destroyed, not left for reuse"
    );

    // And the failed task is never offered again: a rerun finds nothing ready,
    // so V0 retry means a fresh mission from the base commit — never the
    // failed worktree.
    assert!(matches!(
        core.run_mission(MID).unwrap(),
        CommandOutcome::Rejected { .. }
    ));

    // The base branch and primary worktree remain untouched throughout.
    assert_eq!(
        std::fs::read_to_string(repo.join("src/main.rs")).unwrap(),
        "fn main() {}\n"
    );
    assert!(core.verify().unwrap().is_valid());
}

#[test]
fn a_boundary_sized_valid_plan_is_accepted() {
    let workdir = tempfile::tempdir().unwrap();
    let store = tempfile::tempdir().unwrap();
    // Title exactly at the byte bound, satisfies within bounds, AC covered.
    let plan = serde_json::json!({
        "tasks": [ { "id": "T1", "title": "t".repeat(MAX_TASK_TITLE_BYTES), "satisfies": ["AC1"] } ]
    });
    let (mut core, outcome) = plan_with(store.path(), workdir.path(), &["AC1"], plan);
    assert!(
        matches!(outcome, CommandOutcome::Accepted { .. }),
        "a payload exactly at the bounds must be accepted: {outcome:?}"
    );
    assert_eq!(
        core.mission_row("mis_dev").unwrap().unwrap().1,
        "plan_review"
    );
    // The boundary-sized plan passes re-validation at approval and its task
    // rows are materialized.
    assert!(matches!(
        core.approve_plan("mis_dev", "principal_local").unwrap(),
        CommandOutcome::Accepted { .. }
    ));
    assert_eq!(core.tasks("mis_dev").unwrap().len(), 1);
}
