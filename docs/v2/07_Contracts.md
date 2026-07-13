# v2-07 — Contracts

Nine normative contracts. All are JSON-schema-governed, semantically versioned (`schema_version` on every instance; additive minor changes only; breaking change ⇒ new major + coexistence window). Field lists here are the v0 freeze candidates for Phase A; illustrative values use ULIDs and truncated hashes.

---

## 1. Mission Contract

The only entry point for work. A structured brief — never free chat.

~~~json
{
  "schema_version": 1,
  "mission_id": "mis_01J8QZ3F...",
  "title": "Add rate limiting to the public API",
  "outcome": "Requests beyond a configurable per-key limit receive 429 with Retry-After; existing clients under the limit are unaffected.",
  "scope": {
    "repo": "/home/ana/projects/orbit-api",
    "base_branch": "main",
    "paths": ["src/api/**", "tests/api/**"],
    "forbidden_paths": ["src/billing/**"]
  },
  "acceptance_criteria": [
    { "id": "AC1", "text": "429 + Retry-After beyond limit", "verify": "test" },
    { "id": "AC2", "text": "limit configurable per key via existing config system", "verify": "test" },
    { "id": "AC3", "text": "no regression in existing API test suite", "verify": "gate:test" }
  ],
  "gates_required": ["build", "test", "review"],
  "autonomy_mode": "bounded_auto",
  "envelope_declared": { "network": "deny", "dependency_install": "ask", "secrets": [] },
  "budget": { "max_cost_usd": 5.0, "max_wall_minutes": 90, "max_interrupts": 3 },
  "classification": "internal",
  "owner": "principal_local"
}
~~~

Rules: scope changes create a revision and (in either mode) a decision; `budget.max_interrupts` funds the interrupt economics (v2-10); `verify` binds each criterion to the evidence type that can satisfy it — a criterion with no verify method is rejected at submission, which forces testable missions.

---

## 2. Worker Contract (WWP v0)

JSON-RPC 2.0. Key messages (envelope object per v2-05):

~~~json
// Core → Worker
{ "method": "attempt.start", "params": {
  "attempt_id": "att_01J8R0AB...",
  "task_id": "tsk_01J8QZ9C...",
  "phase": "build",
  "role_profile": { "name": "builder", "version": 3, "brain_profile": "default-build", "skills": [{"name":"rust-service-conventions","version":"1.2.0","hash":"sha256:9f2c…"}] },
  "context_pack_ref": { "artifact": "art_01J8R0AC...", "hash": "sha256:5d41…" },
  "envelope": { "...": "v2-05 schema" },
  "gates": ["build", "test"],
  "budget": { "max_brain_calls": 40, "max_wall_minutes": 30 },
  "idempotency_key": "att_01J8R0AB:1"
}}

// Worker → Core (selected)
{ "method": "heartbeat", "params": { "attempt_id": "att_…", "progress": "implementing token bucket in middleware" }}
{ "method": "brain.request", "params": { "attempt_id": "att_…", "intent": "implement_step", "pack_ref": {"artifact":"art_…"}, "output_schema_id": "builder_step.v1", "budget_hint": {"max_tokens": 8000} }}
{ "method": "artifact.put", "params": { "attempt_id": "att_…", "kind": "diff", "media_type": "text/x-diff", "content_b64": "…", "meta": {"files_changed": 4} }}
{ "method": "envelope.extend", "params": { "attempt_id": "att_…", "request": {"type": "dependency_install", "detail": "cargo add governor@0.6", "reason": "rate-limit algorithm", "reversible": true} }}
{ "method": "escalation.raise", "params": { "attempt_id": "att_…", "kind": "ambiguity", "question": "Apply limit per API key or per client IP for unauthenticated routes?", "options": [{"id":"key"},{"id":"ip"},{"id":"both"}], "context_refs": ["art_…"] }}
{ "method": "phase.result", "params": {
  "attempt_id": "att_…", "status": "succeeded",
  "outputs": [{"artifact":"art_diff…","kind":"diff"}],
  "evidence": [{"artifact":"art_locallog…","kind":"worklog"}],
  "summary": { "schema": "phase_summary.v1", "what": "…", "decisions_made": ["…"], "uncertainties": ["…"] },
  "next_hint": "validate"
}}
~~~

