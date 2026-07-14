//! wepld-runtime — the Core (v2-02). Day-3 scope: the command pipeline
//! (idempotency → authorization stub → validation → transition, one
//! transaction) with `create_mission`, plus read ports for timeline and
//! chain verification. The phase engine arrives on Day 5+.
//!
//! Boundary rule (IMPL-02): this crate's transition code is the only holder
//! of `wepld_ledger::Tx` in the workspace.

mod commands;
mod gates;
mod memory;
mod orchestration;
mod phase;
mod recipe;
mod report;
mod spec;

pub use commands::command_id_for;
pub use memory::RecordedLesson;
pub use orchestration::{builder_pack, planner_pack, AcceptFault};
pub use phase::{PhaseOutcome, PhaseRun, PhaseSpec};
pub use recipe::{
    specify_memory_entries, specify_pack, BuildFeatureReport, CompletionProposal, RecipeOutcome,
    MEMORY_POLICY,
};
pub use report::EngineeringReport;

use std::path::{Path, PathBuf};
use wepld_contracts::command::{Command, CommandOutcome};
use wepld_contracts::envelope::SandboxTier;
use wepld_contracts::ledger::{ActorType, AggregateType, LedgerEntry};
use wepld_contracts::mission::AutonomyMode;
use wepld_contracts::vocabulary::EventType;
use wepld_ledger::{ChainReport, LedgerError, LedgerStore, NewEntry, TaskRow};
use wepld_workspace::Workspace;

/// The truthful DEV-tier disclosure surfaced to operators (IADR-0003). The
/// Envelope is *descriptive* under DEV — there is no OS-level enforcement.
pub const DEV_TIER_WARNING: &str =
    "DEV tier: no OS containment; worker and gate processes have ambient host authority.";

/// The single authenticated local principal in the MVP. Governance decisions
/// (plan approval, completion acceptance, tier override) must name a principal;
/// the Core refuses to fabricate a human actor.
pub const LOCAL_PRINCIPAL: &str = "principal_local";

/// DEV-tier safety caps (IADR-0003): fixture repositories only, unless the
/// founder grants an explicit, actor-attributed override for one repo.
#[derive(Default)]
struct DevTier {
    /// Canonical fixtures root; missions must operate within it by default.
    fixtures_root: Option<PathBuf>,
    /// Explicit override: (canonical repo path, granting actor). Permits that
    /// one uncontained repo — never a blanket default.
    allow_uncontained: Option<(PathBuf, String)>,
}

