# D-0216 · Decide what is excluded from a SystemGeneration and how mutable state is separated
- Status: proposed
- Task: PKG-007
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §30, §31
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
What is excluded from a SystemGeneration and how mutable state is separated must be fixed so rollback never rewrites user data (§30, §31), answering Q-023.

## Options

### Option A · Exclude user data, ApplicationData, logs and caches
Summary: These classes live outside the generation.
Consequences: Clean rollback; app state unmanaged.
Evidence: none

### Option B · Snapshot selected mutable trees into the generation
Summary: Selected trees are snapshotted.
Consequences: Restorable; bloat.
Evidence: none

### Option C · Hybrid with explicit restorable classes
Summary: Named classes are restorable.
Consequences: Precise; classification.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
