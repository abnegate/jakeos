# ABI · Native kernel ABI
- Prefix: ABI
- Lead: none
- Baseline: §7, §8, §12, §65, §66

<!-- roadmap:generated:begin summary -->
Tasks: 54 live, 1 done, 0 in-progress, 53 todo, 0 dropped. Ready: 4. Blocked: 49. Weighted: 1%.
<!-- roadmap:generated:end -->

## Scope

This workstream owns the Native ABI: the Layer 1 interface between native software and the kernel. It covers the Object handle table and typed Object registry, the kernel entry mechanism and the bound object-operation surface, the error model, Layer 1 version identification and feature negotiation from V0, the machine-readable ABI definition and the generators that emit the C-compatible header, golden snapshots, fuzz descriptions and conformance cases from it, the ABI review gate and snapshot check, dual Native ABI and Linux ABI execution worlds at the entry layer, reservation of ComputeDevice object types and Operation slots, the Layer 1 freeze process (prototyped through V0, freeze candidates at V1, frozen at V4, declared stable for 1.x at 1.0), per-layer deprecation policy, and the fossilization review against future hardware. It owns Layer 1 ABI surfaces S-001, S-002, S-004 and S-011.

## Out of scope

Capability rights encoding, derivation and revocation (CAP, S-003). Component create, destroy and panic semantics (CMP, S-007). Operation ring layout, Task and TaskGroup (TSK, S-005, S-008). Channel transport, IDL and Layer 2 evolution rules (IPC, S-012, S-013, S-014). MemoryObject mapping (MEM, S-006). ResourceDomain budgets (SCH, S-009). Tracing event records (OBS, S-010). SDK crates, language bindings beyond the C-compatible header, and the `os` CLI (SDK). Linux personality syscall retention and translation (LNX). Windows personality (WIN). ComputeDevice dispatch implementation (HET). Wasm as a machine ABI (WASM). Kernel fork, retained Linux syscall path and divergence phases (KRN). CI plumbing for the conformance suite and syzkaller (BLD). Publishing generated reference pages (DOC). License firewall and RFC process (GOV).

## Tasks

### ABI-001 · Publish native entry/return cost per mechanism against Linux syscall and io_uring
- Type: benchmark
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: ABI-019, Q-001
- Baseline: §65, §54
- Benchmarks: B-009

Measure entry and return cost of each prototype from ABI-019 on a no-op Operation so the Layer 1 entry-mechanism Decision can cite a published B-009 report. V0 is publish-only. Comparison baselines are the Linux syscall path and io_uring NOP submit-to-completion named on B-009. Native software still never sees those Linux interfaces.

<!-- covers: GAP-0500 -->

#### Out of scope
Standing Operation latency publication after the Decision (BEN). io_uring lineage evaluation (TSK).

#### Acceptance criteria
- [ ] A report exists under `reports/benchmarks/B-009/` for H-001 and for H-002 covering syscall-per-Operation, shared submission page, and trampoline entry.
- [ ] Each report names the B-009 method, the Linux syscall and io_uring NOP baselines, and the mechanism under test.
- [ ] The reports are cited from the Decision record of ABI-008.

#### Verification
- Bench: B-009 on H-001 and H-002; target per register (V0 publish).
- Review: BEN lead confirms the reports follow the B-009 method recorded in the register.

#### Evidence
- none

### ABI-002 · Implement the Native ABI entry layer in the Linux-derived kernel
- Type: build
- Milestone: V0
- Status: todo
- Size: L
- Owner: none
- Depends on: ABI-008, ABI-012, ABI-014, ABI-015, LNX-001
- Baseline: §6, §7, §65
- Risks: R-004

Implement the Native ABI entry layer in the Linux-derived kernel so every V0 primitive test reaches the kernel through the chosen entry mechanism and object-operation dispatch (§6 Phase A, §65). The retained Linux syscall path stays intact for the L0 corpus. Native software enters only through this layer.

<!-- covers: INV-1321 -->

#### Out of scope
Linux syscall implementation (LNX). Operation ring internals (TSK). Capability table storage (CAP).

#### Acceptance criteria
- [ ] A native Component on `qemu-x86_64` and `hw-h002` submits a no-op Operation through the Native ABI entry layer and observes a completion without using a Linux syscall number.
- [ ] The Linux syscall path used by C-001 still boots and is not routed through the Native ABI dispatcher.
- [ ] `os inspect` on a live native Component shows the entry-layer identity chosen by ABI-008.
- [ ] No `unsafe` outside the entry-layer crate files listed in the pull request.

#### Verification
- Unit: `kernel:tests/abi/entry_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Integration: V0 primitive suite reaches the kernel only through this layer on H-001 and H-002.
- Compat: C-001 scenario run on H-001 and H-002 after the layer lands.
- Demo: native Component A to B round trip on H-002 enters through this layer, shown with `os trace`.

#### Evidence
- none

### ABI-003 · Add build and lint rules forbidding native crates from linking Linux Personality or libc
- Type: build
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: ABI-018, BLD-082
- Baseline: §3, §13, §57
- Invariants: I-005, I-006, I-025, I-046, I-049

Enforce the §3 compatibility firewall as a lint on every native crate from V0: native software does not link the Linux personality, libc, or a Wasm runtime used as the machine ABI (§57). Personalities consume the Native ABI and never extend it. The Layer 3 libc-compatible library decided by SDK-097 is not Linux `libc`; it is named in the lint allowlist by crate name and is the only C runtime a native crate may link.

<!-- covers: INV-0867, INV-0010, INV-0011, INV-1121, INV-1123, INV-0272, INV-1130 -->

#### Out of scope
POSIX-shaped name lint on ABI headers (ABI-018). Personality opt-in (LNX). Wasm runtime as machine ABI (WASM).

#### Acceptance criteria
- [ ] A native crate whose `Cargo.toml` depends on `libc` or the Linux personality crate fails CI on `qemu-x86_64`.
- [ ] A native crate that imports a Wasm runtime as the machine ABI fails the same lint.
- [ ] The ImageDecoder sample crate passes the lint.

#### Verification
- Unit: `sdk:tests/lint/firewall_*` on CI matrix entry `qemu-x86_64`.
- Integration: pre-merge lint job on every native crate in the V0 image.

#### Evidence
- none

### ABI-004 · Implement the Layer 1 version handshake and its forward/backward compatibility test
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: ABI-016, ABI-002
- Baseline: §12, §65
- Invariants: I-041

Implement the Layer 1 handshake that identifies ABI version and features at kernel entry, not only as an IDL message test (§12, §65 rule 6). An unknown newer field is accepted by an older receiver and an older message by a newer receiver. This test is the first ABI conformance case and is retained permanently.

<!-- covers: INV-1274, INV-0147 -->

#### Out of scope
IDL unknown-field tests (IPC). Layer 2 interface version negotiation (IPC).

#### Acceptance criteria
- [ ] A native Component built against ABI v0 completes the handshake with a kernel that advertises a newer optional field, and the Operation succeeds.
- [ ] A native Component that omits a field the kernel knows completes the handshake with a newer kernel, and the Operation succeeds.
- [ ] Handshake failure returns the typed error named by ABI-009 and allocates no handle.
- [ ] The handshake test is listed as case 0 of the conformance suite skeleton.

#### Verification
- Unit: `kernel:tests/abi/handshake_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Integration: retained as the first ABI conformance case on every merge to main.
- Demo: handshake success and unknown-field acceptance shown in `os inspect` on H-002.

#### Evidence
- none

### ABI-005 · Implement the Object<T> typed registry with type identifier checked on every Operation
- Type: build
- Milestone: V0
- Status: todo
- Size: L
- Owner: none
- Depends on: ABI-010, ABI-013, ABI-009, ABI-002
- Baseline: §7, §65
- Risks: R-003
- Threats: T-003
- Invariants: I-015, I-028

Implement `Object<T>` as the typed kernel object with a type identifier checked on every Operation (§7). Userspace holds only a Capability to the object. Forged handles and wrong-type operations fail with the typed error from ABI-009 and allocate no handle.

<!-- covers: INV-0049, INV-0158 -->

#### Out of scope
Capability rights encoding and derivation (CAP). Which types live in kernel versus user services (ABI-013). Channel endpoints (IPC).

#### Acceptance criteria
- [ ] Operating on a forged handle returns `Error::Rights` (or the equivalent named by ABI-009) and allocates no handle.
- [ ] Operating on a live handle with the wrong type identifier returns a typed error and does not invoke the target object's operation.
- [ ] `os inspect` prints type identifier, owning Component and live handle count for every V0 object kind.
- [ ] Property tests cover unforgeability and type mismatch on `qemu-x86_64` and `hw-h002`.

