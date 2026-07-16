# 28 — Release Strategy

## Release philosophy and current status

WePLD releases a control plane that can change repositories, invoke providers, and eventually broker credentials and external effects. Releases must be reversible, explainable, signed, measurable, and staged. A successful build, green Draft PR, model benchmark, or milestone demo is not a release; compatibility, migration, policy, security, evidence truth, and operational recovery are release inputs.

Canonical `main` currently contains the planning package. Draft PR #1 is an open, unmerged, unratified candidate Build Feature baseline. Its candidate contract version, event vocabulary, test count, and branch-local “M0” labels are not official release versions or release evidence until the Build Feature Baseline Gate independently accepts them. This document does not authorize a push, merge, tag, or release.

## Release channels

| Channel | Audience | Purpose | Policy |
| --- | --- | --- | --- |
| Candidate baseline | independent reviewers | decide whether a narrow prerequisite is acceptable | no distribution; head pinned; V0 limits and findings explicit |
| Development | internal engineers | rapid fixture and integration execution | no production credentials/data; DEV/no-containment disclosed where applicable |
| Preview | invited design partners | validate narrow governed workflows and UX | accepted gates only; explicit data/telemetry consent; supported matrix visible |
| Beta | broader approved users | reliability, compatibility, and recovery validation | staged rollout, support and repair runbooks |
| Stable | supported production users | predictable local operation | signed build, migration/rollback, profile/adapter certification evidence |
| Enterprise LTS | future managed organizations | controlled cadence and retention | separately authorized policy/package/identity matrix and extended support |

## Versioning and compatibility

The desktop shell, Core, domain artifacts, event vocabulary, WWP, Hermes, skills/hooks, context compiler, LSP/retrieval adapters, memory, public APIs, and provider/model profiles are independently versioned but released as a compatibility-tested set. Semantic versioning applies to public contracts; breaking changes require a new major, coexistence window, migration/reader plan, and explicit risk decision. Internal refactors that preserve contracts do not force external version changes.

Every mission/phase/task/attempt records the relevant component, artifact schema, event vocabulary, policy, skill/hook, context compiler/selection strategy, LSP adapter/server, retrieval/index version, worker, Brain/builder profile, harness manifest, and toolchain versions. Historical evidence remains interpretable after upgrades.

An approved specification is never migrated by silent editing. Upgrades preserve the immutable version, its Outcome Contract, provenance, and completion evidence. Older Core/runtime/adapter combinations reject unsupported versions safely; newer releases retain historical readers/upcasters or documented export/repair paths.

## Release readiness gates

- approved scope and changelog with linked mission, ADR, risk review, and milestone evidence;
- resolved Build Feature Baseline Gate—through retained accepted prerequisites or an approved replacement-foundation plan—and every required H1–H9 gate/ADR for included capability;
- reproducible build, signing, SBOM/provenance, dependency/license/advisory review;
- unit/property/contract/integration/E2E/evaluation/security/accessibility/performance evidence at required thresholds;
- migration, downgrade, backup/restore, derived-index rebuild, and interrupted-upgrade scenarios;
- supported-platform sandbox/permission posture and DEV limitations verified and disclosed;
- effect-firewall, hook, skill, LSP/retrieval, loop, subagent, memory, and non-convergence adversarial evidence where those features ship;
- fixed-fixture outcome-equivalence and provider/profile certification evidence for every supported profile;
- truthful supported-language/adapter/model matrix—language-neutral contracts do not imply universal support;
- telemetry, alerting, support, incident, recovery, and rollback runbooks reviewed;
- no unapproved critical/high security finding, expired exception, unresolved evidence conflict, or false completion claim.

## Staged rollout and monitoring

Releases move from accepted development builds through preview/beta cohorts only when health thresholds hold: startup/recovery, state/ledger integrity, effect uncertainty, task/worker failure, policy denials/false positives, WIP/conflicts, provider/LSP/retrieval errors, context freshness, loop non-progress, subagent isolation, memory contradictions, outcome-equivalence, evidence completeness, resources, and user-reported outcome quality.

Feature flags control behavior but never disable authority, hard security, or evidence gates. Each rollout gate has owner, threshold, halt condition, and rollback/repair path. A model or harness component may be disabled when regression, unsafe-effect, evidence-completeness, or honest-convergence thresholds fail. Cost or speed improvement cannot compensate for a lower acceptance bar.

## Migration and rollback

Operational-store migrations are forward-compatible and resumable where possible. Before migration, the system creates a verified backup and communicates downtime/data impact. After interruption, Core detects incomplete migration and resumes, restores, or enters a supportable read-only recovery state. Destructive reset is not a default.

Derived context, LSP, retrieval, projection, and evaluation indexes are rebuildable from authoritative sources; rollback must not present a stale derived index as current. Memory migration preserves provenance, authority class, scope, freshness, contradiction, and supersession. A certification/ablation result is immutable evidence tied to its exact harness manifest; a later run supersedes rather than overwrites it.

A revoked skill, hook, adapter, profile, or package can be disabled without making historical timelines, evidence, or specifications unreadable. Rollback preserves audit semantics and proposal refs and never silently moves a protected branch.

## Security, disclosure, and customer communication

Release artifacts are signed and verifiable; updater channels validate authenticity and platform-appropriate rollback protection. Vulnerability handling includes intake, triage, scoped mitigation, advisory, communication, remediation release, and evidence-backed lessons.

Release notes distinguish capability, permission/data changes, supported adapters/profiles, known limitations, required actions, migration status, and rollback instructions. The product must not surprise a user with a provider/model, LSP server, parser/index, skill/hook, permission scope, memory behavior, network destination, protected effect, or telemetry/data flow after an update.

## Deferred release capabilities

Autonomous production deployment, open marketplace distribution, remote worker fleets, cloud-first control planes, cross-organization learning, and universal language/model support remain outside H1–H9. None may enter a release by being hidden behind a feature flag; each needs architecture, security, evaluation, migration, and explicit authorization.

See also: [14_Security_Architecture.md](14_Security_Architecture.md), [17_Event_System.md](17_Event_System.md), [20_Risk_Assessment.md](20_Risk_Assessment.md), [22_Milestones.md](22_Milestones.md), [33_Model_Independent_Outcome_Convergence.md](33_Model_Independent_Outcome_Convergence.md), and [34_Harness_Evaluation_Protocol.md](34_Harness_Evaluation_Protocol.md).
