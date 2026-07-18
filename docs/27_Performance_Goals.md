# 27 — Performance Goals

## Performance principle

WePLD is a local operational cockpit. It must feel responsive even while workers, models, tests, LSP servers, retrieval, subagents, and indexing are slow. User-facing Core responsiveness is separated from reasoning and engineering duration, which must instead be visible, budgeted, cancellable, attributable, and measurable.

All numbers here are planning targets until their milestone publishes a workload, environment, and result. Draft PR #1 is an unmerged candidate baseline; its test counts are not performance evidence and this plan makes no achieved-performance claim for it.

## Provisional V1 service objectives

These are planning targets to calibrate in Phase 0 on a published baseline machine and supported operating systems. They are not claims of achieved performance.

| Concern | Target | Measurement boundary |
| --- | --- | --- |
| Local command acknowledgment | p95 under 150 ms | Studio → local Core acceptance; excludes an approval/workflow outcome |
| Projection/event freshness | p95 under 1 s | committed local event → subscribed Studio/Mission Control update |
| Event append | p95 under 25 ms | validated domain event → durable local ledger commit under nominal load |
| Read projection | p95 under 100 ms | indexed local query for common mission/task views, excluding cold large artifacts |
| Scheduler dispatch | p95 under 500 ms | ready task → compatible local worker lease under available capacity |
| Studio interaction | p95 under 100 ms | navigation/filter/selection response using already available data |
| Startup to usable cached state | under 5 s | warm-start Studio plus Core connection; no provider call required |
| Recovery correctness | 100% fixture pass | restart/lease/event replay scenarios; correctness dominates time |
| Resource guardrails | configurable hard CPU/memory/disk/process quotas | per worker/tool task and Core health reporting |
| Cost attribution | 100% of provider invocations attributed | profile, mission, task, retry, and budget class present |
| Context compilation | p95 target established at H4 from representative packs | collect/filter/rank/compress/validate plus manifest write; report source count and token budget |
| LSP freshness | every language-intelligence result carries document/index version and freshness | stale results are rejected or visibly labelled; no universal latency target before adapter evidence |
| Retrieval provenance | 100% of context items explain source, trust, freshness, rank reason, scope, and token estimate | pack manifest completeness, not retrieval eloquence |
| Loop boundedness | 100% of attempts stop/escalate at configured attempt/time/cost/no-progress guards | H5 guard fixtures; no completion after guard violation |
| Subagent/WIP bounds | zero concurrency or writable-scope violations | H6 scheduler/supervisor evidence |
| Evidence completeness | 100% of accepted outcomes satisfy the same required evidence contract | invariant across supported model profiles |
| Outcome convergence | reported per fixed harness, never as an unconditional promise | attempts, wall time, cost, escalation, and honest non-convergence by profile |

The baseline machine, sample project sizes, event rate, worker count, and data corpus must be published alongside each benchmark. A number without workload and hardware context is not a performance goal.

## Responsiveness model

| Work type | UX expectation | Control strategy |
| --- | --- | --- |
| Local command/projection | immediate acknowledgment and visible durable state | priority scheduling, bounded queries, backpressure |
| Brain invocation | honest in-progress status, cancellation, cost/time estimate | deadlines, profile budget, fallback, streaming only as non-authoritative progress |
| Build/test/scan | live evidence and resource state, not blocked UI | isolated worker quota, artifact streaming, cancellation |
| Indexing/embedding | background, retryable, does not block mission ledger | queues, rate limits, degraded retrieval state |
| LSP/structural analysis | bounded, cancellable, versioned; stale state is visible | adapter health, document versions, cache invalidation, fallback to authoritative exact sources |
| Context compilation | completes before a Brain/builder call or fails explicitly | deterministic stages, per-tier budgets, omission manifest, no silent truncation of pinned truth |
| Subagent work | independent progress with enforced WIP and budget | supervisor quotas, read/write distinction, deadlines, structured handoffs |
| Large timeline/artifact | incremental rendering and drill-down | pagination, summaries, lazy artifact fetch |

## Hermes Intelligence measurement model

Performance must never be improved by lowering acceptance quality. Compare profiles and harness components on a fixed mission/repository/specification/policy/environment/budget. Report at least:

- context compile, LSP, retrieval, model, tool, gate, and human-wait time separately;
- pack sources/items/tokens, omissions, cache/index state, and freshness;
- loop iterations, repeated/no-progress guard activations, replans, escalations, and recovery;
- subagent concurrency, queue time, writable occupancy, conflicts, and handoff size;
- outcome-equivalence, regression, unsafe-effect, evidence-completeness, and non-convergence honesty alongside cost/time.

Faster but unsafe, under-evidenced, or non-equivalent output fails. A weaker profile may legitimately require more context, attempts, time, cost, or escalation; the final bar remains constant.

## Scalability path

The local roadmap is sized for one desktop project and bounded local work. Core uses backpressure rather than accepting unlimited tasks. Scheduling considers CPU, memory, disk, process count, network, provider/LSP concurrency, context/index pressure, writable-worktree occupancy, mission priority, and human decision WIP. When pressure crosses policy, lower-priority ready work pauses and the reason is recorded.

Scale-out is not assumed from local metrics. Future organization deployments establish separate targets for remote queue latency, artifact transfer, synchronization, tenant isolation, and regional data residency before admitting remote workers or shared control planes.

## Performance quality gates

- New Core/Studio features include a benchmark or reasoned exemption for their critical path.
- Event and query schemas avoid unbounded payloads and N+1 projection patterns.
- Large logs/artifacts are streamed/stored out of the event payload path.
- Provider usage exceeds no mission/profile budget without an explicit policy action.
- A measurable regression beyond the project’s approved budget blocks release or requires a time-bound exception.
- Resource starvation, thermal pressure, disk exhaustion, and battery-sensitive modes are tested as degraded states.
- New context, LSP, retrieval, loop, skill, hook, memory, or subagent machinery includes an attribution benchmark and an ablation result or a reasoned exemption.
- Performance telemetry never records secret or unrestricted context bodies; source references and aggregate sizes are sufficient.

## Observability

Performance telemetry includes trace/correlation IDs, queue time, execution time, retry reason, resource usage, payload/artifact sizes, cache/index state, model/provider usage, and result classification. Telemetry must be classified/redacted and locally usable; external export is optional and policy-governed. Mission Control distinguishes measurement gaps from healthy performance.

## Open calibration decisions

The applicable gate must decide supported hardware tiers; whether low-power/mobile hardware is in scope; maximum worker/subagent/WIP limits; initial LSP adapters; repository and evaluation corpus; offline index/data limits; context and model budgets; and whether responsiveness targets differ for accessibility technology. H4/H8 evidence—not assumptions—sets realistic defaults without weakening correctness or safety.

See also: [13_Mission_Control.md](13_Mission_Control.md), [19_Implementation_Roadmap.md](19_Implementation_Roadmap.md), [23_Technology_Evaluation.md](23_Technology_Evaluation.md), [26_Testing_Strategy.md](26_Testing_Strategy.md), and [34_Harness_Evaluation_Protocol.md](34_Harness_Evaluation_Protocol.md).

