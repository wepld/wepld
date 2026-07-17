//! Deterministic parser: markdown → canonical [`SpecificationDocument`].
//! Tolerant (unrecognized lines are ignored) so real Spec Kit markdown can be
//! imported best-effort, and total (never panics). The canonical render format
//! round-trips exactly: `parse(render(doc)) == doc` for single-line field
//! values (documented constraint).

use wepld_contracts::specification::{
    Clarification, SpecAcceptanceCriterion, SpecificationDocument,
};

pub fn parse(md: &str) -> SpecificationDocument {
    let mut doc = SpecificationDocument::default();
    let mut section = String::new();
    let mut overview: Vec<String> = Vec::new();

    for line in md.lines() {
        if let Some(h) = line.strip_prefix("## ") {
            section = h.trim().to_owned();
            continue;
        }
        if line.starts_with("# ") {
            continue; // document title
        }
        match section.as_str() {
            "Overview" => {
                let t = line.trim();
                if !t.is_empty() {
                    overview.push(t.to_owned());
                }
            }
            "User Stories" => push_item(&mut doc.user_stories, line),
            "Functional Requirements" => push_item(&mut doc.functional_requirements, line),
            "Acceptance Criteria" => {
                if let Some(c) = parse_criterion(line) {
                    doc.acceptance_criteria.push(c);
                }
            }
            "Non-Functional" => push_item(&mut doc.non_functional, line),
            "Edge Cases" => push_item(&mut doc.edge_cases, line),
            "Constraints" => push_item(&mut doc.constraints, line),
            "Dependencies" => push_item(&mut doc.dependencies, line),
            "Required Skills" => push_item(&mut doc.required_skills, line),
            "Success Metrics" => push_item(&mut doc.success_metrics, line),
            "Verification" => {
                if let Some((g, c)) = parse_kv(line) {
                    doc.verification.insert(g, c);
                }
            }
            "Clarifications" => {
                if let Some(c) = parse_clarification(line) {
                    doc.clarifications.push(c);
                }
            }
            "Open Questions" => push_item(&mut doc.open_questions, line),
            _ => {}
        }
    }
    doc.overview = overview.join(" ");
    doc
}

fn push_item(v: &mut Vec<String>, line: &str) {
    if let Some(item) = line.strip_prefix("- ") {
        let t = item.trim();
        if !t.is_empty() {
            v.push(t.to_owned());
        }
    }
}

fn parse_criterion(line: &str) -> Option<SpecAcceptanceCriterion> {
    let s = line.strip_prefix("- ")?.trim();
    let s = s.strip_prefix('[')?;
    let close = s.find(']')?;
    let id = s[..close].trim().to_owned();
    let rest = s[close + 1..].trim();
    let vpos = rest.rfind("(verify:")?;
    let text = rest[..vpos].trim().to_owned();
    let verify = rest[vpos + "(verify:".len()..]
        .trim()
        .strip_suffix(')')?
        .trim()
        .to_owned();
    Some(SpecAcceptanceCriterion { id, text, verify })
}

fn parse_kv(line: &str) -> Option<(String, String)> {
    let s = line.strip_prefix("- ")?;
    let pos = s.find(": ")?;
    Some((s[..pos].trim().to_owned(), s[pos + 2..].trim().to_owned()))
}

fn parse_clarification(line: &str) -> Option<Clarification> {
    let s = line.strip_prefix("- ")?;
    let pos = s.find(" :: ")?;
    Some(Clarification {
        question: s[..pos].trim().to_owned(),
        answer: s[pos + 4..].trim().to_owned(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::render;
    use wepld_contracts::specification::SpecAcceptanceCriterion;

    fn sample() -> SpecificationDocument {
        let mut d = SpecificationDocument {
            overview: "Add a --version flag to notes-cli".to_owned(),
            user_stories: vec!["As a user I can see the version".to_owned()],
            functional_requirements: vec!["Print the version and exit".to_owned()],
            acceptance_criteria: vec![SpecAcceptanceCriterion {
                id: "AC1".to_owned(),
                text: "version constant present".to_owned(),
                verify: "gate:build".to_owned(),
            }],
            non_functional: vec!["Startup under 50ms".to_owned()],
            dependencies: vec!["clap".to_owned()],
            required_skills: vec!["rust".to_owned()],
            clarifications: vec![Clarification {
                question: "Which version string?".to_owned(),
                answer: "0.1.0 from Cargo.toml".to_owned(),
            }],
            ..Default::default()
        };
        d.verification
            .insert("build".to_owned(), "grep -q VERSION src/main.rs".to_owned());
        d
    }

    #[test]
    fn round_trip_is_exact() {
        let doc = sample();
        let back = parse(&render(&doc));
        assert_eq!(back.overview, doc.overview);
        assert_eq!(back.user_stories, doc.user_stories);
        assert_eq!(back.functional_requirements, doc.functional_requirements);
        assert_eq!(back.acceptance_criteria.len(), 1);
        assert_eq!(back.acceptance_criteria[0].id, "AC1");
        assert_eq!(back.acceptance_criteria[0].verify, "gate:build");
        assert_eq!(back.verification, doc.verification);
        assert_eq!(back.clarifications.len(), 1);
        assert_eq!(back.clarifications[0].answer, "0.1.0 from Cargo.toml");
        assert_eq!(back.required_skills, doc.required_skills);
        // Full structural equality via JSON.
        assert_eq!(
            serde_json::to_value(&back).unwrap(),
            serde_json::to_value(&doc).unwrap()
        );
    }

    #[test]
    fn render_is_deterministic() {
        let doc = sample();
        assert_eq!(render(&doc), render(&doc));
    }

    #[test]
    fn empty_doc_round_trips() {
        let doc = SpecificationDocument::default();
        let back = parse(&render(&doc));
        assert_eq!(
            serde_json::to_value(&back).unwrap(),
            serde_json::to_value(&doc).unwrap()
        );
    }

    #[test]
    fn tolerant_of_unknown_sections() {
        let md = "# Specification\n\n## Overview\nHello\n\n## Notes\n- ignored\n";
        let doc = parse(md);
        assert_eq!(doc.overview, "Hello");
    }
}
