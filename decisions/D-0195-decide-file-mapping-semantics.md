# D-0195 · Decide mapping of Capability<File> into a MemoryObject
- Status: proposed
- Task: MEM-020
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §16, §17, §25
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Text Editor and Image Viewer map files, so coherence with Write Operations, explicit sync and CoW snapshot interaction must be fixed jointly with STO (§16, §17, §25).

## Options

### Option A · Shared mapping coherent with Write Operations
Summary: The mapping shares the page cache with Write Operations.
Consequences: Coherent view; snapshot semantics are complex.
Evidence: none

### Option B · Private CoW view with explicit sync
Summary: The mapping is a private copy-on-write view flushed by an explicit sync Operation.
Consequences: Snapshot isolation; explicit sync required for durability.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
