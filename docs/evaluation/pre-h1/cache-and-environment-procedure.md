# Cache and Environment Procedure — Pre-H1 Baseline EvaluationRun

Mandatory for the future official run; recorded here as protocol
documentation.

1. **Fresh detached checkout** of the exact frozen commit
   (`d5ef318468b6c35df3c14c1c5f72beb1191baf29`) into a new directory; no
   branch is created.
2. **No reused target directory** unless its reuse is explicitly recorded in
   the RunManifest as an environment deviation.
3. **Cache-clear checks:** before the run, verify no stale build caches feed
   the binaries under measurement; record the cache policy used.
4. **Source-tree hygiene:** no unexpected `__pycache__`, `target/`,
   temporary fixture, or cassette artifacts may exist inside the source tree
   before or after the run; the tree must verify clean (`git status` empty)
   at both checkpoints.
5. **Pinned toolchain:** the committed `rust-toolchain.toml` governs; the
   resolved compiler version is recorded.
6. **Locked dependencies:** builds run `--locked` against the committed
   `Cargo.lock`; the lock hash is recorded.
7. **Unix-class environment** (the security boundary is Unix-verified);
   kernel and architecture are captured in the manifest.
8. **Network denied** for the entire run.
9. **One case, one process at a time** — no concurrent case execution.
10. **Active fault seams recorded** per case (hermes failure modes,
    acceptance fault points); a seam firing outside its scripted case is a
    `ProtocolDeviation`.
11. **Clean-tree verification** before and after every case.
12. **All deviations retained** — nothing is cleaned up out of the record.
