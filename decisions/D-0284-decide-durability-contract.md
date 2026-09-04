# D-0284 · Decide when Write and StorageTransaction data is power-loss safe
- Status: proposed
- Task: STO-038
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §18, §26
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The point at which Write and StorageTransaction data is power-loss safe must be defined (§18, §26).

## Options

### Option A · fsync-on-commit
Summary: Every commit syncs.
Consequences: Simple contract; latency.
Evidence: none

### Option B · Group-commit with a bounded window
Summary: Commits are grouped.
Consequences: Throughput; a bounded window of loss.
Evidence: none

### Option C · Explicit Durable flag on Write
Summary: A flag requests durability.
Consequences: Control; application burden.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
