//! Deterministic specification validation — the completeness gate. Pure and
//! reasoning-free (structural checks only); reasoning-based findings arrive
//! with Specification Intelligence (Spec-C). A spec that fails validation
//! cannot be converted to a mission.

use wepld_contracts::specification::SpecificationDocument;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationReport {
    pub valid: bool,
    pub issues: Vec<ValidationIssue>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationIssue {
    pub code: String,
    pub detail: String,
}

impl ValidationReport {
    fn issue(code: &str, detail: impl Into<String>) -> ValidationIssue {
        ValidationIssue {
            code: code.to_owned(),
            detail: detail.into(),
        }
    }
}

/// Validate a canonical specification document. Deterministic.
pub fn validate(doc: &SpecificationDocument) -> ValidationReport {
    let mut issues = Vec::new();

    if doc.overview.trim().is_empty() {
        issues.push(ValidationReport::issue(
            "empty_overview",
            "overview is empty",
        ));
    }
    if doc.user_stories.is_empty() && doc.functional_requirements.is_empty() {
        issues.push(ValidationReport::issue(
            "no_intent",
            "a spec needs at least one user story or functional requirement",
        ));
    }
    if doc.acceptance_criteria.is_empty() {
        issues.push(ValidationReport::issue(
            "no_acceptance_criteria",
            "at least one acceptance criterion is required",
        ));
    }
    for c in &doc.acceptance_criteria {
        if c.verify.trim().is_empty() {
            issues.push(ValidationReport::issue(
                "criterion_without_verify",
                format!("acceptance criterion '{}' has no verify method", c.id),
            ));
        }
    }
    if !doc.open_questions.is_empty() {
        issues.push(ValidationReport::issue(
            "unresolved_clarifications",
            format!(
                "{} unresolved [NEEDS CLARIFICATION] marker(s) must be resolved before planning",
                doc.open_questions.len()
            ),
        ));
    }

    ValidationReport {
        valid: issues.is_empty(),
        issues,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wepld_contracts::specification::SpecAcceptanceCriterion;

    fn good() -> SpecificationDocument {
        SpecificationDocument {
            overview: "Add a --version flag".to_owned(),
            functional_requirements: vec!["Print the version and exit".to_owned()],
            acceptance_criteria: vec![SpecAcceptanceCriterion {
                id: "AC1".to_owned(),
                text: "version printed".to_owned(),
                verify: "gate:test".to_owned(),
            }],
            ..Default::default()
        }
    }

    #[test]
    fn a_complete_spec_is_valid() {
        assert!(validate(&good()).valid);
    }

    #[test]
    fn empty_spec_reports_all_gaps() {
        let r = validate(&SpecificationDocument::default());
        assert!(!r.valid);
        let codes: Vec<_> = r.issues.iter().map(|i| i.code.as_str()).collect();
        assert!(codes.contains(&"empty_overview"));
        assert!(codes.contains(&"no_intent"));
        assert!(codes.contains(&"no_acceptance_criteria"));
    }

    #[test]
    fn unresolved_clarification_blocks_validity() {
        let mut d = good();
        d.open_questions.push("Which auth scheme?".to_owned());
        let r = validate(&d);
        assert!(!r.valid);
        assert!(r
            .issues
            .iter()
            .any(|i| i.code == "unresolved_clarifications"));
    }

    #[test]
    fn criterion_without_verify_is_rejected() {
        let mut d = good();
        d.acceptance_criteria[0].verify = "  ".to_owned();
        let r = validate(&d);
        assert!(!r.valid);
        assert!(r
            .issues
            .iter()
            .any(|i| i.code == "criterion_without_verify"));
    }
}
