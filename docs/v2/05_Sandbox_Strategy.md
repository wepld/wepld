# v2-05 — Sandbox Strategy (honest, per-OS)

**Premise (ADR-0004, ADR-0007):** development toolchains execute arbitrary code by design (`npm install` postinstall, `build.rs`, test suites). Therefore the security unit is the **envelope** — what a whole phase may touch — enforced by the operating system, not per-command adjudication and not worker cooperation. Platforms differ materially; WePLD states the difference instead of averaging it.

## The envelope

Issued by Core per attempt/phase; a signed-in-spawn, expiring grant:

~~~json
{
  "envelope_id": "env_01J...",
  "attempt_id": "att_01J...",
  "sandbox_tier": "S1",
  "fs": {
    "write": ["<worktree>"],
    "read":  ["<worktree>", "<toolchain paths>", "<declared read-only deps>"],
    "deny":  ["$HOME", "app-data", "keychains", "everything else by default"]
  },
  "network": { "mode": "deny" },
  "process": { "max_procs": 64, "max_mem_mb": 4096, "cpu_share": 0.5, "timeout_s": 1800 },
  "secrets": [],
  "expires_at": "..."
}
~~~

Hard-gate crossings (`envelope.extend`): network egress (with destination class), dependency installation, paths outside the worktree, any secret, protected-branch effects, destructive operations. In Bounded-Auto, extensions inside the *mission-declared* envelope auto-approve with a ledger record; outside it, a decision packet.

## Tiers

### S1 — Linux native (reference platform): **strong**

Implementation: unprivileged user namespaces + mount namespace (bind worktree + read-only toolchain, nothing else), PID and network namespaces (no interfaces in deny mode; slirp/proxy when granted), cgroups v2 (memory/CPU/pids), Landlock for file-path allowlisting on kernels ≥ 5.13, optional seccomp baseline filter. Practical composition: `bubblewrap`-class launcher owned by Core. Guarantees: filesystem and network containment are kernel-enforced; escape requires a kernel/namespace vulnerability.

### S2 — macOS native: **strong filesystem, best-effort network**

Implementation: Seatbelt profile via `sandbox-exec` (deprecated by Apple yet used by Apple's own build system and by Bazel/Nix — a monitored risk, with S0 as fallback), generated per envelope: file-read/write scoped to the envelope's paths, `network-outbound` denied unless granted. Resource limits via `rlimit`/task policies. Honest statement: Seatbelt's network rules cover common cases but macOS offers no cgroup-equivalent or airtight per-process egress control without a system extension (out of MVP scope); network deny is *enforced primarily by Seatbelt and additionally routed through Core's local proxy* — labelled **best-effort** in the ledger. Tier cap: network extensions always human-gated even in Bounded-Auto.

### S2W — Windows via WSL2: **strong** (recommended Windows path)

Core (Windows) drives a worker inside a dedicated WSL2 distro using the full S1 stack. Worktrees live on the Linux filesystem (also dodging NTFS/OneDrive performance and locking pathologies). Detection at mission start; one-time guided setup. This is the honest way to give Windows users real containment without pretending Win32 provides it.

### S3 — Windows native fallback: **moderate filesystem, weak network**

Implementation: dedicated low-privilege local account or restricted token (deny-only SIDs, removed privileges), explicit ACLs granting that identity access to the worktree and toolchain only, Job Object for memory/CPU/process-count/kill-on-close. No practical syscall filtering; AppContainer rejected (breaks mainstream toolchains). Network: WFP-based per-process filtering is possible but fragile — MVP declares network control **weak** on S3 rather than shipping a half-measure. Tier cap: Bounded-Auto restricted — network use, dependency installation, and test execution of untrusted repositories all require explicit approval; the Studio explains why.

### S0 — Container backend (any OS): **strongest, opt-in**

Rootless Podman/Docker when present: per-attempt container, worktree bind-mount, `--network=none` default, resource flags. Not a hard dependency (footprint principle) but the recommended posture for security-sensitive users and the fallback if a native tier degrades (e.g., future Seatbelt removal).

## Tier mechanics

- **Detection:** at mission start Core probes (kernel version/Landlock, `sandbox-exec` presence, WSL2, container runtime) and runs a **self-test**: a canary process inside a candidate envelope attempts forbidden reads/writes/egress; the tier is *confirmed by observed denial*, not by feature flags. Result → `SandboxTierDetected` ledger entry.
- **Disclosure:** Mission surface shows the tier and its one-line honest statement; the mission brief records it; changing tier mid-mission is impossible (new mission).
- **Autonomy binding:** the envelope generator consults the tier cap table (ADR-0007). Policy weakens *autonomy*, never *claims*.
- **Observability inside the envelope:** commands executed by the phase are logged (shell wrapper) as **observability, explicitly not enforcement** — useful for timeline and debugging; the security statement never rests on it.

## What this does not claim

No tier defends against a hostile kernel, malicious toolchain binaries already trusted by the user, or hardware side channels. S2/S3 network statements are printed on the tin. The gate review's C2 standard — "security claims must match enforcement reality" — is met by making the reality the interface.