#### Verification
- Unit: `kernel:tests/abi/object_registry_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Fuzz: `kernel:fuzz/abi_handle` one hour nightly without panic.
- Integration: V0 forged-handle and wrong-type cases of the isolation demo on H-002.

#### Evidence
- none

### ABI-006 · Institute the ABI review Gate checklist enforcing the §65 rules on every L1 change
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: ABI-011, ABI-018, GOV-005, BLD-082
- Baseline: §3, §8, §38, §65
- Risks: R-007
- Invariants: I-002, I-013, I-026, I-055, I-056, I-057, I-058

Collapse the §65 rules into one review-gate checklist on every Layer 1 change: reject interfaces justified only by a POSIX, Linux or Windows equivalent; flag x86-64-only and page-table-layout assumptions; reject exposed kernel internals (`task_struct`, `mm_struct`, cgroups, namespaces); require a deprecation and compatibility-shim plan; keep the surface Capability-based, Component-oriented, asynchronous and object-based. No Layer 1 surface is frozen in V0.

<!-- covers: INV-0081, INV-0103, INV-0034, INV-0003, INV-1277, INV-1275, INV-0708, INV-0702, INV-0198, INV-1276, INV-1305, INV-1283, INV-1268 -->

#### Out of scope
Mechanical snapshot diff (ABI-027). RFC process text (GOV). Capability rights review (CAP).

#### Acceptance criteria
- [ ] A pull request that adds a Layer 1 entry justified only by a POSIX, Linux or Windows equivalent fails the review-gate job.
- [ ] A pull request that names `task_struct`, `mm_struct`, cgroups, namespaces or page-table layout in a native header fails the review-gate job.
- [ ] A pull request that changes Layer 1 without a linked adr id fails the review-gate job.
- [ ] The checklist is a checked-in file consumed by CI and by the RFC template.

#### Verification
- Unit: `tools:tests/abi_review_gate_*` on CI matrix entry `qemu-x86_64`.
- Integration: pre-merge job on a fixture pull request for each failing class.
- Review: ABI lead sign-off recorded on the pull request that lands the checklist.

#### Evidence
- none

### ABI-007 · Decide the binding substrate: C-compatible ABI header plus IDL-generated language stubs
- Type: adr
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: ABI-011
- Baseline: §50, §65
- Decision: D-0003
- Invariants: I-002

Decide how languages bind to Layer 1 so the Native ABI stays language-neutral (§50). Options are a stable C-compatible header plus IDL-generated per-language stubs, an IDL-only substrate with no C header, and a Rust-native ABI with other languages as afterthoughts. The Decision is recorded before the Rust runtime and C header are built.

<!-- covers: INV-0945, INV-0944 -->

#### Out of scope
IDL language choice (IPC). Rust SDK crate (SDK). C wrapper safety (SDK).

#### Acceptance criteria
- [ ] The Decision record evaluates C-compatible header plus IDL stubs, IDL-only, and Rust-native ABI as named options.
- [ ] The accepted option states how a non-Rust language reaches Layer 1 without a second Native ABI.
- [ ] Review records ABI lead and SDK lead sign-off on the pull request.

#### Verification
- Review: ABI lead and SDK lead sign-off recorded on the pull request.

#### Evidence
- none

### ABI-008 · Decide the Native ABI entry mechanism and the maximum count of kernel entry points
- Type: adr
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: ABI-019, ABI-001
- Baseline: §65
- Decision: D-0005
- Risks: R-003, R-007
- Invariants: I-055

Decide how a Component enters the kernel and bound the Layer 1 entry-point count (§65 rule 1). Options are a syscall instruction per Operation, a shared submission page with rare doorbell syscalls, and vDSO-style trampolines. The Decision cites the B-009 reports from ABI-001. Surface S-002 becomes prototyped, not frozen.

<!-- covers: GAP-0500, INV-1269, INV-1279 -->

#### Out of scope
Operation ring layout (TSK). Implementation of the chosen mechanism (ABI-002). Layer 1 freeze (ABI-049).

#### Acceptance criteria
- [ ] The Decision record evaluates syscall-per-Operation, shared submission page with rare doorbell syscalls, and trampoline entry as named options.
- [ ] The accepted option records a maximum count of kernel entry points and lists rejected options with reasons.
- [ ] The Decision lists S-002 and cites a B-009 report produced by ABI-001.
- [ ] Surface S-002 is recorded as prototyped, not frozen.

#### Verification
- Review: ABI lead sign-off recorded on the pull request.

#### Evidence
- none

### ABI-009 · Decide the Operation result error model: typed enum per kind or uniform error Object
- Type: adr
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: ABI-022, ABI-020
- Baseline: §19, §65
- Decision: D-0006
- Threats: T-003

Decide the failure encoding for Operation results so forged handles, denied derivation and Timeout are stable across the ABI (§19). Options are a typed error enum per Operation kind, a uniform error object, and a hybrid with a uniform class plus a per-kind payload. Native errors are not errno values (S-004).

<!-- covers: INV-0375, INV-1279 -->

#### Out of scope
Capability denial audit log (CAP). Operation completion delivery (TSK).

#### Acceptance criteria
- [ ] The Decision record evaluates typed enum per kind, uniform error object, and hybrid class-plus-payload as named options.
- [ ] The accepted option names the encoding for forged handle, wrong type, denied rights, timeout, cancellation and resource exhaustion.
- [ ] The Decision lists S-004 and states that native errors are not errno values.
- [ ] Surface S-004 is recorded as prototyped, not frozen.

#### Verification
- Review: ABI lead sign-off recorded on the pull request.

#### Evidence
- none

### ABI-010 · Decide the Layer 1 handle word: packing of the CAP-008 representation, type tag and Generation
- Type: adr
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: CAP-008, ABI-022
- Baseline: §7, §8, §65
- Decision: D-0007
- Risks: R-003, R-012
- Threats: T-003
- Invariants: I-015, I-028, I-058

Decide how the handle representation chosen by CAP-008 is packed into the Layer 1 syscall word (S-001): where the type tag and generation live, how many bits stay reserved for a future sealed-pointer layout, and what the kernel checks at the boundary (§7, §8). CAP-008 chooses the representation and table layout; this Decision fixes only its ABI-visible packing so the two are not decided twice. Surface S-001 becomes prototyped, not frozen.

<!-- covers: INV-0186, INV-1271, INV-1276, INV-1279 -->

#### Out of scope
Rights word encoding (CAP, S-003). Capability table implementation (CAP). Hardware CHERI emulator validation (CAP).

#### Acceptance criteria
- [ ] The Decision record evaluates at least two packings of the CAP-008 representation (type tag and generation inline in the word; opaque word with the tag held in the table), each with a CHERI/tagged-memory paragraph.
- [ ] The accepted option states that userspace cannot mint a valid handle and that type tags are checked at the kernel boundary.
- [ ] The Decision lists S-001 and records the representation as prototyped, not frozen.
- [ ] Review records ABI lead and CAP lead sign-off on the pull request.

#### Verification
- Review: ABI lead and CAP lead sign-off recorded on the pull request.

#### Evidence
- none

### ABI-011 · Decide Layer 1 scope: enumerate L1 primitives and place every concept in L1 or L2
- Type: adr
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: ABI-022
- Baseline: §66, §65
- Decision: D-0010
- Risks: R-067
- Invariants: I-040, I-055

Enumerate which primitives are Layer 1 and place every public concept in Layer 1 or Layer 2, answering Q-047 for compositor protocol, Package format and ResourceDomain policy (§66). Options are a minimal Layer 1 (handles, entry, errors, negotiation, object type ids), a Layer 1 that also includes Channel and ResourceDomain as kernel objects, and a Layer 1 that also includes compositor protocol and Package format. No Layer 1 surface is frozen.

<!-- covers: INV-1284, INV-1290 -->

#### Out of scope
Layer 2 evolution rules (IPC). SDK crate surface (SDK). Freeze of any Layer 1 surface (ABI-049).

#### Acceptance criteria
- [ ] The Decision record evaluates minimal Layer 1, Layer 1 including Channel and ResourceDomain, and Layer 1 including compositor protocol and Package format as named options.
- [ ] The accepted option lists every V0 kernel primitive as Layer 1 or Layer 2 and names the S-ID for each.
- [ ] Q-047 is answerable from the accepted option without a second Decision.
- [ ] No listed Layer 1 surface is recorded as frozen.

#### Verification
- Review: ABI lead sign-off recorded on the pull request.

#### Evidence
- none

### ABI-012 · Decide Object-Operation dispatch with async-only submission and move semantics
- Type: adr
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: ABI-008, ABI-010, ABI-009, ABI-014, ABI-015
- Baseline: §18, §65
- Decision: D-0012
- Invariants: I-030, I-056, I-063

Fix how Operations are invoked on typed handles: no blocking entry other than wait-for-completion (§65 rule 4), and Capability and MemoryObject arguments move rather than copy (§65 rule 5). Options are syscall-per-operation dispatch, ring-indexed dispatch through the chosen entry mechanism, and a hybrid with inline completion for already-ready work.

<!-- covers: INV-1279, INV-1272, INV-1273 -->

#### Out of scope
Operation ring byte layout (TSK). Inline-completion signalling details (TSK). MemoryObject map/unmap (MEM).

#### Acceptance criteria
- [ ] The Decision record evaluates syscall-per-operation dispatch, ring-indexed dispatch, and hybrid-with-inline-completion as named options.
- [ ] The accepted option states that no Native ABI entry blocks the calling execution context as its primary mode except explicit wait-for-completion.
- [ ] The accepted option states that Capability and MemoryObject arguments use ownership transfer by default.
- [ ] The Decision lists S-002 and S-004 as the dispatch and error surfaces it depends on, still prototyped.

#### Verification
- Review: ABI lead sign-off recorded on the pull request.

#### Evidence
- none

### ABI-013 · Decide which Object<T> types live in the kernel and the kernel-residency criteria
- Type: adr
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: ABI-022, ABI-011
- Baseline: §7, §33, §65
- Decision: D-0013
- Invariants: I-008, I-055

Decide which `Object<T>` types are implemented in the kernel and which are user-service objects reached through Channels, with written kernel-residency criteria (§65 rule 2, I-008). Options are kernel residency for isolation or privilege only, kernel residency when measured cost requires it, and kernel residency for every typed object. V0 object types are placed against the accepted criterion.

<!-- covers: INV-0172, INV-1270 -->

#### Out of scope
User-space driver hosting (SVC). Channel implementation (IPC). ComputeDevice dispatch (HET).

#### Acceptance criteria
- [ ] The Decision record evaluates isolation-or-privilege, measured-cost, and all-objects-in-kernel as named residency criteria.
- [ ] The accepted option lists every V0 `Object<T>` as kernel-resident or user-service and cites the criterion used.
- [ ] High-level semantics that fail the criterion are recorded as Layer 2 services, not Layer 1 entry points.
- [ ] Review records ABI lead sign-off on the pull request.

#### Verification
- Review: ABI lead sign-off recorded on the pull request.

#### Evidence
- none

### ABI-014 · Decide whether the Operation kind set is a closed kernel enum or extensible registry
- Type: adr
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: ABI-011
- Baseline: §18, §65
- Decision: D-0014

Decide whether the Operation kind set (Read, Write, Receive, Send, Connect, Accept, Timer, Wait, GPUDispatch, DeviceOperation, StorageTransaction) is a closed kernel enum or an extensible registry that user-space services can add to (§18). The extensibility rule is an ABI stability property and precedes the entry layer build. GPUDispatch, DeviceOperation and StorageTransaction are reserved even if implemented later.

<!-- covers: INV-0357 -->

#### Out of scope
Implementation of each kind (TSK). ComputeDevice dispatch (HET). Storage durability (STO).

#### Acceptance criteria
- [ ] The Decision record evaluates a closed kernel enum, an extensible user-service registry, and a closed V0 set with reserved slots as named options.
- [ ] The accepted option names how a new kind is added after V0 and whether that addition is a Layer 1 change.
- [ ] GPUDispatch, DeviceOperation and StorageTransaction occupy reserved slots or are explicitly deferred with a reservation plan.
- [ ] Review records ABI lead and TSK lead sign-off on the pull request.

#### Verification
- Review: ABI lead and TSK lead sign-off recorded on the pull request.

#### Evidence
- none

### ABI-015 · Decide how user space identifies an Operation: Capability, ring index or opaque handle
- Type: adr
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: TSK-014
- Baseline: §19, §65
- Decision: D-0015

Decide how an in-flight Operation is identified from user space: as a `Capability<Operation>`, as an index in the submission ring, or as an opaque handle with a separate cancellation token (§19). V0 cancellation and deadline tests need a stable reference. TSK-014 informs the options; TSK owns completion delivery.

<!-- covers: INV-0373 -->

#### Out of scope
Completion ring layout (TSK). Cancellation of hardware-committed work (TSK, Q-009).

#### Acceptance criteria
- [ ] The Decision record evaluates Capability-to-Operation, ring index, and opaque handle plus cancellation token as named options.
- [ ] The accepted option states how cancel and deadline name the in-flight Operation without a blocking syscall other than wait-for-completion.
- [ ] Review records ABI lead and TSK lead sign-off on the pull request.

#### Verification
- Review: ABI lead and TSK lead sign-off recorded on the pull request.

#### Evidence
- none

### ABI-016 · Decide the Layer 1 version identification and feature-negotiation scheme
- Type: adr
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: ABI-008, ABI-021
- Baseline: §12, §65
- Decision: D-0016
- Invariants: I-041

Decide what the Layer 1 handshake negotiates (§65 rule 6): a version word at first entry, feature bits, or both. The V0 handshake test (ABI-004) implements this scheme at kernel entry, not only as an IDL message test. Surface S-011 becomes prototyped, not frozen.

<!-- covers: INV-1274, INV-1279 -->

#### Out of scope
Layer 2 interface version negotiation (IPC). Implementation of the handshake (ABI-004).

#### Acceptance criteria
- [ ] The Decision record evaluates version word, feature bits, and version word plus feature bits as named options.
- [ ] The accepted option states how an older Component talks to a newer kernel and how a newer Component talks to an older kernel.
- [ ] The Decision lists S-011 and records it as prototyped, not frozen.
- [ ] Review records ABI lead sign-off on the pull request.

#### Verification
- Review: ABI lead sign-off recorded on the pull request.

#### Evidence
- none

### ABI-017 · Write the normative, versioned Native ABI specification v0 defining every entry point
- Type: docs
- Milestone: V0
- Status: todo
- Size: L
- Owner: none
- Depends on: ABI-011, ABI-008, ABI-010, ABI-009, ABI-012, ABI-013, ABI-014, ABI-015, ABI-016, ABI-007
- Baseline: §7, §65, §66
- Risks: R-067
- Invariants: I-040, I-055

Write the normative, versioned Native ABI specification v0 that enumerates and bounds every syscall and object-operation, records every Layer 1 surface as prototyped, and carries the stability-layer declaration from ABI-011 (§65, §66). The specification is the source of truth for the snapshot, the C header and the first conformance cases.

<!-- covers: INV-1280, INV-1269 -->

#### Out of scope
IDL-to-docs generation (DOC). SDK crate guide (SDK). Freeze-candidate marking (ABI-038).

#### Acceptance criteria
- [ ] A versioned specification document exists that defines every V0 Layer 1 entry point, object type and error code.
- [ ] Each Layer 1 surface S-001, S-002, S-004 and S-011 is recorded as prototyped, not frozen.
- [ ] The document states the entry-point bound from ABI-008.
- [ ] Review records ABI lead sign-off on the pull request.

#### Verification
- Review: ABI lead sign-off recorded on the pull request.

#### Evidence
- none

### ABI-018 · Lint native surfaces against POSIX-shaped names and Linux syscall numbers
- Type: build
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: ABI-011, BLD-082
- Baseline: §3, §57, §65
- Invariants: I-013, I-026, I-049

Lint Native ABI headers and the surfaces register so a Layer 1 entry does not exist because POSIX or Linux has an equivalent (§57, §65 rule 9). Names such as `open`, `read`, `write`, `fork` and `ioctl`, Linux syscall numbers, and Wayland objects on native surfaces fail the lint unless an accepted Decision exempts them.

<!-- covers: INV-1130 -->

#### Out of scope
Linking firewall against libc and the Linux personality (ABI-003). CI execution plumbing (BLD). Wayland bridge (LNX).

#### Acceptance criteria
- [ ] A native header that declares `open`, `read`, `write`, `fork` or `ioctl` as a Layer 1 entry point fails CI on `qemu-x86_64`.
- [ ] A native header that embeds a Linux syscall number as a Native ABI entry fails the same lint.
- [ ] An accepted Decision that names an exemption is the only way a matching symbol lands.

#### Verification
- Unit: `tools:tests/abi_posix_lint_*` on CI matrix entry `qemu-x86_64`.
- Integration: pre-merge lint job on native headers and `registers/surfaces.md`.

#### Evidence
- none

### ABI-019 · Prototype syscall-per-Operation, shared submission page and vDSO trampoline entry
- Type: spike
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: none
- Baseline: §65
- Explores: S-002
- Risks: R-007

Prototype the three candidate Native ABI entry mechanisms so ABI-008 is not a paper Decision (§65). Each prototype submits a no-op Operation on H-001 and H-002 and is measured by ABI-001. Nothing is frozen.

<!-- covers: GAP-0500 -->

#### Out of scope
The Decision (ABI-008). io_uring lineage inside TSK. Production entry layer (ABI-002).

#### Acceptance criteria
- [ ] Three prototypes exist: syscall-per-Operation, shared submission page with doorbell, and trampoline entry.
- [ ] Each prototype submits and completes a no-op Operation on H-001 and H-002.
- [ ] The Spike report records what each prototype rules out and recommends which options the adr must evaluate.
- [ ] Surface S-002 remains `open` or `prototyped`, never `frozen`.

#### Verification
- Report: which mechanism preserves async-only entry, how many kernel entry points each needs, what breaks on a future tagged-memory CPU, and what ABI-001 must measure.
- Integration: each prototype boots on `qemu-x86_64` and `hw-h002`.

#### Evidence
- none

### ABI-020 · Prototype typed kernel-boundary errors without errno
- Type: spike
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: none
- Baseline: §7, §12, §65
- Explores: S-004
- Risks: R-007

Prototype typed kernel-boundary errors (`Error::Rights`, exhaustion, disconnect, timeout) so ABI-009 is not a paper Decision (§7, §12). Native errors are not errno values. Surface S-004 remains open.

#### Out of scope
The Decision (ABI-009). Personality errno translation (LNX). Freeze of S-004 (ABI-051).

#### Acceptance criteria
- [ ] A prototype returns `Error::Rights`, exhaustion, disconnect and timeout on H-001 and H-002 without producing an errno.
- [ ] The prototype ships a negative fixture crate that matches on errno, for ABI-003 to reject once that lint lands.
- [ ] Surface S-004 remains `open` or `prototyped`, never `frozen`.

#### Verification
- Report: which encodings preserve typed errors across a Channel, what errno translation would leak into native crates, and which options ABI-009 must evaluate.
- Integration: the prototype boots on `qemu-x86_64` and `hw-h002`.

#### Evidence
- none

### ABI-021 · Prototype Layer 1 version and feature handshake
- Type: spike
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: none
- Baseline: §12, §65
- Explores: S-011
- Risks: R-007

Prototype the Layer 1 handshake that identifies ABI version and features so ABI-016 is informed by running code (§12, §65). An unknown newer field is accepted by an older receiver and an older message by a newer receiver. Surface S-011 remains open.

#### Out of scope
The Decision (ABI-016). IDL message versioning (IPC-019). Freeze of S-011 (ABI-051).

#### Acceptance criteria
- [ ] A prototype handshake identifies ABI version and a feature bit on H-001 and H-002.
- [ ] An older receiver accepts a newer message that carries an unknown field and a newer receiver accepts an older message.
- [ ] Surface S-011 remains `open` or `prototyped`, never `frozen`.

#### Verification
- Report: where the handshake runs relative to the first Operation, what happens on a mismatch, and which options ABI-016 must evaluate.
- Integration: the prototype boots on `qemu-x86_64` and `hw-h002`.

#### Evidence
- none

### ABI-022 · Study Zircon handles, rights, VMOs, Channels, FIDL and Component framework
- Type: spike
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: none
- Baseline: §7, §58
- Explores: S-001

Study Fuchsia/Zircon handles, rights, VMOs, channels, FIDL and the component framework as the closest existing analogue of the native object model (§58). The report feeds ABI-010, ABI-013 and ABI-012. It does not adopt Zircon as the Native ABI.

<!-- covers: INV-1133 -->

#### Out of scope
seL4 capability study (CAP). FIDL versus other IDL choice (IPC). NT object manager study (ABI-031).

#### Acceptance criteria
- [ ] The Spike report describes Zircon handle tables, rights, VMOs, channels, FIDL and components with citations.
- [ ] The report lists ABI assumptions worth taking and ABI assumptions worth rejecting, each mapped to S-001 or to an adr.
- [ ] The report does not recommend freezing any Layer 1 surface.

#### Verification
- Report: what Zircon handle representation implies for S-001, which object types Zircon keeps in-kernel and why, how Zircon errors map onto S-004 options, and which ideas fail §65 rules 7 and 9.

#### Evidence
- none

### ABI-023 · Generate the C header, snapshot, docs and fuzz descriptions from the ABI definition
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: ABI-017, ABI-007, ABI-027
- Baseline: §50, §65

Make the machine-readable Layer 1 definition the single source of truth: generators emit the C-compatible header, the golden snapshot, documentation stubs and fuzz descriptions from it (§50). BLD's syzkaller adaptation and DOC's IDL-to-docs consume this output rather than a second hand-written surface.

<!-- covers: INV-0945 -->

#### Out of scope
syzkaller executor (BLD). IDL-to-docs site (DOC). Safe C wrappers (SDK).

#### Acceptance criteria
- [ ] A checked-in generator produces a C header, a snapshot, doc stubs and fuzz descriptions from the Layer 1 definition.
- [ ] Editing the definition and rerunning the generator changes header, snapshot, stubs and fuzz descriptions in one build.
- [ ] A hand-edited header that does not match the generator output fails CI on `qemu-x86_64`.
- [ ] The generated header contains no POSIX-shaped names unless an accepted Decision exempts them.

#### Verification
- Unit: `tools:tests/abi_codegen_*` on CI matrix entry `qemu-x86_64`.
- Integration: snapshot check and POSIX-name lint run against generator output on every merge to main.

#### Evidence
- none

### ABI-024 · Build the ABI conformance suite with one test per prototyped Layer 1 entry point
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: ABI-017, ABI-002, ABI-005, ABI-004
- Baseline: §6, §65
- Invariants: I-041

Build the ABI conformance suite with one test per prototyped Layer 1 entry point, including the retained handshake case, so v0-to-v0.1 compatibility is proven before the first ABI revision (§6). ABI owns suite content; BLD runs it in CI.

<!-- covers: INV-0147, GAP-0501 -->

#### Out of scope
CI wiring (BLD-015). Cross-version binary runs of SDK v1 (ABI-033).

#### Acceptance criteria
- [ ] The suite contains one test per prototyped Layer 1 entry point named in the v0 specification.
- [ ] Case 0 is the Layer 1 handshake test from ABI-004.
- [ ] The suite passes on `qemu-x86_64` and `hw-h002`.
- [ ] A missing test for a named entry point fails a coverage check in CI.

#### Verification
- Unit: `kernel:tests/abi/conformance/v0_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Integration: BLD post-merge job once BLD-015 exists.

