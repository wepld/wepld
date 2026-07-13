use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum LedgerError {
    /// v2-06: the operational store must never live in a sync-managed folder
    /// (WAL/SHM corruption and lock storms). Refused at open, loudly.
    #[error("refusing to open ledger store inside a synced folder: {0} (v2-06; choose a local app-data directory)")]
    SyncedFolder(PathBuf),

    #[error("sqlite error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error(
        "entry id generation overflow (monotonic ULID counter exhausted within one millisecond)"
    )]
    IdGeneration,
}
