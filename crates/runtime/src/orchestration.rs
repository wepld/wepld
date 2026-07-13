//! Mission orchestration (v2-02 §5): the lifecycle operations that drive a
//! mission through planning, approval, execution, and completion. Unlike the
//! pure command handlers, these spawn worker phases, so they run outside a
//! single transaction — but every state change they make is still recorded
//! as a durable ledger fact in its own transaction.

use crate::gates::run_gate;
use crate::phase::PhaseSpec;
use crate::{Core, PhaseOutcome, RuntimeError};
use wepld_contracts::command::CommandOutcome;
use wepld_contracts::ledger::{ActorType, AggregateType};
use wepld_contracts::mission::{MissionBrief, PlanDoc};
use wepld_contracts::vocabulary::EventType;
use wepld_ledger::NewEntry;
use wepld_workspace::Workspace;

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
            max_brain_calls: 8,
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

    /// Execute the mission's ready tasks: for each, run a build phase in an
    /// isolated worktree, snapshot it, record the diff, run gates, and — when
    /// every required gate passes and every acceptance criterion is satisfied
    /// — propose completion.
    pub fn run_mission(&mut self, mission_id: &str) -> Result<CommandOutcome, RuntimeError> {
        let Some((_title, state)) = self.mission_row(mission_id)? else {
            return Ok(rejected(format!("no such mission: {mission_id}")));
        };
        if state != "running" {
            return Ok(rejected(format!(
                "mission is {state}, expected running (approve a plan first)"
            )));
        }
        let brief_json = self
            .store_brief(mission_id)?
            .expect("running mission has brief");
        let brief: MissionBrief = serde_json::from_value(brief_json.clone())?;

        let ws = match Workspace::open(std::path::Path::new(&brief.scope.repo)) {
            Ok(w) => w,
            Err(e) => {
                return Ok(rejected(format!(
                    "cannot open repository {}: {e}",
                    brief.scope.repo
                )))
            }
        };
        let base = brief.scope.base_branch.clone();
        let worktrees_root = self.root().join("worktrees");

        // Gate name → passed, accumulated across the mission's tasks.
        let mut gate_pass: std::collections::BTreeMap<String, bool> = Default::default();

        let ready: Vec<_> = self
            .tasks(mission_id)?
            .into_iter()
            .filter(|t| t.state == "ready")
            .collect();
        if ready.is_empty() {
            return Ok(rejected("no ready tasks to run".to_owned()));
        }

        for task in ready {
            let task_spec: serde_json::Value = serde_json::from_str(&task.spec_json)?;
            let attempt_id = format!("att_{}_build", task.task_id);
            let worktree = ws.create_worktree(&base, &attempt_id, &worktrees_root)?;

            let spec = PhaseSpec {
                mission_id: mission_id.to_owned(),
                task_id: task.task_id.clone(),
                attempt_id: attempt_id.clone(),
                phase: "build".to_owned(),
                worker_cmd: self.worker_cmd(),
                pack: builder_pack(&brief_json, &task_spec),
                brain_profile: "fixture-default".to_owned(),
                workspace_path: Some(worktree.path.to_string_lossy().into_owned()),
                max_brain_calls: 8,
                heartbeat_timeout_ms: 4000,
                deadline_ms: 30_000,
            };
            let run = self.run_phase(&spec)?;
            if run.outcome != PhaseOutcome::Succeeded {
                self.set_task_state(&task.task_id, "failed")?;
                ws.cleanup(worktree)?;
                return Ok(rejected(format!(
                    "build phase for {} did not complete: {:?}",
                    task.task_id, run.outcome
                )));
            }

            // Snapshot the worktree (ADR-0013) and record the diff artifact.
            let snap = ws.snapshot(&worktree, "build-end")?;
            self.append_fact(
                EventType::WorkspaceSnapshotRecorded,
                AggregateType::Attempt,
                &attempt_id,
                mission_id,
                serde_json::json!({ "ref": snap.name, "commit": snap.commit, "task_id": task.task_id }),
            )?;
            let diff = ws.diff(&base, &snap.commit)?;
            let diff_ref = self.cas().put(diff.as_bytes())?;
            self.append_fact(
                EventType::ArtifactRecorded,
                AggregateType::Task,
                &task.task_id,
                mission_id,
                serde_json::json!({ "kind": "diff", "hash": diff_ref.hash, "task_id": task.task_id }),
            )?;

            // Run the required gates in the worktree.
            for gate in &brief.gates_required {
                let Some(command) = brief.gate_commands.get(gate) else {
                    return Ok(rejected(format!(
                        "gate '{gate}' has no command in the brief"
                    )));
                };
                let result = run_gate(&worktree.path, command);
                let log_ref = self.cas().put(result.log.as_bytes())?;
                let status = if result.passed { "passed" } else { "failed" };
                gate_pass
                    .entry(gate.clone())
                    .and_modify(|p| *p = *p && result.passed)
                    .or_insert(result.passed);
                self.append_fact(
                    EventType::GateEvaluated,
                    AggregateType::Task,
                    &task.task_id,
                    mission_id,
                    serde_json::json!({
                        "gate": gate,
                        "status": status,
                        "exit_code": result.exit_code,
                        "log_ref": log_ref.hash,
                        "command": command,
                    }),
                )?;
            }

            let task_passed = brief
                .gates_required
                .iter()
                .all(|g| gate_pass.get(g).copied().unwrap_or(false));
            let task_state = if task_passed { "completed" } else { "failed" };
            self.set_task_state(&task.task_id, task_state)?;
            self.append_fact(
                EventType::TaskCompleted,
                AggregateType::Task,
                &task.task_id,
                mission_id,
                serde_json::json!({ "task_id": task.task_id, "state": task_state }),
            )?;
            ws.cleanup(worktree)?;
        }

        // Acceptance-criteria evaluation: each criterion names a gate.
        let criteria: Vec<serde_json::Value> = brief
            .acceptance_criteria
            .iter()
            .map(|c| {
                let gate = c.verify.strip_prefix("gate:").unwrap_or(&c.verify);
                let met = gate_pass.get(gate).copied().unwrap_or(false);
                serde_json::json!({ "id": c.id, "gate": gate, "met": met })
            })
            .collect();
        let all_met = criteria.iter().all(|c| c["met"] == serde_json::json!(true));
        let all_gates = brief
            .gates_required
            .iter()
            .all(|g| gate_pass.get(g).copied().unwrap_or(false));

        if all_met && all_gates {
            let mid = mission_id.to_owned();
            let payload =
                serde_json::json!({ "criteria": criteria, "state": "completion_proposed" });
            self.store_mut().transact(|tx| {
                tx.append(&NewEntry {
                    entry_type: EventType::CompletionProposed,
                    schema_version: 1,
                    aggregate_type: AggregateType::Mission,
                    aggregate_id: mid.clone(),
                    actor_type: ActorType::Core,
                    actor_id: "core".to_owned(),
                    correlation_id: mid.clone(),
                    causation_ref: None,
                    payload,
                })?;
                tx.set_mission_state(&mid, "completion_proposed")?;
                Ok(())
            })?;
            Ok(accepted(serde_json::json!({
                "mission_id": mission_id,
                "state": "completion_proposed",
                "criteria": criteria,
            })))
        } else {
            Ok(accepted(serde_json::json!({
                "mission_id": mission_id,
                "state": "running",
                "criteria": criteria,
                "note": "gates or criteria unmet; mission remains running",
            })))
        }
    }

    /// Accept a proposed completion. With `merge`, merge the final workspace
    /// snapshot into the base branch — the completion hard gate — then record
    /// MissionAccepted and mark the mission accepted.
    pub fn accept_mission(
        &mut self,
        mission_id: &str,
        merge: bool,
    ) -> Result<CommandOutcome, RuntimeError> {
        let Some((_title, state)) = self.mission_row(mission_id)? else {
            return Ok(rejected(format!("no such mission: {mission_id}")));
        };
        if state != "completion_proposed" {
            return Ok(rejected(format!(
                "mission is {state}, expected completion_proposed before acceptance"
            )));
        }

        // The final snapshot is the latest WorkspaceSnapshotRecorded fact.
        let Some(snapshot_commit) = self
            .timeline(mission_id)?
            .into_iter()
            .rev()
            .find(|e| e.entry_type == EventType::WorkspaceSnapshotRecorded)
            .and_then(|e| e.payload_json["commit"].as_str().map(str::to_owned))
        else {
            return Ok(rejected("no workspace snapshot to accept".to_owned()));
        };

        let brief: MissionBrief = serde_json::from_value(
            self.store_brief(mission_id)?
                .expect("completion-proposed mission has brief"),
        )?;

        let mut detail = serde_json::json!({
            "mission_id": mission_id,
            "merge": merge,
            "snapshot_commit": snapshot_commit,
            "state": "accepted",
        });
        let mut payload = detail.clone();

        if merge {
            let ws = Workspace::open(std::path::Path::new(&brief.scope.repo))?;
            let merge_commit = ws.merge(
                &snapshot_commit,
                &format!("wepld: accept mission {mission_id}"),
            )?;
            detail["merge_commit"] = serde_json::json!(merge_commit);
            payload["merge_commit"] = serde_json::json!(merge_commit);
        }

        let mid = mission_id.to_owned();
        self.store_mut().transact(|tx| {
            tx.append(&NewEntry {
                entry_type: EventType::MissionAccepted,
                schema_version: 1,
                aggregate_type: AggregateType::Mission,
                aggregate_id: mid.clone(),
                actor_type: ActorType::Human,
                actor_id: "principal_local".to_owned(),
                correlation_id: mid.clone(),
                causation_ref: None,
                payload,
            })?;
            tx.set_mission_state(&mid, "accepted")?;
            Ok(())
        })?;

        Ok(accepted(detail))
    }

    /// The stored mission brief, if present.
    pub(crate) fn store_brief(
        &self,
        mission_id: &str,
    ) -> Result<Option<serde_json::Value>, RuntimeError> {
        Ok(self.store.mission_brief(mission_id)?)
    }

    fn set_task_state(&mut self, task_id: &str, state: &str) -> Result<(), RuntimeError> {
        let id = task_id.to_owned();
        let st = state.to_owned();
        self.store_mut()
            .transact(|tx| tx.set_task_state(&id, &st))?;
        Ok(())
    }

    /// Append a single durable fact in its own transaction.
    pub(crate) fn append_fact(
        &mut self,
        entry_type: EventType,
        aggregate_type: AggregateType,
        aggregate_id: &str,
        mission_id: &str,
        payload: serde_json::Value,
    ) -> Result<(), RuntimeError> {
        let agg = aggregate_id.to_owned();
        let mid = mission_id.to_owned();
        self.store_mut().transact(|tx| {
            tx.append(&NewEntry {
                entry_type,
                schema_version: 1,
                aggregate_type,
                aggregate_id: agg,
                actor_type: ActorType::Core,
                actor_id: "core".to_owned(),
                correlation_id: mid,
                causation_ref: None,
                payload,
            })?;
            Ok(())
        })?;
        Ok(())
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
