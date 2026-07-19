# S0-B Decision and Acceptance Register

**Status:** Adopted for S0-B (2026-07-20). Documentation only. Classifies
every material product decision and defines the acceptance gate for S1.

## 1. Classification key

- **Final** — constitutional or already-ratified; not reopened here.
- **Adopted for S0-B** — product semantics accepted in this package.
- **Provisional** — needs usability/product evidence before final.
- **Deferred** — implementation detail, out of scope until later.
- **Rejected** — considered and excluded.
- **Open Question** — product research still required.

## 2. Decision register

| # | Decision | Classification |
| --- | --- | --- |
| 1 | Human is the final authority over all consequential effects | Final |
| 2 | UI-zero-authority (interface presents/requests, never enforces) | Final |
| 3 | Proprietary, closed-source posture | Final |
| 4 | Separate Rust trusted-core + typed bounded local IPC (Windows Personal Alpha scope) | Final (per TDR-002 amendment) |
| 5 | Tauri 2 shell for founder-controlled Windows Personal Alpha only | Final for that scope; macOS/Linux Provisional |
| 6 | Product identity: Local-first AI Development and Computer Control Studio | Adopted for S0-B |
| 7 | Product promise ("You direct. Your AI studio works. WePLD keeps you in control.") | Adopted for S0-B |
| 8 | Personal and CoWork are distinct modes; CoWork must not weaken Personal | Adopted for S0-B |
| 9 | Product object model (Workspace…Outcome Bundle) | Adopted for S0-B |
| 10 | Chat / Work / Code top-level modes | Adopted for S0-B |
| 11 | Studio Canvas (compose views; tabs/splits) | Adopted for S0-B (view inventory Provisional) |
| 12 | Inspector (context…CoWork panels) | Adopted for S0-B (panel inventory Provisional) |
| 13 | Command Dock inventory | Provisional |
| 14 | Assignment hierarchy (workspace→run; narrower wins, policy caps) | Adopted for S0-B |
| 15 | Autonomy levels 1–5, no unrestricted level | Adopted for S0-B |
| 16 | Mastermind coordinates, no unrestricted effect authority | Final (from S0-A boundary) |
| 17 | Model/Provider/Role/Agent/Assignment kept distinct | Adopted for S0-B |
| 18 | No silent provider or data-routing change; classification-gated | Final (from S0-A) |
| 19 | GitHub first-class but never a hidden authority; approval + identity guard | Adopted for S0-B |
| 20 | Delivery Center activities (Build…Handoff) | Provisional |
| 21 | Outcome Bundle concept | Adopted for S0-B |
| 22 | Security profiles (Fortress/Controlled/Enterprise Managed) as product-facing profiles | Provisional |
| 23 | Denial/failure/unavailable recovery UX; no false success; Restart Core | Adopted for S0-B (grounded in S0-A + S05A-RUNTIME-001) |
| 24 | Progressive disclosure | Adopted for S0-B |
| 25 | Accessibility + Arabic/RTL as architecture | Final (constitutional stance) |
| 26 | Studio role vocabulary and saved lineups | Provisional |
| 27 | Coordination-mode defaults | Provisional |
| 28 | Detailed Studio layout inventory + defaults | Provisional |
| 29 | Cross-platform UI beyond Windows Personal Alpha | Open Question |
| 30 | Local persistence = SQLite (direction) | Provisional |
| 31 | Server persistence = PostgreSQL (direction) | Provisional |
| 32 | CoWork identity/sync/conflict model | Deferred |
| 33 | Plugin system timing | Deferred |
| 34 | Updater timing | Deferred |
| 35 | External distribution / signing timing | Deferred |
| 36 | Reuse of S0.5A prototype code | Rejected |

## 3. Why the Provisional/Open items are not Final

| Item | Why not final | Evidence required | Latest responsible decision | Consequence of delay |
| --- | --- | --- | --- | --- |
| Studio layouts/view/panel/dock inventories (11,12,13,28) | UX not validated | usability testing of real workflows | before S1 UI work begins | UI rework if frozen wrong |
| Role vocabulary + lineups + coordination defaults (26,27) | naming/ergonomics unproven | founder + early-user trials | before crew UI is built | churn in crew UX and docs |
| Security profiles as product surface (22) | product framing untested | mapping to Capability Model in use | before external evaluation | mislabeled security posture |
| Cross-platform UI (29) | macOS/Linux runtime unverified (S0.5A) | S0.5B-style runtime evidence | before any non-Windows Alpha | overcommitting to unproven platforms |
| SQLite/PostgreSQL directions (30,31) | persistence not prototyped at product scale | a persistence spike | before S1 storage work | schema rework |
| CoWork model (32) | multi-writer semantics unproven | a CoWork sync prototype | before CoWork Alpha | unsafe collaboration |
| Plugin/updater/distribution timing (33,34,35) | premature surface expansion | phase demand + security review | before their respective phases | supply-chain and trust risk |

## 4. Cross-cutting requirements (must hold in every S0-B document)

Human final authority; UI-zero-authority; explicit capability/approval
boundaries; local-first Personal mode; no silent provider switching; no
silent data-routing change; understandable denial/failure states;
accessibility as architecture; Arabic/RTL first-class; evidence and
auditability; progressive disclosure; no automatic reuse of S0.5A
prototype code.

## 5. S0-B acceptance criteria

S0-B is accepted when: (a) the ten package documents exist on canonical
main; (b) each material decision is classified here; (c) every Provisional
and Open item states why-not-final, evidence-required, latest decision
point, and delay consequence; (d) no document assigns authority to the UI,
grants the Mastermind unrestricted control, conflates
model/provider/role/agent/assignment, makes routing implicit, treats
CoWork as networked Personal, invents schemas/APIs, defers accessibility,
generalizes Windows evidence, or implies prototype reuse; (e) the
architecture validator passes and cross-references resolve.

## 6. Gate for starting S1

**S1 (implementation) must not begin until all of the following are
satisfied by separate, explicit founder authorization:**

1. This S0-B package is merged and accepted on canonical main.
2. The Final and Adopted-for-S0-B decisions above are confirmed by the
   founder as the implementation baseline.
3. The Provisional items required by the first S1 slice have their stated
   evidence gathered (or are explicitly accepted as provisional risk for
   that slice).
4. The S1 slice is scoped to specific product surfaces with its own
   authorization, honoring UI-zero-authority and the Capability Model.
5. No S1 slice reuses S0.5A prototype code by default; any reference use
   is a separate, reviewed decision.
6. macOS/Linux remain out of scope until their runtime evidence exists.

Until this gate is passed by a separate authorization, **no S1 or
implementation work is permitted.** This register does not itself open S1.
