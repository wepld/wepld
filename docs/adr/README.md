# WePLD Architecture Decision Records

This directory contains **Proposed** decisions for the governed-delivery and Hermes Intelligence roadmap. None is accepted, and their presence does not authorize implementation.

Draft PR #1 contains branch-local ADR-0001 through ADR-0014. Those records are unmerged and non-canonical. This package begins at ADR-0015 only to avoid a future filename collision; the numbering does not ratify the candidate ADRs or the PR.

## Convention

Each ADR uses these metadata fields:

- `Status`
- `Date`
- `Owner`
- `Review`

The standard sections are `Context`, `Decision`, `Reason`, `Benefits`, `Trade-offs`, and `Migration`. Required milestone/gate evidence and compatibility with Draft PR #1 are recorded under `Migration`. Every ADR in this package remains `Proposed` until an explicit architecture decision accepts, rejects, or supersedes it outside this planning commit.

## Index

| ADR | Decision | Earliest gate |
| --- | --- | --- |
| [0015](ADR-0015-governed-specification-contract.md) | Specification as executable governance contract | H1 |
| [0016](ADR-0016-brain-plan-builder-execution-separation.md) | Brain-plan / builder-execution authority separation | H2 |
| [0017](ADR-0017-phase-kanban-controlled-change.md) | Phase, Kanban, and controlled-change semantics | H2 |
| [0018](ADR-0018-hermes-skill-runtime-hook-bus.md) | Hermes Skill Runtime and Hook Bus | H3 |
| [0019](ADR-0019-context-compiler-lsp-hybrid-retrieval.md) | Context Compiler, LSP, and hybrid retrieval | H4 |
| [0020](ADR-0020-typed-memory-memory-judge.md) | Typed memory and Memory Judge | H7 |
| [0021](ADR-0021-bounded-subagents-structured-handoffs.md) | Bounded subagents and structured handoffs | H6 |
| [0022](ADR-0022-controlled-loop-escalation.md) | Controlled loop and escalation semantics | H5 |
| [0023](ADR-0023-model-independent-outcome-equivalence.md) | Model-independent outcome equivalence | H8 |
| [0024](ADR-0024-evaluation-spine-run-provenance.md) | Evaluation spine and exact run provenance | Before H1/H2 |
| [0025](ADR-0025-model-profile-certification.md) | Controlled model/profile certification | H8 |

Each ADR must be reviewed against canonical `main`, the final disposition of Draft PR #1, affected risks, threat models, contracts, and milestone evidence. No ADR authorizes implementation or a PR merge by itself.

History note: the earlier unaccepted `ADR-0024-harness-evaluation-provider-certification.md` combined early evidence capture with H8 certification. This remediation replaces that Proposed record with independently reviewable ADR-0024 (evaluation spine before H1/H2) and ADR-0025 (model/profile certification at H8). No Accepted decision was superseded.
