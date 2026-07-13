//! Engineering Recipes — executable engineering knowledge. The reference
//! recipe is **Build Feature**: the user states a feature; the recipe
//! orchestrates specify → (clarify) → convert → mission → Hermes → evidence →
//! accept → report, hiding every internal step. It owns no execution and no
//! state — it composes the Runtime, which performs and records everything.
//!
//! This is one concrete recipe, not a recipe engine (no manifest/SDK/registry
//! — that would be dead infrastructure). When a second recipe earns its keep,
//! the shared shape can be extracted.

use crate::{Core, EngineeringReport, RuntimeError};
use wepld_contracts::brain::BrainStatus;
use wepld_contracts::command::CommandOutcome;
use wepld_contracts::ledger::{ActorType, AggregateType};
use wepld_contracts::specification::SpecificationDocument;
use wepld_contracts::vocabulary::EventType;
use wepld_ledger::NewEntry;

/// The result a user sees from running a recipe.
pub enum RecipeOutcome {
    /// The feature was implemented; here is the evidence-derived report.
    Completed(Box<EngineeringReport>),
    /// Reasoning could not resolve everything — Hermes asks the user (only
    /// when reasoning and evidence are insufficient).
    NeedsClarification {
        slug: String,
        questions: Vec<String>,
    },
    Rejected(String),
}

impl Core {
    /// The Build Feature recipe. Deterministic under cassettes (fixture-first);
    /// uses a real local model when one is configured. The user never sees the
    /// words specification, clarification, plan, or task.
    pub fn run_build_feature(
        &mut self,
        request: &str,
        slug: &str,
        repo: &str,
        base: &str,
    ) -> Result<RecipeOutcome, RuntimeError> {
        // 1. Hermes reasons a specification from the request.
        let Some(doc) = self.reason_spec_from_request(request, slug)? else {
            return Ok(RecipeOutcome::Rejected(
                "Hermes could not produce a specification for this request \
                 (no reasoning provider available)."
                    .to_owned(),
            ));
        };

        // 2. Clarification gate: unresolved questions go back to the user.
        if !doc.open_questions.is_empty() {
            return Ok(RecipeOutcome::NeedsClarification {
                slug: slug.to_owned(),
                questions: doc.open_questions.clone(),
            });
        }

        // 3. Specification → Mission (deterministic conversion).
        let mission_id = match self.create_mission_from_spec(&doc, slug, repo, base)? {
            CommandOutcome::Accepted { detail } => {
                detail["mission_id"].as_str().unwrap_or_default().to_owned()
            }
            CommandOutcome::Rejected { reason } => return Ok(RecipeOutcome::Rejected(reason)),
            other => return Ok(RecipeOutcome::Rejected(format!("{other:?}"))),
        };

        // 4. Execute through the unchanged Runtime lifecycle.
        if let CommandOutcome::Rejected { reason } = self.approve_plan(&mission_id)? {
            return Ok(RecipeOutcome::Rejected(reason));
        }
        if let CommandOutcome::Rejected { reason } = self.run_mission(&mission_id)? {
            return Ok(RecipeOutcome::Rejected(reason));
        }
        // Accept only if execution actually proposed completion (gates green).
        let state = self
            .mission_row(&mission_id)?
            .map(|(_, s)| s)
            .unwrap_or_default();
        if state == "completion_proposed" {
            self.accept_mission(&mission_id, true)?;
        }

        // 5. The evidence-derived engineering report.
        Ok(RecipeOutcome::Completed(Box::new(
            self.engineering_report(&mission_id)?,
        )))
    }

    /// Reason a specification document from a natural-language request through
    /// the gateway. Records the reasoning as a `BrainInvoked` fact under the
    /// spec's correlation. Returns `None` if reasoning was unavailable/invalid.
    fn reason_spec_from_request(
        &mut self,
        request: &str,
        slug: &str,
    ) -> Result<Option<SpecificationDocument>, RuntimeError> {
        let spec_id = format!("spec_{slug}");
        let pack = serde_json::json!({
            "schema_version": 1, "intent": "specify", "request": request
        });
        let pack_ref = self.cas().put(&serde_json::to_vec(&pack)?)?;
        let invocation_id = format!("brn_{spec_id}_specify");

        let result = self.gateway().invoke(
            &invocation_id,
            "fixture-default",
            "specify",
            &pack,
            &pack_ref.hash,
            "specification.v1",
        )?;
        let response_ref = self.cas().put(&serde_json::to_vec(&result.output)?)?;
        let status = serde_json::to_value(result.status)?
            .as_str()
            .unwrap_or("unknown")
            .to_owned();

        let sid = spec_id.clone();
        let payload = serde_json::json!({
            "invocation_id": invocation_id, "intent": "specify",
            "pack_hash": pack_ref.hash, "response_artifact": response_ref.hash,
            "status": status, "provider": result.usage.provider, "model": result.usage.model,
        });
        self.store_mut().transact(|tx| {
            tx.append(&NewEntry {
                entry_type: EventType::BrainInvoked,
                schema_version: 1,
                aggregate_type: AggregateType::Specification,
                aggregate_id: sid.clone(),
                actor_type: ActorType::BrainAdapter,
                actor_id: "gateway".to_owned(),
                correlation_id: sid.clone(),
                causation_ref: None,
                payload,
            })?;
            Ok(())
        })?;

        if result.status != BrainStatus::Ok {
            return Ok(None);
        }
        Ok(serde_json::from_value::<SpecificationDocument>(result.output).ok())
    }
}
