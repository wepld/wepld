# 15 — Plugin, Skill, and Extension System

## Goal and authority boundary

Everything that extends WePLD—brain and builder adapters, worker hosts, Hermes skills, hook handlers, LSP/retrieval adapters, UI extensions, MCP servers, integrations, and toolchains—uses an inspectable, permissioned, versioned, removable, and revocable package lifecycle. Extension is never a path around Core authority, the Effect Firewall, evidence, or observability.

Core records package policy, approval, activation scope, capability issuance, revocation, budget, and any workflow transition. The Registry verifies and stages packages; Hermes may route an already approved skill or adapter; a package cannot activate itself, broaden its permissions, approve its output, or write durable governance state.

## Extension types

| Type | Adds | Runtime boundary | Principal risk |
| --- | --- | --- | --- |
| Brain/builder adapter | provider-neutral reasoning or bounded builder execution | Brain Gateway / approved builder host | data egress, credentials, model drift |
| Worker adapter | executor protocol compatibility | Worker Registry / Worker Host | code execution, outcome spoofing |
| Hermes skill | executable engineering procedure and evidence contract | Skill Runtime | instruction/tool escalation, invalid method |
| Hook handler | typed lifecycle observation/validation/blocking behavior | Hook Bus host | reentrancy, hidden effect, deadlock |
| LSP/retrieval adapter | symbols, diagnostics, code/context retrieval | intelligence broker | poisoned, omitted, or cross-scope context |
| Toolchain | compiler, test, formatter, scanner, migration tool | Tool Executor | arbitrary process and supply chain |
| Integration | external service or messaging channel | Integration Gateway | external data, identity, replay |
| MCP server | typed tools/resources/context | mediated MCP adapter | tool/data access, prompt injection |
| UI extension/theme | presentation, panels, commands | Studio extension host | UI confusion, data exposure, false authority |

## Universal package descriptor

Every package declares identity, publisher, semantic version, integrity hash, signature/provenance, compatibility range, dependency graph, requested capabilities, supported platforms, data handling and destinations, license, configuration schema, migration/rollback instructions, health checks, evaluation evidence, release channel, and revocation information. Type-specific metadata augments rather than replaces this descriptor.

Exact package set, configuration fingerprint, model/profile identity, skill version, hook set, toolchain, and adapter versions are recorded for every relevant task attempt and evaluation. Display names are never identity.

## Hermes skill contract

A skill is an executable, testable procedure—not only prompt text. Its signed manifest declares:

- identity, version, applicability rules, compatibility, and trust tier;
- context requirements and accepted trust/classification;
- required tools, allowed capabilities, writable and forbidden scopes;
- deterministic procedure stages and bounded model-dependent steps;
- verification procedure, failure modes, retry and escalation conditions;
- typed input/output schemas and evidence contract;
- budget class, telemetry, privacy, and retention behavior;
- evaluation fixtures and measured compatibility with supported profiles.

Initial families cover repository exploration, architecture analysis, Rust, TypeScript and Python engineering, debugging, test planning/generation, security review, dependency analysis, API/schema design, database migration, performance, documentation, Git forensics, and recovery investigation.

The Skill Router selects a skill, brain/builder profile, subagent role, tools, context, budget, and verification level from the approved Task Packet, policy, capabilities, environment, and measured evidence. Its decision and alternatives are recorded. Routing is replaceable by policy and cannot weaken the Task Packet or higher contract.

## Installation and activation lifecycle

1. Discover through a local or organization-approved registry.
2. Resolve versions and dependencies without mutating the active environment.
3. Verify identity, signature, hash, publisher policy, license, advisory status, data handling, and compatibility.
4. Evaluate declared fixtures, schemas, failure paths, evidence production, and capability behavior in isolation.
5. Present exact permissions, external destinations, hook class, budget impact, and trust evidence for approval where required.
6. Install into an isolated versioned store; do not activate on install.
7. Core atomically activates an approved version for named organization/project/mission scopes and records the event.
8. Monitor health, profile drift, vulnerabilities, and evidence quality; support disable, rollback, deprecate, quarantine, and revoke.

Removal is dependency-aware. Core blocks new resolution, drains or cancels affected leases safely, preserves historical manifest/hash/evaluation evidence, and removes executable or sensitive content according to retention policy. An active mission whose reproducibility depends on a revoked package becomes blocked or requires an authorized replacement plan.

## Runtime isolation and capabilities

Extensions do not share unrestricted process memory, Core database access, secrets, or the user’s filesystem. UI extensions receive a narrow view/command API; worker, skill, hook, LSP/retrieval, and toolchain packages execute in appropriate isolated hosts; integrations and MCP servers are proxied through policy enforcement; brain adapters obtain provider credentials only through the Brain Gateway.

Core-issued capabilities bind project, phase, task, subject, action, resource, classification, conditions, budget, and expiry. A capability for reading context does not imply a write; a hook invocation does not inherit the triggering effect; a plugin cannot exchange its token for a broader lease.

## Hook Bus policy

Hook types and payload schemas are versioned. A handler declares whether it is:

- **observational:** records or exports an allowed observation;
- **validating:** returns a typed validation result;
- **blocking:** can request denial/hold under an approved rule but cannot advance state;
- **effect-producing:** may only propose a new effect, which re-enters the complete Effect Firewall.

Ordering, timeout, recursion, idempotency, data access, failure behavior, and conflict resolution are explicit. Unknown or failed blocking hooks fail safely according to policy. Hooks cannot edit approved artifacts, mint capabilities, suppress mandatory evidence, or communicate externally by implication.

## Versioning, compatibility, and evaluation

Public extension APIs are versioned independently of internal modules. Compatibility is checked at installation and task resolution. An update cannot silently change requested capabilities, output/evidence schemas, retrieval trust, or model data handling; expansion requires a new policy decision. Major versions coexist during a bounded migration window.

Package evaluation is evidence, not a permanent guarantee. Provider drift, changed dependencies, new advisories, repeated schema failures, unsafe effects, or degraded outcome-equivalence rates can trigger recertification, quarantine, fallback, or revocation. Controlled harness and ablation runs use exact package fingerprints.

## Marketplace stance

V1 uses a local and organization-approved registry. An open marketplace is outside the governed-delivery increments. Community distribution, if later authorized, requires publisher verification, signed releases, permission transparency, automated inspection, evaluation evidence, vulnerability disclosure, and rapid revocation. Ratings never override policy or evidence.

## MCP policy

MCP servers are third-party integrations with typed tools/resources, not trusted local helpers. Each is versioned, evaluated, permissioned, mediated, and sandboxed where possible. Returned content carries source/trust/classification and passes context-injection defenses. MCP-facing user operations call the same Core command/query workflow as CLI, Studio, and other APIs; MCP never becomes an alternate authority path.

See [32_Hermes_Engineering_Intelligence_Runtime.md](32_Hermes_Engineering_Intelligence_Runtime.md), [34_Harness_Evaluation_Protocol.md](34_Harness_Evaluation_Protocol.md), [06_Brain_Architecture.md](06_Brain_Architecture.md), [09_Skills_System.md](09_Skills_System.md), [14_Security_Architecture.md](14_Security_Architecture.md), [18_API_Architecture.md](18_API_Architecture.md), and [28_Release_Strategy.md](28_Release_Strategy.md). Proposed ADRs 0018–0020 define the skill/hook, context/retrieval, and typed-memory boundaries; Proposed ADR 0024 governs evaluation and profile certification.
