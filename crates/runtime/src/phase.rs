//! Phase engine v0 (v2-02 §5): spawn a WWP worker for one phase, stream its
//! events, and record every ending honestly. A worker that vanishes is never
//! assumed to have failed cleanly — it is UNCERTAIN until probed (v2-02 §7).

use crate::{Core, RuntimeError};
use std::time::{Duration, Instant};
use wepld_contracts::envelope::{Envelope, FsScope, NetworkPolicy, ProcessLimits, SandboxTier};
use wepld_contracts::ledger::{ActorType, AggregateType};
use wepld_contracts::vocabulary::EventType;
use wepld_contracts::wwp::{
    ArtifactRef, AttemptStart, PhaseBudget, PhaseStatus, RoleProfile, WwpMessage,
};
use wepld_ledger::NewEntry;
use wepld_wwp::{spawn_worker, WorkerEvent};

pub struct PhaseSpec {
    pub mission_id: String,
    pub task_id: String,
    pub attempt_id: String,
    pub phase: String,
    /// Worker program + args (the Runtime decides which WWP runtime runs).
    pub worker_cmd: Vec<String>,
    pub heartbeat_timeout_ms: u64,
    pub deadline_ms: u64,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PhaseOutcome {
    Succeeded,
    Failed,
    Cancelled,
    /// The worker ended without a result; observable state was recorded and
    /// the Runtime (or a human) must decide the recovery path.
    Uncertain(String),
}

impl Core {
    /// Run one stub phase end-to-end (Day-5 engine; context packs and real
    /// envelopes deepen on Days 6–8).
    pub fn run_phase_stub(&mut self, spec: &PhaseSpec) -> Result<PhaseOutcome, RuntimeError> {
        let envelope = dev_envelope(&spec.attempt_id);
        let start = AttemptStart {
            attempt_id: spec.attempt_id.clone(),
            task_id: spec.task_id.clone(),
            phase: spec.phase.clone(),
            role_profile: RoleProfile {
                name: "stub".to_owned(),
                version: 0,
                brain_profile: "none".to_owned(),
                skills: vec![],
            },
            context_pack_ref: ArtifactRef {
                artifact: "none".to_owned(),
                hash: "0".repeat(64),
            },
            envelope: envelope.clone(),
            gates: vec![],
            budget: PhaseBudget {
                max_brain_calls: 0,
                max_wall_minutes: 1,
            },
            idempotency_key: format!("{}:1", spec.attempt_id),
        };

        // Attempt row + AttemptSpawned fact, one transaction.
        let envelope_json = serde_json::to_string(&envelope)?;
        self.transact_phase_entry(spec, EventType::AttemptSpawned, None, |tx| {
            tx.insert_attempt(
                &spec.attempt_id,
                &spec.task_id,
                &spec.phase,
                "stub",
                &envelope_json,
                &start.idempotency_key,
            )
        })?;

        let mut handle = match spawn_worker(
            &spec.worker_cmd,
            &start,
            Duration::from_millis(spec.heartbeat_timeout_ms),
        ) {
            Ok(h) => h,
            Err(e) => {
                let reason = format!("spawn failed: {e}");
                self.record_attempt_end(spec, "failed", EventType::AttemptCompleted, &reason)?;
                return Ok(PhaseOutcome::Failed);
            }
        };
        self.transact_phase_entry(spec, EventType::PhaseStarted, None, |_| Ok(()))?;

        let deadline = Instant::now() + Duration::from_millis(spec.deadline_ms);
        loop {
            let remaining = deadline.saturating_duration_since(Instant::now());
            if remaining.is_zero() {
                handle.kill();
                let reason = "phase deadline exceeded".to_owned();
                self.record_attempt_end(spec, "uncertain", EventType::AttemptUncertain, &reason)?;
                return Ok(PhaseOutcome::Uncertain(reason));
            }
            match handle.events.recv_timeout(remaining) {
                Ok(WorkerEvent::Message(WwpMessage::Heartbeat(_))) => continue,
                Ok(WorkerEvent::Message(WwpMessage::PhaseResult(result))) => {
                    let _ = handle.wait_exit();
                    let (state, outcome) = match result.status {
                        PhaseStatus::Succeeded => ("succeeded", PhaseOutcome::Succeeded),
                        PhaseStatus::Failed => ("failed", PhaseOutcome::Failed),
                        PhaseStatus::Cancelled => ("cancelled", PhaseOutcome::Cancelled),
                    };
                    let summary = result.summary.clone();
                    self.transact_phase_entry(
                        spec,
                        EventType::PhaseCompleted,
                        Some(serde_json::json!({ "status": state, "summary": summary })),
                        |_| Ok(()),
                    )?;
                    self.record_attempt_end(spec, state, EventType::AttemptCompleted, state)?;
                    return Ok(outcome);
                }
                Ok(WorkerEvent::Message(_)) => continue, // other messages: Day 6+
                Ok(WorkerEvent::HeartbeatTimeout) => {
                    handle.kill();
                    let reason = "heartbeat timeout".to_owned();
                    self.record_attempt_end(
                        spec,
                        "uncertain",
                        EventType::AttemptUncertain,
                        &reason,
                    )?;
                    return Ok(PhaseOutcome::Uncertain(reason));
                }
                Ok(WorkerEvent::Malformed(e)) => {
                    handle.kill();
                    let reason = format!("protocol violation: {e}");
                    self.record_attempt_end(
                        spec,
                        "uncertain",
                        EventType::AttemptUncertain,
                        &reason,
                    )?;
                    return Ok(PhaseOutcome::Uncertain(reason));
                }
                Ok(WorkerEvent::Eof) => {
                    let code = handle.wait_exit().ok().flatten();
                    let reason = format!("worker exited without phase.result (code {code:?})");
                    self.record_attempt_end(
                        spec,
                        "uncertain",
                        EventType::AttemptUncertain,
                        &reason,
                    )?;
                    return Ok(PhaseOutcome::Uncertain(reason));
                }
                Err(_) => continue, // recv timeout tick; deadline re-checked above
            }
        }
    }

