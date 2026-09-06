# CAP · Capabilities
- Prefix: CAP
- Lead: none
- Baseline: §7, §8, §9, §51

<!-- roadmap:generated:begin summary -->
Tasks: 53 live, 0 done, 0 in-progress, 53 todo, 0 dropped. Ready: 6. Blocked: 47. Weighted: 0%.
<!-- roadmap:generated:end -->

## Scope

CAP owns the native Capability model: the kernel Capability object, the per-Component table, rights and transfer-rights encoding, attenuating derivation, revocation of derived Capabilities, transfer into a table and over Channels, inspectability, grant and denial audit, persistence and grant continuity, service discovery of Capabilities, and ABI room for hardware-assisted enforcement. Native software holds only `Capability<T, Rights>`; it never holds an Object pointer and never receives ambient authority (§7, §8, §9.1, §51).

## Out of scope

Handle-word encoding in the syscall ABI and Layer 1 freeze (ABI). Component creation and graph instantiation (CMP). Channel message slots that carry handles (IPC). TaskGroup lifetime and in-flight Operation cancel (TSK). Permission policy, grant taxonomy, identity and the threat-model document (SEC). Consent, grant and chooser UI (APP). UserSelected minting (STO). Package manifest schema and signing (PKG). `os inspect` and `os trace` CLI (OBS, SDK). Semantic registry and AI broker (SEM). Fuzz infrastructure (BLD). Documentation site generation (DOC). Personality ptrace (LNX). MemoryObject mapping (MEM).

## Tasks

### CAP-001 · Record Capability grants, derivations, revocations and denials in the audit log
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: CAP-005, CAP-003, CAP-004, CAP-007
- Baseline: §7, §9.1, §51
- Threats: T-001

Every grant, successful and failed derive, transfer, revocation and rights denial emits a typed audit record with holder, object identity, rights word and outcome. The V0 isolation demo requires a typed denial to be visible in that log (§7). Records are live events; durable tamper-evident storage is later OBS work.

#### Out of scope
Durable tamper-evident log store (OBS-044). CLI rendering (OBS, SDK). Permissions log viewer (APP-012).

#### Acceptance criteria
- [ ] A denied object access with no matching table entry appends one denial record naming the Component and the requested type, visible to `os inspect` of that Component's audit stream on `qemu-x86_64`.
- [ ] Mint, derive, transfer and revoke each append a record with holder, object identity and rights; a test creates the four events and reads them back in order.
- [ ] Audit emission is compiled into the same change as the primitive it records; a primitive without an audit probe fails the OBS inspect review gate.

