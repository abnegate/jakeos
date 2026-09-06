# MEM · Memory objects and zero-copy
- Prefix: MEM
- Lead: none
- Baseline: §16, §17, §38

<!-- roadmap:generated:begin summary -->
Tasks: 56 live, 1 done, 0 in-progress, 55 todo, 0 dropped. Ready: 1. Blocked: 54. Weighted: 1%.
<!-- roadmap:generated:end -->

## Scope

MEM owns MemoryObject: the kernel object for large data, its properties, mappings, charging, and the zero-copy dataflow that moves those objects by ownership transfer rather than by copying bytes (§16, §17). Native software holds `Capability<MemoryObject, Rights>` and never a POSIX file descriptor, an `mmap` address as authority, or a Linux dma-buf handle. The kernel tracks size, properties, owner, and the mapping table; retained Linux mm, shmem, and dma-buf are mechanisms underneath, not the ABI (§2, §65).

The property set covers writable, immutable (sealing), shared, copy-on-write, DMA-compatible, GPU-compatible, executable with W^X, pinned, and encrypted. Persistent backing is a storage property consumed here and implemented by STO. Placement attributes name NUMA node, device-local, remote, and persistent media so HET dispatch can read where an object lives without assuming coherent DRAM (§38). Spikes explore S-006 before decisions; Layer 1 freeze of that surface is V4.

## Out of scope

Handle encoding, syscall entry, and the error taxonomy (ABI). Capability rights words, derivation, and CHERI readiness of the handle (CAP). Component address-space construction from Packages (CMP). Channel message slots that carry a MemoryObject Capability (IPC). ResourceDomain budget policy and exhaustion (SCH). `os inspect` command rendering (SDK) and the inspect data plane (OBS). `Capability<File>` and snapshots (STO). Content-addressed store layout (PKG). Buffer, Surface, and compositor import (GFX). ComputeDevice dispatch (HET). NIC DMA into a receive object (NET). Codec Components (MED). Threat-model publication (SEC). Linux personality descriptors (LNX). Suspend cycle harness (PWR). Benchmark register and runner (BEN). Fuzz fleet (BLD). Generated reference site (DOC). Inherited-CVE SLA (REL).

## Tasks

### MEM-001 · Decide the MemoryObject sharing coherence model across CPUs and devices
- Type: adr
- Milestone: V0
- Status: done
- Size: S
- Owner: @agent/claude
- Depends on: none
- Baseline: §16, §17, §38
- Decision: D-0192
- Invariants: I-024
- Verified by: @jakebarnby

Shared mappings need a written coherence contract before the shared property lands. The native platform does not assume coherent memory between all devices (§38); CPU-only coherence with explicit device sync, or a per-mapping coherence attribute, are the options the V0 shared-mapper tests then implement.

<!-- covers: INV-0329, INV-0703 -->

#### Out of scope
GPU-compatible allocation (MEM-024). Device-local backing (MEM-046). Explicit GPU synchronization protocol (GFX).

#### Acceptance criteria
- [x] Option A (CPU-coherent mappings plus explicit device sync Operations) and Option B (per-mapping coherence attribute) are evaluated against non-coherent devices.
- [x] The accepted option states what a second mapper may observe after a write by the first mapper, on CPU and on a non-coherent device.
- [x] Review sign-off is recorded on the pull request.

#### Verification
- Review: ABI lead and MEM reviewer sign-off recorded on the pull request.

#### Evidence
- decision:D-0192

### MEM-002 · Decide the MemoryObject kernel implementation basis
- Type: adr
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: MEM-011
- Baseline: §16, §2, §65
- Decision: D-0197
- Invariants: I-009

V0 requires an accepted choice of kernel backing for MemoryObject over retained Linux mm: shmem/memfd, dma-buf, or a new native object. The report from MEM-011 supplies transfer cost including TLB shootdown so the choice is not made on familiarity. dma-buf interoperation for GPU and DMA is a consequence of the choice, not a later surprise (§16).

<!-- covers: INV-0317 -->

#### Out of scope
Whether transfer is enforced or advisory (MEM-003). Whether dma-buf backs GPU-compatible objects at V0.5 (MEM-019).

#### Acceptance criteria
- [ ] Option A (shmem/memfd), Option B (dma-buf as the object), and Option C (new native object over retained Linux mm) are evaluated with dma-buf export/import consequences.
- [ ] The accepted option cites the spike report for transfer cost of the three B-007 sizes on H-001 and H-002.
- [ ] The accepted option leaves native software holding MemoryObject Capabilities, not memfd or dma-buf descriptors.
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: ABI lead and MEM reviewer sign-off recorded on the pull request.
- Report: `reports/spikes/MEM-011.md` is cited as evidence for the chosen option.

#### Evidence
- none

### MEM-003 · Decide whether Ownership transfer is kernel-enforced or advisory
- Type: adr
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: MEM-011
- Baseline: §16, §17, §65, §67
- Decision: D-0199
- Invariants: I-056, I-063

Ownership transfer is the ABI default for MemoryObjects (§65, §67). The decision fixes whether the kernel unmaps the sender and invalidates its handle, or the sender promises not to touch the object after the move. Enforcement cost from MEM-011 is attached so SDK move types are not designed against an unmeasured unmap.

<!-- covers: GAP-0498 -->

#### Out of scope
Channel handle slots (IPC-014). Borrowing lifetimes (MEM-018).

#### Acceptance criteria
- [ ] Option A (kernel unmaps the sender and invalidates the sender handle) and Option B (advisory transfer; sender promised not to touch) are evaluated with the measured enforcement cost from the spike report.
- [ ] The accepted option states what a load or store through a sender mapping does after a successful transfer.
- [ ] The accepted option states what happens to derived Capabilities and outstanding map Operations at transfer.
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: ABI lead and MEM reviewer sign-off recorded on the pull request.
- Report: `reports/spikes/MEM-011.md` is cited for enforcement cost.

#### Evidence
- none

### MEM-004 · Charge MemoryObject pages to the owning ResourceDomain memory budget
- Type: build
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: MEM-005
- Baseline: §16, §23
- Threats: T-016
- Invariants: I-033

V0 ResourceDomain memory-budget enforcement is meaningless unless MemoryObject pages are charged to the owner's domain. SCH owns the budget object and the typed exhaustion error; MEM installs the charging hook on create, grow, and destroy so a Component cannot exceed the domain memory budget by allocating MemoryObjects (§23).

<!-- covers: INV-0427, INV-0237 -->

#### Out of scope
Budget policy and CPU share (SCH-008). Charge-follows-owner across a transfer (MEM-015). Kernel-object count limits (SCH-009).

#### Acceptance criteria
- [ ] Creating a MemoryObject charges its resident pages to the owner's ResourceDomain and is visible in the domain's memory consumption.
- [ ] An allocation that would exceed the domain memory budget returns a typed exhaustion error and allocates no additional pages.
- [ ] Destroying the object or its owning Component reclaims the charge so a subsequent create at the same size succeeds.

#### Verification
- Unit: `kernel:tests/mem/charge_budget_*` on CI matrix entries `qemu-x86_64` (H-001) and `hw-h002` (H-002).
- Integration: V0 ResourceDomain memory-budget scenario creates MemoryObjects until the limit and records the typed error.

#### Evidence
- none

### MEM-005 · Implement the MemoryObject kernel Object over retained Linux mm
- Type: build
- Milestone: V0
- Status: todo
- Size: L
- Owner: none
- Depends on: MEM-002
- Baseline: §7, §16, §24, §51, §59, §69
- Invariants: I-009, I-034, I-082

Object<Memory> is the kernel object for large data (§7, §16). Create and destroy track size, properties, owner, and the mapping table over retained Linux mm as decided by MEM-002. Destroy of the owning Component reclaims the object. Owner, mappers, and properties are exported on the inspect interface so `os inspect memory` can print state without reconstructing it from raw mm internals (§24, §59).

<!-- covers: INV-0052, INV-0163, INV-0305, INV-1318, INV-1323 -->

#### Out of scope
Map and unmap (MEM-007). Inspect CLI rendering (SDK-007). Object registry and handles (ABI-005).

#### Acceptance criteria
- [ ] A Component creates a MemoryObject of a requested size and the kernel records size, default properties, owner, and an empty mapping table.
- [ ] Pages of a newly created MemoryObject read as zero before the first write; a test that destroys an object and creates another of the same size never observes prior contents.
- [ ] Destroying the object or its Component leaves no kernel accounting for that object; a leak test of repeated create/destroy shows no unbounded growth.
- [ ] Inspect data for a live object includes owner, mapper set, size, and properties.
- [ ] Native crates have no API that returns a Linux memfd, dma-buf fd, or POSIX descriptor for the object.
- [ ] New kernel code for the object is Rust unless a file is exempted by an accepted decision.

