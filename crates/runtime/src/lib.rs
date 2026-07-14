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
pub use orchestration::{builder_pack, planner_pack};
pub use phase::{PhaseOutcome, PhaseRun, PhaseSpec};
pub use recipe::{specify_memory_entries, BuildFeatureReport, RecipeOutcome};
pub use report::EngineeringReport;

use std::path::Path;
use wepld_contracts::command::{Command, CommandOutcome};
use wepld_contracts::envelope::SandboxTier;
use wepld_contracts::ledger::{ActorType, AggregateType, LedgerEntry};
use wepld_contracts::vocabulary::EventType;
use wepld_ledger::{ChainReport, LedgerError, LedgerStore, NewEntry, TaskRow};

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
        })
    }

    /// Point the Core at a specific WWP worker binary (tests and the CLI,
    /// which locate `hermes` next to `wepld`).
    pub fn set_worker_cmd(&mut self, cmd: Vec<String>) {
        self.worker_cmd = cmd;
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
        if cmd.actor != "principal_local" {
            return Ok(CommandOutcome::Rejected {
                reason: format!("unknown principal: {}", cmd.actor),
            });
        }
        // 3–4. Validate + apply per command type.
        match cmd.command_type.as_str() {
            "create_mission" => commands::create_mission(&mut self.store, cmd),
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
