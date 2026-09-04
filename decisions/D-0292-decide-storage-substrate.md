# D-0292 · Decide the initial Linux filesystem under the native storage layer
- Status: proposed
- Task: STO-016
- Surfaces: none
- Layer: none
- Spikes: STO-026
- Supersedes: none
- Superseded by: none
- Baseline: §26, §57
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
V0.5 requires an accepted filesystem-choice ADR against the nine §26 properties (§26, §57); the spike is the evidence and no native filesystem is built before 1.0.

## Options

### Option A · btrfs
Summary: btrfs is the substrate.
Consequences: Snapshots, reflink and checksums; reputation.
Evidence: none

### Option B · bcachefs
Summary: bcachefs is the substrate.
Consequences: Modern design; maturity.
Evidence: none

### Option C · XFS with reflink
Summary: XFS is the substrate.
Consequences: Mature and fast; no snapshots.
Evidence: none

### Option D · ZFS
Summary: ZFS is the substrate.
Consequences: Excellent features; rejected on CDDL and I-067.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
