# WePLD Security Constitution

**Status:** adopted (SDR-001, founder decision, 2026-07-19). Constitutional
document: it states principles and red lines, not implementation detail.
Nothing in it authorizes implementation.

## Security mission

Make compromise of WePLD exceptionally difficult, minimize the damage
radius when compromise occurs, make unauthorized actions detectable, and
make recovery fast and verifiable. WePLD does not claim to be impossible
to hack, and no technology choice — Rust, Tauri, encryption, sandboxing,
or private repository visibility — is treated as proof of security by
itself.

## Constitutional principles

Every action must be **authorized, isolated, observable, verifiable, and
recoverable**:

1. **Authorized** — every effect requires an explicit, scoped, expiring,
   revocable capability. There is no ambient authority anywhere in the
   product.
2. **Isolated** — every actor operates in the smallest scope that serves
   its task: process boundaries, path confinement, network destination
   pinning, budget limits.
3. **Observable** — every sensitive action emits a typed event to a
   tamper-evident ledger, attributing actor, identity type, capability,
   policy decision, and approval.
4. **Verifiable** — claims rest on evidence: signed artifacts, content
   hashes, chained audit records, reproducible checks. Assertion is not
   evidence.
5. **Recoverable** — destructive effects prefer soft deletion, retention
   windows, snapshots, and tested restore paths over irreversibility.

## Trust minimization and fail-closed behavior

- The Trusted Computing Base is kept as small as auditable practice
  allows (`TRUSTED_COMPUTING_BASE.md`).
- The UI, models, agents, plugins, provider output, imported files, and
  workspace content are **untrusted** at every read.
- When a policy check, capability lookup, signature verification, or
  identity binding cannot complete, the affected operation **fails
  closed**: no effect, an honest error, and an evidence record.

## Non-negotiable red lines

- No ambient authority; no component receives unrestricted filesystem,
  shell, network, secret, or database access.
- The UI holds zero authority and never directly controls filesystem,
  shell, network, secrets, databases, policy, capabilities, ledger,
  providers, updates, or agent execution.
- No secret value crosses into UI state, model prompts or context, CLI
  arguments, logs, crash reports, or plaintext local storage. Secrets
  are handles; rotation replaces display (`secret.reveal` is absent from
  the standard permission model).
- No model or agent output executes without schema validation and a
  policy decision.
- Destructive or sensitive effects require their approval class; agents
  never hold merge, release, membership, permission, or permanent-delete
  authority.
- No unsigned or version-downgraded update is ever installed (see the
  update principles below for the founder-only Alpha exception, which
  removes the updater entirely rather than weakening verification).
- Audit evidence is append-only and tamper-evident; no actor, including
  the highest authority, may silently rewrite it.

## Secret boundaries

Secret material lives only behind the secret mediator (operating-system
keychain or an approved vault). Components receive handles bound to a
purpose; the mediator injects values only at the final protocol boundary
(for example, a provider HTTP header) and never returns them to callers.

## Agent boundaries

Hermes and all external agents are untrusted actors. They may observe,
propose plans, and emit typed actions; every effectful action passes
through: Work Contract (or approved template) → capability evaluation →
applicable approval → constrained execution → verification → evidence
receipt. See `CAPABILITY_MODEL.md`.

## Update and dependency principles (constitutional level)

- Builds are tied to known commits with recorded hashes and build
  evidence.
- The founder-only Personal Alpha ships **no automatic updater**.
- Platform code signing is mandatory before any build leaves
  founder-controlled devices; secure update verification is mandatory
  before external evaluation; key-rotation and TUF-style hardening are
  mandatory before customer distribution. These thresholds are never
  weakened for externally distributed builds.
- Dependencies enter only through the admission process of the
  Technology Constitution and the proprietary governance package's
  third-party dependency policy.

## Honest residual risk

Tamper-evident is not tamper-impossible. Controls raise attacker cost
and shrink blast radius; they do not eliminate compromise. In
particular: **full compromise of the user's operating system or account
defeats local confidentiality for any data that device can decrypt** —
a rooted device can read what the user can read and impersonate what the
user can run. WePLD's design aims to preserve detectability (anchored
evidence) and bounded damage under such compromise, not to deny that the
limit exists. Residual risks are recorded per threat in
`THREAT_MODEL.md` and are reported honestly rather than marketed away.
