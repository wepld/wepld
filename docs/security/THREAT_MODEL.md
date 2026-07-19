# WePLD Threat Model

**Status:** adopted with the S0-A package (2026-07-19); maintained as a
living document. Threats are grouped by standing: **present** (apply to
any WePLD build from the first line of product code), **architectural
future** (apply to designed-but-unbuilt components), **deferred CoWork**
(apply only when CoWork exists; recorded now as design constraints), and
**residual** (accepted limits stated honestly).

## Present threats

| Threat | Vector | Constitutional control |
| --- | --- | --- |
| Hostile workspace files | crafted repo/project content triggers unintended reads/writes | all workspace content is untrusted input; confined IO; typed parsers |
| Path traversal and symlinks | `..`, symlinks, hard links escape the workspace | no-follow, capability-scoped filesystem layer; worker path confinement (frozen PR #1 baseline provides verified design precedent, cited as evidence only) |
| Malicious model output | model proposes harmful actions or payloads | typed-action schemas; validation before any execution; approval classes |
| Prompt injection | hostile content in workspace/provider data steers an agent | untrusted-content rule: instructions in data are never authority; capability ceilings cap the damage of a steered agent; sensitive effects require human approval |
| Secret leakage | logs, CLI args, crash dumps, UI state, model prompts | handle-only secrets; scrubbing at logging layer; no-reveal model |
| Supply-chain compromise | malicious crates, npm packages, build scripts, proc-macros, CI actions | lockfiles, `--locked`, SHA-pinned actions (already practiced on main), admission review, build-time execution treated as attack surface |
| Update compromise / downgrade | tampered or replayed artifacts | founder-only Alpha has no updater; signed, version-monotonic updates required before external distribution |
| Hostile localhost / Ollama impersonation | another process binds the expected local port; unauthenticated local APIs | loopback is not identity: endpoint registration, first-contact fingerprinting, warn-on-change, classification gating of content sent locally |
| Cloud-synced folders | OneDrive-class sync exposes confidential material | detection and warning; default workspace locations outside synced dirs (risk already recorded in the governance package) |
| Data loss / ransomware / deletion | accident or malware destroys work | soft delete, retention, snapshots, independent backups, tested restore |

## Architectural future threats (designed components, not yet built)

| Threat | Applies to | Constraint recorded now |
| --- | --- | --- |
| UI/WebView compromise | Tauri/Electron shell | UI holds zero authority; compromise of the WebView must yield only what typed IPC + per-call authorization allow; separate-process core (provisional, S0.5A) |
| IPC abuse | shell ↔ core channel | schema-validated, versioned commands; session identity on every call; authorization in the core, never the shell |
| Worker escape | execution workers | capability-scoped paths, no network by default, kill-on-cancel |
| Provider response abuse | provider gateway | responses are untrusted, size-capped, schema-validated; never executed |
| Local database theft | SQLite store | OS disk encryption assumption stated; sensitive-field encryption; no plaintext secrets in the database |

## Deferred CoWork threat domains

Recorded as future design constraints only; CoWork is not part of S0-A:

- **Tenant isolation failure** — cross-tenant data access in the future
  shared PostgreSQL store; constraint: dual-layer isolation
  (application scoping plus database row-level security) with failure
  modes analyzed before any CoWork implementation.
- **Insider and multi-role abuse** — a user combining roles to bypass
  review; constraint: separation-of-duties rules, explicit-deny
  precedence, and dual control must exist in the CoWork authorization
  design before CoWork Alpha.
- **Offline authority forgery** — an offline client queues operations it
  was never entitled to; constraint: server-side authorization at apply
  time is non-negotiable in the future sync design.

## Residual risks (accepted and stated)

- **Full OS/account compromise defeats local confidentiality** for
  everything the device can decrypt; same-privilege malware can read
  what the user reads and impersonate local services. WePLD preserves
  detectability and bounded damage, not secrecy, under this condition.
- A local ledger can be rewritten by a root-compromised host before it
  is anchored externally; anchoring bounds, but does not eliminate, the
  rewrite window.
- Capability revocation reaches offline devices only at next contact;
  cached policy governs the gap.
- A steered agent operating **within** its granted capabilities can
  still misuse them until a human notices; budgets, small scopes, and
  approval classes bound — do not erase — this risk.
