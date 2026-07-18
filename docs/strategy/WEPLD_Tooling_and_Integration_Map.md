# WePLD Tooling and Integration Map

**Standing:** planning only; no integration is implemented or authorized here.
Rule for every row: WePLD's contracts and authority survive even when an
external tool performs the mechanics; all integrations sit behind adapters,
capability manifests, and the Effect Firewall.

## Build / wrap / integrate / standard / defer / reject

| Category | Decision | Reasoning |
| --- | --- | --- |
| Git | Integrate (local Git is the substrate) | never rebuild; safe-operation wrappers only (PR #1 pattern) |
| GitHub / GitLab / Bitbucket | Wrap behind an adapter | forges are effects; PRs/issues through the firewall |
| LSP servers (rust-analyzer first) | Integrate behind the LSP Broker | mature; language-neutral broker is ours |
| Compiler diagnostics | Integrate | cargo/clippy-class output normalized to typed evidence |
| Test runners | Integrate | cargo test/nextest-class; the Lab orchestrates, never re-implements |
| Static analyzers | Integrate | per-language, SARIF-normalized |
| Secret scanners | Integrate | gitleaks-class behind an inspector adapter |
| Vulnerability scanners | Integrate | cargo-audit/OSV-class |
| SBOM + provenance | Adopt open standards | SPDX/CycloneDX; SLSA-class provenance |
| SARIF | Adopt open standard | findings interchange |
| OpenAPI + JSON Schema | Adopt open standards | already in use (PR #1 schemas) |
| MCP | Adopt where appropriate | as a **tool surface behind capability manifests**; never an alternate control plane |
| ACP | Defer to Stage 9 | editor interop after Mission Control proves itself |
| Container runtimes (Docker) | Integrate | sandbox runner class |
| Kubernetes / VM / enterprise runners | Integrate later (Stage 8) | behind the runner contract |
| CI platforms (GitHub Actions/GitLab CI/Jenkins) | Wrap behind an adapter | CI results are evidence inputs, not authority |
| Issue systems (Jira/Linear/GitHub Issues) | Wrap behind an adapter | two-way sync is an effect; no automatic backlog mutation |
| Communication (Slack/Teams/email/webhooks) | Wrap behind an adapter | notify-only first; commands require Core authentication |
| Observability systems | Integrate (Stage 7) | redaction before any model exposure |
| Identity providers | Integrate (Stage 8) | OIDC-class; Core remains the authorization authority |
| Billing | Integrate | Stripe-class dedicated provider; never WooCommerce as billing authority |
| Artifact storage | Integrate | S3-class behind the artifact-store contract; CAS semantics ours |
| Model provider APIs | Wrap behind gateway adapters | OpenAI-/Anthropic-/Kimi-/Gemini-compatible families |
| Local model runtimes | Integrate | llama.cpp/vLLM-class behind the same gateway |
| External coding agents | Conditional (Stage 6+, Research first) | as governed workers via the Engineering Worker Protocol only |
| WordPress | Integrate (marketing/content only, if operationally useful) | never Core, never Studio |
| WooCommerce | Defer (conditional marketplace evaluation, Stage 9) | rejected for mission state, billing authority, engineering truth |

## Letta / MemGPT study

**What it is:** MemGPT introduced hierarchical/virtual context (an agent that
edits its own bounded memory and pages to archival storage); Letta is its
productized stateful-agent runtime — persistent agents, memory blocks,
archival memory, Letta Code, AgentFile portability, local/cloud deployment,
and continual-learning claims with evaluation tooling.

**What WePLD takes now (concepts, not code):** explicit memory-block typing,
paged context discipline, and portability-file thinking inform the Context
Compiler, governed Engineering Memory, and profile portability design.

**Required boundaries (all hold in every disposition):** Letta is not required
for V0; it does not replace Hermes or Core; Letta memory does not become
Engineering Memory automatically — it enters as a `MemoryCandidate` for the
Memory Judge; Letta-derived procedures enter as `SkillCandidate`s; any Letta
runtime sits behind the Universal Agent Gateway; shared writable Letta memory
cannot be authoritative project state and cannot be shared between independent
Consulting or Committee roles; Committee membership requires a frozen,
hash-bound memory snapshot (doc 36 pack immutability); cloud Letta obeys
data-egress policy; local Letta obeys sandbox and Effect Firewall rules.

**Candidate uses (each individually evaluated):** long-lived advisory
Mastermind profile; persistent Researcher; Wisdom profile; Committee member;
personal engineering assistant; continual-learning experimental arm; AgentFile
import/export adapter; benchmark target in the Arena.

**Comparison arms (EV-S11/EV-S12 in the evaluation programme):**
A stateless model · B Hermes with bounded memory · C Letta self-editing
memory · D `MemoryCandidate` + Memory Judge · E governed memory + certified
Skills — measured on repeated-task improvement, unsupported claims, privacy
violations, and correction rates.

**Proposed disposition:** adopt concepts now; **optional external
runtime/worker adapter behind the gateway later (Conditional, Stage 6+, own
ADR candidate ADR-0037)**; AgentFile as an import/export portability adapter
only — AgentFile is not automatically WePLD's native profile format; reject
Letta as Core/Hermes replacement or as authoritative memory. No Letta
integration is implemented by this package.

## Commercial and public web surfaces

Studio is a custom application. Core is not implemented in WordPress.
WordPress may host marketing, blog, or public content only if operationally
useful. WooCommerce is not a Core dependency; it may be evaluated later for a
simple marketplace storefront only — never for mission state, billing
authority, or engineering truth. SaaS subscriptions use a dedicated
subscription-billing provider abstraction (Stripe-class). Provider usage
metering and WePLD product entitlement remain separate concerns. Marketing,
application, API, SkillHouse, and documentation surfaces remain separable.
