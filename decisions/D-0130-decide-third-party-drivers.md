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
Vendors and community porters need a driver channel without forking the OS (§55, §62).

## Options

### Option A · Signed Packages with review and revocation
Summary: Third-party drivers ship as reviewed, signed Packages that can be revoked.
Consequences: A safe channel with an HCL entry per driver; review load on the project.
Evidence: none

### Option B · In-tree only
Summary: No out-of-tree drivers exist.
Consequences: Quality control; vendors must upstream or fork.
Evidence: none

### Option C · Unrestricted user-space loaders
Summary: Anyone can load a user-space driver.
Consequences: Maximum flexibility; no review, signing or revocation.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
