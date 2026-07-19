//! S0.5A automated security tests. EXPERIMENTAL — NEVER MERGE.
//! Every negative must fail closed with a typed, sanitized response.
//! Zero third-party dependencies.
#![forbid(unsafe_code)]

use s05a_core::CoreClient;
use s05a_protocol as proto;
use s05a_protocol::json::Json;
use std::path::PathBuf;

const BIN: &str = env!("CARGO_BIN_EXE_s05a-core");
const READ: Option<&str> = Some("cap-read-fixture-001");
const WRITE: Option<&str> = Some("cap-write-output-001");

fn fixtures_dir() -> String {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../fixtures")
        .to_string_lossy()
        .into_owned()
}

fn temp_out(tag: &str) -> String {
    let p = std::env::temp_dir().join(format!(
        "s05a-out-{tag}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()
    ));
    std::fs::create_dir_all(&p).unwrap();
    p.to_string_lossy().into_owned()
}

fn client(tag: &str) -> (CoreClient, String) {
    let out = temp_out(tag);
    (CoreClient::spawn(BIN, Some(&fixtures_dir()), Some(&out), &[]), out)
}

fn assert_denied(r: &Json, needle: &str) {
    assert_eq!(proto::kind_of(r), Some("Denied"), "expected Denied, got {r:?}");
    let reason = proto::reason_of(r).unwrap_or("");
    assert!(reason.contains(needle), "reason {reason:?} lacks {needle:?}");
}
fn assert_error(r: &Json, needle: &str) {
    assert_eq!(proto::kind_of(r), Some("Error"), "expected Error, got {r:?}");
    let reason = proto::reason_of(r).unwrap_or("");
    assert!(reason.contains(needle), "reason {reason:?} lacks {needle:?}");
}
fn assert_ok(r: &Json) {
    assert_eq!(proto::kind_of(r), Some("Ok"), "expected Ok, got {r:?}");
}

// ---------- positive controls ----------

