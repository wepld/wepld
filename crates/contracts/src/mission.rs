//! Mission Contract (v2-07 §1). The only entry point for work — a structured
//! brief, never free chat.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MissionBrief {
    pub schema_version: u32,
    pub mission_id: String,
    pub title: String,
    pub outcome: String,
    pub scope: Scope,
    pub acceptance_criteria: Vec<AcceptanceCriterion>,
    pub gates_required: Vec<String>,
    pub autonomy_mode: AutonomyMode,
    pub envelope_declared: DeclaredEnvelope,
    pub budget: Budget,
    pub classification: Classification,
    pub owner: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Scope {
    pub repo: String,
    pub base_branch: String,
    pub paths: Vec<String>,
    pub forbidden_paths: Vec<String>,
}

/// A criterion with no machine-checkable `verify` binding is rejected at
/// submission (v2-07 §1 rule) — this forces testable missions.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AcceptanceCriterion {
    pub id: String,
    pub text: String,
    pub verify: String,
}

/// ADR-0009: two modes in the MVP, both presets over one envelope + hard-gate
/// mechanism. Future presets extend this enum additively.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AutonomyMode {
    Manual,
    BoundedAuto,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DeclaredEnvelope {
    pub network: String,
    pub dependency_install: String,
    pub secrets: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Budget {
    pub max_cost_usd: f64,
    pub max_wall_minutes: u32,
    /// Human attention is a metered resource (v2-10): one delivery = one interrupt.
    pub max_interrupts: u32,
}

/// Data classification taxonomy (v2-16 of Architecture v1, preserved in v2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum Classification {
    Public,
    Internal,
    Confidential,
    Restricted,
    Secret,
}
