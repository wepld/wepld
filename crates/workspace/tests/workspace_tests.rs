//! Day-4 integration tests against real git: worktree isolation, snapshot
//! refs via temporary index, diff, scope input, fork branch, cleanup — and
//! the invariant that the user's primary worktree never shows WePLD noise.

use std::path::Path;
use std::process::Command;
use wepld_workspace::{Workspace, WorkspaceError};

fn sh(dir: &Path, cmd: &str, args: &[&str]) {
    let out = Command::new(cmd)
        .args(args)
        .current_dir(dir)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{cmd} {args:?}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

/// Build a tiny real repository: two files, one commit on `main`.
fn fixture_repo() -> (tempfile::TempDir, std::path::PathBuf) {
    let dir = tempfile::tempdir().unwrap();
    let repo = dir.path().join("notes-cli");
    std::fs::create_dir_all(repo.join("src")).unwrap();
    std::fs::write(
        repo.join("src/main.rs"),
        "fn main() { println!(\"notes\"); }\n",
    )
    .unwrap();
    std::fs::write(repo.join("README.md"), "# notes-cli\n").unwrap();
    sh(&repo, "git", &["init", "-q", "-b", "main"]);
    sh(&repo, "git", &["config", "user.name", "Fixture"]);
    sh(&repo, "git", &["config", "user.email", "fixture@local"]);
    sh(&repo, "git", &["add", "-A"]);
    sh(&repo, "git", &["commit", "-q", "-m", "initial"]);
    (dir, repo)
}

#[test]
fn worktree_edit_snapshot_diff_materialize_cleanup() {
    let (dir, repo) = fixture_repo();
    let ws = Workspace::open(&repo).unwrap();
    let root = dir.path().join("wt");

    // Isolated worktree from main.
    let wt = ws.create_worktree("main", "att_1", &root).unwrap();
    std::fs::write(
        wt.path.join("src/main.rs"),
        "fn main() { println!(\"notes v2\"); }\n",
    )
    .unwrap();
    std::fs::write(wt.path.join("NEW.md"), "added\n").unwrap();

    // Core's scope input sees both the edit and the untracked file.
    let mut changed = ws.changed_paths(&wt).unwrap();
    changed.sort();
    assert_eq!(changed, vec!["NEW.md", "src/main.rs"]);

    // Snapshot via temporary index; worktree status unchanged afterwards.
    let snap = ws.snapshot(&wt, "build-end").unwrap();
    assert!(snap.name.starts_with("refs/wepld/att_1/"));
    let still_changed = ws.changed_paths(&wt).unwrap();
    assert_eq!(still_changed.len(), 2, "snapshot must not touch the index");

    // Diff base..snapshot shows the edit.
    let diff = ws.diff("main", &snap.commit).unwrap();
    assert!(diff.contains("notes v2"));
    assert!(diff.contains("NEW.md"));

    // Materialize the snapshot elsewhere; content matches byte-for-byte.
    let mat = ws.materialize(&snap.name, &dir.path().join("mat")).unwrap();
    assert_eq!(
        std::fs::read_to_string(mat.join("src/main.rs")).unwrap(),
        "fn main() { println!(\"notes v2\"); }\n"
    );

    // The user's primary worktree never saw any of it.
    assert!(ws.primary_is_clean().unwrap());

    ws.cleanup(wt).unwrap();
    assert!(!root.join("att_1").exists());
}

#[test]
fn branch_from_snapshot_enables_fork_restore() {
    let (dir, repo) = fixture_repo();
    let ws = Workspace::open(&repo).unwrap();
    let wt = ws
        .create_worktree("main", "att_2", &dir.path().join("wt"))
        .unwrap();
    std::fs::write(wt.path.join("README.md"), "# forked\n").unwrap();
    let snap = ws.snapshot(&wt, "snap").unwrap();

    ws.branch_from(&snap.name, "wepld/mis_fork").unwrap();
    let head = Command::new("git")
        .args(["rev-parse", "wepld/mis_fork"])
        .current_dir(&repo)
        .output()
        .unwrap();
    assert_eq!(
        String::from_utf8_lossy(&head.stdout).trim(),
        snap.commit,
        "branch must point at the snapshot commit"
    );
    assert!(ws.primary_is_clean().unwrap());
}

#[test]
fn snapshots_are_invisible_to_normal_git_log() {
    let (dir, repo) = fixture_repo();
    let ws = Workspace::open(&repo).unwrap();
    let wt = ws
        .create_worktree("main", "att_3", &dir.path().join("wt"))
        .unwrap();
    std::fs::write(wt.path.join("x.txt"), "x").unwrap();
    ws.snapshot(&wt, "s1").unwrap();

    let log = Command::new("git")
        .args(["log", "--oneline", "main"])
        .current_dir(&repo)
        .output()
        .unwrap();
    let log = String::from_utf8_lossy(&log.stdout);
    assert!(
        !log.contains("wepld snapshot"),
        "snapshot commits must not appear on user branches: {log}"
    );
}

#[test]
fn open_refuses_non_repos() {
    let dir = tempfile::tempdir().unwrap();
    assert!(Workspace::open(dir.path()).is_err());
}

// ── Blocker 2: worktree + ref defense in depth ─────────────────────────────

#[test]
fn create_worktree_refuses_unsafe_attempt_ids() {
    let (dir, repo) = fixture_repo();
    let ws = Workspace::open(&repo).unwrap();
    let root = dir.path().join("wt");
    for bad in ["../escape", "a/b", "..", ".", "", "-x", "a\\b"] {
        assert!(
            ws.create_worktree("main", bad, &root).is_err(),
            "must refuse unsafe attempt id {bad:?}"
        );
    }
    // Nothing escaped the worktrees root.
    assert!(!dir.path().join("escape").exists());
}

#[test]
fn resolve_commit_refuses_option_like_and_missing_refs() {
    let (_dir, repo) = fixture_repo();
    let ws = Workspace::open(&repo).unwrap();
    assert!(ws.resolve_commit("--output=/tmp/leak").is_err());
    assert!(ws.resolve_commit("-x").is_err());
    assert!(ws.resolve_commit("no-such-branch").is_err());
    // A real branch resolves to a 40-hex commit (never an option).
    let sha = ws.resolve_commit("main").unwrap();
    assert_eq!(sha.len(), 40);
    assert!(sha.bytes().all(|b| b.is_ascii_hexdigit()));
}

// ── Blocker 4: project fingerprint fails closed ────────────────────────────

#[test]
fn project_fingerprint_requires_a_root_commit() {
    let dir = tempfile::tempdir().unwrap();
    let repo = dir.path().join("unborn");
    std::fs::create_dir_all(&repo).unwrap();
    sh(&repo, "git", &["init", "-q", "-b", "main"]); // no commit yet
    let ws = Workspace::open(&repo).unwrap();
    assert!(matches!(
        ws.project_fingerprint(),
        Err(WorkspaceError::NoRootCommit)
    ));
}

#[test]
fn project_fingerprint_is_independent_of_path_form() {
    let (_dir, repo) = fixture_repo();
    let fp_plain = Workspace::open(&repo)
        .unwrap()
        .project_fingerprint()
        .unwrap();
    // The same repo referenced with a trailing "." resolves to one identity.
    let fp_dot = Workspace::open(&repo.join("."))
        .unwrap()
        .project_fingerprint()
        .unwrap();
    assert_eq!(fp_plain, fp_dot);
    assert!(!fp_plain.root_commit.is_empty());
}

/// Regression for the stale-lock collision: repeated snapshots of the same
/// worktree with the *same* label must each succeed (the temp index is unique
/// per call, so no `index.lock: File exists`) and leave no temp dir behind.
#[test]
fn repeated_same_label_snapshots_do_not_collide_or_leak() {
    let (dir, repo) = fixture_repo();
    let ws = Workspace::open(&repo).unwrap();
    let wt = ws
        .create_worktree("main", "att_lock", &dir.path().join("wt"))
        .unwrap();

    for i in 0..3 {
        std::fs::write(wt.path.join("x.txt"), format!("v{i}")).unwrap();
        ws.snapshot(&wt, "build-end")
            .unwrap_or_else(|e| panic!("snapshot {i} must not collide: {e}"));
    }
    assert!(ws.primary_is_clean().unwrap());

    // Every temp index directory was removed on drop — nothing leaks.
    let prefix = format!("wepld-index-{}-", wt.attempt_id);
    let leftovers = std::fs::read_dir(std::env::temp_dir())
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name().to_string_lossy().starts_with(&prefix))
        .count();
    assert_eq!(leftovers, 0, "snapshot temp index dirs must be cleaned up");
}

