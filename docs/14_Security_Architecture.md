# 14 — Security Architecture

## Security and authority posture

Autonomous engineering is a high-consequence execution system. Security is a non-bypassable policy and enforcement discipline, not a scanner invoked at release time. The secure default is: isolated worktree, no network, no home-directory access, no ambient credentials, minimal provenance-labelled context, immutable evidence identity, bounded budgets, and explicit authority for protected operations.

Core is the only component that commits durable governance/workflow truth, evaluates and records policy decisions, records approvals, issues capabilities, accounts budgets, validates transitions, records effect intent and observed result, decides completion, and establishes recovery truth. The Brain Agent proposes plans; Hermes supervises; builders and subagents propose actions and return artifacts, findings, or evidence; tool boundaries execute an authorized effect. None may approve itself or write the ledger directly.

The governing precedence is:

`Policy > approved EngineeringSpecification > approved OutcomeContract > approved DeliveryPlan > approved PhasePlan > authorized TaskPacket > ToolAction`

Lower layers may narrow authority but never silently broaden scope, weaken evidence, or redefine a higher contract.

## Threat model

| Asset / boundary | Primary threats | Core mitigations |
| --- | --- | --- |
| Specifications and plans | prompt-injected requirements, self-approval, silent scope drift, stale Task Packet | immutable approved versions, typed change requests, authority separation, trace validation |
| Source and repositories | unbounded edits, exfiltration, destructive Git/process actions | isolated worktrees, scoped capabilities, Effect Firewall, reviewable artifacts |
| Secrets and credentials | prompt/log/context leakage, plugin theft, provider egress | secret broker, no ambient exposure, classification, minimization, redaction |
| User identity and decisions | spoofed messages, approval replay, confused deputy | authenticated principal, expiring decision requests, exact artifact/version binding |
| Worker/tool host | malicious repository, sandbox escape, dependency exploit, uncertain crash effect | process/container isolation, quotas, network deny, result probes, posture disclosure |
| Context, LSP, and retrieval | poisoned retrieval, omitted impact, semantic result overriding exact/policy truth | source ranking, provenance, trust/freshness, exact-source precedence, validation |
| Memory | poisoned/stale lesson, cross-project leak, Governance Memory downgraded to advice | typed stores, Memory Judge, access scope, contradiction/freshness/supersession |
| Skills, hooks, plugins, MCP | supply-chain compromise, capability escalation, hook reentrancy or escape | signatures/hashes, declared contracts, isolation, bounded hooks, revocation |
| Hermes and subagents | uncontrolled swarm, authority leakage, write conflict, fabricated handoff | bounded supervisor, one objective, scoped context/tools/budget, structured results |
| Brain/builder profiles | non-convergence, deceptive confidence, provider drift, quality-bar reduction | fixed outcome contract, independent evidence, retry caps, certification/evaluation |
| Event/evidence state | forged transition, replay, direct boundary write, deletion/tampering | Core-only writer, optimistic concurrency, idempotency, hashes, retained audit proof |
| Harness evaluation | benchmark contamination, non-reproducibility, unsafe optimization | controlled fixtures, fixed variables, ablation records, safety/truthfulness metrics |

## Effect Firewall

Every consequential effect, regardless of caller or surface, follows one ordered protocol:

1. **Propose:** Brain Agent, Hermes, builder, subagent, user surface, or integration submits a typed `ToolAction` proposal tied to a Task Packet or authorized administrative command.
2. **Classify:** Core identifies action, resource, project, environment, data, reversibility, blast radius, and risk class.
3. **Policy:** Core evaluates the exact governing policy and higher-contract envelope; denial is durable and actionable.
4. **Capability:** Core binds a least-privilege, single-purpose, expiring capability to subject, action, resource, conditions, budget, task, correlation, and idempotency key. A protected capability remains unusable until required approval is recorded.
5. **Approval:** where policy requires it, an authenticated authorized principal decides an expiring request bound to the exact proposal and capability scope. Any material change invalidates the approval.
6. **Durable intent:** Core records the approved effect intent before dispatch. Intent does not claim success.
7. **Execute:** the policy-enforcement/tool boundary validates the active capability immediately before performing the effect.
8. **Probe:** the boundary observes actual postconditions, including partial or uncertain effects after timeout/crash; it never infers rollback from lost delivery.
9. **Evidence:** the boundary reports result and evidence to Core. Core validates, records the result, accounts actual budget, and alone advances any workflow state.

