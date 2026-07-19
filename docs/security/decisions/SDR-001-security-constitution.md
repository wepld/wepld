# SDR-001 — Security Constitution

- **Status:** Adopted (founder decision, 2026-07-19)
- **Prerequisites:** none (security series root); builds on the merged
  proprietary governance package (GDR-001 through GDR-008)

## Context

WePLD is to become a proprietary, local-first, cross-platform desktop
product operated by humans and untrusted AI agents. Before any product
implementation resumes, the project needs non-negotiable security
principles that survive technology changes, and an honest statement of
what those principles cannot deliver.

## Decision

Adopt `../SECURITY_CONSTITUTION.md`: every action authorized, isolated,
observable, verifiable, and recoverable; ambient authority denied;
fail-closed behavior; minimized Trusted Computing Base; UI, models,
agents, plugins, provider output, imported files, and workspace content
treated as untrusted; explicit scoped expiring revocable capabilities;
secrets kept from UI and model context with no `secret.reveal` in the
standard model; no unvalidated execution of model output; approval
classes for destructive or sensitive effects; tamper-evident evidence
without tamper-impossibility claims; residual risks documented honestly,
including that full OS compromise defeats local confidentiality for
data the device can decrypt.

## Rationale

Principles fixed at the constitutional level prevent the two failure
modes this project's history was built to avoid: authority acquired by
default rather than by decision, and security claimed by technology
label rather than by evidence.

## Consequences

Every future architecture, implementation, and review is measured
against these principles; deviations require recorded decisions; no
marketing or documentation may claim compromise is impossible.

## Legal-review boundary

None required for internal adoption. External security claims made to
customers are a future counsel-review matter under the governance
package's gates.

## Supersession rules

Superseded only by an explicit successor SDR recording a founder
decision and linking here; never edited to retroactively change the
adopted principles.