#### Verification
- Unit: `kernel:tests/mem/object_create_destroy_*` on H-001 and H-002.
- Integration: inspect provider for MemoryObject is exercised by the V0 `os inspect memory` path.
- Review: ABI review-gate checklist records no exposed mm_struct or page-table layout on the native surface (I-057).

#### Evidence
- none

### MEM-006 · Implement the executable property with W^X enforcement
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: MEM-007
- Baseline: §16, §51
- Invariants: I-082

Component code pages are executable MemoryObjects. The kernel refuses any mapping that is simultaneously writable and executable, and refuses to add executable to an object that already has a writable mapping (§16, §51). The check is a regression test on every kernel build.

<!-- covers: INV-0312 -->

#### Out of scope
Sealing (MEM-008). Component code-page layout from Packages (CMP-003).

#### Acceptance criteria
- [ ] Mapping an object with both writable and executable requested returns a typed rights error and installs no mapping.
- [ ] Adding executable to an object that has a writable mapping returns a typed rights error and leaves mappings unchanged.
- [ ] Adding writable to an object that has an executable mapping returns a typed rights error and leaves mappings unchanged.
- [ ] A sealed executable object maps read-execute and rejects a write fault.

#### Verification
- Unit: `kernel:tests/mem/wx_*` on H-001 and H-002.
- Integration: CI matrix runs the W^X suite on every kernel build.

#### Evidence
- none

### MEM-007 · Implement MemoryObject map and unmap in a Component address space
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: MEM-005
- Baseline: §16, §59, §65
- Invariants: I-057

V0 exit requires a Component to create and map a MemoryObject (§59). The writable property governs mapping protection. Mappings are tracked per Component and torn down on unmap, on object destroy, and on Component destroy. The ABI does not expose page-table layout (§65).

<!-- covers: INV-1161, INV-0306 -->

#### Out of scope
Ownership transfer unmap of the sender (MEM-010). File-backed mappings (MEM-023). Address-space object internals (CMP).

#### Acceptance criteria
- [ ] A Component maps a writable MemoryObject and can store and load through the mapping.
- [ ] A non-writable object mapped writable returns a typed rights error and installs no mapping.
- [ ] Unmap removes the mapping; a subsequent access through the old address does not observe the object.
- [ ] Destroying the Component removes every mapping it held and leaves the object owned if another holder exists.

#### Verification
- Unit: `kernel:tests/mem/map_unmap_*` on H-001 and H-002.
- Integration: V0 demo Component maps the result MemoryObject and reads it without a copy.
- Fuzz: `kernel:fuzz/mem_map` on the nightly matrix without panic.

#### Evidence
- none

### MEM-008 · Implement the immutable property and sealed read-only mappings
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: MEM-007
- Baseline: §16, §17, §67

Sealing is one-way and kernel-enforced (§16, §17). A sealed object rejects writable mappings and write faults. The V0 demo result object is sealed so the receiver cannot mutate what it was given; later immutable Package pages reuse the same property.

<!-- covers: INV-0307, INV-0330 -->

#### Out of scope
Copy-on-write private views of sealed pages (MEM-027). Package mapping (CMP-017).

#### Acceptance criteria
- [ ] Seal is idempotent and irreversible: a sealed object reports immutable and rejects an unseal Operation.
- [ ] Mapping a sealed object writable returns a typed rights error and installs no mapping.
- [ ] A write fault on a sealed mapping does not modify the object and is reported as a typed fault to the Component.
- [ ] The V0 demo result object is sealed before transfer; the receiver's mapping is read-only.

#### Verification
- Unit: `kernel:tests/mem/seal_*` on H-001 and H-002.
- Integration: V0 demo pipeline seals the result MemoryObject before the transfer.

#### Evidence
- none

### MEM-009 · Document MemoryObject security semantics for the V0 threat model
- Type: docs
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: MEM-003, MEM-001, MEM-008, MEM-006
- Baseline: §16, §17, §51
- Risks: R-008, R-080
- Threats: T-015, T-016

SEC publishes the V0 threat model from the register; MEM supplies the MemoryObject section so transfer enforcement, sealing, W^X, shared-mapping side channels, and ownership as a memory-safety layer are written before CAP and SEC designs freeze around them (§51).

<!-- covers: INV-0953 -->

#### Out of scope
The threat-model document itself (SEC-002). Side-channel position statement (SEC-029).

#### Acceptance criteria
- [ ] A committed document section enumerates transfer enforcement, sealing, W^X, shared-mapping side channels, and ownership, each citing T-IDs from the register.
- [ ] The section states what the capability model claims for MemoryObject isolation and what it does not claim (T-015).
- [ ] Review sign-off from SEC and MEM is recorded on the pull request.

#### Verification
- Review: SEC and MEM reviewers sign off on the pull request; the V0 threat-model task cites this section.

#### Evidence
- none

### MEM-010 · Implement single-owner MemoryObject transfer over a Channel with sender losing access
- Type: build
- Milestone: V0
- Status: todo
- Size: L
- Owner: none
- Depends on: MEM-007, MEM-003
- Baseline: §15, §16, §17, §59, §65, §67
- Benchmarks: B-007
- Risks: R-009
- Invariants: I-056, I-063

A MemoryObject Capability moves from Component A to Component B without copying payload bytes (§15, §17, §59). After the move there is a single owner. Per MEM-003 the sender's mappings are revoked or its handle is invalidated. Channel encoding of the move is IPC; this task is the kernel ownership and mapping side.

<!-- covers: INV-0327, INV-0300, INV-1162, INV-0953, INV-1297 -->

#### Out of scope
Handle slots in the wire format (IPC-014). Physical-page identity harness (MEM-012). Borrowing (MEM-026).

#### Acceptance criteria
- [ ] After a successful transfer, inspect shows B as owner and A is not a mapper or owner.
- [ ] A load or store through A's prior mapping after transfer does not observe the object.
- [ ] Physical-page identity of the payload is unchanged across the transfer.
- [ ] Transfer of a MemoryObject the sender does not own returns a typed rights error and allocates no handle on the receiver.

#### Verification
- Unit: `kernel:tests/mem/transfer_owner_*` on H-001 and H-002.
- Integration: V0-D01 pipeline transfers the result object; `os trace` shows the move.
- Bench: B-007 publish on H-001 and H-002 (harness owned with BEN-003).

#### Evidence
- none

### MEM-011 · Prototype MemoryObject Ownership transfer over shmem, dma-buf and native backings
- Type: spike
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: none
- Baseline: §16, §17, §53
- Benchmarks: B-007
- Explores: S-006

Zero-copy is real only if transfer cost, including unmap and TLB shootdown, is measured against copying at realistic sizes. The spike prototypes ownership transfer on shmem/memfd, dma-buf, and a dedicated native object, and publishes cost for the B-007 register sizes (4 KiB, 1 MiB and 1 GiB) on H-001 and H-002. The report feeds MEM-002, MEM-003, and B-007.

<!-- covers: GAP-0497 -->

#### Out of scope
The standing B-007 harness in CI (BEN-003). Production object implementation (MEM-005).

#### Acceptance criteria
- [ ] Each of the three backings transfers an object of each of the three B-007 sizes on H-001 and H-002.
- [ ] The report records p50 and p99 transfer cost, copy count, and TLB-shootdown observations per backing and size, with memcpy as the baseline.
- [ ] The report answers the Report questions and does not freeze S-006.

#### Verification
- Bench: B-007 method applied to the three prototypes on H-001 and H-002; publish only.
- Report: Which backing has the lowest transfer cost at each size? What fraction of cost is TLB shootdown versus handle move? Does kernel-enforced unmap change the ranking versus advisory transfer? What dma-buf export/import tax appears at the larger size?

#### Evidence
- none

### MEM-012 · Verify MemoryObject transfer by physical-page identity in the V0 Demo pipeline
- Type: build
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: MEM-010
- Baseline: §17, §59
- Benchmarks: B-007
- Risks: R-009

V0-G06 and V0-D01 require transfer with no payload copy, checked by physical-page identity, and B-007 published for the register sizes. The suite also asserts the sender cannot use the object after transfer and that a leaked object is reclaimed with its Component.

<!-- covers: INV-0300, INV-1162, INV-1161 -->

#### Out of scope
Harness runner and result publication into the register (BEN-003). Channel wiring of the demo (CMP-011).

