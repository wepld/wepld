//! Central validation contracts (final security-boundary remediation).
//!
//! Untrusted model/brief text — feature slugs, operational identifiers, plan
//! task ids, base refs, and worker edit paths — must be validated into safe
//! forms **before** becoming filesystem components or Git refs. The rule:
//! **identifiers are data, never path or ref syntax**, and a validated
//! worktree-relative path can never escape its root. Validation is intentionally
//! narrow ASCII, deterministic, and inspects `std::path::Component` (not
//! substrings) so harmless names like `release..notes.txt` are accepted.

use std::fmt;
use std::path::{Component, Path, PathBuf};

/// Max feature-slug length.
pub const MAX_SLUG_LEN: usize = 64;
/// Max operational-identifier length (mission/task/attempt ids).
pub const MAX_IDENT_LEN: usize = 128;
/// Max Git ref / branch-name length.
pub const MAX_REF_LEN: usize = 200;
/// Max worker edit-path length.
pub const MAX_EDIT_PATH_LEN: usize = 1024;

// ── Resource bounds on untrusted model-produced payloads ───────────────────
// Conservative, deterministic caps so an oversized or flooding model output is
// rejected at the boundary before any write or persistence (final remediation).

/// Max edits applied in one builder step.
pub const MAX_EDITS_PER_STEP: usize = 200;
/// Max bytes of content in a single edit (1 MiB).
pub const MAX_BYTES_PER_EDIT: usize = 1 << 20;
/// Max aggregate bytes across all edits in one step (8 MiB).
pub const MAX_TOTAL_EDIT_BYTES: usize = 8 << 20;
/// Max tasks in a single plan.
pub const MAX_PLAN_TASKS: usize = 64;
/// Max bytes of a task title.
pub const MAX_TASK_TITLE_BYTES: usize = 200;
/// Max acceptance-criterion references per task.
pub const MAX_SATISFIES_PER_TASK: usize = 32;
/// Max bytes of a serialized plan document (64 KiB).
pub const MAX_TOTAL_PLAN_BYTES: usize = 64 << 10;

/// A deterministic validation failure — safe to record as a command rejection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    Empty {
        what: &'static str,
    },
    TooLong {
        what: &'static str,
        len: usize,
        max: usize,
    },
    Illegal {
        what: &'static str,
        value: String,
        why: &'static str,
    },
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::Empty { what } => write!(f, "{what} is empty"),
            ValidationError::TooLong { what, len, max } => {
                write!(f, "{what} is too long ({len} > {max})")
            }
            ValidationError::Illegal { what, value, why } => {
                write!(f, "{what} is invalid ({why}): {value:?}")
            }
        }
    }
}

impl std::error::Error for ValidationError {}

fn illegal(what: &'static str, value: &str, why: &'static str) -> ValidationError {
    ValidationError::Illegal {
        what,
        value: value.to_owned(),
        why,
    }
}

fn check_len(what: &'static str, s: &str, max: usize) -> Result<(), ValidationError> {
    if s.is_empty() {
        return Err(ValidationError::Empty { what });
    }
    if s.len() > max {
        return Err(ValidationError::TooLong {
            what,
            len: s.len(),
            max,
        });
    }
    Ok(())
}

/// A feature slug: `[a-z0-9]+(-[a-z0-9]+)*` — lowercase alphanumerics joined by
/// single hyphens, no leading/trailing/double hyphen, bounded length.
pub fn validate_slug(s: &str) -> Result<(), ValidationError> {
    check_len("slug", s, MAX_SLUG_LEN)?;
    let mut after_boundary = true; // start behaves like just after a hyphen
    for &b in s.as_bytes() {
        match b {
            b'a'..=b'z' | b'0'..=b'9' => after_boundary = false,
            b'-' => {
                if after_boundary {
                    return Err(illegal("slug", s, "leading or doubled hyphen"));
                }
                after_boundary = true;
            }
            _ => return Err(illegal("slug", s, "only [a-z0-9-] allowed")),
        }
    }
    if after_boundary {
        return Err(illegal("slug", s, "trailing hyphen"));
    }
    Ok(())
}

/// An operational identifier (mission/task/attempt id): `[A-Za-z0-9][A-Za-z0-9_-]*`
/// — no leading hyphen, and the charset excludes `.` `/` `\` `@` `{` whitespace
/// and control characters, so it can never carry `..`, a separator, a Git-special
/// sequence, or a trailing dot.
pub fn validate_identifier(what: &'static str, s: &str) -> Result<(), ValidationError> {
    check_len(what, s, MAX_IDENT_LEN)?;
    let bytes = s.as_bytes();
    if bytes[0] == b'-' {
        return Err(illegal(what, s, "leading hyphen"));
    }
    for &b in bytes {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'_' | b'-' => {}
            _ => return Err(illegal(what, s, "only [A-Za-z0-9_-] allowed")),
        }
    }
    Ok(())
}

