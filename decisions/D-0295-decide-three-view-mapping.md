# D-0295 · Decide three-view mapping of user data across native and personalities
- Status: proposed
- Task: STO-042
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §25, §46, §48
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
One object graph must back native Collections, the Linux home and the Windows profile (§25, §46, §48).

## Options

### Option A · One object graph with two path facades
Summary: One graph, two facades.
Consequences: Single truth; facade work.
Evidence: none

### Option B · Copy-on-first-use
Summary: Copy per view.
Consequences: Simple; divergence.
Evidence: none

### Option C · Adopt-in-place per personality
Summary: Adopt each view.
Consequences: Fast; three models.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