#### Acceptance criteria
- [ ] The V0 demo transfer passes a physical-page identity check on H-001 and H-002.
- [ ] After transfer the sender's access fails and the receiver reads the payload.
- [ ] Destroying the owner Component with no other holder reclaims the object; the leak test reports no residual kernel charge.
- [ ] B-007 reports for the register sizes exist for H-001 and H-002.

#### Verification
- Integration: `kernel:tests/mem/transfer_page_identity_*` on H-001 and H-002.
- Bench: B-007 on H-001 and H-002; target per register (V0 publish).
- Demo: V0-D01 on H-002 with `os trace` showing the move.

#### Evidence
- none

### MEM-013 · Benchmark CoW snapshot creation and first-write fault cost
- Type: benchmark
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: MEM-027
- Baseline: §16, §34, §53
- Benchmarks: B-016, B-037
- Invariants: I-061

Copy-on-write is used for file-mapping snapshots and cheap private views of immutable Package pages. Before V0.5 apps rely on it, snapshot-create and first-write fault cost are published for 4 KiB, 1 MiB, and 1 GiB objects against Linux fork-plus-write and memfd copy on the same hardware. Numbers live in the reports; this task does not restate them.

<!-- covers: INV-0309 -->

#### Out of scope
Warm-startup gate publication (BEN-009). File-backed mapping (MEM-023).

#### Acceptance criteria
- [ ] Reports exist for H-001, H-002, and H-003 covering snapshot-create and first-write fault at the three sizes.
- [ ] Each report names Linux fork-plus-write and memfd copy as baselines on the same machine.
- [ ] No documentation or task prose states a CoW speedup except by citing these reports.

#### Verification
- Bench: B-016 and B-037 on H-001, H-002, and H-003; V0.5 publish per register.
- Review: BEN claim-lint is clean for CoW-related prose.

#### Evidence
- none

### MEM-014 · Benchmark resident-page sharing across the four V0.5 applications
- Type: benchmark
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: MEM-016
- Baseline: §34, §53
- Benchmarks: B-008
- Invariants: I-061

The V0.5 deduplication gate covers store size; this covers memory. The harness publishes the shared-versus-private resident page ratio for Terminal, File Browser, Text Editor, and Image Viewer against Linux shared-library sharing on the same hardware.

<!-- covers: INV-0632 -->

#### Out of scope
Store-size deduplication (PKG, B-021). Idle Component overhead without sharing (CMP-002).

#### Acceptance criteria
- [ ] A report per in-scope machine lists shared and private resident pages for the four apps launched together from the same library Package.
- [ ] The same session records the Linux shared-library baseline on that machine.
- [ ] Two apps using the same library show a shared set of code pages in the report.

#### Verification
- Bench: B-008 on H-002 and H-003; V0.5 publish per register.
- Integration: `os inspect memory` lists the shared object as mapped by both Components.

#### Evidence
- none

### MEM-015 · Move MemoryObject charging with ownership and never double-charge a borrow
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: MEM-004, MEM-010, MEM-026
- Baseline: §16, §17, §23
- Risks: R-076
- Threats: T-016

When a MemoryObject moves between ResourceDomains the charge follows the owner. A borrow never double-charges: pages stay on the owner's budget while the borrower maps them (§23). Tests cover a transfer that would exceed the receiver budget and a borrow that must not inflate the borrower's budget.

<!-- covers: EXTRA-003 -->

#### Out of scope
Exhaustion policy (SCH-016). Channel queue charging (IPC-027).

#### Acceptance criteria
- [ ] After ownership transfer, the sender domain's memory consumption drops by the object size and the receiver's rises by the same amount, within page-rounding.
- [ ] Transfer into a domain that cannot afford the object returns a typed exhaustion error, allocates no handle on the receiver, and leaves the sender as owner.
- [ ] A borrow does not increase the borrower's domain memory consumption by the object size.
- [ ] Return or revocation of a borrow leaves the owner's charge unchanged and the borrower's mapping gone.

#### Verification
- Unit: `kernel:tests/mem/charge_transfer_*` and `kernel:tests/mem/charge_borrow_*` on H-001 and H-002.
- Integration: two ResourceDomains exchange an object at the receiver's budget limit.

#### Evidence
- none

### MEM-016 · Share identical code pages across applications via content-store MemoryObjects
- Type: build
- Milestone: V0.5
- Status: todo
- Size: L
- Owner: none
- Depends on: MEM-008, MEM-029
- Baseline: §16, §17, §27, §34
- Invariants: I-039, I-062

V0.5 exit: two applications using the same library share one set of resident code pages. Immutable content-addressed objects are mapped shared and sealed. PKG and STO provide the store; CMP lays out the address space; MEM supplies sealed shared MemoryObjects so the pages are one physical copy (§34).

<!-- covers: INV-0632 -->

#### Out of scope
Store schema (PKG-038). Address-space construction (CMP-017, CMP-029). Huge-page policy (MEM-021).

#### Acceptance criteria
- [ ] Two Components mapped from the same content-addressed library object show one resident page set in inspect and in `/proc`-free kernel accounting.
- [ ] The shared object is sealed; a writable mapping request returns a typed rights error.
- [ ] Unmapping one Component leaves the other's mapping and the resident pages intact.
- [ ] Distinct content hashes do not share pages.

#### Verification
- Integration: four V0.5 apps launched; inspect shows shared sealed objects for common libraries on H-002 and H-003.
- Bench: consumed by MEM-014.

#### Evidence
- none

### MEM-017 · Decide the MemoryObject backing-provider abstraction for future memory media
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: MEM-002
- Baseline: §16, §38, §65
- Decision: D-0190
- Risks: R-007
- Invariants: I-024, I-058

A MemoryObject is properties plus a backing provider, not a DRAM page list. The decision records how DRAM, CXL-attached, persistent, disaggregated, unified CPU/GPU, and accelerator-local media plug in without changing the Layer 1 mapping operations (§16, §38). Native software keeps seeing MemoryObject; media differences appear as queryable properties and placement attributes, never as a new object type per medium.

<!-- covers: INV-0704, INV-0706, INV-0707, INV-0709, INV-0710, INV-0711, INV-0712, INV-0713 -->

#### Out of scope
Persistent crash consistency (STO-040, Q-006). Device-local implementation (MEM-046). CXL hardware bring-up (MEM-056).

#### Acceptance criteria
- [ ] Option A (DRAM object now, retrofit providers later) and Option B (provider interface from V0) are evaluated with consequences for CXL, persistent, disaggregated, unified, and accelerator-local media.
- [ ] The accepted option keeps map, unmap, transfer, and property queries stable when a new backing is added.
- [ ] The accepted option does not assume coherent memory between all devices or uniform access (§38).
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: ABI lead and MEM reviewer sign-off recorded on the pull request.

#### Evidence
- none

### MEM-018 · Decide how borrowing lifetimes are enforced across Component boundaries
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: MEM-010
- Baseline: §17
- Decision: D-0191
- Invariants: I-063

Borrowing is temporary access that returns to the owner (§17). The compositor and renderer borrow application buffers in V0.5; the enforcement model must be fixed first. Options are revocation on return, deadline-bound borrows with forced unmap, or trust in the borrower with audit. The accepted option answers Q-008.

<!-- covers: INV-0338 -->

#### Out of scope
Implementation (MEM-026). Channel lifetime of the borrow Capability (CAP, IPC).

#### Acceptance criteria
- [ ] Option A (revocation on return), Option B (deadline-bound borrow with forced unmap), and Option C (trust in the borrower with audit) are evaluated for compositor buffer borrow, borrower death, and owner destroy during a borrow.
- [ ] The accepted option states whether the owner may mutate, transfer, or destroy the object while a borrow is outstanding.
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: ABI lead and MEM reviewer sign-off recorded on the pull request.

#### Evidence
- none

### MEM-019 · Decide whether dma-buf backs DMA- and GPU-compatible MemoryObjects
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: MEM-030
- Baseline: §16, §17, §39
- Decision: D-0193
- Risks: R-016
- Invariants: I-045

The V0.5 compositor needs GPU buffers on inherited DRM drivers. Options are dma-buf as the backing, a native object exported as dma-buf on demand, or a native object only. The spike on H-002 shows whether a NIC-to-GPU path without a copy is achievable. Native software still does not see dma-buf descriptors; GFX wraps Buffer around the MemoryObject.

<!-- covers: GAP-0499 -->

#### Out of scope
Buffer abstraction (GFX-005). Personality import/export (MEM-025). Native GPU driver stack (forbidden by I-045).

#### Acceptance criteria
- [ ] Option A (dma-buf is the backing), Option B (native object exported as dma-buf on demand), and Option C (native object only, no dma-buf) are evaluated against inherited DRM/Mesa and the spike report.
- [ ] The accepted option states how a native Component without a personality Capability obtains a GPU-compatible MemoryObject.
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: GFX, MEM, and ABI reviewers sign off on the pull request.
- Report: `reports/spikes/MEM-030.md` is cited.

