//! Hermes (M0 Day 6): connect over WWP, heartbeat, optionally consult a
//! reasoning provider through the Runtime's gateway, report `phase.result`,
//! exit. Hermes is stateless and deterministic-first (IADR-0007 §1): the
//! `echo` mode completes a phase with ZERO brain calls — a normal execution,
//! not a degraded one. Reasoning is requested only when the phase benefits
//! (`brain` mode here; real heuristics arrive with real phases).
//!
//! Test levers (env, used by the integration suite): WEPLD_HERMES_MODE =
//! echo (default) | brain | die | mute | hang; WEPLD_HEARTBEAT_MS.

use std::time::Duration;
use wepld_contracts::brain::{BrainResult, BrainStatus};
use wepld_contracts::wwp::{
    AttemptStart, BrainRequest, Heartbeat, PhaseResult, PhaseStatus, WwpMessage,
};
use wepld_wwp::{send_request_to_core, send_to_core, worker_read_incoming, Incoming};

fn main() {
    let mode = std::env::var("WEPLD_HERMES_MODE").unwrap_or_else(|_| "echo".to_owned());
    let hb_ms: u64 = std::env::var("WEPLD_HEARTBEAT_MS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(2000);

    let mut stdin = std::io::stdin().lock();

    // The first frame must be attempt.start (lease acceptance).
    let start = match worker_read_incoming(&mut stdin) {
        Ok(Some(Incoming::Message(f))) => match f.msg {
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
            // Deterministic phase: zero brain calls, first-class execution.
            std::thread::sleep(Duration::from_millis(3 * hb_ms / 2));
            finish(
                attempt_id,
                PhaseStatus::Succeeded,
                serde_json::json!({
                    "schema": "phase_summary.v1",
                    "what": format!("stub {} phase executed by hermes", start.phase),
                    "decisions_made": [],
                    "uncertainties": []
                }),
            );
        }
        "brain" => run_brain_phase(&mut stdin, start),
        // mute: no heartbeats, wait for cancel (exercises the watchdog).
        // hang: heartbeats forever, wait for cancel (exercises cancellation).
        "mute" | "hang" => loop {
            match worker_read_incoming(&mut stdin) {
                Ok(Some(Incoming::Message(f))) => {
                    if let WwpMessage::AttemptCancel(_) = f.msg {
                        finish(
                            attempt_id,
                            PhaseStatus::Cancelled,
                            serde_json::json!({
                                "schema": "phase_summary.v1",
                                "what": "cancelled cooperatively"
                            }),
                        );
                        return;
                    }
                }
                _ => return,
            }
        },
        _ => std::process::exit(2),
    }
}

/// A phase where Hermes determines reasoning would improve the work: one
/// gateway round-trip, then a result grounded in the (recorded) answer.
fn run_brain_phase(stdin: &mut std::io::StdinLock<'static>, start: AttemptStart) {
    let attempt_id = start.attempt_id.clone();
    let _ = send_request_to_core(
        WwpMessage::BrainRequest(BrainRequest {
            attempt_id: attempt_id.clone(),
            intent: "stub_step".to_owned(),
            pack_ref: start.context_pack_ref.clone(),
            output_schema_id: "phase_summary.v1".to_owned(),
            budget_hint: None,
        }),
        1,
    );
    loop {
        match worker_read_incoming(stdin) {
            Ok(Some(Incoming::Response(r))) if r.id == 1 => {
                let Ok(result) = serde_json::from_value::<BrainResult>(r.result) else {
                    std::process::exit(2);
                };
                match result.status {
                    BrainStatus::Ok => {
                        finish(attempt_id, PhaseStatus::Succeeded, result.output);
                    }
                    other => {
                        finish(
                            attempt_id,
                            PhaseStatus::Failed,
                            serde_json::json!({
                                "schema": "phase_summary.v1",
                                "what": format!(
                                    "reasoning unavailable: {other:?} — {}",
                                    result.reason.unwrap_or_default()
                                )
                            }),
                        );
                    }
                }
                return;
            }
            Ok(Some(Incoming::Message(f))) => {
                if let WwpMessage::AttemptCancel(_) = f.msg {
                    finish(
                        attempt_id,
                        PhaseStatus::Cancelled,
                        serde_json::json!({
                            "schema": "phase_summary.v1",
                            "what": "cancelled while awaiting reasoning"
                        }),
                    );
                    return;
                }
            }
            _ => std::process::exit(2),
        }
    }
}

fn finish(attempt_id: String, status: PhaseStatus, summary: serde_json::Value) {
    let _ = send_to_core(WwpMessage::PhaseResult(PhaseResult {
        attempt_id,
        status,
        outputs: vec![],
        evidence: vec![],
        summary,
        next_hint: None,
    }));
}
