# D-0221 · Decide the on-disk and on-wire Package format and its relation to the store
- Status: proposed
- Task: PKG-012
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §27, §28
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The on-disk and on-wire Package format and its relation to the store must be decided before the first immutable install (§27, §28).

## Options

### Option A · Content-addressed tree with a manifest
Summary: A tree of objects.
Consequences: Dedup; many objects.
Evidence: none

### Option B · Single signed archive unpacking into the store
Summary: One archive.
Consequences: Simple transfer; no dedup on wire.
Evidence: none

### Option C · Hybrid archive that is also a store object
Summary: An archive that is itself an object.
Consequences: Both; complexity.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
