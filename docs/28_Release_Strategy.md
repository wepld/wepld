# 28 — Release Strategy

## Release philosophy

WePLD releases a control plane that can change repositories, invoke providers, and manage credentials. Releases must be reversible, explainable, signed, measurable, and staged. A successful build is not a release; compatibility, migration, policy, security, and operational recovery are part of release readiness.

## Release channels

| Channel | Audience | Purpose | Policy |
| --- | --- | --- | --- |
| Development | internal engineers | rapid integration and fixture execution | no production credentials/data; frequent diagnostics |
| Preview | invited design partners | validate narrow workflows and UX | feature flags, explicit data/telemetry consent |
| Beta | broader approved users | reliability and compatibility validation | staged rollout, support/recovery runbook |
| Stable | supported production users | predictable operation | signed release, migration/rollback evidence |
| Enterprise LTS | managed organizations | controlled cadence and retention | approved package/policy matrix, extended support |

## Versioning and compatibility

The desktop shell, Core Daemon, public contracts, schemas, worker/brain/plugin adapters, skills, and package registry versions are independently tracked but compatibility-tested as a release set. Semantic versioning is used for public contracts; breaking changes require a new major contract and coexistence/migration window. Internal refactors do not force external version changes.

Every completed mission/attempt records the relevant component, schema, package, skill, worker, brain profile, and toolchain versions. This makes historical evidence interpretable after an upgrade.

## Release readiness gates

- approved scope and changelog with linked mission/ADR/risk review;
- reproducible build, signing, SBOM/provenance, dependency/license/advisory review;
- unit/property/contract/integration/E2E/evaluation/security/accessibility/performance results at required thresholds;
- migration, downgrade, backup/restore, and interrupted-upgrade scenario evidence;
- supported-platform sandbox/permission posture verification;
- telemetry, alerting, support, incident, and rollback runbooks reviewed;
- no unapproved critical/high security finding or expired exception.

## Staged rollout and monitoring

Releases begin in internal/dev, move through preview/beta cohorts, then expand based on health metrics: startup/recovery, task/worker failure, policy denials/false positives, provider errors, resource pressure, data migration integrity, and user-reported outcome quality. Feature flags control behavior but never disable hard security gates. Rollout gates have owners, thresholds, and a halt/rollback path.

## Migration and rollback

Operational-store migrations are forward-compatible and resumable when possible. Before migration, the system creates a verified backup and communicates any downtime/data impact. After an interruption, the daemon detects incomplete migration and either resumes safely, restores, or enters a supportable read-only recovery state. Rollback preserves user data and audit semantics; it does not use destructive reset as a default.

Package/schema changes are coordinated: an older Core must reject an incompatible adapter safely, and a new Core must retain readers/upcasters or export tools for supported historical data. A revoked package can be disabled without making an audit timeline unreadable.

## Security and disclosure

Release artifacts are signed and verifiable; updater channels validate authenticity and rollback protections appropriate to the platform. Vulnerability handling includes intake, triage, scoped mitigation, advisory, credit/communication policy, remediation release, and post-incident lessons. Sensitive incident details go only to authorized channels through Messenger policy.

## Customer communication

Messenger and release notes distinguish new capability, changed permission/data behavior, known limitations, required user action, migration status, and rollback instructions. The product should not surprise a user with a new external provider, permission scope, or network destination after an update.

See also: [14_Security_Architecture.md](14_Security_Architecture.md), [15_Plugin_System.md](15_Plugin_System.md), [17_Event_System.md](17_Event_System.md), [20_Risk_Assessment.md](20_Risk_Assessment.md), and [22_Milestones.md](22_Milestones.md).

