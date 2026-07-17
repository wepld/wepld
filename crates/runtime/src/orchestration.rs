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
use wepld_contracts::validation::{
    validate_identifier, MAX_PLAN_TASKS, MAX_SATISFIES_PER_TASK, MAX_TASK_TITLE_BYTES,
    MAX_TOTAL_PLAN_BYTES,
};
use wepld_contracts::vocabulary::EventType;
use wepld_ledger::NewEntry;
use wepld_workspace::Workspace;

/// Semantic + resource validation of a (model-produced) plan at the Core
/// boundary: bounded task count and serialized size, every task id valid +
/// unique, bounded non-empty titles, bounded and duplicate-free `satisfies`
/// resolving to real acceptance-criterion ids, and **every acceptance criterion
/// covered by at least one task**. Enforced before a plan is stored, approved,
/// materialized into task rows, or executed.
fn validate_plan(plan: &PlanDoc, ac_ids: &[String]) -> Result<(), String> {
    if plan.tasks.is_empty() {
        return Err("plan contains no tasks".to_owned());
    }
    if plan.tasks.len() > MAX_PLAN_TASKS {
        return Err(format!(
            "plan has too many tasks ({} > {MAX_PLAN_TASKS})",
            plan.tasks.len()
        ));
    }
    // Reject an oversized serialized plan before any persistence.
    let serialized = serde_json::to_vec(plan).map_err(|e| e.to_string())?;
    if serialized.len() > MAX_TOTAL_PLAN_BYTES {
        return Err(format!(
            "serialized plan too large ({} > {MAX_TOTAL_PLAN_BYTES} bytes)",
            serialized.len()
        ));
    }
    let mut seen_ids = std::collections::HashSet::new();
    let mut covered: std::collections::HashSet<&str> = std::collections::HashSet::new();
    for t in &plan.tasks {
        if let Err(e) = validate_identifier("task id", &t.id) {
            return Err(format!("invalid task id: {e}"));
        }
        if !seen_ids.insert(t.id.as_str()) {
            return Err(format!("duplicate task id: {}", t.id));
        }
        if t.title.trim().is_empty() {
            return Err(format!("task {} has an empty title", t.id));
        }
        if t.title.len() > MAX_TASK_TITLE_BYTES {
            return Err(format!(
                "task {} title too long ({} > {MAX_TASK_TITLE_BYTES} bytes)",
                t.id,
                t.title.len()
            ));
        }
        if t.satisfies.len() > MAX_SATISFIES_PER_TASK {
            return Err(format!(
                "task {} has too many satisfies ({} > {MAX_SATISFIES_PER_TASK})",
                t.id,
                t.satisfies.len()
            ));
        }
        let mut seen_ac = std::collections::HashSet::new();
        for ac in &t.satisfies {
            if !seen_ac.insert(ac.as_str()) {
                return Err(format!("task {} has a duplicate satisfies {ac:?}", t.id));
            }
            if !ac_ids.iter().any(|a| a == ac) {
                return Err(format!(
                    "task {} references unknown acceptance criterion {ac:?}",
                    t.id
                ));
            }
            covered.insert(ac.as_str());
        }
    }
    // Coverage: every acceptance criterion must be satisfied by some task.
    for ac in ac_ids {
        if !covered.contains(ac.as_str()) {
            return Err(format!(
                "acceptance criterion {ac:?} is not covered by any task"
            ));
        }
    }
    Ok(())
}

/// Acceptance-criterion ids from a stored brief JSON value.
fn brief_ac_ids(brief: &serde_json::Value) -> Vec<String> {
    brief["acceptance_criteria"]
        .as_array()
        .map(|a| {
            a.iter()
                .filter_map(|c| c["id"].as_str().map(str::to_owned))
                .collect()
        })
        .unwrap_or_default()
}

