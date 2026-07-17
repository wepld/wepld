//! Specification Templates — reusable engineering assets. A template
//! pre-populates the canonical document's sections with guidance and seeds
//! `[NEEDS CLARIFICATION]` markers so authors resolve the important unknowns
//! before planning. Templates are the seed of Engineering Packs (later).

use wepld_contracts::specification::{SpecAcceptanceCriterion, SpecificationDocument};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemplateKind {
    Blank,
    RestApi,
    CliApplication,
    RustLibrary,
}

impl TemplateKind {
    pub fn from_slug(s: &str) -> Option<Self> {
        match s {
            "blank" => Some(Self::Blank),
            "rest-api" => Some(Self::RestApi),
            "cli" | "cli-application" => Some(Self::CliApplication),
            "rust-library" | "rust-lib" => Some(Self::RustLibrary),
            _ => None,
        }
    }
}

/// Build a starter document for a template kind. The result is intentionally
/// *invalid* until the author fills it in (open questions seeded), so the
/// validation gate guides authoring.
pub fn template(kind: TemplateKind) -> SpecificationDocument {
    match kind {
        TemplateKind::Blank => SpecificationDocument {
            overview: String::new(),
            open_questions: vec!["What is the outcome this specification delivers?".to_owned()],
            ..Default::default()
        },
        TemplateKind::RestApi => SpecificationDocument {
            overview: "<what this API does and why>".to_owned(),
            functional_requirements: vec![
                "Define the endpoints and their request/response contracts".to_owned(),
                "Define authentication and authorization".to_owned(),
            ],
            acceptance_criteria: vec![SpecAcceptanceCriterion {
                id: "AC1".to_owned(),
                text: "endpoints return the specified contracts".to_owned(),
                verify: "gate:test".to_owned(),
            }],
            non_functional: vec!["Latency and throughput targets".to_owned()],
            required_skills: vec!["backend".to_owned(), "security".to_owned()],
            open_questions: vec![
                "Which data store backs the API?".to_owned(),
                "What are the rate-limiting requirements?".to_owned(),
            ],
            ..Default::default()
        },
        TemplateKind::CliApplication => SpecificationDocument {
            overview: "<what this CLI does and for whom>".to_owned(),
            functional_requirements: vec!["Define the commands, flags, and outputs".to_owned()],
            acceptance_criteria: vec![SpecAcceptanceCriterion {
                id: "AC1".to_owned(),
                text: "commands behave as specified".to_owned(),
                verify: "gate:test".to_owned(),
            }],
            required_skills: vec!["cli".to_owned()],
            open_questions: vec!["What are the primary user workflows?".to_owned()],
            ..Default::default()
        },
        TemplateKind::RustLibrary => SpecificationDocument {
            overview: "<what this library provides and its public API surface>".to_owned(),
            functional_requirements: vec!["Define the public API and its guarantees".to_owned()],
            acceptance_criteria: vec![SpecAcceptanceCriterion {
                id: "AC1".to_owned(),
                text: "public API matches the specification and is documented".to_owned(),
                verify: "gate:test".to_owned(),
            }],
            non_functional: vec!["No unsafe without justification; semver discipline".to_owned()],
            required_skills: vec!["rust".to_owned()],
            open_questions: vec!["What is the minimum supported Rust version?".to_owned()],
            ..Default::default()
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::validate::validate;

    #[test]
    fn templates_seed_clarifications_so_they_start_invalid() {
        for k in [
            TemplateKind::Blank,
            TemplateKind::RestApi,
            TemplateKind::CliApplication,
            TemplateKind::RustLibrary,
        ] {
            let doc = template(k);
            // A fresh template must be resolved before it can convert to a mission.
            assert!(!validate(&doc).valid, "template {k:?} should start invalid");
        }
    }

    #[test]
    fn slug_resolution() {
        assert_eq!(
            TemplateKind::from_slug("rest-api"),
            Some(TemplateKind::RestApi)
        );
        assert_eq!(TemplateKind::from_slug("nope"), None);
    }
}
