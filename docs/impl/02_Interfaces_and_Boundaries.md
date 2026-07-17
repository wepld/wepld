# IMPL-02 — Interfaces and Boundaries

Public interface = the package's `index.ts` re-exports, nothing else. Deep imports (`@wepld/x/src/…`) are a CI failure. Internal interfaces are folder-level within packages, listed where they matter. All types come from `@wepld/contracts` — no package defines a cross-boundary type locally.

## Public interfaces (the ones that matter)

~~~ts
// @wepld/contracts — schemas & types only, zero logic beyond zod parsing
export const schemas: { mission, plan, task, attempt, decision, artifactMeta, envelope,
  ledgerEntry, eventTypes /* v2-07 rev2, closed enum */, wwp: { attemptStart, heartbeat,
  contextGet, brainRequest, artifactPut, envelopeExtend, escalationRaise, phaseResult, cancel },
  brainRequest, brainResult, contextPack, frame, lens, session, causalEdge, lineage,
  decisionDelta, comparison, rcaReport, insight, command, inboundIntent, outboundMessage };
export type /* inferred from all of the above */;

// @wepld/ledger — the only writer is the transaction it hands you
export function open(dir: string): LedgerStore;            // refuses synced-folder paths (v2-06)
export interface LedgerStore {
  transact<T>(fn: (tx: Tx) => T): T;                        // synchronous, single-writer
  fold(missionId: string, uptoSeq?: number): MissionState;  // THE reducer (consistency, replay, checkpoints)
  verifyChain(missionId?: string): ChainReport;
  entries(q: EntryQuery): LedgerEntry[];                    // filtered, paginated reads
  tail(fromSeq: number): AsyncIterable<LedgerEntry>;        // feeds SSE
  checkpoint(missionId: string, seq: number): void;         // Chronicle writes via this, M6
}
export interface Tx {                                       // used ONLY by runtime's transition fn
  mutate(table: TableMutation): void;
  append(entry: NewLedgerEntry): SeqNo;                     // hash-chains automatically
  enqueue(work: WorkItem): void;                            // the outbox-lite (v2-02 §3)
}

// @wepld/artifacts
export interface Cas {
  put(body: Buffer | string, meta: ArtifactMeta): ArtifactRef;   // content-addressed, write-once
  get(ref: ArtifactRef | string): { meta: ArtifactMeta; body: Buffer };
  verify(hash: string): boolean;
  tombstone(hash: string, reason: string): void;
}

// @wepld/workspace
export interface Workspace {
  createWorktree(repo: string, base: string, attemptId: string): Worktree;
  snapshot(wt: Worktree, label: string): SnapRef;            // hidden ref, ADR-0013
  materialize(ref: SnapRef): string;                         // detached worktree path (Chronicle/fork)
  diff(a: SnapRef | string, b: SnapRef | string): DiffDoc;
  changedPaths(wt: Worktree): string[];                      // Core's own scope re-check (v2-02 §4)
  branchFrom(ref: SnapRef, name: string): void;              // fork restore
  cleanup(wt: Worktree): void;
}

// @wepld/sandbox
export interface SandboxHost {
  detect(): TierReport;                                      // probe + canary self-test (v2-05)
  launch(cmd: Cmd, envelope: Envelope): SandboxedProc;       // stdio piped; enforces fs/net/quotas
  capsFor(tier: Tier): AutonomyCaps;                         // ADR-0007 table as data
}

// @wepld/wwp
export function serve(proc: ChildProcess, handlers: CoreHandlers): WorkerSession;  // Core side
export function connectAsWorker(handlers: WorkerHandlers): CoreClient;             // Hermes side
// message shapes come from contracts; this package is framing + dispatch + heartbeat watchdog only

// @wepld/providers  (renamed from brains, IADR-0007 §5; the architecture's Brain Gateway role)
export interface Gateway {
  invoke(req: BrainRequest, profile: ProfileName): Promise<BrainResult>;  // validate → route →
}                                                                          // schema-check → retry-once →
export function registerAdapter(a: BrainAdapter): void;                    // record invocation via callback
// adapters: fixture (record/replay), anthropic, openaiCompat — nothing else knows providers exist
// IADR-0007 §1: invocation is OPTIONAL — deterministic phases legitimately make zero calls

