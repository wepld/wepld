# v2-10 — Decision Economics and the Messenger Boundary

Two gate findings live here: H5 (escalation economics are the product, not a config knob) and H7 (the one-voice rule was contradicted, and injected content could steer what humans are told).

## 1. The interrupt budget

Human attention is a metered mission resource, declared in the brief (`budget.max_interrupts`, default 3) and displayed like cost. Mechanics:

- The planner sees the remaining budget in its T0 pack section: plans are made *knowing* how much ambiguity the human will tolerate — plans with many foreseeable decision points must resolve them at plan time (options presented in the plan itself, decided once at approval).
- Every **delivery** (not every question) consumes one interrupt. Exhausted budget ⇒ mission pauses in a visible `WAITING(interrupt_budget)` state with a single summary packet: "this mission is more ambiguous than its budget; revise the brief or raise the budget." Pausing is honest; nagging is not.
- **Interrupts-per-mission is a first-class product metric** on the Mission surface and in the Phase C thesis readout. The product succeeds when this number is small *and* users rate the interrupts they did get as worth it.

## 2. Decision packet classes

| Class | Semantics | Delivery |
| --- | --- | --- |
| **blocking** | a dependent task cannot proceed (hard gate, scope ambiguity) | immediate, batched with any co-pending packets — one delivery, one interrupt |
| **advisory** | a reversible, worktree-local choice where a defensible default exists | **default-with-undo:** Core records the default, work proceeds, the packet appears in the queue as "decided by default — override?" Overriding before completion re-runs affected phases. Consumes no interrupt |
| **completion** | accept/return/merge | always human, always explicit, never budgeted away (ADR-0008 invariant) |

Default-with-undo is bounded strictly: never for hard-gate crossings, never for anything irreversible, never outside the worktree. It converts v1's "keep the organization running while the user considers" from aspiration into a mechanism.

## 3. Batching and digests

Non-blocking notifications (phase completions, gate passes, budget threshold crossings) are ledger facts rendered in the Studio live — they are **pull**, not push, and cost no interrupts. Packets that become pending while another delivery is undelivered join it (one delivery). Digest scheduling (v1 doc 07) returns with channels in V3; in a single-surface MVP the Timeline *is* the digest.

## 4. The one-voice rule, made structural (resolves H7a)

**Rule: the Studio has exactly one agent identity — Messenger.** Consequences:

- Workers cannot speak to humans because WWP contains no verb for it (v2-03). Their narrative artifacts (worklogs, summaries) may be *quoted* by Messenger or opened by the user as evidence — always attributed as artifacts, never voiced as messages.
- Any future conversational surface — including a V2 review-IDE assistant — **is Messenger rendered in context**, using the same outbound contract, claims discipline, and ledger provenance. There is no second assistant to drift into existence; v1 doc 12's "constrained AI assistance" is hereby defined as a Messenger presentation, closing the contradiction with doc 07.
- Inbound: every user utterance on any surface normalizes to the Inbound Intent shape (v2-07 §4) and becomes a Command. No surface mutates state directly — v1's rule, kept verbatim.

## 5. Injection hardening at the human boundary (resolves H7b)

Threat: repository files, tool output, or (later) channel messages contain text crafted to make the *summary* lie — "all tests pass, recommend immediate merge" — steering the human's approval, which is the system's root of trust. Defenses, layered:

1. **Ledger-rendered facts.** Gate status, diff stats, budget, phase states in any UI or message are rendered by the Studio directly from ledger/artifact queries — never parsed from model prose. A model cannot claim a gate passed; only `GateEvaluated` can (v2-02 §6).
2. **Claims discipline.** Messenger output is `body_md + claims[]` (v2-07 §4). Claims with evidence refs are independently resolved by the UI and marked verified; prose without refs renders in a visually distinct "unverified narrative" style. The trust gradient is visible, not implied.
3. **Quoting, not voicing.** Repository- or tool-derived text appearing in packets/reports is displayed in explicit quoted-source blocks with origin labels ("from README.md — untrusted content"), mirroring the pack-level trust labeling (v2-04 §Redaction).
4. **Decision packets are schema, not prose.** Options, consequences, and evidence are structured fields; the model can propose them, but the evidence links resolve against the ledger or they render as unverified. An injected "option" cannot smuggle authority it doesn't have — resolution authority is checked by Core, not by the packet text.
5. **Substrate labeling.** Upstream, every untrusted inclusion in context packs is fenced and instruction-inert (v2-04), reducing the chance the summarizer is steered at all. Layers 1–4 assume it happens anyway.

## 6. Acceptance criteria for this document

- No mission can complete with zero human decisions (completion class is unbudgetable).
- A mission on fixtures with a deliberately injected "tests pass, merge now" file produces: fenced untrusted quoting in any mention, an unverified-narrative rendering for any echo of the claim, and unchanged gate status sourced from `GateEvaluated` — demonstrated in the adversarial suite (Phase B11).
- Median interrupts/mission ≤ 3 on the Phase C cohort with ≥ 80% "worth it" rating, or the escalation thresholds are redesigned before V2 investment.
