# D-0288 · Decide persistent MemoryObject semantics: storage backing, crash consistency, content addressing
- Status: proposed
- Task: STO-040
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §16, §26, §27
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Persistent MemoryObject semantics must be answered before the persistent property ships (§16, §26, §27).

## Options

### Option A · File-backed CoW
Summary: File-backed copy-on-write.
Consequences: Simple; not content-addressed.
Evidence: none

### Option B · Content-addressed Blob
Summary: A Blob.
Consequences: Dedup; write path.
Evidence: none

### Option C · Hybrid seal-on-durable
Summary: Seal to a Blob on durability.
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
