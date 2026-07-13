# IMPL-08 — Solo Founder Execution Guide

The program's constitution for daily work. Short on purpose: rules you can actually hold in your head.

## Weekly rhythm

- **Mon–Thu:** build. One milestone in flight at a time, ever. Small PRs against `main` — yes, PRs while solo: CI is your reviewer, the PR description is your engineering log, and the golden-trace diff is your design review.
- **Friday:** demo + housekeeping. Record the demo (2–5 min screen capture, every week, even when it's boring — *especially* when it's boring); update `DECISIONS.md`; triage the backlog file; tick the milestone checklist.
- **Never:** more than 2 days without something merged; more than 1 week without something demoable. If either rule is about to break, the work item is too big — split it until it isn't.

## PR discipline (the per-PR checklist)

Every PR: ☐ builds green with all goldens ☐ boundary check passes (no new dependency edges without an IMPL-02 edit in the same PR) ☐ contracts untouched *or* version bumped with changelog line ☐ no new event types without vocabulary-lock update + reference to the authorizing document ☐ no TODOs without a backlog entry ☐ description says which milestone line-item this advances. Target size: reviewable by a tired stranger in 15 minutes. If a PR needs an essay, it needed to be three PRs.

## Freeze discipline

The architecture is frozen. When something *feels* missing: (1) write a **gap note** in `docs/impl/gap-notes/` — what you need, which v2 mechanisms you tried, why each fails; (2) sleep on it; (3) 90% of the time the morning answer is an existing mechanism (the ledger, an envelope, a command, a lens, an insight rule); (4) if the gap survives, it becomes an IADR if implementation-level — and if it's genuinely architectural, it *waits for the post-preview architecture review* unless it blocks the thesis demo itself. Nothing has that severity in the current plan.

Ideas are not enemies — they go to `BACKLOG.md` with one line each. The Friday triage keeps the graveyard honest.

## When stuck (timebox ladder)

30 min stuck → write the problem down in one paragraph (half solve themselves). 2 h stuck → spike branch, hack without tests until it works once, then reimplement clean (never merge the spike). 1 day stuck → this is a risk-register event: log it, pick the mitigation or the fallback (every M-risk in IMPL-07 has one), move on. Nothing in this program is allowed to block silently for two days.

## What never gets skipped vs. what always gets deferred

**Never skip:** golden traces green before merge · chain verify + fold check · pack capture on every brain call · tier honesty (never claim containment you didn't canary) · the Friday demo.
**Always defer (until their milestone or a partner asks):** styling polish · configuration options ("make it a flag later") · performance work without a failing timing assertion · abstractions with one caller · any second implementation of anything (second OS, second provider beyond the two, second worker runtime).

## Milestone Definition-of-Done checklist (run at every tag)

☐ Demo recorded and script committed ☐ All acceptance criteria in IMPL-03 row ticked with links to evidence (test names, golden names) ☐ New golden traces merged ☐ Risk register row for this milestone reviewed and updated ☐ `DECISIONS.md` updated (anything decided under pressure gets written down cold) ☐ Docs touched if a contract or interface moved ☐ Tag pushed (`v0.0.x-mN`) ☐ Next milestone's first three tasks written before stopping — never end a milestone without knowing Monday's first task.

## The engineering checklist (the whole program on one screen)

1. Prereqs done (IMPL-00 list) — OS recorded, keys obtained, fixture repos chosen.
2. Sprint 1, days 1–10 (IMPL-04) → tag `v0.0.1-m0`, demo #1.
3. M1–M8 in order (IMPL-03), one at a time, DoD checklist each.
4. E1-lite at M5.5; result to DECISIONS.md.
5. M8 → invite 10–20 partners (founder-OS or S0), run Phase C per v2-09.
6. Thesis readout against v2-01 metrics → the only success criterion that counts.
7. Only then: the post-preview architecture review (unfreeze checkpoint) with cohort evidence in hand.

## The reminder that matters

You are not building all of WePLD. You are proving a thesis: *people will delegate bounded engineering work to a system that is isolated, evidence-gated, decision-routed, and replayable.* Every Friday demo either strengthens the evidence for that sentence or tells you something the architecture documents could not. Both outcomes are the job.
