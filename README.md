# WePLD Architecture & Master Plan

**Status:** architecture complete; implementation deliberately not started.

This repository is the planning baseline for WePLD, an Autonomous Software Engineering Operating System. The mandated architecture documents live in [docs](docs/). They define the product before any production code, build tooling, or vendor commitment is introduced.

## Reading order

1. Start with [30_ARCHITECTURE_SUMMARY.md](docs/30_ARCHITECTURE_SUMMARY.md) for the executive view.
2. Read [01_Project_Vision.md](docs/01_Project_Vision.md), [03_System_Architecture.md](docs/03_System_Architecture.md), and [19_Implementation_Roadmap.md](docs/19_Implementation_Roadmap.md) to understand the intended product and sequence.
3. Treat [16_Data_Model.md](docs/16_Data_Model.md), [17_Event_System.md](docs/17_Event_System.md), [18_API_Architecture.md](docs/18_API_Architecture.md), and [14_Security_Architecture.md](docs/14_Security_Architecture.md) as normative foundation contracts.

## Scope boundary

Nothing in this repository is an implementation. Technology choices are architectural recommendations subject to the exit criteria in the roadmap. Production implementation begins only after the document set is reviewed, any open strategic decisions are resolved, and the architecture gate is explicitly approved.

## Source-of-truth rules

- The documents in this repository are the current product and architecture source of truth.
- A future implementation must preserve the stated boundaries or record an Architecture Decision Record (ADR) explaining a change.
- The directory named `WePLD` elsewhere on the Desktop is not part of this project and was not used or changed.

