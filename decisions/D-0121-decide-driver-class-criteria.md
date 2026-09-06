# D-0121 · Decide criteria classifying each driver as inherited, native or rewritten
- Status: proposed
- Task: HW-016
- Surfaces: none
- Layer: none
- Spikes: HW-014
- Supersedes: none
- Superseded by: none
- Baseline: §33, §55
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
D-0122 decided that driver residency is pragmatic and per class. This decision writes the criteria a class is measured against (latency, DMA safety, interrupt performance, isolation value, compatibility with inherited drivers, maintenance cost), using HW-014's measurements, and the dual-path period an inherited class must keep before it is removed (§33, §55). Its output feeds the driver status registry that the HCL and KRN's retained-mechanism inventory read.

## Options

### Option A · Keep in-kernel
Summary: A class stays as the inherited in-kernel Linux driver until a measured criterion fails.
Consequences: Every device Linux supports keeps working and no rewrite is scheduled without a reason. No isolation gain: a driver bug is still a kernel bug, and the class carries Linux's device model (sysfs, device nodes) that native software must never see.
Evidence: `reports/spikes/HW-014.md`

### Option B · Move to user-space
Summary: A class moves to a user-space driver Component hosted by SVC with typed DMA through MemoryObjects.
Consequences: A crash is a Component restart, the class is inspectable and capability-scoped, and it is the shape §33 wants for classes that can afford it. Latency and interrupt handling cross the kernel boundary, DMA safety depends on IOMMU groups the hardware register must confirm, and inherited drivers of the same class must coexist during the dual-path period.
Evidence: `reports/spikes/HW-014.md`

### Option C · Rewrite in-kernel
Summary: A class is rewritten in Rust inside the kernel.
Consequences: Memory safety without the user-space boundary cost, suitable where latency forbids option B. A rewrite is the most expensive path, must reach parity with a mature C driver before the dual path ends, and upstream-first (D-0167) means the Rust driver is offered to Linux.
Evidence: `reports/spikes/HW-014.md`

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
