# D-0255 · Decide SDK language binding order and milestones
- Status: proposed
- Task: SDK-024
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §50
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The SDK language binding order must be recorded so later binding tasks do not invent a different ladder (§50).

## Options

### Option A · Rust V0, C V1, C++ V2, others V3
Summary: The §50 order is kept.
Consequences: Matches the baseline and the V1 C-binding gate; C++ developers wait.
Evidence: none

### Option B · C++ pulled to V1
Summary: C++ arrives with C at V1.
Consequences: Wider V1 reach; more binding effort before V1.
Evidence: none

### Option C · C delayed to V2
Summary: C arrives at V2.
Consequences: Focus on Rust; the V1 C-binding gate slips.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