#### Verification
- Unit: `kernel:tests/cap/audit_events_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Integration: V0 isolation demo on H-001: Component B opens a File it was not granted and the denial record is present before the demo process exits.
- Review: OBS lead confirms the event fields match S-010 on the pull request.

#### Evidence
- none

### CAP-002 · Expose Capability type, rights and Object identity to holders and os inspect
- Type: build
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: CAP-005, CAP-003, OBS-006
- Baseline: §7, §24, §64
- Invariants: I-034

A holder can query type, rights and object identity of Capabilities in its own table. The kernel inspect provider for kind `capability` supplies state, ownership and derivation relationships so `os inspect capability` can print them (§7). Holders cannot inspect another Component's table through this surface.

<!-- covers: INV-0182 -->

#### Out of scope
`os inspect` CLI rendering (SDK). Cross-Component trace access (OBS-012). Revocation-walk implementation (CAP-004).

#### Acceptance criteria
- [ ] A holder query of a live Capability returns type tag, rights word and object identity that match the table entry; a query of an index not in the table returns `Error::Rights`.
- [ ] `os inspect capability` on a live handle prints holder Component, object identity, rights and parent handle when derived, on `qemu-x86_64`.
- [ ] A Component cannot inspect another Component's table entries; the call returns `Error::Rights` and allocates no handle.

#### Verification
- Unit: `kernel:tests/cap/inspect_*` on `qemu-x86_64` and `hw-h002`.
- Integration: V0 exit inspect suite `os inspect capability` on H-001 against the demo pair.
- Review: OBS lead sign-off that the provider registers under OBS-006.

#### Evidence
- none

### CAP-003 · Implement Capability mint and attenuating derive with rights monotonicity tests
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: CAP-005, CAP-010, CAP-011, ABI-009
- Baseline: §7, §9.1, §59
- Threats: T-004
- Invariants: I-028

Kernel mint inserts a typed Capability into the caller's table. `derive(cap, mask)` returns a new Capability whose rights are a strict subset of the parent's; a mask that is not a subset returns `Error::Rights` and allocates no handle. `Capability<File, ReadWrite>` derives `Capability<File, Read>`; deriving Admin without Admin authority fails. The encoding is S-003 as decided by CAP-010.

<!-- covers: INV-1158, INV-0181, INV-0184 -->

#### Out of scope
Revocation walk (CAP-004). Transfer over Channels (CAP-006). Handle-table layout (CAP-005).

#### Acceptance criteria
- [ ] `Capability<File, ReadWrite>` derives `Capability<File, Read>` and the child handle is a distinct table entry whose parent is the original.
- [ ] Deriving Admin from `Capability<File, ReadWrite>` returns `Error::Rights` and the table entry count is unchanged.
- [ ] Deriving with a mask that is not a subset of the holder's rights returns `Error::Rights` and allocates no handle.
- [ ] The rights-monotonicity regression is retained permanently in CI on `qemu-x86_64` and `hw-h002`.

#### Verification
- Unit: `kernel:tests/cap/derive_*` on `qemu-x86_64` and `hw-h002`.
- Fuzz: `kernel:fuzz/cap_derive` one hour nightly without panic.
- Review: ABI lead sign-off that derive uses the S-003 encoding from CAP-010.

#### Evidence
- none

### CAP-004 · Implement Capability revocation invalidating all derived Capabilities
- Type: build
- Milestone: V0
- Status: todo
- Size: L
- Owner: none
- Depends on: CAP-009, CAP-003, CAP-005, TSK-010
- Baseline: §7, §59
- Risks: R-003
- Threats: T-005

Revoking a Capability invalidates that handle and every Capability derived from it, using the strategy chosen by CAP-009. The V0 gate is that every derived Capability fails within one Operation at derivation depth 8. In-flight Operations on revoked handles complete with a typed error and never deliver a successful result (§7).

<!-- covers: INV-0180, INV-0187, INV-1158 -->

#### Out of scope
Choosing eager versus lazy strategy (CAP-009). Cancelling unrelated Operations (TSK-010). Cross-object-type immediacy matrix (CAP-044).

#### Acceptance criteria
- [ ] Revoking a root Capability at derivation depth 8 makes every descendant fail its next Operation with `Error::Rights` and no object access, on `qemu-x86_64` and `hw-h002`.
- [ ] An in-flight Operation whose handle is revoked completes with a typed error and never delivers a successful result.
- [ ] After revoke, `os inspect capability` on a descendant reports revoked state; a subsequent derive from it returns `Error::Rights` and allocates no handle.
- [ ] Revocation of a Capability the caller does not hold returns `Error::Rights` and does not alter any other table.

#### Verification
- Unit: `kernel:tests/cap/revoke_*` on `qemu-x86_64` and `hw-h002`.
- Integration: depth-8 revocation suite on H-001 and H-002.
- Fuzz: `kernel:fuzz/cap_revoke` one hour nightly without panic.
- Review: TSK lead sign-off that in-flight Operation completion matches TSK-010.

#### Evidence
- none

### CAP-005 · Implement the kernel Capability Object and per-Component Capability table
- Type: build
- Milestone: V0
- Status: todo
- Size: L
- Owner: none
- Depends on: CAP-008, CAP-010, ABI-002, ABI-005, ABI-009
- Baseline: §7, §10, §51, §69
- Risks: R-003
- Threats: T-003
- Invariants: I-015, I-028, I-056

Every native object access resolves a typed handle through the holder's per-Component table. The table is kernel-owned; userspace cannot mint a valid Capability. Type mismatch fails at the kernel boundary with a typed error. The layout is the one accepted by CAP-008 and stays prototyped, not frozen, through V0 (§7, §10, §51).

<!-- covers: INV-0050, INV-0113, INV-0173, INV-0174, INV-0178, INV-0223, INV-0951, INV-1315, INV-1325 -->

#### Out of scope
Syscall handle-word packing (ABI-010). Component create/destroy (CMP-005). Derive and revoke (CAP-003, CAP-004).

#### Acceptance criteria
- [ ] A Component with an empty table receives `Error::Rights` on any object Operation and the kernel allocates no handle.
- [ ] Looking up a `Capability<File>` through a slot that holds `Capability<Channel>` fails at the kernel boundary with a typed error and does not enter the Channel path.
- [ ] Two Components holding Capabilities to the same Object have distinct table entries; closing one does not remove the other.
- [ ] Table insert and remove reclaim the slot; a leak test of 100,000 insert/remove cycles on `qemu-x86_64` shows no kernel-memory growth beyond the suite's bound.
- [ ] No `unsafe` outside `cap/table.rs` and `cap/rights.rs`.

#### Verification
- Unit: `kernel:tests/cap/table_*` on `qemu-x86_64` and `hw-h002`.
- Integration: Component create-with-empty-table denial on H-001.
- Fuzz: `kernel:fuzz/cap_table` one hour nightly without panic.
- Review: ABI lead sign-off that the table matches CAP-008.

#### Evidence
- none

### CAP-006 · Implement Capability transfer over Channels and at Component creation
- Type: build
- Milestone: V0
- Status: todo
- Size: L
- Owner: none
- Depends on: CAP-005, CAP-003, CAP-011, CMP-005
- Baseline: §7, §10, §53, §59
- Invariants: I-056

A Capability moves from one table to another when sent over a Channel or when attached as part of the explicit set at Component creation. After a move the sender no longer holds the handle; a transfer without transfer rights returns `Error::Rights` and leaves both tables unchanged. Native isolation path step 3 attaches that set at creation (§7, §53, §59).

<!-- covers: INV-0179, INV-0998, INV-1158 -->

#### Out of scope
Wire-format handle slots (IPC-014). Address-space construction (CMP). MemoryObject payload copies (MEM).

#### Acceptance criteria
- [ ] Sending a Capability over a Channel removes it from the sender table and inserts it in the receiver table with the same type and rights, on `qemu-x86_64`.
- [ ] Sending without transfer rights returns `Error::Rights`, allocates no receiver handle, and leaves the sender entry intact.
- [ ] Component creation with an explicit set of N Capabilities yields a table of N entries and no others; a sixth undeclared File open returns `Error::Rights`.
- [ ] The V0 demo transfers a MemoryObject Capability from B to A; A maps the object and B's table no longer holds that handle.

#### Verification
- Unit: `kernel:tests/cap/transfer_*` on `qemu-x86_64` and `hw-h002`.
- Integration: V0 §59 demo on H-001 and H-002.
- Demo: Component A to Channel to B to MemoryObject Capability return, shown with `os trace` on H-002.
- Review: IPC lead sign-off that table move matches IPC-014 slot semantics.

#### Evidence
- none

### CAP-007 · Decide explicit grant sources replacing ambient permissions
- Type: adr
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: SEC-002
- Baseline: §9, §9.1
- Decision: D-0054
- Risks: R-008
- Threats: T-001
- Invariants: I-021, I-060

A Component starts with exactly the Capabilities it was handed. This decision names the sources of that set so V0 Components receive no ambient filesystem, network, device or process-enumeration authority (§9.1). SEC's later grant-taxonomy ADR refines duration and UI class on top of these sources.

<!-- covers: INV-0076, INV-0042, INV-0200 -->

#### Out of scope
Chooser, prompt and settings-only taxonomy (SEC-007). Launch-time binding (CAP-025). Consent UI (APP).

#### Acceptance criteria
- [ ] Options evaluated include at least: (A) creator-at-launch only; (B) creator at launch plus user choice plus Package manifest request; (C) manifest-declared wildcard sets.
- [ ] The accepted option names every source a V0 Component may receive a Capability from, and names that no other source exists.
- [ ] Each option cites T-001 and I-021; the rejected options record why they reintroduce ambient authority.
- [ ] Review records ABI and SEC lead sign-off on the pull request.

#### Verification
- Review: ABI lead and SEC lead sign-off recorded on the pull request.
- Manual: decision file lists at least two options with consequences and names T-001.

#### Evidence
- none

### CAP-008 · Decide the userspace Capability<T> handle representation and table design
- Type: adr
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: CAP-013, CAP-012, CAP-015, SEC-002
- Baseline: §7, §8, §38, §65
- Decision: D-0055
- Risks: R-003, R-012
- Threats: T-003
- Invariants: I-015, I-028, I-058

How a `Capability<T>` is represented to userspace and laid out in the per-Component table, replacing file descriptors and reserving room for hardware enforcement. The accepted option stays prototyped through V0; nothing L1 freezes here (§8, §65). ABI-010 consumes this decision for the syscall word.

<!-- covers: GAP-0483, INV-0072, INV-0036, INV-0188, INV-0714 -->

#### Out of scope
Syscall packing of the handle word (ABI-010). Rights-bit encoding (CAP-010). Layer 1 freeze (ABI-049).

#### Acceptance criteria
- [ ] Options evaluated include at least: dense per-Component handle-table index; sparse unforgeable 64-bit token; sealed-pointer layout reservable for CHERI.
- [ ] Each option is backed by a prototype from CAP-013 and records CHERI implications from CAP-012.
- [ ] The accepted option states that userspace cannot mint a valid Capability and that the ABI reserves a sealed-pointer layout.
- [ ] Surface S-001 remains `prototyped`; the decision does not freeze it.
- [ ] Review records ABI lead sign-off on the pull request.

#### Verification
- Review: ABI lead sign-off recorded on the pull request.
- Manual: decision file lists at least two options, cites `reports/spikes/CAP-013.md` and `reports/spikes/CAP-012.md`, and names T-003.

#### Evidence
- none

### CAP-009 · Decide revocation semantics: eager vs lazy, in-flight Operations, cost bounds
- Type: adr
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: CAP-014, CAP-015, TSK-003, SEC-002
- Baseline: §7
- Decision: D-0058
- Risks: R-003
- Threats: T-005

Chooses how revoke walks derived Capabilities, what happens to in-flight Operations, and which cost bounds the implementation must meet. The V0 depth-8-within-one-Operation gate cannot be implemented without this choice. Coordinates with TSK on cancelling in-flight Operations; measured costs are published by the revocation spike, not claimed here.

<!-- covers: INV-0187, GAP-0484 -->

#### Out of scope
Implementing the walk (CAP-004). Measuring mint/derive/revoke in CI (CAP-018). Hardware-committed DMA cancel (TSK-017).

#### Acceptance criteria
- [ ] Options evaluated include at least: seL4-style derivation-tree walk; indirection with epoch invalidation; lazy check-on-use.
- [ ] The accepted option states when a descendant becomes unusable relative to one Operation, and what an in-flight Operation on a revoked handle returns.
- [ ] Each option cites the CAP-014 report and T-005.
- [ ] Review records TSK lead sign-off that in-flight Operation completion is compatible with TSK-003.

#### Verification
- Review: TSK lead and ABI lead sign-off recorded on the pull request.
- Manual: decision file lists at least two options and cites `reports/spikes/CAP-014.md`.

#### Evidence
- none

### CAP-010 · Decide rights and transfer-rights encoding including Admin authority
- Type: adr
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: CAP-013, CAP-012, SEC-002
- Baseline: §7, §8
- Decision: D-0059
- Risks: R-003, R-012
- Threats: T-004
- Invariants: I-028

Single decision for how rights and transfer or delegation rights are represented, including how Admin authority is expressed. Attenuation must be a subset check that a future hardware-tag path can perform without kernel metadata (S-003). The encoding stays prototyped through V0 (§7, §8).

<!-- covers: INV-0185, INV-0175, INV-0176 -->

#### Out of scope
Per-object-type registry of which bits exist (CAP-036). Implementing the checks (CAP-011). Handle-table layout (CAP-008).

#### Acceptance criteria
- [ ] Options evaluated include at least: generic bitmask; per-object-type typed rights; rights as separate kernel objects.
- [ ] The accepted option states how Admin is expressed and how transfer and delegation rights are distinct from object-operation rights.
- [ ] The accepted option requires that derive is a subset check and that S-003 remains hardware-checkable (R-012).
- [ ] Surface S-003 remains `prototyped`; the decision does not freeze it.
- [ ] Review records ABI lead sign-off on the pull request.

#### Verification
- Review: ABI lead sign-off recorded on the pull request.
- Manual: decision file lists at least two options, names S-003 and T-004, and cites CAP-012.

#### Evidence
- none

### CAP-011 · Implement rights and transfer-rights checks on every Capability Operation
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: CAP-010, CAP-005
- Baseline: §7
- Threats: T-004
- Invariants: I-028

Each Operation on an object checks the holder's rights word; each transfer checks delegation or transfer rights. Missing rights return `Error::Rights` and do not perform the Operation. Implements S-003 as accepted by CAP-010 (§7).

<!-- covers: INV-0175, INV-0176 -->

#### Out of scope
Per-type rights catalog for later object types (CAP-036). Channel slot move (IPC-014).

#### Acceptance criteria
- [ ] An Operation whose rights bit is absent returns `Error::Rights` and the object's state is unchanged, covered for Read, Write and a transfer attempt.
- [ ] A transfer without transfer rights returns `Error::Rights` and both tables are unchanged.
- [ ] Admin-gated Operations fail with `Error::Rights` when the handle lacks Admin, including on a handle derived from an Admin parent with Admin masked out.
- [ ] The rights check is the same code path for every object type registered at V0; adding a type without a rights declaration fails CI.

#### Verification
- Unit: `kernel:tests/cap/rights_check_*` on `qemu-x86_64` and `hw-h002`.
- Fuzz: `kernel:fuzz/cap_rights` one hour nightly without panic or rights amplification.
- Review: ABI lead sign-off that checks match S-003.

#### Evidence
- none

### CAP-012 · Study CHERI Capability hardware for ABI escape hatches
- Type: spike
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: none
- Baseline: §8, §38, §58, §65
- Explores: S-001, S-003
- Risks: R-012
- Invariants: I-058

Written study of CHERI capability hardware: what the ABI must reserve so a sealed-pointer layout and a subset-checkable rights word can be adopted later without breaking binaries. This spike precedes CAP-008 so V0 does not freeze the OS around current x86-64 limits (§8, §65).

<!-- covers: INV-1143 -->

#### Out of scope
QEMU-CHERI validation of freeze candidates (CAP-038). Morello mapping prototype (CAP-039). Handle-layout prototypes (CAP-013).

#### Acceptance criteria
- [ ] Report lists ABI fields that must remain opaque (no pointer-derived identity, no userspace-visible generation arithmetic) for a sealed-pointer path.
- [ ] Report states whether S-003 can be a subset check on a CHERI permission word, or which encoding change would be required.
- [ ] Report names at least one layout that would force a Layer 1 break if chosen now, and rules it out.

#### Verification
- Report: answers (1) which handle bits must stay reserved for a CHERI sealed pointer, (2) how rights map onto CHERI permissions, (3) what application-visible Capability operations remain stable under hardware enforcement, (4) what V0 must not freeze.
- Review: ABI lead records that CAP-008 cites this report.

#### Evidence
- none

### CAP-013 · Prototype dense index, sparse token and sealed-pointer Capability handle layouts
- Type: spike
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: none
- Baseline: §7, §8, §65
- Explores: S-001
- Risks: R-003

GAP-0483 requires the handle-representation ADR to be backed by a prototype of each option. Three layouts are built far enough to mint, look up and reject a forged handle: dense per-Component index, sparse unforgeable token, sealed-pointer-shaped value. Results feed CAP-008; nothing is frozen.

<!-- covers: GAP-0483 -->

#### Out of scope
Accepting the ADR (CAP-008). Production table (CAP-005). CHERI hardware study (CAP-012).

#### Acceptance criteria
- [ ] Each of the three layouts has a prototype that mints a handle, looks it up, and rejects a forged index or token with `Error::Rights`.
- [ ] Report records what each layout would need reserved for a later sealed-pointer path.
- [ ] Report does not claim a performance winner; any timing is labelled unpublished prototype measurement.

#### Verification
- Report: answers (1) forge resistance of each layout, (2) table density and generation reuse, (3) CHERI reservation, (4) which layouts remain viable for the ADR.
- Unit: prototype tests under `kernel:tests/cap/spike_handle_*` on `qemu-x86_64`.
- Review: ABI lead confirms the ADR cites this report.

#### Evidence
- none

### CAP-014 · Prototype and measure revocation strategies at one million derived Capabilities
- Type: spike
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: CAP-015
- Baseline: §7, §53
- Explores: S-003
- Threats: T-005

Derivation-tree walk, epoch indirection and lazy check-on-use are prototyped and measured at one million derived Capabilities. Revocation cost bounds how freely the OS can hand out attenuated Capabilities. Results are published in the spike report; they are not a gate target and are not restated as promises (§7, §53).

<!-- covers: GAP-0484 -->

#### Out of scope
Accepting the strategy (CAP-009). Production revoke (CAP-004). Standing CI harness (CAP-018).

#### Acceptance criteria
- [ ] Each of the three strategies revokes a root with one million descendants in a prototype on `qemu-x86_64` and on H-002.
- [ ] Report records whether each strategy makes every descendant fail within one Operation at depth 8.
- [ ] Report publishes the measurement method and does not state a superiority claim.

#### Verification
- Report: answers (1) which strategies meet the depth-8-within-one-Operation rule, (2) memory overhead of the derivation tree versus epoch table, (3) in-flight Operation behaviour, (4) the viable set for the ADR.
- Bench: prototype runs on H-001 and H-002; numbers live only in `reports/spikes/CAP-014.md`.
- Review: CAP lead confirms CAP-009 cites this report.

#### Evidence
- none

### CAP-015 · Study seL4 CSpaces, derivation trees, revocation and Verification for CAP
- Type: spike
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: none
- Baseline: §7, §58
- Explores: S-001, S-003

Written study of seL4 CSpaces, capability derivation trees, revocation and the formal-verification approach, kept in V0 because it directly informs the handle-representation and revocation ADRs (§58).

<!-- covers: INV-1132 -->

#### Out of scope
Other capability OS survey (CAP-026). Formal tooling choice (CAP-028). Production table (CAP-005).

#### Acceptance criteria
- [ ] Report describes seL4 CSpace lookup, derive, and revoke, and maps each onto a JakeOS table operation.
- [ ] Report names seL4 mechanisms CAP does not copy, with the reason, given Linux-derived kernel residency.
- [ ] Report is cited by CAP-008 and CAP-009.

#### Verification
- Report: answers (1) how seL4 derivation trees revoke, (2) what CSpace guard/radix choices mean for S-001, (3) what of seL4's formal spec applies to a Linux-derived kernel, (4) recommendations for the two V0 ADRs.
- Review: CAP lead sign-off that both V0 ADRs cite this report.

#### Evidence
- none

### CAP-016 · Lint native SDK and system Components for ambient authority and descriptor-shaped APIs
- Type: build
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: CAP-007, CAP-005
- Baseline: §1, §7, §9.1
- Threats: T-001
- Invariants: I-015, I-021

CI lint over native SDK crates and system Components: no entry opens objects by ambient path or a descriptor-shaped handle, and no system Component is launched with a wildcard Capability set. First enforced when the four V0.5 applications and the SDK expand (§9.1).

<!-- covers: INV-0042, INV-0200, INV-0036 -->

#### Out of scope
Personality libc and fd tables (LNX). Grant taxonomy (SEC-007). Graph-description lint (CAP-024).

#### Acceptance criteria
- [ ] A native crate that exposes an API taking a filesystem path as authority fails the lint in CI.
- [ ] A native crate that names `fd`, `FILE*` or a Linux syscall number on a public native surface fails the lint unless an accepted Decision exempts that symbol.
- [ ] A system Component manifest with a wildcard Capability set fails the lint.
- [ ] The four V0.5 application crates pass the lint on `qemu-x86_64`.

#### Verification
- Unit: lint fixtures under `sdk:tests/lint/ambient_*` that fail on planted path and fd APIs and pass on Capability-typed APIs.
- Integration: CI job on every native crate in the V0.5 image.
- Review: SDK lead sign-off recorded on the pull request.

#### Evidence
- none

### CAP-017 · Gate persistent background execution behind Capability<BackgroundExecution>
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: CAP-010, CAP-005, CAP-011
- Baseline: §7, §21
- Threats: T-018
- Invariants: I-031

Persistent background execution beyond the owning application's lifetime requires `Capability<BackgroundExecution>`. CAP defines the type, mints it only from an explicit grant, and rejects a derive that adds it. TSK enforces the check when a Task would outlive its TaskGroup (§21).

<!-- covers: INV-0391 -->

#### Out of scope
TaskGroup teardown and orphan detection (TSK-025). Service supervision (SVC). Automation rules (SEM-030).

#### Acceptance criteria
- [ ] Minting `Capability<BackgroundExecution>` without an explicit grant source returns `Error::Rights` and allocates no handle.
- [ ] A Component that does not hold `Capability<BackgroundExecution>` cannot transfer that type; the send returns `Error::Rights`.
- [ ] `os inspect capability` on a holder shows the BackgroundExecution type tag and rights.
- [ ] The type is declared in the V0.5 rights table so a later object-rights-registry lint accepts it.

#### Verification
- Unit: `kernel:tests/cap/background_exec_*` on `qemu-x86_64` and `hw-h002`.
- Integration: a V0.5 service Component starts with the Capability; a demo app Component without it cannot keep a Task after its window TaskGroup is cancelled (TSK observes `Error::Rights`).
- Review: TSK lead sign-off that the type is the one TSK-025 checks.

#### Evidence
- none

### CAP-018 · Benchmark Capability mint, derive, transfer and revocation cost
- Type: benchmark
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: CAP-003, CAP-004, CAP-006, CAP-009, BEN-005, BEN-007
- Baseline: §7, §53, §54
- Benchmarks: B-053
- Invariants: I-061

Standing harness for mint, derive, transfer and revocation cost so the bounds named by CAP-009 are measured in CI rather than claimed. V0.5 re-runs V0 benchmarks under B-051; this harness is publish-only at V0.5 with no absolute target (§53, §54).

#### Out of scope
Choosing the revocation strategy (CAP-009). Component-creation cost (CMP-001, B-001). IPC round trip (B-004).

#### Acceptance criteria
- [ ] Harness measures mint, derive, transfer and revoke on H-001 and H-002 using the method recorded in BEN-007.
- [ ] A report is committed per in-scope H-ID for this milestone with no superiority claim in the report prose.
- [ ] The harness is invoked from the shared BEN runner on every merge that touches `cap/`.

#### Verification
- Bench: B-053 mint, derive, transfer and revocation cost on H-001 and H-002; V0.5 target is publish; B-051 re-runs prior-rung reports on the same hardware.
- Review: BEN lead sign-off that the harness matches the methodology Decision and I-061.

#### Evidence
- none

### CAP-019 · Write property-based tests for Capability unforgeability, typing and attenuation
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: CAP-003, CAP-004, CAP-005, CAP-028
- Baseline: §7, §9.1, §51
- Threats: T-003, T-004
- Invariants: I-028, I-021

One property and model-based suite enforces the standing Capability invariants: userspace cannot mint a valid handle, a type mismatch fails at the kernel boundary, derive never widens rights, revocation invalidates every descendant, and a Component starts with exactly the Capabilities it was handed. Example-based tests cannot show the absence of escalation paths (§9).

<!-- covers: GAP-0131, INV-0177, INV-0178, INV-0184, INV-0200 -->

#### Out of scope
Machine-checked model in CI (CAP-035). Tooling choice (CAP-028). Fuzz targets for V2 (CAP-042).

#### Acceptance criteria
- [ ] A forged handle index and generation pair is rejected with `Error::Rights` on every Operation, with no object access and no new table entry, on `qemu-x86_64`.
- [ ] Using a `Capability<File>` where `Capability<Channel>` is required fails at the kernel boundary with a typed error and does not invoke the Channel path.
- [ ] A property test over random rights masks never observes a derived Capability whose rights are not a subset of the parent.
- [ ] After revoke of the root, every derived handle at depths 1 through 8 fails its next Operation in the same suite.

#### Verification
- Unit: `kernel:tests/cap/property_*` on `qemu-x86_64` and `hw-h002`.
- Fuzz: `kernel:fuzz/cap_invariants` one hour nightly without panic or invariant violation.
- Review: CAP lead records that the suite covers unforgeability, typing, attenuation and revocation completeness.

#### Evidence
- none

### CAP-020 · Decide Capability persistence across Component restart and reboot
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: CAP-026, CAP-007, SEC-007
- Baseline: §7, §9.1, §25
- Decision: D-0052
- Threats: T-002
- Invariants: I-035

Sturdy references, a revocable persistent grant store, or re-prompt on every launch: how a Component retains access to a user-selected object across restart and reboot, and which object types may ever be persisted. A photo editor that loses every chosen file on relaunch is unusable; persistence that cannot be revoked weakens §7.

<!-- covers: GAP-0486, INV-0491 -->

#### Out of scope
Implementing the store (CAP-037). Publisher-keyed continuity (CAP-043). Chooser minting (STO-034).

#### Acceptance criteria
- [ ] Options evaluated include at least: sturdy references; revocable persistent grant store; re-prompt on every launch.
- [ ] The accepted option names which object types may persist and which must not (for example, never persist Admin or debug-attach).
- [ ] The accepted option states how the user revokes a persisted grant and that revoke invalidates derived handles.
- [ ] Review records SEC and STO lead sign-off on the pull request.

#### Verification
- Review: SEC lead and STO lead sign-off recorded on the pull request.
- Manual: decision file lists at least two options, cites `reports/spikes/CAP-026.md`, and names Q-016 as the question it answers.

#### Evidence
- none

### CAP-021 · Decide the ABI invariants required for hardware-assisted Capability enforcement
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: CAP-027, CAP-012, CAP-008, CAP-010
- Baseline: §8, §38, §65
- Decision: D-0057
- Risks: R-012
- Invariants: I-058, I-028

The list of ABI invariants that keep application-visible Capabilities stable whether enforcement is kernel metadata, page tables, CPU tags or CHERI. Candidates include handle opacity, no pointer-derived identity, and rights living in the object not the handle. This is the hardware-assisted readiness ADR promised by §8.

<!-- covers: INV-0199, INV-0196, INV-0188 -->

#### Out of scope
Concrete CHERI/Morello mapping (CAP-033). Enforcement-backend trait (CAP-034). Layer 1 freeze (ABI).

#### Acceptance criteria
- [ ] Options evaluated include at least: kernel-metadata enforcement with a published invariant list and reserved handle bits; dual representation that already uses a sealed-pointer-shaped handle on x86-64; CHERI-shaped handles from V0.5 (rejected unless the spike supports it).
- [ ] The accepted option publishes the invariant list that CAP-034 and later mapping ADRs must honour.
- [ ] Application-visible mint, derive, transfer and revoke semantics are stated to remain unchanged across backends.
- [ ] Review records ABI lead sign-off on the pull request.

#### Verification
- Review: ABI lead sign-off recorded on the pull request.
- Manual: decision file lists at least two options and cites `reports/spikes/CAP-027.md` and `reports/spikes/CAP-012.md`.

#### Evidence
- none

### CAP-022 · Decide how a Component obtains its initial and later Capabilities
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: CAP-007, SEC-002
- Baseline: §7, §9.1, §32
- Decision: D-0060
- Threats: T-001, T-002

Namespace object, broker Component, or manifest-declared static wiring: how a Component obtains Capabilities after creation, including rebind after a peer restarts. Without this choice every service invents a bootstrap protocol and ambient authority returns. The V0 fault demo rebinds A to a restarted B using the initial handoff this decision names (§9.1).

<!-- covers: GAP-0485 -->

#### Out of scope
Implementing the chosen namespace or broker (CAP-023). Supervisor restart budgets (SVC). IDL rebind contract (IPC).

#### Acceptance criteria
- [ ] Options evaluated include at least: Capability namespace object; broker Component; manifest-declared static wiring.
- [ ] The accepted option states how a Component obtains its initial set and how it obtains a Capability to a restarted peer without a wildcard grant.
- [ ] Each option cites T-001 and T-002 and records whether a confused deputy can mint Capabilities for callers.
- [ ] Review records SVC and IPC lead sign-off on the pull request.

#### Verification
- Review: SVC lead and IPC lead sign-off recorded on the pull request.
- Manual: decision file lists at least two options and names T-001 and T-002.

#### Evidence
- none

### CAP-023 · Implement the Capability discovery namespace decided in the service discovery ADR
- Type: build
- Milestone: V0.5
- Status: todo
- Size: L
- Owner: none
- Depends on: CAP-022, CAP-005, CAP-006
- Baseline: §7, §9.1, §32
- Threats: T-002

V0.5 native init and compositor crash-rebind need a defined way for Components to obtain later Capabilities. This task implements the mechanism accepted by CAP-022 so services do not invent bootstrap protocols. A Component still receives only what that mechanism hands it.

<!-- covers: GAP-0485 -->

#### Out of scope
Supervisor restart budgets and readiness (SVC). Client rebind in the SDK (SDK). IDL disconnect contract (IPC). Grant sources (CAP-007).

#### Acceptance criteria
- [ ] A Component obtains a Capability to a named service only through the chosen mechanism; a raw table insert of a service handle from userspace returns `Error::Rights`.
- [ ] After the service Component restarts, a client obtains a new Capability to the replacement instance without a wildcard grant, on `qemu-x86_64`.
- [ ] `os inspect` shows which namespace or broker entry handed each live service Capability.
- [ ] A Component that was not granted lookup rights receives `Error::Rights` and allocates no handle.

#### Verification
- Unit: `kernel:tests/cap/discovery_*` on `qemu-x86_64` and `hw-h002`.
- Integration: compositor kill-and-rebind on H-001 and H-003 obtains a rebound Capability without widening rights.
- Review: SVC lead sign-off that the mechanism is the one SVC restart uses.

#### Evidence
- none

### CAP-024 · Lint Component graph descriptions for explicit per-Component Capability sets
- Type: build
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: CAP-007, CMP-022
- Baseline: §9.1, §11
- Invariants: I-021

Standing invariant over CMP graph descriptions: every Component in a graph declares its Capability set and receives nothing else. First enforced when component graphs arrive at V0.5 (§11).

<!-- covers: INV-0235 -->

#### Out of scope
Instantiating graphs (CMP-024). Attenuating parent-to-child at runtime (CMP-023). Manifest schema (PKG).

#### Acceptance criteria
- [ ] A graph description that omits a Capability set on any Component fails the lint.
- [ ] A graph description that grants a child a Capability the parent does not hold fails the lint.
- [ ] The four V0.5 application graphs pass the lint on `qemu-x86_64`.

#### Verification
- Unit: lint fixtures under `cmp:tests/lint/graph_caps_*`.
- Integration: CI job on every packaged graph in the V0.5 image.
- Review: CMP lead sign-off recorded on the pull request.

#### Evidence
- none

### CAP-025 · Attach only the granted Capability set to a launched Component
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: CAP-007, CAP-005, CAP-003, SEC-007, PKG-028, CMP-027
- Baseline: §9.1, §11, §28
- Threats: T-001
- Invariants: I-021

Launch-time binding of the sources accepted by CAP-007: the launcher resolves Package requested Capabilities and user grants into a set, and CMP creation receives exactly that set. V0.5 exit: Image Viewer starts with UI and GPU Capabilities only and cannot open other files (§9.1).

<!-- covers: INV-0076, INV-0200, INV-0042 -->

#### Out of scope
Manifest schema (PKG-031). Chooser UI (APP-002). UserSelected minting (STO-034). Component create syscall (CMP-027).

#### Acceptance criteria
- [ ] Image Viewer launched from its Package starts with UI and GPU Capabilities only; opening a File without a UserSelected grant returns `Error::Rights` on `qemu-x86_64`.
- [ ] A requested Capability that the user or policy denied is absent from the table; launching a Package whose required request was denied fails with a typed error and creates no Component.
- [ ] `os inspect capability` on the launched Image Viewer lists only the bound set.
- [ ] The V0.5 chooser demo audit line shows exactly one `Capability<Image, Read>` granted after a successful choose.

#### Verification
- Integration: Image Viewer launch and denial suite on H-001 and H-002.
- Demo: Image Viewer chooses a photo; audit log shows one Image Capability on H-002.
- Review: SEC lead sign-off that binding honours SEC-007.

#### Evidence
- none

### CAP-026 · Survey KeyKOS, EROS, CapROS, Capsicum, Barrelfish and Theseus Capability patterns
- Type: spike
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: none
- Baseline: §7, §25, §58
- Explores: S-022

Research study moved out of V0 because it does not inform a V0 surface. Persistence patterns (KeyKOS/EROS sturdy references) and confinement patterns feed CAP-020.

<!-- covers: INV-1150 -->

#### Out of scope
seL4 study (CAP-015). Implementing persistence (CAP-037). Grant schema surface freeze (SEC, S-022).

#### Acceptance criteria
- [ ] Report compares persistence, revocation and confinement across KeyKOS, EROS, CapROS, Capsicum, Barrelfish and Theseus.
- [ ] Report recommends which persistence pattern is viable on a Linux-derived kernel with a revocable store.
- [ ] Report is cited by CAP-020.

#### Verification
- Report: answers (1) how each system persists capabilities across restart, (2) how revoke interacts with persistence, (3) confinement patterns that map onto Component graphs, (4) recommendations for the persistence ADR.
- Review: CAP lead sign-off that the persistence ADR cites this report.

#### Evidence
- none

### CAP-027 · Document how Capability<T> maps onto CHERI-style Capability pointers
- Type: spike
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: CAP-012, CAP-008
- Baseline: §8, §38
- Explores: S-003
- Invariants: I-058

Documents the mapping from application-visible `Capability<T>` onto CHERI-style capability pointers so CAP-021 can list invariants rather than invent them. No freeze.

<!-- covers: INV-0189 -->

#### Out of scope
Morello emulator prototype (CAP-039). Readiness ADR (CAP-021). MemoryObject mapping (MEM).

#### Acceptance criteria
- [ ] Report maps mint, derive, transfer and revoke onto CHERI permission and bounds operations.
- [ ] Report states which JakeOS Capability operations have no CHERI equivalent and how software metadata covers them.
- [ ] Report does not require a Layer 1 break to adopt the mapping later.

#### Verification
- Report: answers (1) 1:1 versus metadata-plus-tag mapping, (2) how attenuation becomes a permission subset, (3) what stays in kernel metadata, (4) invariants for the readiness ADR.
- Review: ABI lead sign-off that CAP-021 cites this report.

#### Evidence
- none

### CAP-028 · Evaluate Kani, TLA+ and Alloy for modelling the Capability derivation core
- Type: spike
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: none
- Baseline: §7, §51, §58

GAP-0131 asks for a tooling evaluation before the V1 machine-checked model. The report compares Kani, TLA+ and Alloy on the derivation core (attenuation never widens, revoke is complete, transfer preserves type) and recommends one target so CAP-035 is not invented on the freeze-candidate path.

<!-- covers: GAP-0131 -->

#### Out of scope
The machine-checked model itself (CAP-035). Property-based tests in Rust (CAP-019).

#### Acceptance criteria
- [ ] Report builds a minimal model of derive and revoke in at least two of Kani, TLA+ and Alloy.
- [ ] Report records what each tool can check (panic freedom, subset, completeness) and what it cannot.
- [ ] Report recommends one primary tool for CAP-035 with a rejected alternative.

#### Verification
- Report: answers (1) which tool models derive/revoke/transfer, (2) whether it can express unforgeability, (3) CI cost of a regression check, (4) the recommended tool and why the others lost.
- Review: CAP lead sign-off that CAP-035 names this recommendation.

#### Evidence
- none

### CAP-029 · Test that UserSelected<T> results carry authority with no path-based check
- Type: build
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: CAP-007, CAP-003, CAP-001
- Baseline: §9.1, §25
- Threats: T-002
- Invariants: I-016, I-035

Standing test gate: the chooser (STO) returns a Capability and no subsequent path permission check exists. Verifies the V0.5 chooser exit criterion and the demo audit line (§25).

<!-- covers: INV-0488 -->

#### Out of scope
Chooser UI (APP-002). Minting UserSelected (STO-034). Save-as flow (STO-015).

#### Acceptance criteria
- [ ] After `files.choose<Image>()`, the Image Viewer holds `Capability<Image, Read>` (or ReadWrite as minted) and a directory listing of the containing folder returns `Error::Rights`.
- [ ] Opening a sibling file by any native API returns `Error::Rights` and allocates no handle.
- [ ] The audit log for the choose contains the minted Capability and no path string used as an authority check.
- [ ] The test is retained permanently in CI on `qemu-x86_64`.

#### Verification
- Integration: `runtime:tests/cap/userselected_gate_*` on `qemu-x86_64` and `hw-h002`.
- Demo: V0.5 Image Viewer choose-photo demo audit line on H-002.
- Review: STO lead sign-off that no path check remains after mint.

#### Evidence
- none

### CAP-030 · Export Capability audit events for the permissions log viewer and os trace
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: CAP-001
- Baseline: §7, §24, §64

V1 ships a grant log viewer and requires `os trace` session export. CAP exports structured grant, derive, revoke and denial events in the inspect and trace schemas so APP and SDK can render them without parsing kernel text.

<!-- covers: INV-0183, INV-0463 -->

#### Out of scope
Log viewer UI (APP-012). Offline trace format (SDK-051). Tamper-evident store (OBS-044).

#### Acceptance criteria
- [ ] A trace session export includes every grant, derive, revoke and denial that occurred in the session, with holder, object identity and rights.
- [ ] The export schema is versioned; an unknown newer field is ignored by a V0.5 reader of the same records.
- [ ] A Component without inspect rights on another Component's audit stream receives `Error::Rights` and an empty export.

#### Verification
- Integration: export a V1 editor session that opened two UserSelected files and revoked one; the export contains three matching events, on `qemu-x86_64`.
- Review: OBS and SDK leads sign off that the schema is the one `os trace` writes.

#### Evidence
- none

### CAP-031 · Declare Capability L1 surfaces freeze candidates with conformance tests
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: CAP-038, CAP-035, CAP-034, CAP-010, CAP-008, CAP-021, CAP-036
- Baseline: §8, §65, §66
- Risks: R-007
- Invariants: I-040

V1 names Layer 1 freeze candidates with SDK v1. CAP marks S-003 (and the Capability operations on S-001) as candidates, each citing its spike, ADR and CHERI validation. Nothing is frozen; I-040 forbids an L1 freeze before V4.

Required by V4-G01 (Layer 1 ABI frozen with a conformance suite): S-001 through S-012 freeze with conformance tests, and this task names the CAP candidates and their first conformance tests.

#### Out of scope
Accepting the freeze (ABI-049). V4 conformance suite (CAP-051). CHERI re-validation of the frozen ABI (CAP-052).

#### Acceptance criteria
- [ ] Surfaces register entries owned by CAP that are freeze candidates name this task in their candidate notes and remain `prototyped`, not `frozen`.
- [ ] Each candidate cites a done spike that explores it, a done adr task, and CAP-038.
- [ ] A conformance test exists for mint, derive, transfer, revoke, inspect and rights-mismatch on `qemu-x86_64`.
- [ ] CI fails if a candidate is marked `frozen` before V4.

#### Verification
- Unit: candidate conformance tests `kernel:tests/cap/conformance_v1_*` on `qemu-x86_64` and `hw-h002`.
- Review: ABI lead sign-off that no CAP L1 surface is `frozen` and that ABI-034 lists these candidates.
- Integration: register-state check in CI.

#### Evidence
- none

### CAP-032 · Model debugger attach, tracing and memory reading as explicit Capabilities
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: CAP-036, CAP-005, CAP-003, CAP-007
- Baseline: §7, §9.1, §51
- Threats: T-027

Debugger attachment, tracing of another Component, and reading another Component's memory are explicit Capabilities, not same-user checks. V1 native debugger attach and the LNX ptrace-equivalent consume these types so ambient ptrace-style attach cannot leak authority between sandboxed applications.

<!-- covers: GAP-0189 -->

#### Out of scope
Debugger protocol and UI (SDK-038, SDK-052). Personality ptrace equivalent (LNX-049). Trace access policy (OBS-024).

#### Acceptance criteria
- [ ] Attach, trace-another, and read-memory are distinct rights on a debug Capability type declared in the object-rights registry.
- [ ] A debugger Component without the attach Capability receives `Error::Rights` and does not stop the target, on `qemu-x86_64`.
- [ ] Holding attach does not imply read-memory; a read without that right returns `Error::Rights` and copies no bytes.
- [ ] Same-user identity without the Capability is not sufficient; a test with two Components of one user confirms denial.
- [ ] Revoking the attach Capability detaches the debugger at the next Operation.

#### Verification
- Unit: `kernel:tests/cap/debug_attach_*` on `qemu-x86_64` and `hw-h002`.
- Integration: SDK debugger attach granted versus denied on H-001.
- Review: SDK and LNX leads sign off that attach is this Capability, not a uid check.

#### Evidence
- none

### CAP-033 · Decide the Capability<T> and MemoryObject mapping onto hardware capabilities
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: CAP-039, CAP-040, CAP-021
- Baseline: §8, §38
- Decision: D-0056
- Invariants: I-058

Records the CHERI/Morello and tagged-memory findings as the mapping the ABI must remain compatible with. Application-visible Capabilities stay conceptually stable (§8).

<!-- covers: INV-0719, INV-0190 -->

#### Out of scope
Enforcement-backend trait (CAP-034). Freeze (ABI). MemoryObject property bits (MEM).

#### Acceptance criteria
- [ ] Options evaluated include at least: 1:1 map of `Capability<T>` onto a CHERI capability; split where MemoryObject uses tagged memory and Capability stays a table handle; document-only mapping with no kernel backend until hardware exists in lab.
- [ ] The accepted option honours the invariant list from CAP-021.
- [ ] The accepted option states what CAP-034 must implement on x86-64 today.
- [ ] Review records ABI and MEM lead sign-off on the pull request.

#### Verification
- Review: ABI lead and MEM lead sign-off recorded on the pull request.
- Manual: decision file lists at least two options and cites `reports/spikes/CAP-039.md` and `reports/spikes/CAP-040.md`.

#### Evidence
- none

### CAP-034 · Abstract Capability enforcement behind a kernel backend Interface
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: CAP-021, CAP-033, CAP-005, CAP-011
- Baseline: §8, §51
- Invariants: I-058, I-028

A kernel backend interface so Capability checks run on kernel metadata today and on hardware later. Must exist before freeze candidates are declared so conformance tests exercise the abstraction rather than a single hardcoded path (§8).

<!-- covers: INV-0197, INV-0196 -->

#### Out of scope
CHERI emulator validation (CAP-038). Production CHERI hardware. Rights encoding choice (CAP-010).

#### Acceptance criteria
- [ ] Mint, derive, lookup, transfer and revoke call a backend trait; the x86-64 kernel-metadata backend is the default on H-001 and H-002.
- [ ] A test backend that denies every lookup causes every object Operation to return `Error::Rights` without changing table code.
- [ ] Switching backends does not change application-visible handle values on the metadata backend.
- [ ] Conformance tests for derive subset and type mismatch pass on the metadata backend on `qemu-x86_64`.

#### Verification
- Unit: `kernel:tests/cap/backend_*` on `qemu-x86_64` and `hw-h002`.
- Integration: metadata backend runs the V1 candidate conformance suite.
- Review: ABI lead sign-off that the trait matches CAP-033.

#### Evidence
- none

### CAP-035 · Build a machine-checked model of Capability derivation, transfer and revocation
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: CAP-028, CAP-003, CAP-004, CAP-006, CAP-009, CAP-010
- Baseline: §7, §51
- Invariants: I-028

A machine-checked model of derive, attenuation, transfer and revoke in the tool chosen by CAP-028. Security invariants that must hold for decades are checked here before freeze candidates, and re-checked at the V4 freeze. Model checking runs in CI.

<!-- covers: GAP-0488 -->

#### Out of scope
Tooling choice (CAP-028). Rust property tests (CAP-019). CHERI emulator (CAP-038).

#### Acceptance criteria
- [ ] The model expresses mint, derive, transfer and revoke and is checked in CI on every change to `cap/`.
- [ ] The checker reports a failure if a spec mutation allows rights amplification or a surviving descendant after root revoke.
- [ ] CI on `qemu-x86_64` runs the model check as a required job.
- [ ] The model is the one CAP-052 re-runs at V4.

#### Verification
- Unit: model-check job `kernel:formal/cap` in CI.
- Integration: a planted amplification in a branch fails CI; the main model passes.
- Review: CAP lead sign-off that the tool is the one recommended by CAP-028.

#### Evidence
- none

### CAP-036 · Register typed rights and Admin authority for every kernel Object type
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: CAP-010, CAP-011, CAP-005
- Baseline: §7
- Invariants: I-028

V1 introduces NetworkConnection, AudioStream, Surface and Device object types. §7 requires per-type rights. A lint fails when a new object type lacks a rights declaration, scaling CAP-010 as the object set grows.

Required by the CAP scope: "rights and transfer-rights encoding" for every Object type native software can hold.

#### Out of scope
Defining those objects (NET, AUD, GFX, HW). Encoding choice (CAP-010). Grant UI (APP).

#### Acceptance criteria
- [ ] Every Object type in the V1 ABI object registry has a rights declaration including Admin and transfer bits.
- [ ] Adding an Object type without a rights declaration fails CI.
- [ ] NetworkConnection, AudioStream, Surface and Device declarations exist and are used by `Error::Rights` tests for a missing bit of each type.
- [ ] `os inspect capability` prints the per-type rights names, not only a raw mask.

#### Verification
- Unit: `kernel:tests/cap/rights_registry_*` on `qemu-x86_64`.
- Integration: CI lint over the object registry.
- Review: NET, AUD, GFX and HW leads confirm their types are declared.

#### Evidence
- none

### CAP-037 · Implement the revocable persistent grant store with user revocation
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: CAP-020, CAP-005, CAP-004, CAP-001, STO-034, SEC-007
- Baseline: §7, §9.1, §25
- Threats: T-002
- Invariants: I-035

V1 daily-driving requires editors to retain user-selected files across restarts. Implements the persistence Decision, including a recent-files list of persisted grants and user revocation that invalidates derived handles.

<!-- covers: GAP-0486, INV-0491 -->

#### Out of scope
Publisher-keyed continuity (CAP-043). Grant settings UI (APP-029). Chooser minting (STO-034). Per-user isolation (CAP-049).

#### Acceptance criteria
- [ ] An editor that received `UserSelected<File>` and persisted the grant still holds a usable Capability after Component restart, on `qemu-x86_64`.
- [ ] User revocation of that grant makes the next Operation on the restored handle return `Error::Rights` and allocates no new handle.
- [ ] Object types the persistence Decision forbids cannot be written to the store; the insert returns `Error::Rights`.
- [ ] Audit log records persist and revoke of grants.

#### Verification
- Integration: editor persist-restart-revoke suite on H-001 and H-004.
- Unit: `runtime:tests/cap/grant_store_*`.
- Review: SEC lead sign-off that store classes match SEC-007.

#### Evidence
- none

### CAP-038 · Validate Capability ABI freeze candidates on QEMU-CHERI
- Type: spike
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: CAP-039, CAP-034, CAP-003, CAP-004, CAP-021
- Baseline: §8, §65
- Explores: S-001, S-003
- Invariants: I-058

Validates the prototyped Capability ABI on QEMU-CHERI so hardware-readiness is tested before V1 freeze candidacy. Re-run on the frozen ABI by CAP-052 at V4. No L1 freeze happens here.

<!-- covers: GAP-0487 -->

#### Out of scope
Declaring freeze candidates (CAP-031). V4 re-validation (CAP-052). Production CHERI hardware.

#### Acceptance criteria
- [ ] Mint, derive, transfer and revoke of a File Capability succeed on QEMU-CHERI with the same application-visible errors as on `qemu-x86_64`.
- [ ] A forged handle is rejected with `Error::Rights` on QEMU-CHERI.
- [ ] Report lists every ABI assumption that broke on QEMU-CHERI and whether it is fixed or deferred with a named follow-up.

#### Verification
- Report: answers (1) which candidate operations pass on QEMU-CHERI, (2) which invariants from CAP-021 held, (3) which failed and the fix or deferral, (4) whether candidacy can proceed.
- Integration: QEMU-CHERI run of `kernel:tests/cap/derive_*` and `revoke_*`.
- Review: ABI lead sign-off that freeze-candidate review cites this report.

#### Evidence
- none

### CAP-039 · Prototype Capability<T> and MemoryObject mapping on a CHERI/Morello emulator
- Type: spike
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: CAP-021, CAP-027
- Baseline: §8, §38
- Explores: S-003

Concrete mapping prototype on a CHERI/Morello emulator, preceding CAP-033 and CAP-038. MEM is consulted for MemoryObject implications; CAP owns the Capability side.

<!-- covers: INV-0719 -->

#### Out of scope
Tagged-memory study (CAP-040). Mapping ADR (CAP-033). MemoryObject kernel mapping (MEM).

#### Acceptance criteria
- [ ] Prototype holds a `Capability<File>` as a CHERI capability and performs a subset derive on the emulator.
- [ ] Report records how MemoryObject identity relates to the Capability (shared tag, split, or software metadata).
- [ ] Report names at least one mapping that would break application-visible semantics and rules it out.

#### Verification
- Report: answers (1) representation of `Capability<T>` on Morello, (2) MemoryObject relationship, (3) operations that stay in software, (4) input to the mapping ADR.
- Review: MEM lead comments on the MemoryObject section; ABI lead confirms the mapping ADR cites this report.

#### Evidence
- none

### CAP-040 · Study tagged memory implications for MemoryObject and Capability storage
- Type: spike
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: CAP-021, CAP-027
- Baseline: §8, §38
- Explores: S-003

Study of tagged-memory architectures and what they imply for storing Capabilities and MemoryObject metadata. Input to CAP-033. MEM is consulted; CAP does not change MemoryObject properties here.

<!-- covers: INV-0190 -->

#### Out of scope
Morello prototype (CAP-039). MemoryObject ABI (MEM, S-006). Mapping ADR (CAP-033).

#### Acceptance criteria
- [ ] Report describes at least two tagged-memory designs and how each would store a Capability rights word.
- [ ] Report states whether application-visible handles must change if tags are adopted.
- [ ] Report is cited by CAP-033.

#### Verification
- Report: answers (1) tag width versus S-003, (2) interaction with MemoryObject, (3) whether kernel metadata remains necessary, (4) recommendations for the mapping ADR.
- Review: MEM lead sign-off on the MemoryObject section.

#### Evidence
- none

### CAP-041 · Make AI action Capabilities revocable mid-run with a logged action graph
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: SEM-029, SEM-010, CAP-004, CAP-001
- Baseline: §7, §44, §57
- Threats: T-017
- Invariants: I-051, I-023

V2 AI demo: the full action graph is logged and one step is revoked mid-run. CAP makes each AI action a revocable Capability so the broker cannot keep acting on a step the user revoked. Depends on a Semantic interface registry; no AI work precedes that registry (§44, §57).

<!-- covers: INV-0828 -->

#### Out of scope
AI broker implementation (SEM-010). Registry (SEM-029). Action-graph logging store (OBS-042). Assistant host (SEM-012).

#### Acceptance criteria
- [ ] Each step in the Workspace.search to Editor.runTests graph holds a distinct Capability that `os inspect` lists.
- [ ] Revoking the Capability for one step causes that step's next Operation to return `Error::Rights` and later steps are not issued, on `qemu-x86_64`.
- [ ] The audit log contains the graph, the grants, and the mid-run revoke.
- [ ] Building an AI-broker crate that links without a done SEM registry task fails CAP/SEM lint SEM-001.

#### Verification
- Integration: V2 AI demo on H-002 with mid-run revoke.
- Demo: action graph logged and one step revoked, on H-002.
- Review: SEM lead sign-off that revoke uses these Capabilities, not a broker-internal flag.

#### Evidence
- none

### CAP-042 · Add Capability forge, type-confusion and derivation fuzz targets
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: CAP-019, BLD-035, BLD-016
- Baseline: §7, §51
- Risks: R-051
- Threats: T-003, T-004

Capability-specific fuzz targets (forge, type confusion, derive amplification) land in BLD's continuous fuzzing so they are seasoned before the V3 crasher-age gate and the V4 longer window.

<!-- covers: INV-0961 -->

#### Out of scope
Fuzz infrastructure (BLD-035). syzkaller port (BLD-016). Closing audit findings (CAP-050).

#### Acceptance criteria
- [ ] Harnesses exist for forged handles, wrong-type Operations, and derive masks, registered with BLD's continuous fuzzer.
- [ ] A planted amplification crash is found by the derive harness in a CI smoke run.
- [ ] Targets run on the V2 nightly fuzz fleet; crashers file tasks with a reproducing input.

#### Verification
- Fuzz: `kernel:fuzz/cap_forge`, `cap_type`, `cap_derive` on BLD continuous infra.
- Review: BLD lead sign-off that targets are in the V2 corpus consumed by V3/V4 crasher-age gates.

#### Evidence
- none

### CAP-043 · Key persistent grants on Package identity and publisher, revoke on publisher change
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: CAP-037, PKG-055, PKG-029
- Baseline: §7, §28
- Risks: R-078
- Threats: T-033

Persistent grants key on Package identity plus publisher, not content hash, so an application update does not drop grants. A publisher change revokes them. V2 store-client and update flows would otherwise force a re-prompt on every update.

<!-- covers: EXTRA-018 -->

#### Out of scope
Publisher identity scheme for the public repository (REL-025). Store client UI (APP-045). Signing hierarchy (REL-002).

#### Acceptance criteria
- [ ] Updating a Package to a new content hash with the same identity and publisher leaves persisted grants usable after relaunch.
- [ ] Replacing the Package with the same name and a different publisher makes the next Operation on a persisted grant return `Error::Rights`.
- [ ] Audit log records the revoke-on-publisher-change event with old and new publisher identities.
- [ ] Grants are never keyed only on content hash; a unit test of the store index proves the key contains identity and publisher.

#### Verification
- Integration: update-same-publisher and replace-publisher suites on `qemu-x86_64` and H-002.
- Unit: `runtime:tests/cap/grant_continuity_*`.
- Review: PKG lead sign-off that identity fields match PKG-029.

#### Evidence
- none

### CAP-044 · Test immediate revocation across camera, microphone, files, network and screen capture
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: CAP-004, CAP-036, CAP-037, NET-012, AUD-003, STO-034, GFX-061, MED-013
- Baseline: §7, §9.1
- Threats: T-005, T-013, T-014

V2 permissions UI revocation takes effect immediately. CAP owns the cross-object-type matrix; SEC and APP drive the UI. After revoke, camera, microphone, files, network and screen capture fail on the next Operation with no leftover mapping.

#### Out of scope
Permissions UI (APP-029, APP-025). SEC's UI-level proof (SEC-036). Device stacks (MED, AUD, NET, GFX).

#### Acceptance criteria
- [ ] Revoking Camera makes the next capture Operation return `Error::Rights` and delivers no frame, on `qemu-x86_64`.
- [ ] Revoking microphone, File, network connect and screen-capture each fail the next Operation of that type with `Error::Rights` and no leftover mapping.
- [ ] The matrix runs as one CI job naming all five types; a missing type fails the job.
- [ ] In-flight Operations on revoked handles complete with a typed error and never a successful result.

#### Verification
- Integration: `runtime:tests/cap/revoke_matrix_*` on `qemu-x86_64` and H-002.
- Review: SEC lead sign-off that the matrix is the one SEC-036 cites.

#### Evidence
- none

### CAP-045 · Study hardware provenance tracking for Capability audit
- Type: spike
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: CAP-033, CAP-001
- Baseline: §8

§8 research item at its inventory rung: whether hardware provenance tracking can feed Capability audit. Non-gating input to audit design. Does not change the V2 audit export.

<!-- covers: INV-0192 -->

#### Out of scope
Audit export (CAP-030). CHERI mapping (CAP-033). Tamper-evident log (OBS-044).

#### Acceptance criteria
- [ ] Report describes at least one hardware provenance mechanism and how it would attach to a Capability audit record.
- [ ] Report states whether application-visible audit records must change to use it.
- [ ] Report records a keep-software-audit option.

#### Verification
- Report: answers (1) what hardware provenance can attest, (2) what remains software, (3) ABI impact, (4) whether to defer past 1.0.
- Review: OBS lead comments on audit-record shape.

#### Evidence
- none

### CAP-046 · Document every Capability Object type and right for the Layer 1 reference
- Type: docs
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: CAP-036, CAP-031, DOC-010
- Baseline: §7, §66
- Invariants: I-056

V3 exit requires reference pages for every Layer 1 entry point; the 1.0 definition requires every Capability right documented. CAP authors the semantics via the object-rights registry; DOC generates pages.

Required by V3-G12 (Layer 1 ABI reference pages exist for every entry point).

#### Out of scope
Page generation and site (DOC-023, DOC-010). ABI entry-point list (ABI-046).

#### Acceptance criteria
- [ ] Every object type in the rights registry has a reference page naming each right, Admin, and transfer bit.
- [ ] Every Capability Operation (mint, derive, transfer, revoke, inspect) has a page stating errors including `Error::Rights`.
- [ ] A CI check fails when a registry right lacks a page.

#### Verification
- Review: DOC lead sign-off that pages ingest into DOC-023.
- Manual: coverage checklist of registry entries versus published pages.

#### Evidence
- none

### CAP-047 · Decide how Capability unforgeability survives machine boundaries
- Type: adr
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: CAP-008, CAP-021, IPC-056
- Baseline: §7, §43, §57
- Decision: D-0053
- Invariants: I-047

Answers the §43 question (cryptographic Capabilities versus sturdy references versus proxies) so the ABI does not foreclose later remote transports. Distributed interfaces themselves remain LATER and outside 1.0; this decision only preserves unforgeability.

<!-- covers: INV-0817 -->

#### Out of scope
Remote-machine transport prototype (IPC-071). VM transport implementation (IPC, VIRT). Making the kernel distributed (forbidden, §57).

#### Acceptance criteria
- [ ] Options evaluated include at least: cryptographic Capabilities; sturdy references; proxy objects on the sending machine.
- [ ] The accepted option states what V3 L1 handles must remain so a later transport can honour attenuation and revoke.
- [ ] The accepted option does not make remote delivery a kernel concern.
- [ ] Review records ABI and IPC lead sign-off on the pull request.

#### Verification
- Review: ABI lead and IPC lead sign-off recorded on the pull request.
- Manual: decision file lists at least two options and names Q-036 as the question it answers.

#### Evidence
- none

### CAP-048 · Implement one-time and time-bounded Capability grants with per-application history
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: CAP-037, CAP-020, SEC-007, CAP-001
- Baseline: §7, §9.1
- Threats: T-001

V3 permissions UI v2 includes per-application audit history and one-time grants. CAP implements one-time and time-bounded grants so converting a persistent grant to one-time is a kernel-visible state change, not only a UI flag.

#### Out of scope
Permissions UI v2 (SEC-062). Usage-history aggregation (OBS-048). Grant taxonomy Decision (SEC-007).

#### Acceptance criteria
- [ ] A one-time grant is unusable after a single successful Operation; the second Operation returns `Error::Rights`.
- [ ] A time-bounded grant returns `Error::Rights` after its bound; `os inspect capability` shows remaining bound while live.
- [ ] Converting a persistent grant to one-time is recorded in the audit log and the handle's inspect record.
- [ ] Per-application grant history lists each grant, its class, and revoke or expiry, queryable by the APP UI.

#### Verification
- Unit: `runtime:tests/cap/one_time_*` on `qemu-x86_64`.
- Integration: convert-grant-to-one-time suite on H-002.
- Review: SEC and APP leads sign off that UI v2 consumes this state.

#### Evidence
- none

### CAP-049 · Isolate persistent grant stores per user session
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: CAP-037, SEC-042, SEC-028
- Baseline: §7, §9.1, §63
- Threats: T-026

V3 exit: two users with separate sessions, separate Capability stores and separate encrypted data. CAP isolates the persistent grant store so one session cannot resolve the other's grants.

#### Out of scope
Session objects and switching (SEC-060, SEC-064). Encrypted home keys (SEC). UI switcher (APP-063).

#### Acceptance criteria
- [ ] User A's persisted grants are not in user B's store; B's lookup of A's handle returns `Error::Rights` and allocates no handle.
- [ ] After session switch, A still sees A's grants and not B's, on `qemu-x86_64`.
- [ ] `os inspect` in A's session lists only A's store.
- [ ] A planted cross-store index probe in CI fails closed.

#### Verification
- Integration: two-session grant isolation suite on H-002.
- Unit: `runtime:tests/cap/per_user_store_*`.
- Review: SEC lead sign-off that store identity is the Session object from SEC-028.

#### Evidence
- none

### CAP-050 · Close external audit findings in Capability enforcement
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: CAP-031, CAP-005, CAP-004, CAP-019, CAP-042
- Baseline: §7, §51, §63
- Risks: R-055
- Threats: T-003, T-004

V4 exit: external security audit of kernel Capability enforcement with all High and Critical findings fixed and re-verified. Each fix lands with a regression test in the property or conformance suite.

#### Out of scope
The audit engagement itself (SEC, GOV). IPC and personality findings (IPC-066, LNX, WIN). Freeze ADR (ABI-049).

#### Acceptance criteria
- [ ] Every High and Critical finding against Capability enforcement has a fix in tree and a regression test that fails without the fix.
- [ ] Re-verification records that those tests pass on H-001 and on every in-scope V4 H-ID used by the audit.
- [ ] Open Medium findings are listed with a named follow-up task or an accepted Decision to accept the risk.

#### Verification
- Unit: new regression tests under `kernel:tests/cap/audit_fix_*`.
- Review: independent verifier reruns the finding tests; sign-off is not the Owner.
- Manual: finding-by-finding checklist attached to the pull request.

#### Evidence
- none

### CAP-051 · Complete conformance tests for every frozen Capability entry point
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: CAP-031, CAP-052, ABI-049, CAP-036, CAP-010, CAP-012, CAP-014, CAP-007
- Baseline: §7, §65, §66
- Freezes: S-003
- Invariants: I-040

V4 freezes L1. This suite is the conformance tests for every frozen Capability entry point, and it is the freeze of S-003. Binaries built against the freeze candidate run on every subsequent build. Spike and decision are in the dependency closure (CAP-012, CAP-010, CAP-052, ABI-049).

#### Out of scope
Accepting the freeze ADR (ABI-049). Golden binary suite across all L1 (ABI-047). Docs (CAP-046).

#### Acceptance criteria
- [ ] Every frozen Capability entry point (mint, derive, transfer, revoke, inspect, rights check) has a conformance test that passes on `qemu-x86_64` and H-002.
- [ ] A binary built against the freeze candidate runs those tests on a later V4 build without rebuild.
- [ ] S-003 state becomes `frozen` when this task is done; CI fails if S-003 is frozen while any listed dependency is not done.
- [ ] Type-mismatch and forged-handle cases remain in the suite.

#### Verification
- Unit: `kernel:tests/cap/conformance_v4_*` on `qemu-x86_64` and `hw-h002`.
- Integration: freeze-candidate binary replay on a subsequent image.
- Review: ABI lead sign-off that S-003 freeze cites this task, the exploring spike, and the rights-encoding ADR.

#### Evidence
- none

### CAP-052 · Re-validate the frozen Capability ABI on CHERI emulator and formal model
- Type: spike
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: CAP-038, CAP-035, CAP-031, CAP-034
- Baseline: §8, §65, §66
- Explores: S-001, S-003
- Risks: R-054
- Invariants: I-040, I-058

V4 freezes L1. GAP-0487 and GAP-0488 require the frozen ABI to be re-validated on a CHERI emulator and against the formal model before the freeze ADR is accepted. This spike precedes ABI-049.

<!-- covers: GAP-0487, GAP-0488 -->

#### Out of scope
Accepting the freeze (ABI-049). Conformance suite (CAP-051). First V1 CHERI run (CAP-038).

#### Acceptance criteria
- [ ] QEMU-CHERI run of mint, derive, transfer, revoke and forge-reject matches `qemu-x86_64` error codes for the freeze-candidate ABI.
- [ ] Formal model check of the freeze-candidate tree passes in CI.
- [ ] Report lists any mismatch with the V1 CHERI validation and whether it blocks freeze.

#### Verification
- Report: answers (1) which frozen operations pass on QEMU-CHERI, (2) whether the formal model still holds, (3) regressions versus the V1 validation, (4) go or no-go input to ABI-049.
- Integration: QEMU-CHERI plus `kernel:formal/cap` on the freeze-candidate commit.
- Review: ABI lead records that ABI-049 cites this report.

#### Evidence
- none

### CAP-053 · Publish the Capability model guarantees and non-promises for 1.0
- Type: docs
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: ABI-050, ABI-054, CAP-051, CAP-021
- Baseline: §7, §8, §65, §66
- Invariants: I-058, I-040

1.0 ABI stability statement and explicit non-promises. The Capability section states what is frozen (unforgeable typed attenuable revocable handles, S-003) and that CHERI-class hardware enforcement is not promised, only ABI room.

#### Out of scope
Whole-ABI stability statement (ABI-050, ABI-053). Fossilization review body (ABI-054).

#### Acceptance criteria
- [ ] Published Capability section lists frozen operations and rights, and names S-003 as frozen.
- [ ] Published non-promises include that hardware capability enforcement is not a 1.0 guarantee.
- [ ] Section cites CAP-021 for the reserved ABI room and ABI-054 for future-hardware review.
- [ ] No performance number appears in the section; any claim cites a B-ID.

#### Verification
- Review: ABI lead and DOC lead sign-off recorded on the pull request.
- Manual: checklist against the 1.0 definition item on ABI stability and the CHERI non-promise.

#### Evidence
- none
