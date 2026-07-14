//! Phase engine v0 (v2-02 §5): spawn a WWP worker for one phase, stream its
//! events, and record every ending honestly. A worker that vanishes is never
//! assumed to have failed cleanly — it is UNCERTAIN until probed (v2-02 §7).

use crate::{Core, RuntimeError};
use std::time::{Duration, Instant};
use wepld_contracts::brain::{BrainResult, BrainStatus, Usage};
use wepld_contracts::envelope::{Envelope, FsScope, NetworkPolicy, ProcessLimits, SandboxTier};
use wepld_contracts::ledger::{ActorType, AggregateType};
use wepld_contracts::vocabulary::EventType;
use wepld_contracts::wwp::{
    ArtifactRef, AttemptStart, BrainRequest, PhaseBudget, PhaseStatus, RoleProfile, WwpMessage,
};
use wepld_ledger::{BrainInvocationRow, NewEntry};
use wepld_wwp::{spawn_worker, WorkerEvent};

pub struct PhaseSpec {
    pub mission_id: String,
    pub task_id: String,
    pub attempt_id: String,
    pub phase: String,
    /// The repository this phase operates on — authorized at the worker-spawn
    /// boundary (Blocker 5 defense in depth) before any worker starts.
    pub repo: String,
    /// Worker program + args (the Runtime decides which WWP runtime runs).
    pub worker_cmd: Vec<String>,
    /// Context pack v0 (brief + task); captured to the CAS and referenced by
    /// hash — never duplicated (v2-04 capture discipline).
    pub pack: serde_json::Value,
    /// Brain profile the phase may use; reasoning is optional (IADR-0007 §1).
    pub brain_profile: String,
    /// The worktree the worker may write (build phases). Passed to the worker
    /// as the envelope's single writable path; empty for read-only phases.
    pub workspace_path: Option<String>,
    /// Hard cap on brain calls per attempt. Enforced by the Core: a worker
    /// that exceeds it is killed and the attempt fails (bounds request spam).
    pub max_brain_calls: u32,
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

/// The result of running a phase: its outcome plus the worker's reported
/// summary (empty when the phase did not complete).
pub struct PhaseRun {
    pub outcome: PhaseOutcome,
    pub summary: serde_json::Value,
}

impl Core {
    /// Backwards-compatible wrapper returning only the outcome.
    pub fn run_phase_stub(&mut self, spec: &PhaseSpec) -> Result<PhaseOutcome, RuntimeError> {
        Ok(self.run_phase(spec)?.outcome)
    }

    /// Run one phase end-to-end: capture pack, spawn worker, mediate brain
    /// requests, record every ending honestly, return outcome + summary.
    pub fn run_phase(&mut self, spec: &PhaseSpec) -> Result<PhaseRun, RuntimeError> {
        // Defense in depth (Blocker 5): the worker-spawn boundary itself
        // authorizes the repository under the DEV tier. A denied repository
        // spawns NO worker and records NO attempt — we return before any effect.
        if let Err(reason) =
            self.dev_tier_gate(&spec.repo, wepld_contracts::mission::AutonomyMode::Manual)
        {
            return Ok(PhaseRun {
                outcome: PhaseOutcome::Failed,
                summary: serde_json::json!({ "denied": reason }),
            });
        }

        // Capture the context pack: store once, reference by hash forever.
        let pack_bytes = serde_json::to_vec(&spec.pack)?;
        let stored = self.cas().put(&pack_bytes)?;
        let pack_ref = ArtifactRef {
            artifact: format!("art_{}", &stored.hash[..16]),
            hash: stored.hash.clone(),
        };

        let envelope = dev_envelope(&spec.attempt_id, spec.workspace_path.as_deref());
        let start = AttemptStart {
            attempt_id: spec.attempt_id.clone(),
            task_id: spec.task_id.clone(),
            phase: spec.phase.clone(),
            role_profile: RoleProfile {
                name: "stub".to_owned(),
                version: 0,
                brain_profile: spec.brain_profile.clone(),
                skills: vec![],
            },
            context_pack_ref: pack_ref,
            envelope: envelope.clone(),
            gates: vec![],
            budget: PhaseBudget {
                max_brain_calls: spec.max_brain_calls,
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
                return Ok(PhaseRun {
                    outcome: PhaseOutcome::Failed,
                    summary: serde_json::json!({}),
                });
            }
        };
        self.transact_phase_entry(spec, EventType::PhaseStarted, None, |_| Ok(()))?;

        let deadline = Instant::now() + Duration::from_millis(spec.deadline_ms);
        let mut brain_calls: u32 = 0;
        loop {
            let remaining = deadline.saturating_duration_since(Instant::now());
            if remaining.is_zero() {
                handle.kill();
                let reason = "phase deadline exceeded".to_owned();
                self.record_attempt_end(spec, "uncertain", EventType::AttemptUncertain, &reason)?;
                return Ok(PhaseRun {
                    outcome: PhaseOutcome::Uncertain(reason),
                    summary: serde_json::json!({}),
                });
            }
            match handle.events.recv_timeout(remaining) {
                Ok(WorkerEvent::Message(frame)) => match frame.msg {
                    WwpMessage::Heartbeat(_) => continue,
                    WwpMessage::PhaseResult(result) => {
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
                        return Ok(PhaseRun { outcome, summary });
                    }
                    WwpMessage::BrainRequest(req) => {
                        let Some(rpc_id) = frame.id else {
                            handle.kill();
                            let reason = "brain.request without an id".to_owned();
                            self.record_attempt_end(
                                spec,
                                "uncertain",
                                EventType::AttemptUncertain,
                                &reason,
                            )?;
                            return Ok(PhaseRun {
                                outcome: PhaseOutcome::Uncertain(reason),
                                summary: serde_json::json!({}),
                            });
                        };
                        brain_calls += 1;
                        if brain_calls > spec.max_brain_calls {
                            handle.kill();
                            let reason = format!(
                                "brain call budget exceeded ({} > {})",
                                brain_calls, spec.max_brain_calls
                            );
                            self.record_attempt_end(
                                spec,
                                "failed",
                                EventType::AttemptCompleted,
                                &reason,
                            )?;
                            return Ok(PhaseRun {
                                outcome: PhaseOutcome::Failed,
                                summary: serde_json::json!({}),
                            });
                        }
                        let result = self.handle_brain_request(spec, &req, rpc_id)?;
                        // A failed respond means the worker is going away;
                        // the Eof/timeout paths will classify it.
                        let _ = handle.respond(rpc_id, serde_json::to_value(&result)?);
                        continue;
                    }
                    _ => continue, // remaining messages: Days 7–8
                },
                Ok(WorkerEvent::HeartbeatTimeout) => {
                    handle.kill();
                    let reason = "heartbeat timeout".to_owned();
                    self.record_attempt_end(
                        spec,
                        "uncertain",
                        EventType::AttemptUncertain,
                        &reason,
                    )?;
                    return Ok(PhaseRun {
                        outcome: PhaseOutcome::Uncertain(reason),
                        summary: serde_json::json!({}),
                    });
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
                    return Ok(PhaseRun {
                        outcome: PhaseOutcome::Uncertain(reason),
                        summary: serde_json::json!({}),
                    });
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
                    return Ok(PhaseRun {
                        outcome: PhaseOutcome::Uncertain(reason),
                        summary: serde_json::json!({}),
                    });
                }
                Err(_) => continue, // recv timeout tick; deadline re-checked above
            }
        }
    }

