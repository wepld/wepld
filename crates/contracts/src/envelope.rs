//! Sandbox envelope (v2-05) — the per-phase grant that is the real
//! enforcement unit (ADR-0004), and the honest tier taxonomy (ADR-0007,
//! `DEV` added by IADR-0003).

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Envelope {
    pub envelope_id: String,
    pub attempt_id: String,
    pub sandbox_tier: SandboxTier,
    pub fs: FsScope,
    pub network: NetworkPolicy,
    pub process: ProcessLimits,
    /// Default empty; any secret is a hard gate.
    pub secrets: Vec<String>,
    pub expires_at: String,
}

/// Tiers are confirmed by canary self-test, recorded in the ledger, and cap
/// the autonomy envelope. `DEV` means: no isolation, full disclosure,
/// Manual-mode-and-fixture-repos-only caps (IADR-0003).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum SandboxTier {
    #[serde(rename = "DEV")]
    Dev,
    S0,
    S1,
    S2,
    #[serde(rename = "S2W")]
    S2w,
    S3,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FsScope {
    pub write: Vec<String>,
    pub read: Vec<String>,
    pub deny: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct NetworkPolicy {
    /// "deny" by default; grants name destination classes (v2-05).
    pub mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ProcessLimits {
    pub max_procs: u32,
    pub max_mem_mb: u32,
    pub cpu_share: f64,
    pub timeout_s: u32,
}
