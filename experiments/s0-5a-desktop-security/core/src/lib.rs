//! Test/bench client helpers for the S0.5A prototype core.
//! EXPERIMENTAL — NEVER MERGE. Zero third-party dependencies.
#![forbid(unsafe_code)]

use s05a_protocol as proto;
use s05a_protocol::json::Json;
use std::collections::BTreeMap;
use std::io::{BufReader, BufWriter, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

pub struct CoreClient {
    pub child: Child,
    pub session: String,
    pub core_build: String,
    writer: BufWriter<ChildStdin>,
    reader: BufReader<ChildStdout>,
    next_id: u64,
}

impl CoreClient {
    /// Spawn the core exactly as the host would: child process, piped
    /// stdio, no shell, no inherited command line beyond typed args.
    pub fn spawn(bin: &str, fixtures: Option<&str>, output: Option<&str>, drop: &[&str]) -> Self {
        let mut cmd = Command::new(bin);
        if let Some(f) = fixtures {
            cmd.arg("--fixtures").arg(f);
        }
        if let Some(o) = output {
            cmd.arg("--output").arg(o);
        }
        for d in drop {
            cmd.arg("--drop-cap").arg(d);
        }
        let mut child = cmd
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .expect("spawn core");
        let writer = BufWriter::new(child.stdin.take().expect("stdin"));
        let mut reader = BufReader::new(child.stdout.take().expect("stdout"));
        let hello = proto::read_frame(&mut reader).expect("hello frame");
        let hello = proto::json::parse(std::str::from_utf8(&hello).unwrap()).expect("hello json");
        assert_eq!(proto::kind_of(&hello), Some("Hello"), "expected Hello, got {hello:?}");
        let session = hello.get("session").and_then(Json::as_str).unwrap().to_string();
        let core_build = hello.get("core_build").and_then(Json::as_str).unwrap().to_string();
        CoreClient { child, session, core_build, writer, reader, next_id: 1 }
    }

    pub fn raw_send(&mut self, body: &[u8]) {
        proto::write_frame(&mut self.writer, body).expect("write frame");
    }

    /// Best-effort send that does not panic on a closed pipe — used to
    /// observe crash/closure behavior.
    pub fn try_send(&mut self, body: &[u8]) -> std::io::Result<()> {
        proto::write_frame(&mut self.writer, body)
    }

    /// Write a raw 4-byte length prefix with no body — used to test the
    /// oversized-frame rejection path.
    pub fn raw_send_prefix(&mut self, prefix: [u8; 4]) {
        self.writer.write_all(&prefix).expect("prefix");
        self.writer.flush().expect("flush");
    }

    pub fn raw_recv(&mut self) -> Result<Json, String> {
        let body = proto::read_frame(&mut self.reader).map_err(|e| format!("{e:?}"))?;
        proto::json::parse(std::str::from_utf8(&body).map_err(|e| e.to_string())?)
    }

    /// Build and send a request from raw JSON text for params so tests
    /// can exercise strict decoding precisely.
    pub fn request(&mut self, op: &str, capability: Option<&str>, params_json: &str) -> Json {
        self.request_with(op, capability, params_json, None, None)
    }

    pub fn request_with(
        &mut self,
        op: &str,
        capability: Option<&str>,
        params_json: &str,
        session_override: Option<&str>,
        version_override: Option<&str>,
    ) -> Json {
        let id = self.next_id;
        self.next_id += 1;
        let params = proto::json::parse(params_json).expect("params json");
        let mut m = BTreeMap::new();
        m.insert("v".to_string(), Json::Str(version_override.unwrap_or(proto::PROTOCOL_VERSION).into()));
        m.insert("id".to_string(), Json::Num(id as f64));
        m.insert("session".to_string(), Json::Str(session_override.unwrap_or(&self.session).into()));
        m.insert("capability".to_string(), capability.map(|c| Json::Str(c.into())).unwrap_or(Json::Null));
        m.insert("op".to_string(), Json::Str(op.into()));
        m.insert("params".to_string(), params);
        self.raw_send(proto::json::to_string(&Json::Obj(m)).as_bytes());
        self.raw_recv().expect("response")
    }
}

impl Drop for CoreClient {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}
