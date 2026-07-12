# 25 — Development Guidelines

## Purpose

These guidelines apply after M0 authorizes implementation. They preserve WePLD’s trust boundaries as the codebase grows. A fast feature that bypasses policy, loses provenance, or couples an adapter to the domain is a defect, not productive speed.

## Contract-first delivery

1. State the mission/backlog acceptance criteria and affected bounded contexts.
2. Decide whether an ADR is required before implementation.
3. Define or revise command, event, data, and policy contracts before adapter/UI behavior.
4. Write acceptance/evaluation fixtures for normal, denied, degraded, and recovery paths.
5. Implement domain behavior and adapters behind ports.
6. Demonstrate evidence through tests, review, security checks, and documentation updates.

## Design rules

- Keep domain invariants deterministic, explicit, and independently testable.
- Make effects requestable, policy-evaluated, idempotent where possible, observable, and recoverable.
- Prefer typed schemas and bounded artifacts over free-form cross-component prose.
- Store large/sensitive content as classified artifacts; place references/hashes in events.
- Avoid shared mutable state across contexts and direct database access outside an owner.
- Model unavailability, uncertainty, and partial completion explicitly rather than catching and hiding errors.
- Do not add a model/provider/tool shortcut that changes the brain-worker-policy separation.

## Security requirements

- Never log, commit, render, or transmit secrets outside a brokered policy path.
- Use capability tokens and least privilege for every tool, worker, plugin, and integration action.
- Treat model output, channel input, repository content, plugin content, and external tool output as untrusted.
- Add/alter a provider, package, network destination, sandbox control, or data flow only with threat-model review and policy documentation.
- Follow secure dependency, signing, SBOM, vulnerability, and license practices defined by Security.

## Code review requirements

Reviewers verify correctness, boundary compliance, failure/recovery behavior, race/idempotency concerns, performance/resource impact, security/data classification, observability, user disclosure, documentation, and tests. A reviewer should be able to trace a material effect from command through policy, execution, evidence, and event without relying on an author’s explanation.

Security, policy, data model, event schema, API contract, package permission, and release changes require designated owner review. Independent reviewer and security worker outputs are evidence, not automatic approval; designated human/code-owner rules remain authoritative.

## Documentation and knowledge

Every material feature updates its architecture map, contract docs, runbook, relevant ADR, and test/evaluation fixture. Lessons or recurring defects become knowledge candidates with evidence and applicability limits. Documentation is part of the completion gate, not a post-release chore.

## Change management

Use small, reversible changes. Branch/worktree changes must be attributable to a mission/task/attempt. Commit messages and merge proposals link to the relevant decision, check, and artifact. Database/event migrations are forward-compatible where possible, tested under interruption, and accompanied by rollback/repair instructions. Feature flags are policy-governed and have an owner, expiry/review date, and removal plan.

## Quality bar

No completion claim is accepted without the task’s required build/static/test/review/security/accessibility/performance/documentation evidence. Test changes must explain what behavior they protect. Flaky tests, ignored findings, opaque retries, and unexplained coverage/performance regressions are tracked defects, not background noise.

## AI-assisted development

Workers and assistants follow the same engineering process as humans: scoped task, approved capabilities, isolated workspace, citations, checks, review, and provenance. They do not directly modify protected branches, publish packages, approve their own exceptions, or claim quality without machine-verifiable evidence.

See also: [02_Product_Principles.md](02_Product_Principles.md), [14_Security_Architecture.md](14_Security_Architecture.md), [17_Event_System.md](17_Event_System.md), [26_Testing_Strategy.md](26_Testing_Strategy.md), and [28_Release_Strategy.md](28_Release_Strategy.md).

