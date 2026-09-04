# D-0218 · Decide that Package mutation is replaced by immutable Packages and SystemGenerations
- Status: proposed
- Task: PKG-009
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §2, §28, §30, §67
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The §2 replacement of package mutation by immutable Packages and SystemGenerations must be recorded as standing rules the immutability gate enforces (§2, §28, §30, §67).

## Options

### Option A · Immutable Packages plus SystemGenerations
Summary: No in-place mutation, ever.
Consequences: Rollback and reproducibility; no in-place edits.
Evidence: none

### Option B · In-place Package mutation with snapshots
Summary: Mutate with snapshots.
Consequences: Familiar; drift.
Evidence: none

### Option C · Hybrid writable overlay on immutable bases
Summary: Writable overlays.
Consequences: Flexibility; complexity.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
