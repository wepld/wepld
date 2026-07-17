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
    core.set_fixtures_root(Path::new(repo)).unwrap();
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
    core.set_fixtures_root(Path::new(&repo_str)).unwrap();
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
    core.set_fixtures_root(Path::new(&repo_str)).unwrap();
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

/// Attempt to create a mission on `repo` with the given autonomy mode via the
/// raw command path (so we can choose Bounded-Auto). Returns the Core and the
/// **creation outcome** — since the DEV preflight now runs at mission creation,
/// an unauthorized repo or Bounded-Auto is refused here. `override_repo` grants
/// an explicit uncontained override before creating.
fn running_mission(
    store: &Path,
    repo: &str,
    mode: &str,
    set_root: bool,
    override_repo: Option<&str>,
) -> (Core, CommandOutcome) {
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
        core.set_fixtures_root(Path::new(repo)).unwrap();
    }
    if let Some(o) = override_repo {
        core.allow_uncontained_repo(o, "principal_local").unwrap();
    }
    let cmd = WCommand {
        command_id: command_id_for("create_mission", &brief),
        command_type: "create_mission".to_owned(),
        actor: "principal_local".to_owned(),
        payload: brief,
    };
    let created = core.submit(&cmd).unwrap();
    // Best-effort next stages (no-ops if creation was denied).
    core.plan_mission("mis_dev").unwrap();
    core.approve_plan("mis_dev", "principal_local").unwrap();
    (core, created)
}

#[test]
fn dev_tier_accepts_a_repo_within_the_fixtures_root() {
    let workdir = tempfile::tempdir().unwrap();
    let store = tempfile::tempdir().unwrap();
    let repo = fixture_repo(workdir.path());
    let repo_str = repo.to_string_lossy().into_owned();
    let (mut core, created) = running_mission(store.path(), &repo_str, "manual", true, None);
    assert!(
        matches!(created, CommandOutcome::Accepted { .. }),
        "fixture repo creation accepted"
    );
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
    // No fixtures root, no override — creation itself is refused (before any
    // durable mission, planner spawn, gate, or proposal ref).
    let (core, created) = running_mission(store.path(), &repo_str, "manual", false, None);
    assert!(
        matches!(&created, CommandOutcome::Rejected { reason }
            if reason.contains("fixtures root") && reason.contains("--i-understand-dev-tier")),
        "got {created:?}"
    );
    assert!(
        core.mission_row("mis_dev").unwrap().is_none(),
        "no durable mission"
    );
    // No attempts were ever spawned for the denied repo.
    assert!(!core
        .all_entries()
        .unwrap()
        .iter()
        .any(|e| e.entry_type.code() == "AttemptSpawned"));
}

