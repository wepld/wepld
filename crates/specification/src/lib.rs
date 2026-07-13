//! wepld-specification — the Engineering Specification domain.
//!
//! A permanent, first-class domain (peer of Mission, Chronicle, Knowledge).
//! This crate is **pure**: it defines the canonical specification behaviour —
//! validation, quality scoring, templates, and (later) parse/render and
//! Mission Conversion — over the [`wepld_contracts::specification`] types.
//! It owns no state, performs no I/O, calls no gateway, and executes nothing.
//! The Runtime composes it and owns all persistence and execution
//! (docs/impl/M1_SPEC_KIT_INTEGRATION.md, boundary rules).

mod quality;
mod templates;
mod validate;

pub use quality::score_quality;
pub use templates::{template, TemplateKind};
pub use validate::{validate, ValidationIssue, ValidationReport};

// Re-export the canonical contract types so consumers use one path.
pub use wepld_contracts::specification::{
    SpecAcceptanceCriterion, SpecQuality, SpecStatus, SpecificationDocument,
};
