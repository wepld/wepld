//! Event (Ledger Entry) Contract (v2-06 schema, v2-07 §5 vocabulary).
//! Entries are append-only, hash-chained facts; payloads are body-light and
//! reference artifacts by hash.

use crate::vocabulary::EventType;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LedgerEntry {
    /// Authoritative local order (v2-06: the deterministic sequence).
    pub seq: i64,
    /// ULID.
    pub entry_id: String,
    pub ts_utc: String,
    pub entry_type: EventType,
    pub schema_version: u32,
    pub aggregate_type: AggregateType,
    pub aggregate_id: String,
    pub actor_type: ActorType,
    pub actor_id: String,
    /// Mission-level trace id.
    pub correlation_id: String,
    /// Parent command id or parent entry id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub causation_ref: Option<String>,
    /// Minimal typed fact; large or sensitive bodies live in the artifact
    /// store and are referenced by hash.
    pub payload_json: serde_json::Value,
    pub payload_hash: String,
    /// Chain: entry_hash = H(prev_hash || payload_hash || entry_id).
    pub prev_hash: String,
    pub entry_hash: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AggregateType {
    Mission,
    Task,
    Attempt,
    Decision,
    System,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ActorType {
    Human,
    Core,
    Worker,
    BrainAdapter,
}
