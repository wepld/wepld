//! S0.5A IPC bench client. EXPERIMENTAL — NEVER MERGE.
//! Spawns the core exactly as the host does and measures handshake
//! latency, echo round-trips (p50/p95/p99), and malformed-rejection
//! latency. Prints one JSON document to stdout. Zero dependencies.
#![forbid(unsafe_code)]

use s05a_core::CoreClient;
use s05a_protocol as proto;
use s05a_protocol::json::Json;
use std::collections::BTreeMap;
use std::time::Instant;

fn main() {
    let bin = std::env::var("S05A_CORE_BIN").unwrap_or_else(|_| {
        let mut p = std::env::current_exe().expect("exe");
        p.set_file_name(if cfg!(windows) { "s05a-core.exe" } else { "s05a-core" });
        p.to_string_lossy().into_owned()
    });
    let iters: usize = std::env::var("S05A_BENCH_ITERS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(2000);

    let t0 = Instant::now();
    let mut client = CoreClient::spawn(&bin, None, None, &[]);
    let handshake_ms = t0.elapsed().as_secs_f64() * 1e3;

    let mut samples = Vec::with_capacity(iters);
    for i in 0..iters {
        let t = Instant::now();
        let r = client.request("echo", None, &format!(r#"{{"data":"ping-{i}"}}"#));
        let dt = t.elapsed().as_secs_f64() * 1e3;
        assert_eq!(proto::kind_of(&r), Some("Ok"), "echo failed: {r:?}");
        samples.push(dt);
    }
    samples.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let pct = |p: f64| samples[((samples.len() as f64 - 1.0) * p) as usize];

    let t = Instant::now();
    client.raw_send(b"{ this is not json");
    let rej = client.raw_recv().expect("rejection response");
    let malformed_reject_ms = t.elapsed().as_secs_f64() * 1e3;
    assert_eq!(proto::kind_of(&rej), Some("Error"));

    let mut rtt = BTreeMap::new();
    rtt.insert("p50".to_string(), Json::Num(pct(0.50)));
    rtt.insert("p95".to_string(), Json::Num(pct(0.95)));
    rtt.insert("p99".to_string(), Json::Num(pct(0.99)));
    rtt.insert("min".to_string(), Json::Num(*samples.first().unwrap()));
    rtt.insert("max".to_string(), Json::Num(*samples.last().unwrap()));

    let mut m = BTreeMap::new();
    m.insert("core_build".to_string(), Json::Str(client.core_build.clone()));
    m.insert("iterations".to_string(), Json::Num(iters as f64));
    m.insert("handshake_ms".to_string(), Json::Num(handshake_ms));
    m.insert("echo_rtt_ms".to_string(), Json::Obj(rtt));
    m.insert("malformed_reject_ms".to_string(), Json::Num(malformed_reject_ms));
    println!("{}", proto::json::to_string(&Json::Obj(m)));
}
