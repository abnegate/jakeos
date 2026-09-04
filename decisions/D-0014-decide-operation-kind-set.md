# D-0014 · Decide whether the Operation kind set is a closed kernel enum or extensible registry
- Status: proposed
- Task: ABI-014
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §18, §65
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Whether the Operation kind set is a closed kernel enum or an extensible registry is an ABI stability property that precedes the entry layer build (§18), with GPUDispatch, DeviceOperation and StorageTransaction reserved.

## Options

### Option A · Closed kernel enum
Summary: The kind set is fixed by the kernel and adding a kind is a Layer 1 change.
Consequences: Exhaustive matching and simple verification; new kinds wait for a major version.
Evidence: none

### Option B · Extensible user-service registry
Summary: User-space services register new kinds at runtime.
Consequences: Kinds grow without ABI change; the kernel cannot validate unknown kinds and the registry itself becomes a surface.
Evidence: none

### Option C · Closed V0 set with reserved slots
Summary: The V0 kinds are fixed and a range of slots is reserved for named future kinds.
Consequences: Predictable growth for GPUDispatch, DeviceOperation and StorageTransaction; reserved slots must be allocated carefully and cannot be reclaimed.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
