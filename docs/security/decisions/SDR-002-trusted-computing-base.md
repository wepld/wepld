# SDR-002 — Trusted Computing Base

- **Status:** Adopted (founder decision, 2026-07-19); the
  separate-process topology clause is **provisional pending S0.5A**
- **Prerequisites:** SDR-001

## Context

The security objective depends on a small set of components whose
correctness cannot be delegated: authorization, evidence, secret
mediation, verification. Everything else must be survivable when
compromised. Without an explicit TCB boundary, trust spreads by
convenience.

## Decision

Adopt `../TRUSTED_COMPUTING_BASE.md`: the TCB comprises the capability
broker, policy engine, identity/session verification, canonical
serialization, cryptographic verification, secure-update verifier (once
an updater exists), ledger writer, secret mediator, workspace
confinement layer, and IPC authorization layer — each individually
justified. The UI, agents, model output, plugins, workspace content,
provider responses, and integrations are untrusted; workers and the
future CoWork server are less trusted. TCB code lives in dedicated
crates linked only by the trusted-core process, with the highest review
tier and no unmeasured size claims. The trusted core provisionally runs
as a process separate from the WebView shell, subject to S0.5A
prototype validation.

## Rationale

A small, named, auditable TCB is the only structure under which "the UI
holds zero authority" and "agents are untrusted" are enforceable facts
rather than aspirations.

## Consequences

Compromise of any non-TCB component is a contained event by design;
every TCB change carries the highest review class; the process-topology
clause may be revised by prototype evidence without reopening the TCB
principle itself.

## Legal-review boundary

None.

## Supersession rules

Component list changes require a successor or amending SDR linking
here; the S0.5A outcome is recorded as a dated amendment, not a silent
edit.
