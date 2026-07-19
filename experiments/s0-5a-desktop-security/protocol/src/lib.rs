//! S0.5A prototype IPC protocol: typed envelopes over length-prefixed
//! JSON frames, with ZERO third-party dependencies. EXPERIMENTAL —
//! NEVER MERGE; not product code. See DEPENDENCY_EVIDENCE.md for why
//! this prototype hand-rolls JSON instead of using serde.
#![forbid(unsafe_code)]

pub mod json;

use json::Json;
use std::io::{Read, Write};

/// Protocol version string; requests with any other value are rejected.
pub const PROTOCOL_VERSION: &str = "s05a/1";
/// Hard frame ceiling. Oversized frames are rejected fail-closed and
/// terminate the core, because a desynchronized stream cannot be
/// trusted afterwards.
pub const MAX_FRAME_BYTES: u32 = 65_536;

/// Recognized envelope keys. Any other key is a strict-decode failure.
const ENVELOPE_KEYS: &[&str] = &["v", "id", "session", "capability", "op", "params"];

#[derive(Debug, Clone)]
pub struct Envelope {
    pub v: String,
    pub id: u64,
    pub session: String,
    pub capability: Option<String>,
    pub op: String,
    pub params: Json,
}

/// Strictly decode an envelope: every key must be recognized, and
/// `capability` must be a string or null. Anything else fails closed.
pub fn decode_envelope(v: &Json) -> Result<Envelope, String> {
    let keys = v.obj_keys().ok_or("envelope-not-object")?;
    for k in &keys {
        if !ENVELOPE_KEYS.contains(k) {
            return Err(format!("unknown-field:{k}"));
        }
    }
    let ver = v.get("v").and_then(Json::as_str).ok_or("missing-v")?.to_string();
    let id = v.get("id").and_then(Json::as_u64).ok_or("missing-id")?;
    let session = v.get("session").and_then(Json::as_str).ok_or("missing-session")?.to_string();
    let capability = match v.get("capability") {
        None | Some(Json::Null) => None,
        Some(Json::Str(s)) => Some(s.clone()),
        // A structured capability object is not accepted at this seam.
        Some(_) => return Err("capability-not-string".into()),
    };
    let op = v.get("op").and_then(Json::as_str).ok_or("missing-op")?.to_string();
    let params = v.get("params").cloned().unwrap_or(Json::Null);
    Ok(Envelope { v: ver, id, session, capability, op, params })
}

/// Strictly extract the single `path` string from a params object.
pub fn path_param(params: &Json) -> Result<String, String> {
    only_keys(params, &["path"])?;
    params.get("path").and_then(Json::as_str).map(str::to_string).ok_or("missing-path".into())
}

/// Strictly extract `path` and `content` for write.
pub fn write_params(params: &Json) -> Result<(String, String), String> {
    only_keys(params, &["path", "content"])?;
    let p = params.get("path").and_then(Json::as_str).ok_or("missing-path")?.to_string();
    let c = params.get("content").and_then(Json::as_str).ok_or("missing-content")?.to_string();
    Ok((p, c))
}

/// Strictly extract `data` for echo.
pub fn echo_param(params: &Json) -> Result<String, String> {
    only_keys(params, &["data"])?;
    params.get("data").and_then(Json::as_str).map(str::to_string).ok_or("missing-data".into())
}

/// Strictly extract `op` and `path` for explain.
pub fn explain_params(params: &Json) -> Result<(String, String), String> {
    only_keys(params, &["op", "path"])?;
    let o = params.get("op").and_then(Json::as_str).ok_or("missing-op")?.to_string();
    let p = params.get("path").and_then(Json::as_str).ok_or("missing-path")?.to_string();
    Ok((o, p))
}

fn only_keys(params: &Json, allowed: &[&str]) -> Result<(), String> {
    let keys = params.obj_keys().ok_or("params-not-object")?;
    for k in keys {
        if !allowed.contains(&k) {
            return Err(format!("unknown-param:{k}"));
        }
    }
    Ok(())
}

/// Read one length-prefixed frame (4-byte big-endian length + body).
#[derive(Debug)]
pub enum FrameError {
    Closed,
    Oversized(u32),
    Io(std::io::Error),
}

pub fn write_frame(w: &mut impl Write, body: &[u8]) -> std::io::Result<()> {
    let len = u32::try_from(body.len()).map_err(|_| {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, "frame too large")
    })?;
    w.write_all(&len.to_be_bytes())?;
    w.write_all(body)?;
    w.flush()
}

pub fn read_frame(r: &mut impl Read) -> Result<Vec<u8>, FrameError> {
    let mut len_buf = [0u8; 4];
    if let Err(e) = r.read_exact(&mut len_buf) {
        return if e.kind() == std::io::ErrorKind::UnexpectedEof {
            Err(FrameError::Closed)
        } else {
            Err(FrameError::Io(e))
        };
    }
    let len = u32::from_be_bytes(len_buf);
    if len > MAX_FRAME_BYTES {
        return Err(FrameError::Oversized(len));
    }
    let mut body = vec![0u8; len as usize];
    r.read_exact(&mut body).map_err(FrameError::Io)?;
    Ok(body)
}

/// Helpers to build response frames as JSON.
pub fn hello(core_build: &str, session: &str, engine: &str) -> String {
    let mut m = std::collections::BTreeMap::new();
    m.insert("kind".into(), Json::Str("Hello".into()));
    m.insert("v".into(), Json::Str(PROTOCOL_VERSION.into()));
    m.insert("core_build".into(), Json::Str(core_build.into()));
    m.insert("session".into(), Json::Str(session.into()));
    m.insert("capability_engine".into(), Json::Str(engine.into()));
    json::to_string(&Json::Obj(m))
}

pub fn ok(id: u64, result: Json) -> Json {
    let mut m = std::collections::BTreeMap::new();
    m.insert("kind".into(), Json::Str("Ok".into()));
    m.insert("id".into(), Json::Num(id as f64));
    m.insert("result".into(), result);
    Json::Obj(m)
}

pub fn denied(id: u64, reason: &str, capability: Option<&str>, resource: Option<&str>) -> Json {
    let mut m = std::collections::BTreeMap::new();
    m.insert("kind".into(), Json::Str("Denied".into()));
    m.insert("id".into(), Json::Num(id as f64));
    m.insert("reason".into(), Json::Str(reason.into()));
    m.insert(
        "capability".into(),
        capability.map(|c| Json::Str(c.into())).unwrap_or(Json::Null),
    );
    m.insert(
        "resource".into(),
        resource.map(|r| Json::Str(r.into())).unwrap_or(Json::Null),
    );
    Json::Obj(m)
}

pub fn error(id: u64, reason: &str) -> Json {
    let mut m = std::collections::BTreeMap::new();
    m.insert("kind".into(), Json::Str("Error".into()));
    m.insert("id".into(), Json::Num(id as f64));
    m.insert("reason".into(), Json::Str(reason.into()));
    Json::Obj(m)
}

/// Convenience accessors used by test/bench clients.
pub fn kind_of(v: &Json) -> Option<&str> {
    v.get("kind").and_then(Json::as_str)
}
pub fn reason_of(v: &Json) -> Option<&str> {
    v.get("reason").and_then(Json::as_str)
}
