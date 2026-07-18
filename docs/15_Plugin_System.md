# 15 — Plugin, Skill, and Extension System

## Goal and authority boundary

In the target architecture, externally distributable brain and builder adapters, worker hosts, Hermes skills, hook handlers, LSP/retrieval adapters, UI extensions, MCP servers, integrations, and toolchains use an inspectable, permissioned, versioned, removable, and revocable package lifecycle. H3.1 repository-owned built-ins instead use the normal reviewed source/manifests lifecycle described below. Neither path is ever a route around Core authority, the Effect Firewall, evidence, or observability.

For any later packaged resource, Core records package policy, approval, activation scope, capability issuance, revocation, budget, and any workflow transition. The Registry verifies and stages packages; Hermes may route an already approved skill or adapter; a package cannot activate itself, broaden its permissions, approve its output, or write durable governance state. H3.1 has no Registry path and resolves only reviewed repository-owned manifests by exact hash.

This document describes the long-term extension architecture. Delivery is deliberately split. **H3.1** proves a repository-owned, built-in Agent Kernel, Skill Runtime, and typed lifecycle Hook Bus without an installer or third-party executable extension surface. **H3.2** may add governed packaging only after H3.1 has demonstrated measurable value and the package-governance experiments have passed. Describing a later package lifecycle here does not move registry, marketplace, signing infrastructure, or external installation into H3.1.

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
| Agent/client protocol adapter | ACP, JSONL/OpenAPI or editor/terminal-agent interoperability | Core command/query/event adapter | confused deputy, semantic loss, direct filesystem/terminal authority |
| UI extension/theme | presentation, panels, commands | Studio extension host | UI confusion, data exposure, false authority |

## H3.1 built-in runtime descriptor

H3.1 resources live in the WePLD repository and use static, versioned manifests. Each manifest declares an exact identity/version and content hash; typed input and output schemas; applicability; required context, tools, and Core-issued capabilities; procedure and bounded failure behavior; verification and evidence obligations; compatibility; and evaluation fixtures. The recorded repository revision, manifest hash, skill hash, built-in hook set, configuration, and model/profile identity make every use reconstructable.

The H3.1 runtime does not discover or install external packages, resolve third-party dependencies, contact a registry, operate a marketplace, or execute third-party hooks. It does not require a general publisher-signing service. A repository review and exact content hash identify the built-in resource; Core still grants capabilities and accepts evidence. Built-in lifecycle hooks are compiled or repository-owned handlers registered through typed, versioned contracts and may not be replaced by ambient configuration.

## Long-term universal package descriptor

Beginning no earlier than H3.2, every externally distributable package declares identity, publisher, semantic version, integrity hash, signature/provenance, compatibility range, dependency graph, requested capabilities, supported platforms, data handling and destinations, license, configuration schema, migration/rollback instructions, health checks, evaluation evidence, release channel, and revocation information. Type-specific metadata augments rather than replaces this descriptor.

An agent/client protocol adapter is a projection and request translator, never an agent trust boundary. ACP sessions/plans/permissions, MCP declarations, JSONL events and imported workflow files map to exact Core identities and validated commands; unsupported or authority-losing mappings fail closed. The adapter cannot mint capabilities, invoke raw client filesystem/terminal access by default, write authoritative state, or convert transport success into effect/evidence success. ACP remains an H9 experiment under a new Proposed ADR after Core contracts stabilize.

Exact package set, configuration fingerprint, model/profile identity, skill version, hook set, toolchain, and adapter versions are recorded for every relevant task attempt and evaluation. Display names are never identity.

Upstream availability is not installability. Until WePLD has an approved repository licensing/contribution policy and an exact component-level provenance review, reference-system implementations remain clean-room and no source/template/fixture reuse is authorized. RS-00, RS-03, RS-05, RS-06, RS-11, RS-19, RS-20, RS-23 and RS-26 in [35_Reference_Systems_and_Competitive_Architecture.md](35_Reference_Systems_and_Competitive_Architecture.md) are the admission experiments for the relevant runtime, projection, output and protocol ideas.

## Hermes skill contract

A skill is an executable, testable procedure—not only prompt text. Its H3.1 repository-owned manifest declares:

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

## Capability-projected tool and catalogue views

Hermes does not send every registered tool schema to every model. For each attempt, Core deterministically intersects the approved Task Packet, current policy, issued capabilities, role, workspace scope, data classification, runtime availability, and budget. Hermes receives a `CapabilityProjectedToolCatalog` containing only eligible tool identities and versioned input/output schemas, plus exact hashes, selection reasons, capability classes, effect classes, and declared context cost. Denied, unavailable, out-of-scope, or incompatible tools are omitted and recorded with reasons in the attempt manifest.

Schema visibility is not authorization. Every call still re-enters capability, policy, approval, idempotency, and Effect Firewall enforcement; a forged or stale schema cannot invoke a tool. Conversely, hiding a tool is context hygiene, not a security boundary. OpenCode's per-agent tool/skill/MCP filtering is useful evidence for reducing irrelevant schemas, but its permissive or automatic trust posture is not adopted.

Tool, skill, and future MCP catalogue descriptions have explicit item and token budgets inside the context pack. Mandatory authority is never displaced to fit a catalogue. H3.1 can project only repository-owned built-ins. Future MCP or package catalogues remain disabled unless their owning gates pass; discovery returns a bounded, provenance-labelled candidate list and never auto-connects, auto-installs, auto-trusts, or grants ambient package authority.

The first slice contains only the minimum built-in skill families needed by accepted H3.1 fixtures. Adding a family means a normal reviewed repository change with a new manifest/content hash; it is not an installation operation. Dynamic discovery, downloaded resources, public registries, arbitrary executable hooks, and general-purpose package signing belong to no earlier than H3.2.