#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {
    #[error(transparent)]
    Ledger(#[from] LedgerError),
    #[error("serialization error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("artifact store error: {0}")]
    Cas(#[from] wepld_artifacts::CasError),
    #[error("gateway error: {0}")]
    Gateway(#[from] wepld_providers::GatewayError),
    #[error("workspace error: {0}")]
    Workspace(#[from] wepld_workspace::WorkspaceError),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("unauthenticated principal: {0:?} (a governance decision must name an authenticated principal)")]
    Unauthenticated(String),
    #[error("path cannot be canonicalized (does it exist?): {0:?}")]
    UnresolvablePath(String),
}

/// Whether `actor` is an authenticated principal that may make a governance
/// decision. MVP: the single local principal. Empty or unknown ids are refused,
/// so the Core never fabricates a human actor.
pub(crate) fn is_authenticated_principal(actor: &str) -> bool {
    actor == LOCAL_PRINCIPAL
}

/// Canonicalize a path for stable scope comparison and project identity, with
/// **platform-correct** case handling: Windows filesystems are case-insensitive
/// so case is normalized; Unix/macOS paths are case-sensitive so case is
/// **preserved** (never lossily lowercased — two repos differing only by case
/// must stay distinct). Returns `None` if the path cannot be canonicalized.
pub(crate) fn canonical_scope_path(path: &Path) -> Option<PathBuf> {
    let canon = std::fs::canonicalize(path).ok()?;
    #[cfg(windows)]
    {
        Some(PathBuf::from(canon.to_string_lossy().to_lowercase()))
    }
    #[cfg(not(windows))]
    {
        Some(canon)
    }
}

pub struct Core {
    store: LedgerStore,
    cas: wepld_artifacts::Cas,
    gateway: wepld_providers::Gateway,
    /// How to spawn the WWP worker runtime (default: `hermes` on PATH). The
    /// Runtime decides which runtime executes; Hermes is the flagship.
    worker_cmd: Vec<String>,
    /// The operational store directory (worktrees are created beneath it).
    root: std::path::PathBuf,
    /// DEV-tier safety caps and any explicit override.
    dev: DevTier,
}

impl Core {
    /// Open with default configuration: cassettes from `<dir>/cassettes`.
    pub fn open(dir: &Path) -> Result<Self, RuntimeError> {
        let cassettes = dir.join("cassettes");
        Self::open_with(dir, &[&cassettes])
    }

    /// Open (or create) the operational store and ensure the tier fact exists.
    /// Until M4 implements real tier detection, the honest tier is `DEV`
    /// (IADR-0003): no isolation, disclosed, Manual-mode caps.
    ///
    /// All provider access is constructed here: the fixture adapter is the
    /// default and only M0 adapter (IADR-0002); reasoning stays optional
    /// (IADR-0007 §1).
    pub fn open_with(dir: &Path, cassette_dirs: &[&Path]) -> Result<Self, RuntimeError> {
        let cas = wepld_artifacts::Cas::open(&dir.join("artifacts"))?;
        let mut gateway = wepld_providers::Gateway::new(wepld_providers::SchemaRegistry::default());
        gateway.register_adapter(Box::new(wepld_providers::FixtureAdapter::load(
            cassette_dirs,
        )?));
        gateway.register_profile(wepld_providers::Profile {
            name: "fixture-default".to_owned(),
            adapter: "fixture".to_owned(),
            model: "fixture-model".to_owned(),
            timeout_ms: 5000,
        })?;

        let mut store = LedgerStore::open(dir)?;
        if store.last_seq()? == 0 {
            let tier = serde_json::to_value(SandboxTier::Dev)?;
            store.transact(|tx| {
                tx.append(&NewEntry {
                    entry_type: EventType::SandboxTierDetected,
                    schema_version: 1,
                    aggregate_type: AggregateType::System,
                    aggregate_id: "system".to_owned(),
                    actor_type: ActorType::Core,
                    actor_id: "core".to_owned(),
                    correlation_id: "system".to_owned(),
                    causation_ref: None,
                    payload: serde_json::json!({
                        "tier": tier,
                        "statement": "no isolation — development tier; Manual mode and fixture repositories only",
                        "self_test": "not_applicable"
                    }),
                })?;
                Ok(())
            })?;
        }
        Ok(Self {
            store,
            cas,
            gateway,
            worker_cmd: vec!["hermes".to_owned()],
            root: dir.to_path_buf(),
            dev: DevTier::default(),
        })
    }

    /// Point the Core at a specific WWP worker binary (tests and the CLI,
    /// which locate `hermes` next to `wepld`).
    pub fn set_worker_cmd(&mut self, cmd: Vec<String>) {
        self.worker_cmd = cmd;
    }

    // ── DEV-tier safety (IADR-0003, Blocker 4) ─────────────────────────────

    /// Set the canonical fixtures root. Under DEV, missions must operate on a
    /// repository within this root unless an explicit override is granted.
    /// **Fails closed (Blocker 4):** a path that cannot be canonicalized returns
    /// `UnresolvablePath` and leaves any previously-authorized root **unchanged**
    /// — a failed update never silently clears or weakens authorization.
    pub fn set_fixtures_root(&mut self, path: &Path) -> Result<(), RuntimeError> {
        let canon = canonical_scope_path(path)
            .ok_or_else(|| RuntimeError::UnresolvablePath(path.display().to_string()))?;
        self.dev.fixtures_root = Some(canon);
        Ok(())
    }

    /// Grant an explicit, actor-attributed override permitting one uncontained
    /// (non-fixture) repository under the DEV tier — the `--i-understand-dev-tier`
    /// escape hatch. Recorded durably (repo, actor, tier, warning). Refuses an
    /// unauthenticated actor; there is no silent or default override.
    pub fn allow_uncontained_repo(&mut self, repo: &str, actor: &str) -> Result<(), RuntimeError> {
        if !is_authenticated_principal(actor) {
            return Err(RuntimeError::Unauthenticated(actor.to_owned()));
        }
        // No lowercase fallback: an override for a path that cannot be resolved
        // is refused outright (never silently trusts an unresolved string).
        let canon = canonical_scope_path(Path::new(repo))
            .ok_or_else(|| RuntimeError::UnresolvablePath(repo.to_owned()))?;
        let repo_disp = canon.to_string_lossy().into_owned();
        self.dev.allow_uncontained = Some((canon, actor.to_owned()));
        let tier = serde_json::to_value(SandboxTier::Dev)?;
        self.store_mut().transact(|tx| {
            tx.append(&NewEntry {
                entry_type: EventType::SandboxTierDetected,
                schema_version: 1,
                aggregate_type: AggregateType::System,
                aggregate_id: "system".to_owned(),
                actor_type: ActorType::Human,
                actor_id: actor.to_owned(),
                correlation_id: "system".to_owned(),
                causation_ref: None,
                payload: serde_json::json!({
                    "tier": tier,
                    "override": "allow_uncontained_repo",
                    "repo": repo_disp,
                    "granted_by": actor,
                    "warning": DEV_TIER_WARNING,
                }),
            })?;
            Ok(())
        })?;
        Ok(())
    }

    /// A stable V0 project identity for memory scoping (Blocker 7): a hash of
    /// the canonical Git common directory plus the repository's root commit.
    /// Relative/absolute/case-variant paths to the same repo resolve alike; a
    /// reinitialized repo (new root commit) gets a new identity.
    pub fn project_identity(&self, repo_path: &str) -> Result<String, RuntimeError> {
        let ws = Workspace::open(Path::new(repo_path))?;
        let fp = ws.project_fingerprint()?;
        let digest = wepld_artifacts::hash_hex(
            format!("{}\u{0}{}", fp.common_dir, fp.root_commit).as_bytes(),
        );
        Ok(format!("proj_{}", &digest[..16]))
    }

    /// Whether a repository would be permitted to run under the current DEV-tier
    /// configuration (Manual mode). Public for governance tests and inspection.
    pub fn is_repo_allowed_under_dev(&self, repo: &str) -> bool {
        self.dev_tier_gate(repo, AutonomyMode::Manual).is_ok()
    }

    /// Enforce DEV-tier caps for a mission about to run: Manual mode only, and
    /// the repository must be within the fixtures root or explicitly overridden.
    /// Returns the rejection reason if the mission may not run.
    pub(crate) fn dev_tier_gate(&self, repo: &str, mode: AutonomyMode) -> Result<(), String> {
        if mode == AutonomyMode::BoundedAuto {
            return Err(format!(
                "DEV tier permits Manual mode only; Bounded-Auto is refused. {DEV_TIER_WARNING}"
            ));
        }
        // A repository that cannot be canonicalized cannot be authorized — no
        // unresolved-string fallback.
        let Some(canon) = canonical_scope_path(Path::new(repo)) else {
            return Err(format!(
                "DEV tier: repository {repo:?} cannot be canonicalized (does it exist?). \
                 {DEV_TIER_WARNING}"
            ));
        };
        if let Some(root) = &self.dev.fixtures_root {
            if canon.starts_with(root) {
                return Ok(());
            }
        }
        if let Some((allowed, _actor)) = &self.dev.allow_uncontained {
            if &canon == allowed {
                return Ok(());
            }
        }
        Err(format!(
            "DEV tier: repository {} is not within the fixtures root and has no explicit \
             override. Re-run with --i-understand-dev-tier to authorize this throwaway repo. \
             {DEV_TIER_WARNING}",
            canon.display()
        ))
    }

    /// Centralized DEV preflight (Blocker 5): authorize a mission's repository +
    /// autonomy mode before any worker may spawn. Used before mission creation,
    /// planning, running, and — for defense in depth — at the worker-spawn
    /// boundary itself. Returns the rejection reason if denied.
    pub(crate) fn dev_preflight(&self, mission_id: &str) -> Result<(), String> {
        let brief = self
            .store
            .mission_brief(mission_id)
            .ok()
            .flatten()
            .ok_or_else(|| format!("no brief for mission {mission_id}"))?;
        let repo = brief["scope"]["repo"].as_str().unwrap_or("");
        let mode = serde_json::from_value::<AutonomyMode>(brief["autonomy_mode"].clone())
            .unwrap_or(AutonomyMode::Manual);
        self.dev_tier_gate(repo, mode)
    }

    /// DEV preflight for an incoming brief that is not yet durable (mission
    /// creation), so an unauthorized repository is refused before any record.
    pub(crate) fn dev_preflight_brief(&self, brief: &serde_json::Value) -> Result<(), String> {
        let repo = brief["scope"]["repo"].as_str().unwrap_or("");
        let mode = serde_json::from_value::<AutonomyMode>(brief["autonomy_mode"].clone())
            .unwrap_or(AutonomyMode::Manual);
        self.dev_tier_gate(repo, mode)
    }

    /// The command pipeline (v2-02 §2): idempotency, validation, transition —
    /// command record and effects committed in one transaction.
    pub fn submit(&mut self, cmd: &Command) -> Result<CommandOutcome, RuntimeError> {
        // 1. Idempotency: a known command_id returns its stored outcome.
        if let Some((stored_hash, outcome_json)) = self.store.command_record(&cmd.command_id)? {
            if stored_hash != wepld_ledger::payload_hash(&cmd.payload) {
                return Ok(CommandOutcome::Rejected {
                    reason: "command_id reused with a different payload".to_owned(),
                });
            }
            return Ok(serde_json::from_str(&outcome_json)?);
        }
        // 2. Authorization: MVP has one local principal; checked, not assumed.
        if !is_authenticated_principal(&cmd.actor) {
            return Ok(CommandOutcome::Rejected {
                reason: format!("unknown principal: {}", cmd.actor),
            });
        }
        // 3–4. Validate + apply per command type.
        match cmd.command_type.as_str() {
            "create_mission" => {
                // DEV preflight before any durable mission exists: an arbitrary
                // (non-fixture, non-overridden) repository is refused up front.
                if let Err(reason) = self.dev_preflight_brief(&cmd.payload) {
                    return Ok(CommandOutcome::Rejected { reason });
                }
                commands::create_mission(&mut self.store, cmd)
            }
            other => Ok(CommandOutcome::Rejected {
                reason: format!("unknown command type: {other}"),
            }),
        }
    }

    pub fn timeline(&self, mission_id: &str) -> Result<Vec<LedgerEntry>, RuntimeError> {
        Ok(self.store.entries_for(mission_id)?)
    }

    pub fn all_entries(&self) -> Result<Vec<LedgerEntry>, RuntimeError> {
        Ok(self.store.all_entries()?)
    }

    pub fn verify(&self) -> Result<ChainReport, RuntimeError> {
        Ok(self.store.verify_chain()?)
    }

    pub fn mission_row(&self, mission_id: &str) -> Result<Option<(String, String)>, RuntimeError> {
        Ok(self.store.mission_row(mission_id)?)
    }

    /// The repository path a mission operates on (from its stored brief).
    pub fn mission_repo(&self, mission_id: &str) -> Result<Option<String>, RuntimeError> {
        Ok(self
            .store
            .mission_brief(mission_id)?
            .and_then(|b| b["scope"]["repo"].as_str().map(str::to_owned)))
    }

    pub fn attempt_state(&self, attempt_id: &str) -> Result<Option<String>, RuntimeError> {
        Ok(self.store.attempt_state(attempt_id)?)
    }

    pub fn brain_invocations(
        &self,
        attempt_id: &str,
    ) -> Result<Vec<wepld_ledger::BrainInvocationRow>, RuntimeError> {
        Ok(self.store.brain_invocations(attempt_id)?)
    }

    pub fn artifact(&self, hash: &str) -> Result<Vec<u8>, RuntimeError> {
        Ok(self.cas.get(hash)?)
    }

    pub fn tasks(&self, mission_id: &str) -> Result<Vec<TaskRow>, RuntimeError> {
        Ok(self.store.tasks_for_mission(mission_id)?)
    }

    pub(crate) fn store_mut(&mut self) -> &mut LedgerStore {
        &mut self.store
    }

    pub(crate) fn cas(&self) -> &wepld_artifacts::Cas {
        &self.cas
    }

    pub(crate) fn gateway(&self) -> &wepld_providers::Gateway {
        &self.gateway
    }

    pub(crate) fn worker_cmd(&self) -> Vec<String> {
        self.worker_cmd.clone()
    }

    pub(crate) fn root(&self) -> &Path {
        &self.root
    }
}