/// A Git branch/ref name, validated lexically against the sequences that make a
/// value dangerous as a ref or a CLI option. `git check-ref-format` remains the
/// authority at the workspace layer; this rejects the obvious attacks early
/// (leading `-`, `..`, `@{`, control/space, `~^:?*[\`, trailing `.`/`/`/`.lock`).
pub fn validate_git_ref_name(what: &'static str, s: &str) -> Result<(), ValidationError> {
    check_len(what, s, MAX_REF_LEN)?;
    if s.starts_with('-') {
        return Err(illegal(what, s, "leading '-' (could become a git option)"));
    }
    if s.contains("..") || s.contains("@{") || s.contains("//") {
        return Err(illegal(what, s, "contains '..', '@{', or '//'"));
    }
    if s.ends_with('.') || s.ends_with('/') || s.ends_with(".lock") {
        return Err(illegal(what, s, "invalid trailing '.', '/', or '.lock'"));
    }
    for &b in s.as_bytes() {
        if b <= 0x20 || b == 0x7f || matches!(b, b'~' | b'^' | b':' | b'?' | b'*' | b'[' | b'\\') {
            return Err(illegal(what, s, "control/space or git-special character"));
        }
    }
    Ok(())
}

/// A worktree edit path validated to be a safe, confined relative path
/// (Blocker 1). Rejects absolute paths, drive/UNC prefixes, root, every
/// parent-dir (`..`) component, backslashes/NUL, and empties — by inspecting
/// `Component`s, so `release..notes.txt` is accepted while `a/../../x` is not.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceRelativePath(PathBuf);

impl WorkspaceRelativePath {
    pub fn parse(raw: &str) -> Result<Self, ValidationError> {
        check_len("edit path", raw, MAX_EDIT_PATH_LEN)?;
        if raw.contains('\0') {
            return Err(illegal("edit path", raw, "contains NUL"));
        }
        // A backslash or a `X:` drive prefix marks a Windows path even on Unix,
        // where `Component` would otherwise treat it as an ordinary name.
        if raw.contains('\\') {
            return Err(illegal("edit path", raw, "contains a backslash"));
        }
        if raw.as_bytes().first().is_some_and(u8::is_ascii_alphabetic)
            && raw.as_bytes().get(1) == Some(&b':')
        {
            return Err(illegal("edit path", raw, "has a drive-letter prefix"));
        }

        let mut normal = PathBuf::new();
        for comp in Path::new(raw).components() {
            match comp {
                Component::Normal(c) => normal.push(c),
                Component::CurDir => {} // "." is harmless; drop it
                Component::ParentDir => {
                    return Err(illegal(
                        "edit path",
                        raw,
                        "has a parent-dir ('..') component",
                    ))
                }
                Component::RootDir => return Err(illegal("edit path", raw, "is absolute")),
                Component::Prefix(_) => {
                    return Err(illegal("edit path", raw, "has a drive/UNC prefix"))
                }
            }
        }
        if normal.as_os_str().is_empty() {
            return Err(illegal("edit path", raw, "resolves to an empty path"));
        }
        Ok(Self(normal))
    }

    /// The validated, normalized relative path (only `Normal` components).
    pub fn as_path(&self) -> &Path {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slug_grammar() {
        for ok in ["version-flag", "a", "a1", "add-a-flag", "x9y8"] {
            assert!(validate_slug(ok).is_ok(), "{ok} should be valid");
        }
        for bad in [
            "", "-a", "a-", "a--b", "A", "a_b", "a/b", "../x", "a b", "café",
        ] {
            assert!(validate_slug(bad).is_err(), "{bad} should be invalid");
        }
    }

    #[test]
    fn identifier_grammar() {
        for ok in ["mis_version-flag_v1", "T1", "att_x_build", "a-b_c9"] {
            assert!(
                validate_identifier("id", ok).is_ok(),
                "{ok} should be valid"
            );
        }
        for bad in [
            "", "-x", "a/b", "..", "../../x", "a.b", "a@{b", "a b", "a\\b",
        ] {
            assert!(
                validate_identifier("id", bad).is_err(),
                "{bad} should be invalid"
            );
        }
    }

    #[test]
    fn git_ref_grammar() {
        for ok in ["main", "feature/x", "release-1.2"] {
            assert!(
                validate_git_ref_name("ref", ok).is_ok(),
                "{ok} should be valid"
            );
        }
        for bad in [
            "-x",
            "--output=/tmp/leak",
            "a..b",
            "a@{0}",
            "a b",
            "a//b",
            "a.lock",
            "a/",
            "a.",
            "a~b",
        ] {
            assert!(
                validate_git_ref_name("ref", bad).is_err(),
                "{bad} should be invalid"
            );
        }
    }

    #[test]
    fn edit_path_validation() {
        for ok in [
            "src/main.rs",
            "src/generated/file.rs",
            "release..notes.txt",
            "./a.txt",
        ] {
            assert!(
                WorkspaceRelativePath::parse(ok).is_ok(),
                "{ok} should be valid"
            );
        }
        for bad in [
            "",
            "../outside.txt",
            "a/../../outside.txt",
            "/etc/passwd",
            "C:\\Windows\\x",
            "a\\b",
            "..",
            "\0bad",
        ] {
            assert!(
                WorkspaceRelativePath::parse(bad).is_err(),
                "{bad} should be invalid"
            );
        }
        // Normalization drops "." and keeps the safe remainder.
        assert_eq!(
            WorkspaceRelativePath::parse("./src/main.rs")
                .unwrap()
                .as_path(),
            Path::new("src/main.rs")
        );
    }
}