#### Evidence
- none

### ABI-025 · Select Native ABI or Linux ABI execution world per Component or process at entry
- Type: build
- Milestone: V0.5
- Status: todo
- Size: L
- Owner: none
- Depends on: ABI-002, LNX-001, LNX-003
- Baseline: §6, §3
- Invariants: I-025

Implement Phase B dual execution worlds: Native ABI and Linux ABI, selectable per Component or personality process at the entry layer (§6). The world tag lives in the entry layer so a Wayland-bridged Linux GUI app can run beside native apps without native software seeing Linux syscalls.

<!-- covers: INV-0142 -->

#### Out of scope
Linux syscall implementation (LNX). Wayland bridge (LNX). Native syscall-filter proof (ABI-035).

#### Acceptance criteria
- [ ] A native Component is tagged Native ABI at entry and cannot be retagged from userspace.
- [ ] A Linux-personality process is tagged Linux ABI at entry and uses the retained Linux syscall path.
- [ ] `os inspect` shows the world tag on each live Component and personality process.
- [ ] A native Component and a busybox process run concurrently on H-002.

#### Verification
- Unit: `kernel:tests/abi/world_tag_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Integration: native Component plus C-001 busybox side by side on H-002.
- Demo: world tags visible in `os inspect` during the V0.5 Wayland-beside-native scenario.

#### Evidence
- none

### ABI-026 · Exercise Layer 1 evolution: add an Operation, keep the v0 binary running, retain the test
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: ABI-004, ABI-024
- Baseline: §12, §65
- Invariants: I-041

Exercise Layer 1 evolution the way the V0.5 UI protocol bumps v0 to v0.1: add an Operation kind or optional field, keep a v0 native binary running, and retain the test permanently (§12, §65 rule 6). Negotiation is proven against a real change before freeze candidates are named at V1.

<!-- covers: INV-1274, INV-0147 -->

#### Out of scope
UI protocol v0-to-v0.1 bump (UIP, IPC). Freeze-candidate review (ABI-034).

#### Acceptance criteria
- [ ] A v0 native binary runs against a kernel that has added one Operation or optional field and completes its existing Operations.
- [ ] The new Operation is rejected or ignored by the v0 binary according to ABI-016.
- [ ] The evolution test is retained in the conformance suite and passes on `qemu-x86_64` and `hw-h002`.

#### Verification
- Integration: `kernel:tests/abi/evolution/v0_to_v0_1_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Unit: handshake plus added-kind case in the conformance suite.

