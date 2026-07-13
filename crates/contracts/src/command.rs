//! Command contract (v2-07 command model, subset). Commands are named intent
//! with an idempotency identity; outcomes are exactly the four of v2-02 §2.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Command {
    /// Idempotency key: resubmitting the same id returns the prior outcome.
    pub command_id: String,
    /// Named intent, e.g. "create_mission".
    pub command_type: String,
    /// Authenticated principal (MVP: the local principal).
    pub actor: String,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub enum CommandOutcome {
    /// Durable; effects appear as ledger facts.
    Accepted {
        detail: serde_json::Value,
    },
    Rejected {
        reason: String,
    },
    AwaitingApproval {
        decision_id: String,
    },
    Deferred {
        reason: String,
    },
}
