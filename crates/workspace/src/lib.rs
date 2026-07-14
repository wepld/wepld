//! wepld-workspace — Git as isolation and time machine (v2-02 §4, ADR-0013).
//! Per-attempt detached worktrees; workspace snapshots as hidden refs under
//! `refs/wepld/…` created with a temporary index (the user's branches, index,
//! and `git status` are never touched); diffs and scope re-verification for
//! the Core's own completion checks.
//!
//! Implementation: spawned `git` (no libgit bindings — fewer native deps,
//! and behavior identical to what the user's own git does).

use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceError {
    #[error("not a git repository: {0}")]
    NotARepo(PathBuf),
    #[error("git {args:?} failed: {stderr}")]
    GitFailed { args: Vec<String>, stderr: String },
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

/// A handle on the user's repository (the primary worktree is never written).
pub struct Workspace {
    repo: PathBuf,
}

/// An isolated, detached working copy for one attempt.
pub struct Worktree {
    pub path: PathBuf,
    pub attempt_id: String,
}

/// A workspace snapshot: hidden ref + the commit it points at.
#[derive(Debug, Clone)]
pub struct SnapRef {
    pub name: String,
    pub commit: String,
}

impl Workspace {
    pub fn open(repo: &Path) -> Result<Self, WorkspaceError> {
        let ok = Command::new("git")
            .args(["rev-parse", "--git-dir"])
            .current_dir(repo)
            .output()?;
        if !ok.status.success() {
            return Err(WorkspaceError::NotARepo(repo.to_path_buf()));
        }
        Ok(Self {
            repo: repo.to_path_buf(),
        })
    }

    /// `git worktree add --detach <root>/<attempt_id> <base>`
    pub fn create_worktree(
        &self,
        base: &str,
        attempt_id: &str,
        root: &Path,
    ) -> Result<Worktree, WorkspaceError> {
        std::fs::create_dir_all(root)?;
        let path = root.join(attempt_id);
        self.git(&[
            "worktree",
            "add",
            "--detach",
            path.to_str().expect("utf8 path"),
            base,
        ])?;
        Ok(Worktree {
            path,
            attempt_id: attempt_id.to_owned(),
        })
    }

    /// Snapshot the worktree state to `refs/wepld/<attempt>/<label>` using a
    /// temporary index — the worktree's own index and HEAD are untouched.
    pub fn snapshot(&self, wt: &Worktree, label: &str) -> Result<SnapRef, WorkspaceError> {
        // A unique temp index outside the worktree — never pollutes the
        // worktree and never collides with a stale lock across missions.
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let tmp_index =
            std::env::temp_dir().join(format!("wepld-index-{}-{label}-{nanos}", wt.attempt_id));
        let _ = std::fs::remove_file(&tmp_index);
        let _ = std::fs::remove_file(format!("{}.lock", tmp_index.display()));
        let index_env = tmp_index.to_str().expect("utf8 path").to_owned();

        let run = |args: &[&str]| -> Result<String, WorkspaceError> {
            git_in(&wt.path, args, &[("GIT_INDEX_FILE", &index_env)])
        };
        run(&["add", "-A"])?;
        let tree = run(&["write-tree"])?;
        let parent = git_in(&wt.path, &["rev-parse", "HEAD"], &[])?;
        let commit = git_in(
            &wt.path,
            &[
                "commit-tree",
                tree.trim(),
                "-p",
                parent.trim(),
                "-m",
                &format!("wepld snapshot: {label}"),
            ],
            &[
                ("GIT_AUTHOR_NAME", "wepld"),
                ("GIT_AUTHOR_EMAIL", "wepld@local"),
                ("GIT_COMMITTER_NAME", "wepld"),
                ("GIT_COMMITTER_EMAIL", "wepld@local"),
            ],
        )?;
        let commit = commit.trim().to_owned();
        let name = format!("refs/wepld/{}/{}", wt.attempt_id, label);
        git_in(&wt.path, &["update-ref", &name, &commit], &[])?;
        let _ = std::fs::remove_file(&tmp_index);
        Ok(SnapRef { name, commit })
    }

    /// Materialize any snapshot/commit as a new detached worktree (Chronicle
    /// replay and fork restore both ride on this).
    pub fn materialize(&self, refname: &str, dest: &Path) -> Result<PathBuf, WorkspaceError> {
        self.git(&[
            "worktree",
            "add",
            "--detach",
            dest.to_str().expect("utf8 path"),
            refname,
        ])?;
        Ok(dest.to_path_buf())
    }

    /// `git diff <a> <b>` between any two commits/refs.
    pub fn diff(&self, a: &str, b: &str) -> Result<String, WorkspaceError> {
        self.git(&["diff", a, b])
    }

    /// Paths changed in a worktree vs. its HEAD (modified + untracked) — the
    /// Core's own scope re-verification input (worker claims don't count).
    pub fn changed_paths(&self, wt: &Worktree) -> Result<Vec<String>, WorkspaceError> {
        let out = git_in(&wt.path, &["status", "--porcelain"], &[])?;
        Ok(out
            .lines()
            .filter(|l| l.len() > 3)
            .map(|l| {
                let p = &l[3..];
                // Renames render as "old -> new"; the effect is on `new`.
                match p.split_once(" -> ") {
                    Some((_, new)) => new.to_owned(),
                    None => p.to_owned(),
                }
            })
            .collect())
    }

    /// Create a branch at a snapshot (fork restore, mission merge).
    pub fn branch_from(&self, refname: &str, branch: &str) -> Result<(), WorkspaceError> {
        self.git(&["branch", branch, refname])?;
        Ok(())
    }

    /// Merge a commit into the currently checked-out branch (mission
    /// acceptance — the completion hard gate). Returns the merge commit.
    pub fn merge(&self, commit: &str, message: &str) -> Result<String, WorkspaceError> {
        self.git(&["merge", "--no-ff", "--no-edit", "-m", message, commit])?;
        Ok(self.git(&["rev-parse", "HEAD"])?.trim().to_owned())
    }

    /// The commit the given ref/branch currently points at.
    pub fn rev_parse(&self, refname: &str) -> Result<String, WorkspaceError> {
        Ok(self.git(&["rev-parse", refname])?.trim().to_owned())
    }

    pub fn cleanup(&self, wt: Worktree) -> Result<(), WorkspaceError> {
        self.git(&[
            "worktree",
            "remove",
            "--force",
            wt.path.to_str().expect("utf8 path"),
        ])?;
        Ok(())
    }

    /// The primary worktree must always look untouched to the user.
    pub fn primary_is_clean(&self) -> Result<bool, WorkspaceError> {
        Ok(self.git(&["status", "--porcelain"])?.trim().is_empty())
    }

    fn git(&self, args: &[&str]) -> Result<String, WorkspaceError> {
        git_in(&self.repo, args, &[])
    }
}

fn git_in(dir: &Path, args: &[&str], envs: &[(&str, &str)]) -> Result<String, WorkspaceError> {
    let mut cmd = Command::new("git");
    cmd.args(args).current_dir(dir);
    for (k, v) in envs {
        cmd.env(k, v);
    }
    let out = cmd.output()?;
    if !out.status.success() {
        return Err(WorkspaceError::GitFailed {
            args: args.iter().map(|s| s.to_string()).collect(),
            stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
        });
    }
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
}
