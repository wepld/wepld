# 27 — Performance Goals

## Performance principle

WePLD is a local operational cockpit. It must feel responsive even while workers, models, tests, and indexing are slow. User-facing control-plane responsiveness is separated from external-provider and task-execution duration, which must instead be visible, budgeted, cancellable, and measurable.

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

The baseline machine, sample project sizes, event rate, worker count, and data corpus must be published alongside each benchmark. A number without workload and hardware context is not a performance goal.

## Responsiveness model

| Work type | UX expectation | Control strategy |
| --- | --- | --- |
| Local command/projection | immediate acknowledgment and visible durable state | priority scheduling, bounded queries, backpressure |
| Brain invocation | honest in-progress status, cancellation, cost/time estimate | deadlines, profile budget, fallback, streaming only as non-authoritative progress |
| Build/test/scan | live evidence and resource state, not blocked UI | isolated worker quota, artifact streaming, cancellation |
| Indexing/embedding | background, retryable, does not block mission ledger | queues, rate limits, degraded retrieval state |
| Large timeline/artifact | incremental rendering and drill-down | pagination, summaries, lazy artifact fetch |

## Scalability path

V1 is sized for a single desktop project and a bounded number of concurrent local worker attempts. The Core uses backpressure rather than accepting unlimited tasks. Worker scheduling considers CPU, memory, disk, process count, network, provider concurrency, and mission priority. When resource pressure passes a policy threshold, the scheduler pauses lower-priority ready work and shows the reasoning in Mission Control.

Scale-out is not assumed from local metrics. Future organization deployments establish separate targets for remote queue latency, artifact transfer, synchronization, tenant isolation, and regional data residency before admitting remote workers or shared control planes.

## Performance quality gates

- New Core/Studio features include a benchmark or reasoned exemption for their critical path.
- Event and query schemas avoid unbounded payloads and N+1 projection patterns.
- Large logs/artifacts are streamed/stored out of the event payload path.
- Provider usage exceeds no mission/profile budget without an explicit policy action.
- A measurable regression beyond the project’s approved budget blocks release or requires a time-bound exception.
- Resource starvation, thermal pressure, disk exhaustion, and battery-sensitive modes are tested as degraded states.

## Observability

Performance telemetry includes trace/correlation IDs, queue time, execution time, retry reason, resource usage, payload/artifact sizes, cache/index state, model/provider usage, and result classification. Telemetry must be classified/redacted and locally usable; external export is optional and policy-governed. Mission Control distinguishes measurement gaps from healthy performance.

## Open calibration decisions

Phase 0 must decide supported hardware tiers; whether low-power/mobile hardware is in scope; maximum concurrent worker attempts; test-project corpus; offline data-size limits; default model budgets; and whether UI responsiveness targets differ for accessibility technology. These choices set realistic targets without weakening correctness or safety.

See also: [13_Mission_Control.md](13_Mission_Control.md), [19_Implementation_Roadmap.md](19_Implementation_Roadmap.md), [23_Technology_Evaluation.md](23_Technology_Evaluation.md), and [26_Testing_Strategy.md](26_Testing_Strategy.md).

