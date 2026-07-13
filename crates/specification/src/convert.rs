//! Mission Conversion — the deterministic transform from a canonical
//! specification into a WePLD `MissionBrief` + `PlanDoc`. Pure: same inputs
//! (same document hash) → same mission, so spec-derived missions are
//! replayable. The Runtime performs the CAS capture, lineage, and mission
//! creation; this function only computes.

use std::collections::BTreeMap;
use wepld_contracts::mission::{
    AcceptanceCriterion, AutonomyMode, Budget, Classification, DeclaredEnvelope, MissionBrief,
    PlanDoc, Scope, TaskSpec,
};
use wepld_contracts::specification::{SpecProvenance, SpecSourceRef, SpecificationDocument};

use crate::validate::validate;

/// Inputs the Runtime supplies alongside the canonical document.
pub struct ConvertInput<'a> {
    pub doc: &'a SpecificationDocument,
    pub spec_id: &'a str,
    pub version: u32,
    pub document_hash: &'a str,
    pub slug: &'a str,
    pub repo: &'a str,
    pub base_branch: &'a str,
    pub paths: Vec<String>,
}

pub struct Conversion {
    pub brief: MissionBrief,
    pub plan: PlanDoc,
    pub provenance: SpecProvenance,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConversionError {
    /// The specification failed validation (codes listed).
    Invalid(Vec<String>),
    /// An acceptance criterion references a gate with no verification command.
    MissingGateCommand(String),
}

/// Convert a validated specification into a runnable mission. Deterministic.
pub fn convert(input: ConvertInput) -> Result<Conversion, ConversionError> {
    let doc = input.doc;

    let report = validate(doc);
    if !report.valid {
        return Err(ConversionError::Invalid(
            report.issues.into_iter().map(|i| i.code).collect(),
        ));
    }

    // Gates: distinct gate names referenced by acceptance criteria, each of
    // which must have a verification command (an executable engineering spec).
    let mut gates: Vec<String> = Vec::new();
    for c in &doc.acceptance_criteria {
        let gate = c
            .verify
            .strip_prefix("gate:")
            .unwrap_or(&c.verify)
            .to_owned();
        if !gates.contains(&gate) {
            gates.push(gate);
        }
    }
    for g in &gates {
        if !doc.verification.contains_key(g) {
            return Err(ConversionError::MissingGateCommand(g.clone()));
        }
    }
    let gate_commands: BTreeMap<String, String> = doc.verification.clone();

    let mission_id = format!("mis_{}_v{}", input.slug, input.version);
    let acceptance_criteria = doc
        .acceptance_criteria
        .iter()
        .map(|c| AcceptanceCriterion {
            id: c.id.clone(),
            text: c.text.clone(),
            verify: c.verify.clone(),
        })
        .collect();

    let brief = MissionBrief {
        schema_version: 1,
        mission_id: mission_id.clone(),
        title: input.slug.replace('-', " "),
        outcome: doc.overview.clone(),
        scope: Scope {
            repo: input.repo.to_owned(),
            base_branch: input.base_branch.to_owned(),
            paths: input.paths,
            forbidden_paths: vec![],
        },
        acceptance_criteria,
        gates_required: gates,
        gate_commands,
        autonomy_mode: AutonomyMode::BoundedAuto,
        envelope_declared: DeclaredEnvelope {
            network: "deny".to_owned(),
            dependency_install: "ask".to_owned(),
            secrets: vec![],
        },
        budget: Budget {
            max_cost_usd: 5.0,
            max_wall_minutes: 90,
            max_interrupts: 3,
        },
        classification: Classification::Internal,
        owner: "principal_local".to_owned(),
    };

    // One build task satisfying every criterion (Spec-C generates richer task
    // graphs via reasoning; the vertical slice runs one deterministic task).
    let satisfies = doc
        .acceptance_criteria
        .iter()
        .map(|c| c.id.clone())
        .collect();
    let plan = PlanDoc {
        tasks: vec![TaskSpec {
            id: "T1".to_owned(),
            title: format!("implement {}", input.slug),
            satisfies,
        }],
    };

    let provenance = SpecProvenance {
        sources: vec![SpecSourceRef {
            spec_id: input.spec_id.to_owned(),
            version: input.version,
            document_hash: input.document_hash.to_owned(),
        }],
    };

    Ok(Conversion {
        brief,
        plan,
        provenance,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use wepld_contracts::specification::SpecAcceptanceCriterion;

    fn runnable_doc() -> SpecificationDocument {
        let mut d = SpecificationDocument {
            overview: "Add a --version flag".to_owned(),
            functional_requirements: vec!["Print version and exit".to_owned()],
            acceptance_criteria: vec![SpecAcceptanceCriterion {
                id: "AC1".to_owned(),
                text: "VERSION present".to_owned(),
                verify: "gate:build".to_owned(),
            }],
            ..Default::default()
        };
        d.verification
            .insert("build".to_owned(), "grep -q VERSION src/main.rs".to_owned());
        d
    }

    fn input(doc: &SpecificationDocument) -> ConvertInput<'_> {
        ConvertInput {
            doc,
            spec_id: "spec_1",
            version: 1,
            document_hash: "hash1",
            slug: "version-flag",
            repo: "/tmp/repo",
            base_branch: "main",
            paths: vec!["src/**".to_owned()],
        }
    }

    #[test]
    fn converts_a_valid_spec_deterministically() {
        let doc = runnable_doc();
        let a = convert(input(&doc)).unwrap();
        let b = convert(input(&doc)).unwrap();
        assert_eq!(a.brief.mission_id, "mis_version-flag_v1");
        assert_eq!(a.brief.outcome, "Add a --version flag");
        assert_eq!(a.brief.gates_required, vec!["build"]);
        assert_eq!(
            a.brief.gate_commands["build"],
            "grep -q VERSION src/main.rs"
        );
        assert_eq!(a.plan.tasks.len(), 1);
        assert_eq!(a.plan.tasks[0].satisfies, vec!["AC1"]);
        assert_eq!(a.provenance.sources[0].document_hash, "hash1");
        // Deterministic.
        assert_eq!(
            serde_json::to_value(&a.brief).unwrap(),
            serde_json::to_value(&b.brief).unwrap()
        );
    }

    #[test]
    fn invalid_spec_is_rejected() {
        let doc = SpecificationDocument::default();
        assert!(matches!(
            convert(input(&doc)),
            Err(ConversionError::Invalid(_))
        ));
    }

    #[test]
    fn missing_gate_command_is_rejected() {
        let mut doc = runnable_doc();
        doc.verification.clear(); // AC1 references gate:build with no command
        assert!(matches!(
            convert(input(&doc)),
            Err(ConversionError::MissingGateCommand(g)) if g == "build"
        ));
    }
}
