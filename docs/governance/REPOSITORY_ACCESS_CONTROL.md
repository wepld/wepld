# Repository Access-Control Policy

**Status:** internally adopted (GDR-003, 2026-07-19). Lightweight
internal policy; not legal advice.

## Rules

1. **Visibility:** the repository is Private. Visibility changes are a
   founder decision recorded in a governance record — never an incidental
   act. (Private visibility is an access control, not a legal protection
   by itself; see `CONFIDENTIALITY.md`.)
2. **Least privilege:** collaborator access is granted per person, per
   need, at the lowest sufficient permission level. Current collaborator
   set: the founder only.
3. **2FA:** required for all accounts with access.
4. **No agent accounts:** AI coding agents hold no repository access of
   their own; they operate only under a responsible human's
   authenticated session (see `AI_ASSISTED_DEVELOPMENT.md`).
5. **Protected refs:** the canonical main branch advances only through
   reviewed pull requests; history rewrite and force-push to shared
   branches are prohibited; protected pull requests (currently Draft
   PR #1) are modified only under explicit founder authorization.
6. **Branch hygiene:** new work happens on task-scoped branches created
   from verified canonical state; deleted remote branches are not
   restored or reused.
7. **Access review:** the collaborator list, deploy keys, personal access
   tokens, and third-party app installations are reviewed whenever
   membership changes and at least periodically.
8. **Offboarding:** departure of any collaborator triggers immediate
   revocation per `CONFIDENTIALITY.md`.
9. **Before any external party receives access** (contractor, auditor,
   evaluator), the deferred legal instruments in GDR-006/GDR-007 must
   exist and be executed — access without them is prohibited.
