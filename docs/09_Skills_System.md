# 09 — Skills System

## Purpose

A skill packages repeatable engineering expertise so workers do not rely only on an unversioned prompt. Skills are independently versioned, testable, discoverable, removable capabilities that add structured knowledge and methods to an authorized role.

## Package model

Every skill package contains the following declarative parts:

| Part | Purpose |
| --- | --- |
| Manifest | identity, semantic version, publisher, compatibility, license, integrity hash, release channel |
| Capability declaration | tools, data classes, network, workspace paths, and human/enterprise approval requirements |
| Knowledge | curated instructions, references, domain constraints, and citations |
| Templates | task briefs, checklists, reports, artifacts, and decision packet forms |
| Validation rules | schemas, lint/check commands, quality thresholds, safety rules |
| Examples | representative inputs, expected outputs, and negative cases |
| Documentation | intended roles, limits, risks, setup, and deprecation guidance |
| Tests / evaluations | deterministic tests and evaluation fixtures that prove the claimed behavior |

The package has no ambient access. A manifest declares what it wishes to use; the Policy Engine grants a subset for a particular project, worker, and task.

## Resolution and runtime

At task lease time, the Worker Registry resolves a pinned skill set compatible with the worker role, project policy, data classification, toolchain, and task schema. It records exact package versions and hashes in the task attempt. A skill may guide reasoning, validation, templates, and approved tool invocation, but it cannot replace task policy or persist hidden state outside declared artifacts.

## Lifecycle

~~~mermaid
stateDiagram-v2
  [*] --> Draft
  Draft --> Candidate: packaged + signed by publisher
  Candidate --> Validated: security, compatibility, evaluation pass
  Validated --> Approved: policy / curator approval
  Approved --> Published: registry channel
  Published --> Deprecated: successor or risk notice
  Published --> Revoked: critical security/policy issue
  Deprecated --> [*]
  Revoked --> [*]
~~~

## Skill evolution

A worker can propose a skill improvement only as a candidate package with linked mission evidence: task outcome, before/after evaluation fixtures, expected benefit, risk, and compatibility impact. The proposal goes through automated validation, security and license checks, evaluation comparison, and designated human or enterprise approval. Successful project work alone never silently alters a shared skill.

## Registry and marketplace

V1 provides a local registry and approved-package catalog. Marketplace support is a later distribution layer over the same package contract. Registry entries require publisher identity, signing/provenance, package hash, compatibility range, changelog, permissions, license, evaluation summary, support/deprecation status, and vulnerability/revocation metadata. Installation is a policy-governed transaction; removal checks dependent worker profiles and preserves historical reproducibility through retained package metadata.

## Trust tiers

| Tier | Source | Default treatment |
| --- | --- | --- |
| Core | WePLD-maintained | reviewed and signed; still capability-limited |
| Organization | approved internal publisher | enterprise policy and review |
| Project-local | scoped to one repository | no automatic cross-project publishing |
| Community | marketplace publisher | quarantined review, explicit permissions, evaluations required |

## Acceptance criteria

- A completed task can identify the exact skills and versions that informed it.
- A revoked skill cannot be newly selected and causes a visible compatibility/risk review for active work.
- A worker cannot install, publish, or self-upgrade a skill outside the validation workflow.
- Removing a skill does not erase historical audit metadata.

See also: [05_Worker_Architecture.md](05_Worker_Architecture.md), [14_Security_Architecture.md](14_Security_Architecture.md), [15_Plugin_System.md](15_Plugin_System.md), and [26_Testing_Strategy.md](26_Testing_Strategy.md).

