# D-0068 · Decide the native Component spawn primitive that replaces fork and exec
- Status: proposed
- Task: CMP-009
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §2, §10, §53
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
fork has no native equivalent, so the native Component spawn primitive must be chosen and Unix process startup recorded as never the native creation mechanism (§2, §10, §53); S-007 is prototyped.

## Options

### Option A · Spawn from immutable code object
Summary: A Component is created from an immutable, verified code object plus initial Capabilities.
Consequences: Clean and auditable; no cheap copy-of-self pattern.
Evidence: none

### Option B · Template clone
Summary: A prewarmed template Component is cloned.
Consequences: Fast creation; template state must be provably neutral.
Evidence: none

### Option C · Builder object then start
Summary: A builder object accumulates configuration and is started atomically.
Consequences: Rich, typed configuration; two-phase creation to get right.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