#[test]
fn dev_tier_rejects_bounded_auto() {
    let workdir = tempfile::tempdir().unwrap();
    let store = tempfile::tempdir().unwrap();
    let repo = fixture_repo(workdir.path());
    let repo_str = repo.to_string_lossy().into_owned();
    // Repo allowed via fixtures root, so Bounded-Auto is the ONLY blocker — and
    // it is refused at creation.
    let (core, created) = running_mission(store.path(), &repo_str, "bounded_auto", true, None);
    assert!(
        matches!(&created, CommandOutcome::Rejected { reason } if reason.contains("Bounded-Auto")),
        "got {created:?}"
    );
    assert!(
        core.mission_row("mis_dev").unwrap().is_none(),
        "no durable mission"
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

    // Explicit override for exactly this repo, granted (by an authenticated
    // actor) before creation — so creation and running are authorized.
    let (mut core, created) =
        running_mission(store.path(), &repo_str, "manual", false, Some(&repo_str));
    assert!(
        matches!(created, CommandOutcome::Accepted { .. }),
        "overridden repo creation accepted: {created:?}"
    );

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

// ── Blocker 1: no public auto-approval entrypoint ──────────────────────────

#[test]
fn no_public_recipe_entrypoint_can_auto_approve_or_auto_accept() {
    let workdir = tempfile::tempdir().unwrap();
    let store = tempfile::tempdir().unwrap();
    let repo = fixture_repo(workdir.path());
    let repo_str = repo.to_string_lossy().into_owned();
    write_cassettes(store.path(), &repo_str);
    let mut core = Core::open(store.path()).unwrap();
    core.set_worker_cmd(vec![hermes_bin()]);
    core.set_fixtures_root(Path::new(&repo_str)).unwrap();

    // The only public recipe entrypoint. A single call stops at plan approval.
    let outcome = core
        .start_build_feature(REQUEST, SLUG, &repo_str, "main", "principal_local")
        .unwrap();
    assert!(matches!(outcome, RecipeOutcome::NeedsPlanApproval { .. }));

    // No governance decision or execution happened from that one call.
    let codes: Vec<String> = core
        .all_entries()
        .unwrap()
        .iter()
        .map(|e| e.entry_type.code().to_owned())
        .collect();
    assert!(
        !codes.contains(&"PlanApproved".to_owned()),
        "no auto plan approval"
    );
    assert!(
        !codes.contains(&"MissionAccepted".to_owned()),
        "no auto acceptance"
    );
    assert!(
        !codes.contains(&"AttemptSpawned".to_owned()),
        "no execution attempt"
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
    assert!(
        !has_ref.status.success(),
        "no proposal ref from a start call"
    );
}

// ── Blocker 3: proposal-ref conflict safety & actor preservation ───────────

#[test]
fn acceptance_refuses_to_overwrite_a_conflicting_proposal_ref() {
    let workdir = tempfile::tempdir().unwrap();
    let store = tempfile::tempdir().unwrap();
    let repo = fixture_repo(workdir.path());
    let repo_str = repo.to_string_lossy().into_owned();
    write_cassettes(store.path(), &repo_str);
    let mut core = to_completion_proposed(store.path(), &repo_str);

    // Pre-create the proposal ref at a DIFFERENT commit (the base HEAD).
    let base = String::from_utf8(
        Command::new("git")
            .args(["rev-parse", "main"])
            .current_dir(&repo)
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap()
    .trim()
    .to_owned();
    Command::new("git")
        .args([
            "update-ref",
            &format!("refs/heads/wepld/mission-{MID}"),
            &base,
        ])
        .current_dir(&repo)
        .output()
        .unwrap();

    // Acceptance must detect the conflict, NOT overwrite, and defer.
    assert!(matches!(
        core.accept_mission(MID, "principal_local").unwrap(),
        CommandOutcome::Deferred { .. }
    ));
    assert_eq!(
        core.mission_row(MID).unwrap().unwrap().1,
        "acceptance_uncertain"
    );
    assert_eq!(accepted_events(&core), 0, "no MissionAccepted on conflict");
    let now = String::from_utf8(
        Command::new("git")
            .args(["rev-parse", &format!("refs/heads/wepld/mission-{MID}")])
            .current_dir(&repo)
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap()
    .trim()
    .to_owned();
    assert_eq!(
        now, base,
        "conflicting proposal ref must not be force-replaced"
    );
    assert!(core.all_entries().unwrap().iter().any(|e| e.entry_type
        == EventType::AttemptUncertain
        && e.payload_json["reason"]
            .as_str()
            .unwrap_or("")
            .contains("conflict")));
    assert!(core.verify().unwrap().is_valid());
}

#[test]
fn acceptance_retry_preserves_the_original_approver() {
    let workdir = tempfile::tempdir().unwrap();
    let store = tempfile::tempdir().unwrap();
    let repo = fixture_repo(workdir.path());
    let repo_str = repo.to_string_lossy().into_owned();
    write_cassettes(store.path(), &repo_str);
    let mut core = to_completion_proposed(store.path(), &repo_str);

    // Crash after the effect, before the final record — decision is durable.
    assert!(matches!(
        core.accept_mission_at(MID, "principal_local", AcceptFault::BeforeFinalRecord)
            .unwrap(),
        CommandOutcome::Deferred { .. }
    ));
    // A retry by a non-authenticated principal is refused — never rewrites the
    // recorded decision.
    assert!(matches!(
        core.accept_mission(MID, "intruder").unwrap(),
        CommandOutcome::Rejected { .. }
    ));
    assert_eq!(accepted_events(&core), 0);

    // The legitimate retry completes; MissionAccepted names the ORIGINAL
    // approver taken from the recorded DecisionResolved fact.
    assert!(matches!(
        core.accept_mission(MID, "principal_local").unwrap(),
        CommandOutcome::Accepted { .. }
    ));
    let ma = core
        .timeline(MID)
        .unwrap()
        .into_iter()
        .find(|e| e.entry_type == EventType::MissionAccepted)
        .unwrap();
    assert_eq!(
        ma.actor_id, "principal_local",
        "recorded approver preserved"
    );
    assert_eq!(accepted_events(&core), 1);
}

// ── Blocker 4: platform-correct (case-sensitive) path identity on Unix ─────

#[cfg(unix)]
#[test]
fn unix_case_sensitive_repos_are_distinct_scopes() {
    let workdir = tempfile::tempdir().unwrap();
    let store = tempfile::tempdir().unwrap();
    // Two real repos differing only by case (Unix filesystems are case-sensitive).
    let upper = fixture_repo(&workdir.path().join("Upper"));
    let lower = fixture_repo(&workdir.path().join("upper"));
    let upper_s = upper.to_string_lossy().into_owned();
    let lower_s = lower.to_string_lossy().into_owned();

    let mut core = Core::open(store.path()).unwrap();
    assert_ne!(
        core.project_identity(&upper_s).unwrap(),
        core.project_identity(&lower_s).unwrap(),
        "case-differing repos must not share a project identity"
    );
    core.set_fixtures_root(&upper).unwrap();
    assert!(core.is_repo_allowed_under_dev(&upper_s));
    assert!(
        !core.is_repo_allowed_under_dev(&lower_s),
        "lowercase sibling must not pass an uppercase fixtures-root check"
    );
}

// ── Blocker 5: worker-spawn boundary denies a denied repo (marker proof) ───

#[cfg(unix)]
#[test]
fn a_denied_repo_never_spawns_a_worker() {
    let workdir = tempfile::tempdir().unwrap();
    let store = tempfile::tempdir().unwrap();
    let repo = fixture_repo(workdir.path());
    let repo_str = repo.to_string_lossy().into_owned();
    let marker = workdir.path().join("WORKER_SPAWNED");

    let mut core = Core::open(store.path()).unwrap();
    core.set_fixtures_root(workdir.path()).unwrap(); // authorize creation
    let brief = serde_json::json!({
        "schema_version": 1, "mission_id": "mis_deny", "title": "t", "outcome": "o",
        "scope": { "repo": repo_str, "base_branch": "main", "paths": ["src/**"], "forbidden_paths": [] },
        "acceptance_criteria": [ { "id": "AC1", "text": "x", "verify": "gate:build" } ],
        "gates_required": ["build"], "gate_commands": { "build": "true" },
        "autonomy_mode": "manual",
        "envelope_declared": { "network": "deny", "dependency_install": "ask", "secrets": [] },
        "budget": { "max_cost_usd": 5.0, "max_wall_minutes": 90, "max_interrupts": 3 },
        "classification": "internal", "owner": "principal_local"
    });
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

    // Now DENY the repo (fixtures root no longer contains it) and arm a marker
    // "worker" that would prove a spawn if it ever ran.
    core.set_fixtures_root(store.path()).unwrap();
    core.set_worker_cmd(vec![
        "sh".to_owned(),
        "-c".to_owned(),
        format!("touch {}; exit 1", marker.display()),
    ]);
    let outcome = core.plan_mission("mis_deny").unwrap();
    assert!(
        matches!(outcome, CommandOutcome::Rejected { .. }),
        "denied plan: {outcome:?}"
    );
    assert!(
        !marker.exists(),
        "no planner worker may spawn for a denied repository"
    );
    assert!(!core
        .all_entries()
        .unwrap()
        .iter()
        .any(|e| e.entry_type.code() == "AttemptSpawned"));
}

// ── Blocker 2: untrusted identifiers never reach paths or refs ──────────────

fn dev_brief(repo: &str, mission_id: &str, base: &str, mode: &str) -> serde_json::Value {
    serde_json::json!({
        "schema_version": 1, "mission_id": mission_id, "title": "t", "outcome": "o",
        "scope": { "repo": repo, "base_branch": base, "paths": ["src/**"], "forbidden_paths": [] },
        "acceptance_criteria": [ { "id": "AC1", "text": "x", "verify": "gate:build" } ],
        "gates_required": ["build"], "gate_commands": { "build": "true" },
        "autonomy_mode": mode,
        "envelope_declared": { "network": "deny", "dependency_install": "ask", "secrets": [] },
        "budget": { "max_cost_usd": 5.0, "max_wall_minutes": 90, "max_interrupts": 3 },
        "classification": "internal", "owner": "principal_local"
    })
}

fn submit_create(core: &mut Core, brief: serde_json::Value) -> CommandOutcome {
    let cmd = WCommand {
        command_id: command_id_for("create_mission", &brief),
        command_type: "create_mission".to_owned(),
        actor: "principal_local".to_owned(),
        payload: brief,
    };
    core.submit(&cmd).unwrap()
}

#[test]
fn an_invalid_slug_is_rejected_before_any_mission_is_created() {
    let workdir = tempfile::tempdir().unwrap();
    let store = tempfile::tempdir().unwrap();
    let repo = fixture_repo(workdir.path());
    let repo_str = repo.to_string_lossy().into_owned();
    let mut core = Core::open(store.path()).unwrap();
    core.set_worker_cmd(vec![hermes_bin()]);
    core.set_fixtures_root(Path::new(&repo_str)).unwrap();
    let doc = spec();
    for bad in [
        "../../outside",
        "/absolute",
        "a\\b",
        "UPPER",
        "a/b",
        "-x",
        "",
    ] {
        let outcome = core
            .create_mission_from_spec(&doc, bad, &repo_str, "main", "principal_local")
            .unwrap();
        assert!(
            matches!(&outcome, CommandOutcome::Rejected { reason } if reason.contains("slug")),
            "slug {bad:?} → {outcome:?}"
        );
    }
    assert!(core
        .all_entries()
        .unwrap()
        .iter()
        .all(|e| e.entry_type.code() != "MissionCreated"));
    assert!(!workdir.path().join("outside").exists());
}

#[test]
fn malicious_mission_id_and_base_branch_are_rejected_at_creation() {
    let workdir = tempfile::tempdir().unwrap();
    let store = tempfile::tempdir().unwrap();
    let repo = fixture_repo(workdir.path());
    let repo_str = repo.to_string_lossy().into_owned();
    let mut core = Core::open(store.path()).unwrap();
    core.set_fixtures_root(workdir.path()).unwrap();

    for bad_base in ["--output=/tmp/leak", "a..b", "a@{0}", "a b"] {
        let outcome = submit_create(
            &mut core,
            dev_brief(&repo_str, "mis_ok", bad_base, "manual"),
        );
        assert!(
            matches!(&outcome, CommandOutcome::Rejected { reason } if reason.contains("base_branch")),
            "base {bad_base:?} → {outcome:?}"
        );
    }
    for bad_id in ["../evil", "a/b", "-x", "..", "a\\b"] {
        let outcome = submit_create(&mut core, dev_brief(&repo_str, bad_id, "main", "manual"));
        assert!(
            matches!(&outcome, CommandOutcome::Rejected { reason } if reason.contains("mission_id")),
            "id {bad_id:?} → {outcome:?}"
        );
        assert!(
            core.mission_row(bad_id).unwrap().is_none(),
            "no mission for {bad_id:?}"
        );
    }
    assert!(core
        .all_entries()
        .unwrap()
        .iter()
        .all(|e| e.entry_type.code() != "MissionCreated"));
    assert!(!workdir.path().join("leak").exists());
}

#[test]
fn a_malicious_plan_task_id_is_rejected_with_no_task_row_or_worker() {
    let workdir = tempfile::tempdir().unwrap();
    let store = tempfile::tempdir().unwrap();
    let repo = fixture_repo(workdir.path());
    let repo_str = repo.to_string_lossy().into_owned();
    let brief = dev_brief(&repo_str, "mis_ok", "main", "manual");

    // The planner "returns" a task whose id is a path-traversal string.
    let evil_plan = serde_json::json!({
        "tasks": [ { "id": "../../../outside", "title": "t", "satisfies": ["AC1"] } ]
    });
    let key = wepld_providers::cassette_key(
        "plan",
        &wepld_artifacts::hash_hex(&serde_json::to_vec(&planner_pack(&brief)).unwrap()),
        "plan.v1",
        "fixture-model",
    );
    wepld_providers::write_cassette_entry(
        &store.path().join("cassettes/p.jsonl"),
        &key,
        &evil_plan,
        "fixture-model",
    )
    .unwrap();

    let mut core = Core::open(store.path()).unwrap();
    core.set_worker_cmd(vec![hermes_bin()]);
    core.set_fixtures_root(workdir.path()).unwrap();
    assert!(matches!(
        submit_create(&mut core, brief),
        CommandOutcome::Accepted { .. }
    ));

    let outcome = core.plan_mission("mis_ok").unwrap();
    assert!(
        matches!(&outcome, CommandOutcome::Rejected { reason } if reason.contains("task id")),
        "malicious task id → {outcome:?}"
    );
    assert!(core.tasks("mis_ok").unwrap().is_empty());
    assert_eq!(core.mission_row("mis_ok").unwrap().unwrap().1, "draft");
    assert!(!workdir.path().join("outside").exists());
}

// ── Blocker 3: Deferred is preserved (not flattened to Rejected) ────────────

#[test]
fn an_acceptance_conflict_is_deferred_not_rejected() {
    let workdir = tempfile::tempdir().unwrap();
    let store = tempfile::tempdir().unwrap();
    let repo = fixture_repo(workdir.path());
    let repo_str = repo.to_string_lossy().into_owned();
    write_cassettes(store.path(), &repo_str);
    let mut core = to_completion_proposed(store.path(), &repo_str);

    let base = String::from_utf8(
        Command::new("git")
            .args(["rev-parse", "main"])
            .current_dir(&repo)
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap()
    .trim()
    .to_owned();
    Command::new("git")
        .args([
            "update-ref",
            &format!("refs/heads/wepld/mission-{MID}"),
            &base,
        ])
        .current_dir(&repo)
        .output()
        .unwrap();

    match core
        .decide_completion(MID, "principal_local", true)
        .unwrap()
    {
        RecipeOutcome::Deferred {
            mission_id, state, ..
        } => {
            assert_eq!(mission_id, MID);
            assert_eq!(state, "acceptance_uncertain");
        }
        other => panic!(
            "expected Deferred, got {:?}",
            std::mem::discriminant(&other)
        ),
    }

    // Deferred and Rejected are distinct outcomes.
    let deferred = matches!(
        core.decide_completion(MID, "principal_local", true)
            .unwrap(),
        RecipeOutcome::Deferred { .. }
    );
    let rejected = matches!(
        core.decide_completion(MID, "not-a-principal", true)
            .unwrap(),
        RecipeOutcome::Rejected(_)
    );
    assert!(
        deferred && rejected,
        "Deferred must be distinct from Rejected"
    );
}

// ── Blocker 4: fixtures-root canonicalization fails closed ─────────────────

#[test]
fn set_fixtures_root_fails_closed_and_preserves_the_prior_root() {
    let good = tempfile::tempdir().unwrap();
    let store = tempfile::tempdir().unwrap();
    let repo = fixture_repo(good.path());
    let repo_str = repo.to_string_lossy().into_owned();
    let mut core = Core::open(store.path()).unwrap();

    core.set_fixtures_root(good.path()).unwrap();
    assert!(core.is_repo_allowed_under_dev(&repo_str));

    // A non-canonicalizable root errors AND does not clear the prior one.
    assert!(
        core.set_fixtures_root(Path::new("/nonexistent/definitely/not/here"))
            .is_err(),
        "non-canonicalizable fixtures root must error"
    );
    assert!(
        core.is_repo_allowed_under_dev(&repo_str),
        "a failed update must preserve the previously-authorized root"
    );
}
