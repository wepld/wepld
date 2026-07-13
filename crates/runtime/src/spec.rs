//! Specification orchestration — the Runtime side of the Engineering
//! Specification domain. Captures the canonical document to the CAS, records
//! the spec lifecycle in the ledger, converts it to a mission (via the pure
//! `wepld-specification` domain), and creates that mission through the same
//! single-writer path as any other. The spec never executes and never holds a
//! `Tx` — the Runtime does, here.

use crate::{Core, RuntimeError};
use wepld_contracts::command::CommandOutcome;
use wepld_contracts::ledger::{ActorType, AggregateType};
use wepld_contracts::specification::SpecificationDocument;
use wepld_contracts::vocabulary::EventType;
use wepld_ledger::NewEntry;
use wepld_specification::{convert, ConvertInput};

impl Core {
    /// Create a mission from a canonical specification document. Deterministic:
    /// the same document + slug produce the same spec_id, mission_id, and plan,
    /// so the whole flow is replayable. Leaves the mission in `plan_review`
    /// (approve → run → accept run through the unchanged lifecycle).
    pub fn create_mission_from_spec(
        &mut self,
        doc: &SpecificationDocument,
        slug: &str,
        repo: &str,
        base_branch: &str,
    ) -> Result<CommandOutcome, RuntimeError> {
        let spec_id = format!("spec_{slug}");
        let version = 1u32;

        // Capture the canonical document (the truth) to the CAS.
        let doc_bytes = serde_json::to_vec(doc)?;
        let stored = self.cas().put(&doc_bytes)?;
        let document_hash = stored.hash.clone();

        // Record the specification's creation under its own correlation.
        {
            let sid = spec_id.clone();
            let slug_s = slug.to_owned();
            let dh = document_hash.clone();
            self.store_mut().transact(|tx| {
                tx.append(&NewEntry {
                    entry_type: EventType::SpecificationCreated,
                    schema_version: 1,
                    aggregate_type: AggregateType::Specification,
                    aggregate_id: sid.clone(),
                    actor_type: ActorType::Human,
                    actor_id: "principal_local".to_owned(),
                    correlation_id: sid.clone(),
                    causation_ref: None,
                    payload: serde_json::json!({
                        "slug": slug_s, "version": version,
                        "document_hash": dh, "status": "draft"
                    }),
                })?;
                Ok(())
            })?;
        }

        // Convert (pure). Validation failures reject cleanly.
        let conv = match convert(ConvertInput {
            doc,
            spec_id: &spec_id,
            version,
            document_hash: &document_hash,
            slug,
            repo,
            base_branch,
            paths: vec!["src/**".to_owned()],
        }) {
            Ok(c) => c,
            Err(e) => {
                return Ok(CommandOutcome::Rejected {
                    reason: format!("specification does not convert: {e:?}"),
                })
            }
        };

        let mission_id = conv.brief.mission_id.clone();
        if self.mission_row(&mission_id)?.is_some() {
            return Ok(CommandOutcome::Rejected {
                reason: format!("mission already exists: {mission_id}"),
            });
        }

        let brief_json = serde_json::to_value(&conv.brief)?;
        let plan_json = serde_json::to_value(&conv.plan)?;
        let provenance_json = serde_json::to_value(&conv.provenance)?;
        let plan_id = format!("plan_{mission_id}_1");
        let task_count = conv.plan.tasks.len();
        let title = conv.brief.title.clone();

        // Mission creation + derivation + link + spec-derived plan, one tx.
        let mid = mission_id.clone();
        let sid = spec_id.clone();
        let dh = document_hash.clone();
        self.store_mut().transact(|tx| {
            tx.insert_mission(&mid, &title, "draft", &brief_json)?;
            tx.append(&NewEntry {
                entry_type: EventType::MissionCreated,
                schema_version: 1,
                aggregate_type: AggregateType::Mission,
                aggregate_id: mid.clone(),
                actor_type: ActorType::Human,
                actor_id: "principal_local".to_owned(),
                correlation_id: mid.clone(),
                causation_ref: None,
                payload: serde_json::json!({ "title": title, "source": "specification" }),
            })?;
            tx.append(&NewEntry {
                entry_type: EventType::MissionDerivedFromSpec,
                schema_version: 1,
                aggregate_type: AggregateType::Mission,
                aggregate_id: mid.clone(),
                actor_type: ActorType::Core,
                actor_id: "core".to_owned(),
                correlation_id: mid.clone(),
                causation_ref: None,
                payload: provenance_json,
            })?;
            tx.append(&NewEntry {
                entry_type: EventType::SpecLinked,
                schema_version: 1,
                aggregate_type: AggregateType::Specification,
                aggregate_id: sid.clone(),
                actor_type: ActorType::Core,
                actor_id: "core".to_owned(),
                correlation_id: sid.clone(),
                causation_ref: None,
                payload: serde_json::json!({
                    "kind": "mission", "target_ref": mid, "relation": "creates",
                    "document_hash": dh
                }),
            })?;
            tx.insert_plan(&plan_id, &mid, 1, &plan_json)?;
            tx.append(&NewEntry {
                entry_type: EventType::PlanProposed,
                schema_version: 1,
                aggregate_type: AggregateType::Mission,
                aggregate_id: mid.clone(),
                actor_type: ActorType::Core,
                actor_id: "core".to_owned(),
                correlation_id: mid.clone(),
                causation_ref: None,
                payload: serde_json::json!({
                    "plan_id": plan_id, "task_count": task_count, "source": "specification"
                }),
            })?;
            tx.set_mission_state(&mid, "plan_review")?;
            Ok(())
        })?;

        Ok(CommandOutcome::Accepted {
            detail: serde_json::json!({
                "spec_id": spec_id,
                "mission_id": mission_id,
                "task_count": task_count,
                "state": "plan_review",
            }),
        })
    }
}
