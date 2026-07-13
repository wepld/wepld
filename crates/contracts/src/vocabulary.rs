//! The closed event vocabulary, revision 2 (v2-07 §5, amended by v2-17).
//!
//! Adding a variant is a contract change: update [`EventType::code`],
//! [`EventType::ALL`], and the lock test together, bump the contracts
//! version, and cite the authorizing architecture document in the PR.
//! The `code()` match makes forgetting impossible: a new variant fails to
//! compile until it is named everywhere.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Every fact the ledger may record. Names are past-tense facts; a
/// `…Requested` entry never implies the effect occurred.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum EventType {
    // ── Base vocabulary (v2-07 §5, rev 1) ──────────────────────────────
    MissionCreated,
    MissionRevised,
    PlanProposed,
    PlanApproved,
    PlanRejected,
    TaskStarted,
    AttemptSpawned,
    PhaseStarted,
    BrainInvoked,
    ArtifactRecorded,
    EnvelopeExtensionRequested,
    EnvelopeExtensionResolved,
    EscalationRaised,
    DecisionRequested,
    DecisionResolved,
    GateEvaluated,
    PhaseCompleted,
    AttemptCompleted,
    AttemptUncertain,
    RecoverySnapshotRecorded,
    RecoveryPerformed,
    TaskCompleted,
    MissionWaiting,
    CompletionProposed,
    MissionAccepted,
    MissionReturned,
    MissionCancelled,
    MissionFailed,
    SandboxTierDetected,
    RedactionApplied,
    MessageSent,
    BudgetThresholdCrossed,
    // ── Chronicle MVP additions (rev 2, ADR-0011) ──────────────────────
    WorkspaceSnapshotRecorded,
    MissionForked,
    DecisionRevised,
    MissionSuperseded,
    // ── Chronicle V1 additions (rev 2, ADR-0011) ───────────────────────
    InsightRecorded,
    AnnotationRecorded,
    ReplayExported,
    // ── Engineering Specification System additions (rev 3) ─────────────
    SpecificationCreated,
    SpecificationRevised,
    SpecClarified,
    SpecResearched,
    SpecPlanGenerated,
    SpecTasksGenerated,
    SpecValidated,
    SpecReviewed,
    MissionDerivedFromSpec,
    SpecRevisionProposed,
    SpecRevisionResolved,
    SpecLinked,
    SpecStatusChanged,
}

impl EventType {
    /// Every member of the vocabulary, in normative order.
    pub const ALL: [EventType; 52] = [
        EventType::MissionCreated,
        EventType::MissionRevised,
        EventType::PlanProposed,
        EventType::PlanApproved,
        EventType::PlanRejected,
        EventType::TaskStarted,
        EventType::AttemptSpawned,
        EventType::PhaseStarted,
        EventType::BrainInvoked,
        EventType::ArtifactRecorded,
        EventType::EnvelopeExtensionRequested,
        EventType::EnvelopeExtensionResolved,
        EventType::EscalationRaised,
        EventType::DecisionRequested,
        EventType::DecisionResolved,
        EventType::GateEvaluated,
        EventType::PhaseCompleted,
        EventType::AttemptCompleted,
        EventType::AttemptUncertain,
        EventType::RecoverySnapshotRecorded,
        EventType::RecoveryPerformed,
        EventType::TaskCompleted,
        EventType::MissionWaiting,
        EventType::CompletionProposed,
        EventType::MissionAccepted,
        EventType::MissionReturned,
        EventType::MissionCancelled,
        EventType::MissionFailed,
        EventType::SandboxTierDetected,
        EventType::RedactionApplied,
        EventType::MessageSent,
        EventType::BudgetThresholdCrossed,
        EventType::WorkspaceSnapshotRecorded,
        EventType::MissionForked,
        EventType::DecisionRevised,
        EventType::MissionSuperseded,
        EventType::InsightRecorded,
        EventType::AnnotationRecorded,
        EventType::ReplayExported,
        EventType::SpecificationCreated,
        EventType::SpecificationRevised,
        EventType::SpecClarified,
        EventType::SpecResearched,
        EventType::SpecPlanGenerated,
        EventType::SpecTasksGenerated,
        EventType::SpecValidated,
        EventType::SpecReviewed,
        EventType::MissionDerivedFromSpec,
        EventType::SpecRevisionProposed,
        EventType::SpecRevisionResolved,
        EventType::SpecLinked,
        EventType::SpecStatusChanged,
    ];

