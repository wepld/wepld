# 06 — Brain Architecture

## Purpose and invariant

Brains are replaceable reasoning providers. They analyze, plan, consult, classify, synthesize, and propose structured actions. They do **not** edit files, run shells, retrieve undeclared secrets, or communicate with the user. The Brain Gateway is the only place that knows provider-specific APIs.

## Logical architecture

~~~mermaid
flowchart LR
  Worker["Worker role"] --> Request["Provider-neutral Brain Request"]
  Request --> Router["Brain Gateway\nvalidation + routing + budget"]
  Router --> Local["Local adapter\nOllama / vLLM / future"]
  Router --> Hosted["Hosted adapters\nHermes / OpenAI / Anthropic / Google / OpenRouter / future"]
  Local --> Normalize["Normalized Brain Result"]
  Hosted --> Normalize
  Normalize --> Worker
  Router --> Audit["Invocation evidence\nusage, cost, policy, traces"]
~~~

## Provider-neutral contract

| Request field | Meaning |
| --- | --- |
| Intent | Planning, review, classification, synthesis, research, diagnosis, or another named capability |
| Context references | Immutable artifact and knowledge references, not unbounded prompt history |
| Output contract | Versioned structured schema, acceptance constraints, and required citations |
| Constraints | Role policy, data classification, deadline, quality level, and prohibited behavior |
| Budget | Maximum tokens, spend, retries, latency, and provider fallback policy |
| Trace identity | Mission, task attempt, correlation, causation, and cancellation identifiers |

| Result field | Meaning |
| --- | --- |
| Structured output | Schema-valid answer or a validation failure |
| Evidence links | Artifact/knowledge references supporting material claims |
| Proposed actions | Suggestions for later policy evaluation, never directly executable effects |
| Invocation record | Provider, model, adapter version, settings profile, usage, cost, latency, warnings |
| Uncertainty | Confidence bands, missing context, conflicts, and refusal/degradation reason |

Native tool calls are normalized into `ProposedAction` records. The Policy Engine and Tool Executor decide whether the action can occur. This prevents a provider feature from becoming an unreviewed capability escalation.

## Profiles, routing, and fallback

A **brain profile** is a named, versioned policy configuration: allowed adapters/models, capability requirements, context size, residency/data-classification limits, cost ceiling, latency target, retention policy, and fallback sequence. A worker requests a profile, not a vendor/model name.

The router filters profiles by policy first, then selects based on required capability, structured-output reliability, expected latency, availability, cost, context capacity, locality, and previous evaluation results. It records why a route was selected. Fallback may occur only when output semantics are preserved; it never silently upgrades a data-classification or budget permission.

Examples:

- A confidential architecture review may allow only a local model profile.
- Routine documentation can prefer a low-cost hosted model with citations required.
- A high-stakes security finding can request two independent profiles and mark disagreement as a review condition.

## Privacy and credential rules

Provider credentials live in an OS-backed secure store or approved enterprise secret provider, never in skills, worker prompts, logs, or plugin manifests. Context is minimized and redacted before egress. The invocation record stores identifiers, policy decisions, usage, and content hashes; retention of raw prompts/responses follows project classification and explicit consent. An unavailable provider leaves the mission in a visible degraded or waiting state rather than fabricating progress.

## Evaluation and change management

Each approved profile has an evaluation suite appropriate to its role: structured-output validity, task quality, hallucination resistance, citation quality, policy adherence, cost, latency, and regression cases. A new provider/model is introduced as a candidate profile, benchmarked on non-sensitive fixtures, and promoted only by a documented policy decision. A provider regression can disable or quarantine a profile without altering worker or mission code.

## Non-goals

- A universal prompt format that hides materially different provider behavior.
- Trusting a model’s self-reported tool result or test result.
- Letting a brain own durable task state, code patches, credential access, or final authorization.

See also: [05_Worker_Architecture.md](05_Worker_Architecture.md), [14_Security_Architecture.md](14_Security_Architecture.md), [16_Data_Model.md](16_Data_Model.md), and [23_Technology_Evaluation.md](23_Technology_Evaluation.md).