#### Evidence
- none

### MEM-020 · Decide mapping of Capability<File> into a MemoryObject
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: MEM-007
- Baseline: §16, §17, §25
- Decision: D-0195

Text Editor and Image Viewer map files. The decision fixes coherence with Write Operations, the explicit sync Operation (not a POSIX msync as a native API), and interaction with copy-on-write filesystem snapshots, jointly with STO. Options are a page-cache-shared mapping versus a private CoW view with explicit sync.

<!-- covers: EXTRA-006 -->

#### Out of scope
Implementation (MEM-023). File object Operations (STO-020). Snapshot Operations (STO-025).

#### Acceptance criteria
- [ ] Option A (shared mapping coherent with Write Operations on the same File) and Option B (private CoW view with explicit sync) are evaluated against snapshot isolation and durability.
- [ ] The accepted option names the Operation that makes writes durable and what a concurrent snapshot contains.
- [ ] Native software maps via MemoryObject, not via a POSIX mmap entry point.
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: STO and MEM reviewers sign off on the pull request.

#### Evidence
- none

### MEM-021 · Decide the per-Component page-table and huge-page policy for MemoryObjects
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: MEM-031
- Baseline: §16, §34, §38
- Decision: D-0196

Warm startup of Terminal and Editor depends on how cheaply verified immutable pages are mapped and shared (§34). The spike measures per-Component page-table cost and huge-page TLB effects; this decision chooses shared page-table fragments, transparent huge pages, or an explicit huge-page property, without exposing page-table layout on the ABI (§38, §65).

<!-- covers: GAP-0549 -->

#### Out of scope
Implementation on Package mappings (MEM-036). Address-space object (CMP).

#### Acceptance criteria
- [ ] Option A (shared page-table fragments), Option B (transparent huge pages), and Option C (explicit huge-page property on the MemoryObject) are evaluated with the spike's startup and TLB measurements.
- [ ] The accepted option does not add a native API that names Linux THP or hugetlbfs.
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: ABI lead and MEM reviewer sign-off recorded on the pull request.
- Report: `reports/spikes/MEM-031.md` is cited.

#### Evidence
- none

### MEM-022 · Transfer Decoder output MemoryObjects to the Renderer without copying
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: MEM-010, MEM-024
- Baseline: §11, §17, §39
- Invariants: I-063

The V0.5 Image Viewer is the first §17 pipeline stage that ships: a sandboxed decoder Component hands its output by ownership to the renderer. Physical-page identity verifies no copy. MED owns decoder Packages later; GFX owns Buffer and presentation; MEM owns the move.

<!-- covers: INV-0324 -->

#### Out of scope
Isolated codec Packages (MED-024). Buffer and scanout (GFX). Network receive stage (MEM-039).

#### Acceptance criteria
- [ ] Decoder output transferred to the renderer passes a physical-page identity check on H-002.
- [ ] After transfer the decoder Component is not a mapper or owner of the output object.
- [ ] The renderer maps the object as GPU-compatible without a CPU copy of the payload.

#### Verification
- Integration: Image Viewer scripted scenario on H-002 and H-003 with page-identity assertion.
- Demo: V0.5 Image Viewer opens a user-chosen image; inspect shows one MemoryObject moving decoder to renderer.

#### Evidence
- none

### MEM-023 · Implement file-backed MemoryObjects with sync and snapshot interaction
- Type: build
- Milestone: V0.5
- Status: todo
- Size: L
- Owner: none
- Depends on: MEM-020, MEM-027
- Baseline: §16, §17, §25
- Benchmarks: B-037

Implements MEM-020 so Text Editor and Image Viewer map `Capability<File>` into a MemoryObject. Tests cover Write coherence, the explicit sync Operation, and snapshot isolation against STO CoW snapshots. Native software does not call POSIX mmap or msync.

<!-- covers: EXTRA-006 -->

#### Out of scope
File object itself (STO). Chooser authority (STO, APP). Persistent MemoryObject property (STO-046).

#### Acceptance criteria
- [ ] Mapping a File Capability produces a MemoryObject whose payload matches a Read of the same File at map time, per the accepted coherence option.
- [ ] The sync Operation makes mapped writes durable to the point named by the decision; a crash-restart test on NVMe observes only synced bytes.
- [ ] A snapshot taken after a write and before sync contains the pre-write contents; a snapshot after sync contains the write, per the decision.
- [ ] A Component without the File Capability cannot map that File.

#### Verification
- Unit: `kernel:tests/mem/file_map_*` on H-001 and H-002.
- Integration: Text Editor and Image Viewer map user-selected files on H-002 and H-003.
- Bench: B-037 consume path on H-002; V0.5 publish per register.

#### Evidence
- none

### MEM-024 · Implement GPU buffer allocation through MemoryObject with dma-buf interoperability
- Type: build
- Milestone: V0.5
- Status: todo
- Size: L
- Owner: none
- Depends on: MEM-028, MEM-019
- Baseline: §16, §17, §39
- Risks: R-016
- Invariants: I-045, I-084

V0.5 GPU-accelerated composition on inherited DRM drivers needs an allocation and dma-buf export/import primitive that GFX Buffer wraps. MEM allocates GPU-compatible, DMA-compatible MemoryObjects per MEM-019 and performs export/import without exposing DRM ioctls to applications (§39).

<!-- covers: INV-0736 -->

#### Out of scope
Buffer, Surface, and scene graph (GFX). Mesa process/fd adaptation (GFX, R-016). Personality bridge (MEM-025).

#### Acceptance criteria
- [ ] A native Component with GPU-buffer rights allocates a GPU-compatible MemoryObject that the compositor imports without a CPU copy of the payload.
- [ ] dma-buf export/import, when the decision requires it, is a kernel mechanism; native crates have no dma-buf fd type.
- [ ] Explicit GPU synchronization is required; there is no implicit-sync path for the object (I-084).
- [ ] Allocation failure returns a typed exhaustion or rights error and allocates no handle.

#### Verification
- Integration: compositor GPU composition on H-002 using MEM-allocated buffers; H-003 virtio-gpu CI path.
- Unit: `kernel:tests/mem/gpu_alloc_export_*` on H-002.

#### Evidence
- none

### MEM-025 · Import and export Linux Personality dma-buf and memfd descriptors as MemoryObjects
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: MEM-024, MEM-019
- Baseline: §3, §16, §17, §39
- Risks: R-016, R-020
- Invariants: I-025

V0.5-G09: a Wayland Linux application appears as a native window. Its buffers cross the personality bridge as MemoryObjects without copying, and native objects can be handed to personality processes as descriptors. Native Components without a personality Capability never see those descriptors (§3).

<!-- covers: INV-0736, INV-0317 -->

#### Out of scope
Wayland hosting (LNX-006). Native Buffer protocol (GFX). Dual execution worlds (ABI-025).

#### Acceptance criteria
- [ ] A personality dma-buf imported as a MemoryObject has the same physical pages; the compositor presents it without a CPU copy.
- [ ] A native GPU-compatible MemoryObject exported to a personality process is usable as the descriptor type the decision names.
- [ ] A native Component that does not hold a personality Capability cannot import or export descriptors; the attempt returns a typed rights error.
- [ ] Clipboard or window contents moving native to personality and back keep page identity where hardware permits.

#### Verification
- Integration: L1 Wayland app from C-002 beside a native app on H-002; page-identity check on presented buffers.
- Compat: C-002 GUI scenario window and clipboard on H-002 and H-003.

#### Evidence
- none

### MEM-026 · Implement MemoryObject borrowing with return to owner
- Type: build
- Milestone: V0.5
- Status: todo
- Size: L
- Owner: none
- Depends on: MEM-018, MEM-010
- Baseline: §17
- Invariants: I-063

Temporary access that returns to the owner, per MEM-018. The compositor and renderer borrow application buffers in V0.5. Tests cover return, revocation, and borrower death so a crashed compositor does not leak or pin application memory.

<!-- covers: INV-0328, INV-0338 -->

#### Out of scope
Charge accounting during borrow (MEM-015). Compositor rebind protocol (GFX, SVC).

#### Acceptance criteria
- [ ] A borrower maps the object; inspect shows owner unchanged and a borrow record.
- [ ] Successful return removes the borrower's mapping and leaves the owner as the sole holder.
- [ ] Borrower death unmaps and returns (or revokes) per the decision; the owner can map again.
- [ ] Behavior of owner mutate, transfer, or destroy during an outstanding borrow matches the decision and is tested.