    /// The wire spelling of the fact. Exhaustive by construction: a new
    /// variant does not compile until it is spelled here.
    pub const fn code(self) -> &'static str {
        match self {
            EventType::MissionCreated => "MissionCreated",
            EventType::MissionRevised => "MissionRevised",
            EventType::PlanProposed => "PlanProposed",
            EventType::PlanApproved => "PlanApproved",
            EventType::PlanRejected => "PlanRejected",
            EventType::TaskStarted => "TaskStarted",
            EventType::AttemptSpawned => "AttemptSpawned",
            EventType::PhaseStarted => "PhaseStarted",
            EventType::BrainInvoked => "BrainInvoked",
            EventType::ArtifactRecorded => "ArtifactRecorded",
            EventType::EnvelopeExtensionRequested => "EnvelopeExtensionRequested",
            EventType::EnvelopeExtensionResolved => "EnvelopeExtensionResolved",
            EventType::EscalationRaised => "EscalationRaised",
            EventType::DecisionRequested => "DecisionRequested",
            EventType::DecisionResolved => "DecisionResolved",
            EventType::GateEvaluated => "GateEvaluated",
            EventType::PhaseCompleted => "PhaseCompleted",
            EventType::AttemptCompleted => "AttemptCompleted",
            EventType::AttemptUncertain => "AttemptUncertain",
            EventType::RecoverySnapshotRecorded => "RecoverySnapshotRecorded",
            EventType::RecoveryPerformed => "RecoveryPerformed",
            EventType::TaskCompleted => "TaskCompleted",
            EventType::MissionWaiting => "MissionWaiting",
            EventType::CompletionProposed => "CompletionProposed",
            EventType::MissionAccepted => "MissionAccepted",
            EventType::MissionReturned => "MissionReturned",
            EventType::MissionCancelled => "MissionCancelled",
            EventType::MissionFailed => "MissionFailed",
            EventType::SandboxTierDetected => "SandboxTierDetected",
            EventType::RedactionApplied => "RedactionApplied",
            EventType::MessageSent => "MessageSent",
            EventType::BudgetThresholdCrossed => "BudgetThresholdCrossed",
            EventType::WorkspaceSnapshotRecorded => "WorkspaceSnapshotRecorded",
            EventType::MissionForked => "MissionForked",
            EventType::DecisionRevised => "DecisionRevised",
            EventType::MissionSuperseded => "MissionSuperseded",
            EventType::InsightRecorded => "InsightRecorded",
            EventType::AnnotationRecorded => "AnnotationRecorded",
            EventType::ReplayExported => "ReplayExported",
            EventType::SpecificationCreated => "SpecificationCreated",
            EventType::SpecificationRevised => "SpecificationRevised",
            EventType::SpecClarified => "SpecClarified",
            EventType::SpecResearched => "SpecResearched",
            EventType::SpecPlanGenerated => "SpecPlanGenerated",
            EventType::SpecTasksGenerated => "SpecTasksGenerated",
            EventType::SpecValidated => "SpecValidated",
            EventType::SpecReviewed => "SpecReviewed",
            EventType::MissionDerivedFromSpec => "MissionDerivedFromSpec",
            EventType::SpecRevisionProposed => "SpecRevisionProposed",
            EventType::SpecRevisionResolved => "SpecRevisionResolved",
            EventType::SpecLinked => "SpecLinked",
            EventType::SpecStatusChanged => "SpecStatusChanged",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXPECTED: [&str; 52] = [
        "MissionCreated",
        "MissionRevised",
        "PlanProposed",
        "PlanApproved",
        "PlanRejected",
        "TaskStarted",
        "AttemptSpawned",
        "PhaseStarted",
        "BrainInvoked",
        "ArtifactRecorded",
        "EnvelopeExtensionRequested",
        "EnvelopeExtensionResolved",
        "EscalationRaised",
        "DecisionRequested",
        "DecisionResolved",
        "GateEvaluated",
        "PhaseCompleted",
        "AttemptCompleted",
        "AttemptUncertain",
        "RecoverySnapshotRecorded",
        "RecoveryPerformed",
        "TaskCompleted",
        "MissionWaiting",
        "CompletionProposed",
        "MissionAccepted",
        "MissionReturned",
        "MissionCancelled",
        "MissionFailed",
        "SandboxTierDetected",
        "RedactionApplied",
        "MessageSent",
        "BudgetThresholdCrossed",
        "WorkspaceSnapshotRecorded",
        "MissionForked",
        "DecisionRevised",
        "MissionSuperseded",
        "InsightRecorded",
        "AnnotationRecorded",
        "ReplayExported",
        "SpecificationCreated",
        "SpecificationRevised",
        "SpecClarified",
        "SpecResearched",
        "SpecPlanGenerated",
        "SpecTasksGenerated",
        "SpecValidated",
        "SpecReviewed",
        "MissionDerivedFromSpec",
        "SpecRevisionProposed",
        "SpecRevisionResolved",
        "SpecLinked",
        "SpecStatusChanged",
    ];

    #[test]
    fn vocabulary_is_locked_at_revision_3() {
        assert_eq!(crate::EVENT_VOCABULARY_REVISION, 3);
        assert_eq!(EventType::ALL.len(), EXPECTED.len());
        for (ev, name) in EventType::ALL.iter().zip(EXPECTED.iter()) {
            assert_eq!(ev.code(), *name, "code() disagrees with the lock list");
            assert_eq!(
                serde_json::to_value(ev).unwrap(),
                serde_json::json!(name),
                "serde spelling disagrees with the lock list"
            );
        }
    }
}
