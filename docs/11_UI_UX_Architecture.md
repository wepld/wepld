# 11 — UI/UX Architecture

## Design thesis

WePLD should feel like entering an engineering studio: calm, legible, and operationally credible. It must make a complex autonomous organization understandable without turning every worker thought into a chat feed. The interface privileges mission outcomes, evidence, risk, and decisions over token streams or editor chrome.

## Information architecture

The top-level navigation is organized around executive intent rather than technical implementation:

| Area | User question it answers |
| --- | --- |
| Home / Mission Control | What needs attention and how healthy is the organization? |
| Missions | What outcomes are underway, planned, blocked, or complete? |
| Executive | Are roadmap, risk, cost, and capacity aligned with strategy? |
| Architecture | What system shape and dependency impact does this work have? |
| Timeline | What happened, why, and what evidence supports it? |
| Knowledge | What has the organization learned and can it be trusted? |
| IDE | What does the project contain and what change is under review? |
| Settings | Which brains, workers, skills, plugins, policy, and integrations are approved? |

Workspace views are projections over the same Core data; no screen owns a separate workflow state.

## Primary interaction patterns

- **Mission brief:** a structured outcome, scope boundary, acceptance criteria, autonomy mode, budget, data classification, and priority—not an unbounded chat prompt.
- **Decision queue:** compact, evidence-linked decision packets with an explicit owner and deadline; users can compare options without navigating raw logs.
- **Progress narrative:** Messenger-generated summaries cite task, artifact, and gate state; users can drill down to the timeline.
- **Evidence drawer:** any material claim exposes its source artifact, freshness, actor, validation, and policy outcome.
- **Plan before action:** users see a proposed task graph and capability/risk envelope before a mission begins where policy requires it.
- **Safe intervention:** pause, cancel, reprioritize, or constrain a mission through auditable commands with clear blast radius.

## Live-state design

The Studio subscribes to versioned query projections and event deltas from the Core Daemon. It renders an explicit synchronization state: live, delayed, offline snapshot, or reconnecting. The UI never invents progress from an optimistic client action; it shows command submitted, accepted/rejected, and durable state transition separately.

For high-volume data, the Studio renders summaries first and progressively loads logs/artifacts. It collapses routine worker activity into meaningful milestones, while preserving a forensic Timeline for users who need it.

## Trust and explainability

Autonomous systems earn trust through visible boundaries. Every consequential UI affordance must expose:

| Display | Example |
| --- | --- |
| Actor and role | “QA worker, v1.4 profile” |
| Authority | “Allowed by Limited Approval / project policy” |
| Effect scope | “Writes only isolated worktree for task T-42” |
| Evidence | “Targeted tests passed; review pending” |
| Uncertainty | “Benchmark unavailable; completion blocked” |
| Time | Start, last heartbeat, event time, and staleness |

The product must avoid anthropomorphic certainty. A confident sentence without traceable evidence receives a visual uncertainty state, not an executive-looking green badge.

## Accessibility and inclusive design

The Studio targets keyboard-complete operation, semantic structure, visible focus, scalable typography, high-contrast themes, screen-reader labels for dynamic status, non-color-only risk indicators, reduced-motion support, localization-ready strings, and time-zone-aware timelines. Live announcements are batched and user-configurable so screen readers are not flooded by worker churn. Accessibility checks are a required UX quality gate, not late visual polish.

## Error and degraded-mode UX

If a provider, worker, sandbox, integration, or index is unavailable, the interface shows the affected capability, consequences, fallback state, and next safe action. It never suggests a task completed because its reporting channel failed. Offline mode preserves cached read projections, accepts locally valid commands into a durable queue where safe, and marks any state that cannot be confirmed.

## Design system boundaries

A future design system supplies semantic tokens for risk, policy, quality, freshness, and execution state; reusable cards for missions, workers, decisions, and evidence; and a layout grammar shared across workspaces. Theme plugins may change presentation but cannot hide mandatory safety disclosure or alter status semantics.

See also: [12_Workspaces.md](12_Workspaces.md), [13_Mission_Control.md](13_Mission_Control.md), [17_Event_System.md](17_Event_System.md), and [26_Testing_Strategy.md](26_Testing_Strategy.md).

