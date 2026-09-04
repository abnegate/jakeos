# D-0297 · Decide how user data maps across native, Linux home and Windows profile views
- Status: proposed
- Task: STO-068
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §25, §46, §48
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
INS migration must import an existing Linux home or Windows profile into the live graph without three diverging copies (§25, §46, §48).

## Options

### Option A · Import-as-copy into native Collections
Summary: Copy in.
Consequences: Clean; time.
Evidence: none

### Option B · Adopt-in-place on first personality launch
Summary: Adopt.
Consequences: Fast; divergence.
Evidence: none

### Option C · Dual-write during import
Summary: Dual-write.
Consequences: Transitional; complexity.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
