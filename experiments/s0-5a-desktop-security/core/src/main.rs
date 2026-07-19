//! S0.5A prototype core process. EXPERIMENTAL — NEVER MERGE.
//!
//! A deliberately small authority boundary under evaluation: typed
//! framed-stdio IPC, a static two-entry capability table, and
//! component-checked path confinement. This is NOT a production policy
//! engine; it implements only enough to test the proposed boundary.
#![forbid(unsafe_code)]

use s05a_protocol as proto;
use s05a_protocol::json::Json;
use std::collections::BTreeMap;
use std::io::Write;
use std::path::{Component, Path, PathBuf};

const CAP_READ: &str = "cap-read-fixture-001";
const CAP_WRITE: &str = "cap-write-output-001";
const ENGINE: &str = "static-prototype-table/2-entries";

struct Caps {
    read_root: Option<PathBuf>,
    write_root: Option<PathBuf>,
}

fn build_id() -> String {
    format!("s05a-core {} (prototype)", env!("CARGO_PKG_VERSION"))
}

fn main() {
    let mut fixtures: Option<PathBuf> = None;
    let mut output: Option<PathBuf> = None;
    let mut drop_caps: Vec<String> = Vec::new();
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--fixtures" => fixtures = args.next().map(PathBuf::from),
            "--output" => output = args.next().map(PathBuf::from),
            "--drop-cap" => drop_caps.extend(args.next()),
            _ => {
                eprintln!("unknown argument: {a}");
                std::process::exit(2);
            }
        }
    }
    // Non-cryptographic session identifier (documented in PROTOCOL.md):
    // uniqueness across restarts is what the replay tests require;
    // unguessability is NOT claimed and no hand-written crypto is used.
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let session = format!("s{:x}-{:x}", std::process::id(), nanos);

    let caps = Caps {
        read_root: fixtures.filter(|_| !drop_caps.iter().any(|c| c == CAP_READ)),
        write_root: output.filter(|_| !drop_caps.iter().any(|c| c == CAP_WRITE)),
    };

    let stdout = std::io::stdout();
    let mut out = stdout.lock();
    let hello = proto::hello(&build_id(), &session, ENGINE);
    send_raw(&mut out, hello.as_bytes());

    let stdin = std::io::stdin();
    let mut inp = stdin.lock();
    loop {
        let body = match proto::read_frame(&mut inp) {
            Ok(b) => b,
            Err(proto::FrameError::Closed) => std::process::exit(0),
            Err(proto::FrameError::Oversized(n)) => {
                // Fail closed hard: a desynchronized stream is untrusted.
                let e = proto::error(0, &format!("oversized-frame:{n}>max:{}", proto::MAX_FRAME_BYTES));
                send(&mut out, &e);
                std::process::exit(2);
            }
            Err(proto::FrameError::Io(_)) => std::process::exit(2),
        };
        let text = match std::str::from_utf8(&body) {
            Ok(t) => t,
            Err(_) => {
                send(&mut out, &proto::error(0, "non-utf8-frame"));
                continue;
            }
        };
        let parsed = match proto::json::parse(text) {
            Ok(j) => j,
            Err(_) => {
                send(&mut out, &proto::error(0, "malformed-envelope"));
                continue;
            }
        };
        let env = match proto::decode_envelope(&parsed) {
            Ok(e) => e,
            Err(_) => {
                send(&mut out, &proto::error(0, "malformed-envelope"));
                continue;
            }
        };
        let resp = handle(&env, &session, &caps);
        send(&mut out, &resp);
    }
}

fn send_raw(out: &mut impl Write, body: &[u8]) {
    if proto::write_frame(out, body).is_err() {
        std::process::exit(2);
    }
}
fn send(out: &mut impl Write, resp: &Json) {
    send_raw(out, proto::json::to_string(resp).as_bytes());
}

fn handle(env: &proto::Envelope, session: &str, caps: &Caps) -> Json {
    if env.v != proto::PROTOCOL_VERSION {
        return proto::error(env.id, &format!("unsupported-protocol-version:{}", env.v));
    }
    if env.session != session {
        return proto::denied(env.id, "stale-or-unknown-session", None, None);
    }
    match env.op.as_str() {
        "health" => {
            let mut m = BTreeMap::new();
            m.insert("protocol".to_string(), Json::Str(proto::PROTOCOL_VERSION.into()));
            m.insert("core_build".to_string(), Json::Str(build_id()));
            m.insert("session".to_string(), Json::Str(session.into()));
            m.insert("capability_engine".to_string(), Json::Str(ENGINE.into()));
            proto::ok(env.id, Json::Obj(m))
        }
        "echo" => match proto::echo_param(&env.params) {
            Ok(d) => {
                let mut m = BTreeMap::new();
                m.insert("data".to_string(), Json::Str(d));
                proto::ok(env.id, Json::Obj(m))
            }
            Err(_) => proto::error(env.id, "malformed-params"),
        },
        "read_fixture" => match proto::path_param(&env.params) {
            Ok(p) => scoped_read(env, caps, &p),
            Err(_) => proto::error(env.id, "malformed-params"),
        },
        "write_output" => match proto::write_params(&env.params) {
            Ok((p, c)) => scoped_write(env, caps, &p, &c),
            Err(_) => proto::error(env.id, "malformed-params"),
        },
        "explain" => match proto::explain_params(&env.params) {
            Ok((op, p)) => explain(env, caps, &op, &p),
            Err(_) => proto::error(env.id, "malformed-params"),
        },
        other => proto::denied(env.id, &format!("unknown-operation:{other}"), env.capability.as_deref(), None),
    }
}

