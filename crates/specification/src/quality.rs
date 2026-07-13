//! Deterministic, evidence-based specification quality scoring. Structural
//! only at this stage (scores derive from the document's own shape, so every
//! score is explainable and reproducible); reasoning-based signals
//! (contradiction detection) join at Spec-C via Specification Intelligence.

use wepld_contracts::specification::{SpecQuality, SpecificationDocument};

/// Sections whose presence defines a "complete" specification.
const EXPECTED_SECTIONS: [&str; 9] = [
    "overview",
    "user_stories",
    "functional_requirements",
    "acceptance_criteria",
    "non_functional",
    "edge_cases",
    "constraints",
    "dependencies",
    "success_metrics",
];

fn present(doc: &SpecificationDocument, section: &str) -> bool {
    match section {
        "overview" => !doc.overview.trim().is_empty(),
        "user_stories" => !doc.user_stories.is_empty(),
        "functional_requirements" => !doc.functional_requirements.is_empty(),
        "acceptance_criteria" => !doc.acceptance_criteria.is_empty(),
        "non_functional" => !doc.non_functional.is_empty(),
        "edge_cases" => !doc.edge_cases.is_empty(),
        "constraints" => !doc.constraints.is_empty(),
        "dependencies" => !doc.dependencies.is_empty(),
        "success_metrics" => !doc.success_metrics.is_empty(),
        _ => false,
    }
}

/// Compute evidence-based quality scores for a canonical document. Pure.
pub fn score_quality(doc: &SpecificationDocument) -> SpecQuality {
    let missing_sections: Vec<String> = EXPECTED_SECTIONS
        .iter()
        .filter(|s| !present(doc, s))
        .map(|s| (*s).to_owned())
        .collect();
    let completeness =
        (EXPECTED_SECTIONS.len() - missing_sections.len()) as f64 / EXPECTED_SECTIONS.len() as f64;

    let coverage = if doc.acceptance_criteria.is_empty() {
        0.0
    } else {
        let with_verify = doc
            .acceptance_criteria
            .iter()
            .filter(|c| !c.verify.trim().is_empty())
            .count();
        with_verify as f64 / doc.acceptance_criteria.len() as f64
    };

    let ambiguity = doc.open_questions.len() as u32;
    // Structural consistency: no detectable structural contradiction yet; a
    // spec that leaves criteria unverifiable is internally inconsistent.
    let consistency = coverage;
    // Risk rises with missing sections and unresolved ambiguity.
    let risk = ((1.0 - completeness) + (ambiguity as f64 * 0.05)).min(1.0);
    let maintainability = completeness;

    SpecQuality {
        completeness,
        consistency,
        ambiguity,
        coverage,
        risk,
        maintainability,
        missing_sections,
        review_status: "unreviewed".to_owned(),
        verification_status: "unverified".to_owned(),
        evidence_refs: vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wepld_contracts::specification::SpecAcceptanceCriterion;

    #[test]
    fn empty_doc_scores_zero_completeness() {
        let q = score_quality(&SpecificationDocument::default());
        assert_eq!(q.completeness, 0.0);
        assert_eq!(q.coverage, 0.0);
        assert_eq!(q.missing_sections.len(), 9);
        assert!(q.risk >= 1.0 - f64::EPSILON);
    }

    #[test]
    fn full_doc_scores_high_and_is_deterministic() {
        let doc = SpecificationDocument {
            overview: "o".to_owned(),
            user_stories: vec!["s".to_owned()],
            functional_requirements: vec!["f".to_owned()],
            acceptance_criteria: vec![SpecAcceptanceCriterion {
                id: "AC1".to_owned(),
                text: "t".to_owned(),
                verify: "gate:test".to_owned(),
            }],
            non_functional: vec!["nf".to_owned()],
            edge_cases: vec!["e".to_owned()],
            constraints: vec!["c".to_owned()],
            dependencies: vec!["d".to_owned()],
            success_metrics: vec!["m".to_owned()],
            ..Default::default()
        };
        let q = score_quality(&doc);
        assert_eq!(q.completeness, 1.0);
        assert_eq!(q.coverage, 1.0);
        assert_eq!(q.ambiguity, 0);
        assert!(q.missing_sections.is_empty());
        // Deterministic: same input → same scores.
        let q2 = score_quality(&doc);
        assert_eq!(q.completeness, q2.completeness);
        assert_eq!(q.risk, q2.risk);
    }

    #[test]
    fn partial_coverage_lowers_scores() {
        let doc = SpecificationDocument {
            overview: "o".to_owned(),
            acceptance_criteria: vec![
                SpecAcceptanceCriterion {
                    id: "AC1".to_owned(),
                    text: "t".to_owned(),
                    verify: "gate:test".to_owned(),
                },
                SpecAcceptanceCriterion {
                    id: "AC2".to_owned(),
                    text: "t".to_owned(),
                    verify: String::new(),
                },
            ],
            ..Default::default()
        };
        let q = score_quality(&doc);
        assert_eq!(q.coverage, 0.5);
    }
}