#### Verification
- Unit: `kernel:tests/mem/borrow_return_*`, `borrow_death_*`, `borrow_revoke_*` on H-001 and H-002.
- Integration: compositor kill/rebind loop on H-003 does not leak application MemoryObjects.

#### Evidence
- none

### MEM-027 · Implement the copy-on-write property and CoW mappings
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: MEM-007, MEM-008
- Baseline: §16, §17, §34

Moved out of V0: CoW snapshots of a MemoryObject back file-mapping snapshots and cheap private views of immutable Package pages needed by the V0.5 apps (§16, §17). First write to a CoW mapping allocates a private page; unread pages stay shared.

<!-- covers: INV-0309, INV-0331 -->

#### Out of scope
Cost publication (MEM-013). File-backed snapshot interaction (MEM-023).

#### Acceptance criteria
- [ ] A CoW snapshot of a sealed object shares physical pages with the parent until the first write.
- [ ] First write to a snapshot mapping copies only the written page; other pages remain shared.
- [ ] Writes to the snapshot are not visible through the parent mapping, and writes to the parent after snapshot are not visible through the snapshot.
- [ ] Snapshot of a non-CoW-capable object returns a typed error and allocates no handle.

#### Verification
- Unit: `kernel:tests/mem/cow_snapshot_*` on H-001 and H-002.
- Integration: private view of a sealed library page used by a V0.5 app without copying the whole object.

#### Evidence
- none

### MEM-028 · Implement the DMA-compatible property and DMA suitability query
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: MEM-019, MEM-005
- Baseline: §16, §17, §33
- Threats: T-024

Pulled to V0.5 because GPU buffer allocation depends on it. Physically contiguous or IOMMU-mappable allocation with a queryable suitability attribute per MEM-019 (§16, §17). Native software queries DMA suitability; it does not program an IOMMU.

<!-- covers: INV-0310, INV-0332 -->

#### Out of scope
Per-domain IOMMU translation (MEM-037). Device wiring (HW-026). GPU allocation (MEM-024).

#### Acceptance criteria
- [ ] Allocating with DMA-compatible set produces an object whose suitability query reports mappable for DMA on H-002.
- [ ] Allocating DMA-compatible when the platform cannot satisfy it returns a typed error and allocates no handle, or reports unsuitability as the decision requires, never a silent non-DMA object flagged as suitable.
- [ ] Inspect shows the DMA-compatible property and the suitability result.

#### Verification
- Unit: `kernel:tests/mem/dma_suitability_*` on H-001 (emulated) and H-002.
- Integration: GPU buffer allocation consumes a DMA-compatible object on H-002.

#### Evidence
- none

### MEM-029 · Implement the shared property with multiple concurrent mappers
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: MEM-007, MEM-001
- Baseline: §16, §17, §38
- Threats: T-015

Multiple readers map one MemoryObject when the shared property is set, with mapper accounting and the coherence contract from MEM-001 (§16, §17). A non-shared object rejects a second mapper with a typed error.

<!-- covers: INV-0308, INV-0329 -->

#### Out of scope
Borrowing (MEM-026). GPU import (MEM-024). Glyph atlas policy (TXT).

#### Acceptance criteria
- [ ] Two Components map a shared object concurrently; inspect lists both mappers.
- [ ] Mapping a non-shared object from a second Component returns a typed error and installs no mapping.
- [ ] Writes by one CPU mapper are observed by the other according to the accepted coherence option.
- [ ] The last unmap leaves the object owned and unmapped, not destroyed, if an owner handle remains.

#### Verification
- Unit: `kernel:tests/mem/shared_map_*` on H-001 and H-002.
- Integration: V0 acceptance suite includes the second-mapper rejection case.

#### Evidence
- none

### MEM-030 · Prototype dma-buf-backed MemoryObjects with a NIC-to-GPU Zero-copy path
- Type: spike
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: MEM-005
- Baseline: §16, §17, §39
- Explores: S-006
- Risks: R-016

Demonstrates on H-002 that a dma-buf-backed MemoryObject can be filled by NIC DMA and imported by the GPU driver without a CPU copy of the payload. Evidence for MEM-019. Does not rewrite NIC or GPU drivers.

<!-- covers: GAP-0499 -->

#### Out of scope
Production GPU allocation (MEM-024). NET receive path (NET-018). Native GPU stack (I-045).

#### Acceptance criteria
- [ ] On H-002, a prototype object is filled by NIC DMA and imported by the GPU driver with physical-page identity recorded at each stage.
- [ ] The report records copies per stage and names hardware that could not complete the path.
- [ ] The report does not freeze S-006.

#### Verification
- Integration: prototype path on H-002 with page-identity probes at NIC fill and GPU import.
- Report: Can dma-buf be the MemoryObject backing without breaking inherited DRM? What copies remain on H-002? What is the export/import tax versus a native object? What fallback exists when NIC DMA cannot target the buffer?

#### Evidence
- none

### MEM-031 · Measure page-table and huge-page effects on startup and TLB for Package mappings
- Type: spike
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: MEM-007, MEM-008
- Baseline: §34, §38, §53
- Benchmarks: B-016
- Explores: S-006

The V0.5 warm-startup publication depends on how cheaply verified immutable pages are mapped and shared. The spike measures per-Component page-table cost and huge-page TLB effects for Package-sized mappings and feeds MEM-021.

<!-- covers: GAP-0549 -->

#### Out of scope
Policy decision (MEM-021). Production Package mapping (CMP, MEM-036).

#### Acceptance criteria
- [ ] Measurements exist on H-001 and H-002 for base pages, transparent huge pages, and explicit huge pages covering page-table memory and TLB-miss observations during map of a sealed Package-sized object.
- [ ] The report relates those measurements to B-016 warm-startup stages without restating a target number in prose.
- [ ] The report does not freeze S-006.

#### Verification
- Bench: B-016 method notes on H-001 and H-002 for the mapping stage only; publish.
- Report: What is the page-table memory per Component for shared sealed libraries? Do huge pages reduce TLB misses enough to matter at Terminal/Editor map time? Do shared page-table fragments avoid per-Component duplication? Which option does the evidence support for MEM-021?

#### Evidence
- none

### MEM-032 · Add an Interface review lint that large payloads move MemoryObjects by default
- Type: build
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: MEM-010
- Baseline: §15, §17, §67
- Invariants: I-063

Principle 6 first matters when V0.5 SDK and IDL surfaces expand. The lint flags interface methods that carry large byte arrays inline or copy where a MemoryObject move is possible. IPC owns codegen lowering; this lint is the review gate on native IDL and SDK crates.

<!-- covers: INV-1297, INV-0321 -->

#### Out of scope
IDL threshold decision (IPC-007, Q-005). Codegen lowering (IPC-036).

#### Acceptance criteria
- [ ] CI fails a native IDL method that passes a large inline byte array when a MemoryObject parameter is possible, unless an accepted decision exempts that method.
- [ ] CI fails an SDK API that copies a MemoryObject payload on the default path.
- [ ] The V0.5 Image Viewer and Text Editor IDL surfaces pass the lint.

#### Verification
- Unit: lint fixtures under `tools/lint/memobj_move_*` in CI.
- Review: ABI review-gate checklist includes the MemoryObject-move item for Layer 2 interfaces.

#### Evidence
- none

### MEM-033 · Benchmark copies and bytes moved in the NIC to Decoder to Renderer to GPU path
- Type: benchmark
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: MEM-022, MEM-039
- Baseline: §17, §53, §54
- Benchmarks: B-013, B-046
- Invariants: I-061

Each avoidable copy in a native data path is a defect to be measured and removed (§17). The harness counts copies and bytes per frame against a Linux socket-plus-decoder-plus-Wayland pipeline on the same hardware. B-046's first register target is V2 publish; this V1 task lands the harness and a published preview on V1 hardware so V2 is not the first measurement.

<!-- covers: INV-0321 -->

#### Out of scope
Standing B-046 gate at V2 (BEN-045, MED-028). Codec Packages (MED).

#### Acceptance criteria
- [ ] Reports on H-002 and H-004 list copies and bytes per stage from NIC or storage through decoder and renderer to GPU.
- [ ] The same session records the Linux baseline pipeline on that machine.
- [ ] Physical-page identity is the copy detector; a software-copy stage is counted.

#### Verification
- Bench: B-046 and B-013 on H-002 and H-004; V1 preview publish, V2 target per register.
- Integration: page-identity probes at each named stage.

#### Evidence
- none

### MEM-034 · Decide the MemoryObject locality and placement attribute model
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: MEM-017
- Baseline: §17, §37, §38
- Decision: D-0198
- Invariants: I-064