Hard rules: no verb addresses a human; no verb writes mission state; `phase.result.summary` is schema-enforced (it feeds T1/T4 compression, v2-04). Fleet seam: identical messages over an authenticated socket + explicit `lease.renew` replace stdio + parent-child liveness; when workers leave the machine, envelope grants gain asymmetric signatures (noted, deferred).

---

## 3. Brain Contract

Provider-neutral; only Core's gateway speaks provider APIs.

~~~json
// Request
{ "schema_version": 1, "invocation_id": "brn_01J8R0AE...",
  "intent": "review",                       // plan | implement_step | review | classify | summarize | diagnose
  "profile": "independent-review",          // resolved by Core, never a vendor/model literal from the worker
  "pack": { "artifact": "art_…", "hash": "sha256:…" },
  "output_schema_id": "review_findings.v1",
  "constraints": { "classification_max": "internal", "deadline_ms": 120000 },
  "budget": { "max_tokens": 16000, "max_cost_usd": 0.6, "fallback": ["local-default"] },
  "trace": { "mission_id": "mis_…", "attempt_id": "att_…", "correlation_id": "mis_…" } }

// Result
{ "schema_version": 1, "invocation_id": "brn_…", "status": "ok",   // ok | schema_invalid | refused | provider_error | budget_denied
  "output": { "…": "validates against review_findings.v1" },
  "proposed_actions": [],                   // normalized from native tool-calls; suggestions only, never effects
  "usage": { "provider": "anthropic", "model": "claude-…", "tokens_in": 9412, "tokens_out": 1877, "cost_usd": 0.31, "latency_ms": 8400 },
  "uncertainty": { "self_reported": "…", "missing_context": ["…"] } }
~~~

Rules: fallback only same-or-lower data classification, never a silent budget/classification upgrade (v1 rule preserved verbatim); `schema_invalid` after one reformat retry escalates per the failure taxonomy.

---

## 4. Messenger Contract

Everything a human reads or writes passes through these two shapes — across every surface (v2-10).

~~~json
// Outbound (report | decision_delivery | alert | completion_proposal)
{ "schema_version": 1, "message_id": "msg_01J8R0AF...", "kind": "report",
  "mission_id": "mis_…", "audience": "principal_local", "channel": "studio",
  "body_md": "Build phase completed. 4 files changed; targeted tests added.",
  "claims": [
    { "text": "targeted tests pass (12/12)", "evidence": { "ledger_seq": 3121, "artifact": "art_testlog…" }, "verified": true },
    { "text": "approach follows existing middleware conventions", "evidence": null, "verified": false }   // renders as unverified narrative
  ],
  "provenance": { "derived_from_ledger_seqs": [3101, 3121], "generated_by": "messenger.v1" } }

// Inbound — normalized user intent; always becomes a Command, never direct mutation
{ "schema_version": 1, "intent_id": "int_01J8R0AG...", "channel": "studio",
  "principal": "principal_local", "verb": "resolve_decision",
  "payload": { "decision_id": "dec_…", "option": "key", "rationale": "unauthenticated routes are out of scope" } }
~~~

The `claims[]` array is the anti-injection mechanism at the human boundary: any assertion with `verified:true` must carry a ledger/artifact reference the UI resolves independently; unverifiable prose is visually demoted (v2-10).

---

## 5. Event (Ledger Entry) Contract

Schema in v2-06. **Closed v0 vocabulary** (adding a type is a contract change):

`MissionCreated · MissionRevised · PlanProposed · PlanApproved · PlanRejected · TaskStarted · AttemptSpawned · PhaseStarted · BrainInvoked · ArtifactRecorded · EnvelopeExtensionRequested · EnvelopeExtensionResolved · EscalationRaised · DecisionRequested · DecisionResolved · GateEvaluated · PhaseCompleted · AttemptCompleted · AttemptUncertain · RecoverySnapshotRecorded · RecoveryPerformed · TaskCompleted · MissionWaiting · CompletionProposed · MissionAccepted · MissionReturned · MissionCancelled · MissionFailed · SandboxTierDetected · RedactionApplied · MessageSent · BudgetThresholdCrossed`

**Revision 2 (Chronicle, ADR-0011):** adds `WorkspaceSnapshotRecorded · MissionForked · DecisionRevised · MissionSuperseded` (Chronicle MVP) and `InsightRecorded · AnnotationRecorded · ReplayExported` (Chronicle V1). See [v2-17](17_Chronicle_Contracts_and_API.md).

