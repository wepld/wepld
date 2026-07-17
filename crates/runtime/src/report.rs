//! Engineering Completion Report — the moat, not the model. Derived entirely
//! from the ledger's evidence: gates, acceptance criteria, chain integrity,
//! recovery cleanliness, and replayability. **Confidence is derived, never
//! guessed** (the founder's evidence-first rule): it is the fraction of the
//! checkable engineering evidence that actually passed. No brain call, no
//! network — this is valuable even if every LLM were perfect.

use crate::{Core, RuntimeError};
use wepld_contracts::vocabulary::EventType;

#[derive(Debug, Clone)]
pub struct EngineeringReport {
    pub mission_id: String,
    pub title: String,
    pub state: String,
    pub spec_id: Option<String>,
    /// (gate name, passed) in evaluation order.
    pub gates: Vec<(String, bool)>,
    /// (criterion id, met) from the completion proposal.
    pub criteria: Vec<(String, bool)>,
    pub chain_verified: bool,
    pub uncertain_attempts: usize,
    pub evidence_artifacts: usize,
    pub brain_calls: usize,
    pub replay_available: bool,
    /// Evidence-derived confidence in [0,1] — see `derive_confidence`.
    pub confidence: f64,
}

impl EngineeringReport {
    pub fn gates_passed(&self) -> usize {
        self.gates.iter().filter(|(_, p)| *p).count()
    }
    pub fn criteria_met(&self) -> usize {
        self.criteria.iter().filter(|(_, m)| *m).count()
    }
}

impl Core {
    /// Build the evidence-derived engineering report for a mission. Pure
    /// derivation from durable facts — no reasoning, no guessing.
    pub fn engineering_report(&self, mission_id: &str) -> Result<EngineeringReport, RuntimeError> {
        let entries = self.timeline(mission_id)?;
        let (title, state) = self
            .mission_row(mission_id)?
            .unwrap_or_else(|| (String::new(), "unknown".to_owned()));

        let mut gates: Vec<(String, bool)> = Vec::new();
        let mut criteria: Vec<(String, bool)> = Vec::new();
        let mut spec_id = None;
        let mut uncertain_attempts = 0usize;
        let mut evidence_artifacts = 0usize;
        let mut brain_calls = 0usize;

        for e in &entries {
            match e.entry_type {
                EventType::GateEvaluated => {
                    let name = e.payload_json["gate"].as_str().unwrap_or("?").to_owned();
                    let passed = e.payload_json["status"] == serde_json::json!("passed");
                    gates.push((name, passed));
                }
                EventType::CompletionProposed => {
                    if let Some(arr) = e.payload_json["criteria"].as_array() {
                        criteria = arr
                            .iter()
                            .map(|c| {
                                (
                                    c["id"].as_str().unwrap_or("?").to_owned(),
                                    c["met"] == serde_json::json!(true),
                                )
                            })
                            .collect();
                    }
                }
                EventType::MissionDerivedFromSpec => {
                    spec_id = e.payload_json["sources"][0]["spec_id"]
                        .as_str()
                        .map(str::to_owned);
                }
                EventType::AttemptUncertain => uncertain_attempts += 1,
                EventType::ArtifactRecorded => evidence_artifacts += 1,
                EventType::BrainInvoked => brain_calls += 1,
                _ => {}
            }
        }

        let chain_verified = self.verify()?.is_valid();
        let replay_available = !entries.is_empty();
        let confidence = derive_confidence(&gates, &criteria, chain_verified, uncertain_attempts);

        Ok(EngineeringReport {
            mission_id: mission_id.to_owned(),
            title,
            state,
            spec_id,
            gates,
            criteria,
            chain_verified,
            uncertain_attempts,
            evidence_artifacts,
            brain_calls,
            replay_available,
            confidence,
        })
    }
}

/// Confidence = 40% gate pass-rate + 40% criteria met-rate + 20% integrity
/// (chain verified and no uncertain attempts). Every input is a recorded fact,
/// so the number is explainable and reproducible — never invented.
fn derive_confidence(
    gates: &[(String, bool)],
    criteria: &[(String, bool)],
    chain_verified: bool,
    uncertain_attempts: usize,
) -> f64 {
    let ratio = |v: &[(String, bool)]| -> f64 {
        if v.is_empty() {
            0.0
        } else {
            v.iter().filter(|(_, ok)| *ok).count() as f64 / v.len() as f64
        }
    };
    let integrity = if chain_verified && uncertain_attempts == 0 {
        1.0
    } else {
        0.5
    };
    0.4 * ratio(gates) + 0.4 * ratio(criteria) + 0.2 * integrity
}

#[cfg(test)]
mod tests {
    use super::derive_confidence;

    #[test]
    fn all_evidence_green_is_full_confidence() {
        let gates = vec![("build".to_owned(), true), ("test".to_owned(), true)];
        let criteria = vec![("AC1".to_owned(), true)];
        assert_eq!(derive_confidence(&gates, &criteria, true, 0), 1.0);
    }

    #[test]
    fn a_failed_gate_lowers_confidence_proportionally() {
        let gates = vec![("build".to_owned(), true), ("test".to_owned(), false)];
        let criteria = vec![("AC1".to_owned(), true)];
        // 0.4*0.5 + 0.4*1.0 + 0.2*1.0 = 0.8
        assert!((derive_confidence(&gates, &criteria, true, 0) - 0.8).abs() < 1e-9);
    }

    #[test]
    fn a_broken_chain_or_uncertain_attempt_reduces_integrity() {
        let gates = vec![("build".to_owned(), true)];
        let criteria = vec![("AC1".to_owned(), true)];
        // integrity 0.5 → 0.4 + 0.4 + 0.1 = 0.9
        assert!((derive_confidence(&gates, &criteria, false, 0) - 0.9).abs() < 1e-9);
        assert!((derive_confidence(&gates, &criteria, true, 1) - 0.9).abs() < 1e-9);
    }

    #[test]
    fn no_evidence_is_low_confidence() {
        assert!(derive_confidence(&[], &[], true, 0) <= 0.2);
    }
}
