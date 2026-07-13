//! Mission orchestration (v2-02 §5): the lifecycle operations that drive a
//! mission through planning, approval, execution, and completion. Unlike the
//! pure command handlers, these spawn worker phases, so they run outside a
//! single transaction — but every state change they make is still recorded
//! as a durable ledger fact in its own transaction.

use crate::phase::PhaseSpec;
use crate::{Core, PhaseOutcome, RuntimeError};
use wepld_contracts::command::CommandOutcome;
use wepld_contracts::ledger::{ActorType, AggregateType};
use wepld_contracts::mission::PlanDoc;
use wepld_contracts::vocabulary::EventType;
use wepld_ledger::NewEntry;

impl Core {
    /// Run the planner phase for a draft mission and record the proposed plan.
    /// The planner is a worker that reasons through the gateway; the plan is
    /// its brain output, captured to the CAS by the phase engine.
    pub fn plan_mission(&mut self, mission_id: &str) -> Result<CommandOutcome, RuntimeError> {
        let Some((_title, state)) = self.mission_row(mission_id)? else {
            return Ok(rejected(format!("no such mission: {mission_id}")));
        };
        if state != "draft" {
            return Ok(rejected(format!(
                "mission is {state}, expected draft before planning"
            )));
        }
        let Some(brief) = self.store_brief(mission_id)? else {
            return Ok(rejected("mission brief missing".to_owned()));
        };

        let attempt_id = format!("att_{mission_id}_plan");
        let pack = planner_pack(&brief);
        let spec = PhaseSpec {
            mission_id: mission_id.to_owned(),
            task_id: format!("{mission_id}_planning"),
            attempt_id: attempt_id.clone(),
            phase: "plan".to_owned(),
            worker_cmd: self.worker_cmd(),
            pack,
            brain_profile: "fixture-default".to_owned(),
            workspace_path: None,
            heartbeat_timeout_ms: 4000,
            deadline_ms: 30_000,
        };
        let run = self.run_phase(&spec)?;
        if run.outcome != PhaseOutcome::Succeeded {
            return Ok(rejected(format!(
                "planning did not complete: {:?}",
                run.outcome
            )));
        }

        // The plan is the planner's brain output, captured by the phase engine.
        let Some(plan_ref) = self
            .brain_invocations(&attempt_id)?
            .into_iter()
            .rev()
            .find(|r| r.intent == "plan" && r.status == "ok")
            .and_then(|r| r.response_artifact)
        else {
            return Ok(rejected(
                "planner produced no accepted plan output".to_owned(),
            ));
        };
        let plan_doc: PlanDoc = serde_json::from_slice(&self.artifact(&plan_ref)?)?;
        if plan_doc.tasks.is_empty() {
            return Ok(rejected("plan contains no tasks".to_owned()));
        }

        let plan_id = format!("plan_{mission_id}_1");
        let task_count = plan_doc.tasks.len();
        let plan_json = serde_json::to_value(&plan_doc)?;
        let payload = serde_json::json!({
            "plan_id": plan_id,
            "plan_ref": plan_ref,
            "task_count": task_count,
        });
        let mid = mission_id.to_owned();
        let pid = plan_id.clone();
        self.store_mut().transact(|tx| {
            tx.insert_plan(&pid, &mid, 1, &plan_json)?;
            tx.append(&NewEntry {
                entry_type: EventType::PlanProposed,
                schema_version: 1,
                aggregate_type: AggregateType::Mission,
                aggregate_id: mid.clone(),
                actor_type: ActorType::Core,
                actor_id: "core".to_owned(),
                correlation_id: mid.clone(),
                causation_ref: None,
                payload,
            })?;
            tx.set_mission_state(&mid, "plan_review")?;
            Ok(())
        })?;

        Ok(accepted(serde_json::json!({
            "mission_id": mission_id,
            "plan_id": plan_id,
            "task_count": task_count,
            "state": "plan_review",
        })))
    }

    /// Approve the latest proposed plan: materialize its tasks and move the
    /// mission to running.
    pub fn approve_plan(&mut self, mission_id: &str) -> Result<CommandOutcome, RuntimeError> {
        let Some((_title, state)) = self.mission_row(mission_id)? else {
            return Ok(rejected(format!("no such mission: {mission_id}")));
        };
        if state != "plan_review" {
            return Ok(rejected(format!(
                "mission is {state}, expected plan_review before approval"
            )));
        }
        let Some((plan_id, plan_json)) = self.store.latest_plan(mission_id)? else {
            return Ok(rejected("no proposed plan to approve".to_owned()));
        };
        let plan_doc: PlanDoc = serde_json::from_value(plan_json)?;

        let task_count = plan_doc.tasks.len();
        let mid = mission_id.to_owned();
        self.store_mut().transact(|tx| {
            tx.approve_plan_row(&plan_id, "principal_local")?;
            for (i, spec) in plan_doc.tasks.iter().enumerate() {
                let task_id = format!("{mid}_{}", spec.id);
                tx.insert_task(
                    &task_id,
                    &mid,
                    &serde_json::to_value(spec)?,
                    "ready",
                    i as i64,
                )?;
            }
            tx.append(&NewEntry {
                entry_type: EventType::PlanApproved,
                schema_version: 1,
                aggregate_type: AggregateType::Mission,
                aggregate_id: mid.clone(),
                actor_type: ActorType::Human,
                actor_id: "principal_local".to_owned(),
                correlation_id: mid.clone(),
                causation_ref: None,
                payload: serde_json::json!({ "plan_id": plan_id, "task_count": task_count }),
            })?;
            tx.set_mission_state(&mid, "running")?;
            Ok(())
        })?;

        Ok(accepted(serde_json::json!({
            "mission_id": mission_id,
            "task_count": task_count,
            "state": "running",
        })))
    }

    /// The stored mission brief, if present.
    pub(crate) fn store_brief(
        &self,
        mission_id: &str,
    ) -> Result<Option<serde_json::Value>, RuntimeError> {
        Ok(self.store.mission_brief(mission_id)?)
    }
}

/// The planner's context pack (single source of truth: orchestration and
/// tests build the identical pack, so cassette keys match).
pub fn planner_pack(brief: &serde_json::Value) -> serde_json::Value {
    serde_json::json!({
        "schema_version": 1,
        "phase": "plan",
        "mission": {
            "title": brief["title"],
            "outcome": brief["outcome"],
            "acceptance_criteria": brief["acceptance_criteria"],
        },
        "request": "decompose this mission into an ordered task list"
    })
}

/// The builder's context pack for a specific task.
pub fn builder_pack(brief: &serde_json::Value, task: &serde_json::Value) -> serde_json::Value {
    serde_json::json!({
        "schema_version": 1,
        "phase": "build",
        "mission": {
            "title": brief["title"],
            "outcome": brief["outcome"],
            "acceptance_criteria": brief["acceptance_criteria"],
        },
        "task": task,
        "request": "produce the file edits that satisfy this task"
    })
}

fn accepted(detail: serde_json::Value) -> CommandOutcome {
    CommandOutcome::Accepted { detail }
}

fn rejected(reason: String) -> CommandOutcome {
    CommandOutcome::Rejected { reason }
}
