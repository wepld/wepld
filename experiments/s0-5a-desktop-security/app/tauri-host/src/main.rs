//! S0.5A minimal Tauri 2 host bridge. EXPERIMENTAL — NEVER MERGE.
//!
//! Responsibilities, deliberately minimal:
//!   * spawn and supervise the SEPARATE Rust core process (the UI never
//!     receives a process-spawn capability);
//!   * expose EXACTLY ONE command (`core_request`) that forwards a
//!     typed, size-bounded request to the core over framed stdio and
//!     returns the core's typed response;
//!   * grant the WebView no other Tauri command, no fs, no shell, no
//!     network, no secret access.
//!
//! The authority boundary under evaluation is the core, not this host.
#![forbid(unsafe_code)]
#![cfg_attr(all(not(debug_assertions), windows), windows_subsystem = "windows")]

use s05a_protocol as proto;
use s05a_protocol::json::Json;
use std::collections::BTreeMap;
use std::io::{BufReader, BufWriter};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::sync::Mutex;
use tauri::{Manager, State};

/// Supervised handle to the separate core process.
struct Core {
    inner: Mutex<CoreConn>,
}

struct CoreConn {
    child: Child,
    session: String,
    writer: BufWriter<ChildStdin>,
    reader: BufReader<ChildStdout>,
    next_id: u64,
}

impl CoreConn {
    fn spawn() -> std::io::Result<CoreConn> {
        // Locate the core binary next to the host executable (bundled
        // as a sidecar in a real build) or via an explicit env override
        // for the prototype. No shell, no string command construction.
        let core_path = std::env::var("S05A_CORE_BIN").unwrap_or_else(|_| {
            let mut p = std::env::current_exe().unwrap_or_default();
            p.set_file_name(if cfg!(windows) { "s05a-core.exe" } else { "s05a-core" });
            p.to_string_lossy().into_owned()
        });
        let fixtures = std::env::var("S05A_FIXTURES").unwrap_or_default();
        let output = std::env::var("S05A_OUTPUT").unwrap_or_default();
        let mut cmd = Command::new(core_path);
        if !fixtures.is_empty() {
            cmd.arg("--fixtures").arg(fixtures);
        }
        if !output.is_empty() {
            cmd.arg("--output").arg(output);
        }
        let mut child = cmd
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()?;
        let writer = BufWriter::new(child.stdin.take().expect("core stdin"));
        let mut reader = BufReader::new(child.stdout.take().expect("core stdout"));
        let hello = proto::read_frame(&mut reader)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "no-hello"))?;
        let hello = proto::json::parse(std::str::from_utf8(&hello).unwrap_or("{}"))
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "bad-hello"))?;
        let session = hello
            .get("session")
            .and_then(Json::as_str)
            .unwrap_or_default()
            .to_string();
        Ok(CoreConn { child, session, writer, reader, next_id: 1 })
    }

    fn forward(&mut self, op: &str, capability: Option<&str>, params: Json) -> Json {
        let id = self.next_id;
        self.next_id += 1;
        let mut m = BTreeMap::new();
        m.insert("v".to_string(), Json::Str(proto::PROTOCOL_VERSION.into()));
        m.insert("id".to_string(), Json::Num(id as f64));
        m.insert("session".to_string(), Json::Str(self.session.clone()));
        m.insert("capability".to_string(), capability.map(|c| Json::Str(c.into())).unwrap_or(Json::Null));
        m.insert("op".to_string(), Json::Str(op.into()));
        m.insert("params".to_string(), params);
        let body = proto::json::to_string(&Json::Obj(m));
        if proto::write_frame(&mut self.writer, body.as_bytes()).is_err() {
            return proto::error(id, "core-write-failed");
        }
        match proto::read_frame(&mut self.reader) {
            Ok(b) => proto::json::parse(std::str::from_utf8(&b).unwrap_or("{}"))
                .unwrap_or_else(|_| proto::error(id, "core-bad-response")),
            Err(_) => proto::error(id, "core-unavailable"),
        }
    }
}

/// The single exposed command. The WebView can invoke only this.
#[tauri::command]
fn core_request(
    core: State<Core>,
    op: String,
    capability: Option<String>,
    params_json: String,
) -> Result<String, String> {
    // Bound the untrusted UI payload before it reaches the core.
    if params_json.len() > 8192 {
        return Err("params-too-large".into());
    }
    let params = proto::json::parse(&params_json).map_err(|_| "malformed-params-json".to_string())?;
    let mut conn = core.inner.lock().map_err(|_| "core-lock-poisoned".to_string())?;
    let resp = conn.forward(&op, capability.as_deref(), params);
    Ok(proto::json::to_string(&resp))
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let conn = CoreConn::spawn()?;
            app.manage(Core { inner: Mutex::new(conn) });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![core_request])
        .run(tauri::generate_context!())
        .expect("error while running the S0.5A prototype host");
}
