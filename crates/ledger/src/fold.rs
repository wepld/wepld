//! The fold reducer v0: derive a mission's current state from its ledger
//! entries. This is THE reducer (v2-06): the same function serves the
//! consistency check now, Chronicle's `state_at` at M6, and the event-sourcing
//! promotion path if it is ever earned. Pure — no I/O.

use wepld_contracts::ledger::LedgerEntry;
use wepld_contracts::vocabulary::EventType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoldedMission {
    pub mission_id: String,
    pub title: String,
    pub state: String,
}

/// Fold a mission's entries (ascending seq) into its current state.
/// Returns `None` when no `MissionCreated` fact exists.
pub fn fold_mission(entries: &[LedgerEntry]) -> Option<FoldedMission> {
    let mut out: Option<FoldedMission> = None;
    for e in entries {
        match e.entry_type {
            EventType::MissionCreated => {
                out = Some(FoldedMission {
                    mission_id: e.aggregate_id.clone(),
                    title: e.payload_json["title"].as_str().unwrap_or("").to_owned(),
                    state: "draft".to_owned(),
                });
            }
            EventType::PlanProposed => set_state(&mut out, "plan_review"),
            EventType::PlanApproved => set_state(&mut out, "running"),
            EventType::CompletionProposed => set_state(&mut out, "completion_proposed"),
            EventType::MissionAccepted => set_state(&mut out, "accepted"),
            EventType::MissionCancelled => set_state(&mut out, "cancelled"),
            EventType::MissionFailed => set_state(&mut out, "failed"),
            _ => {}
        }
    }
    out
}

fn set_state(m: &mut Option<FoldedMission>, state: &str) {
    if let Some(m) = m.as_mut() {
        m.state = state.to_owned();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_entries_fold_to_none() {
        assert!(fold_mission(&[]).is_none());
    }
}
