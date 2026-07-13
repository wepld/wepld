//! wepld-runtime — the Core (v2-02). Day-3 scope: the command pipeline
//! (idempotency → authorization stub → validation → transition, one
//! transaction) with `create_mission`, plus read ports for timeline and
//! chain verification. The phase engine arrives on Day 5+.
//!
//! Boundary rule (IMPL-02): this crate's transition code is the only holder
//! of `wepld_ledger::Tx` in the workspace.

mod commands;
mod phase;

pub use commands::command_id_for;
pub use phase::{PhaseOutcome, PhaseSpec};

use std::path::Path;
use wepld_contracts::command::{Command, CommandOutcome};
use wepld_contracts::envelope::SandboxTier;
use wepld_contracts::ledger::{ActorType, AggregateType, LedgerEntry};
use wepld_contracts::vocabulary::EventType;
use wepld_ledger::{ChainReport, LedgerError, LedgerStore, NewEntry};

#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {
    #[error(transparent)]
    Ledger(#[from] LedgerError),
    #[error("serialization error: {0}")]
    Serde(#[from] serde_json::Error),
}

pub struct Core {
    store: LedgerStore,
}

impl Core {
    /// Open (or create) the operational store and ensure the tier fact exists.
    /// Until M4 implements real tier detection, the honest tier is `DEV`
    /// (IADR-0003): no isolation, disclosed, Manual-mode caps.
    pub fn open(dir: &Path) -> Result<Self, RuntimeError> {
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
        Ok(Self { store })
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

    pub(crate) fn store_mut(&mut self) -> &mut LedgerStore {
        &mut self.store
    }
}
