//! Record mode (IADR-0002): a decorator that wraps a real adapter, forwards
//! each call, and writes the interaction to a cassette keyed identically to
//! replay. Recording a session against a real provider produces the exact
//! cassette that later replays deterministically in CI — the record/replay
//! harness the architecture mandates, as a side effect of the first real
//! adapter.

use crate::fixture::{cassette_key, write_recorded};
use crate::{Adapter, AdapterError, AdapterRequest, AdapterResponse};
use std::path::PathBuf;
use std::sync::Mutex;

pub struct RecordingAdapter {
    inner: Box<dyn Adapter>,
    cassette_path: PathBuf,
    /// Serializes appends so concurrent invocations cannot interleave lines.
    lock: Mutex<()>,
}

impl RecordingAdapter {
    pub fn new(inner: Box<dyn Adapter>, cassette_path: PathBuf) -> Self {
        Self {
            inner,
            cassette_path,
            lock: Mutex::new(()),
        }
    }
}

impl Adapter for RecordingAdapter {
    fn name(&self) -> &str {
        // Routes exactly like the wrapped adapter — recording is transparent.
        self.inner.name()
    }

    fn invoke(&self, req: &AdapterRequest) -> Result<AdapterResponse, AdapterError> {
        let resp = self.inner.invoke(req)?;
        let key = cassette_key(
            &req.intent,
            &req.pack_hash,
            &req.output_schema_id,
            &req.model,
        );
        let _guard = self.lock.lock().expect("recording lock");
        write_recorded(&self.cassette_path, &key, &resp)
            .map_err(|e| AdapterError::Provider(format!("cassette write failed: {e}")))?;
        Ok(resp)
    }

    fn supports_reformat_retry(&self) -> bool {
        self.inner.supports_reformat_retry()
    }
}
