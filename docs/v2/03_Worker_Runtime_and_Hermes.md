# v2-03 — Worker Runtime, WWP, and Hermes

## The decision (ADR-0002, ADR-0005)

**One worker runtime with role switching, on a fleet-ready protocol.** Considered options:

| Option | Verdict | Reasoning |
| --- | --- | --- |
| Separate runtimes per role | Rejected for MVP | Requires registry/scheduler/lease machinery before first mission; empirical record shows rigid multi-agent pipelines (MetaGPT/ChatDev style) underperform strong single-loop agents (CodeAct/OpenHands, Claude-Code-style) on real-repo benchmarks; every mediated handoff loses context, and context quality dominates outcome quality |
| One runtime, in-process role switching (one long process per task) | Rejected | Cannot change sandbox envelope mid-process without trusting the worker; reviewer independence would rely on the worker's own discipline |
| **One runtime, one process per phase (chosen)** | Accepted | OS-enforced envelope per role; Core-enforced context isolation per role; brain-profile isolation per role; zero fleet machinery |
| Hybrid (fleet-ready protocol) | Accepted as posture | WWP keeps lease/heartbeat/cancel semantics so separate/remote runtimes are a transport change, not a redesign |

Role separation is real, but it is enforced by **three isolations** rather than by process identity:

1. **Envelope isolation** — Builder: worktree read/write + execute. Validator: read + execute. Reviewer: read-only, plus diff/test artifacts. Enforced by the sandbox (v2-05), not by convention.
2. **Context isolation** — each phase's context pack is assembled by Core (v2-04). The Review phase receives the mission brief, acceptance criteria, the diff, and gate evidence — **not** the Builder's reasoning transcript. Self-review bias is removed by construction; the reviewer literally cannot see the builder's rationalizations.
3. **Brain isolation** — the Review role profile may pin a different brain profile (second provider) for genuinely independent judgment; disagreement becomes a finding, per v1 doc 06's instinct, now mechanized.

**Phase A experiment E1** (v2-09) tests this design against a split-worker arm with a pre-registered decision rule, so the topology question is settled by evidence, not taste.

## WWP — the WePLD Worker Protocol (v0)

Transport: JSON-RPC 2.0 over stdio (MVP); over an authenticated socket for future remote runtimes — messages identical. Full schemas in [v2-07 §Worker Contract](07_Contracts.md).

| Direction | Message | Purpose |
| --- | --- | --- |
| Core → Worker | `attempt.start` | role profile, phase, `context_pack_ref`, envelope, gates, budgets, idempotency key |
| Core → Worker | `attempt.cancel` | cooperative cancel; Core kills the process group after grace period regardless |
| Worker → Core | `heartbeat` | liveness + progress note (every ≤15 s; two missed → lost-worker recovery) |
| Worker → Core | `context.get` | fetch pack sections by reference (large bodies pulled lazily) |
| Worker → Core | `brain.request` | provider-neutral reasoning request; **routed through Core's gateway — workers never hold provider credentials or choose vendors** |
| Worker → Core | `artifact.put` | store content, get `{artifact_id, hash}` |
| Worker → Core | `envelope.extend` | request a hard-gate crossing (network, dependency, path, secret) — Core decides or escalates |
| Worker → Core | `escalation.raise` | surface ambiguity/blocking question → becomes a decision packet (subject to interrupt budget, v2-10) |
| Worker → Core | `phase.result` | terminal: status, output artifacts, evidence refs, uncertainties, next-phase hint |

Design rules that make the protocol a boundary rather than a suggestion:

- **No message addresses a human.** There is no "message user" verb; principle 4 is enforced by the protocol's vocabulary, not by policy review.
- **No message mutates mission state.** Workers report; only Core's transition function changes state.
- **Everything the worker learns arrives via `context.get`; everything it produces leaves via `artifact.put`/`phase.result`.** No shared filesystem side channels outside the envelope.
- Conformance = passing the WWP fixture suite: lease honor, heartbeat cadence, cancel compliance, envelope-denial behavior, artifact hashing, schema validity. Required equally of Hermes and any future third-party runtime.

## Hermes — the reference runtime

Hermes is WePLD's first-party WWP implementation (ADR-0005): a small program that runs the phase loop — read pack → reason (`brain.request`) → act inside the envelope (run commands, edit files in the worktree) → validate its own work → emit artifacts and `phase.result`. It contains **no provider SDKs, no credentials, no user-facing text generation** (its narrative outputs are artifacts that Messenger may quote with provenance). It has no privileged API: anything Hermes can do, a conformant third-party runtime can do. "Hermes-compatible" as a phrase is retired; the compatibility target is WWP.

## Brain routing (gateway mechanics)

The gateway is a Core module, keeping principle 2 (replaceable brains) and the credential boundary in one place:

1. Worker sends `brain.request{intent, pack_ref, output_schema_id, budget_hint}`.
2. Gateway resolves the phase's **brain profile** (named config: adapter, model, max context, cost ceiling, classification limits, fallback list — versioned data, not code).
3. Projected cost check against mission budget (v2-02 §8). Over → `BUDGET` failure → escalation path.
4. Adapter translates the neutral request to the provider API, enforcing structured output (JSON-schema constrained where supported; validate-and-single-retry otherwise).
5. Response validated against `output_schema_id`. Invalid after one reformat retry → `SCHEMA` failure per taxonomy.
6. Invocation recorded: profile, provider, model, pack hash, response artifact, tokens, cost, latency. This record + the pack is the replay substrate (v2-04).

MVP adapters: one hosted family (Anthropic-compatible Messages API) and one local family (OpenAI-compatible endpoint — covers Ollama, vLLM, LM Studio). Two families prove the port; adding a third is an adapter, not an architecture event. Fallback in MVP is deliberately dumb: same-classification-or-lower, next profile in list, once. Router sophistication is earned in V2 with evaluation data.

## Skills at runtime

A skill (v2-07 §Skill Contract) is a versioned, hash-pinned package of instructions/templates/checks. Role profiles declare skill requirements; Context Assembly resolves pinned versions into the pack's T0 tier and records exactly which skill hashes informed the attempt — v1's reproducibility promise at MVP cost. Workers cannot load skills themselves; skills arrive only via packs.
