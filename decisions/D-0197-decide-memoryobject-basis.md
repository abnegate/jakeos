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
MemoryObject is the only native way to share or move memory (§16), and its kernel backing decides what a mapping is, what ownership transfer costs (B-007) and how GPU, camera and DMA devices see the object (§17, §38). MEM-011 measures transfer including TLB shootdown at the three B-007 sizes on H-001 and H-002 over shmem, dma-buf and a native backing. Whatever is chosen, native software holds `Capability<MemoryObject>` and never a file descriptor (§57, §65); the question is the kernel object underneath and what interoperation with retained drivers and the Linux personality costs (§2).

## Options

### Option A · shmem/memfd
Summary: A MemoryObject is a shmem-backed memfd inode wrapped by a Capability; mapping and transfer reuse the retained mm paths.
Consequences: Paging, swap, huge pages and accounting come for free, and the Linux personality gets memfd interoperation almost unchanged. memfd semantics (sealing, growth, file offsets) leak into what the object can promise, and ownership transfer has to be layered on because inodes have no owner. Every GPU export needs a dma-buf wrapper, giving each object a second identity to keep consistent.
Evidence: `reports/spikes/MEM-011.md`

### Option B · dma-buf as the object
Summary: Every MemoryObject is a dma-buf; CPU access goes through the dma-buf map path and device handoff uses its fences.
Consequences: GPU, camera and codec interoperation is direct with no conversion step, and device ownership handoff maps onto existing fence machinery. Ordinary heap-sized objects pay exporter and importer bookkeeping designed for buffers; dma-buf has no swap, no copy-on-write and no growth, so MEM's later CoW and NUMA work has no foundation. The coherence model of D-0192 must be rebuilt on top.
Evidence: `reports/spikes/MEM-011.md`

### Option C · New native object over retained Linux mm
Summary: A native kernel object with its own ownership, rights and transfer semantics, backed by retained mm pages, that exports a dma-buf or memfd view only when a retained driver or a personality needs one.
Consequences: Exact fit: unmap-and-invalidate on transfer, copy-on-write, zeroing on reuse (T-044), NUMA placement and ResourceDomain charging are properties of the object rather than workarounds. It is the largest V0 kernel effort in MEM and the object's shape becomes the Layer 1 ABI; every export view must track the object's rights so a personality cannot widen them.
Evidence: `reports/spikes/MEM-011.md`

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
