# D-0275 · Decide disk-key eviction on suspend
- Status: proposed
- Task: SEC-031
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §51, §61
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
V1 ships suspend and resume, so disk-key eviction on suspend must be decided (§51, §61).

## Options

### Option A · Evict keys from RAM on suspend and re-unlock on resume
Summary: Keys are evicted.
Consequences: Safe; re-unlock friction.
Evidence: none

### Option B · Keep keys in RAM while locked
Summary: Keys stay.
Consequences: Convenient; cold-boot risk.
Evidence: none

### Option C · Suspend-then-hibernate
Summary: Suspend then hibernate.
Consequences: Battery; image handling.
Evidence: none

### Option D · Forbid hibernate under lockdown
Summary: Hibernate is forbidden.
Consequences: Simple; no hibernate.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
