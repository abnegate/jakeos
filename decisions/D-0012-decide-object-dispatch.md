# D-0012 · Decide Object-Operation dispatch with async-only submission and move semantics
- Status: proposed
- Task: ABI-012
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §18, §65
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Operation invocation on typed handles must have no blocking entry other than wait-for-completion (§65 rule 4) and must move Capability and MemoryObject arguments rather than copy them (§65 rule 5), on the S-002 and S-004 surfaces that remain prototyped.

## Options

### Option A · Syscall-per-operation dispatch
Summary: Each Operation is dispatched by its own kernel entry.
Consequences: Simple mapping from Operation to entry; the entry-point count grows and batching is impossible.
Evidence: none

### Option B · Ring-indexed dispatch
Summary: Operations are dispatched through the chosen ring entry mechanism indexed by Operation kind.
Consequences: Batching and few entry points; the ring layout becomes part of the dispatch contract.
Evidence: none

### Option C · Hybrid with inline completion
Summary: Ring dispatch, with already-ready work completing inline at submit.
Consequences: Cheap fast path for cached reads and signalled waits; the caller must handle two completion paths.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
