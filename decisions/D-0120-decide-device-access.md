# D-0120 · Decide user-space driver access: VFIO, UIO or native Device DMA
- Status: proposed
- Task: HW-006
- Surfaces: none
- Layer: none
- Spikes: HW-013, HW-014
- Supersedes: none
- Superseded by: none
- Baseline: §33, §17
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
INV-0623 must precede the V1 user-space driver framework: VFIO/UIO reuse, a native Device Capability with IOMMU-protected MemoryObject DMA, or a hybrid (§33, §17); the USB HID spike supplies measured cost.

## Options

### Option A · VFIO/UIO-style interfaces
Summary: Native drivers reuse VFIO or UIO.
Consequences: Mature and already IOMMU-aware; Linux descriptors inside native drivers.
Evidence: none

### Option B · Native Device Capability with IOMMU-protected MemoryObject DMA
Summary: A native Device object performs DMA through MemoryObjects.
Consequences: Capability-shaped and inspectable; new kernel work and a new surface.
Evidence: none

### Option C · Hybrid
Summary: A native Device Capability fronts VFIO internals.
Consequences: Incremental path to native; two layers to maintain.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
