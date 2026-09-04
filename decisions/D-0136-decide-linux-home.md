# D-0136 · Decide Linux home adopt-in-place versus copy
- Status: proposed
- Task: INS-023
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §25, §46, §63
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Whether an existing Linux /home can be adopted in place for the personality or must be copied into Collections must be decided (§25, §46, §63).

## Options

### Option A · Copy-only
Summary: Data is copied into native Collections.
Consequences: Clean model; time and disk space for the copy.
Evidence: none

### Option B · Adopt-in-place read-write for the Linux personality
Summary: The personality uses the volume directly.
Consequences: Fast migration; two data models coexist.
Evidence: none

### Option C · Adopt read-only with copy into Collections
Summary: Old data is readable and copied on use.
Consequences: Safe migration; complexity in the view.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
