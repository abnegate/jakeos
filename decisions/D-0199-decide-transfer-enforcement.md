# D-0199 · Decide whether Ownership transfer is kernel-enforced or advisory
- Status: proposed
- Task: MEM-003
- Surfaces: none
- Layer: none
- Spikes: MEM-011
- Supersedes: none
- Superseded by: none
- Baseline: §16, §17, §65, §67
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Ownership transfer is the default way a MemoryObject crosses a Channel (§16, §17, §65, §67): after a move the receiver owns the object. Whether the kernel enforces that or trusts the sender decides whether zero-copy is also zero-trust, what a stale sender mapping does, what happens to derived Capabilities and in-flight map Operations, and the fixed cost of every transfer, which is a TLB shootdown proportional to the sender's mappings and cores. MEM-011 measures that cost per B-007 size so the SDK move types are designed against a number.

## Options

### Option A · Kernel unmaps the sender and invalidates the sender handle
Summary: Transfer unmaps every sender mapping, invalidates the sender's handle and every Capability derived from it, and completes outstanding map Operations from the sender with `Error::Revoked`.
Consequences: The receiver can trust the bytes it holds, so zero-copy is safe across trust boundaries, which the compositor, media codecs (MED) and the personalities depend on. Every transfer pays an unmap and shootdown that grows with the sender's mapping count and the number of cores it ran on; the SDK must discourage many small live mappings. A load or store through a stale sender mapping faults and becomes a typed Component exit cause under D-0066.
Evidence: `reports/spikes/MEM-011.md`

### Option B · Advisory transfer
Summary: Transfer moves the handle only; the sender promises not to touch the object and its mappings stay valid until it unmaps them itself.
Consequences: The cheapest possible move with no TLB traffic; safe Rust move types in the SDK provide the discipline for well-behaved code. A buggy or hostile sender can read or corrupt the receiver's data after the move, so any transfer across a trust boundary needs a copy anyway and the threat model must enumerate every such path. Derived Capabilities stay usable, so revocation (Q-004) has no meaning for moved objects.
Evidence: `reports/spikes/MEM-011.md`

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
