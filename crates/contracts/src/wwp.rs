//! WePLD Worker Protocol (WWP v0) message shapes — Sprint-1 subset of
//! v2-07 §2. Design rules the vocabulary itself enforces: no message
//! addresses a human; no message mutates mission state.
//! Framing (JSON-RPC 2.0 over stdio) lives in `wepld-wwp`, not here.

use crate::envelope::Envelope;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "method", content = "params")]
pub enum WwpMessage {
    #[serde(rename = "attempt.start")]
    AttemptStart(Box<AttemptStart>),
    #[serde(rename = "heartbeat")]
    Heartbeat(Heartbeat),
    #[serde(rename = "brain.request")]
    BrainRequest(BrainRequest),
    #[serde(rename = "artifact.put")]
    ArtifactPut(ArtifactPut),
    #[serde(rename = "phase.result")]
    PhaseResult(PhaseResult),
    #[serde(rename = "attempt.cancel")]
    AttemptCancel(AttemptCancel),
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AttemptStart {
    pub attempt_id: String,
    pub task_id: String,
    pub phase: String,
    pub role_profile: RoleProfile,
    pub context_pack_ref: ArtifactRef,
    pub envelope: Envelope,
    pub gates: Vec<String>,
    pub budget: PhaseBudget,
    pub idempotency_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RoleProfile {
    pub name: String,
    pub version: u32,
    pub brain_profile: String,
    pub skills: Vec<SkillPin>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SkillPin {
    pub name: String,
    pub version: String,
    pub hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ArtifactRef {
    pub artifact: String,
    pub hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PhaseBudget {
    pub max_brain_calls: u32,
    pub max_wall_minutes: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Heartbeat {
    pub attempt_id: String,
    pub progress: String,
}

/// Routed through the Core's Brain Gateway — workers never hold provider
/// credentials or choose vendors (v2-03).
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrainRequest {
    pub attempt_id: String,
    pub intent: String,
    pub pack_ref: ArtifactRef,
    pub output_schema_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_hint: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ArtifactPut {
    pub attempt_id: String,
    pub kind: String,
    pub media_type: String,
    pub content_b64: String,
    pub meta: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PhaseResult {
    pub attempt_id: String,
    pub status: PhaseStatus,
    pub outputs: Vec<OutputRef>,
    pub evidence: Vec<OutputRef>,
    /// Schema-enforced summary (`phase_summary.v1`) — feeds T1/T4 compression.
    pub summary: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_hint: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PhaseStatus {
    Succeeded,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct OutputRef {
    pub artifact: String,
    pub kind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AttemptCancel {
    pub attempt_id: String,
}
