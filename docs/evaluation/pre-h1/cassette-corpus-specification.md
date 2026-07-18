# Cassette-Corpus Specification (planned; not generated in Package A)

Defines the committed, path-independent cassette corpora
(`cc-corpus-spec-flow`, `cc-corpus-build-edits`) that will replace the
ephemeral runtime cassettes as frozen evaluation inputs. **This package does
not implement cassette generation and does not copy any ephemeral test
output into the registry.**

## Requirements

- No temporary absolute paths anywhere in keys or payloads.
- No usernames or machine identifiers.
- Repository identity is represented by a stable placeholder or the fixture
  id (`fx-notes-cli`), never a filesystem path; instantiation paths are
  substituted at run time outside the hashed identity.
- Normalized ContextPack identity: the pack fields entering the lookup hash
  are canonicalized (fixture id in place of repo path, sorted keys, fixed
  schema version) so identical logical inputs hash identically on every
  machine.
- Content-addressed payloads: each response payload carries its own SHA-256.
- Registry-bound corpus version and hash: a corpus is referenced only through
  its `fixture-registry.yaml` entry.
- Synthetic payloads only; no credentials; no live network calls at
  generation or replay.
- Deterministic serialization with canonical ordering (sorted JSON keys,
  fixed separators, trailing-newline policy stated in the corpus header).
- Explicit corpus schema version.
- Qualified corpora are immutable; any modification creates a new version
  with a supersession link — never an in-place edit.
