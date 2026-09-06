# D-0130 · Decide policy for third-party user-space drivers and firmware packages
- Status: proposed
- Task: HW-082
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §55, §62
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Vendors and community porters need a way to ship user-space drivers and firmware Packages without forking the OS (§55, §62). This V4 decision fixes the channel: signed Packages with review and revocation, in-tree only, or unrestricted user-space loaders, and how out-of-tree drivers appear in the Hardware Compatibility List. It sits on the user-space driver hosting model (HW-056) and the probe data model (HW-047).

## Options

### Option A · Signed Packages with review and revocation
Summary: Third-party user-space drivers and firmware ship as Packages signed by an attested publisher (D-0237), reviewed for the device classes they claim, revocable through the repository, and listed in the HCL as third-party entries with their probe signatures.
Consequences: A safe channel that keeps the OS unforked and lets the HCL show exactly what a machine runs. Review load on the project for every driver Package, and revocation must reach installed machines through the update path.
Evidence: none

### Option B · In-tree only
Summary: Only in-tree drivers exist; vendors upstream to the platform repository or to Linux.
Consequences: Every driver meets the project's quality bar and licence rules. Vendors who cannot or will not upstream fork the OS or leave, and hardware support for niche devices lags by a release cycle.
Evidence: none

### Option C · Unrestricted user-space loaders
Summary: Any user-space driver Package may be loaded by a user who grants it the device Capability.
Consequences: Maximum flexibility and no review queue. No signing, review or revocation, so a malicious driver Package is one grant away from DMA-capable hardware (T-043 class); rejected.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
