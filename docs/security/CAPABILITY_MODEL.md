# WePLD Capability and Policy Model

**Status:** adopted (SDR-003, founder decision, 2026-07-19).

## Capability schema

A capability is an explicit, scoped, expiring, revocable grant of one
kind of authority to one actor. Canonical fields:

```text
capability_id     unique, unforgeable identifier
actor_id          who may use it
identity_type     Human | Agent | Service | ExternalIntegration | Device
action            one permission (e.g. write.files, net.connect)
resource          what it applies to
scope             path globs / object ids / provider ids / destinations
constraints       size, count, request-class, protocol, port limits
classification    highest data classification the action may touch
budget            tokens, money, invocations, bytes (as applicable)
expires_at        hard expiry; short by default
binding           session, task, or run the grant is tethered to
approval_ref      approval record that authorized issuance, if required
revocation        revocation status and reference
audit_level       evidence detail recorded on use
```

Use, denial, issuance, expiry, and revocation each emit a ledger event.
Revocation takes effect at the next policy check — checks happen per
operation, not per session.

## Policy precedence

```text
Emergency controls
→ Organization policy
→ Explicit deny
→ Resource policy
→ Approval requirements
→ Temporary capabilities
→ Role grants
```

Rules: **nothing overrides an active emergency freeze**; an explicit
deny defeats every grant below it; approval requirements attach before
any grant satisfies a request; role grants are the weakest source of
authority. Evaluation is deterministic — the same inputs always produce
the same decision — and produces an explanation record naming the
winning rule, so a user interface can always answer "why was this
allowed or denied."

## No ambient authority

An actor holding no capabilities can cause no effects. There are no
default filesystem, shell, network, secret, database, or provider
rights for any component, human, or agent. The UI holds zero authority
categorically.

## Work Contracts

Every effectful agent run (and sensitive human-initiated automation)
executes under a Work Contract: goal, scope, allowed and prohibited
paths, tools, provider, budgets, time limit, network policy, secret
access (handles only), required evidence, acceptance criteria, review
requirements, reversibility, damage-radius estimate, and completion
definition. Low-risk classes may use founder-approved templates
instantiated per run; sensitive classes require explicit contracts.
Contract → capability evaluation → applicable approval → constrained
execution → verification → evidence receipt is the mandatory pipeline.

## Example capabilities (illustrative)

```text
{cap: read.files,   actor: agent:hermes-run-42, scope: ["src/**"], expires: +2h}
{cap: write.files,  actor: agent:hermes-run-42, scope: ["src/feature-x/**"],
                    deny: ["**/.git/**"], max_bytes: 5MB, binding: run-42}
{cap: net.connect,  actor: core.providers, dest: "127.0.0.1:11434",
                    proto: http, purpose: ollama}
{cap: net.connect,  actor: core.providers, dest: "api.example-provider.com:443",
                    proto: https, classification: Confidential, budget: $2.00}
{cap: exec.tests,   actor: worker-7, cmd_class: "cargo test --locked",
                    cwd: worktree-42, wall: 15m, net: deny}
{cap: secret.use,   actor: core.providers, handle: kc://provider-key,
                    reveal: never, binding: provider-call}
{cap: export.artifact, actor: user:founder, object: run-42/report,
                    approval_ref: apr-118}
{cap: delete.file,  actor: user:founder, scope: ["notes/draft.md"],
                    mode: soft, restore_window: 30d}
```

`secret.reveal` does not exist in the standard model; secrets are
handles, and rotation replaces display.

## Approvals, freeze, and evidence

Approval classes (none / one human / elevated / dual / impossible-by-
policy) attach to permissions by policy; an approval is itself a ledger
record referenced by the capabilities it authorizes. The emergency
freeze suspends all capability use except freeze-management itself, and
the freeze action is indelibly recorded before it takes effect. Every
decision — allow, deny, freeze, override — leaves evidence sufficient
to reconstruct who did what, under which authority, and why.
