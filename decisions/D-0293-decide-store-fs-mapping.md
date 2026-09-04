# D-0293 · Decide how the content store maps onto the chosen filesystem without double storage
- Status: proposed
- Task: STO-017
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §27
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The content store must not keep a second copy of Package bytes on the chosen filesystem (§27).

## Options

### Option A · Hardlinked object directory
Summary: Hardlinks into an object directory.
Consequences: Simple; inode sharing.
Evidence: none

### Option B · Reflinked files
Summary: Reflinks.
Consequences: Flexible; filesystem support.
Evidence: none

### Option C · Filesystem-native dedup
Summary: Native dedup.
Consequences: Transparent; cost.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
