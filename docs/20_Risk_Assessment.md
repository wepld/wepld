# 20 — Risk Assessment

## Risk posture

WePLD combines autonomous execution, proprietary source, external model providers, plugins, and executive decision-making. The architecture treats uncertainty as operational state: risks have owners, triggers, controls, evidence, and review dates. A risk register is not a one-time compliance artifact.

## Top risk register

| Risk | Likelihood | Impact | Early trigger | Primary mitigation | Owner |
| --- | --- | --- | --- | --- | --- |
| Product collapses into an AI IDE | Medium | High | roadmap dominated by editor features / chat | V1 scope lock; Mission Control and durable workflow first | Product/CTO |
| Unsafe tool execution or prompt injection | High | Critical | undeclared tool request, data exfiltration attempt | isolated worktrees, capability enforcement, no-network default, input classification | Security |
| Model unreliability causes incorrect changes | High | High | schema failures, disagreeing reviews, repeated rework | structured outputs, independent review, gates, evaluation suites, retry ceilings | Quality |
| Runaway cost/resource consumption | Medium | High | budget burn rate/CPU above envelope | profile budgets, quotas, rate limits, scheduler backpressure, mandatory stop | Orchestration |
| State loss or duplicate execution after crash | Medium | High | missed heartbeats / uncertain effect | event ledger, idempotency, leases, recovery probes, backups | Core |
| Plugin/skill supply-chain compromise | Medium | Critical | advisory, signature mismatch, permission change | signed packages, isolation, allowlists, SBOM/advisory scans, revocation | Security |
| Sensitive context leaks to provider/channel | Medium | Critical | classification mismatch / redaction failure | brokered credentials, egress policy, minimization, audit, local profiles | Security/Privacy |
| Knowledge poisoning or stale guidance | Medium | High | uncited claim, superseded source, conflicting evidence | provenance, freshness, access controls, review and retrieval audit | Knowledge |
| Cross-platform sandbox gap | High | High | control unavailable on an OS | platform posture tests, reduced autonomy, documented support tiers | Core/Security |
| UI overstates progress or confidence | Medium | High | missing source/freshness on green status | projection-only UI, evidence drawer, staleness indicators | Studio/Quality |
| Sync/conflict complexity overwhelms V1 | Medium | High | remote/multi-user demand before local recovery | defer sync, single writer, event/blob replication design | CTO |
| Regulatory/compliance mismatch | Medium | High | data residency/retention conflict | enterprise policy mode, classification, legal review, configurable retention | Legal/Compliance |

## Risk tolerance by action

| Class | Examples | Default treatment |
| --- | --- | --- |
| Low | read local project, analyze tests, draft plan | autonomous inside task scope |
| Moderate | isolated worktree edit, local test, knowledge proposal | automated with capability and evidence |
| High | install dependency, enable network, external provider egress, merge proposal | policy gate; often approval in Limited mode |
| Critical | secrets, production deployment, destructive operation, policy exception, public release | protected human/enterprise decision path |

Risk classification is contextual: a local benchmark may be low risk while its code execution is high risk for an untrusted repository. The Policy Engine determines class from action, resource, data, environment, actor, and mission mode.

## Residual-risk decisions

The following choices require executive sign-off before Phase 2: supported operating systems/sandbox guarantees; whether any hosted brain profile may process Confidential data; initial provider retention policy; default approval envelope; maximum allowed local resource use; definition of protected deployment; and minimum security/review gate for a completed change.

## Review cadence and escalation

Risk owners review high/critical risks before each release, whenever a new external boundary is introduced, after an incident, and when an assumption is invalidated. Messenger escalates urgent risks via the configured channel but never discloses restricted details to an unauthorized destination. A risk with no credible mitigation blocks the affected capability, not necessarily unrelated project work.

## Acceptance criteria

- Every risk has an owner, trigger, mitigation, residual status, and linked architecture/control evidence.
- Autonomy modes map to explicit action-risk rules rather than vague trust labels.
- New plugin/provider/worker/sync capability cannot ship without threat-model and risk-register updates.

See also: [02_Product_Principles.md](02_Product_Principles.md), [10_Loop_Engineering.md](10_Loop_Engineering.md), [14_Security_Architecture.md](14_Security_Architecture.md), [22_Milestones.md](22_Milestones.md), and [28_Release_Strategy.md](28_Release_Strategy.md).

