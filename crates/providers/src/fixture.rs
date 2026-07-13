//! The fixture adapter (IADR-0002): deterministic replay of recorded
//! responses from JSONL cassettes. Keyed by (intent, pack hash, output schema
//! id, model). A miss fails loudly — never improvisation, never a silent
//! fallback to a live provider.

use crate::{Adapter, AdapterError, AdapterRequest, AdapterResponse};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::io::{BufRead, Write};
use std::path::Path;
use wepld_contracts::brain::Usage;

#[derive(Debug, Serialize, Deserialize)]
struct CassetteEntry {
    key: String,
    output: serde_json::Value,
    provider: String,
    model: String,
    tokens_in: u64,
    tokens_out: u64,
    cost_usd: f64,
    latency_ms: u64,
}

pub fn cassette_key(intent: &str, pack_hash: &str, output_schema_id: &str, model: &str) -> String {
    let mut h = Sha256::new();
    h.update(intent.as_bytes());
    h.update(b"\x1f");
    h.update(pack_hash.as_bytes());
    h.update(b"\x1f");
    h.update(output_schema_id.as_bytes());
    h.update(b"\x1f");
    h.update(model.as_bytes());
    let d = h.finalize();
    let mut s = String::with_capacity(64);
    use std::fmt::Write as _;
    for b in d {
        let _ = write!(s, "{b:02x}");
    }
    s
}

/// Author or append a cassette entry (used by tests now; by the record-mode
/// proxy against real providers from M1).
pub fn write_cassette_entry(
    path: &Path,
    key: &str,
    output: &serde_json::Value,
    model: &str,
) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let entry = CassetteEntry {
        key: key.to_owned(),
        output: output.clone(),
        provider: "fixture".to_owned(),
        model: model.to_owned(),
        tokens_in: 0,
        tokens_out: 0,
        cost_usd: 0.0,
        latency_ms: 0,
    };
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;
    writeln!(f, "{}", serde_json::to_string(&entry)?)?;
    Ok(())
}

pub struct FixtureAdapter {
    entries: HashMap<String, CassetteEntry>,
}

impl FixtureAdapter {
    /// Load every `*.jsonl` cassette under the given directories. Missing
    /// directories are fine (empty cassette set — every call then misses,
    /// loudly).
    pub fn load(dirs: &[&Path]) -> std::io::Result<Self> {
        let mut entries = HashMap::new();
        for dir in dirs {
            if !dir.exists() {
                continue;
            }
            for file in std::fs::read_dir(dir)? {
                let path = file?.path();
                if path.extension().and_then(|e| e.to_str()) != Some("jsonl") {
                    continue;
                }
                let reader = std::io::BufReader::new(std::fs::File::open(&path)?);
                for line in reader.lines() {
                    let line = line?;
                    if line.trim().is_empty() {
                        continue;
                    }
                    let entry: CassetteEntry = serde_json::from_str(&line)?;
                    entries.insert(entry.key.clone(), entry);
                }
            }
        }
        Ok(Self { entries })
    }
}

impl Adapter for FixtureAdapter {
    fn name(&self) -> &str {
        "fixture"
    }

    fn invoke(&self, req: &AdapterRequest) -> Result<AdapterResponse, AdapterError> {
        let key = cassette_key(
            &req.intent,
            &req.pack_hash,
            &req.output_schema_id,
            &req.model,
        );
        let entry = self
            .entries
            .get(&key)
            .ok_or(AdapterError::CassetteMiss(key))?;
        Ok(AdapterResponse {
            output: entry.output.clone(),
            usage: Usage {
                provider: entry.provider.clone(),
                model: entry.model.clone(),
                tokens_in: entry.tokens_in,
                tokens_out: entry.tokens_out,
                cost_usd: entry.cost_usd,
                latency_ms: entry.latency_ms,
            },
        })
    }
}
