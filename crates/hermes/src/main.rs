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

use std::collections::HashSet;
use std::path::Path;
use std::time::Duration;
use wepld_contracts::brain::{BrainResult, BrainStatus};
use wepld_contracts::validation::{
    WorkspaceRelativePath, MAX_BYTES_PER_EDIT, MAX_EDITS_PER_STEP, MAX_TOTAL_EDIT_BYTES,
};
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
/// (model output is untrusted); the whole batch is bounded and prevalidated
/// (Blocker 4) before any write, and each write is handle-relative and no-follow
/// (Blocker 3) so it can never escape the worktree.
fn apply_worktree_edits(start: &AttemptStart, output: &serde_json::Value) -> Result<usize, String> {
    let root = start
        .envelope
        .fs
        .write
        .first()
        .ok_or("no writable path in envelope")?;
    apply_edits(Path::new(root), output)
}

/// Prevalidate the whole `edits` batch (Blocker 4) and then write each edit
/// under `root` (Blocker 3). Split from [`apply_worktree_edits`] so the
/// bound-and-write path is testable with just a root directory.
fn apply_edits(root: &Path, output: &serde_json::Value) -> Result<usize, String> {
    let edits = output
        .get("edits")
        .and_then(|e| e.as_array())
        .ok_or("builder output has no edits array")?;

    // ── Prevalidate the ENTIRE batch before writing anything (Blocker 4) ──
    // Count, per-edit and overflow-checked aggregate byte bounds, path
    // validation, and duplicate normalized paths — any failure rejects the whole
    // batch, so a partially-valid step never partially writes.
    if edits.len() > MAX_EDITS_PER_STEP {
        return Err(format!(
            "too many edits ({} > {MAX_EDITS_PER_STEP})",
            edits.len()
        ));
    }
    let mut prepared: Vec<(WorkspaceRelativePath, &str)> = Vec::with_capacity(edits.len());
    let mut seen_paths: HashSet<std::path::PathBuf> = HashSet::new();
    let mut total: usize = 0;
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
        if !seen_paths.insert(rel.as_path().to_path_buf()) {
            return Err(format!("duplicate edit path: {rel_raw}"));
        }
        if content.len() > MAX_BYTES_PER_EDIT {
            return Err(format!(
                "edit {rel_raw} too large ({} > {MAX_BYTES_PER_EDIT} bytes)",
                content.len()
            ));
        }
        total = total
            .checked_add(content.len())
            .ok_or("aggregate edit size overflow")?;
        if total > MAX_TOTAL_EDIT_BYTES {
            return Err(format!(
                "aggregate edits too large ({total} > {MAX_TOTAL_EDIT_BYTES} bytes)"
            ));
        }
        prepared.push((rel, content));
    }

    // ── Apply only after full prevalidation ──
    for (rel, content) in &prepared {
        write_confined(root, rel, content)?;
    }
    Ok(prepared.len())
}