Names are past-tense facts; a `…Requested` entry never implies the effect occurred (v1 rule preserved).

---

## 6. Decision Contract

~~~json
{ "schema_version": 1, "decision_id": "dec_01J8R0AH...", "mission_id": "mis_…",
  "class": "blocking",                       // blocking | advisory | completion  (v2-10 semantics)
  "question": "Apply rate limit per API key or per client IP for unauthenticated routes?",
  "options": [
    { "id": "key",  "consequence": "unauthenticated routes unlimited; simplest; matches AC1 wording" },
    { "id": "ip",   "consequence": "covers unauthenticated routes; adds proxy/X-Forwarded-For handling risk" },
    { "id": "both", "consequence": "fullest coverage; larger diff; touches config schema" },
    { "id": "defer", "consequence": "build proceeds for authenticated routes only; question returns at review" }
  ],
  "recommendation": { "option": "key", "rationale": "AC1 says 'per-key'; IP limiting is scope expansion", "confidence": "medium" },
  "evidence": [{ "artifact": "art_planexcerpt…" }],
  "why_now": { "blocked": ["tsk_…"], "deadline": null },
  "authority": "principal_local",
  "interrupt_budget": { "consumed": 1, "remaining": 2 },
  "resolution": null }
~~~

Resolution is a Command (`resolve_decision`) → Core validates authority → `DecisionResolved` ledger fact → dependent tasks unblock. Identical semantics from any surface or (future) channel.

---

## 7. Artifact Contract

~~~json
{ "schema_version": 1, "artifact_id": "art_01J8R0AJ...",
  "hash": "sha256:5d41402a…", "kind": "diff",   // diff | test_log | build_log | plan | review_findings | context_pack | worklog | snapshot | report | skill_doc
  "media_type": "text/x-diff", "size_bytes": 18234,
  "producer": { "type": "worker", "attempt_id": "att_…" },
  "classification": "internal", "retention": "standard",
  "provenance": { "base_commit": "9ac31f2", "tool": "hermes/0.1.0" } }
~~~

Bodies are content-addressed, write-once, hash-verified on read; tombstoning removes bodies, never hashes (v2-06).

---

## 8. Knowledge Contract

MVP: three typed records, human- or mission-produced, zero extraction pipeline.

~~~json
{ "schema_version": 1, "record_id": "kno_01J8R0AK...", "type": "decision",   // decision | lesson | finding
  "title": "Rate limiting is enforced per API key, not per IP",
  "body_md": "Chosen during mis_01J8QZ3F… AC1 wording governs. Revisit if unauthenticated abuse appears.",
  "tags": ["api", "rate-limiting"], "paths": ["src/api/middleware/**"],
  "sources": [{ "decision_id": "dec_…" }, { "artifact": "art_…" }],
  "status": "active", "supersedes": null,
  "classification": "internal", "review_after": "2027-01-01" }
~~~

Rules preserved from v1 at MVP cost: no canonical record without a source; correction supersedes, never overwrites; retrieval (Context Assembly T3) filters by status/classification and labels freshness. Claims graphs, semantic indexes, and cross-project memory are V2+ layers over these same records.

---

## 9. Skill Contract

MVP package = a directory: `skill.json` + content files, hash-pinned.

~~~json
{ "schema_version": 1, "name": "rust-service-conventions", "version": "1.2.0",
  "publisher": "local", "hash": "sha256:9f2c…",
  "compatible_roles": ["builder", "reviewer"],
  "content": [
    { "file": "conventions.md", "inject": "T0" },
    { "file": "review-checklist.md", "inject": "T0", "roles": ["reviewer"] }
  ],
  "checks": [{ "name": "clippy-pedantic", "command": "cargo clippy -- -D warnings", "gate": "build" }],
  "requested_capabilities": [],            // must be empty in MVP: skills inform, they do not act
  "license": "internal" }
~~~

Resolution: role profiles pin `name@version`; Context Assembly injects content and records hashes in the attempt — full reproducibility of "which expertise informed this change." Signing, registries, revocation, and the evolution workflow return in V2 over this same descriptor (fields are added, none change meaning) — v1 doc 09's lifecycle intact, deferred, not deleted.
