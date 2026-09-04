# D-0212 · Decide whether application-state restore is a 1.0 goal or non-goal
- Status: proposed
- Task: PKG-069
- Surfaces: none
- Layer: none
- Spikes: PKG-079
- Supersedes: none
- Superseded by: none
- Baseline: §31
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Whether application-state restore is a 1.0 goal or non-goal must be settled before V4 feature freeze using the spike (§31), answering Q-056.

## Options

### Option A · In-scope via checkpointing
Summary: Processes are checkpointed.
Consequences: Transparent to apps; hard for GPU and network state.
Evidence: none

### Option B · In-scope via cooperative state interfaces
Summary: Apps implement state interfaces.
Consequences: Feasible; app work.
Evidence: none

### Option C · Explicit 1.0 non-goal
Summary: Out of scope.
Consequences: Focus; a gap to name.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