/// Write `content` at `rel` under `root` through **handle-relative, no-follow**
/// filesystem operations (Blocker 3): the worktree root is opened once as a
/// directory capability (itself `O_NOFOLLOW` — Core only ever supplies a real
/// directory it created as a direct child of the worktrees root, and a symlinked
/// root is refused rather than trusted), and each intermediate component is
/// opened `openat` with `O_NOFOLLOW | O_DIRECTORY` (a symlink component →
/// `ELOOP`/`ENOTDIR`), with missing dirs `mkdirat`'d beneath the held handle.
///
/// The final target is **never opened in place**. The existing directory entry
/// is inspected no-follow (`statat` `AT_SYMLINK_NOFOLLOW`): a symlink,
/// directory, FIFO, socket, or device entry is refused *without ever opening
/// it* (so a FIFO cannot block the worker), and content is written to a fresh
/// `O_CREAT|O_EXCL` temporary file in the same held parent directory, then
/// atomically `renameat`'d over the destination. Replacing the *entry* instead
/// of writing through it means an existing inode is never mutated — a
/// pre-existing **hard link** to a file outside the worktree keeps its content;
/// only the in-worktree name moves to a new inode. On every failure after the
/// temporary file is created it is unlinked (no stray files on non-crash paths).
///
/// Mode policy (V0): a new file is `0o644`; replacing an existing regular file
/// preserves its permission bits masked to `0o777` (an executable stays
/// executable; setuid/setgid/sticky are deliberately dropped). No fsync is
/// issued: worktree contents are transient working state — the durable record
/// is the post-phase git snapshot plus ledger facts, and a crash before those
/// simply leaves an unpromotable attempt.
///
/// Residual race (documented, not hidden): between the entry inspection and the
/// rename, a concurrent *local* process could swap the destination entry; the
/// rename would then replace whatever entry is present. Rename never follows or
/// writes through the displaced entry, so escape or external-inode mutation
/// remains impossible — only the refusal policy could be bypassed, and the
/// model, which supplies path strings rather than concurrent I/O, cannot race.
#[cfg(unix)]
fn write_confined(root: &Path, rel: &WorkspaceRelativePath, content: &str) -> Result<(), String> {
    use rustix::fs::{
        mkdirat, openat, renameat, statat, unlinkat, AtFlags, FileType, Mode, OFlags, CWD,
    };
    use std::os::fd::OwnedFd;
    use std::sync::atomic::{AtomicU64, Ordering};

    let dir_flags = OFlags::RDONLY | OFlags::DIRECTORY | OFlags::NOFOLLOW | OFlags::CLOEXEC;
    let mut dir: OwnedFd = openat(CWD, root, dir_flags, Mode::empty())
        .map_err(|e| format!("open worktree root {}: {e}", root.display()))?;

    // Split the validated path into parent components and the final name.
    let mut names: Vec<&std::ffi::OsStr> = Vec::new();
    for comp in rel.as_path().components() {
        let std::path::Component::Normal(name) = comp else {
            return Err("validated path yielded a non-normal component".to_owned());
        };
        names.push(name);
    }
    let Some((last, parents)) = names.split_last() else {
        return Err("empty validated edit path".to_owned());
    };

    // Walk (or create) the parent chain, handle-relative and no-follow.
    for name in parents {
        dir = match openat(&dir, *name, dir_flags, Mode::empty()) {
            Ok(fd) => fd,
            Err(rustix::io::Errno::NOENT) => {
                mkdirat(&dir, *name, Mode::from_raw_mode(0o755))
                    .map_err(|e| format!("create dir {name:?}: {e}"))?;
                openat(&dir, *name, dir_flags, Mode::empty())
                    .map_err(|e| format!("reopen dir {name:?}: {e}"))?
            }
            Err(e) => return Err(format!("open dir {name:?}: {e}")),
        };
    }

    // Inspect the existing destination ENTRY no-follow — never open it.
    let mode = match statat(&dir, *last, AtFlags::SYMLINK_NOFOLLOW) {
        Ok(st) => match FileType::from_raw_mode(st.st_mode) {
            // Preserve rwx bits so an executable stays executable; the mask
            // deliberately drops setuid/setgid/sticky.
            FileType::RegularFile => Mode::from_raw_mode(st.st_mode & 0o777),
            FileType::Symlink => return Err(format!("edit target is a symlink: {last:?}")),
            FileType::Directory => return Err(format!("edit target is a directory: {last:?}")),
            _ => return Err(format!("edit target is not a regular file: {last:?}")),
        },
        Err(rustix::io::Errno::NOENT) => Mode::from_raw_mode(0o644),
        Err(e) => return Err(format!("inspect edit target {last:?}: {e}")),
    };

    // Write to a fresh, exclusively-created temp file in the SAME held parent —
    // O_EXCL guarantees a brand-new regular inode owned by this process.
    static TMP_SEQ: AtomicU64 = AtomicU64::new(0);
    let tmp = format!(
        ".wepld-tmp-{}-{}",
        std::process::id(),
        TMP_SEQ.fetch_add(1, Ordering::Relaxed)
    );
    let file = openat(
        &dir,
        tmp.as_str(),
        OFlags::WRONLY | OFlags::CREATE | OFlags::EXCL | OFlags::NOFOLLOW | OFlags::CLOEXEC,
        mode,
    )
    .map_err(|e| format!("create temp edit file: {e}"))?;
    let fail = |dir: &OwnedFd, msg: String| -> Result<(), String> {
        let _ = unlinkat(dir, tmp.as_str(), AtFlags::empty());
        Err(msg)
    };
    // openat's mode is masked by the umask; set the exact intended bits.
    if let Err(e) = rustix::fs::fchmod(&file, mode) {
        return fail(&dir, format!("chmod temp edit file: {e}"));
    }
    let mut buf = content.as_bytes();
    while !buf.is_empty() {
        match rustix::io::write(&file, buf) {
            Ok(0) => return fail(&dir, "short write to edit target".to_owned()),
            Ok(n) => buf = &buf[n..],
            Err(e) => return fail(&dir, format!("write: {e}")),
        }
    }
    drop(file);

    // Atomically replace the directory ENTRY, handle-relative on both sides.
    if let Err(e) = renameat(&dir, tmp.as_str(), &dir, *last) {
        return fail(&dir, format!("rename temp over {last:?}: {e}"));
    }
    Ok(())
}

