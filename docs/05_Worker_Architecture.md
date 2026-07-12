# 05 — Worker Architecture

## Definition

A worker is an executable organizational role, not a language model. It is composed of:

**role policy + selected brain profile + resolved skills + scoped tool capabilities + durable task lease + isolated workspace.**

This composition allows Hermes-compatible workers first while keeping future worker runtimes interchangeable. A worker may reason through a Brain Gateway, but the worker host is the only component that can request approved execution effects.

## Worker taxonomy

The initial registry defines these role families: Executive, Planner, Builder, Reviewer, QA, Debugger, Architecture, Python, Rust, Frontend, Backend, Database, Security, DevOps, Documentation, Performance, Research, Legal, Compliance, Accessibility, UX, Testing, Benchmark, and Release Manager. A role is a declarative profile, not necessarily a separate binary.

| Family | Primary output | Default authority |
| --- | --- | --- |
| Executive / Planner | mission brief, decomposition, risk and decision proposals | read project and knowledge; no mutation |
| Builder / specialist | isolated change set and implementation evidence | scoped worktree and approved tools |
| Reviewer / QA / Test | findings, gate result, reproducible evidence | read artifact/worktree; test sandbox |
| Security / Compliance / Legal | risk classification, policy findings, exception proposal | read-only unless a remediation task is separately leased |
| DevOps / Release | deployment or release proposal and evidence | no production effect without protected gate |
| Research / Documentation / UX | cited recommendations and documentation artifacts | read-only by default |

## Worker contract

Every adapter must support the following conceptual lifecycle:

1. **Advertise:** register runtime version, role compatibility, tool/sandbox support, resource limits, and health.
2. **Lease:** accept a task only with a signed, expiring task lease, declared artifact inputs, capability token, quality gates, and cancellation channel.
3. **Plan actions:** emit structured reasoning result and proposed actions; actions are not effects.
4. **Execute through mediation:** invoke the Tool Executor with an approved capability and idempotency key.
5. **Report artifacts:** attach hashes, logs, measurements, citations, and uncertainties.
6. **Finish or fail:** emit a terminal outcome, clean up sandbox resources, and relinquish the lease.

The normalizer maps Hermes-compatible protocol messages into this contract. A future worker adapter must conform to the same lease, action, artifact, and outcome semantics.

## Lifecycle and recovery

~~~mermaid
stateDiagram-v2
  [*] --> Registered
  Registered --> Eligible: compatible role + policy
  Eligible --> Leased: scheduler assigns task
  Leased --> Running: heartbeat accepted
  Running --> AwaitingPolicy: action requires decision
  AwaitingPolicy --> Running: permitted
  Running --> Succeeded: artifacts + required evidence
  Running --> Failed: classified error
  Running --> Cancelled: cancellation acknowledged
  Leased --> Expired: heartbeat timeout
  Expired --> Eligible: safe retry or reassignment
  Succeeded --> [*]
  Failed --> [*]
  Cancelled --> [*]
~~~

Leases are time-bound; workers heartbeat while performing effects. A lost worker is never assumed to have failed cleanly. The Orchestrator records an uncertain attempt, inspects observable artifact/tool state, then retries, reassigns, or escalates according to idempotency and risk classification.

## Isolation and capability model

Each task attempt receives an isolated Git worktree or equivalent project snapshot, an explicit filesystem allowlist, CPU/memory/time quota, network policy, and only the secrets necessary for the task. Untrusted code runs without network by default, never sees the user home directory, and cannot write the primary worktree. Platform-specific sandbox limitations are surfaced to the user and policy engine; they are not silently treated as equivalent protection.

## Scheduling

The Scheduler selects workers by role compatibility, required skills, permitted data classification, tool support, locality, health, resource capacity, historical reliability, budget, and declared priority. It favors bounded parallelism: independent tasks may run concurrently, but changes to shared artifacts require dependency ordering or separate worktrees followed by reviewable merge.

## Completion and escalation

A worker cannot set a mission complete. It can only report a task outcome. It must escalate through Orchestration for scope ambiguity, permissions, new dependencies, confidential data exposure, budget breach, repeated failure, contradictory evidence, protected branch merge, deployment, or any action classified as strategic.

## Acceptance criteria

- Worker adapters are replaceable without changing task-domain semantics.
- Every tool effect has a worker, task attempt, policy decision, capability token, and artifact/log reference.
- No worker has a direct user channel or direct mutable access to mission state.
- A killed worker can be recovered without silently duplicating a non-idempotent action.

See also: [06_Brain_Architecture.md](06_Brain_Architecture.md), [10_Loop_Engineering.md](10_Loop_Engineering.md), [14_Security_Architecture.md](14_Security_Architecture.md), and [17_Event_System.md](17_Event_System.md).

