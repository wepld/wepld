# WePLD Studio and User Experience

**Standing:** planning only; no UI implementation is authorized.

## Strategy

WePLD Studio begins as **Engineering Mission Control**, not a full IDE and not
a VS Code clone. The founding bet: the governed engineer's scarce resource is
**decision attention**, not keystrokes. Studio therefore optimizes for
material decisions, evidence inspection, and honest state — every surface is a
projection of Core truth, owns no state, and issues only authenticated Core
commands. Editor breadth is deliberately deferred; if users need an editor,
they keep their editor, and ACP-class interop is a Stage 9 question.

Primary user and initial wedge: the solo senior engineer or technical founder
delivering bounded outcomes on one repository who wants provable results from
AI capacity without surrendering authority. Teams, security reviewers, and
enterprises follow the same surfaces with more roles.

## Two evaluated entry modes (product hypothesis)

The approved native V0 is not replaced. Two entry modes are defined for
evaluation, and a founder decision is required before the imported mode may
precede or replace native V0 in commercial sequencing:

**Native Delivery Entry** — WePLD performs specification → qualified plan →
bounded build → verification → Consulting → completion decision.

**Imported Change Assurance Entry** — an external human or coding agent
supplies a change; WePLD performs import → scope reconstruction → claim
extraction → Verification Lab → independent Consulting → Change Passport →
explicit completion recommendation. The imported mode grants no trust to the
producing agent; requires an exact base and diff; treats producer metadata as
untrusted provenance; cannot silently infer missing acceptance criteria (it
may stop `MoreSpecificationRequired`); and never merges automatically.
Evaluated as EV-S14.

## Surface inventory (requirements staging; product GUI gated at H9)

The Stage column below names the stage whose exit produces the surface's
**requirements and typed query foundations**. Actual Studio product GUI
surfaces are admitted only at the canonical H9 gate (document 22: H8 closed,
stable Core contracts, approved surface scope) and land with Stage 8. Before
that, every workflow below is served by CLI commands, structured JSON output,
generated reports, and typed query APIs — capability classes the canonical
milestones already permit.

| Surface | Requirements stage | Notes |
| --- | --- | --- |
| Mission dashboard | 5 | the cockpit spine |
| Outcome + specification views | 5 | review/approve journeys |
| Plan qualification view | 5 | proposal vs assessment vs decision |
| Decision Inbox | 5 | material decisions only; expiry visible |
| Phases + Kanban view | 5 | Core-enforced states, read/decide |
| Evidence Viewer | 5 | claims ↔ evidence ↔ freshness |
| Diff viewer (controlled) | 5 | proposal refs, never live mutation |
| Worktree inspector | 5 | leases, isolation, orphans |
| Flight Recorder timeline | 5 | ledger projection |
| Recovery Room | 5 | guided reconciliation |
| Data-egress preview | 5 | pre-execution provider disclosure (PR #3 §8 pattern) |
| Provider settings | 5 | profiles, assurance tiers, budgets |
| Cost dashboard | 5 | budgets, spend, exhaustion states |
| Policy Studio | 5 | author policy; simulation before activation |
| Architecture view | 5–6b | Constitution rules + drift findings |
| Agent Hive view | 6b | roles, leases, budgets — status, not chat voyeurism |
| Committee Room | 6b | sessions, dispositions, dissent (projected per PR #3) |
| Verification Lab surface | 6b | gaps, coverage, inspector results |
| Truth Graph explorer + Ask Why | 6b | evidence-linked answers |
| Project DNA explorer | 6b | identity + conventions |
| Repository + symbol maps | 6b | knowledge navigation |
| Test explorer | 6b | integrate runners' results |
| Security findings | 6b | SARIF-normalized |
| Skill + memory inspectors | 6b | provenance, judgments, candidates |
| Controlled terminal | 6b | capability-scoped, recorded, never ambient |
| Technical-debt view | 7 | ledger + economics |
| Migration viewer | 7 | Migration Lab projection |
| Dependency viewer | 7 | supply-chain guardian projection |
| Release view | 7 | Change Passports, guardians, rollout |
| Digital Twin explorer | 7 | clearly labeled as derived |
| Stakeholder views | 8 | audience-scoped truth projections |

Rejection rule (applies to every surface): a Studio surface that proves less
useful than an existing integration is dropped — measured in EV-S16, not
argued.

## Trust adoption and the Autonomy Ladder

Autonomy is **earned, scoped, expiring, and reversible** — never a switch.

| Level | Name | Meaning |
| --- | --- | --- |
| 0 | Observe | read-only analysis; Shadow Mode lives here |
| 1 | Recommend | advisory findings and proposals only |
| 2 | Prepare artifacts | draft specs/plans/changes; nothing executes |
| 3 | Execute after every approval | the V0 posture |
| 4 | Execute admitted low-risk tasks | per task-class admission, evidence-based |
| 5 | Run bounded approved phases | phase-scoped, budgeted, interruptible |
| 6b | Governed programme autonomy | Stage 9; policy-admitted, never universal |

Rules: promotion requires evidence (accepted-task history, correction rates)
per capability and risk class — evidence is produced independently of the
promoted party, and **no self-promotion** exists in any form; rollback and
automatic downgrade are immediate on material regression; every grant expires
and is re-evaluated after any policy, model, or project change; autonomy is
scoped by project, capability, effect and risk tier and never leaks between
projects; a capability lease can never exceed the admitted autonomy tier;
human override always works; every promotion, downgrade, and override is a
full audit record; **no universal full-autonomy switch exists** at any level —
level 6 is still bounded by policy, budgets, and the Effect Firewall.
`HumanAttentionBudget` bundling can never hide an authorization request:
its never-suppress categories (security authorization, irreversible effects,
financial commitments, data-egress approval, plan approval, completion
approval, policy exceptions, capability escalation, production release
authorization) are invariant. Shadow Mode runs the whole method silently against
real work to build calibration evidence; its false-warning rate is its own
rejection criterion.

## Journey sketch (the canonical loop)

Connect repository + runtime → describe outcome → resolve material
clarifications → approve specification → approve qualified plan → observe
bounded execution (Mission Control, not transcripts) → decide on the
evidence-backed completion proposal → retained intelligence (memory/skill
candidates) awaits judgment. Variants for security reviewers (findings-first),
teams (role-scoped inboxes), enterprises (policy + audit projections), and
incidents (Recovery Room first) reuse the same projections.