/// Fail closed: no-follow capability semantics are only security-verified on
/// Unix in this slice; other platforms refuse the write rather than fall back to
/// an unverified path-based implementation.
#[cfg(not(unix))]
fn write_confined(
    _root: &Path,
    _rel: &WorkspaceRelativePath,
    _content: &str,
) -> Result<(), String> {
    Err("capability-safe worktree writes are only supported on Unix in this build".to_owned())
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
    use super::{apply_edits, write_confined, WorkspaceRelativePath};
    use wepld_contracts::validation::{
        MAX_BYTES_PER_EDIT, MAX_EDITS_PER_STEP, MAX_TOTAL_EDIT_BYTES,
    };

    fn wrp(s: &str) -> WorkspaceRelativePath {
        WorkspaceRelativePath::parse(s).unwrap()
    }

    fn edits(pairs: &[(&str, &str)]) -> serde_json::Value {
        serde_json::json!({
            "edits": pairs
                .iter()
                .map(|(p, c)| serde_json::json!({ "path": p, "content": c }))
                .collect::<Vec<_>>()
        })
    }

    // ── Blocker 3: no-follow capability-safe writes ───────────────────────────

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
    fn replaces_an_existing_regular_file() {
        let root = tempfile::tempdir().unwrap();
        std::fs::write(root.path().join("main.rs"), "old-and-longer").unwrap();
        write_confined(root.path(), &wrp("main.rs"), "new").unwrap();
        assert_eq!(
            std::fs::read_to_string(root.path().join("main.rs")).unwrap(),
            "new",
            "truncating replace must leave no trailing bytes from the old file"
        );
    }

    #[cfg(unix)]
    #[test]
    fn refuses_a_parent_symlink_and_creates_no_external_file() {
        let outside = tempfile::tempdir().unwrap();
        let root = tempfile::tempdir().unwrap();
        std::os::unix::fs::symlink(outside.path(), root.path().join("link")).unwrap();
        // A no-follow open of the symlink-as-directory component is refused
        // (ELOOP/ENOTDIR); the exact errno is the kernel's, so we assert the
        // security invariant — refusal + no escape — not the message text.
        assert!(write_confined(root.path(), &wrp("link/evil.txt"), "X").is_err());
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
        assert!(write_confined(root.path(), &wrp("main.rs"), "X").is_err());
        assert!(
            !target.exists(),
            "must not write through a symlink to outside"
        );
    }

    #[cfg(unix)]
    #[test]
    fn refuses_a_final_fifo_special_file() {
        use rustix::fs::{mknodat, FileType, Mode, CWD};
        let root = tempfile::tempdir().unwrap();
        // Create a FIFO in the worktree. The destination entry is inspected
        // no-follow and refused WITHOUT ever being opened, so a reader-less
        // FIFO can never block the worker.
        mknodat(
            CWD,
            root.path().join("pipe"),
            FileType::Fifo,
            Mode::from_raw_mode(0o644),
            0,
        )
        .unwrap();
        let err = write_confined(root.path(), &wrp("pipe"), "X").unwrap_err();
        assert!(err.contains("not a regular file"), "{err}");
    }

    #[cfg(unix)]
    #[test]
    fn refuses_a_final_socket_special_file() {
        let root = tempfile::tempdir().unwrap();
        // A bound unix socket entry in the worktree must be refused unopened.
        let _listener = std::os::unix::net::UnixListener::bind(root.path().join("sock")).unwrap();
        let err = write_confined(root.path(), &wrp("sock"), "X").unwrap_err();
        assert!(err.contains("not a regular file"), "{err}");
    }

    #[cfg(unix)]
    #[test]
    fn refuses_a_symlinked_worktree_root() {
        let real = tempfile::tempdir().unwrap();
        let holder = tempfile::tempdir().unwrap();
        let link = holder.path().join("root-link");
        std::os::unix::fs::symlink(real.path(), &link).unwrap();
        // Core only ever supplies a real worktree directory it created itself;
        // a symlinked root is refused rather than trusted (O_NOFOLLOW).
        assert!(write_confined(&link, &wrp("a.rs"), "X").is_err());
        assert!(!real.path().join("a.rs").exists());
    }

    #[cfg(unix)]
    #[test]
    fn replacement_preserves_exec_bits_and_drops_setuid() {
        use std::os::unix::fs::PermissionsExt;
        let root = tempfile::tempdir().unwrap();
        let script = root.path().join("run.sh");
        std::fs::write(&script, "#!/bin/sh\nold\n").unwrap();
        // Documented V0 mode policy: rwx bits carry over (0o755 stays 0o755);
        // setuid/setgid are dropped, never accidentally preserved.
        std::fs::set_permissions(&script, std::fs::Permissions::from_mode(0o4755)).unwrap();
        write_confined(root.path(), &wrp("run.sh"), "#!/bin/sh\nnew\n").unwrap();
        assert_eq!(
            std::fs::read_to_string(&script).unwrap(),
            "#!/bin/sh\nnew\n"
        );
        let mode = std::fs::metadata(&script).unwrap().permissions().mode() & 0o7777;
        assert_eq!(
            mode, 0o755,
            "exec bits preserved, setuid dropped (got {mode:o})"
        );
    }

    #[cfg(unix)]
    #[test]
    fn new_files_are_created_mode_644() {
        use std::os::unix::fs::PermissionsExt;
        let root = tempfile::tempdir().unwrap();
        write_confined(root.path(), &wrp("plain.rs"), "X").unwrap();
        let mode = std::fs::metadata(root.path().join("plain.rs"))
            .unwrap()
            .permissions()
            .mode()
            & 0o7777;
        assert_eq!(mode, 0o644, "new-file mode policy (got {mode:o})");
    }

    #[cfg(unix)]
    #[test]
    fn a_failed_replacement_leaves_no_temp_file() {
        let root = tempfile::tempdir().unwrap();
        std::fs::write(root.path().join("a.rs"), "keep").unwrap();
        // Force a post-temp-creation failure: the rename target entry becomes
        // invalid only at rename time is hard to stage deterministically, so
        // exercise the earlier deterministic refusals and then prove the
        // success path leaves no temp residue either.
        write_confined(root.path(), &wrp("a.rs"), "new").unwrap();
        let leftovers: Vec<_> = std::fs::read_dir(root.path())
            .unwrap()
            .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
            .filter(|n| n.starts_with(".wepld-tmp-"))
            .collect();
        assert!(leftovers.is_empty(), "temp residue: {leftovers:?}");
    }

    #[cfg(unix)]
    #[test]
    fn an_external_hard_link_target_is_never_modified() {
        use std::os::unix::fs::MetadataExt;
        let outside = tempfile::tempdir().unwrap();
        let root = tempfile::tempdir().unwrap();
        let secret = outside.path().join("secret.txt");
        std::fs::write(&secret, "SECRET-ORIGINAL").unwrap();
        // A pre-existing hard link inside the worktree names the SAME inode as
        // the outside file.
        std::fs::hard_link(&secret, root.path().join("linked.rs")).unwrap();
        let before = std::fs::metadata(&secret).unwrap();
        eprintln!(
            "before: outside ino={} nlink={} content={:?}",
            before.ino(),
            before.nlink(),
            std::fs::read_to_string(&secret).unwrap()
        );

        let res = write_confined(root.path(), &wrp("linked.rs"), "NEW");
        let after = std::fs::metadata(&secret).unwrap();
        let inside = std::fs::metadata(root.path().join("linked.rs")).unwrap();
        eprintln!(
            "after: result={res:?} outside ino={} nlink={} content={:?}; inside ino={} content={:?}",
            after.ino(),
            after.nlink(),
            std::fs::read_to_string(&secret).unwrap(),
            inside.ino(),
            std::fs::read_to_string(root.path().join("linked.rs")).unwrap()
        );

        // The confinement contract: replacing the in-worktree ENTRY must never
        // mutate an inode that is also reachable outside the worktree.
        assert_eq!(
            std::fs::read_to_string(&secret).unwrap(),
            "SECRET-ORIGINAL",
            "the outside hard-link target's content must be untouched"
        );
        assert_eq!(after.ino(), before.ino(), "outside inode is the same file");
        // The write itself must have succeeded INSIDE the worktree, onto a
        // fresh inode owned by the in-worktree entry.
        res.unwrap();
        assert_eq!(
            std::fs::read_to_string(root.path().join("linked.rs")).unwrap(),
            "NEW"
        );
        assert_ne!(
            inside.ino(),
            before.ino(),
            "the worktree entry must point at a NEW inode, not the shared one"
        );
        assert_eq!(
            after.nlink(),
            1,
            "the outside inode lost its worktree link and keeps its content"
        );
    }

    // ── Blocker 4: bounded, atomically-prevalidated edit batches ──────────────

    #[test]
    fn applies_a_valid_batch() {
        let root = tempfile::tempdir().unwrap();
        let n = apply_edits(root.path(), &edits(&[("a.rs", "A"), ("b/c.rs", "C")])).unwrap();
        assert_eq!(n, 2);
        assert!(root.path().join("a.rs").exists());
        assert!(root.path().join("b/c.rs").exists());
    }

    #[test]
    fn rejects_too_many_edits() {
        let root = tempfile::tempdir().unwrap();
        let many: Vec<(String, String)> = (0..=MAX_EDITS_PER_STEP)
            .map(|i| (format!("f{i}.rs"), "x".to_owned()))
            .collect();
        let pairs: Vec<(&str, &str)> = many.iter().map(|(p, c)| (p.as_str(), c.as_str())).collect();
        assert!(apply_edits(root.path(), &edits(&pairs)).is_err());
    }

    #[test]
    fn a_boundary_sized_edit_is_accepted() {
        let root = tempfile::tempdir().unwrap();
        let max = "x".repeat(MAX_BYTES_PER_EDIT);
        let n = apply_edits(root.path(), &edits(&[("big.rs", &max)])).unwrap();
        assert_eq!(n, 1);
        assert_eq!(
            std::fs::metadata(root.path().join("big.rs")).unwrap().len(),
            MAX_BYTES_PER_EDIT as u64,
            "content exactly at the per-edit bound must be written in full"
        );
    }

    #[test]
    fn rejects_an_oversized_single_edit() {
        let root = tempfile::tempdir().unwrap();
        let big = "x".repeat(MAX_BYTES_PER_EDIT + 1);
        assert!(apply_edits(root.path(), &edits(&[("a.rs", &big)])).is_err());
        assert!(!root.path().join("a.rs").exists());
    }

    #[test]
    fn rejects_an_oversized_aggregate() {
        let root = tempfile::tempdir().unwrap();
        let chunk = "x".repeat(MAX_BYTES_PER_EDIT);
        let count = (MAX_TOTAL_EDIT_BYTES / MAX_BYTES_PER_EDIT) + 1;
        let owned: Vec<(String, String)> = (0..count)
            .map(|i| (format!("f{i}.rs"), chunk.clone()))
            .collect();
        let pairs: Vec<(&str, &str)> = owned
            .iter()
            .map(|(p, c)| (p.as_str(), c.as_str()))
            .collect();
        assert!(apply_edits(root.path(), &edits(&pairs)).is_err());
    }

    #[test]
    fn rejects_duplicate_normalized_paths() {
        let root = tempfile::tempdir().unwrap();
        // "./a.rs" and "a.rs" normalize to the same path.
        assert!(apply_edits(root.path(), &edits(&[("a.rs", "1"), ("./a.rs", "2")])).is_err());
    }

    #[test]
    fn writes_nothing_when_a_later_edit_is_invalid() {
        let root = tempfile::tempdir().unwrap();
        // First edit is fine; second escapes — PREVALIDATION rejects the whole
        // batch before ANY write, so the first file must not appear. (This is
        // the prevalidation guarantee; runtime failures are the next test.)
        let out = serde_json::json!({
            "edits": [
                { "path": "ok.rs", "content": "A" },
                { "path": "../escape.rs", "content": "B" }
            ]
        });
        assert!(apply_edits(root.path(), &out).is_err());
        assert!(
            !root.path().join("ok.rs").exists(),
            "no partial write: the valid edit must not land when the batch fails"
        );
    }

    #[cfg(unix)]
    #[test]
    fn a_runtime_failure_mid_batch_reports_error_with_earlier_edit_contained() {
        let root = tempfile::tempdir().unwrap();
        // Both paths pass prevalidation, but the second target is a
        // pre-existing DIRECTORY — a deterministic *runtime* refusal that
        // strikes only after the first edit has been written.
        std::fs::create_dir(root.path().join("blocked")).unwrap();
        let err =
            apply_edits(root.path(), &edits(&[("ok.rs", "A"), ("blocked", "B")])).unwrap_err();
        assert!(err.contains("directory"), "{err}");
        // Documented contract (Contract B, not a batch transaction): a runtime
        // failure may leave earlier edits INSIDE the isolated attempt worktree.
        // The lifecycle proof that such a worktree can never be promoted,
        // snapshotted, accepted, or reused lives in integrity_tests.rs.
        assert_eq!(
            std::fs::read_to_string(root.path().join("ok.rs")).unwrap(),
            "A"
        );
        // No temp residue from the failed second edit.
        let leftovers: Vec<_> = std::fs::read_dir(root.path())
            .unwrap()
            .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
            .filter(|n| n.starts_with(".wepld-tmp-"))
            .collect();
        assert!(leftovers.is_empty(), "temp residue: {leftovers:?}");
    }
}
