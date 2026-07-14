//! Hermes — WePLD's flagship WWP worker runtime (M0 skeleton). Stateless and
//! deterministic-first (IADR-0007 §1): it owns engineering execution and
//! consults a reasoning provider through the Runtime's gateway only when a
//! phase benefits. Hermes never owns persistence, the ledger, or Chronicle;
//! it receives Context Packs and produces Results.
//!
//! Modes (env WEPLD_HERMES_MODE):
//!   auto  (default) — dispatch on the phase: plan → request a plan;
//!                     build → request edits and apply them to the worktree;
//!                     other → request a phase summary.
//!   echo  — deterministic phase, zero brain calls.
//!   brain — single stub brain round-trip (used by brain_tests).
//!   die / mute / hang — failure-mode levers for lifecycle tests.

use std::path::Path;
use std::time::Duration;
use wepld_contracts::brain::{BrainResult, BrainStatus};
use wepld_contracts::validation::WorkspaceRelativePath;
use wepld_contracts::wwp::{
    AttemptStart, BrainRequest, Heartbeat, PhaseResult, PhaseStatus, WwpMessage,
};
use wepld_wwp::{send_request_to_core, send_to_core, worker_read_incoming, Incoming};

fn main() {
    let mode = std::env::var("WEPLD_HERMES_MODE").unwrap_or_else(|_| "auto".to_owned());
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

    if mode == "garbage" {
        // Protocol violation: emit non-frame bytes on stdout.
        use std::io::Write as _;
        let mut out = std::io::stdout().lock();
        let _ = out.write_all(b"this is not a valid wwp frame\n\n");
        let _ = out.flush();
        std::thread::sleep(Duration::from_secs(30));
        return;
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
        "brain" => brain_phase(
            &mut stdin,
            &start,
            "stub_step",
            "phase_summary.v1",
            Post::PassSummary,
        ),
        "auto" => match start.phase.as_str() {
            "plan" => brain_phase(&mut stdin, &start, "plan", "plan.v1", Post::Plan),
            "build" => brain_phase(
                &mut stdin,
                &start,
                "build",
                "builder_step.v1",
                Post::ApplyEdits,
            ),
            _ => brain_phase(
                &mut stdin,
                &start,
                "stub_step",
                "phase_summary.v1",
                Post::PassSummary,
            ),
        },
        // brainspam: flood the Core with brain.requests (exercises the
        // Core-side brain-call budget enforcement).
        "brainspam" => {
            for i in 0..1000u64 {
                let _ = send_request_to_core(
                    WwpMessage::BrainRequest(BrainRequest {
                        attempt_id: attempt_id.clone(),
                        intent: "stub_step".to_owned(),
                        pack_ref: start.context_pack_ref.clone(),
                        output_schema_id: "phase_summary.v1".to_owned(),
                        budget_hint: None,
                    }),
                    i + 1,
                );
                std::thread::sleep(Duration::from_millis(1));
            }
        }
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

/// What Hermes does with a successful brain result.
enum Post {
    /// The brain output *is* the phase summary (a `phase_summary.v1`).
    PassSummary,
    /// A plan was produced (captured by the Core as an artifact); summarize.
    Plan,
    /// The output carries file edits; apply them to the worktree, then summarize.
    ApplyEdits,
}

/// One gateway round-trip, then the post-brain action.
fn brain_phase(
    stdin: &mut std::io::StdinLock<'static>,
    start: &AttemptStart,
    intent: &str,
    schema: &str,
    post: Post,
) {
    let attempt_id = start.attempt_id.clone();
    let _ = send_request_to_core(
        WwpMessage::BrainRequest(BrainRequest {
            attempt_id: attempt_id.clone(),
            intent: intent.to_owned(),
            pack_ref: start.context_pack_ref.clone(),
            output_schema_id: schema.to_owned(),
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
                        let summary = match post {
                            Post::PassSummary => result.output,
                            Post::Plan => serde_json::json!({
                                "schema": "phase_summary.v1",
                                "what": "proposed a plan",
                                "decisions_made": [],
                                "uncertainties": []
                            }),
                            Post::ApplyEdits => match apply_worktree_edits(start, &result.output) {
                                Ok(n) => serde_json::json!({
                                    "schema": "phase_summary.v1",
                                    "what": format!("applied {n} edit(s)"),
                                    "decisions_made": [],
                                    "uncertainties": []
                                }),
                                Err(e) => {
                                    finish(
                                        attempt_id,
                                        PhaseStatus::Failed,
                                        serde_json::json!({
                                            "schema": "phase_summary.v1",
                                            "what": format!("edit application failed: {e}")
                                        }),
                                    );
                                    return;
                                }
                            },
                        };
                        finish(attempt_id, PhaseStatus::Succeeded, summary);
                    }
                    other => finish(
                        attempt_id,
                        PhaseStatus::Failed,
                        serde_json::json!({
                            "schema": "phase_summary.v1",
                            "what": format!(
                                "reasoning unavailable: {other:?} — {}",
                                result.reason.unwrap_or_default()
                            )
                        }),
                    ),
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

/// Apply `{ edits: [ { path, content } ] }` under the single writable path the
/// envelope granted. Every edit path is a validated [`WorkspaceRelativePath`]
/// (no absolute, drive, or `..` component — model output is untrusted) and the
/// write is symlink-safe (Blocker 1): no existing component may be a symlink,
/// so a pre-existing link in the worktree cannot redirect a write outside it.
fn apply_worktree_edits(start: &AttemptStart, output: &serde_json::Value) -> Result<usize, String> {
    let root = start
        .envelope
        .fs
        .write
        .first()
        .ok_or("no writable path in envelope")?;
    let root = Path::new(root);
    let edits = output
        .get("edits")
        .and_then(|e| e.as_array())
        .ok_or("builder output has no edits array")?;
    let mut count = 0;
    for edit in edits {
        let rel_raw = edit
            .get("path")
            .and_then(|p| p.as_str())
            .ok_or("edit missing path")?;
        let content = edit
            .get("content")
            .and_then(|c| c.as_str())
            .ok_or("edit missing content")?;
        let rel = WorkspaceRelativePath::parse(rel_raw).map_err(|e| e.to_string())?;
        write_confined(root, &rel, content)?;
        count += 1;
    }
    Ok(count)
}

/// Write `content` at `rel` under `root` without ever following a symlink out
/// of the worktree. Walks the validated relative path component by component:
/// any *existing* component that is a symlink is refused (parent or final
/// target), missing directories are created one level at a time (so a created
/// dir is never a symlink), and the final regular file is written in place.
/// The path is lexically confined to `root` by [`WorkspaceRelativePath`]; the
/// only residual gap is a concurrent filesystem swap, which the model — which
/// supplies path strings, not concurrent I/O — cannot perform.
fn write_confined(root: &Path, rel: &WorkspaceRelativePath, content: &str) -> Result<(), String> {
    let comps: Vec<_> = rel.as_path().components().collect();
    let mut cur = root.to_path_buf();
    for (i, comp) in comps.iter().enumerate() {
        let std::path::Component::Normal(name) = comp else {
            return Err("validated path yielded a non-normal component".to_owned());
        };
        cur.push(name);
        let is_last = i + 1 == comps.len();
        match std::fs::symlink_metadata(&cur) {
            Ok(md) => {
                if md.file_type().is_symlink() {
                    return Err(format!(
                        "edit path component is a symlink: {}",
                        cur.display()
                    ));
                }
                if is_last {
                    if md.is_dir() {
                        return Err(format!("edit target is a directory: {}", cur.display()));
                    }
                } else if !md.is_dir() {
                    return Err(format!(
                        "edit path parent is not a directory: {}",
                        cur.display()
                    ));
                }
            }
            Err(_) => {
                // Does not exist yet: create only intermediate directories.
                if !is_last {
                    std::fs::create_dir(&cur)
                        .map_err(|e| format!("create dir {}: {e}", cur.display()))?;
                }
            }
        }
    }
    std::fs::write(&cur, content).map_err(|e| format!("write {}: {e}", cur.display()))
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

#[cfg(test)]
mod edit_tests {
    use super::{write_confined, WorkspaceRelativePath};

    fn wrp(s: &str) -> WorkspaceRelativePath {
        WorkspaceRelativePath::parse(s).unwrap()
    }

    #[test]
    fn writes_a_nested_file_under_root() {
        let root = tempfile::tempdir().unwrap();
        write_confined(root.path(), &wrp("src/generated/file.rs"), "X").unwrap();
        assert_eq!(
            std::fs::read_to_string(root.path().join("src/generated/file.rs")).unwrap(),
            "X"
        );
    }

    #[cfg(unix)]
    #[test]
    fn refuses_a_parent_symlink_and_creates_no_external_file() {
        let outside = tempfile::tempdir().unwrap();
        let root = tempfile::tempdir().unwrap();
        std::os::unix::fs::symlink(outside.path(), root.path().join("link")).unwrap();
        let err = write_confined(root.path(), &wrp("link/evil.txt"), "X").unwrap_err();
        assert!(err.contains("symlink"), "{err}");
        assert!(
            !outside.path().join("evil.txt").exists(),
            "no file may be created outside the worktree"
        );
    }

    #[cfg(unix)]
    #[test]
    fn refuses_a_final_symlink_and_does_not_write_through_it() {
        let outside = tempfile::tempdir().unwrap();
        let target = outside.path().join("secret.txt");
        let root = tempfile::tempdir().unwrap();
        std::os::unix::fs::symlink(&target, root.path().join("main.rs")).unwrap();
        let err = write_confined(root.path(), &wrp("main.rs"), "X").unwrap_err();
        assert!(err.contains("symlink"), "{err}");
        assert!(
            !target.exists(),
            "must not write through a symlink to outside"
        );
    }
}