Fixes the attribute vocabulary (NUMA node, device-local, remote, persistent) exposed to placement and dispatch, jointly with HET, before the V2 ComputeDevice dispatch demo (§17, §37). Abstractions expose locality and cost rather than hiding them.

<!-- covers: INV-0720, INV-0333 -->

#### Out of scope
Implementation (MEM-041, MEM-040). ComputeDevice dispatch (HET). Persistent semantics (STO, Q-006).

#### Acceptance criteria
- [ ] Option A (query-only attributes), Option B (requestable placement at allocation and migrate), and Option C (placement as a Capability right) are evaluated for NUMA, device-local, remote, and persistent.
- [ ] The accepted option names the inspect fields HET dispatch will read.
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: HET and MEM reviewers sign off on the pull request.

#### Evidence
- none

### MEM-035 · Review MemoryObject L1 surfaces for freeze candidacy against future hardware
- Type: docs
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: MEM-017, MEM-043
- Baseline: §8, §16, §38, §65, §66
- Risks: R-007
- Invariants: I-040, I-058, I-100

V1 nominates Layer 1 freeze candidates; nothing is frozen here (I-040). The review checks every MemoryObject entry point against the §38 rules (CXL, persistent, disaggregated, unified, accelerator-local, non-coherent) and records which S-006 operations stay prototyped versus become candidates for the V4 freeze.

<!-- covers: INV-0052, INV-0703, INV-0704, INV-0706, INV-0707, INV-0709, INV-0710, INV-0711, INV-0712, INV-0713 -->

#### Out of scope
ABI-wide candidate list (ABI-034). V4 freeze (ABI-049, MEM-054).

#### Acceptance criteria
- [ ] Every MemoryObject map, unmap, transfer, borrow, charge, and property Operation is listed as freeze candidate or kept prototyped, with a §38 reason.
- [ ] No MemoryObject surface is marked `frozen` in the surfaces register.
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: ABI lead sign-off recorded on the pull request; surfaces register still lists S-006 as prototyped or open.

#### Evidence
- none

### MEM-036 · Apply the huge-page policy to immutable Package and MemoryObject mappings
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: MEM-021, MEM-016
- Baseline: §16, §34
- Benchmarks: B-016

Implements MEM-021 on immutable Package mappings and large MemoryObjects. Warm startup of Terminal and Editor is measured by B-016; this task is the mapping-side implementation, not the gate publication.

<!-- covers: GAP-0549 -->

#### Out of scope
B-016 publication (BEN-030). Address-space construction (CMP).

#### Acceptance criteria
- [ ] Sealed Package MemoryObjects are mapped using the accepted huge-page policy on H-002 and H-004.
- [ ] Inspect reports the huge-page property or fragment sharing actually in use, matching the decision.
- [ ] A regression test fails CI if Package mappings silently fall back to a rejected option.

#### Verification
- Integration: Terminal and Editor launch map path on H-002 and H-004; inspect dump of mapping attributes.
- Bench: mapping-stage notes consumed by B-016 on those machines.

#### Evidence
- none

### MEM-037 · Map DMA-compatible MemoryObjects through per-domain IOMMU translation
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: MEM-028
- Baseline: §16, §17, §33, §55
- Threats: T-020, T-024
- Invariants: I-038, I-074

User-space drivers and network zero-copy at V1 need device DMA confined to the objects a Component owns (§33). MEM maps DMA-compatible MemoryObjects through per-domain IOMMU translation. HW wires devices and decides the no-IOMMU fallback; this task implements the object-side tables and the fallback when no IOMMU is present.

<!-- covers: INV-0310, INV-0332 -->

#### Out of scope
IOMMU requirement decision and device wiring (HW-017, HW-026). Thunderbolt authorization (HW-057).

#### Acceptance criteria
- [ ] A DMA-compatible MemoryObject owned by Component A is mapped into A's IOMMU domain and is not mapped into B's.
- [ ] When no IOMMU is present, behavior matches HW-017: user-space DMA is refused or kernel-only, and a test shows it is not silently enabled.
- [ ] Destroy of the owner removes IOMMU mappings for that object.

#### Verification
- Unit: `kernel:tests/mem/iommu_domain_*` on H-002 (IOMMU present).
- Integration: no-IOMMU fallback test on H-001; user-space DMA path is not enabled.

#### Evidence
- none

### MEM-038 · Implement the pinned property so a MemoryObject is never swapped or migrated
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: MEM-005, MEM-004
- Baseline: §16, §22

Low-latency audio and DMA in V1 need pinned buffers. Pinned pages are limited per ResourceDomain with SCH and released on owner death. Unpin is explicit; the object then becomes eligible for the reclaim policy.

<!-- covers: INV-0314 -->

#### Out of scope
Reclaim of unpinned objects (MEM-042). Audio path (AUD). Pin charge in the domain budget (SCH).

#### Acceptance criteria
- [ ] A pinned object is not swapped or migrated while the pin holds; a reclaim attempt leaves it resident.
- [ ] Pin that would exceed the domain's pin limit returns a typed exhaustion error and does not pin.
- [ ] Owner death or explicit unpin releases the pin; inspect shows the property clear.
- [ ] DMA-compatible objects used for V1 audio or GPU buffers can be pinned.

#### Verification
- Unit: `kernel:tests/mem/pin_reclaim_*` on H-001 and H-004.
- Integration: audio or GPU buffer remains resident across a memory-pressure injection.

#### Evidence
- none

### MEM-039 · Transfer network-received MemoryObjects to a Decoder without copying
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: MEM-010, MEM-028, MEM-037
- Baseline: §17
- Invariants: I-063

§17 first stage: a MemoryObject filled by NIC DMA is handed to a media decoder by ownership. NET owns the fill; MED owns the decoder; MEM owns the move and DMA-compatible object the NIC targets. Production NIC DMA landing is NET-025 at V2; this task ships the transfer API and a V1 path using the NET spike's mechanism on H-002 and H-004.

<!-- covers: INV-0323 -->

#### Out of scope
NIC driver rewrite (NET). Isolated codecs (MED-024). Renderer stage already in V0.5 (MEM-022).

#### Acceptance criteria
- [ ] A DMA-compatible MemoryObject filled by the V1 receive mechanism transfers to a decoder Component with physical-page identity unchanged.
- [ ] After transfer the network Component is not a mapper or owner.
- [ ] The path runs on H-002 and H-004; machines that cannot DMA into the object record a documented fallback copy counted by the pipeline harness.

#### Verification
- Integration: receive-to-decoder page-identity test on H-002 and H-004.
- Bench: stage consumed by MEM-033.

#### Evidence
- none

### MEM-040 · Track and expose NUMA locality of a MemoryObject
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: MEM-034
- Baseline: §17, §38
- Invariants: I-064

The system understands NUMA locality of a MemoryObject (§17). Allocation honors a preferred node from the placement model; locality is queryable for placement and `os inspect`. No assumption of uniform memory access (§38).

<!-- covers: INV-0334, INV-0333 -->

#### Out of scope
Full placement attribute set (MEM-041). Scheduler intent influencing placement (SCH-049).

#### Acceptance criteria
- [ ] Allocating with a preferred NUMA node places pages on that node when the hardware has it; inspect reports the node.
- [ ] On single-node H-001, the query returns the single node and does not fail.
- [ ] On H-002, cross-node allocation is observable in inspect when requested.

#### Verification
- Unit: `kernel:tests/mem/numa_place_*` on H-001 and H-002.
- Integration: inspect memory listing includes NUMA node on H-002.

#### Evidence
- none

### MEM-041 · Implement MemoryObject placement attributes exposed to dispatch
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: MEM-034, MEM-040
- Baseline: §17, §37, §38
- Invariants: I-064

Implements MEM-034 so HET ComputeDevice dispatch can read where an object lives and request placement. Attributes cover NUMA node, device-local, remote, and persistent as vocabulary; device-local backing itself is V2.

<!-- covers: INV-0720 -->

#### Out of scope
Device-local providers (MEM-046). Dispatch (HET-017). Persistent property (STO).

#### Acceptance criteria
- [ ] Inspect and a typed query return the placement attributes named by the decision for a live object.
- [ ] Requesting a placement the backing cannot satisfy returns a typed error and does not silently lie about locality.
- [ ] HET dispatch can read the attributes without a Layer 1 break.

#### Verification
- Unit: `kernel:tests/mem/placement_attr_*` on H-001 and H-002.
- Integration: inspect dump consumed by a HET CPU ComputeDevice sample on H-002.

#### Evidence
- none