#[test]
fn positive_health_read_write_echo_explain() {
    let (mut c, out) = client("pos");
    let h = c.request("health", None, "{}");
    assert_ok(&h);
    let res = h.get("result").unwrap();
    assert_eq!(res.get("protocol").and_then(Json::as_str), Some("s05a/1"));
    assert!(res.get("core_build").and_then(Json::as_str).unwrap().contains("prototype"));
    assert!(res.get("env").is_none(), "health must not leak environment");

    let r = c.request("read_fixture", READ, r#"{"path":"hello.txt"}"#);
    assert_ok(&r);
    assert!(r.get("result").unwrap().get("content").and_then(Json::as_str).unwrap().contains("S0.5A fixture"));

    let w = c.request("write_output", WRITE, r#"{"path":"run/result.txt","content":"ok"}"#);
    assert_ok(&w);
    assert_eq!(std::fs::read_to_string(PathBuf::from(&out).join("run/result.txt")).unwrap(), "ok");

    assert_ok(&c.request("echo", None, r#"{"data":"x"}"#));

    let ex = c.request("explain", None, r#"{"op":"read_fixture","path":"hello.txt"}"#);
    assert_eq!(ex.get("result").unwrap().get("decision").and_then(Json::as_str), Some("Allowed"));
    assert_eq!(ex.get("result").unwrap().get("capability").and_then(Json::as_str), Some("cap-read-fixture-001"));

    let exd = c.request("explain", None, r#"{"op":"read_fixture","path":"../../outside.txt"}"#);
    assert_eq!(exd.get("result").unwrap().get("decision").and_then(Json::as_str), Some("Denied"));
    assert_eq!(exd.get("result").unwrap().get("resource").and_then(Json::as_str), Some("../../outside.txt"));
}

// ---------- the 18 required negatives ----------

#[test]
fn n01_unknown_operation() {
    let (mut c, _o) = client("n01");
    assert_denied(&c.request("spawn_process", None, "{}"), "unknown-operation");
}

#[test]
fn n02_malformed_message() {
    let (mut c, _o) = client("n02");
    c.raw_send(b"{ not json at all");
    assert_error(&c.raw_recv().unwrap(), "malformed-envelope");
    // Core must keep serving after a malformed envelope.
    assert_ok(&c.request("health", None, "{}"));
}

#[test]
fn n03_oversized_message_fails_closed() {
    let (mut c, _o) = client("n03");
    c.raw_send_prefix((proto::MAX_FRAME_BYTES + 1).to_be_bytes());
    assert_error(&c.raw_recv().unwrap(), "oversized-frame");
    let status = c.child.wait().unwrap();
    assert!(!status.success(), "core must exit non-zero after oversize");
}

#[test]
fn n04_unsupported_protocol_version() {
    let (mut c, _o) = client("n04");
    let r = c.request_with("health", None, "{}", None, Some("s05a/999"));
    assert_error(&r, "unsupported-protocol-version");
}

#[test]
fn n05_wrong_session() {
    let (mut c, _o) = client("n05");
    let r = c.request_with("health", None, "{}", Some("s-forged-0000"), None);
    assert_denied(&r, "stale-or-unknown-session");
}

#[test]
fn n06_missing_capability_grant() {
    let out = temp_out("n06");
    let mut c = CoreClient::spawn(BIN, Some(&fixtures_dir()), Some(&out), &["cap-read-fixture-001"]);
    assert_denied(&c.request("read_fixture", READ, r#"{"path":"hello.txt"}"#), "capability-not-granted");
}

#[test]
fn n07_wrong_capability_action() {
    let (mut c, _o) = client("n07");
    // Cite the write capability for a read action.
    assert_denied(&c.request("read_fixture", WRITE, r#"{"path":"hello.txt"}"#), "missing-or-wrong-capability-for-action:read");
}

#[test]
fn n08_path_traversal() {
    let (mut c, _o) = client("n08");
    assert_denied(&c.request("read_fixture", READ, r#"{"path":"../core/Cargo.toml"}"#), "traversal-or-prefix-rejected");
}

#[test]
fn n09_absolute_path() {
    let (mut c, _o) = client("n09");
    let params = if cfg!(windows) { r#"{"path":"C:/Windows/win.ini"}"# } else { r#"{"path":"/etc/hostname"}"# };
    assert_denied(&c.request("read_fixture", READ, params), "absolute-path-rejected");
}

#[test]
fn n10_symlink_or_junction_escape() {
    let out = temp_out("n10");
    let fx = PathBuf::from(temp_out("n10fx"));
    let outside = PathBuf::from(temp_out("n10outside"));
    std::fs::write(outside.join("secret.txt"), "outside").unwrap();
    let link = fx.join("leak");
    if !make_link_dir(&outside, &link) {
        eprintln!("SKIP-HONESTLY: link creation unavailable on this host");
        return;
    }
    let mut c = CoreClient::spawn(BIN, Some(&fx.to_string_lossy()), Some(&out), &[]);
    assert_denied(&c.request("read_fixture", READ, r#"{"path":"leak/secret.txt"}"#), "symlink-or-reparse-rejected");
}

#[test]
fn n11_write_outside_scope() {
    let (mut c, _o) = client("n11");
    assert_denied(&c.request("write_output", WRITE, r#"{"path":"../escape.txt","content":"x"}"#), "traversal-or-prefix-rejected");
}

#[test]
fn n12_git_metadata_access() {
    let (mut c, _o) = client("n12");
    assert_denied(&c.request("write_output", WRITE, r#"{"path":".git/config","content":"x"}"#), "git-metadata-access-rejected");
    assert_denied(&c.request("read_fixture", READ, r#"{"path":".git/HEAD"}"#), "git-metadata-access-rejected");
}

#[test]
fn n13_unexposed_command_surface() {
    let (mut c, _o) = client("n13");
    for op in ["shell_exec", "net_connect", "secret_get", "env_read", "db_query"] {
        assert_denied(&c.request(op, None, "{}"), "unknown-operation");
    }
}

#[test]
fn n14_core_crash_is_visible() {
    let (mut c, _o) = client("n14");
    c.child.kill().unwrap();
    assert!(!c.child.wait().unwrap().success());
    // Best-effort send: the pipe may already be closed. Either the send
    // or the receive must fail — the crash is observable, never a hang.
    let send_failed = c.try_send(b"{}").is_err();
    let recv_failed = c.raw_recv().is_err();
    assert!(send_failed || recv_failed, "crash must be observable as channel closure");
}

#[test]
fn n15_restart_gets_fresh_session() {
    let (c1, _o1) = client("n15a");
    let s1 = c1.session.clone();
    drop(c1);
    let (c2, _o2) = client("n15b");
    assert_ne!(s1, c2.session, "restart must mint a fresh session");
}

#[test]
fn n16_stale_replay_after_restart() {
    let (c1, _o1) = client("n16a");
    let old = c1.session.clone();
    drop(c1);
    let (mut c2, _o2) = client("n16b");
    let r = c2.request_with("health", None, "{}", Some(&old), None);
    assert_denied(&r, "stale-or-unknown-session");
}

#[test]
fn n17_malformed_capability_object() {
    let (mut c, _o) = client("n17");
    // Structured capability object instead of a string handle.
    let env = format!(
        r#"{{"v":"s05a/1","id":1,"session":"{}","capability":{{"id":"x","scope":"all"}},"op":"read_fixture","params":{{"path":"hello.txt"}}}}"#,
        c.session
    );
    c.raw_send(env.as_bytes());
    assert_error(&c.raw_recv().unwrap(), "malformed-envelope");
}

#[test]
fn n18_unexpected_extra_fields() {
    let (mut c, _o) = client("n18");
    let env = format!(
        r#"{{"v":"s05a/1","id":1,"session":"{}","capability":null,"op":"health","params":{{}},"sudo":true}}"#,
        c.session
    );
    c.raw_send(env.as_bytes());
    assert_error(&c.raw_recv().unwrap(), "malformed-envelope");
    // Extra field inside params for a strict operation.
    assert_error(&c.request("echo", None, r#"{"data":"x","extra":1}"#), "malformed-params");
}

// ---------- helpers ----------

#[cfg(windows)]
fn make_link_dir(target: &PathBuf, link: &PathBuf) -> bool {
    // Directory junction: creatable without special privilege and is a
    // reparse point, which the core must reject component-wise.
    std::process::Command::new("cmd")
        .args(["/C", "mklink", "/J"])
        .arg(link)
        .arg(target)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

#[cfg(not(windows))]
fn make_link_dir(target: &PathBuf, link: &PathBuf) -> bool {
    std::os::unix::fs::symlink(target, link).is_ok()
}
