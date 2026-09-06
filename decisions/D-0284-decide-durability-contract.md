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
Write and StorageTransaction are Operations (§18) over the retained filesystem (§26), and applications need one stated point after which data survives power loss. This decision fixes the completion condition that means power-loss safety, on the transaction model (STO-031) and the storage object API (STO-020); the personality's fsync maps onto it.

## Options

### Option A · fsync-on-commit
Summary: A Write or StorageTransaction completes only when the data and metadata are durable (fsync semantics on commit).
Consequences: The simplest contract: completion means safe. Every commit pays a device flush, so small frequent writes are slow on consumer NVMe and SD storage, and applications that do not need durability pay for it anyway.
Evidence: none

### Option B · Group-commit with a bounded window
Summary: Commits are grouped and flushed within a bounded window; completion means ordered and visible, and a separate Durable event fires when the group is flushed.
Consequences: High throughput for the common case and durability still observable. There is a window in which a completed write can be lost, which every application must understand, and the two-event model is a new shape for the SDK to explain.
Evidence: none

### Option C · Explicit Durable flag on Write
Summary: Write carries a Durable flag; without it completion means visible, with it completion means power-loss safe.
Consequences: Applications choose per Operation, editors set it on save and caches never do, and the fast path stays fast. The burden of knowing when to set the flag is on every developer, and the personality must map fsync onto a flagged write of everything outstanding.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
