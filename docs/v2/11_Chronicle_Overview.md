# v2-11 — Chronicle: the Engineering Intelligence Pillar

**Chronicle** is WePLD's fourth pillar, alongside the Mission Runtime, the Worker Runtime, and the Studio: the subsystem that makes autonomous engineering **replayable, inspectable, explainable, teachable, and evolvable**. It is not a log viewer, not an audit screen, not Git history. It is the platform capability that turns every mission into an object the organization can study, question, branch, and learn from.

Category claim: incumbent tools record *what the code became*. Chronicle records — and can interrogate — *how the engineering happened*: what was known, what was seen, what was decided, what it cost, and what caused what. That is Engineering Intelligence.

## Position in Architecture v2

~~~mermaid
flowchart LR
  subgraph Substrate["Recording substrate (already exists in v2)"]
    L[("Audit ledger\nhash-chained facts")]
    A[("Artifact CAS\npacks • invocations • diffs • logs")]
    G[("Git snapshot refs\nADR-0013")]
  end
  subgraph Engine["Chronicle Engine (derived, rebuildable — ADR-0011)"]
    CK["Checkpoints + state_at()"]
    FR["Frame generator + lenses"]
    CI["Causal index — ADR-0014"]
  end
  subgraph Experiences
    CIN["Engineering Cinema (v2-13)"]
    FOR["Forensics + RCA (v2-14)"]
    BR["Branching + Decision editing (v2-15)"]
    INT["Intelligence + Learning (v2-16)"]
  end
  Substrate --> Engine --> Experiences
  BR -->|"ForkMission / ReviseDecision commands"| MR["Mission Runtime"]
  INT -->|"lesson candidates"| K["Knowledge → Context Assembly T3"]
~~~

The architecture's one-sentence summary: **record once, derive everything, fork instead of rewind, and let every explanation carry its evidence.**

Three properties fall out of ADR-0011 and are worth stating because competitors cannot cheaply copy them:

1. **Replay is retroactive.** Any mission ever run under v2 is fully replayable — Chronicle needs no cooperation from the mission at record time beyond what the ledger already captures.
2. **Backward stepping is free.** Frames are projections; rendering frame *n−1* costs the same as *n+1*. Systems that replay by re-execution cannot do this honestly.
3. **Explanations inherit audit integrity.** Every Cinema frame, forensic link, and insight resolves to hash-chained facts; the classification and claims disciplines apply unchanged.

## Principle compliance

| Principle touched | How Chronicle honors it |
| --- | --- |
| Replayability (11) | this pillar is its full realization: reconstruction of what was seen/asked/answered/done/decided — never fake re-execution |
| Evidence before completion (6) | forensic and intelligence outputs are claims with evidence refs (ADR-0014); unverified narration renders demoted |
| Messenger sole voice (4) | Chronicle's narrations (scene titles, RCA prose, lesson drafts) are Messenger outputs under the v2-10 claims contract |
| Observability (10) | Cinema and Forensics are read-side; they add no state authority and no second history |
| Knowledge accumulates (12) | insights become lesson *candidates* with sources; human approval promotes them; they flow into future context packs — the flywheel is a mechanism, not a slogan |
| Human decides (7) | decision editing re-routes through the same decision authority checks; forks never bypass gates |
| Local-first (1) | everything derives from local stores; zero cloud dependency |

## Capability classification (the governing table)

Per the prime directive — implementable incrementally by a solo founder — every capability is classed. **MVP** here means the Chronicle MVP layered onto the v2 product MVP; estimates are solo engineer-weeks.