#### Evidence
- none

### ABI-027 · Add CI golden ABI snapshot diff that fails unless the change links an accepted ADR
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: ABI-017, ABI-006
- Baseline: §65
- Risks: R-007, R-067

Make accidental Layer 1 changes mechanically impossible from the first prototype: CI diffs syscall surface, object types and message layouts against a golden snapshot generated from the machine-readable ABI definition, and fails unless the change names an accepted adr.

<!-- covers: GAP-0099, INV-1268 -->

#### Out of scope
Generators that emit the snapshot from the definition (ABI-023). syzkaller adaptation (BLD).

#### Acceptance criteria
- [ ] Adding an entry point to the ABI definition without an accepted adr id fails CI on `qemu-x86_64`.
- [ ] Changing an existing object-type identifier or message layout without an accepted adr id fails CI.
- [ ] A change that names a done adr whose Decision lists the touched ABI surface updates the golden snapshot in the same pull request.
- [ ] The snapshot covers entry points, object types and message layouts named in the v0 specification.

#### Verification
- Unit: `tools:tests/abi_snapshot_*` on CI matrix entry `qemu-x86_64`.
- Integration: pre-merge job against a fixture that mutates the snapshot without an adr link.

#### Evidence
- none

### ABI-028 · Classify public symbols into stability layers and enforce policy in CI
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: ABI-011, ABI-018, ABI-027
- Baseline: §66, §57
- Invariants: I-005, I-046

