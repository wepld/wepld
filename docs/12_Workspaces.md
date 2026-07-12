# 12 — Workspaces

## Principle

Workspaces are tailored lenses over the same mission, evidence, and policy state. The IDE is important but deliberately not the product center. A person can lead an engineering portfolio in WePLD without being forced to read source code.

## Workspace catalog

| Workspace | Audience | Primary content | Allowed actions |
| --- | --- | --- | --- |
| Mission Control | everyone | operational health, running work, alerts, decisions | inspect, triage, approved interventions |
| Executive | executive / leader | roadmap, outcomes, risk, portfolio metrics, reports | set goals/priorities, approve strategy |
| Mission | mission owner | brief, plan, graph, artifacts, acceptance matrix | scope, pause, cancel, provide decisions |
| Architecture | architect / engineer | services, modules, APIs, data flow, dependencies, ADRs | propose/review architecture changes |
| Timeline | auditor / engineer | causally linked events, commits, tests, decisions, deployments | inspect, filter, export permitted evidence |
| Knowledge | all authorized roles | decisions, patterns, lessons, findings, sources | search, correct, propose records |
| IDE | engineers | editor, explorer, terminal, Git, diff, agent status | review/work directly under policy |
| Settings / Registry | admin | policy, brains, workers, skills, plugins, integrations | configure approved capabilities |

## Executive Workspace

The Executive Workspace shows no code by default. It focuses on project health, roadmap confidence, current mission outcomes, progress trends, business and delivery risks, architecture impact, engineering metrics, decision queue, and digest reports. Drill-down routes into evidence or a mission context rather than dumping raw worker logs. It answers “Should we change direction or remove a constraint?” rather than “Which file changed?”

## IDE Workspace

The IDE includes code editor, explorer, terminal, Git, Timeline, Knowledge, Memory, agent status, and constrained AI assistance. It is a review and focused intervention environment. Autonomous changes appear in isolated worktrees/diffs with a provenance banner; the default is not to write a user’s primary worktree. Terminal execution is capability-mediated and visibly attributed to a user or worker task.

## Architecture Workspace

The Architecture Workspace renders the declared and observed system: services, modules, dependencies, APIs, data flows, infrastructure, knowledge links, policy boundaries, and change impact. It distinguishes intended architecture from observed evidence and flags undocumented dependencies. It links each important edge to the latest artifact, decision, and validation status.

## Timeline Workspace

The Timeline is a causally navigable engineering record. It shows who or what changed what, why, decision links, reviews, tests, benchmarks, commits, deployments, and current status. Events retain correlation/causation identifiers, so a user can travel from an executive decision to a worker lease, tool effect, test result, merge proposal, and release outcome without reconstructing a chat history.

## Workspace permissions and data handling

The Core evaluates authorization and classification before sending a projection. Workspaces do not receive hidden fields then merely conceal them in the client. Sensitive raw logs, secret-related findings, or provider traces may be summarized in Executive and Messenger views while remaining available only to authorized security roles. Export actions create an auditable artifact and respect retention and sharing policy.

## Cross-workspace continuity

Context follows stable links: a Mission Control alert opens the mission’s relevant task; a decision opens its evidence; a diff opens the associated review and tests; a knowledge claim opens its source and supersession chain. Navigation preserves filters and time range but does not create a second mutable copy of the data.

## V1 delivery

V1 prioritizes Mission Control, Mission, Timeline, a minimal Executive view, and a review-oriented IDE surface. Architecture visualizations, richer Knowledge browsing, and advanced editor parity evolve after the control-plane vertical slice is reliable.

See also: [07_Messenger_Agent.md](07_Messenger_Agent.md), [11_UI_UX_Architecture.md](11_UI_UX_Architecture.md), [13_Mission_Control.md](13_Mission_Control.md), and [29_Future_Vision.md](29_Future_Vision.md).

