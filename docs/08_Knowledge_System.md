# 08 — Knowledge and Memory System

## Purpose and authority

WePLD's knowledge system is evidence-backed, typed organizational memory. It answers not only “what was said?” but what was observed, which version applied, why a claim is trusted, where it is valid, when it expires, and whether policy permits its use.

Memory never outranks governance. Core-owned policy, approved specifications, OutcomeContracts, plans, decisions, ADRs, budgets, capabilities, transitions, and completion state are authoritative records. The Knowledge System may index and project them, but a stale index, embedding, summary, or learned lesson cannot redefine them. Chat transcripts and model output are untrusted sources until validated.

## Memory classes

| Class | Scope and lifetime | Trust rule |
| --- | --- | --- |
| Working Memory | one Hermes attempt/inner loop | transient, bounded, reconstructable, never governance truth |
| Mission Memory | current mission and its approved artifacts, evidence, decisions, and unresolved context | durable in Core; access scoped to mission and policy |
| Engineering Memory | verified repository lessons, patterns, architecture findings, and recovery knowledge | consolidated only from evidence-linked MemoryCandidates |
| Skill Memory | measured outcomes, failure modes, compatibility, and procedure improvements | evaluation-derived; cannot silently modify a skill package |
| Provider/Model Performance Memory | measured profile behavior by role/task/risk/environment | routing evidence, not a permission or lower quality bar |
| Governance Memory | authoritative policy/ADR/specification/contract/decision state | exact Core versions; never optional untrusted advice |

The Hermes Agent Kernel may cache Working Memory, but durable Mission and Governance Memory remain in Core. Restarting Hermes cannot change their meaning.

## Knowledge graph model

V1 uses typed relational records, immutable artifacts, and indexes rather than a graph database.

| Node | Examples | Required provenance |
| --- | --- | --- |
| Governance reference | approved specification, OutcomeContract, plan, ADR, policy decision | Core identifier, exact version/status, authority, event link |
| Artifact | design, diff, log, diagnostic snapshot, benchmark, scan, screenshot | content hash, source, creator, timestamp, classification |
| Evidence | test result, LSP diagnostic delta, review finding, effect probe | requirement/gate binding, method, environment, producer, result |
| Claim | “API latency is below target” | source evidence, claimant, confidence, freshness, status |
| Decision | architecture or mission decision | options, authority, rationale, evidence, supersession link |
| Pattern / Lesson | retry convention, prior failure and prevention | scope, examples, validation, applicability, owner, review/expiry date |
| Finding | bug, security issue, performance regression | severity, lifecycle, affected version/scope, evidence |
| Mission / Phase / Task / Attempt | operational history reference | immutable event and governing-artifact links |
| MemoryCandidate | proposed durable lesson | candidate type, evidence, scope, confidence, security classification |
| Retrospective | outcome, plan variance, loop behavior, lessons, unresolved risk | mission versions, CompletionDecision, evidence bundle |
| Entity / Relation | symbol calls symbol; requirement traces to test | typed relation, source, confidence, validity interval |

## Memory Judge and consolidation

The Memory Judge is a governed Core/Knowledge service, not a model with unilateral write authority. Models may extract candidates; deterministic checks and policy determine admissibility.

1. Capture a typed `MemoryCandidate` from a completed artifact, finding, retrospective, or event.
2. Classify, hash, deduplicate, and attach access, retention, provenance, and candidate scope.
3. Verify cited evidence and exact governing versions; reject self-assertion and stale or unauthorized sources.
4. Detect contradiction, overlap, supersession, security exposure, and applicability boundaries.
5. Score confidence and freshness, assign review/expiry dates, and request human/security review where policy requires it.
6. Approve, reject, defer, quarantine, or supersede the candidate through a durable decision.
7. Build full-text, metadata, structural, graph, and optional semantic indexes asynchronously.

Indexing is idempotent. Embedding or extraction failure cannot erase the source artifact, mission history, or decision. Successful mission work never silently becomes shared memory.

## Hybrid code retrieval

Authorized retrieval combines complementary sources rather than treating semantic vectors as truth:

1. exact Core governance records and governing artifact versions;
2. exact TaskPacket, current phase/task state, and evidence requirements;
3. repository/path and lexical retrieval;
4. LSP symbols, definitions, references, implementations, call hierarchy, diagnostics, and affected tests;
5. structural AST/tree-sitter relationships;
6. Git history, ADRs, specifications, evidence, and typed relations;
7. scoped verified Engineering, Skill, and performance memory;
8. semantic similarity as a recall aid, always provenance-labelled.

Ranking accounts for authority, exactness, scope, task relevance, freshness, trust, and token cost. Semantic similarity cannot outrank exact policy, an approved artifact, structural evidence, or current diagnostics.

## Context Compiler contract

Hermes's Context Compiler consumes authorized retrieval results through this pipeline:

`Collect → filter → rank → deduplicate → compress → provenance-label → fit budget → validate → send`.

Every context item carries source identifier/hash, trust level, freshness, reason selected, applicable scope, estimated token cost, classification, and governing-version relationship. The compiled pack and selection rationale are reproducible invocation evidence. Brains/builders receive permitted excerpts and references, not an unlimited hidden memory dump.

## Lifecycle, privacy, and retention

Each durable record has classification, owner, access policy, retention class, legal-hold state, confidence where applicable, freshness/review date, expiry, scope, and tombstone/supersession semantics. Contradictory claims remain linked and visible until resolved; the system does not select whichever model sounds more confident.

Embeddings, extracts, summaries, caches, and provider copies are derived data subject to the source artifact's access, deletion, and policy changes. Deleted or revoked content leaves retrieval and propagation queues; audit retains only what policy permits. Retrieval and context selection are logged because they influence autonomous behavior.

## Security model

- Authorization and classification filtering occur before retrieval and again before context egress.
- Untrusted repository content, messages, generated summaries, and community skill material cannot issue instructions or capabilities.
- Secret-bearing content is redacted or withheld according to policy; memory does not become a secret cache.
- Cross-project and cross-organization retrieval is denied unless an explicit policy and scope permit it.
- A compromised semantic index can reduce recall, but cannot change authoritative Core records or authorize an effect.

## V1 scope and evolution

V1 uses SQLite metadata, a content-addressed local artifact store, full-text search, explicit typed links, repository/LSP indexes, and an optional local semantic index. It first proves Working, Mission, Governance, and narrowly scoped Engineering Memory. Cross-project inference, organization-wide memory, graph databases, cloud sync, and automatic shared-skill learning are deferred until evidence and policy justify them.

Cross-device replication, when introduced, replicates governed events and hashed blobs. CRDTs may support collaborative document-like artifacts, never mission, approval, budget, phase, task, effect, or completion truth.

## Acceptance criteria

- Every retrieved item identifies source, authority/trust, scope, freshness, and selection reason.
- Governance retrieval resolves exact current Core versions before derived memory.
- Only evidence-derived, policy-approved candidates become Engineering or Skill Memory.
- Contradiction, expiry, deletion, and supersession change retrieval predictably and audibly.
- Disabling semantic retrieval cannot prevent exact governance, lexical, LSP, structural, and evidence retrieval.

See also: [16_Data_Model.md](16_Data_Model.md), [17_Event_System.md](17_Event_System.md), [21_Project_Backlog.md](21_Project_Backlog.md), [29_Future_Vision.md](29_Future_Vision.md), [31_Governed_Specification_Workflow.md](31_Governed_Specification_Workflow.md), and [32_Hermes_Engineering_Intelligence_Runtime.md](32_Hermes_Engineering_Intelligence_Runtime.md).
