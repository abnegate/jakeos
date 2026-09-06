# D-0198 · Decide the MemoryObject locality and placement attribute model
- Status: proposed
- Task: MEM-034
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §17, §37, §38
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Heterogeneous compute exposes locality and cost rather than hiding them (§37, §38): a MemoryObject may be on a NUMA node, device-local, remote or persistent (§17). Before the V2 ComputeDevice dispatch demo, MEM and HET fix the attribute vocabulary and whether a Component can only read placement, request it, or must hold a right to it. The accepted option names the `os inspect` fields HET dispatch reads.

## Options

### Option A · Query-only attributes
Summary: Placement attributes are readable on every MemoryObject but the kernel alone decides placement.
Consequences: Nothing to get wrong for applications and the vocabulary is inspect-only. A dispatcher that knows a buffer belongs on the GPU cannot ask for it, so HET stages copies, and NUMA-aware allocation is impossible from user space.
Evidence: none

### Option B · Requestable placement at allocation and migrate
Summary: A creation request may name a preferred placement and an Operation may migrate an object; the kernel honours or returns a typed error.
Consequences: Dispatchers and allocators steer placement where it matters and the request vocabulary doubles as the inspect vocabulary. The allocator gains policy (fallback, migration cost), migration interacts with mappings and ownership transfer, and a bad request is a performance bug the SDK must make visible.
Evidence: none

### Option C · Placement as a Capability right
Summary: Placement on a scarce class (device-local, persistent) requires a right on the MemoryObject Capability; ordinary Components get default placement.
Consequences: Scarce memory is governed like every other scarce resource and cannot be hogged by an unprivileged application. Rights registration for a performance attribute is heavy, and the common case (a game wanting VRAM) needs a grant path that does not exist yet.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