### MEM-042 · Define reclaim, swap and migration behavior for unpinned MemoryObjects
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: MEM-038
- Baseline: §16, §23
- Threats: T-016

V1 daily-driving on H-004 needs memory pressure handled: which MemoryObjects retained Linux mm may swap or migrate, how ownership and shared mappings survive migration, and typed pressure signals to the owner. SCH owns exhaustion policy; MEM defines object-level reclaim relative to pin, DMA-compatible, and sealed state.

#### Out of scope
Discardable caches (MEM-047). Budget exhaustion decision (SCH-016). Pin implementation (MEM-038).

#### Acceptance criteria
- [ ] Unpinned anonymous MemoryObjects are eligible for reclaim; pinned, and DMA-compatible objects that the policy forbids swapping, remain resident under injected pressure.
- [ ] After migration, inspect still names the same owner and mapper set; physical pages may change except where identity is required (DMA).
- [ ] The owner receives a typed pressure notification before OOM termination, matching SCH exhaustion policy.
- [ ] Shared sealed Package pages are not privately copied by reclaim.

#### Verification
- Integration: pressure injection on H-004 with pinned versus unpinned objects; inspect before and after.
- Unit: `kernel:tests/mem/reclaim_migrate_*` on H-001.

#### Evidence
- none

### MEM-043 · Study bounds-enforced pointers and bounded MemoryObject views in the ABI
- Type: spike
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: MEM-007
- Baseline: §8, §16, §65
- Explores: S-006
- Invariants: I-058

§8 study before L1 freeze candidates: whether the ABI can express bounded sub-views of a MemoryObject without committing to a hardware capability model. Complements CAP CHERI spikes; application-visible Capabilities stay conceptually stable if enforcement later moves to tags or hardware capabilities.

<!-- covers: INV-0191 -->

#### Out of scope
Hardware capability mappings (MEM-049). Capability handle encoding (CAP). Freeze candidacy write-up (MEM-035).

#### Acceptance criteria
- [ ] The report prototypes software-bounded sub-views (offset and length on a parent MemoryObject) and records whether they can be Capabilities without a new object type.
- [ ] The report lists ABI assumptions that would break on CHERI-like bounds-enforced pointers.
- [ ] The report does not freeze S-006 and does not require CHERI hardware in V1.

#### Verification
- Report: Can a bounded view be a derived Capability with subset rights? Does the view need kernel metadata, or can length live in the handle? What breaks on CHERI if views are software-only? What must stay prototyped at V1 so V4 can adopt hardware bounds without a Layer 1 break?

#### Evidence
- none

### MEM-044 · Preserve pinned and DMA-compatible MemoryObjects across suspend and resume
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: MEM-038, MEM-028
- Baseline: §16, §32

V1 suspend/resume with display and audio functional afterwards requires GPU and audio buffers backed by MemoryObjects to survive. MEM adds a post-resume check to the PWR cycle harness: pinned and DMA-compatible objects remain mapped, suitable, and owned. Required by V1-G07 (Suspend and resume succeed on laptop and desktop).

#### Out of scope
Cycle harness and wake policy (PWR-014). Audio and GPU rebind (AUD, GFX).

#### Acceptance criteria
- [ ] After a suspend/resume cycle on H-004 and H-002, pinned DMA-compatible objects used as GPU or audio buffers remain pinned, mapped, and DMA-suitable.
- [ ] Owner and mapper set are unchanged across the cycle.
- [ ] A failure is a typed inspectable error, not a silent stale mapping.

#### Verification
- Integration: hook in the PWR cycle harness on H-002 and H-004; MEM checks run each cycle.
- Manual: inspect memory before and after one cycle on H-004.

#### Evidence
- none

### MEM-045 · Decide encrypted MemoryObject key ownership and hardware encryption
- Type: adr
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: MEM-007
- Baseline: §16, §51
- Decision: D-0194
- Invariants: I-021

Decide who owns the key for an encrypted MemoryObject, who may map plaintext, and whether hardware memory encryption (SME/TDX class) is a software-visible map or a backing-provider property. Parallel to STO encrypted-at-rest (INV-0318). Options: per-Component key in the secrets service; ResourceDomain-held key; hardware memory encryption with no software plaintext map.

<!-- covers: INV-0319 -->

#### Out of scope
Implementation (MEM-048). Disk encryption (SEC, STO). Answering Q-007 (SEC).

#### Acceptance criteria
- [ ] At least two options are recorded, including a hardware-encryption option that never maps plaintext in software.
- [ ] The accepted option names the Capability that authorises a plaintext map, or records that no software plaintext map exists.
- [ ] The Decision file lists STO INV-0318 as a related persistent-property Decision, not as a substitute.

#### Verification
- Review: MEM and SEC reviewers sign off on the pull request.
- Report: the Decision records the rejected options and why.

#### Evidence
- none

### MEM-046 · Implement accelerator-local and unified-memory backing providers
- Type: build
- Milestone: V2
- Status: todo
- Size: L
- Owner: none
- Depends on: MEM-017, MEM-041, MEM-019
- Baseline: §16, §17, §38
- Invariants: I-024

V2: a ComputeDevice workload runs on the GPU. MemoryObjects can live in device memory or unified CPU/GPU memory via the V0 provider decision. HET consumes locality; MEM provides the backings. No native GPU driver stack (§39, I-045).

<!-- covers: INV-0712, INV-0713 -->

#### Out of scope
ComputeDevice dispatch (HET-015). CXL and disaggregated media (MEM-056). Placement vocabulary (already decided).

#### Acceptance criteria
- [ ] Allocating device-local on H-002 places the object in GPU memory; inspect reports device-local and the device id.
- [ ] Unified CPU/GPU backing, where hardware has it, is queryable and mappable from CPU and GPU without a silent extra copy; remaining copies are counted.
- [ ] A CPU-only machine returns a typed error for device-local allocation and allocates no handle.
- [ ] Native software still holds MemoryObject Capabilities, not vendor device pointers.

#### Verification
- Integration: HET Throughput-on-GPU sample on H-002 reads a device-local MemoryObject.
- Unit: `kernel:tests/mem/device_local_*` on H-002; CPU-only refusal on H-001.

#### Evidence
- none

### MEM-047 · Add a discardable property and pressure-driven release of MemoryObjects
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: MEM-042
- Baseline: §16, §23
- Threats: T-016

V2 laptops with limited RAM: caches held as discardable MemoryObjects are reclaimed under ResourceDomain pressure with a typed notification instead of an OOM kill. SCH owns the pressure signal; MEM drops discardable pages and notifies the owner.

#### Out of scope
Pin and swap policy (MEM-038, MEM-042). Domain exhaustion (SCH-014).

#### Acceptance criteria
- [ ] A discardable object is dropped under injected domain pressure; the owner receives a typed notification and the mapping is gone.
- [ ] A non-discardable object is not dropped by this path.
- [ ] After discard, creating a replacement object of the same size succeeds within the budget.
- [ ] Discard does not run on pinned or DMA-mapped objects.

#### Verification
- Integration: pressure injection on H-004 and H-005 with discardable caches versus pinned buffers.
- Unit: `kernel:tests/mem/discard_*` on H-001.

#### Evidence
- none

### MEM-048 · Implement the encrypted MemoryObject property and key-ownership semantics
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: MEM-045, MEM-007
- Baseline: §16, §51
- Invariants: I-021

§16 encrypted property: who may map plaintext, and how hardware memory encryption is used, once Q-007 is answered. Parallel to STO encrypted-at-rest. Keys are Capabilities, never ambient. Mapping plaintext without the key Capability returns a typed rights error.

<!-- covers: INV-0315, INV-0319 -->

#### Out of scope
Answering Q-007 (SEC). Disk encryption (SEC, STO). Persistent MemoryObject (STO).

#### Acceptance criteria
- [ ] An encrypted object maps plaintext only for a holder of the key Capability named by Q-007's answer.
- [ ] A mapper without the key receives a typed rights error and no plaintext mapping.
- [ ] Keys are Capabilities; no process-wide or ambient key slot exists.
- [ ] Inspect does not print key material.

#### Verification
- Unit: `kernel:tests/mem/encrypted_map_*` on H-001 and H-002.
- Review: SEC sign-off that key ownership matches the answered Q-007 semantics.

#### Evidence
- none

### MEM-049 · Study hardware memory capabilities as an enforcement backend for mappings
- Type: spike
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: MEM-043
- Baseline: §8, §16, §65
- Explores: S-006
- Invariants: I-058

§8 study, with CAP CHERI work, of whether MemoryObject mappings could be enforced by hardware capabilities without ABI change. Informs the V4 freeze: application-visible map/unmap/transfer stay stable if the backend becomes tags or CHERI-like capabilities.

