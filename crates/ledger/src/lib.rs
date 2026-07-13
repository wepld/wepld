//! wepld-ledger — the spine of WePLD (ADR-0003, v2-06): transactional state
//! tables plus a hash-chained, append-only audit ledger, written together in
//! one SQLite transaction by a single writer.
//!
//! Boundary rule (IMPL-02): only the runtime's transition function may hold a
//! [`Tx`]. Everything else reads.

mod error;
mod fold;
mod store;

pub use error::LedgerError;
pub use fold::{fold_mission, FoldedMission};
pub use store::{payload_hash, AppendedRef, ChainReport, LedgerStore, NewEntry, Tx, GENESIS_HASH};
