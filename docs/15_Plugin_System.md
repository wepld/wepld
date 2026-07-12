# 15 — Plugin System

## Goal

Everything that extends WePLD—extensions, skills, brains, workers, themes, MCP servers, integrations, and toolchains—uses a common package lifecycle without becoming a path around security, policy, or observability. “Installable” also means inspectable, permissioned, versioned, removable, and revocable.

## Extension types

| Type | Adds | Runs / connects through | Default risk |
| --- | --- | --- | --- |
| Brain adapter | reasoning provider translation | Brain Gateway port | data egress / credentials |
| Worker adapter | execution runtime compatibility | Worker Registry / Worker Host | code execution |
| Skill | reusable method and validation assets | Skill resolver | instruction / tool capability |
| Toolchain | compilers, test tools, formatters | Tool Executor | arbitrary process / supply chain |
| Integration | external service or messaging channel | Integration Gateway | external data / identity |
| MCP server | structured tool/context provider | mediated MCP adapter | tool and data access |
| UI extension / theme | presentation, panels, commands | Studio extension host | UI confusion / data exposure |

## Universal package descriptor

All packages declare identity, publisher, semantic version, integrity hash, signature/provenance, compatibility range, dependency graph, requested capabilities, supported platforms, data handling, license, configuration schema, migration/rollback instructions, health checks, evaluation evidence, release channel, and revocation endpoint. Type-specific metadata augments but does not replace this descriptor.

## Installation lifecycle

1. Discover a package through a local/approved registry.
2. Resolve version and dependency graph without mutating the active environment.
3. Verify signature, hash, publisher policy, license, advisory status, and compatibility.
4. Present requested capabilities and data destinations for approval where required.
5. Install into an isolated versioned store; run declared health/evaluation checks.
6. Atomically activate for selected project/organization scopes and record an event.
7. Monitor health and advisories; support disable, rollback, deprecate, and revoke.

Removal is a dependency-aware transaction. It blocks new use first, drains or cancels affected leases safely, retains historical manifest/hash evidence, then removes executable or sensitive content according to retention policy.

## Runtime isolation

Extensions do not share unrestricted process memory or database access with the Core. UI extensions receive a narrow view model and command API; worker/toolchain plugins execute in isolated hosts; integrations and MCP servers are proxied through policy enforcement; brain adapters access provider credentials only through the Brain Gateway. Capability tokens are scoped by project, task, action, resource, and expiry.

## Versioning and compatibility

The public plugin API is versioned independently of internal modules. Compatibility is checked at install and task-resolution time. A package update cannot silently change approved capability scope; expanded permissions require a new policy decision. Major contract changes use parallel adapter versions and migration windows. Reproducibility requires recording the exact package set, configuration fingerprint, and toolchain identity for every task attempt.

## Marketplace stance

The marketplace is a distribution and trust problem, not merely a catalog UI. V1 ships a local and organization-approved registry. Community publication comes later with publisher verification, automated inspection, signed releases, permission transparency, evaluations, vulnerability disclosure, ratings that cannot override policy, and rapid revocation. An enterprise can mirror/allowlist packages without relying on public infrastructure.

## MCP policy

MCP servers are treated as third-party integrations with typed tools/resources, not automatically trusted local helpers. Each server is versioned, evaluated, permissioned, sandboxed where possible, and mediated so its calls and returned content are logged and subject to prompt-injection/data-classification defenses.

See also: [06_Brain_Architecture.md](06_Brain_Architecture.md), [09_Skills_System.md](09_Skills_System.md), [14_Security_Architecture.md](14_Security_Architecture.md), [18_API_Architecture.md](18_API_Architecture.md), and [28_Release_Strategy.md](28_Release_Strategy.md).

