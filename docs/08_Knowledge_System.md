# 08 — Knowledge System

## Purpose

WePLD’s knowledge system is evidence-backed organizational memory. It answers not only “what was said?” but “what changed, why, who or what established it, which version was true, how confident are we, and may it still be used?” Chat transcripts may be ingested as sources, but they are never the knowledge model.

## Knowledge graph model

The V1 graph is implemented as typed relational records plus immutable artifacts and indexes. This is simpler to govern, back up, query, and evolve than introducing a graph database before the access patterns justify it.

| Node | Examples | Required provenance |
| --- | --- | --- |
| Artifact | design doc, diff, log, benchmark, scan report, screenshot | content hash, source, creator, timestamp, classification |
| Claim | “API latency is below target” | source artifact(s), claimant, confidence, freshness, status |
| Decision | selected storage architecture | options, authority, rationale, evidence, supersession link |
| Pattern | retry/idempotency convention | scope, examples, validation evidence, owner |
| Lesson | prior failure and prevention | trigger, evidence, applicability, review date |
| Finding | bug, security issue, performance regression | severity, lifecycle, affected version, evidence |
| Mission / Task / Run | operational history | immutable event links, owner, outcome |
| Entity / Relation | module depends on service; claim supports decision | relation type, source, confidence, validity interval |

## Ingestion pipeline

1. Capture a completed artifact or event reference from an owning context.
2. Classify, hash, deduplicate, and attach access/retention metadata.
3. Extract typed candidates such as claims, decisions, findings, patterns, and links; retain source spans or artifact anchors.
4. Validate required provenance and policy. Uncertain extraction remains a candidate, not canonical knowledge.
5. Build full-text, metadata, structural, and optional semantic/vector indexes.
6. Publish a retrieval-ready record or queue a review where policy requires it.

Indexing is asynchronous and idempotent. A failed embedding service cannot make the source artifact or mission history disappear.

## Retrieval contract

Retrieval begins with authorization and query intent, then combines structured filters, graph traversal, full-text search, and semantic similarity where permitted. Results include citations, source version/hash, confidence, freshness, classification, and reasons for ranking. A brain or worker receives references and permitted excerpts, not an unlimited hidden memory dump.

Retrieval is logged because context selection can affect autonomous behavior. A stale, superseded, revoked, or unauthorized record must be excluded or visibly labeled.

## Lifecycle, privacy, and retention

“Knowledge accumulates forever” means learning should not be casually discarded; it does not override deletion rights, contractual retention, export controls, or incident response. Each record has classification, retention class, owner, legal hold status, access policy, review/freshness date, and tombstone/supersession semantics. Deleted content is removed from retrieval and propagation queues; the audit record preserves only what policy permits.

Embeddings are derived data. If an artifact’s policy changes or it is removed, its embeddings, extracts, caches, and outbound provider copies are subject to the same deletion workflow.

## Quality controls

- Claims must cite evidence and identify uncertainty.
- Decisions identify alternatives and are never silently overwritten; a later decision supersedes them.
- Lessons have applicability boundaries and review dates to avoid cargo-cult reuse.
- Knowledge suggestions from workers must pass provenance and policy validation before becoming shared records.
- Human corrections create a new versioned record and preserve the correction rationale.

## V1 scope and evolution

V1 uses SQLite metadata, a content-addressed local artifact store, full-text search, explicit typed links, and an optional local semantic index. Graph databases, global organization memory, cross-project inference, and cloud sync are deferred until query volume and collaboration prove their value. Cross-device replication should replicate events and hashed blobs; CRDTs are appropriate only for collaborative document-like artifacts, not task state.

See also: [16_Data_Model.md](16_Data_Model.md), [17_Event_System.md](17_Event_System.md), [21_Project_Backlog.md](21_Project_Backlog.md), and [29_Future_Vision.md](29_Future_Vision.md).