// @wepld/context
export function assemble(input: {
  mission: MissionState; task?: Task; phase: Phase; role: RoleProfile; budgetTokens: number;
  sources: { repoMap: RepoMap; files: FileAccess; knowledge: KnowledgeQuery; priorPhases: Summary[] };
}): ContextPack;   // tiers T0–T4, selection manifest, redaction log, throws on T0 overflow (v2-04)

// @wepld/runtime — the Core; composes everything below it
export function startCore(cfg: CoreConfig): Core;
export interface Core {
  submit(cmd: Command): CommandOutcome;                      // idempotent pipeline (v2-02 §2)
  query: { mission(id): MissionView; missions(f): …; decisions(f): …; artifacts(f): … };
  events(fromSeq: number): AsyncIterable<LedgerEntry>;
  stop(): Promise<void>;                                     // drains workers, kills process group
}
// internal modules (folder boundaries): commands/ state-machine/ phase-engine/ gates/
// decisions/ messenger/ budgets/ recovery/ — only state-machine/ touches Tx

// @wepld/chronicle — read-side + command composition (never imports runtime)
export function frames(store, cas, mission, opts): Frame[];  // deterministic, cached, versioned
export function stateAt(store, mission, seq): MissionState;  // checkpoint + fold
export function causeCone(store, ref, depth): CausalChain;   // deterministic edges (MVP)
export function compare(a: Point, b: Point, facets): ComparisonDoc;
export function planFork(store, mission, seq): ForkPlan;     // invalidation report; execution goes
                                                             // through a ForkMission command
export function session(deps, missionId, mode): ReplaySession; // v2-12 state machine

// @wepld/studio-api
export function mount(core: Core, chronicle: …, port: number): { url: string; token: string };
// routes are exactly v2-17's; nothing undocumented is exposed
~~~

## Layering and drift prevention

The dependency rules of IMPL-01 are encoded in `.dependency-cruiser.cjs` and run in CI. The five that carry the architecture:

1. `contracts` depends on nothing (zod only).
2. **Only `runtime/state-machine` may use `ledger.Tx`** — grep-rule + lint rule; everything else reads. This is ADR-0003's single-writer, mechanically enforced.
3. **`hermes` → `wwp` only.** The moment Hermes imports `ledger` or `brains`, worker replaceability is dead; CI dies first.
4. `chronicle` never imports `runtime` (reads stores; mutates via commands) — ADR-0011's read-side purity.
5. `studio` imports no workspace package — HTTP/SSE only.

Cycles are impossible if the layer table holds; dependency-cruiser fails on any cycle regardless. Contract changes require a `contracts` version bump + changelog line + (for event types) the vocabulary-lock test update — making every drift a *visible decision* (IADR-0004).

## Internal interfaces worth naming (inside `runtime`)

| Module | Responsibility | Talks to |
| --- | --- | --- |
| `commands/` | idempotency, authorization, validation, outcome | `state-machine/` |
| `state-machine/` | THE transition function `apply()`; sole `Tx` user | ledger |
| `phase-engine/` | pack request → envelope → spawn hermes → stream WWP → phase result | context, sandbox, wwp, brains, workspace |
| `gates/` | Core-run checks in validator envelopes; `GateEvaluated` facts | sandbox, workspace, artifacts |
| `decisions/` | packet lifecycle, classes, batching, interrupt budget | state-machine via commands |
| `messenger/` | outbound composition under claims discipline; inbound intent → command | ledger (read), decisions |
| `budgets/` | projected-cost check, attribution rows | brains callback |
| `recovery/` | startup scan, probe, classify, dispose (v2-02 §7) | workspace, state-machine |

These are folders, not packages — promotion to packages is allowed later without interface change because they already communicate through the types above.