~~~mermaid
flowchart LR
  P["Typed effect proposal"] --> C["Core classification"]
  C --> D["Policy decision"]
  D --> K["Scoped capability"]
  K --> A["Approval if required"]
  A --> I["Core durable intent"]
  I --> E["Enforcement boundary executes"]
  E --> O["Probe observed result"]
  O --> R["Report evidence to Core"]
  R --> T["Core records result / transition"]
~~~

The protocol covers filesystem writes and deletion; processes and shell; Git commits, pushes, branches, pull requests, merges, and history operations; network and external messages; secrets; dependency, toolchain, plugin, skill, and MCP changes; database queries and migrations; provider/model calls and context egress; budget increases; package publication; releases; and deployments. Prompt wording is never an enforcement mechanism.

## Policy and approval semantics

Policy decisions bind subject, action, resource, project, governing artifact versions, data classification, environment, risk, autonomy mode, time, and rule version. Approval is a distinct human or enterprise authority fact, not the same as policy evaluation and not implied by an autonomy label.

Enforcement points exist at the Brain Gateway for context egress and model spend, Context Compiler/retrieval brokers for source access, Skill and Hook runtimes for invocation, Tool Executor for effects, Worker Host for sandbox resources, Registry for packages, Messenger/Integration Gateway for disclosure, and Core for all transitions. An enforcement point can deny or report; it cannot mint broader authority or write governance truth.

## Core controls

### Execution, isolation, and WIP

Worker workspaces are isolated from the primary worktree. Network is denied unless a task-specific policy permits named destinations and purpose. CPU, memory, process, disk, model, token, time, retry, and WIP quotas prevent denial-of-service and runaway loops. Sandboxing is tested on each supported OS; missing controls reduce allowed autonomy and are visibly reported.

One writable implementation task is admitted per isolated worktree by default. Parallel read-only exploration, unresolved decisions, and pending protected effects are bounded. Hermes schedules within limits; Core owns admission and counters.

### Secrets, context, and sensitive data

Secrets are referenced through a broker with narrow, short-lived access, never copied into prompts, skills, logs, ordinary artifacts, or configuration. Classification governs context compilation, retrieval, brain routing, tool input, messaging, retention, and export. Every context item carries source, trust, freshness, scope, selection reason, token estimate, and provenance. Semantic retrieval cannot override exact source, policy, approved specification, ADR, or LSP evidence.

### Skills, hooks, subagents, and extensions

H3.1 skills are repository-owned, statically registered, versioned/hash-pinned executable procedures with declared context, tools, capabilities, verification, output schema, and evidence contract; a generalized signing/installation surface is deliberately absent. If conditional H3.2 packaging is later authorized, packages additionally require identity, signature/provenance, staging, atomic activation, rollback, and revocation. Hooks are typed as observational, validating, blocking, or effect-producing; an effect-producing hook re-enters the Effect Firewall and cannot use the triggering capability by implication. Reentrancy, time, failure, and recursion limits are enforced.

Each subagent receives one objective, scoped context, allowed skills/tools/capabilities, budget, deadline, output schema, and evidence requirements. Read-only exploration may be bounded in parallel; writable work remains isolated and conflict-controlled. Subagents return structured findings through Hermes to Core and cannot approve plans, effects, gates, or completion.

### Supply chain and profile trust

Brains, builders, workers, built-in skills/hooks, toolchains, LSP/retrieval adapters, and MCP servers are versioned principals or resources with exact identity/integrity and declared capability ceilings; they are not all installable packages. H3.1 built-ins resolve only from the reviewed release/repository. Where a later gate explicitly authorizes a distributable plugin, skill, hook, adapter, toolchain, or server package, staging verifies identity, signature/provenance, integrity, compatibility, vulnerabilities, license/policy, data handling, requested permissions, and evaluation evidence before atomic Core activation. Permission expansion requires a new decision. Critical advisories trigger quarantine/revocation and active-mission impact analysis.

