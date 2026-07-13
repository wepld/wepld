//! Day-4 integration tests against real git: worktree isolation, snapshot
//! refs via temporary index, diff, scope input, fork branch, cleanup — and
//! the invariant that the user's primary worktree never shows WePLD noise.

use std::path::Path;
use std::process::Command;
use wepld_workspace::Workspace;

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
