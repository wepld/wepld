//! WePLD frozen contracts (Architecture v2.0) as Rust types.
//!
//! This crate is the L0 of the workspace: every other crate imports its types;
//! it imports nothing from the workspace and contains no logic beyond
//! serialization. Field names and enum spellings are normative per
//! docs/v2/07_Contracts.md and docs/v2/17_Chronicle_Contracts_and_API.md.
//! Changing anything here is a contract change: bump [`CONTRACTS_VERSION`],
//! update the lock tests, and cite the authorizing document in the PR.

pub mod brain;
pub mod command;
pub mod envelope;
pub mod ledger;
pub mod mission;
pub mod specification;
pub mod vocabulary;
pub mod wwp;

/// Semantic version of the contracts crate (additive minors only; a breaking
/// change requires a new major and a coexistence window per v2-07).
/// 0.2.0: added the Command contract (additive).
/// 0.3.0: added the Brain result contract (additive).
/// 0.4.0: added gate_commands, PlanDoc, TaskSpec (additive).
/// 0.5.0: Engineering Specification System — specification contract +
///        rev-3 vocabulary + Specification aggregate (additive).
pub const CONTRACTS_VERSION: &str = "0.5.0";

/// Event vocabulary revision (rev 2 = base 32 + Chronicle 7;
/// rev 3 = + Specification System 13).
pub const EVENT_VOCABULARY_REVISION: u32 = 3;