Classify every public symbol into Layer 1 through Layer 4 and fail CI when a native surface is unlabelled or POSIX-shaped without an accepted Decision (§66, §57). The V1 task ABI-036 extends the map to SDK crate roots; this task lands the classifier on the Native ABI header when the first immutable packages appear.

<!-- covers: INV-1289 -->

#### Out of scope
SDK crate semver labels (ABI-036). Layer 2 evolution-rule freeze (IPC). Publishing the map (DOC).

#### Acceptance criteria
- [ ] Every public symbol in the Native ABI header is labelled Layer 1, Layer 2, Layer 3 or Layer 4 in a checked-in map.
- [ ] An unlabelled public symbol fails CI on `qemu-x86_64`.
- [ ] A POSIX-shaped name, Linux syscall number or Wayland object on a native surface fails CI unless an accepted Decision names it.

#### Verification
- Unit: `tools:tests/abi_layer_map_*` on CI matrix entry `qemu-x86_64`.
- Integration: pre-merge job on native headers.

#### Evidence
- none

### ABI-029 · Decide whether ABI headers carry a syscall-note-style exception for native programs
- Type: adr
- Milestone: V0.5
- Status: done
- Size: S
- Owner: @agent/claude
- Depends on: GOV-003
- Baseline: §65
- Decision: D-0008
- Verified by: @jakebarnby

Decide whether Native ABI headers and the syscall surface carry a Linux-syscall-note-style exception so native userspace programs are never derivative works of the kernel. Options are a syscall-note-style exception on Layer 1 headers, headers under the SDK license only with no kernel exception, and dual-licensed headers. GOV-003 is the input; SDK-027 is accepted in the same rung.

<!-- covers: GAP-0003 -->

#### Out of scope
Outbound kernel license (GOV). SDK crate license (SDK). Generated header emission (ABI-023).

#### Acceptance criteria
- [x] The Decision record evaluates syscall-note-style exception, SDK-license-only headers, and dual-licensed headers as named options.
- [x] The accepted option states whether a proprietary native application linking only the generated header is a derivative work of the kernel.
- [x] Review records ABI lead and GOV lead sign-off on the pull request.

#### Verification
- Review: ABI lead and GOV lead sign-off recorded on the pull request.

#### Evidence
- decision:D-0008

### ABI-030 · Publish Layer 1 change control: mandatory RFC, compatibility review, per-Milestone policy
- Type: docs
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: ABI-006, ABI-027, GOV-005
- Baseline: §65, §66
- Invariants: I-059

Publish heightened change control for Layer 1: mandatory RFC, compatibility review, and a per-milestone freeze policy (§65 rule 10). This complements the mechanical snapshot check once the ABI has consumers beyond the V0 demo (the four V0.5 apps). Each Layer 1 change ships with a deprecation strategy and compatibility-shim plan.

<!-- covers: GAP-0058, INV-1278, INV-1283 -->

#### Out of scope
RFC venue and templates for external contributors (GOV). Snapshot CI (ABI-027). Per-layer deprecation windows (ABI-039).

#### Acceptance criteria
- [ ] A published process document requires an RFC and an ABI compatibility review for every Layer 1 change.
- [ ] The document states the per-milestone policy: prototyped in V0, freeze candidates at V1, frozen at V4.
- [ ] The document requires a deprecation strategy and compatibility-shim plan on every Layer 1 change.
- [ ] Review records ABI lead and GOV lead sign-off on the pull request.

#### Verification
- Review: ABI lead and GOV lead sign-off recorded on the pull request.

#### Evidence
- none

### ABI-031 · Study NT Object manager, handles, access masks and I/O request packets
- Type: spike
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: ABI-022
- Baseline: §7, §58

Study Windows NT object manager, handles, access masks and I/O request packets for object-model and Windows-personality design (§58). The report informs DeviceOperation shape and the V1 non-gated WIN bring-up. Native software still never sees Win32.

<!-- covers: INV-1137 -->

#### Out of scope
Wine hosting Decision (WIN). DeviceOperation implementation (TSK). Handle encoding Decision (ABI-010).

#### Acceptance criteria
- [ ] The Spike report describes NT object manager, handles, access masks and IRPs with citations.
- [ ] The report lists which NT ideas inform DeviceOperation and which would violate §65 rules 7 and 9 if copied into Layer 1.
- [ ] The report does not recommend exposing Win32 as a native API.

#### Verification
- Report: how NT handles differ from S-001, whether access masks map onto Capability rights or must stay in WIN, and what IRPs imply for DeviceOperation without making IRP a Native ABI type.

#### Evidence
- none

### ABI-032 · Reserve the ComputeDevice Object type and Operation slots in the Layer 1 ABI
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: HET-001, ABI-011, ABI-014, ABI-017
- Baseline: §37, §38, §65
- Invariants: I-024, I-058

Reserve the ComputeDevice object type identifier and GPUDispatch Operation slots in Layer 1 so V2 ComputeDevice dispatch (HET) needs no ABI break (§65 rule 8, INV-0701). Enumeration ABI comes from HET-001. Dispatch implementation stays in HET.

<!-- covers: INV-0701 -->

#### Out of scope
ComputeDevice dispatch and placement (HET). GPU driver stack (GFX). Conformance tests for dispatch (HET).

#### Acceptance criteria
- [ ] The Layer 1 definition contains a ComputeDevice object type id and a GPUDispatch Operation kind as reserved slots.
- [ ] A native Component that invokes GPUDispatch before HET implements it receives the typed unsupported error named by ABI-009.
- [ ] Adding a different type id later for ComputeDevice fails the snapshot check.
- [ ] The v1 specification records the reservation and names HET as the implementation owner.