/// An injectable crash point in the acceptance sequence, used to prove
/// effect/ledger recovery (Blocker 3). Production always uses `None`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AcceptFault {
    None,
    /// Stop after the decision is committed, before the proposal-ref effect.
    BeforeEffect,
    /// Stop after the proposal-ref effect, before `MissionAccepted` is recorded.
    BeforeFinalRecord,
}

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
        // DEV preflight: no planner worker spawns for an unauthorized repository.
        if let Err(reason) = self.dev_preflight(mission_id) {
            return Ok(rejected(reason));
        }

        let attempt_id = format!("att_{mission_id}_plan");
        let pack = planner_pack(&brief);
        let spec = PhaseSpec {
            mission_id: mission_id.to_owned(),
            task_id: format!("{mission_id}_planning"),
            attempt_id: attempt_id.clone(),
            phase: "plan".to_owned(),
            repo: brief["scope"]["repo"]
                .as_str()
                .unwrap_or_default()
                .to_owned(),
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
        // The plan is untrusted model output: validate it semantically before it
        // is stored or ever materialized into tasks/attempts/paths.
        if let Err(reason) = validate_plan(&plan_doc, &brief_ac_ids(&brief)) {
            return Ok(rejected(reason));
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
    /// mission to running. **Requires an explicit, authenticated approver** — a
    /// caller-supplied human decision. The recorded `PlanApproved` fact carries
    /// that real principal; the Core never fabricates a human actor.
    pub fn approve_plan(
        &mut self,
        mission_id: &str,
        approver: &str,
    ) -> Result<CommandOutcome, RuntimeError> {
        if !crate::is_authenticated_principal(approver) {
            return Ok(rejected(format!(
                "plan approval requires an authenticated principal, got {approver:?}"
            )));
        }
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
        // Defense in depth: re-validate the plan before materializing task rows.
        let ac_ids = self
            .store_brief(mission_id)?
            .map(|b| brief_ac_ids(&b))
            .unwrap_or_default();
        if let Err(reason) = validate_plan(&plan_doc, &ac_ids) {
            return Ok(rejected(reason));
        }

        let task_count = plan_doc.tasks.len();
        let mid = mission_id.to_owned();
        let approver_s = approver.to_owned();
        self.store_mut().transact(|tx| {
            tx.approve_plan_row(&plan_id, &approver_s)?;
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
                actor_id: approver_s.clone(),
                correlation_id: mid.clone(),
                causation_ref: None,
                payload: serde_json::json!({
                    "plan_id": plan_id, "task_count": task_count, "approved_by": approver_s
                }),
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

        // DEV-tier caps (IADR-0003): Manual mode only, fixture repos unless an
        // explicit override was granted. Enforced before any worker runs.
        if let Err(reason) = self.dev_tier_gate(&brief.scope.repo, brief.autonomy_mode) {
            return Ok(rejected(reason));
        }

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
                repo: brief.scope.repo.clone(),
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

    /// Accept a proposed completion by an explicit, authenticated approver.
    /// **V0 never merges into the base branch or touches the primary worktree:**
    /// it produces a reviewable *proposal ref* (`refs/heads/wepld/mission-<id>`)
    /// at the final snapshot, for a human to merge later through an external
    /// protected workflow. Recovery-safe and idempotent (see `accept_mission_at`).
    pub fn accept_mission(
        &mut self,
        mission_id: &str,
        approver: &str,
    ) -> Result<CommandOutcome, RuntimeError> {
        self.accept_mission_at(mission_id, approver, AcceptFault::None)
    }

    /// The acceptance state machine with an injectable crash point. Production
    /// callers use [`Core::accept_mission`] (`AcceptFault::None`); the fault
    /// variants exist to prove recovery: a crash between the recorded decision
    /// and the effect, or between the effect and the final record, always heals
    /// idempotently on retry with no false `MissionAccepted` and no base mutation.
    pub fn accept_mission_at(
        &mut self,
        mission_id: &str,
        approver: &str,
        fault: AcceptFault,
    ) -> Result<CommandOutcome, RuntimeError> {
        if !crate::is_authenticated_principal(approver) {
            return Ok(rejected(format!(
                "completion acceptance requires an authenticated principal, got {approver:?}"
            )));
        }
        let Some((_title, state)) = self.mission_row(mission_id)? else {
            return Ok(rejected(format!("no such mission: {mission_id}")));
        };

        // Idempotent replay: a duplicate acceptance returns the recorded fact —
        // no second MissionAccepted, no repeated effect.
        if state == "accepted" {
            return Ok(accepted(self.recorded_acceptance_detail(mission_id)?));
        }

        let Some(snapshot_commit) = self
            .timeline(mission_id)?
            .into_iter()
            .rev()
            .find(|e| e.entry_type == EventType::WorkspaceSnapshotRecorded)
            .and_then(|e| e.payload_json["commit"].as_str().map(str::to_owned))
        else {
            return Ok(rejected("no workspace snapshot to accept".to_owned()));
        };
        let proposal_ref = format!("refs/heads/wepld/mission-{mission_id}");

        match state.as_str() {
            "completion_proposed" => {
                // 1. Durably record the explicit human decision + intended effect
                //    and move to `acceptance_pending` BEFORE any git effect.
                let mid = mission_id.to_owned();
                let approver_s = approver.to_owned();
                let intent = serde_json::json!({
                    "decision": "approve_completion",
                    "approved_by": approver_s,
                    "intended_effect": {
                        "kind": "create_proposal_ref",
                        "proposal_ref": proposal_ref,
                        "snapshot_commit": snapshot_commit,
                    }
                });
                self.store_mut().transact(|tx| {
                    tx.append(&NewEntry {
                        entry_type: EventType::DecisionResolved,
                        schema_version: 1,
                        aggregate_type: AggregateType::Mission,
                        aggregate_id: mid.clone(),
                        actor_type: ActorType::Human,
                        actor_id: approver_s.clone(),
                        correlation_id: mid.clone(),
                        causation_ref: None,
                        payload: intent,
                    })?;
                    tx.set_mission_state(&mid, "acceptance_pending")?;
                    Ok(())
                })?;
                if fault == AcceptFault::BeforeEffect {
                    // Simulated crash after the decision, before the effect.
                    return Ok(CommandOutcome::Deferred {
                        reason: "acceptance pending (interrupted before effect)".to_owned(),
                    });
                }
                self.finalize_acceptance(
                    mission_id,
                    approver,
                    &snapshot_commit,
                    &proposal_ref,
                    fault,
                )
            }
            // Recovery: the decision is already durable. Reuse the ORIGINAL
            // recorded approver — a retry caller must not rewrite who approved.
            "acceptance_pending" | "acceptance_uncertain" => {
                match self.recorded_decision_approver(mission_id)? {
                    Some(original) if original == approver => self.finalize_acceptance(
                        mission_id,
                        &original,
                        &snapshot_commit,
                        &proposal_ref,
                        fault,
                    ),
                    Some(original) => Ok(rejected(format!(
                        "acceptance retry approver {approver:?} does not match the recorded \
                         approver {original:?}; refusing to rewrite the decision"
                    ))),
                    None => Ok(rejected(
                        "no recorded completion decision to recover".to_owned(),
                    )),
                }
            }
            other => Ok(rejected(format!(
                "mission is {other}, expected completion_proposed before acceptance"
            ))),
        }
    }

    /// Steps 3–6: perform the reversible proposal-ref effect, probe the real git
    /// state, and record `MissionAccepted` (or an explicit uncertain state).
    /// Idempotent — safe to call repeatedly during recovery.
    fn finalize_acceptance(
        &mut self,
        mission_id: &str,
        approver: &str,
        snapshot_commit: &str,
        proposal_ref: &str,
        fault: AcceptFault,
    ) -> Result<CommandOutcome, RuntimeError> {
        let brief: MissionBrief = serde_json::from_value(
            self.store_brief(mission_id)?
                .expect("pending mission has brief"),
        )?;
        let base_before = self.base_branch_commit(&brief)?;
        let ws = Workspace::open(std::path::Path::new(&brief.scope.repo))?;

        // 3. Conflict-safe effect: compare-and-swap the proposal ref. Never
        //    force-overwrite a ref already pointing elsewhere (Blocker 3).
        let refname = format!("wepld/mission-{mission_id}");
        match ws.branch_commit(&refname)? {
            // Already at the intended snapshot — idempotent recovery no-op.
            Some(c) if c == snapshot_commit => {}
            // A conflicting ref: record uncertain, do not overwrite, defer.
            Some(other) => {
                return self.record_acceptance_conflict(mission_id, snapshot_commit, &other)
            }
            // Absent: create atomically (expected-old = zero). A lost race is a
            // conflict, not a silent overwrite.
            None => {
                if ws.propose_ref(mission_id, snapshot_commit, None).is_err() {
                    let now = ws.branch_commit(&refname)?.unwrap_or_default();
                    return self.record_acceptance_conflict(mission_id, snapshot_commit, &now);
                }
            }
        }
        if fault == AcceptFault::BeforeFinalRecord {
            // Simulated crash after the effect, before the final record.
            return Ok(CommandOutcome::Deferred {
                reason: "acceptance pending (interrupted before final record)".to_owned(),
            });
        }

        // 4. Probe the actual git state rather than trusting the effect call.
        let observed = ws.branch_commit(&refname)?;
        let base_after = self.base_branch_commit(&brief)?;
        if base_after != base_before {
            // A base-branch change would be a governance violation; refuse to
            // record acceptance and flag it.
            let mid = mission_id.to_owned();
            self.store_mut().transact(|tx| {
                tx.set_mission_state(&mid, "acceptance_uncertain")?;
                tx.append(&NewEntry {
                    entry_type: EventType::AttemptUncertain,
                    schema_version: 1,
                    aggregate_type: AggregateType::Mission,
                    aggregate_id: mid.clone(),
                    actor_type: ActorType::Core,
                    actor_id: "core".to_owned(),
                    correlation_id: mid,
                    causation_ref: None,
                    payload: serde_json::json!({
                        "reason": "base branch changed during acceptance",
                        "base_before": base_before, "base_after": base_after,
                    }),
                })?;
                Ok(())
            })?;
            return Ok(CommandOutcome::Deferred {
                reason: "acceptance uncertain: base branch changed".to_owned(),
            });
        }
        if observed.as_deref() != Some(snapshot_commit) {
            // 6. Uncertain outcome: record an explicit uncertain state + evidence.
            let mid = mission_id.to_owned();
            let obs = observed.clone().unwrap_or_default();
            let snap = snapshot_commit.to_owned();
            self.store_mut().transact(|tx| {
                tx.set_mission_state(&mid, "acceptance_uncertain")?;
                tx.append(&NewEntry {
                    entry_type: EventType::AttemptUncertain,
                    schema_version: 1,
                    aggregate_type: AggregateType::Mission,
                    aggregate_id: mid.clone(),
                    actor_type: ActorType::Core,
                    actor_id: "core".to_owned(),
                    correlation_id: mid,
                    causation_ref: None,
                    payload: serde_json::json!({
                        "reason": "proposal ref not at expected snapshot",
                        "expected": snap, "observed": obs,
                    }),
                })?;
                Ok(())
            })?;
            return Ok(CommandOutcome::Deferred {
                reason: "acceptance uncertain: proposal ref mismatch".to_owned(),
            });
        }

        // 5. Effect confirmed → record MissionAccepted (real approver) + accept.
        let detail = serde_json::json!({
            "mission_id": mission_id,
            "state": "accepted",
            "approved_by": approver,
            "proposal_ref": proposal_ref,
            "proposal_commit": snapshot_commit,
            "snapshot_commit": snapshot_commit,
            "merged": false,
        });
        let mid = mission_id.to_owned();
        let payload = detail.clone();
        self.store_mut().transact(|tx| {
            tx.append(&NewEntry {
                entry_type: EventType::MissionAccepted,
                schema_version: 1,
                aggregate_type: AggregateType::Mission,
                aggregate_id: mid.clone(),
                actor_type: ActorType::Human,
                actor_id: approver.to_owned(),
                correlation_id: mid.clone(),
                causation_ref: None,
                payload,
            })?;
            tx.set_mission_state(&mid, "accepted")?;
            Ok(())
        })?;
        Ok(accepted(detail))
    }

    /// A proposal ref already points somewhere unexpected — refuse to overwrite,
    /// record an explicit conflict/uncertain state, and defer (no MissionAccepted).
    fn record_acceptance_conflict(
        &mut self,
        mission_id: &str,
        expected: &str,
        observed: &str,
    ) -> Result<CommandOutcome, RuntimeError> {
        let mid = mission_id.to_owned();
        let exp = expected.to_owned();
        let obs = observed.to_owned();
        self.store_mut().transact(|tx| {
            tx.set_mission_state(&mid, "acceptance_uncertain")?;
            tx.append(&NewEntry {
                entry_type: EventType::AttemptUncertain,
                schema_version: 1,
                aggregate_type: AggregateType::Mission,
                aggregate_id: mid.clone(),
                actor_type: ActorType::Core,
                actor_id: "core".to_owned(),
                correlation_id: mid,
                causation_ref: None,
                payload: serde_json::json!({
                    "reason": "proposal ref conflict; refusing to overwrite",
                    "expected": exp, "observed": obs,
                }),
            })?;
            Ok(())
        })?;
        Ok(CommandOutcome::Deferred {
            reason: "acceptance uncertain: proposal ref conflict (not overwritten)".to_owned(),
        })
    }

    /// The approver recorded in the mission's `DecisionResolved` fact — the
    /// original human decision. Recovery reuses this; a retry caller may not
    /// replace it.
    fn recorded_decision_approver(&self, mission_id: &str) -> Result<Option<String>, RuntimeError> {
        Ok(self
            .timeline(mission_id)?
            .into_iter()
            .rev()
            .find(|e| {
                e.entry_type == EventType::DecisionResolved
                    && e.payload_json["decision"] == "approve_completion"
            })
            .and_then(|e| e.payload_json["approved_by"].as_str().map(str::to_owned)))
    }

    /// Return a proposed completion instead of accepting it (an explicit human
    /// decision). Records `MissionReturned` with the real approver.
    pub fn return_mission(
        &mut self,
        mission_id: &str,
        approver: &str,
        reason: &str,
    ) -> Result<CommandOutcome, RuntimeError> {
        if !crate::is_authenticated_principal(approver) {
            return Ok(rejected(
                "returning a completion requires an authenticated principal".to_owned(),
            ));
        }
        let Some((_t, state)) = self.mission_row(mission_id)? else {
            return Ok(rejected(format!("no such mission: {mission_id}")));
        };
        if state != "completion_proposed" {
            return Ok(rejected(format!(
                "mission is {state}, expected completion_proposed before return"
            )));
        }
        let mid = mission_id.to_owned();
        let approver_s = approver.to_owned();
        let reason_s = reason.to_owned();
        self.store_mut().transact(|tx| {
            tx.append(&NewEntry {
                entry_type: EventType::MissionReturned,
                schema_version: 1,
                aggregate_type: AggregateType::Mission,
                aggregate_id: mid.clone(),
                actor_type: ActorType::Human,
                actor_id: approver_s.clone(),
                correlation_id: mid.clone(),
                causation_ref: None,
                payload: serde_json::json!({ "returned_by": approver_s, "reason": reason_s }),
            })?;
            tx.set_mission_state(&mid, "returned")?;
            Ok(())
        })?;
        Ok(accepted(
            serde_json::json!({ "mission_id": mission_id, "state": "returned" }),
        ))
    }

    /// The recorded acceptance detail (from the `MissionAccepted` fact) — used
    /// for idempotent replay of a duplicate acceptance.
    fn recorded_acceptance_detail(
        &self,
        mission_id: &str,
    ) -> Result<serde_json::Value, RuntimeError> {
        let detail = self
            .timeline(mission_id)?
            .into_iter()
            .rev()
            .find(|e| e.entry_type == EventType::MissionAccepted)
            .map(|e| e.payload_json)
            .unwrap_or_else(
                || serde_json::json!({ "mission_id": mission_id, "state": "accepted" }),
            );
        Ok(detail)
    }

    /// The current commit of a mission's base branch (for the no-mutation probe).
    fn base_branch_commit(&self, brief: &MissionBrief) -> Result<Option<String>, RuntimeError> {
        let ws = Workspace::open(std::path::Path::new(&brief.scope.repo))?;
        Ok(ws.branch_commit(&brief.scope.base_branch)?)
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
