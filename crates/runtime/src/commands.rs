//! Command handlers. Each handler validates against the contracts, then
//! applies state mutation + ledger append + command record in ONE transaction.

use wepld_contracts::command::{Command, CommandOutcome};
use wepld_contracts::ledger::{ActorType, AggregateType};
use wepld_contracts::mission::MissionBrief;
use wepld_contracts::validation::{validate_git_ref_name, validate_identifier};
use wepld_contracts::vocabulary::EventType;
use wepld_ledger::{payload_hash, LedgerStore, NewEntry};

use crate::RuntimeError;

/// Deterministic command id from intent + payload: resubmitting the same
/// brief is the same command (idempotent by construction).
pub fn command_id_for(command_type: &str, payload: &serde_json::Value) -> String {
    let h = payload_hash(payload);
    format!("cmd_{command_type}_{}", &h[..16])
}

pub(crate) fn create_mission(
    store: &mut LedgerStore,
    cmd: &Command,
) -> Result<CommandOutcome, RuntimeError> {
    // Validation (v2-07 §1 rules).
    let brief: MissionBrief = match serde_json::from_value(cmd.payload.clone()) {
        Ok(b) => b,
        Err(e) => {
            return record_rejection(store, cmd, format!("brief does not match schema: {e}"));
        }
    };
    if brief
        .acceptance_criteria
        .iter()
        .any(|c| c.verify.trim().is_empty())
    {
        return record_rejection(
            store,
            cmd,
            "every acceptance criterion must declare a verify method".to_owned(),
        );
    }
    if brief.acceptance_criteria.is_empty() {
        return record_rejection(store, cmd, "at least one acceptance criterion".to_owned());
    }
    // Untrusted identifiers must never reach a filesystem path or Git ref as
    // syntax: validate before persistence, as a deterministic recorded rejection.
    if let Err(e) = validate_identifier("mission_id", &brief.mission_id) {
        return record_rejection(store, cmd, format!("invalid mission_id: {e}"));
    }
    if let Err(e) = validate_git_ref_name("base_branch", &brief.scope.base_branch) {
        return record_rejection(store, cmd, format!("invalid base_branch: {e}"));
    }
    if store.mission_row(&brief.mission_id)?.is_some() {
        return record_rejection(store, cmd, format!("mission exists: {}", brief.mission_id));
    }

    // Transition: mission row + MissionCreated fact + command record, one tx.
    let outcome = CommandOutcome::Accepted {
        detail: serde_json::json!({ "mission_id": brief.mission_id, "state": "draft" }),
    };
    let outcome_json = serde_json::to_string(&outcome)?;
    let hash = payload_hash(&cmd.payload);
    store.transact(|tx| {
        tx.insert_mission(&brief.mission_id, &brief.title, "draft", &cmd.payload)?;
        tx.append(&NewEntry {
            entry_type: EventType::MissionCreated,
            schema_version: 1,
            aggregate_type: AggregateType::Mission,
            aggregate_id: brief.mission_id.clone(),
            actor_type: ActorType::Human,
            actor_id: cmd.actor.clone(),
            correlation_id: brief.mission_id.clone(),
            causation_ref: Some(cmd.command_id.clone()),
            payload: serde_json::json!({
                "title": brief.title,
                "autonomy_mode": brief.autonomy_mode,
                "classification": brief.classification,
                "budget": brief.budget,
            }),
        })?;
        tx.record_command(
            &cmd.command_id,
            &cmd.command_type,
            &cmd.actor,
            &hash,
            &outcome_json,
        )?;
        Ok(())
    })?;
    Ok(outcome)
}

/// Rejections are recorded in the commands table (idempotent replays return
/// the same rejection) but produce no ledger noise (v2-02 §2).
fn record_rejection(
    store: &mut LedgerStore,
    cmd: &Command,
    reason: String,
) -> Result<CommandOutcome, RuntimeError> {
    let outcome = CommandOutcome::Rejected { reason };
    let outcome_json = serde_json::to_string(&outcome)?;
    let hash = payload_hash(&cmd.payload);
    store.transact(|tx| {
        tx.record_command(
            &cmd.command_id,
            &cmd.command_type,
            &cmd.actor,
            &hash,
            &outcome_json,
        )?;
        Ok(())
    })?;
    Ok(outcome)
}
