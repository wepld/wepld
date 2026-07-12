# 13 — Mission Control

## Purpose

Mission Control is the live operational dashboard for WePLD’s engineering organization. It is not a decorative agent monitor. It derives from durable events, current leases, quality evidence, resource telemetry, and policy state to answer: what is running, what is at risk, what is waiting, and what requires executive attention?

## Required views

| View | Required signals | Interpretation rule |
| --- | --- | --- |
| Current mission | outcome, scope, phase, autonomy mode, owner, budget | mission state is the Orchestrator projection |
| Worker board | running/idle/blocked/lost workers, role, task, heartbeat, resources | “running” requires an active lease and recent heartbeat |
| Task graph | dependencies, readiness, critical path, retries, gate state | graph is versioned and links to planning artifact |
| Quality | build, tests, coverage, review, benchmark, security/accessibility evidence | status is per gate, never a single unverifiable score |
| Resource / cost | CPU, memory, disk, provider usage, spend vs budget | distinguish measured, estimated, and unavailable values |
| Decisions | required decisions, authority, deadline, blocked dependents | a decision has an evidence-linked packet |
| Timeline | state changes, actions, outputs, causal links | uses immutable event references |
| Project health | delivery, quality, security, resource, knowledge freshness | shows inputs and confidence, not a magical number |

## Health model

Project health is a transparent vector rather than a single opaque score:

- **Delivery:** task critical-path progress, blocked duration, schedule/budget variance.
- **Quality:** mandatory gate status, trend, test reliability, review backlog.
- **Security:** open finding severity, policy exceptions, dependency/secret/supply-chain status.
- **Operations:** worker health, sandbox integrity, CPU/memory/disk pressure, provider availability.
- **Knowledge:** key decisions and architecture records with current evidence/freshness.

An optional summary status is the worst applicable dimension with an explicit rationale. A green summary is impossible if a required high-severity security finding or decision deadline is unresolved.

## Alert policy

Alerts are classified by materiality and routed to the appropriate workspace/Messenger policy. Examples: a worker heartbeat delay creates an operations alert; a lost task on the critical path creates a delivery alert; a blocked production deployment creates a decision alert; secret detection creates an immediate security alert. Deduplication, escalation, acknowledgment, suppression reason, and resolution are all recorded.

## Data freshness and correctness

Each widget displays last event time, last projection update, and source availability. “No data” is never rendered as “healthy.” Telemetry ingestion may be eventually consistent, but mission/task state uses the durable Core ledger. If a projection rebuilds after a crash, it must reproduce the same health inputs from events and current evidence references.

## Control actions

Mission Control exposes only explicit, policy-evaluated commands: pause/resume/cancel a mission, reprioritize, request a report, acknowledge an alert, set a budget guardrail, or open a decision. It cannot directly terminate a worker process, delete an artifact, or bypass a release gate; those actions use their own policy-controlled path.

## Operational acceptance criteria

- Every displayed material status links to a current projection and its evidence inputs.
- A user can explain why a task is blocked and identify its blocking dependency or decision.
- Resource/cost data differentiates local measurement from provider estimate.
- Losing the Studio does not stop the Core, and reconnecting never loses the authoritative task state.

See also: [03_System_Architecture.md](03_System_Architecture.md), [07_Messenger_Agent.md](07_Messenger_Agent.md), [10_Loop_Engineering.md](10_Loop_Engineering.md), and [27_Performance_Goals.md](27_Performance_Goals.md).

