# V0 Governance, Safety & Limits

This document records the governance and safety contract of the V0 Build Feature
slice as **implemented**, after the architecture/security review (PR #1). It is
deliberately narrow and honest about what is enforced versus disclosed.

## 1. Build Feature is a governed, staged lifecycle

The recipe may hide internal *technical* vocabulary (specification, plan, task)
from the user, but it never hides or invents a **governance** decision. It runs
as explicit, resumable stages:

1. **Start** (`start_build_feature`, authenticated requester) — reason a
   specification, create the mission, propose a plan, then **stop** with a typed
   `NeedsPlanApproval`. Nothing executes.
2. **Approve plan & execute** (`approve_plan_and_execute`, authenticated
   approver) — record the explicit plan approval (real actor), run the mission,
   and on green gates **stop** with `NeedsCompletionApproval` carrying the
   evidence (snapshot commit, diff ref, gates, criteria, proposal ref).
3. **Decide completion** (`decide_completion`, authenticated approver) —
   explicitly accept or return. Acceptance is **never** automatic.

There is **no public one-shot recipe method**: a caller must invoke each stage
as a distinct, explicit action. A shared principal string is not evidence that
two governance decisions occurred, so no convenience that auto-runs approval and
acceptance is offered (`no_public_recipe_entrypoint_can_auto_approve_or_auto_accept`).

**Actor provenance.** `PlanApproved`, `MissionAccepted`, and `MissionReturned`
record `ActorType::Human` with the **caller-supplied** principal id. The Core
never fabricates a human actor: an unauthenticated principal is rejected, and
Core-initiated facts use `ActorType::Core`.

## 2. Acceptance produces a proposal ref — never a merge

V0 acceptance **does not** merge into the base branch or touch the primary
worktree. It creates/updates a proposal ref
`refs/heads/wepld/mission-<id>` at the final snapshot commit (`propose_ref`,
implemented with `git update-ref` — no checkout, no merge). A human merges it
later through an external protected workflow.

`Workspace::merge` remains in the code for a future protected-merge workflow but
is **`#[deprecated]` and out of the V0 execution path**; tests assert the base
branch HEAD and the primary worktree are unchanged after acceptance.

### Recovery-safe acceptance (no distributed transactions)

Acceptance records intent before effect and heals idempotently:

1. record the explicit decision + intended effect, set `acceptance_pending`;
2. perform the ref effect as a **compare-and-swap** (`git update-ref <ref> <new>
   <expected-old>`): create only if absent, no-op if already at the snapshot,
   and **refuse to overwrite** a ref that points elsewhere — a conflict records
   `acceptance_uncertain` and defers (never force-replaces);
3. probe the real git state (and confirm the base branch did not move);
4. record `MissionAccepted` + `accepted`, or an explicit `acceptance_uncertain`
   with evidence.

Recovery reuses the **original** approver recorded in the `DecisionResolved`
fact — a retry caller cannot rewrite who approved.

The provider adapter parses endpoints with a standards-compliant URL parser
(`url::Url`): scheme must be `http`, no userinfo, no query/fragment, and the
host must be a real loopback IP (or a `localhost` that resolves only to
loopback) — so `http://127.0.0.1@evil.example` and similar spoofs are refused.

The DEV preflight runs at **every** point that could reach a worker: mission
creation (including spec-derived), planning, running, and the worker-spawn
boundary itself (defense in depth) — a denied repository creates no mission,
spawns no worker, and produces no attempt, gate, or proposal ref.

A crash before the effect or before the final record leaves `acceptance_pending`
with **no** `MissionAccepted`; a retry probes and completes with exactly one
acceptance and no base mutation. Fault-injection tests
(`AcceptFault::BeforeEffect` / `BeforeFinalRecord`) prove each path.

## 3. DEV tier — disclosed, not enforced (IADR-0003)

The runtime tier is `DEV`: **there is no OS containment.** The Envelope is
*descriptive* under DEV, not an enforcement boundary — do not read a DEV run as
sandboxed. The truthful disclosure, surfaced by the CLI and recorded in the
ledger, is:

> DEV tier: no OS containment; worker and gate processes have ambient host
> authority.

DEV caps, enforced by `dev_tier_gate` before any worker runs:

- **Manual mode only** — Bounded-Auto missions are refused.
- **Fixture repositories only by default** — a mission whose (canonicalized)
  repository is not within the configured fixtures root is refused.
- **Explicit override** — `--i-understand-dev-tier` (CLI) →
  `allow_uncontained_repo(repo, actor)` permits exactly *one* uncontained repo,
  by an authenticated actor, recorded durably (repo, actor, tier, warning). No
  silent or default override exists.

## 4. Provider adapter — local-loopback-only in this build

The OpenAI-compatible adapter is **local-loopback-only**. `OpenAiCompatAdapter::new`
validates configuration and returns a typed `AdapterConfigError`:

- credential-free HTTP is allowed **only** to `127.0.0.1`, `localhost`, `::1`;
- non-loopback HTTP is refused;
- any API key over HTTP is refused;
- HTTPS is refused (no TLS is built in yet) — never silently downgraded;
- malformed / unsupported URLs are refused.

No credential is ever placed in a request, `Debug`, `Display`, log, event, or
error in this build. Hosted / API-key support is **deferred** until a
verified-TLS build lands and is tested.

## 5. Untrusted input is confined at the boundary

Model- and brief-produced text is **untrusted** and never becomes path or ref
*syntax*. The central validation contracts (`wepld_contracts::validation`) are
enforced at the Core boundary:

- **Edit paths** — the builder worker validates every edit path into a
  `WorkspaceRelativePath` (rejects absolute paths, drive/UNC prefixes, and every
  `..` component by inspecting `std::path::Component`, so `release..notes.txt` is
  fine while `a/../../x` is not) and writes **symlink-safe**: no existing parent
  or final component may be a symlink, so a pre-existing link cannot redirect a
  write outside the worktree. Workspace confinement is enforced by validated
  paths and symlink-safe writes — not by string inspection.
- **Identifiers are data** — slugs (`[a-z0-9]+(-[a-z0-9]+)*`), mission/task/
  attempt ids (`[A-Za-z0-9][A-Za-z0-9_-]*`), and base refs are validated before
  they seed a filesystem path or a Git ref. An invalid identifier is a
  deterministic *recorded command rejection*, never a panic or a later Git error.
- **Plan output is validated semantically** — after deserializing a model plan,
  the Core checks bounded task count, valid + unique task ids, non-empty titles,
  and acceptance-criterion references *before* the plan is approved, stored,
  materialized into task rows, or given a worktree.
- **Git defense in depth** — `Workspace::create_worktree` independently refuses
  an unsafe `attempt_id` and proves the destination is a direct child of the
  worktrees root; base refs are resolved to a commit SHA with `--end-of-options`
  (so a value beginning with `-` can never become an option); and every
  generated ref is checked with `git check-ref-format`.

## 6. `Deferred` is not `Rejected`

A recoverable acceptance is preserved as `RecipeOutcome::Deferred { mission_id,
state, reason }` — the durable decision stands, acceptance is not final, no merge
occurred, and a retry can still reach `Completed`. It is never flattened into
`Rejected`. The CLI/demo surfaces this distinctly ("acceptance NOT final —
retry to recover; no merge occurred").

## 7. Fail-closed canonicalization

`set_fixtures_root` returns a `Result`: a path that cannot be canonicalized is
refused and the previously-authorized root is **left unchanged** (never silently
cleared or weakened). `Workspace::project_fingerprint` likewise fails closed —
no fallback to an unresolved path, and an **unborn/empty** repository (no root
commit) yields `NoRootCommit` rather than an empty, unstable identity.

## 8. ADR / status honesty

- **ADR-0013 (workspace snapshot refs)** still holds. What changed is the
  *acceptance effect*: V0 produces a proposal ref instead of an in-repo merge,
  strengthening the "primary worktree is protected" invariant. This is a
  narrowing of behavior within the existing ADR, not a reversal of it.
- **IADR-0003 (DEV tier)** is now actually enforced as caps (Manual-only,
  fixtures-only, explicit override) rather than merely described.
- No accepted ADR decision is reversed by this slice. If a future slice needs
  in-repo merge or Bounded-Auto under a real tier, that requires a new ADR.
