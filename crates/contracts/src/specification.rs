//! Engineering Specification contract (rev 3). The Specification is a
//! canonical, first-class engineering object — markdown is only one
//! serialization. WePLD components interact with these types, never with
//! markdown. See docs/impl/M1_SPEC_KIT_INTEGRATION.md.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// The living identity of a specification (a state-table row).
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Specification {
    pub schema_version: u32,
    pub spec_id: String,
    /// Sequential number (Spec Kit convention: NNN).
    pub number: u32,
    pub slug: String,
    pub status: SpecStatus,
    pub author: String,
    pub current_version: u32,
    pub created_at: String,
    pub updated_at: String,
}

/// A specification never ends: it evolves and is superseded, never deleted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SpecStatus {
    Draft,
    Clarifying,
    Researching,
    Planned,
    Tasked,
    Active,
    Revising,
    Superseded,
    Archived,
}

/// An immutable snapshot of a specification at one version (append-only).
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SpecVersion {
    pub spec_id: String,
    pub version: u32,
    pub revision: u32,
    pub timestamp: String,
    pub author: String,
    /// CAS hash of the canonical [`SpecificationDocument`] JSON — the truth.
    pub document_hash: String,
    /// CAS hashes of derived representations (markdown renders, design docs).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub artifact_hashes: Vec<ArtifactHash>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supersedes: Option<u32>,
    pub reason: String,
    pub quality: SpecQuality,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ArtifactHash {
    /// e.g. "markdown", "plan", "tasks", "research", "data_model".
    pub kind: String,
    pub hash: String,
}

/// The canonical structured content — serialization-independent. Markdown is
/// parsed into this and rendered from it; this object is what everything else
/// reasons over.
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct SpecificationDocument {
    pub overview: String,
    #[serde(default)]
    pub user_stories: Vec<String>,
    #[serde(default)]
    pub functional_requirements: Vec<String>,
    #[serde(default)]
    pub acceptance_criteria: Vec<SpecAcceptanceCriterion>,
    #[serde(default)]
    pub non_functional: Vec<String>,
    #[serde(default)]
    pub edge_cases: Vec<String>,
    #[serde(default)]
    pub constraints: Vec<String>,
    #[serde(default)]
    pub dependencies: Vec<String>,
    #[serde(default)]
    pub required_skills: Vec<String>,
    #[serde(default)]
    pub success_metrics: Vec<String>,
    #[serde(default)]
    pub clarifications: Vec<Clarification>,
    /// How each acceptance gate is verified: gate name → shell command. Makes
    /// an engineering specification executable — the baseline gates a derived
    /// mission runs. Ordered (BTreeMap) for deterministic rendering/hashing.
    #[serde(default)]
    pub verification: BTreeMap<String, String>,
    /// Unresolved `[NEEDS CLARIFICATION]` markers; must be empty before planning.
    #[serde(default)]
    pub open_questions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SpecAcceptanceCriterion {
    pub id: String,
    pub text: String,
    /// Machine-checkable verification (a gate name / `gate:<name>`); a criterion
    /// without one fails validation.
    pub verify: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Clarification {
    pub question: String,
    pub answer: String,
}

/// A first-class typed edge in the Engineering Graph.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SpecLink {
    pub spec_id: String,
    pub kind: SpecLinkKind,
    pub target_ref: String,
    pub relation: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SpecLinkKind {
    Mission,
    Adr,
    Knowledge,
    Skill,
    Spec,
    Context,
    Recipe,
    Pack,
}

/// A Specification Intelligence finding — always evidence-based, never invented.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SpecFinding {
    pub spec_id: String,
    pub version: u32,
    pub class: FindingClass,
    pub severity: Severity,
    pub evidence_refs: Vec<String>,
    pub disposition: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum FindingClass {
    MissingAcceptanceCriteria,
    HiddenAssumption,
    ArchitectureContradiction,
    DependencyConflict,
    MissingRollback,
    MissingBenchmark,
    MissingSecurity,
    MissingMigration,
    MissingTesting,
    MissingDeployment,
    MissingPerformance,
    MissingObservability,
    MissingRecovery,
    MissingOperational,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

/// Evidence-based quality scores (0.0–1.0 unless noted). Every score cites
/// the evidence that produced it.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SpecQuality {
    pub completeness: f64,
    pub consistency: f64,
    /// Count of unresolved clarification markers (lower is better).
    pub ambiguity: u32,
    pub coverage: f64,
    pub risk: f64,
    pub maintainability: f64,
    pub missing_sections: Vec<String>,
    pub review_status: String,
    pub verification_status: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence_refs: Vec<String>,
}

/// Reverse-synchronization proposal — Hermes proposes; the founder decides.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SpecRevisionProposal {
    pub proposal_id: String,
    pub spec_id: String,
    pub from_version: u32,
    pub proposed_by: String,
    pub trigger: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diff_ref: Option<String>,
    pub rationale: String,
    pub evidence_refs: Vec<String>,
    pub status: ProposalStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ProposalStatus {
    Proposed,
    Approved,
    Rejected,
}

/// Provenance recorded on a mission derived from one or more specs.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SpecProvenance {
    pub sources: Vec<SpecSourceRef>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SpecSourceRef {
    pub spec_id: String,
    pub version: u32,
    pub document_hash: String,
}
