//! wepld-providers — the Provider Gateway. The Runtime never calls providers
//! directly; every reasoning request passes through [`Gateway::invoke`]:
//! validate → resolve profile → adapter → schema-check → normalized
//! [`BrainResult`]. Providers are leaf adapters; nothing provider-specific
//! leaks upward, no crate outside this one names a vendor.
//!
//! Reasoning is OPTIONAL (IADR-0007 §1): a phase that never calls the
//! gateway is a normal, first-class execution.
//!
//! Fixture-first (IADR-0002): the deterministic cassette adapter is the
//! default; a cassette miss FAILS LOUDLY and never falls back to a live
//! provider.

mod fixture;
mod openai;
mod record;
mod schema;

pub use fixture::{cassette_key, write_cassette_entry, write_recorded, FixtureAdapter};
pub use openai::{AdapterConfigError, OpenAiCompatAdapter};
pub use record::RecordingAdapter;
pub use schema::SchemaRegistry;

use std::collections::HashMap;
use wepld_contracts::brain::{BrainResult, BrainStatus, Usage};

#[derive(Debug, thiserror::Error)]
pub enum GatewayError {
    #[error("unknown brain profile: {0}")]
    UnknownProfile(String),
    #[error("unknown adapter: {0}")]
    UnknownAdapter(String),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("serialization error: {0}")]
    Serde(#[from] serde_json::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum AdapterError {
    #[error("cassette miss: no recorded response for key {0} (record one; the gateway never falls back silently)")]
    CassetteMiss(String),
    #[error("provider error: {0}")]
    Provider(String),
}

/// A named, versioned reasoning configuration (v2-07 §3). Workers request
/// profiles; only the gateway resolves vendors/models.
#[derive(Debug, Clone)]
pub struct Profile {
    pub name: String,
    pub adapter: String,
    pub model: String,
    pub timeout_ms: u64,
}

pub struct AdapterRequest {
    pub model: String,
    pub intent: String,
    pub pack: serde_json::Value,
    pub pack_hash: String,
    pub output_schema_id: String,
}

pub struct AdapterResponse {
    pub output: serde_json::Value,
    pub usage: Usage,
}

pub trait Adapter: Send {
    fn name(&self) -> &str;
    fn invoke(&self, req: &AdapterRequest) -> Result<AdapterResponse, AdapterError>;
    /// Real adapters may support one reformat retry on schema failure;
    /// deterministic adapters do not (retrying a cassette is meaningless).
    fn supports_reformat_retry(&self) -> bool {
        false
    }
}

pub struct Gateway {
    adapters: HashMap<String, Box<dyn Adapter>>,
    profiles: HashMap<String, Profile>,
    schemas: SchemaRegistry,
}

impl Gateway {
    pub fn new(schemas: SchemaRegistry) -> Self {
        Self {
            adapters: HashMap::new(),
            profiles: HashMap::new(),
            schemas,
        }
    }

    pub fn register_adapter(&mut self, adapter: Box<dyn Adapter>) {
        self.adapters.insert(adapter.name().to_owned(), adapter);
    }

    pub fn register_profile(&mut self, profile: Profile) -> Result<(), GatewayError> {
        if !self.adapters.contains_key(&profile.adapter) {
            return Err(GatewayError::UnknownAdapter(profile.adapter));
        }
        self.profiles.insert(profile.name.clone(), profile);
        Ok(())
    }

    /// The pipeline: resolve profile → invoke adapter → schema-check →
    /// normalized result. Infra errors are `Err`; provider/schema failures
    /// are honest statuses in `Ok` (the failure taxonomy decides upstream).
    pub fn invoke(
        &self,
        invocation_id: &str,
        profile_name: &str,
        intent: &str,
        pack: &serde_json::Value,
        pack_hash: &str,
        output_schema_id: &str,
    ) -> Result<BrainResult, GatewayError> {
        let profile = self
            .profiles
            .get(profile_name)
            .ok_or_else(|| GatewayError::UnknownProfile(profile_name.to_owned()))?;
        let adapter = self
            .adapters
            .get(&profile.adapter)
            .ok_or_else(|| GatewayError::UnknownAdapter(profile.adapter.clone()))?;

        // Request validation before any provider is touched: an output
        // schema the gateway cannot verify is refused up front.
        if !self.schemas.knows(output_schema_id) {
            return Ok(failure_result(
                invocation_id,
                profile,
                BrainStatus::SchemaInvalid,
                &format!("unknown output schema id: {output_schema_id}"),
            ));
        }

        let req = AdapterRequest {
            model: profile.model.clone(),
            intent: intent.to_owned(),
            pack: pack.clone(),
            pack_hash: pack_hash.to_owned(),
            output_schema_id: output_schema_id.to_owned(),
        };

        let response = match adapter.invoke(&req) {
            Ok(r) => r,
            Err(e) => {
                return Ok(failure_result(
                    invocation_id,
                    profile,
                    BrainStatus::ProviderError,
                    &e.to_string(),
                ));
            }
        };

        if let Err(missing) = self.schemas.validate(output_schema_id, &response.output) {
            // One reformat retry where the adapter supports it (real
            // providers, M1); deterministic adapters fail honestly.
            return Ok(failure_result(
                invocation_id,
                profile,
                BrainStatus::SchemaInvalid,
                &format!("output missing required fields {missing:?} for {output_schema_id}"),
            ));
        }

        Ok(BrainResult {
            schema_version: 1,
            invocation_id: invocation_id.to_owned(),
            status: BrainStatus::Ok,
            output: response.output,
            usage: response.usage,
            reason: None,
        })
    }
}

fn failure_result(
    invocation_id: &str,
    profile: &Profile,
    status: BrainStatus,
    reason: &str,
) -> BrainResult {
    BrainResult {
        schema_version: 1,
        invocation_id: invocation_id.to_owned(),
        status,
        output: serde_json::json!({}),
        usage: Usage {
            provider: profile.adapter.clone(),
            model: profile.model.clone(),
            tokens_in: 0,
            tokens_out: 0,
            cost_usd: 0.0,
            latency_ms: 0,
        },
        reason: Some(reason.to_owned()),
    }
}
