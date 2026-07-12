# 02 — Product Principles

## Purpose

These principles are decision rules. When a feature, provider, or shortcut conflicts with one, the principle wins unless an ADR records why the principle itself must change.

| Principle | Operational rule |
| --- | --- |
| Local whenever possible | A core mission must not require a hosted WePLD control plane; external brain and notification calls are explicit dependencies. |
| Brains are replaceable | Domain code sees a stable reasoning contract, never a vendor SDK. |
| Workers are replaceable | A role is a policy and capability profile, not a hard-coded model or process. |
| Skills are reusable | Repeatable expertise is a versioned package with validation, provenance, and compatibility metadata. |
| Knowledge accumulates with evidence | Every retained claim links to sources, owner, freshness, classification, and retention policy. |
| One control plane | Only orchestration changes mission, run, and task state; direct worker-to-worker and worker-to-user messaging is prohibited. |
| One human voice | Only Messenger accepts or sends human-facing communications, via policy-governed commands. |
| Evidence before assertion | Completion, quality, and risk claims require linked artifacts or check results. |
| Least privilege by default | Tools, paths, network, secrets, and integrations are capability grants, not ambient authority. |
| Observable by construction | Each effect has actor, intent, policy outcome, trace, timestamp, and result. |
| Reversible by design | Prefer worktrees, preview, versioning, rollback, and staged release over irreversible change. |
| Offline-first where feasible | Queued, degraded, and read-only behavior are designed explicitly when connectivity or a provider is absent. |
| Extensibility without bypass | Plugins and integrations use versioned ports and capability mediation; no extension may bypass policy or audit. |

## Separation of responsibilities

| Component | May do | Must not do |
| --- | --- | --- |
| Brain | reason, plan, classify, recommend, produce structured proposals | edit files, execute tools, contact a user |
| Worker | perform a leased task through approved tools and skills | mutate mission state, bypass policy, talk to a user |
| Orchestrator | schedule, validate transitions, mediate handoffs, recover runs | contain provider-specific reasoning or UI behavior |
| Policy engine | evaluate action/risk against rules and issue decisions | perform the action itself |
| Tool executor | enact an approved, scoped tool action and emit evidence | invent scope or decide policy |
| Messenger | report and collect intent/decisions | grant privileges or directly alter project state |

## Product non-goals

- Simulating a company by displaying autonomous chat transcripts.
- Treating raw model output as a trusted command, test result, or decision.
- Sending proprietary project context to an external provider without a recorded policy decision.
- Making every capability available to every worker for convenience.
- “Forever” retention that overrides deletion, customer, or legal obligations.
- Claiming full autonomy when protected deployment, secrets, destructive actions, external transfer, or budget expansion remain unsafe.

## Decision hierarchy

1. Safety, legal, compliance, and non-bypassable enterprise policy.
2. Explicit mission scope, budget, and autonomy mode.
3. Quality gates and evidence requirements.
4. Product principles and documented architecture contracts.
5. Worker judgment and optimization.

If evidence conflicts, the system preserves both claims, marks the conflict, and escalates rather than selecting the most confident model response. If a policy decision cannot be evaluated, the safe action is to pause the affected action—not the unrelated organization.

## Autonomy invariant

Every mode permits autonomous reading, planning, and internal analysis inside policy. Every mode retains hard gates for secret access, protected branches, external data transfer, destructive operations, scope expansion, and production deployment. “Full Autonomous” means autonomous execution within a declared envelope; it never means unbounded authority.

## Adoption test

A proposal is acceptable only if it can answer: What mission value does it create? Which bounded context owns it? What capability and data classification does it need? What events and evidence will it emit? How does it fail safely? Can it be removed or replaced without altering mission semantics?

See also: [14_Security_Architecture.md](14_Security_Architecture.md), [15_Plugin_System.md](15_Plugin_System.md), and [25_Development_Guidelines.md](25_Development_Guidelines.md).

