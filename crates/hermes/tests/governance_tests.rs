//! Governance, acceptance-recovery, and DEV-tier tests (Blockers 1–4). Run
//! against the real hermes binary and real git so nothing is mocked away.
//! Proves: explicit approvals with true-actor provenance; acceptance produces a
//! proposal ref and never merges or mutates the primary worktree; the
//! effect/ledger sequence recovers idempotently from an interrupted acceptance;
//! and the DEV tier refuses arbitrary repos and Bounded-Auto unless overridden.

use std::path::Path;
use std::process::Command;
use wepld_contracts::command::{Command as WCommand, CommandOutcome};
use wepld_contracts::vocabulary::EventType;
use wepld_runtime::{builder_pack, command_id_for, planner_pack, AcceptFault, Core, RecipeOutcome};
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
    wepld_providers::write_cassette_entry(
        &cassette,
        &build_key,
        &serde_json::json!({ "edits": [ { "path": "src/main.rs", "content": EDITED } ] }),
        "fixture-model",
    )
    .unwrap();
}

/// Drive a mission to `completion_proposed` via the staged recipe API; return
/// the live Core (with fixtures root set to the repo) and the repo path.
fn to_completion_proposed(store: &Path, repo: &str) -> Core {
    let mut core = Core::open(store).unwrap();
    core.set_worker_cmd(vec![hermes_bin()]);
    core.set_fixtures_root(Path::new(repo));
    match core
        .start_build_feature(REQUEST, SLUG, repo, "main", "principal_local")
        .unwrap()
    {
        RecipeOutcome::NeedsPlanApproval { mission_id, .. } => assert_eq!(mission_id, MID),
        other => panic!(
            "expected NeedsPlanApproval, got {:?}",
            std::mem::discriminant(&other)
        ),
    }
    match core
        .approve_plan_and_execute(MID, "principal_local")
        .unwrap()
    {
        RecipeOutcome::NeedsCompletionApproval {
            mission_id,
            proposal,
        } => {
            assert_eq!(mission_id, MID);
            assert!(!proposal.snapshot_commit.is_empty());
            assert!(proposal.proposal_ref.contains(MID));
        }
        other => panic!(
            "expected NeedsCompletionApproval, got {:?}",
            std::mem::discriminant(&other)
        ),
    }
    core
}

fn accepted_events(core: &Core) -> usize {
    core.all_entries()
        .unwrap()
        .iter()
        .filter(|e| e.entry_type == EventType::MissionAccepted)
        .count()
}

// ── Blocker 1: explicit approvals & true-actor provenance ──────────────────

#[test]
fn start_stops_at_plan_approval_without_executing() {
    let workdir = tempfile::tempdir().unwrap();
    let store = tempfile::tempdir().unwrap();
    let repo = fixture_repo(workdir.path());
    let repo_str = repo.to_string_lossy().into_owned();
    write_cassettes(store.path(), &repo_str);

    let mut core = Core::open(store.path()).unwrap();
    core.set_worker_cmd(vec![hermes_bin()]);
    core.set_fixtures_root(Path::new(&repo_str));
    let outcome = core
        .start_build_feature(REQUEST, SLUG, &repo_str, "main", "principal_local")
        .unwrap();
    assert!(matches!(outcome, RecipeOutcome::NeedsPlanApproval { .. }));

    // The mission exists in plan_review; nothing executed (no attempt, no gate).
    assert_eq!(core.mission_row(MID).unwrap().unwrap().1, "plan_review");
    let types: Vec<_> = core
        .timeline(MID)
        .unwrap()
        .iter()
        .map(|e| e.entry_type)
        .collect();
    assert!(!types.contains(&EventType::PlanApproved));
    assert!(!types.contains(&EventType::AttemptSpawned));
    assert!(!types.contains(&EventType::GateEvaluated));
}

#[test]
fn approvals_require_an_authenticated_principal() {
    let workdir = tempfile::tempdir().unwrap();
    let store = tempfile::tempdir().unwrap();
    let repo = fixture_repo(workdir.path());
    let repo_str = repo.to_string_lossy().into_owned();
    write_cassettes(store.path(), &repo_str);
    let mut core = Core::open(store.path()).unwrap();
    core.set_worker_cmd(vec![hermes_bin()]);
    core.set_fixtures_root(Path::new(&repo_str));
    core.start_build_feature(REQUEST, SLUG, &repo_str, "main", "principal_local")
        .unwrap();

    // An empty / unknown principal cannot approve the plan or accept completion.
    assert!(matches!(
        core.approve_plan(MID, "").unwrap(),
        CommandOutcome::Rejected { .. }
    ));
    assert!(matches!(
        core.approve_plan(MID, "intruder").unwrap(),
        CommandOutcome::Rejected { .. }
    ));
    assert_eq!(core.mission_row(MID).unwrap().unwrap().1, "plan_review");
    assert!(matches!(
        core.accept_mission(MID, "").unwrap(),
        CommandOutcome::Rejected { .. }
    ));
}

