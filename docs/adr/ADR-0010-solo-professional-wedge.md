# ADR-0010 — V1 persona: the solo professional wedge

**Status:** Accepted · **Date:** 2026-07-13 · **Owner:** Product/CTO · **Review:** end of Phase C with cohort evidence

## Context

v1 listed five personas (founder, engineering leader, engineer, compliance leader, platform admin) against a single-user V1, shipping organizational governance ceremony to a user who works alone (gate finding C3). The buyers of governance (organizations) are unreachable until multi-seat exists.

## Decision

V1 targets **one professional developer or technical founder who wants to delegate bounded engineering work safely on their own machine**. Every governance feature is framed and tuned as *self-serving* value:

| Mechanism | Solo framing |
| --- | --- |
| Evidence gates | "I don't merge what I can't verify" |
| Decision queue | "It asks me only what actually needs me" — interrupt budget enforced (v2-10) |
| Ledger/Timeline | "I can see exactly what it did while I wasn't looking" |
| Envelope/sandbox | "It cannot touch what I didn't hand it" |
| Replay | "I can reconstruct why any change happened" |

The enterprise story is a *consequence* kept alive by design (same ledger, same gates, same decision packets later routed to teams), not a V1 audience. A minimal second seat (read-only reviewer/approver link) is the first V2 candidate if design partners pull for it.

## Reason

A thesis experiment run on the wrong persona produces a false negative on a multi-year bet. The solo wedge is the only persona V1 can actually reach, and the interruption economics (v2-10) are tuned to what that persona tolerates.

## Benefits

Honest early metrics; marketing/product/docs speak one language; the "governance overhead at N=1" risk becomes a measured number (interruptions per mission) instead of an unexamined assumption.

## Trade-offs

Enterprise pipeline building is delayed; competitors may claim the governance narrative first in enterprise channels. Mitigation: the ledger/evidence substrate is the durable moat; narratives without substrates don't survive procurement.

## Migration impact

None technical. The Mission and Decision contracts already carry `owner`/`authority` fields that generalize to multiple principals.