    fn transact_phase_entry(
        &mut self,
        spec: &PhaseSpec,
        entry_type: EventType,
        payload: Option<serde_json::Value>,
        extra: impl FnOnce(&mut wepld_ledger::Tx) -> Result<(), wepld_ledger::LedgerError>,
    ) -> Result<(), RuntimeError> {
        let payload = payload
            .unwrap_or_else(|| serde_json::json!({ "phase": spec.phase, "task_id": spec.task_id }));
        self.store_mut().transact(|tx| {
            extra(tx)?;
            tx.append(&NewEntry {
                entry_type,
                schema_version: 1,
                aggregate_type: AggregateType::Attempt,
                aggregate_id: spec.attempt_id.clone(),
                actor_type: ActorType::Core,
                actor_id: "core".to_owned(),
                correlation_id: spec.mission_id.clone(),
                causation_ref: None,
                payload,
            })?;
            Ok(())
        })?;
        Ok(())
    }

    fn record_attempt_end(
        &mut self,
        spec: &PhaseSpec,
        state: &str,
        entry_type: EventType,
        reason: &str,
    ) -> Result<(), RuntimeError> {
        let payload = serde_json::json!({ "phase": spec.phase, "state": state, "reason": reason });
        self.store_mut().transact(|tx| {
            tx.set_attempt_state(&spec.attempt_id, state)?;
            tx.append(&NewEntry {
                entry_type,
                schema_version: 1,
                aggregate_type: AggregateType::Attempt,
                aggregate_id: spec.attempt_id.clone(),
                actor_type: ActorType::Core,
                actor_id: "core".to_owned(),
                correlation_id: spec.mission_id.clone(),
                causation_ref: None,
                payload,
            })?;
            Ok(())
        })?;
        Ok(())
    }
}

fn dev_envelope(attempt_id: &str) -> Envelope {
    Envelope {
        envelope_id: format!("env_{attempt_id}"),
        attempt_id: attempt_id.to_owned(),
        sandbox_tier: SandboxTier::Dev,
        fs: FsScope {
            write: vec![],
            read: vec![],
            deny: vec!["*".to_owned()],
        },
        network: NetworkPolicy {
            mode: "deny".to_owned(),
        },
        process: ProcessLimits {
            max_procs: 16,
            max_mem_mb: 512,
            cpu_share: 0.5,
            timeout_s: 60,
        },
        secrets: vec![],
        expires_at: String::new(),
    }
}