    /// Gateway round-trip + invocation recording (v2-07 §3): pack fetched by
    /// hash, response body stored to the CAS, row + `BrainInvoked` fact in
    /// one transaction. The gateway stays pure; the Runtime owns persistence.
    fn handle_brain_request(
        &mut self,
        spec: &PhaseSpec,
        req: &BrainRequest,
        rpc_id: u64,
    ) -> Result<BrainResult, RuntimeError> {
        let invocation_id = format!("brn_{}_{rpc_id}", spec.attempt_id);

        let result = match self.cas().get(&req.pack_ref.hash) {
            Ok(bytes) => {
                let pack: serde_json::Value = serde_json::from_slice(&bytes)?;
                self.gateway().invoke(
                    &invocation_id,
                    &spec.brain_profile,
                    &req.intent,
                    &pack,
                    &req.pack_ref.hash,
                    &req.output_schema_id,
                )?
            }
            Err(e) => BrainResult {
                schema_version: 1,
                invocation_id: invocation_id.clone(),
                status: BrainStatus::ProviderError,
                output: serde_json::json!({}),
                usage: Usage {
                    provider: "none".to_owned(),
                    model: "none".to_owned(),
                    tokens_in: 0,
                    tokens_out: 0,
                    cost_usd: 0.0,
                    latency_ms: 0,
                },
                reason: Some(format!("unknown context pack: {e}")),
            },
        };

        let response_ref = self.cas().put(&serde_json::to_vec(&result.output)?)?;
        let status = serde_json::to_value(result.status)?
            .as_str()
            .unwrap_or("unknown")
            .to_owned();
        let row = BrainInvocationRow {
            invocation_id: result.invocation_id.clone(),
            attempt_id: spec.attempt_id.clone(),
            profile: spec.brain_profile.clone(),
            provider: result.usage.provider.clone(),
            model: result.usage.model.clone(),
            intent: req.intent.clone(),
            pack_hash: req.pack_ref.hash.clone(),
            response_artifact: Some(response_ref.hash.clone()),
            status: status.clone(),
            tokens_in: result.usage.tokens_in,
            tokens_out: result.usage.tokens_out,
            cost_usd: result.usage.cost_usd,
            latency_ms: result.usage.latency_ms,
        };
        let payload = serde_json::json!({
            "invocation_id": row.invocation_id,
            "profile": row.profile,
            "provider": row.provider,
            "model": row.model,
            "intent": row.intent,
            "pack_hash": row.pack_hash,
            "response_artifact": row.response_artifact,
            "status": status,
            "cost_usd": row.cost_usd,
            "latency_ms": row.latency_ms,
        });
        let attempt_id = spec.attempt_id.clone();
        let mission_id = spec.mission_id.clone();
        self.store_mut().transact(|tx| {
            tx.record_brain_invocation(&row)?;
            tx.append(&NewEntry {
                entry_type: EventType::BrainInvoked,
                schema_version: 1,
                aggregate_type: AggregateType::Attempt,
                aggregate_id: attempt_id,
                actor_type: ActorType::BrainAdapter,
                actor_id: "gateway".to_owned(),
                correlation_id: mission_id,
                causation_ref: None,
                payload,
            })?;
            Ok(())
        })?;
        Ok(result)
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

fn dev_envelope(attempt_id: &str, workspace_path: Option<&str>) -> Envelope {
    let write = match workspace_path {
        Some(p) => vec![p.to_owned()],
        None => vec![],
    };
    Envelope {
        envelope_id: format!("env_{attempt_id}"),
        attempt_id: attempt_id.to_owned(),
        sandbox_tier: SandboxTier::Dev,
        fs: FsScope {
            write,
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
