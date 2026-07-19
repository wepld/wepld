# Confidentiality Classification and Controls

**Status:** internally adopted (GDR-003, 2026-07-19). Internal policy;
not legal advice.

Confidentiality protection comes from controls that are actually
maintained, and — for outside parties — from contracts. Private
repository visibility is an access control and does **not** by itself
create legal protection. Labels do not create secrecy; practices do.
Materials that were previously disclosed may not retain factual secrecy
regardless of classification.

## Classifications

| Class | Meaning | Examples |
| --- | --- | --- |
| Restricted | Highest sensitivity; access strictly by need | credentials, keys, tokens; future customer or regulated data |
| Confidential | Default for WePLD-authored material | source code, prompts, unpublished architecture and evaluation internals, business plans |
| Internal | Operational records, low external sensitivity | process documents, registers, indexes |
| Approved for Disclosure | Explicitly cleared through the publication gate | only material cleared under GDR-007/GDR-008 |

Default classification for repository content: Confidential, per the
`LICENSE` formulation — the repository and its materials are proprietary
and, except where expressly approved for disclosure, confidential.

## Controls

- **Repository access:** Private visibility plus a least-privilege
  collaborator list (currently founder-only). Access grants are per
  person, per need.
- **2FA:** required for every account with repository access.
- **Offboarding:** on any collaborator's departure, revoke access
  immediately; deletion or return of clones becomes a contractual
  obligation once contracts exist (none are drafted here).
- **Local clones and worktrees:** kept on enumerated machines; disk
  encryption recommended; treated at the classification of their
  contents.
- **Cloud-synced folders:** RECORDED RISK — the current working copies
  reside in a cloud-synced folder (OneDrive), which places Confidential
  material on a third-party sync service. This is explicitly recorded as
  a confidentiality risk requiring a future founder disposition decision
  (relocate, exclude from sync, or accept with compensating controls).
  No relocation is performed by this package.
- **Backups:** encrypted and access-controlled; retention deliberate,
  not incidental.
- **Screenshots, logs, and exports:** carry the classification of what
  they show; not shared outside approved channels.
- **AI model-provider submissions:** only to providers approved under
  this policy, per `AI_ASSISTED_DEVELOPMENT.md`; retention or training
  opt-outs applied where offered; customer or regulated data never
  submitted without explicit authorization and contractual basis.
- **Future customer demonstrations:** only material classified Approved
  for Disclosure, through the gates in GDR-007; source is never exposed
  in demonstrations.
- **Incident reporting:** any suspected exposure is recorded with scope
  and disposition, append-only, in the project's honest-failure style.

## Superseded local Package A materials — recorded disposition

The superseded local worktree
`C:/Users/Shehr/OneDrive/Desktop/wepld-pre-h1-evaluation-package-a` and
its local branch `docs/pre-h1-evaluation-protocol-fixture-registry`
(commit `23e1d755fecb624101222a3c87943519b788d056`) are **retained,
untouched internal materials, classification Internal, pending a future
founder disposition decision**. They are not deleted, moved, cleaned,
rewritten, or modified by this package, and nothing in this policy
authorizes doing so. Their eventual disposition is a separate founder
decision.
