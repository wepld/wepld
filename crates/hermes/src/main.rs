//! Hermes skeleton (M0 Day 5): connect over WWP, heartbeat, execute a stub
//! phase, report `phase.result`, exit. Hermes is stateless — it owns
//! execution only; every fact worth keeping leaves through WWP.
//!
//! Test levers (env, used by the integration suite to exercise the Runtime's
//! failure handling): WEPLD_HERMES_MODE = echo (default) | die | mute | hang;
//! WEPLD_HEARTBEAT_MS = heartbeat interval.

use std::time::Duration;
use wepld_contracts::wwp::{Heartbeat, PhaseResult, PhaseStatus, WwpMessage};
use wepld_wwp::{send_to_core, worker_read_frame};

fn main() {
    let mode = std::env::var("WEPLD_HERMES_MODE").unwrap_or_else(|_| "echo".to_owned());
    let hb_ms: u64 = std::env::var("WEPLD_HEARTBEAT_MS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(2000);

    let mut stdin = std::io::stdin().lock();

    // The first frame must be attempt.start (lease acceptance).
    let start = match worker_read_frame(&mut stdin) {
        Ok(Some(f)) => match f.msg {
            WwpMessage::AttemptStart(s) => *s,
            _ => std::process::exit(2),
        },
        _ => std::process::exit(2),
    };
    let attempt_id = start.attempt_id.clone();

    if mode == "die" {
        // Simulates a crashed worker: no result, abnormal exit.
        std::process::exit(9);
    }

    if mode != "mute" {
        let hb_attempt = attempt_id.clone();
        std::thread::spawn(move || loop {
            let _ = send_to_core(WwpMessage::Heartbeat(Heartbeat {
                attempt_id: hb_attempt.clone(),
                progress: "working".to_owned(),
            }));
            std::thread::sleep(Duration::from_millis(hb_ms));
        });
    }

    match mode.as_str() {
        "echo" => {
            // Stub phase: deterministic "work", then a schema-shaped result.
            std::thread::sleep(Duration::from_millis(3 * hb_ms / 2));
            let _ = send_to_core(WwpMessage::PhaseResult(PhaseResult {
                attempt_id,
                status: PhaseStatus::Succeeded,
                outputs: vec![],
                evidence: vec![],
                summary: serde_json::json!({
                    "schema": "phase_summary.v1",
                    "what": format!("stub {} phase executed by hermes", start.phase),
                    "decisions_made": [],
                    "uncertainties": []
                }),
                next_hint: None,
            }));
        }
        // mute: no heartbeats, wait for cancel (exercises the watchdog).
        // hang: heartbeats forever, wait for cancel (exercises cancellation).
        "mute" | "hang" => loop {
            match worker_read_frame(&mut stdin) {
                Ok(Some(f)) => {
                    if let WwpMessage::AttemptCancel(_) = f.msg {
                        let _ = send_to_core(WwpMessage::PhaseResult(PhaseResult {
                            attempt_id,
                            status: PhaseStatus::Cancelled,
                            outputs: vec![],
                            evidence: vec![],
                            summary: serde_json::json!({
                                "schema": "phase_summary.v1",
                                "what": "cancelled cooperatively"
                            }),
                            next_hint: None,
                        }));
                        return;
                    }
                }
                _ => return,
            }
        },
        _ => std::process::exit(2),
    }
}
