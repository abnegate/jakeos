# D-0197 · Decide the MemoryObject kernel implementation basis
- Status: proposed
- Task: MEM-002
- Surfaces: none
- Layer: none
- Spikes: MEM-011
- Supersedes: none
- Superseded by: none
- Baseline: §16, §2, §65
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
V0 requires an accepted kernel backing for MemoryObject over retained Linux mm (§16, §2, §65); the spike supplies transfer cost.

## Options

### Option A · shmem/memfd
Summary: MemoryObject is backed by shmem.
Consequences: Mature and well understood; memfd semantics leak into the object.
Evidence: none

### Option B · dma-buf as the object
Summary: MemoryObject is a dma-buf.
Consequences: Direct GPU interop; heavy for plain memory.
Evidence: none

### Option C · New native object over retained Linux mm
Summary: A native object built on Linux mm.
Consequences: Exact fit for transfer and properties; effort.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