/// Two worktrees snapshotting concurrently must not share an index path or
/// corrupt each other — each holds its own uniquely created temp directory.
#[test]
fn concurrent_snapshots_on_separate_worktrees_are_isolated() {
    let (dir, repo) = fixture_repo();
    let ws = Workspace::open(&repo).unwrap();
    let wt_a = ws
        .create_worktree("main", "att_a", &dir.path().join("wt_a"))
        .unwrap();
    let wt_b = ws
        .create_worktree("main", "att_b", &dir.path().join("wt_b"))
        .unwrap();
    std::fs::write(wt_a.path.join("a.txt"), "from a").unwrap();
    std::fs::write(wt_b.path.join("b.txt"), "from b").unwrap();

    let (sa, sb) = std::thread::scope(|s| {
        let ha = s.spawn(|| ws.snapshot(&wt_a, "s").unwrap());
        let hb = s.spawn(|| ws.snapshot(&wt_b, "s").unwrap());
        (ha.join().unwrap(), hb.join().unwrap())
    });

    // Distinct refs, distinct commits, each carrying only its own file.
    assert_ne!(sa.commit, sb.commit);
    assert!(ws.diff("main", &sa.commit).unwrap().contains("a.txt"));
    assert!(ws.diff("main", &sb.commit).unwrap().contains("b.txt"));
    assert!(ws.primary_is_clean().unwrap());
}
