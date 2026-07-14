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

/// A local project fingerprint (canonical Git common dir + root commit). The
/// runtime hashes this into a stable `project_id` used to scope memory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectFingerprint {
    pub common_dir: String,
    pub root_commit: String,
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
    ///
    /// The index lives inside a freshly created, uniquely named temporary
    /// **directory** (via `tempfile`), never a hand-built predictable path:
    /// creation is race-safe (`mkdir`-with-`O_EXCL` semantics, no following of a
    /// pre-existing symlink), permissions are restrictive where the OS supports
    /// it, and the directory — index and any `.lock` within it — is removed when
    /// `tmp` drops, on both the success path and every `?` error path. Two
    /// snapshots therefore never share an index path or collide on a stale lock.
    pub fn snapshot(&self, wt: &Worktree, label: &str) -> Result<SnapRef, WorkspaceError> {
        let tmp = tempfile::Builder::new()
            .prefix(&format!("wepld-index-{}-", wt.attempt_id))
            .tempdir()?;
        let tmp_index = tmp.path().join("index");
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
        // `tmp` drops here (or on any early `?` above), removing the index.
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

    /// Atomically create/advance a mission's **proposal ref**
    /// `refs/heads/wepld/mission-<id>` to `commit` using Git's compare-and-swap
    /// form `update-ref <ref> <new> <expected-old>`. `expected_old` = `None`
    /// requires the ref to **not exist** (zero old value); `Some(sha)` requires
    /// it to currently equal `sha`. A mismatch fails loudly — the ref is **never**
    /// force-replaced. No checkout, no merge; the base branch, `HEAD`, index, and
    /// primary worktree are untouched. This is the V0 acceptance effect: a
    /// reviewable proposal a human merges later through an external workflow.
    pub fn propose_ref(
        &self,
        mission_id: &str,
        commit: &str,
        expected_old: Option<&str>,
    ) -> Result<SnapRef, WorkspaceError> {
        const ZERO: &str = "0000000000000000000000000000000000000000";
        let name = format!("refs/heads/wepld/mission-{mission_id}");
        let old = expected_old.unwrap_or(ZERO);
        self.git(&["update-ref", &name, commit, old])?;
        Ok(SnapRef {
            name,
            commit: commit.to_owned(),
        })
    }

    /// **UNSUPPORTED in V0.** Retained only for a future protected-merge
    /// workflow. The Build Feature recipe never calls this — acceptance produces
    /// a proposal ref via [`Workspace::propose_ref`] and a human merges through
    /// an external protected flow. Merging here mutates the base branch and the
    /// primary worktree, which V0 forbids; do not add it to the V0 path.
    #[deprecated(
        note = "V0 acceptance uses propose_ref; in-repo base-branch merge is out of scope"
    )]
    pub fn merge(&self, commit: &str, message: &str) -> Result<String, WorkspaceError> {
        self.git(&["merge", "--no-ff", "--no-edit", "-m", message, commit])?;
        Ok(self.git(&["rev-parse", "HEAD"])?.trim().to_owned())
    }

    /// The current commit of a branch, or `None` if the branch does not exist.
    pub fn branch_commit(&self, branch: &str) -> Result<Option<String>, WorkspaceError> {
        match self.git(&[
            "rev-parse",
            "--verify",
            "--quiet",
            &format!("refs/heads/{branch}"),
        ]) {
            Ok(sha) => Ok(Some(sha.trim().to_owned())),
            Err(_) => Ok(None),
        }
    }

    /// A stable-enough V0 project fingerprint for scoping Engineering Memory:
    /// the canonical Git common directory plus the repository's root commit.
    /// See the Engineering Memory contract for clone / relocation / reinit
    /// semantics (this identity intentionally changes on reinitialization).
    pub fn project_fingerprint(&self) -> Result<ProjectFingerprint, WorkspaceError> {
        let common_raw = self.git(&["rev-parse", "--git-common-dir"])?;
        let common_raw = common_raw.trim();
        let common_path = if Path::new(common_raw).is_absolute() {
            PathBuf::from(common_raw)
        } else {
            self.repo.join(common_raw)
        };
        // Canonicalize with platform-correct case handling: Windows filesystems
        // are case-insensitive (normalize case); Unix/macOS are case-sensitive
        // (preserve case) so two repos differing only by case stay distinct.
        let canon = std::fs::canonicalize(&common_path).unwrap_or(common_path);
        let common_dir = normalize_case(&canon.to_string_lossy());
        let root_commit = self
            .git(&["rev-list", "--max-parents=0", "HEAD"])
            .unwrap_or_default()
            .split_whitespace()
            .next()
            .unwrap_or("")
            .to_owned();
        Ok(ProjectFingerprint {
            common_dir,
            root_commit,
        })
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

/// Platform-correct path-case normalization for project identity: lowercase on
/// case-insensitive Windows, preserve on case-sensitive Unix/macOS.
fn normalize_case(s: &str) -> String {
    #[cfg(windows)]
    {
        s.to_lowercase()
    }
    #[cfg(not(windows))]
    {
        s.to_owned()
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