#### Verification
- Unit: `kernel:tests/abi/computedevice_reserve_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Integration: snapshot check includes the reserved type id and kind.

#### Evidence
- none

### ABI-033 · Extend the conformance suite to every entry point plus cross-version binary runs
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: ABI-024, ABI-026, ABI-038
- Baseline: §6, §65

Extend the conformance suite to every Layer 1 entry point in the v1 specification and add cross-version binary runs: binaries built against v0.x run on v1, and v1 binaries run on v0.x where the specification promises it. This is the Layer 1 half of the V1 SDK compatibility suite.

<!-- covers: GAP-0501, INV-0147 -->

#### Out of scope
SDK crate compatibility suite (SDK). CI image plumbing (BLD). Layer 1 freeze (ABI-049).

#### Acceptance criteria
- [ ] The suite contains one test per Layer 1 entry point named in the v1 specification.
- [ ] A binary built against v0.x runs on a v1 kernel for every Operation the specification marks compatible.
- [ ] A binary built against v1 runs on a v0.x kernel for every Operation the specification marks backward-compatible, and otherwise receives a typed unsupported error.
- [ ] The suite passes on `qemu-x86_64` and `hw-h002`.

#### Verification
- Integration: `kernel:tests/abi/conformance/v1_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Unit: coverage check that every v1 entry point has a test.

#### Evidence
- none

### ABI-034 · Review every Layer 1 entry point and mark freeze candidates in the surfaces Register
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: ABI-042, ABI-040, ABI-038, ABI-036, CAP-038
- Baseline: §65, §66
- Risks: R-028
- Invariants: I-040

Review every Layer 1 entry point and mark freeze candidates in `registers/surfaces.md` at SDK v1. Each candidate cites its spike, adr and benchmark report. Nothing is frozen: I-040 forbids a Layer 1 freeze before V4.

<!-- covers: INV-1278, INV-1268 -->

#### Out of scope
Accepting the freeze (ABI-049). Capability freeze candidates (CAP). Operation freeze candidates (TSK).

#### Acceptance criteria
- [ ] Every Layer 1 surface owned by ABI (S-001, S-002, S-004, S-011) is marked freeze-candidate or explicitly deferred with a reason in the v1 specification.
- [ ] Each freeze candidate cites a Spike report, an adr and a B-ID report.
- [ ] No Layer 1 surface has register state `frozen`.
- [ ] Review records ABI lead sign-off on the pull request.

#### Verification
- Review: ABI lead sign-off recorded on the pull request.
- Integration: `roadmap check` on the surfaces register shows no Layer 1 surface `frozen`.

#### Evidence
- none

### ABI-035 · Enforce that native Components cannot invoke Linux syscalls, with a filter test
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: ABI-025, ABI-003
- Baseline: §3, §6, §57
- Invariants: I-005, I-049

Phase B verification: once the Linux personality is a product at V1, native Components run with the Linux syscall world closed (§6). A filter test retained in CI proves a native Component cannot invoke a Linux syscall. Personalities still use the retained path.

<!-- covers: INV-0143 -->

#### Out of scope
Linux personality seccomp for sandboxed Linux apps (LNX). Linking firewall (ABI-003).

#### Acceptance criteria
- [ ] A native Component that issues a Linux syscall number receives a denial and does not enter the Linux syscall implementation.
- [ ] The denial is the typed error named by ABI-009 and is visible in `os inspect`.
- [ ] A Linux-personality process on the same kernel still issues Linux syscalls for C-001.
- [ ] The filter test is retained in CI on `qemu-x86_64` and `hw-h002`.

#### Verification
- Unit: `kernel:tests/abi/syscall_filter_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Integration: native Component denial plus C-001 still passing on H-002.
- Fuzz: `kernel:fuzz/abi_syscall_filter` one hour nightly without panic.

#### Evidence
- none

### ABI-036 · Classify every public symbol into a stability layer
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: ABI-011, ABI-017, ABI-018
- Baseline: §66, §57

Classify every public symbol into Layer 1, Layer 2, Layer 3 or Layer 4 and enforce the classification in CI (§66). The lint rejects POSIX-shaped names, Linux syscall numbers and Wayland objects on native surfaces unless an accepted Decision exempts them.

<!-- covers: INV-1289, INV-1130 -->

#### Out of scope
SDK crate semver (SDK). Layer 2 evolution rules freeze (IPC). Publishing the classification (DOC).

#### Acceptance criteria
- [ ] Every public symbol in the Native ABI header and the SDK crate root is labelled Layer 1, Layer 2, Layer 3 or Layer 4 in a checked-in map.
- [ ] An unlabelled public symbol fails CI on `qemu-x86_64`.
- [ ] A POSIX-shaped name, Linux syscall number or Wayland object on a native surface fails CI unless an accepted Decision names it.
- [ ] The map matches ABI-011 for Layer 1 versus Layer 2.

#### Verification
- Unit: `tools:tests/abi_layer_map_*` on CI matrix entry `qemu-x86_64`.
- Integration: pre-merge job on native headers and SDK crate roots.

#### Evidence
- none

### ABI-037 · Decide whether Layer 2 Interface stability applies at V1 or only at 1.0
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: ABI-011, IPC-002
- Baseline: §66, §12
- Decision: D-0011
- Risks: R-005, R-028

Decide whether Layer 2 core platform interfaces are stability-constrained at V1 or only at 1.0, so SDK v1 developers know which interfaces may still break (§66). Options are Wayland-style stability from V1, no Layer 2 stability until 1.0, and evolution rules frozen at V1 with interface versions unlocked until V4. IPC freezes evolution rules; this adr sets the timing developers are told.

<!-- covers: GAP-0545 -->

#### Out of scope
Freezing Layer 2 evolution rules (IPC-042). Per-layer deprecation windows (ABI-039). Layer 1 freeze (ABI-049).

#### Acceptance criteria
- [ ] The Decision record evaluates stability from V1, stability only at 1.0, and evolution-rules-at-V1 with versions-locked-at-V4 as named options.
- [ ] The accepted option states what an SDK v1 application may assume about Layer 2 breakage before 1.0.
- [ ] Review records ABI lead, IPC lead and SDK lead sign-off on the pull request.

#### Verification
- Review: ABI lead, IPC lead and SDK lead sign-off recorded on the pull request.

#### Evidence
- none

### ABI-038 · Revise the ABI specification to v1 with freeze-candidate marking and full semantics
- Type: docs
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: ABI-017, ABI-026, ABI-032, ABI-036
- Baseline: §65, §66

Revise the Native ABI specification to v1 with freeze-candidate marking and documented semantics for every entry point, including reserved ComputeDevice slots. SDK v1 and DOC's IDL-to-docs generation consume this document. The entry-point bound from ABI-008 is re-verified. No Layer 1 surface is frozen.

<!-- covers: INV-1280, INV-1269 -->

#### Out of scope
Generated reference pages (DOC). Freeze ADR (ABI-049). SDK crate guide (SDK).

#### Acceptance criteria
- [ ] The v1 specification defines semantics for every Layer 1 entry point including reserved ComputeDevice slots.
- [ ] Each Layer 1 surface is marked freeze-candidate or deferred, never frozen.
- [ ] The entry-point count is less than or equal to the bound recorded by ABI-008.
- [ ] Review records ABI lead sign-off on the pull request.

#### Verification
- Review: ABI lead sign-off recorded on the pull request.

#### Evidence
- none

### ABI-039 · Publish the deprecation policy per stability layer with notice periods and overlap
- Type: docs
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: ABI-037, ABI-030, GOV-006
- Baseline: §66

Publish the deprecation policy per stability layer with notice periods and minimum supported overlap for Layer 2 interfaces. This is the Layer 1 and Layer 2 half of the V1 SDK stability policy and refines GOV's per-layer stability statement with concrete windows. Numbers live in the policy document as overlap in minor releases, not as calendar dates.

<!-- covers: GAP-0059 -->

#### Out of scope
SDK Layer 3 semver (SDK). Detection tooling (ABI-043). V2 retirement process (ABI-045).

#### Acceptance criteria
- [ ] A published policy names deprecation and overlap rules for Layer 1, Layer 2, Layer 3 and Layer 4.
- [ ] Layer 2 overlap is expressed as a minimum number of minor interface versions, not a calendar date.
- [ ] Layer 1 changes are described as requiring a new major OS version after freeze.
- [ ] Review records ABI lead and GOV lead sign-off on the pull request.

#### Verification
- Review: ABI lead and GOV lead sign-off recorded on the pull request.

#### Evidence
- none

### ABI-040 · Define the Layer 1 freeze Gate: compat suite, fuzz Corpus, semantics, add-vs-change policy
- Type: docs
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: ABI-030, ABI-024, ABI-011
- Baseline: §65, §66
- Invariants: I-040

Define the Layer 1 freeze gate so V4 freeze and the V1 stable-SDK claim have verifiable meaning: golden binary compatibility suite, fuzz corpus, documented semantics for every entry point, and a policy for adding versus changing entry points. Freeze candidates are named at V1; freeze itself stays at V4.

<!-- covers: GAP-0501, INV-1278 -->

#### Out of scope
Building the V4 compatibility suite (ABI-047). Accepting the freeze (ABI-049). syzkaller infra (BLD).

#### Acceptance criteria
- [ ] A published gate definition names the compatibility suite, fuzz corpus, semantics coverage and add-versus-change policy required to freeze Layer 1.
- [ ] The definition states that no Layer 1 surface is frozen before V4.
- [ ] The definition is cited by ABI-034 and ABI-049.
- [ ] Review records ABI lead sign-off on the pull request.

#### Verification
- Review: ABI lead sign-off recorded on the pull request.

#### Evidence
- none

### ABI-041 · Map every Linux Personality and Windows Personality Object onto its native Object<T> terminus
- Type: docs
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: ABI-013, ABI-005, LNX-003
- Baseline: §3, §4
- Invariants: I-027

Track the §4 invariant that all three application paths terminate in native kernel objects as Phase C translation begins. Map every Linux and Windows personality object onto its native `Object<T>` terminus. Native software still never sees POSIX or Win32. Reviewed with LNX and WIN at each later milestone.

<!-- covers: INV-0107 -->

#### Out of scope
Syscall translation implementation (LNX). Wine object mapping Decision (WIN). NT study (ABI-031).

#### Acceptance criteria
- [ ] A published map names each Linux personality object used at V1 and the native `Object<T>` it terminates in.
- [ ] The map names each Windows personality object in scope for V1 bring-up, or records that WIN has not yet introduced it.
- [ ] No row lists a POSIX or Win32 type as a Native ABI type.
- [ ] Review records ABI lead, LNX lead and WIN lead sign-off on the pull request.

#### Verification
- Review: ABI lead, LNX lead and WIN lead sign-off recorded on the pull request.

#### Evidence
- none

### ABI-042 · Catalog ABI assumptions that break on CHERI, tagged memory and future architectures
- Type: spike
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: ABI-010, CAP-012
- Baseline: §8, §38, §65
- Explores: S-001
- Invariants: I-058, I-100

Catalog ABI assumptions that would break on CHERI, tagged memory and future memory-safe CPUs (§8, §38) so each freeze candidate at V1 carries an escape-hatch analysis. Complements CAP-038. The Native ABI stays architecture-neutral in its definitions.

<!-- covers: INV-0195 -->

#### Out of scope
CHERI emulator validation of Capability encoding (CAP). Hardware bring-up of Morello (CAP). Fossilization review at 1.0 (ABI-054).

#### Acceptance criteria
- [ ] The Spike report lists every Layer 1 assumption that fails on CHERI-class pointers, tagged memory or non-x86-64 page tables.
- [ ] Each listed assumption names the freeze candidate it threatens and a reserved escape hatch, or records that the candidate must be redesigned before freeze.
- [ ] The report does not freeze any Layer 1 surface.

#### Verification
- Report: which S-001 encodings survive CHERI sealing, which entry-mechanism choices bake x86-64 sysret/syscall, and which MemoryObject or ComputeDevice assumptions assume coherent DRAM forever.

#### Evidence
- none

### ABI-043 · Build tooling that detects use of deprecated ABI entry points and interfaces in Packages
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: ABI-045, ABI-039, ABI-023
- Baseline: §66

Build detection of deprecated ABI entry points and Layer 2 interfaces in Packages so the deprecation process is enforceable at SDK build and PKG install from V2, when the store client and third-party packages arrive.

<!-- covers: GAP-0347 -->

#### Out of scope
Package store and install (PKG). SDK deprecation lints for Layer 3 (SDK). Removal of Layer 1 entry points (ABI-048).

#### Acceptance criteria
- [ ] Building a Package that calls a Layer 1 entry marked deprecated in the ABI definition emits a diagnostic naming the entry and the overlap window.
- [ ] Installing such a Package records the same diagnostic in the install log.
- [ ] A Package that does not use deprecated entries produces no diagnostic.
- [ ] The detector reads deprecation marks from the generated ABI definition, not from a second list.

#### Verification
- Unit: `sdk:tests/abi/deprecated_use_*` on CI matrix entry `qemu-x86_64`.
- Integration: SDK build path and PKG install path each run the detector on a fixture Package.

#### Evidence
- none

### ABI-044 · Run the conformance suite against wrapper and native implementations during Phase C
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: ABI-033
- Baseline: §6, §65
- Invariants: I-009

Phase C replaces Linux wrappers with native kernel code under a stable Native ABI (§6). Run the conformance suite against both the wrapper implementation and the native implementation of every migrated primitive so the abstraction stays ABI-stable while the implementation evolves.

<!-- covers: INV-0147 -->

#### Out of scope
Choosing when Phase C or D begins (KRN). Native Component implementation (CMP).

#### Acceptance criteria
- [ ] For every migrated primitive the conformance suite passes on the wrapper implementation and on the native implementation.
- [ ] A primitive whose two implementations disagree fails CI with the failing case named.
- [ ] The suite still passes on `qemu-x86_64` and `hw-h002`.

#### Verification
- Integration: `kernel:tests/abi/conformance/dual_impl_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Review: KRN lead confirms the migrated primitive list on the pull request.

