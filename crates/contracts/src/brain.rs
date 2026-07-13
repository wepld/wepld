//! Brain Contract result shape (v2-07 §3). Requests travel as the WWP
//! `brain.request` message; this is the provider-neutral result the gateway
//! returns to workers. Only the gateway knows provider APIs.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BrainResult {
    pub schema_version: u32,
    pub invocation_id: String,
    pub status: BrainStatus,
    /// Schema-valid answer when status is `ok`; empty object otherwise.
    pub output: serde_json::Value,
    pub usage: Usage,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum BrainStatus {
    Ok,
    SchemaInvalid,
    Refused,
    ProviderError,
    BudgetDenied,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Usage {
    pub provider: String,
    pub model: String,
    pub tokens_in: u64,
    pub tokens_out: u64,
    pub cost_usd: f64,
    pub latency_ms: u64,
}
