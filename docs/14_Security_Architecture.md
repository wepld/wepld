# 14 — Security Architecture

## Security posture

Autonomous engineering is a high-consequence execution system. Security is a first-class engineering organization and a non-bypassable policy layer, not a scanner invoked at release time. The secure default is: isolated worktree, no network, no home-directory access, no ambient credentials, minimal context, immutable audit evidence, and explicit approval for escalation.

## Threat model

| Asset | Primary threats | Core mitigations |
| --- | --- | --- |
| Source code and repositories | unbounded edits, exfiltration, destructive commands | isolated worktrees, capability allowlists, Git-based review, action audit |
| Secrets and credentials | prompt injection, log leakage, plugin theft, provider egress | OS/enterprise vault, secret broker, redaction, no ambient env exposure |
| User decisions / identity | spoofed channel messages, approval replay, social engineering | authenticated principal, signed/expiring commands, decision provenance |
| Worker/tool host | malicious code, sandbox escape, dependency exploit | process/container isolation, resource/network limits, platform posture disclosure |
| Knowledge | data poisoning, stale claims, unauthorized retrieval | provenance, source links, classification, freshness, access-controlled retrieval |
| Plugins/skills | supply-chain compromise, capability escalation | signatures, package hashes, capability grants, isolation, revocation |
| Control-plane state | forged transitions, replay, tampering | single writer, optimistic concurrency, event hashes, least-privilege local RPC |

## Security architecture

~~~mermaid
flowchart LR
  Actor["Worker / Plugin / UI request"] --> PDP["Policy Decision Point"]
  PDP --> Decision["Permit • Deny • Require approval"]
  Decision --> PEP["Policy Enforcement Point\nTool, Brain, Integration, Registry"]
  PEP --> Effect["Scoped effect"]
  Effect --> Evidence["Immutable audit / artifacts"]
  Sec["Security Division"] --> PDP
  Sec --> Evidence
~~~

Policy decisions bind a subject (user, worker role, plugin), action, resource, project, data classification, environment, risk level, and time. Enforcement lives at every boundary: Brain Gateway for data egress, Tool Executor for filesystem/process/network actions, Registry for packages, Messenger for disclosure, and Core for state transitions.

## Security Division responsibilities

- Threat model every major capability and update it when trust boundaries change.
- Run code review, dependency/SBOM, secret, supply-chain, container, cloud, and infrastructure checks appropriate to the mission.
- Define severity, remediation SLAs, exception rules, and release-blocking criteria.
- Operate incident response: contain, preserve evidence, rotate/revoke, investigate, communicate, recover, and learn.
- Audit policy rules, plugin/skill provenance, provider data handling, and sandbox posture.

Security findings are typed evidence with severity, confidence, affected artifacts, exploitability context, remediation owner, due date, and disposition. Suppression requires rationale and authorized expiry; it does not delete the finding.

## Core controls

### Execution and sandboxing

Every tool effect requires a scoped, expiring capability token and idempotency key. Worker workspaces are isolated from the primary worktree. Network is denied unless a task-specific policy permits destinations and purpose. CPU, memory, process, disk, and time quotas prevent local denial-of-service. Sandboxing must be tested on every supported OS; where a control is unavailable, policy reduces allowed autonomy and visibly reports the gap.

### Secrets and sensitive data

Secrets are referenced through a broker with narrow, short-lived access, never copied into prompts, skill files, logs, artifacts, or configuration. Data classification governs brain routing and messaging. Redaction occurs before persistence/export, with a protected raw evidence path only where authorized for incident response.

### Supply chain and extensions

Brains, workers, skills, plugins, toolchains, and MCP servers are versioned capability packages. Installation verifies identity, integrity, compatibility, vulnerabilities, license/policy, and requested permissions. Critical advisories trigger quarantine/revocation and active-mission impact assessment.

## Incident and recovery model

On suspected compromise, the system stops affected capabilities—not all missions—revokes tokens, snapshots relevant evidence, restricts data egress, and alerts authorized users through Messenger. Investigation uses Timeline links from mission intent to tool action, artifact, provider invocation, and package version. Recovery requires a documented remediation, validation run, and an updated threat model or control where needed.

## Hard gates

The following always require a protected policy path regardless of autonomy mode: destructive operations; protected branches; production deployment; secret access; new external destination; new dependency/toolchain/package; policy exception; high/critical security finding disposition; budget/scope expansion; and compliance-significant data export.

See also: [02_Product_Principles.md](02_Product_Principles.md), [05_Worker_Architecture.md](05_Worker_Architecture.md), [15_Plugin_System.md](15_Plugin_System.md), [20_Risk_Assessment.md](20_Risk_Assessment.md), and [26_Testing_Strategy.md](26_Testing_Strategy.md).