#### Evidence
- none

### ABI-045 · Decide the Layer 1 and platform deprecation process: announcement, overlap, detection
- Type: adr
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: ABI-039, ABI-030
- Baseline: §65, §66
- Decision: D-0004

Decide how Layer 1 and platform interfaces are retired: announcement, minimum overlap window, and tooling to detect use of deprecated interfaces. Store client and third-party packages arrive at V2, so an agreed process must exist before V4 removes deprecated entry points. Options are announce-plus-overlap-plus-detection, never remove from Layer 1 (shim forever), and remove only with a major OS version even before freeze.

<!-- covers: GAP-0347 -->

#### Out of scope
Detector implementation (ABI-043). Actual removal (ABI-048). Layer 3 semver (SDK).

#### Acceptance criteria
- [ ] The Decision record evaluates announce-plus-overlap-plus-detection, shim-forever, and major-version-only removal as named options.
- [ ] The accepted option states how a deprecated Layer 1 entry is announced, how long it overlaps, and how use is detected.
- [ ] Review records ABI lead and GOV lead sign-off on the pull request.

#### Verification
- Review: ABI lead and GOV lead sign-off recorded on the pull request.

#### Evidence
- none

### ABI-046 · Complete the Layer 1 reference: documented semantics for every entry point
- Type: docs
- Milestone: V3
- Status: todo
- Size: L
- Owner: none
- Depends on: ABI-038, ABI-033, ABI-034
- Baseline: §65, §66

Author normative semantics for every Layer 1 entry point so the V3 documentation gate can show a complete Layer 1 reference. ABI authors the prose; DOC generates and publishes pages (DOC-023).

<!-- covers: GAP-0501, INV-1280 -->

#### Out of scope
Page generation and the docs site (DOC). SDK guides (SDK). Freeze (ABI-049).

#### Acceptance criteria
- [ ] Every Layer 1 entry point in the v1 specification has ABI-authored semantics prose in the machine-readable definition.
- [ ] A generator coverage check fails CI when an entry point lacks semantics prose.
- [ ] Review records ABI lead sign-off on the pull request.

#### Verification
- Review: ABI lead sign-off recorded on the pull request.
- Integration: coverage check in CI against the Layer 1 definition.

#### Evidence
- none

### ABI-047 · Build the golden binary compatibility suite proving RC1 binaries run on every beta build
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: ABI-033, ABI-040, ABI-048
- Baseline: §65, §66

Build the golden binary compatibility suite that proves binaries built against the freeze candidate run on every subsequent beta build. This is the V4 compatibility demo and the suite named by ABI-040.

<!-- covers: GAP-0501 -->

#### Out of scope
SDK crate compatibility (SDK). Wine tests (WIN). Freeze Decision (ABI-049).

#### Acceptance criteria
- [ ] A binary built against the freeze-candidate header and definition runs on every subsequent V4 beta image in CI on `qemu-x86_64` and `hw-h002`.
- [ ] The suite report is produced as an artifact of each beta image build.
- [ ] A Layer 1 change that breaks a freeze-candidate binary fails the suite.

#### Verification
- Integration: golden-binary job on CI matrix entries `qemu-x86_64` and `hw-h002` for each V4 beta image.
- Demo: a freeze-candidate native application runs unmodified on the current beta, with the suite report displayed on H-002.

#### Evidence
- none

