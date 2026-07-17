//! Deterministic renderer: canonical [`SpecificationDocument`] → markdown.
//! Markdown is one serialization; the object is the truth. Section order is
//! fixed and empty list/map sections are omitted, so rendering is stable and
//! `parse(render(doc)) == doc` (see `parse`).

use std::fmt::Write as _;
use wepld_contracts::specification::SpecificationDocument;

pub fn render(doc: &SpecificationDocument) -> String {
    let mut s = String::new();
    s.push_str("# Specification\n\n## Overview\n");
    s.push_str(doc.overview.trim());
    s.push('\n');

    list(&mut s, "User Stories", &doc.user_stories);
    list(
        &mut s,
        "Functional Requirements",
        &doc.functional_requirements,
    );

    if !doc.acceptance_criteria.is_empty() {
        s.push_str("\n## Acceptance Criteria\n");
        for c in &doc.acceptance_criteria {
            let _ = writeln!(s, "- [{}] {} (verify: {})", c.id, c.text, c.verify);
        }
    }

    list(&mut s, "Non-Functional", &doc.non_functional);
    list(&mut s, "Edge Cases", &doc.edge_cases);
    list(&mut s, "Constraints", &doc.constraints);
    list(&mut s, "Dependencies", &doc.dependencies);
    list(&mut s, "Required Skills", &doc.required_skills);
    list(&mut s, "Success Metrics", &doc.success_metrics);

    if !doc.verification.is_empty() {
        s.push_str("\n## Verification\n");
        for (gate, cmd) in &doc.verification {
            let _ = writeln!(s, "- {gate}: {cmd}");
        }
    }
    if !doc.clarifications.is_empty() {
        s.push_str("\n## Clarifications\n");
        for c in &doc.clarifications {
            let _ = writeln!(s, "- {} :: {}", c.question, c.answer);
        }
    }
    list(&mut s, "Open Questions", &doc.open_questions);
    s
}

fn list(s: &mut String, title: &str, items: &[String]) {
    if items.is_empty() {
        return;
    }
    let _ = write!(s, "\n## {title}\n");
    for i in items {
        let _ = writeln!(s, "- {i}");
    }
}
