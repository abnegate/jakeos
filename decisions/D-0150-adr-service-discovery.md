# D-0150 · Decide service naming and discovery: kernel-held directory or user-space broker
- Status: proposed
- Task: IPC-023
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §32, §14
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Clients must rebind by Interface identity across restarts for the V0.5 compositor crash-recovery Gate (§32, §14), decided with SVC supervision.

## Options

### Option A · Kernel-held directory of Interface identities
Summary: The kernel keeps the directory.
Consequences: Survives userspace crashes; kernel grows naming.
Evidence: none

### Option B · User-space broker Component
Summary: A broker resolves identities.
Consequences: Kernel minimal; broker restart to handle.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