#[test]
fn acceptance_records_real_actor_and_creates_proposal_ref_not_merge() {
    let workdir = tempfile::tempdir().unwrap();
    let store = tempfile::tempdir().unwrap();
    let repo = fixture_repo(workdir.path());
    let repo_str = repo.to_string_lossy().into_owned();
    write_cassettes(store.path(), &repo_str);
    let mut core = to_completion_proposed(store.path(), &repo_str);

    let base_before = String::from_utf8(
        Command::new("git")
            .args(["rev-parse", "main"])
            .current_dir(&repo)
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap();

    // Completion is NOT yet accepted; an explicit decision is required.
    assert_eq!(
        core.mission_row(MID).unwrap().unwrap().1,
        "completion_proposed"
    );
    assert_eq!(
        accepted_events(&core),
        0,
        "no acceptance until a human decides"
    );

    match core
        .decide_completion(MID, "principal_local", true)
        .unwrap()
    {
        RecipeOutcome::Completed(bf) => assert_eq!(bf.report.state, "accepted"),
        other => panic!(
            "expected Completed, got {:?}",
            std::mem::discriminant(&other)
        ),
    }

    // True-actor provenance: PlanApproved and MissionAccepted name the principal.
    let entries = core.timeline(MID).unwrap();
    for et in [EventType::PlanApproved, EventType::MissionAccepted] {
        let e = entries.iter().find(|e| e.entry_type == et).unwrap();
        assert_eq!(
            e.actor_id, "principal_local",
            "{et:?} names the real approver"
        );
        assert_eq!(
            serde_json::to_value(e.actor_type).unwrap(),
            "human",
            "{et:?} is a human decision"
        );
    }

    // The proposal ref exists and carries the change; base branch is untouched.
    let show = Command::new("git")
        .args([
            "show",
            &format!("refs/heads/wepld/mission-{MID}:src/main.rs"),
        ])
        .current_dir(&repo)
        .output()
        .unwrap();
    assert!(show.status.success() && String::from_utf8_lossy(&show.stdout).contains("VERSION"));
    let base_after = String::from_utf8(
        Command::new("git")
            .args(["rev-parse", "main"])
            .current_dir(&repo)
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap();
    assert_eq!(base_before, base_after, "base branch was never merged into");
    // Primary worktree stays clean.
    let status = Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(&repo)
        .output()
        .unwrap();
    assert!(String::from_utf8_lossy(&status.stdout).trim().is_empty());
    assert_eq!(
        std::fs::read_to_string(repo.join("src/main.rs")).unwrap(),
        "fn main() {}\n"
    );
}

// ── Blocker 3: effect/ledger recovery ──────────────────────────────────────

#[test]
fn acceptance_recovers_after_crash_before_effect() {
    let workdir = tempfile::tempdir().unwrap();
    let store = tempfile::tempdir().unwrap();
    let repo = fixture_repo(workdir.path());
    let repo_str = repo.to_string_lossy().into_owned();
    write_cassettes(store.path(), &repo_str);
    let mut core = to_completion_proposed(store.path(), &repo_str);

    // Decision committed, but the process "crashes" before the ref effect.
    assert!(matches!(
        core.accept_mission_at(MID, "principal_local", AcceptFault::BeforeEffect)
            .unwrap(),
        CommandOutcome::Deferred { .. }
    ));
    assert_eq!(
        core.mission_row(MID).unwrap().unwrap().1,
        "acceptance_pending"
    );
    assert_eq!(accepted_events(&core), 0, "no false MissionAccepted");
    let no_ref = Command::new("git")
        .args([
            "rev-parse",
            "--verify",
            "--quiet",
            &format!("refs/heads/wepld/mission-{MID}"),
        ])
        .current_dir(&repo)
        .output()
        .unwrap();
    assert!(!no_ref.status.success(), "no proposal ref yet");

    // Retry heals idempotently → exactly one acceptance.
    assert!(matches!(
        core.accept_mission(MID, "principal_local").unwrap(),
        CommandOutcome::Accepted { .. }
    ));
    assert_eq!(core.mission_row(MID).unwrap().unwrap().1, "accepted");
    assert_eq!(accepted_events(&core), 1);
    assert!(core.verify().unwrap().is_valid());
}

#[test]
fn acceptance_recovers_after_crash_before_final_record() {
    let workdir = tempfile::tempdir().unwrap();
    let store = tempfile::tempdir().unwrap();
    let repo = fixture_repo(workdir.path());
    let repo_str = repo.to_string_lossy().into_owned();
    write_cassettes(store.path(), &repo_str);
    let mut core = to_completion_proposed(store.path(), &repo_str);

    // Effect done (proposal ref created), but "crash" before the final record.
    assert!(matches!(
        core.accept_mission_at(MID, "principal_local", AcceptFault::BeforeFinalRecord)
            .unwrap(),
        CommandOutcome::Deferred { .. }
    ));
    assert_eq!(
        core.mission_row(MID).unwrap().unwrap().1,
        "acceptance_pending"
    );
    assert_eq!(
        accepted_events(&core),
        0,
        "no MissionAccepted before the effect is confirmed"
    );
    let has_ref = Command::new("git")
        .args([
            "rev-parse",
            "--verify",
            "--quiet",
            &format!("refs/heads/wepld/mission-{MID}"),
        ])
        .current_dir(&repo)
        .output()
        .unwrap();
    assert!(has_ref.status.success(), "the reversible effect did happen");

    // Retry probes the existing ref and completes idempotently — one acceptance.
    assert!(matches!(
        core.accept_mission(MID, "principal_local").unwrap(),
        CommandOutcome::Accepted { .. }
    ));
    assert_eq!(accepted_events(&core), 1, "no duplicate MissionAccepted");
    assert!(core.verify().unwrap().is_valid());
}

#[test]
fn duplicate_acceptance_is_idempotent() {
    let workdir = tempfile::tempdir().unwrap();
    let store = tempfile::tempdir().unwrap();
    let repo = fixture_repo(workdir.path());
    let repo_str = repo.to_string_lossy().into_owned();
    write_cassettes(store.path(), &repo_str);
    let mut core = to_completion_proposed(store.path(), &repo_str);

    assert!(matches!(
        core.accept_mission(MID, "principal_local").unwrap(),
        CommandOutcome::Accepted { .. }
    ));
    // A second, duplicate acceptance returns the recorded fact — no new event.
    assert!(matches!(
        core.accept_mission(MID, "principal_local").unwrap(),
        CommandOutcome::Accepted { .. }
    ));
    assert_eq!(accepted_events(&core), 1, "exactly one MissionAccepted");
    assert!(core.verify().unwrap().is_valid());
}

// ── Blocker 4: DEV-tier safety caps ────────────────────────────────────────

/// Build a running mission on `repo` with the given autonomy mode, via the raw
/// command path (so we can choose Bounded-Auto), ready for `run_mission`.
fn running_mission(store: &Path, repo: &str, mode: &str, set_root: bool) -> Core {
    let brief = serde_json::json!({
        "schema_version": 1, "mission_id": "mis_dev", "title": "t", "outcome": "o",
        "scope": { "repo": repo, "base_branch": "main", "paths": ["src/**"], "forbidden_paths": [] },
        "acceptance_criteria": [ { "id": "AC1", "text": "x", "verify": "gate:build" } ],
        "gates_required": ["build"], "gate_commands": { "build": "true" },
        "autonomy_mode": mode,
        "envelope_declared": { "network": "deny", "dependency_install": "ask", "secrets": [] },
        "budget": { "max_cost_usd": 5.0, "max_wall_minutes": 90, "max_interrupts": 3 },
        "classification": "internal", "owner": "principal_local"
    });
    let key = wepld_providers::cassette_key(
        "plan",
        &wepld_artifacts::hash_hex(&serde_json::to_vec(&planner_pack(&brief)).unwrap()),
        "plan.v1",
        "fixture-model",
    );
    wepld_providers::write_cassette_entry(
        &store.join("cassettes/p.jsonl"),
        &key,
        &serde_json::json!({ "tasks": [ { "id": "T1", "title": "t", "satisfies": ["AC1"] } ] }),
        "fixture-model",
    )
    .unwrap();

    let mut core = Core::open(store).unwrap();
    core.set_worker_cmd(vec![hermes_bin()]);
    if set_root {
        core.set_fixtures_root(Path::new(repo));
    }
    let cmd = WCommand {
        command_id: command_id_for("create_mission", &brief),
        command_type: "create_mission".to_owned(),
        actor: "principal_local".to_owned(),
        payload: brief,
    };
    core.submit(&cmd).unwrap();
    core.plan_mission("mis_dev").unwrap();
    core.approve_plan("mis_dev", "principal_local").unwrap();
    core
}

#[test]
fn dev_tier_accepts_a_repo_within_the_fixtures_root() {
    let workdir = tempfile::tempdir().unwrap();
    let store = tempfile::tempdir().unwrap();
    let repo = fixture_repo(workdir.path());
    let repo_str = repo.to_string_lossy().into_owned();
    let mut core = running_mission(store.path(), &repo_str, "manual", true);
    // Within fixtures root + Manual → runs (Deferred/Accepted, not tier-rejected).
    let outcome = core.run_mission("mis_dev").unwrap();
    let rejected_by_tier =
        matches!(&outcome, CommandOutcome::Rejected { reason } if reason.contains("DEV tier"));
    assert!(
        !rejected_by_tier,
        "a fixture repo must not be tier-rejected: {outcome:?}"
    );
}

#[test]
fn dev_tier_rejects_an_arbitrary_repo_by_default() {
    let workdir = tempfile::tempdir().unwrap();
    let store = tempfile::tempdir().unwrap();
    let repo = fixture_repo(workdir.path());
    let repo_str = repo.to_string_lossy().into_owned();
    // No fixtures root, no override.
    let mut core = running_mission(store.path(), &repo_str, "manual", false);
    let outcome = core.run_mission("mis_dev").unwrap();
    assert!(
        matches!(&outcome, CommandOutcome::Rejected { reason }
            if reason.contains("fixtures root") && reason.contains("--i-understand-dev-tier")),
        "got {outcome:?}"
    );
}

#[test]
fn dev_tier_rejects_bounded_auto() {
    let workdir = tempfile::tempdir().unwrap();
    let store = tempfile::tempdir().unwrap();
    let repo = fixture_repo(workdir.path());
    let repo_str = repo.to_string_lossy().into_owned();
    // Repo allowed via fixtures root, so Bounded-Auto is the ONLY blocker.
    let mut core = running_mission(store.path(), &repo_str, "bounded_auto", true);
    let outcome = core.run_mission("mis_dev").unwrap();
    assert!(
        matches!(&outcome, CommandOutcome::Rejected { reason } if reason.contains("Bounded-Auto")),
        "got {outcome:?}"
    );
}

#[test]
fn dev_tier_override_permits_only_that_repo_and_is_recorded() {
    let workdir = tempfile::tempdir().unwrap();
    let store = tempfile::tempdir().unwrap();
    let repo = fixture_repo(workdir.path());
    let repo_str = repo.to_string_lossy().into_owned();
    let other = fixture_repo(&workdir.path().join("other-parent"));
    let other_str = other.to_string_lossy().into_owned();

    let mut core = running_mission(store.path(), &repo_str, "manual", false);
    // Explicit override for exactly this repo, by an authenticated actor.
    core.allow_uncontained_repo(&repo_str, "principal_local")
        .unwrap();

    let outcome = core.run_mission("mis_dev").unwrap();
    assert!(
        !matches!(&outcome, CommandOutcome::Rejected { reason } if reason.contains("DEV tier")),
        "overridden repo runs: {outcome:?}"
    );

    // The override is durable evidence (actor, repo, tier, warning).
    let ov = core
        .all_entries()
        .unwrap()
        .into_iter()
        .find(|e| {
            e.entry_type == EventType::SandboxTierDetected
                && e.payload_json["override"] == "allow_uncontained_repo"
        })
        .expect("override recorded");
    assert_eq!(ov.actor_id, "principal_local");
    assert!(ov.payload_json["warning"]
        .as_str()
        .unwrap()
        .contains("no OS containment"));

    // A DIFFERENT repo is still rejected — the override is not a blanket grant.
    assert!(!core.is_repo_allowed_under_dev(&other_str));
}

#[test]
fn dev_tier_override_requires_an_authenticated_actor() {
    let store = tempfile::tempdir().unwrap();
    let mut core = Core::open(store.path()).unwrap();
    assert!(core.allow_uncontained_repo("/tmp/whatever", "").is_err());
    assert!(core
        .allow_uncontained_repo("/tmp/whatever", "intruder")
        .is_err());
}