### ABI-048 · Remove deprecated Layer 1 entry points before the freeze candidate
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: ABI-045, ABI-043, ABI-034
- Baseline: §65, §66
- Risks: R-054

Remove deprecated Layer 1 entry points before the freeze candidate so the frozen surface does not carry retired operations. Follows ABI-045 and detector results. A Layer 1 change after freeze is a new major version. Required by V4-G01 (Layer 1 ABI frozen with a conformance suite).

#### Out of scope
Layer 2 field deprecation (IPC). Freeze acceptance (ABI-049).

#### Acceptance criteria
- [ ] Every Layer 1 entry marked deprecated by the detector and past its overlap window is absent from the freeze-candidate definition.
- [ ] The snapshot check accepts the removal only when the linked adr is ABI-045 or a follow-up adr it names.
- [ ] A native binary that still calls a removed entry receives the typed unsupported error and does not panic the kernel.
- [ ] The v4 specification lists removed entries and their replacements.

#### Verification
- Integration: `kernel:tests/abi/removed_entry_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Unit: snapshot check on the freeze-candidate definition.

#### Evidence
- none

### ABI-049 · Decide the Layer 1 freeze: accept the freeze ADR over the reviewed candidate set
- Type: adr
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: ABI-034, ABI-048, ABI-040, ABI-042, ABI-033
- Baseline: §65, §66
- Decision: D-0009
- Risks: R-054, R-007
- Invariants: I-040

Accept or reject the Layer 1 freeze over the reviewed candidate set. Options are freeze the full candidate set, freeze a reduced core and defer the rest, or defer the freeze to 1.0. I-040 forbids freezing before this rung. After acceptance, a Layer 1 change is a new major OS version.

<!-- covers: INV-1278, INV-1268 -->

#### Out of scope
1.x stability declaration (ABI-053). Compatibility suite construction (ABI-047). Capability surface freeze (CAP).

#### Acceptance criteria
- [ ] The Decision record evaluates freeze-full-candidate-set, freeze-reduced-core, and defer-to-1.0 as named options.
- [ ] The accepted option lists every Layer 1 surface as frozen, deferred, or superseded, citing spike, adr and benchmark report for each frozen surface.
- [ ] If the freeze is accepted, S-001, S-002, S-004 and S-011 are the ABI-owned surfaces named as frozen or explicitly deferred.
- [ ] Review records ABI lead sign-off on the pull request.

#### Verification
- Review: ABI lead sign-off recorded on the pull request.

#### Evidence
- none

### ABI-050 · Draft the ABI stability statement for RFC review
- Type: docs
- Milestone: V4
- Status: todo
- Size: S
- Owner: none
- Depends on: ABI-049, ABI-046
- Baseline: §65, §66

Draft the ABI stability statement for RFC review as part of the V4 support-policy bundle. ABI drafts; GOV publishes the contract (GOV-075). The statement describes Layer 1 freeze, Layer 2 version lock and the rule that Layer 1 changes after freeze require a new major version.

#### Out of scope
Published support window and CVE SLA (GOV, REL). 1.x amendment (ABI-053).

#### Acceptance criteria
- [ ] A draft stability statement exists that names the frozen Layer 1 surfaces, the locked Layer 2 versions and the major-version rule for Layer 1 changes.
- [ ] The draft is submitted through the RFC process recorded by GOV.
- [ ] Review records ABI lead and GOV lead sign-off on the pull request.

#### Verification
- Review: ABI lead and GOV lead sign-off recorded on the pull request.

#### Evidence
- none

### ABI-051 · Freeze Layer 1 entry, error and version-negotiation surfaces
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: ABI-019, ABI-020, ABI-021, ABI-008, ABI-009, ABI-016, ABI-047
- Baseline: §12, §65, §66
- Freezes: S-002, S-004, S-011, S-001
- Invariants: I-040

V4 freezes Layer 1 surfaces S-002, S-004 and S-011 after their spikes and accepted Decisions (§65, §66). Other Layer 1 surfaces freeze in their owning conformance suites. This task is the freeze record and the wiring of those three surfaces into the V4 compatibility suite.

#### Out of scope
Capability rights freeze (CAP-051). MemoryObject freeze (MEM-054). Component creation freeze (CMP-052). Tracing event freeze (OBS-054). 1.x stability declaration (ABI-053).

#### Acceptance criteria
- [ ] Surfaces S-002, S-004 and S-011 are listed as frozen by this task in the surfaces register.
- [ ] The V4 compatibility suite includes a case for each of the three surfaces on `qemu-x86_64` and `hw-h002`.
- [ ] A change to a frozen surface without an accepted superseding Decision fails CI.

#### Verification
- Integration: `abi:tests/l1/entry_error_negotiate_freeze_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Review: ABI lead sign-off recorded on the pull request that lands the freeze.

#### Evidence
- none

### ABI-052 · Run reference, conformance and compatibility suites on the 1.0 candidate and publish
- Type: build
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: ABI-047, ABI-053, ABI-046
- Baseline: §65, §66

Run the ABI reference coverage check, conformance suite and compatibility suite on the 1.0 candidate and publish the reports. This is the ABI half of the 1.0 compatibility-proof demo: a V4-built native application runs unmodified on 1.0.

#### Out of scope
SDK soak (SDK). Docs snapshot (DOC). Channel launch (REL).

#### Acceptance criteria
- [ ] The conformance suite and the golden compatibility suite pass on the 1.0 candidate on `qemu-x86_64` and `hw-h002`.
- [ ] A native application built against the V4 freeze candidate runs unmodified on the 1.0 candidate.
- [ ] Published reports for reference coverage, conformance and compatibility are attached as Evidence.

#### Verification
- Integration: conformance and compatibility suites on CI matrix entries `qemu-x86_64` and `hw-h002` for the 1.0 candidate.
- Demo: V4-built native application running unmodified on 1.0 with the conformance report displayed on H-002.

#### Evidence
- none

### ABI-053 · Decide the 1.x stability declaration superseding the freeze ADR with stable for 1.x
- Type: adr
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: ABI-049, ABI-050, ABI-054
- Baseline: §65, §66
- Decision: D-0002
- Invariants: I-059

A Decision is immutable, so the freeze ADR is not edited in place. This superseding adr declares Layer 1 stable for the 1.x line and states that Layer 1 changes require a new major OS version. Options are declare stable for 1.x as frozen, declare stable with a listed exception set, and decline to declare stable (remain in freeze-candidate state).

#### Out of scope
2.0 planning RFC (GOV). Layer 3 semver statement (SDK). Support window (GOV, REL).

#### Acceptance criteria
- [ ] The Decision record evaluates stable-for-1.x-as-frozen, stable-with-listed-exceptions, and decline-to-declare as named options, and names the freeze ADR it supersedes.
- [ ] The accepted option states that a Layer 1 change after 1.0 requires a new major OS version, or records the listed exceptions.
- [ ] The public policy text matches the accepted option.
- [ ] Review records ABI lead and GOV lead sign-off on the pull request.

#### Verification
- Review: ABI lead and GOV lead sign-off recorded on the pull request.

#### Evidence
- none

### ABI-054 · Review ABI, MemoryObject, ComputeDevice and Capability shapes against future hardware
- Type: docs
- Milestone: 1.0
- Status: todo
- Size: M
- Owner: none
- Depends on: ABI-049, ABI-042, ABI-032, GOV-025
- Baseline: §8, §38, §65, §70
- Invariants: I-058, I-100

Run the 1.0 fossilization review of the Native ABI, MemoryObject, ComputeDevice and Capability representation against future-hardware scenarios (CXL, CHERI, NPU, disaggregated memory) so later hardware does not require a major-version break that could have been reserved (§38, §70). Findings feed the 2.0 RFC without depending on LATER tasks.

<!-- covers: INV-1338, INV-1305, INV-1276 -->

#### Out of scope
2.0 planning RFC (GOV-082). CHERI hardware enforcement (CAP). ComputeDevice dispatch (HET). MemoryObject CXL implementation (MEM).

#### Acceptance criteria
- [ ] A published review walks ABI, MemoryObject, ComputeDevice and Capability shapes against CXL, CHERI, NPU and disaggregated-memory scenarios.
- [ ] Each scenario records whether the frozen Layer 1 surface can accommodate it without a major-version break, or names the 2.0 RFC item.
- [ ] The review cites ABI-042 and does not introduce a calendar date.
- [ ] Review records ABI lead, CAP lead, MEM lead and HET lead sign-off on the pull request.

#### Verification
- Review: ABI lead, CAP lead, MEM lead and HET lead sign-off recorded on the pull request.

#### Evidence
- none