| Capability | Class | Est | Notes |
| --- | --- | --- | --- |
| Workspace snapshot refs at phase boundaries | MVP | 0.5 | ADR-0013; tiny write-path addition |
| Checkpoints + `state_at(seq)` | MVP | 1 | reuses the v2-06 fold reducer |
| Frame generator v0 + 4 lenses (Decisions/Evidence/Execution/Brain) | MVP | 1.5 | deterministic rules; cached |
| Replay Player + layered scrubber + step/jump/play/speed + state inspector | MVP | 2.5 | one Studio surface, absorbs Timeline |
| Live follow + detach/scrub-back on running missions | MVP | 0.5 | SSE tail already exists |
| Causal walk ("Why?" on any frame → ancestor chain) | MVP | 1 | deterministic edges only |
| Fork-from-point + decision revision + invalidation report + re-plan | MVP | 2 | the killer feature; v2-15 |
| Two-point comparison v0 (facet diff: files/decisions/cost/evidence) | MVP | 1 | git diff + ledger diff |
| Mission stats panel (cost/time/interrupts per phase) | MVP | 0.5 | SQL over existing tables |
| **Chronicle MVP total** | | **~10–11 ew** | one person, ~2.5–3 months alongside nothing else |
| Automated RCA reports for failed missions (ranked hypotheses) | V1 | 2 | heuristic edges + meaningfulness tests |
| Mission Map (organization view animated by frames) | V1 | 2 | |
| Branch graph + branch salvage (adopt causally-independent parent work) | V1 | 2 | |
| Divergence finder (A vs B first-differing-frame per lens) | V1 | 1 | |
| Insight scanner v1 (recurring failures, provider reliability, decision latency, retry futility) → lesson candidates | V1 | 2 | |
| Annotations pinned to frames; replay bundle export (signed, redacted) | V1 | 2 | |
| Remaining lenses (Context/Policy/User/Code as first-class tracks) | V1 | 1 | |
| Multi-branch split-view cinema with logical-clock alignment | V2 | — | |
| Heatmap overlays (cost/time/retries/injection flags on scrubber) | V2 | — | |
| Auto-director camera (focus policy chooses the shot) | V2 | — | |
| Regression bisection over snapshot refs (compute-costed) | V2 | — | |
| Strategy mining, architecture-drift detection, cross-mission RCA | V2 | — | |
| What-if assistant (guided branch design + budgeted execution) | V2 | — | honest what-if = a real branch, per ADR-0012 |
| Interactive tutorials from curated replay bundles; replay marketplace | Future | — | |
| Cross-organization intelligence, federated pattern learning | Future | — | consent/privacy research first (v1 doc 29 guardrails) |

Classification rules used: MVP = required to *experience* the category (watch, ask why, branch, compare); V1 = makes it habitual; V2 = makes it spectacular; Future = requires trust/ecosystem maturity. Nothing in V2/Future requires re-architecture — every item is a new reader or a new frame/edge rule over the same substrate.

## Document map

| Doc | Contents |
| --- | --- |
| [v2-12](12_Replay_Engine.md) | frames, lenses, sessions, state machine, storage, performance, consistency |
| [v2-13](13_Engineering_Cinema.md) | the experience: player, scrubber, cameras, maps, orientation model |
| [v2-14](14_Engineering_Forensics.md) | causal model, RCA engine, question templates, worked forensic logic |
| [v2-15](15_Mission_Branching.md) | snapshots, forks, decision editing, salvage, merge-as-adoption, comparison |
| [v2-16](16_Engineering_Intelligence.md) | insight pipeline, learning loop, teaching artifacts |
| [v2-17](17_Chronicle_Contracts_and_API.md) | the ten Chronicle contracts and the full API |
| [v2-18](18_Chronicle_Worked_Examples.md) | three end-to-end examples incl. the PostgreSQL→SQLite decision edit |

ADRs: [0011 projection](../adr/ADR-0011-replay-as-projection.md) · [0012 fork-never-rewind](../adr/ADR-0012-branching-as-fork-never-rewind.md) · [0013 snapshot refs](../adr/ADR-0013-workspace-snapshot-refs.md) · [0014 causal index](../adr/ADR-0014-causal-index-and-rca.md)