/// Confinement rules, checked in order. Authorization is by resolved
/// path components — never by string prefix.
fn confine(root: &Path, requested: &str) -> Result<PathBuf, &'static str> {
    let rp = Path::new(requested);
    if rp.is_absolute() {
        return Err("absolute-path-rejected");
    }
    for comp in rp.components() {
        match comp {
            Component::Normal(seg) => {
                if seg.to_string_lossy().eq_ignore_ascii_case(".git") {
                    return Err("git-metadata-access-rejected");
                }
            }
            Component::CurDir => {}
            _ => return Err("traversal-or-prefix-rejected"),
        }
    }
    let root_canon = root.canonicalize().map_err(|_| "scope-root-unavailable")?;
    // Component-wise reparse/symlink rejection along the requested path.
    let mut walk = root_canon.clone();
    for comp in rp.components() {
        if let Component::Normal(seg) = comp {
            walk.push(seg);
            if let Ok(md) = std::fs::symlink_metadata(&walk) {
                if is_reparse_or_symlink(&md) {
                    return Err("symlink-or-reparse-rejected");
                }
            }
        }
    }
    // Final containment check: canonicalize the nearest EXISTING
    // ancestor (the target file or its parent dirs may not exist yet
    // for a write) and confirm it resolves inside the scope root. This
    // resolves any symlink in an existing ancestor, so a symlinked
    // ancestor pointing outside the root is rejected here too.
    let mut existing = walk.as_path();
    loop {
        match existing.canonicalize() {
            Ok(canon) => {
                if !canon.starts_with(&root_canon) {
                    return Err("outside-declared-scope");
                }
                break;
            }
            Err(_) => match existing.parent() {
                Some(p) => existing = p,
                None => return Err("scope-resolution-failed"),
            },
        }
    }
    Ok(walk)
}

#[cfg(windows)]
fn is_reparse_or_symlink(md: &std::fs::Metadata) -> bool {
    use std::os::windows::fs::MetadataExt;
    const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x400;
    md.file_type().is_symlink() || (md.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT) != 0
}

#[cfg(not(windows))]
fn is_reparse_or_symlink(md: &std::fs::Metadata) -> bool {
    md.file_type().is_symlink()
}

fn scoped_read(env: &proto::Envelope, caps: &Caps, path: &str) -> Json {
    if env.capability.as_deref() != Some(CAP_READ) {
        return proto::denied(env.id, "missing-or-wrong-capability-for-action:read", env.capability.as_deref(), Some(path));
    }
    let Some(root) = &caps.read_root else {
        return proto::denied(env.id, "capability-not-granted", Some(CAP_READ), Some(path));
    };
    match confine(root, path) {
        Ok(p) => match std::fs::read_to_string(&p) {
            Ok(content) => {
                let mut m = BTreeMap::new();
                m.insert("path".to_string(), Json::Str(path.into()));
                m.insert("bytes".to_string(), Json::Num(content.len() as f64));
                m.insert("content".to_string(), Json::Str(content));
                proto::ok(env.id, Json::Obj(m))
            }
            Err(_) => proto::denied(env.id, "unreadable-or-missing-in-scope", Some(CAP_READ), Some(path)),
        },
        Err(reason) => proto::denied(env.id, reason, Some(CAP_READ), Some(path)),
    }
}

fn scoped_write(env: &proto::Envelope, caps: &Caps, path: &str, content: &str) -> Json {
    if env.capability.as_deref() != Some(CAP_WRITE) {
        return proto::denied(env.id, "missing-or-wrong-capability-for-action:write", env.capability.as_deref(), Some(path));
    }
    let Some(root) = &caps.write_root else {
        return proto::denied(env.id, "capability-not-granted", Some(CAP_WRITE), Some(path));
    };
    match confine(root, path) {
        Ok(p) => {
            if let Some(dir) = p.parent() {
                if std::fs::create_dir_all(dir).is_err() {
                    return proto::denied(env.id, "scope-directory-unavailable", Some(CAP_WRITE), Some(path));
                }
            }
            match std::fs::write(&p, content) {
                Ok(()) => {
                    let mut m = BTreeMap::new();
                    m.insert("path".to_string(), Json::Str(path.into()));
                    m.insert("bytes".to_string(), Json::Num(content.len() as f64));
                    proto::ok(env.id, Json::Obj(m))
                }
                Err(_) => proto::denied(env.id, "write-failed-in-scope", Some(CAP_WRITE), Some(path)),
            }
        }
        Err(reason) => proto::denied(env.id, reason, Some(CAP_WRITE), Some(path)),
    }
}

fn explain(env: &proto::Envelope, caps: &Caps, op: &str, path: &str) -> Json {
    let (cap, root) = match op {
        "read_fixture" => (CAP_READ, caps.read_root.as_deref()),
        "write_output" => (CAP_WRITE, caps.write_root.as_deref()),
        _ => {
            let mut m = BTreeMap::new();
            m.insert("decision".to_string(), Json::Str("Denied".into()));
            m.insert("reason".to_string(), Json::Str(format!("unknown-operation:{op}")));
            m.insert("capability".to_string(), Json::Null);
            m.insert("resource".to_string(), Json::Str(path.into()));
            return proto::ok(env.id, Json::Obj(m));
        }
    };
    let (decision, reason) = match root {
        None => ("Denied", "capability-not-granted".to_string()),
        Some(r) => match confine(r, path) {
            Ok(_) => ("Allowed", format!("within-declared-scope-of:{cap}")),
            Err(reason) => ("Denied", reason.to_string()),
        },
    };
    let mut m = BTreeMap::new();
    m.insert("decision".to_string(), Json::Str(decision.into()));
    m.insert("reason".to_string(), Json::Str(reason));
    m.insert("capability".to_string(), Json::Str(cap.into()));
    m.insert("resource".to_string(), Json::Str(path.into()));
    proto::ok(env.id, Json::Obj(m))
}
