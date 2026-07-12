# 07 — Messenger Agent

## Mandate

Messenger is the sole agent permitted to communicate with a human. It is an interaction and reporting adapter over the control plane, not a privileged executive. Workers never message users directly; Messenger never bypasses policy, directly changes a project, or grants permissions.

## Responsibilities

- Accept new missions, priority changes, goal updates, and decision responses as typed commands.
- Produce mission updates, executive summaries, daily briefings, progress reports, and project-health narratives from durable projections.
- Present decision packets with the minimum context needed to make a strategic choice.
- Maintain channel delivery, consent, thread mapping, notification preferences, and acknowledgment state.
- Keep the organization running while a decision is pending unless the affected dependency actually requires the decision.

## Decision packet contract

A decision packet is a versioned artifact containing:

| Field | Requirement |
| --- | --- |
| Decision | One clear question and permitted options, including “defer” where safe |
| Why now | Triggering event, deadline, and dependent tasks |
| Recommendation | Named owner, rationale, confidence, and dissenting evidence |
| Consequences | Scope, cost, security, schedule, and reversibility impact per option |
| Evidence | Cited artifacts, test/scans/reviews, and known uncertainty |
| Authority | Policy rule and role authorized to decide |
| Resolution | Signed response, timestamp, rationale, and resulting command |

Messenger can collect the response, but the Core validates authorization and policy before changing mission state. A response sent through Telegram, email, or Studio therefore has identical semantic handling.

## Communication model

~~~mermaid
sequenceDiagram
  participant Core as Core / Orchestrator
  participant Msg as Messenger
  participant Channel as Channel Adapter
  participant Human as Human
  Core->>Msg: projection or DecisionRequested event
  Msg->>Msg: summarize, redact, apply preferences
  Msg->>Channel: queued outbound message
  Channel->>Human: report or decision packet
  Human->>Channel: response / new mission / priority change
  Channel->>Msg: normalized inbound intent
  Msg->>Core: authenticated command
  Core->>Core: policy + authorization + event
  Core-->>Msg: outcome projection
~~~

## Interruption policy

Messenger sends a notification when the event meets configured materiality: a required decision, a safety incident, a critical gate failure, a mission completion proposal, an imminent budget/time boundary, or a user-selected digest schedule. Routine worker steps are collected into live views and summaries. A decision in one branch must block only dependent work; research, documentation, verification, or other independent tasks continue.

## Channels and adapters

Initial adapter abstraction supports Studio inbox first, then Telegram, Discord, Slack, WhatsApp, email, and push notifications. Channel adapters provide identity mapping, verification, delivery receipt, rate limiting, thread/conversation mapping, formatting, inbound command parsing, retention, and opt-out semantics. A channel is an untrusted boundary: inbound content is treated as user input, protected against prompt injection, and never interpreted as tool authority.

## Identity and privacy

Every inbound command is bound to an authenticated principal and organization/project scope. Messenger applies data classification and channel policy before sending content; a high-sensitivity artifact may be summarized without details or require Studio-only access. Transcripts are distinct from Knowledge records and follow communication retention policy. No third-party channel receives private project content merely because it is connected.

## Operating modes

| Mode | Messenger behavior |
| --- | --- |
| Manual | Presents every material task/effect for approval; rich progress remains optional |
| Limited Approval | Requests declared gated actions and strategic changes; routine execution reports asynchronously |
| Full Autonomous | Sends summaries and mandatory hard-gate decisions; does not wait for routine action approval |
| Enterprise Policy | Applies centrally defined identity, retention, approval routing, and notification rules |

## Acceptance criteria

- No worker has a user identity, channel credential, or outbound user transport.
- Messenger reports only projections/evidence it is authorized to disclose.
- Inbound messages create auditable commands; they do not mutate state directly.
- A pending decision does not stop unrelated ready work.

See also: [03_System_Architecture.md](03_System_Architecture.md), [12_Workspaces.md](12_Workspaces.md), [13_Mission_Control.md](13_Mission_Control.md), and [18_API_Architecture.md](18_API_Architecture.md).