<!-- covers: INV-0193 -->

#### Out of scope
CAP handle mapping on CHERI (CAP-039). Freeze (MEM-053). 1.0 hardware enforcement promise (non-goal).

#### Acceptance criteria
- [ ] The report maps current S-006 operations onto a CHERI or tagged-memory backend using CAP's V1 mapping notes.
- [ ] The report lists ABI changes that would be required versus changes that can stay kernel-internal.
- [ ] The report does not freeze S-006 and does not promise hardware enforcement at 1.0.

#### Verification
- Report: Which mapping checks become hardware-enforced without a userspace ABI change? What handle layout from CAP is required? What must remain prototyped until V4? What is explicitly not promised at 1.0?

#### Evidence
- none

### MEM-050 · Build a property-based MemoryObject isolation invariant suite
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: MEM-010, MEM-008, MEM-006, MEM-026, MEM-023, MEM-025
- Baseline: §16, §17, §51
- Threats: T-015
- Invariants: I-063

V3 multi-user and V4 external audit need machine-checked evidence that ownership, sealing, and W^X hold across transfer, borrow, CoW, file-backed, and dma-buf import. The suite runs on every kernel build.

<!-- covers: INV-0953, INV-0327, INV-0307, INV-0312 -->

#### Out of scope
Fuzz oracles (MEM-052). External audit process (SEC, V4).

#### Acceptance criteria
- [ ] Property tests assert single owner after transfer, sealed never writable, and no writable-plus-executable mapping, on every path listed in the description.
- [ ] The suite runs in CI on every kernel build for H-001 and fails the build on violation.
- [ ] Multi-user isolation: an object owned in session A is not mappable in session B without a transferred Capability.

#### Verification
- Unit: `kernel:tests/mem/invariants_*` property suite on H-001.
- Integration: two-session negative test on a V3 multi-user image.

#### Evidence
- none

### MEM-051 · Write the MemoryObject Layer 1 ABI reference pages
- Type: docs
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: MEM-035
- Baseline: §16, §66, §56.5

V3 exit: every Layer 1 reference page is written. Every MemoryObject entry point, property, and Capability right is documented through DOC's IDL-to-docs pipeline. MEM authors the semantics; DOC generates pages.

<!-- covers: INV-0052 -->

#### Out of scope
Site generation (DOC-023, DOC-010). ABI-wide completeness (ABI-046).

#### Acceptance criteria
- [ ] Every MemoryObject Operation, property, and right has a reference page generated from IDL plus MEM-authored prose.
- [ ] Pages state that native software does not see POSIX mmap, memfd, or dma-buf as APIs.
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: DOC and ABI reviewers sign off; coverage CI lists no missing MemoryObject symbols.

#### Evidence
- none

### MEM-052 · Add MemoryObject syscall fuzz targets with ownership and W^X oracles
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: MEM-050
- Baseline: §16, §51

V3 continuous syscall fuzzing with no open crashers. MEM supplies targets and invariant oracles (single owner, no post-transfer access, sealed never writable) for BLD's fuzzing fleet. Oracles are the isolation suite encoded for the fuzzer.

<!-- covers: INV-0953 -->

#### Out of scope
Fuzz infrastructure and crasher age gate (BLD-035, BLD-063). syzkaller port (BLD-016).

#### Acceptance criteria
- [ ] Fuzz targets cover create, map, unmap, transfer, borrow, seal, pin, and property changes.
- [ ] Oracles fail the input if a post-transfer sender mapping works, if a sealed object is written, or if W^X is violated.
- [ ] Targets are wired into the BLD nightly fleet.

#### Verification
- Fuzz: `kernel:fuzz/memobj_*` in the BLD nightly fleet; no known open crasher attributed to these targets older than the V3 window.
- Unit: oracle fixtures under `kernel:tests/mem/fuzz_oracles_*`.

#### Evidence
- none

### MEM-053 · Run the MemoryObject fossilization review before the L1 freeze
- Type: docs
- Milestone: V4
- Status: todo
- Size: S
- Owner: none
- Depends on: MEM-035, MEM-049, MEM-046
- Baseline: §8, §38, §65, §70
- Risks: R-054
- Invariants: I-100

Standing §38 rules are verified at freeze: the report feeds ABI's fossilization review and lists any MemoryObject surface deferred by decision so later CXL, persistent, disaggregated, unified, or accelerator-local hardware does not require a major-version break.

<!-- covers: INV-0703, INV-0704, INV-0706, INV-0707, INV-0709, INV-0710, INV-0711, INV-0712, INV-0713 -->

#### Out of scope
ABI-wide freeze ADR (ABI-049). 1.0 fossilization pass (ABI-054).

#### Acceptance criteria
- [ ] The report checks every freeze-candidate MemoryObject Operation against non-coherent, non-DRAM, non-uniform, non-local, CXL, persistent, disaggregated, unified, and accelerator-local scenarios.
- [ ] Surfaces that cannot honor §38 are listed as deferred by decision, not frozen.
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: ABI lead sign-off recorded on the pull request; the freeze ADR cites this report.

#### Evidence
- none

### MEM-054 · Add conformance tests for every MemoryObject Layer 1 entry point
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: MEM-050, MEM-053, MEM-051, MEM-011, MEM-017
- Baseline: §16, §65, §66
- Freezes: S-006
- Risks: R-054
- Invariants: I-040

V4 freezes S-006 with a conformance test per entry point and deprecated entry points removed. MEM owns its portion of the ABI conformance suite. Spike MEM-011 and adr MEM-002 plus MEM-003 sit in the dependency closure; no earlier task freezes S-006.

<!-- covers: INV-0052 -->

#### Out of scope
ABI freeze decision (ABI-049). Golden binary suite (ABI-047).

#### Acceptance criteria
- [ ] Every frozen MemoryObject entry point has a conformance test in the V4 suite.
- [ ] Deprecated MemoryObject entry points are removed; the suite fails if they reappear.
- [ ] S-006 is listed `frozen` only when this task is done and the ABI freeze ADR is accepted.
- [ ] A V4-built test binary runs on a later V4 build without source change.

#### Verification
- Integration: MEM portion of the ABI conformance suite on H-001 and every in-scope Tier 1 machine named by V4 hardware scope.
- Review: ABI lead records S-006 freeze against this suite and the fossilization report.

#### Evidence
- none

### MEM-055 · Write the retained-mm CVE triage runbook for MemoryObject in 1.x support
- Type: docs
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: MEM-005
- Baseline: §2, §56.4, §70
- Risks: R-062
- Invariants: I-009

1.0 support: upstream shmem, dma-buf, and mm CVEs must be mapped to MemoryObject exposure and fixed within the published CVE SLA. The runbook records the mapping and the regression tests to run. REL owns ingestion and the SLA; KRN owns backport; MEM owns which MemoryObject paths are exposed.

<!-- covers: INV-1323 -->

#### Out of scope
SLA publication (REL-060, REL-064). Kernel backport (KRN).

#### Acceptance criteria
- [ ] The runbook maps shmem, dma-buf, and mm CVE classes to MemoryObject create, map, transfer, DMA, and GPU-export paths.
- [ ] Each class names the regression tests in the MEM suite to run before a generation ships.
- [ ] Review sign-off from REL and MEM is recorded on the pull request.

#### Verification
- Review: REL and MEM sign-off recorded on the pull request.

#### Evidence
- none

### MEM-056 · Study CXL-attached and disaggregated memory as MemoryObject backing providers
- Type: spike
- Milestone: LATER
- Status: todo
- Size: L
- Owner: none
- Depends on: MEM-017, MEM-046
- Baseline: §38, §43, §57
- Explores: S-006
- Invariants: I-024, I-047

Parked research: exercises the V0 provider model against remote and CXL memory on hardware not in any 1.0 tier. Keeps distributed memory out of the 1.0 promise (§43, §57). Does not make the kernel a distributed system.

<!-- covers: INV-0707, INV-0709, INV-0711 -->

#### Out of scope
1.0 MemoryObject freeze (already done at V4). Remote interfaces as a kernel concern (forbidden). Device-local GPU memory (MEM-046).

#### Acceptance criteria
- [ ] The report runs the provider interface against at least one CXL or disaggregated configuration, or records that no such hardware was available and lists the interface gaps found on paper.
- [ ] The report names ABI changes that would be required versus provider-internal changes.
- [ ] The report does not add a 1.0 gate and does not freeze S-006.

#### Verification
- Report: Does the V0 provider interface accept CXL-attached memory without a Layer 1 break? What placement attributes are missing? What coherence and failure semantics differ from DRAM? What must stay out of 1.x?

#### Evidence
- none
