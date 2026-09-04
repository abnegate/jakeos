# D-0143 · Freeze the Channel Layer 1 ABI Surface
- Status: proposed
- Task: IPC-064
- Surfaces: none
- Layer: none
- Spikes: IPC-017
- Supersedes: none
- Superseded by: none
- Baseline: §65, §66
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
V4 exit requires Layer 1 frozen with the freeze Decision accepted; IPC's amendment covers Channel syscalls, handle-transfer layout and the version header with deprecated entries removed (§65, §66).

## Options

### Option A · Freeze the V1 candidate set as S-012
Summary: Freeze every V1 candidate.
Consequences: Complete contract; all evidence must be in.
Evidence: none

### Option B · Freeze a reduced send/receive/close core
Summary: Freeze only the core.
Consequences: Lower risk; handle transfer stays unstable.
Evidence: none

### Option C · Defer freeze to 1.0
Summary: No freeze at V4.
Consequences: More time; contradicts the V4 exit.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