## H3.2 governed packaging lifecycle

This lifecycle is conditional, not part of the H3.1 exit. It may begin only if built-in skill/hook evidence shows enough measured value to justify a distributable package surface and RS-19 proves the governance controls. If those conditions are not met, WePLD retains the built-in, repository-reviewed runtime.

1. Discover through a local or organization-approved registry.
2. Resolve versions and dependencies without mutating the active environment.
3. Verify identity, signature, hash, publisher policy, license, advisory status, data handling, and compatibility.
4. Evaluate declared fixtures, schemas, failure paths, evidence production, and capability behavior in isolation.
5. Present exact permissions, external destinations, hook class, budget impact, and trust evidence for approval where required.
6. Install into an isolated versioned store; do not activate on install.
7. Core atomically activates an approved version for named organization/project/mission scopes and records the event.
8. Monitor health, profile drift, vulnerabilities, and evidence quality; support disable, rollback, deprecate, quarantine, and revoke.

Removal is dependency-aware. Core blocks new resolution, drains or cancels affected leases safely, preserves historical manifest/hash/evaluation evidence, and removes executable or sensitive content according to retention policy. An active mission whose reproducibility depends on a revoked package becomes blocked or requires an authorized replacement plan.

## Bounded tool results

The H3.1 tool contract declares output limits and a deterministic head, tail, range, or structured-reduction strategy. When raw output exceeds the attempt's byte, line, item, or token budget, the model receives a bounded excerpt with an explicit truncation marker and a content-addressed `ToolOutputArtifact` reference. The artifact binds the full raw-result hash, tool/action/attempt IDs, producer/version, exit and error state, byte/line/item counts, excerpt and strategy hashes, classification/redaction/retention policy, and capture time.

Complete output is durable evidence only when policy permits capture; otherwise the record states the governed omission or redaction. Full content is never silently reinserted into a later prompt. An authorized range/chunk request is a new, logged context-selection decision. A temporary path alone is not durable evidence, and a summary or excerpt never becomes authority merely because the full result was large.

## Runtime isolation and capabilities

Extensions do not share unrestricted process memory, Core database access, secrets, or the user’s filesystem. UI extensions receive a narrow view/command API; worker, skill, hook, LSP/retrieval, and toolchain packages execute in appropriate isolated hosts; integrations and MCP servers are proxied through policy enforcement; brain adapters obtain provider credentials only through the Brain Gateway.

Core-issued capabilities bind project, phase, task, subject, action, resource, classification, conditions, budget, and expiry. A capability for reading context does not imply a write; a hook invocation does not inherit the triggering effect; a plugin cannot exchange its token for a broader lease.

## Hook Bus policy

H3.1 supplies only built-in, typed lifecycle hooks required by the kernel and its evidence contract. The following policy is also the boundary for any later externally packaged handler; H3.1 does not authorize such handlers.

Hook types and payload schemas are versioned. A handler declares whether it is:

- **observational:** records or exports an allowed observation;
- **validating:** returns a typed validation result;
- **blocking:** can request denial/hold under an approved rule but cannot advance state;
- **effect-producing:** may only propose a new effect, which re-enters the complete Effect Firewall.

Ordering, timeout, recursion, idempotency, data access, failure behavior, and conflict resolution are explicit. Unknown or failed blocking hooks fail safely according to policy. Hooks cannot edit approved artifacts, mint capabilities, suppress mandatory evidence, or communicate externally by implication.

## Versioning, compatibility, and evaluation

The H3.1 internal contracts are versioned independently of internal modules and checked at task resolution. H3.2 and later public extension APIs, if authorized, add install-time compatibility checks. An update cannot silently change requested capabilities, output/evidence schemas, retrieval trust, or model data handling; expansion requires a new policy decision. Major externally packaged versions may coexist only during a bounded migration window.

Package evaluation is evidence, not a permanent guarantee. Provider drift, changed dependencies, new advisories, repeated schema failures, unsafe effects, or degraded outcome-equivalence rates can trigger recertification, quarantine, fallback, or revocation. Controlled harness and ablation runs use exact package fingerprints.

## Marketplace stance

H3.1 has no registry or marketplace. H3.2 may introduce an isolated staging source and organization-approved package catalogue only after measured benefit and governance proof; it does not imply an open marketplace. Community distribution remains outside the governed-delivery increments and, if later authorized, requires publisher verification, signed releases, permission transparency, automated inspection, evaluation evidence, vulnerability disclosure, and rapid revocation. Ratings never override policy or evidence.

## MCP policy

MCP servers are third-party integrations with typed tools/resources, not trusted local helpers. Each is versioned, evaluated, permissioned, mediated, and sandboxed where possible. Returned content carries source/trust/classification and passes context-injection defenses. MCP-facing user operations call the same Core command/query workflow as CLI, Studio, and other APIs; MCP never becomes an alternate authority path.

See [32_Hermes_Engineering_Intelligence_Runtime.md](32_Hermes_Engineering_Intelligence_Runtime.md), [34_Harness_Evaluation_Protocol.md](34_Harness_Evaluation_Protocol.md), [06_Brain_Architecture.md](06_Brain_Architecture.md), [09_Skills_System.md](09_Skills_System.md), [14_Security_Architecture.md](14_Security_Architecture.md), [18_API_Architecture.md](18_API_Architecture.md), and [28_Release_Strategy.md](28_Release_Strategy.md). Proposed ADRs 0018–0020 define the skill/hook, context/retrieval, and typed-memory boundaries; Proposed ADR-0024 defines the evaluation spine and Proposed ADR-0025 governs model/profile certification.