Reference-system compatibility never creates trust. ACP, MCP, JSONL/OpenAPI clients, imported workflows, editor agents, terminal agents, language servers, model routers, package catalogs, and remote workers are least-privilege principals behind the same Core policy and Effect Firewall. Session plans, permission prompts, trust labels, worktrees, transcripts, summaries, checkpoints and telemetry are projections or controls with limited scope; none can mint a capability, approve a plan/effect/completion, write the ledger, or establish external-effect recovery truth.

The repository currently has no approved repository-level license/notice policy. Consequently, even permissively licensed upstream code is unavailable for reuse until that policy exists and a component-level provenance review records the exact source revision, license/notice/patent/copyleft obligations, intended use, modifications, reviewer and approval. Proprietary, mixed-license, strong-copyleft, enterprise-only and unlicensed material remains documentation-informed clean-room input unless separately approved. RS-00, RS-11, RS-19 and RS-20 in the reference study are mandatory negative/admission tests for these boundaries.

Supported brain/builder profiles are certified against controlled outcome, safety, evidence, and non-convergence scenarios. A weaker profile may consume more time or escalate more often; policy never lowers the final acceptance bar for it.

## Security Division responsibilities

- Maintain threat models when authority, data, effect, model, package, hook, memory, or remote boundaries change.
- Define severity, remediation SLAs, exception expiry, protected actions, and release-blocking criteria.
- Run appropriate code, dependency/SBOM, secret, supply-chain, sandbox, context-injection, evidence-integrity, and infrastructure checks.
- Audit policy rules, approvals, capabilities, providers, skill/hook provenance, retrieval ranking, memory consolidation, and profile evaluation.
- Operate incident response: contain, preserve evidence, revoke, investigate, communicate, recover, and update controls.

Security findings are typed evidence with severity, confidence, affected artifacts, exploitability context, remediation owner, due date, and disposition. Suppression requires an authorized rationale and expiry; it never deletes the finding or silently passes a gate.

## Incident and recovery model

On suspected compromise, Core stops affected capabilities rather than unrelated missions, revokes tokens, preserves relevant snapshots and event/artifact identities, restricts egress, and alerts authorized users. Investigation traces mission intent through specification, plan, context, brain/skill/hook invocation, tool intent, observed effect, artifact, and package version. Recovery requires a documented remediation, postcondition probe, validation run, and updated threat model or control.

If an effect outcome is uncertain, Core records `Uncertain`, blocks unsafe retry, and requires a recovery probe or authorized decision. A lost worker or channel is never evidence that a real-world effect failed cleanly.

## Hard gates

The following always use a protected policy path regardless of autonomy mode: destructive operations; protected branches; pushes, pull requests, merges, and public release; production or protected deployment; secret access; new external destination; compliance-significant export; new dependency/toolchain/package or capability expansion; database migration; policy exception; high/critical finding disposition; specification, scope, budget, or acceptance-bar change; and any unresolved effect whose safe replay cannot be established.

The advisory [Engineering Committee](36_Engineering_Committee.md) adds a deliberation threat surface — injection through pack content, member impersonation or silent model substitution, correlated members, fabricated citations, cost amplification, minority-report suppression, and credential or context egress — governed by its own threat model (36 §11): member output is untrusted until structurally validated and evidence-linked, credentials never enter model-visible content, per-member egress projections are recorded by hash, and no Committee artifact carries authority.

See [31_Governed_Specification_Workflow.md](31_Governed_Specification_Workflow.md), [32_Hermes_Engineering_Intelligence_Runtime.md](32_Hermes_Engineering_Intelligence_Runtime.md), [34_Harness_Evaluation_Protocol.md](34_Harness_Evaluation_Protocol.md), [35_Reference_Systems_and_Competitive_Architecture.md](35_Reference_Systems_and_Competitive_Architecture.md), [02_Product_Principles.md](02_Product_Principles.md), [05_Worker_Architecture.md](05_Worker_Architecture.md), [15_Plugin_System.md](15_Plugin_System.md), [20_Risk_Assessment.md](20_Risk_Assessment.md), and [26_Testing_Strategy.md](26_Testing_Strategy.md). Proposed ADRs 0015–0024 define the associated governance, runtime, context, memory, loop, subagent, convergence, and evaluation decisions; they are not implementation authorization while Proposed.
