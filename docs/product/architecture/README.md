# WePLD Product Architecture (S0-B)

**Package:** S0-B Product Architecture Foundation.
**Status:** documentation only (founder decision, 2026-07-20).
**Authorizes no implementation.** This package defines *what WePLD is as a
product* and the human-centred architecture that governs it. It does not
create runtime code, UI code, schemas, providers, agents, or integrations.

## What this package is

The canonical, human-centred **product** architecture for WePLD. It sits
above the S0-A engineering constitutions and inherits every one of their
rules without weakening them. It answers: what WePLD is; who controls it;
what the user sees; how work is represented; how humans, models, roles,
and agents relate; how missions move from intent to verified outcome; how
permissions and approvals appear; how Personal and CoWork modes differ;
how GitHub and Delivery fit without becoming hidden authorities; and what
must be accepted before S1 implementation can begin.

## Documents and status

| Document | Subject | Status |
| --- | --- | --- |
| `S0-B-PRODUCT-ARCHITECTURE.md` | product definition, promise, users, modes, surfaces, journey, principles, non-goals | Adopted for S0-B |
| `HUMAN-CENTRED-CONTROL-MODEL.md` | Director authority, approvals, pause/cancel/freeze/takeover, recovery UX | Adopted for S0-B |
| `PRODUCT-DOMAIN-MODEL.md` | product entities, relationships, invariants (conceptual) | Adopted for S0-B |
| `STUDIO-INFORMATION-ARCHITECTURE.md` | Chat/Work/Code, navigation, canvas, inspector, dock, status, layouts | Provisional |
| `AI-CREW-AND-ASSIGNMENT-MODEL.md` | model/provider/role/agent/assignment, crew, autonomy | Adopted (vocabulary Provisional) |
| `MISSION-AND-RUN-LIFECYCLE.md` | mission/run state model and human authority over it | Adopted for S0-B |
| `PERSONAL-AND-COWORK-BOUNDARY.md` | local vs server authority, identity, sync, audit, privacy | Adopted (CoWork detail Deferred) |
| `GITHUB-AND-DELIVERY-BOUNDARIES.md` | GitHub and Delivery Center product boundaries | Provisional |
| `S0-B-DECISION-AND-ACCEPTANCE-REGISTER.md` | decision classification + S0-B acceptance gate for S1 | Adopted for S0-B |

## Authority and precedence

1. **Human final authority** and the **S0-A Security Constitution,
   Capability Model, Trusted Computing Base, Safe Rust Standard, and
   Technology Constitution** are supreme. Nothing here overrides them.
2. **UI-zero-authority** is a final constitutional rule: the interface
   presents and requests; it never holds authority.
3. Where this package is more specific than S0-A, it *refines* within
   S0-A's limits; where it appears to conflict, S0-A wins and the conflict
   is a defect to be corrected.

## Relationship to S0-A and S0.5A

- **S0-A** (on canonical main) is the engineering constitution set; this
  package is its product-facing companion.
- **S0.5A** produced Windows desktop-security evidence (closed evidence
  PR #9, never-merge; recorded in
  `../../architecture/evidence/S0-5A-WINDOWS-DESKTOP-SECURITY-EVIDENCE.md`
  and the TDR-002 amendment). Its status — Tauri 2 + separate Rust core
  ratified **for founder-controlled Windows Personal Alpha only**;
  macOS/Linux runtime unverified — is inherited here unchanged. **No S0.5A
  prototype code is reused by this package.**

## Non-authorization notice

This package authorizes **no** implementation of any kind. The exhaustive
list is in `S0-B-PRODUCT-ARCHITECTURE.md` (Non-goals) and the register.
Starting S1 requires passing the acceptance gate in
`S0-B-DECISION-AND-ACCEPTANCE-REGISTER.md`.
