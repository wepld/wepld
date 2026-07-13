//! wepld-artifacts — the content-addressed store (CAS). Bodies are write-once
//! files sharded by hash (`objects/ab/cdef…`); every read re-verifies the
//! hash; tombstoning removes a body but preserves the hash and reason
//! (v2-06 retention semantics). Artifact *metadata* rows live in the ledger
//! database and are written by the runtime — this crate owns bodies only.

use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, thiserror::Error)]
pub enum CasError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("artifact not found: {0}")]
    NotFound(String),
    #[error("artifact body is corrupt: expected {expected}, computed {computed}")]
    Corrupt { expected: String, computed: String },
    #[error("artifact {hash} is tombstoned: {reason}")]
    Tombstoned { hash: String, reason: String },
    #[error("invalid hash: {0}")]
    InvalidHash(String),
}

pub struct Cas {
    root: PathBuf,
}

pub struct StoredRef {
    pub hash: String,
    pub size_bytes: u64,
    /// False when an identical body already existed (deduplicated).
    pub newly_written: bool,
}

impl Cas {
    pub fn open(root: &Path) -> Result<Self, CasError> {
        fs::create_dir_all(root.join("objects"))?;
        Ok(Self {
            root: root.to_path_buf(),
        })
    }

    /// Store a body; content-addressed, write-once, deduplicated.
    pub fn put(&self, body: &[u8]) -> Result<StoredRef, CasError> {
        let hash = hex(&Sha256::digest(body));
        let path = self.body_path(&hash)?;
        if path.exists() {
            return Ok(StoredRef {
                hash,
                size_bytes: body.len() as u64,
                newly_written: false,
            });
        }
        fs::create_dir_all(path.parent().expect("sharded path has parent"))?;
        // Write-then-rename so a crash never leaves a half-written body
        // addressable under its hash.
        let tmp = path.with_extension("tmp");
        fs::write(&tmp, body)?;
        fs::rename(&tmp, &path)?;
        Ok(StoredRef {
            hash,
            size_bytes: body.len() as u64,
            newly_written: true,
        })
    }

    /// Read a body, re-verifying its hash (corrupt storage is detected, never
    /// silently served).
    pub fn get(&self, hash: &str) -> Result<Vec<u8>, CasError> {
        if let Some(reason) = self.tombstone_reason(hash)? {
            return Err(CasError::Tombstoned {
                hash: hash.to_owned(),
                reason,
            });
        }
        let path = self.body_path(hash)?;
        if !path.exists() {
            return Err(CasError::NotFound(hash.to_owned()));
        }
        let body = fs::read(&path)?;
        let computed = hex(&Sha256::digest(&body));
        if computed != hash {
            return Err(CasError::Corrupt {
                expected: hash.to_owned(),
                computed,
            });
        }
        Ok(body)
    }

    pub fn verify(&self, hash: &str) -> Result<bool, CasError> {
        match self.get(hash) {
            Ok(_) => Ok(true),
            Err(CasError::Corrupt { .. }) => Ok(false),
            Err(e) => Err(e),
        }
    }

    /// Remove the body, keep the hash and the reason. Irreversible.
    pub fn tombstone(&self, hash: &str, reason: &str) -> Result<(), CasError> {
        let path = self.body_path(hash)?;
        if !path.exists() && self.tombstone_reason(hash)?.is_none() {
            return Err(CasError::NotFound(hash.to_owned()));
        }
        fs::write(self.tombstone_path(hash)?, reason)?;
        if path.exists() {
            fs::remove_file(&path)?;
        }
        Ok(())
    }

    pub fn tombstone_reason(&self, hash: &str) -> Result<Option<String>, CasError> {
        let p = self.tombstone_path(hash)?;
        if p.exists() {
            Ok(Some(fs::read_to_string(p)?))
        } else {
            Ok(None)
        }
    }

    fn body_path(&self, hash: &str) -> Result<PathBuf, CasError> {
        validate_hash(hash)?;
        Ok(self.root.join("objects").join(&hash[..2]).join(&hash[2..]))
    }

    fn tombstone_path(&self, hash: &str) -> Result<PathBuf, CasError> {
        Ok(self.body_path(hash)?.with_extension("tombstone"))
    }
}

fn validate_hash(hash: &str) -> Result<(), CasError> {
    if hash.len() != 64 || !hash.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(CasError::InvalidHash(hash.to_owned()));
    }
    Ok(())
}

fn hex(bytes: &[u8]) -> String {
    use std::fmt::Write as _;
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        let _ = write!(s, "{b:02x}");
    }
    s
}
