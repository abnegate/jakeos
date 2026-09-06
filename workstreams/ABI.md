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
- Depends on: ABI-019, Q-001, BEN-005, BLD-082
- Baseline: §65, §54
- Benchmarks: B-009

ABI-008 must choose the Layer 1 entry mechanism from measured cost, not taste (§65, §54). This benchmark runs the three ABI-019 prototypes (syscall-per-Operation, shared submission page with doorbell, vDSO-style trampoline) on a no-op Operation and publishes entry-plus-return cost per mechanism as B-009, using the BEN-005 runner and the BEN-064 methodology (warm and cold runs, percentiles, CPU pinning, mitigation state recorded). The comparison baselines named on B-009 are the Linux `syscall` path (`getpid`) and io_uring `NOP` submit-to-completion, measured on the same machine in the same session; native software never sees those interfaces, they are baselines only.

The harness is `bench/harness/B-009/` (crate `jakeos-bench-entry-cost`) with one scenario per mechanism and per baseline, each producing the BEN-005 time-series record. Reports are written by hand from the records into `reports/benchmarks/B-009/h001.md` and `h002.md` in the roadmap repository following the BEN-005 report skeleton; V0 is publish-only (D-0031), so the reports state numbers and method and no pass or fail.

<!-- covers: GAP-0500 -->

#### Out of scope
Standing Operation latency publication after the Decision (BEN). io_uring lineage evaluation (TSK-014). The prototypes themselves (ABI-019). Methodology (BEN-064).

#### Deliverables
- bench:harness/B-009/ · Crate `jakeos-bench-entry-cost`: scenarios `syscall-per-op`, `shared-page`, `trampoline`, `baseline-syscall`, `baseline-io-uring-nop`.
- bench:harness/B-009/README.md · How the harness is run per BEN-064, which kernel configuration and prototype build it needs.
- roadmap:reports/benchmarks/B-009/h001.md · The H-001 report (functional coverage only, labelled as QEMU).
- roadmap:reports/benchmarks/B-009/h002.md · The H-002 report that ABI-008 cites.

#### Acceptance criteria
- [ ] `reports/benchmarks/B-009/h001.md` and `h002.md` exist, follow the BEN-005 skeleton, and cover syscall-per-Operation, shared submission page and trampoline entry plus the two baselines.
- [ ] Each report names the B-009 method, the BEN-064 methodology revision, the Linux `syscall` and io_uring `NOP` baselines, the mitigation state and the mechanism under test, and asserts no threshold (V0 publish).
- [ ] D-0005 (ABI-008) cites `reports/benchmarks/B-009/h002.md` in its Evidence lines.
- [ ] `bench/harness/B-009/` runs from the BLD-010 nightly job and writes a BEN-005 time-series record per scenario.

#### Verification
- Bench: B-009 on H-001 and H-002; target per register (V0 publish).
- Unit: `bench:tests/B-009/scenarios_*` asserting each scenario emits a well-formed record.
- Review: BEN lead confirms the reports follow the B-009 method recorded in the register.

#### Evidence
- none

### ABI-002 · Implement the Native ABI entry layer in the Linux-derived kernel
- Type: build
- Milestone: V0
- Status: todo
- Size: L
- Owner: none
- Depends on: ABI-008, ABI-012, ABI-014, ABI-015, LNX-001, KRN-013
- Baseline: §6, §7, §65
- Risks: R-004

The entry layer is the one door native software has into the kernel (§6 phase A, §65): the mechanism D-0005 (ABI-008) chose, the object-Operation dispatch D-0012 (ABI-012) chose, the Operation kind set of D-0014 (ABI-014) and the Operation identity of D-0015 (ABI-015). It lives in `jakeos/abi/` (KRN-013 layout): `entry.rs` implements the chosen mechanism (for a syscall mechanism, one architecture-neutral `jakeos_enter` syscall number registered in `arch/x86/entry/syscalls/syscall_64.tbl` behind `CONFIG_JAKEOS`; for a shared page, the doorbell syscall and page setup), `dispatch.rs` resolves `(handle word, Operation kind)` through the CAP-005 table and the ABI-005 type registry to the target object's handler, and `kinds.rs` holds the kind table with the reserved slots. The user-visible constants are generated into `include/uapi/linux/jakeos/entry.h` from the ABI-017 specification once it exists; until then the header is hand-maintained with a `// TODO(ABI-017)` marker that the KRN-016 lint tolerates only in this file.

The retained Linux syscall table stays intact and untouched for the L0 corpus: `jakeos_enter` is an addition, not a replacement, and the dispatcher never routes a Linux syscall. `os inspect abi` shows the entry-layer identity (mechanism name and ABI version) through the OBS-006 provider. `unsafe` is confined to `entry.rs` and the files the pull request lists.

<!-- covers: INV-1321 -->

#### Out of scope
Linux syscall implementation (LNX). Operation ring internals (TSK-024). Capability table storage (CAP-005). Object type registry (ABI-005). Header generation (ABI-017, SDK).

#### Deliverables
- kernel:jakeos/abi/abi.rs · Crate root for the ABI area.
- kernel:jakeos/abi/entry.rs · The D-0005 entry mechanism; the only `unsafe` in the area outside listed files.
- kernel:jakeos/abi/dispatch.rs · Handle-word plus Operation-kind resolution to object handlers through the CAP-005 table and the ABI-005 registry.
- kernel:jakeos/abi/kinds.rs · The Operation kind table with D-0014's reserved slots.
- kernel:include/uapi/linux/jakeos/entry.h · User-visible entry constants (hand-maintained until ABI-017 generates it).
- kernel:arch/x86/entry/syscalls/syscall_64.tbl · The `jakeos_enter` entry behind `CONFIG_JAKEOS`, if D-0005 chose a syscall mechanism.
- kernel:jakeos/obs/providers/abi.rs · The `os inspect abi` provider: mechanism name and ABI version.
- kernel:tools/testing/selftests/jakeos/abi/entry_*.rs · Selftests: no-op Operation round trip, Linux syscall path untouched, inspect identity.

#### Acceptance criteria
- [ ] A native Component on `qemu-x86_64` and `hw-h002` submits a no-op Operation through `jakeos/abi/entry.rs` and observes its completion without invoking any Linux syscall number.
- [ ] The Linux syscall table used by C-001 boots and passes unchanged, and `dispatch.rs` has no path that handles a Linux syscall.
- [ ] `os inspect abi` on a live native Component prints the mechanism name chosen by D-0005 and the ABI version.
- [ ] No `unsafe` outside `jakeos/abi/entry.rs` and the files listed in the pull request description; the BLD-011 unsafe inventory shows only those.

#### Verification
- Unit: `kernel:tests/abi/entry_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Integration: V0 primitive suite reaches the kernel only through this layer on H-001 and H-002 (the BLD-006 agent run with `strace`-equivalent tracing showing no Linux syscalls from native Components).
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

The §3 firewall is enforced by a lint from V0, before the first native crate exists: native software never links the Linux personality, `libc`, or a Wasm runtime used as the machine ABI (§57, I-005, I-006, I-046). `tools/firewall-lint/` (crate `jakeos-tools-firewall-lint`) reads `cargo metadata` for the workspace and fails when a crate outside `personality-linux/` and `personality-windows/` depends, directly or transitively, on a crate in `build/lints/firewall-denylist.toml` (`libc`, `nix`, `rustix` in its libc backend, the personality crates, `wasmtime` and `wasmer` when used as the process ABI rather than as a hosted runtime Component) unless the crate is on the allowlist in the same file with the decision that permits it. The only C runtime a native crate may link is the Layer 3 library D-0351 (SDK-097) decides, named there by crate name.

The lint runs as the `abi-firewall` job in `pre-merge` (BLD-011 makes it required). The sample crate `sdk/examples/image-decoder/` (SDK-001) is the positive fixture; `tools/firewall-lint/fixtures/` holds a crate that depends on `libc` and one that links the personality, both of which must fail.

<!-- covers: INV-0867, INV-0010, INV-0011, INV-1121, INV-1123, INV-0272, INV-1130 -->

#### Out of scope
POSIX-shaped name lint on ABI headers (ABI-018). Personality opt-in (LNX-016). Wasm runtime as machine ABI decision (WASM-001). The C-library strategy (SDK-097).

#### Deliverables
- tools:firewall-lint/ · Crate `jakeos-tools-firewall-lint`: dependency-graph walk, denylist and allowlist evaluation, human-readable violation report naming the dependency path.
- bld:lints/firewall-denylist.toml · The denied crates and the allowlisted exceptions with their decision IDs.
- tools:firewall-lint/fixtures/ · A crate depending on `libc` and a crate depending on the personality, both expected to fail.
- platform:.github/workflows/pre-merge.yml · The `abi-firewall` job.

#### Acceptance criteria
- [ ] A native crate whose dependency graph reaches `libc` or the Linux personality crate fails the `abi-firewall` job on `qemu-x86_64`, and the failure names the dependency path.
- [ ] A native crate that depends on a Wasm runtime as its process ABI fails the same lint; a Wasm host Component listed in the allowlist with its decision passes.
- [ ] The ImageDecoder sample crate (SDK-001) passes the lint, and both fixture crates fail it.
- [ ] `firewall-denylist.toml` names the SDK-097 Layer 3 C library as the only permitted C runtime once D-0351 is accepted, and lists no other exception without a decision ID.

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

The Layer 1 handshake identifies ABI version and features at kernel entry (§65 rule 6), using the scheme D-0016 (ABI-016) chose: a version word, feature bits, or both, exchanged on the Component's first entry. Kernel side, `jakeos/abi/handshake.rs` validates the Component's advertised version and feature request against the kernel's, records the negotiated set on the Component object, and refuses an Operation from a Component that has not completed the handshake with the typed error D-0006 (ABI-009) names for it, allocating no handle. Runtime side, `runtime/abi/src/handshake.rs` in crate `jakeos-runtime-abi` performs the handshake before the first Operation and exposes the negotiated feature set to the SDK.

The compatibility test is the first ABI conformance case, `abi/conformance/cases/000_handshake.rs`, and is never removed: a Component built against ABI v0 completes the handshake with a kernel that advertises an additional optional field or feature, and a Component that omits a field the kernel knows completes it with a newer kernel; in both directions the following no-op Operation succeeds. The case runs in `pre-merge` on every merge to `main` of both repositories.

<!-- covers: INV-1274, INV-0147 -->

#### Out of scope
IDL unknown-field tests (IPC-019). Layer 2 interface version negotiation (IPC). The scheme decision (ABI-016). The conformance suite runner beyond case 0 (ABI-027).

#### Deliverables
- kernel:jakeos/abi/handshake.rs · Kernel-side negotiation, recorded on the Component object, refusal path with the typed error.
- runtime:abi/src/handshake.rs · Runtime-side handshake in `jakeos-runtime-abi`, run before the first Operation.
- abi:conformance/cases/000_handshake.rs · Conformance case 0: forward and backward compatibility in both directions.
- abi:conformance/README.md · The conformance suite skeleton: case numbering, where cases live, that case 0 is permanent.
- kernel:tools/testing/selftests/jakeos/abi/handshake_*.rs · Selftests for the kernel side, including the refusal path.

#### Acceptance criteria
- [ ] A native Component built against ABI v0 completes the handshake with a kernel that advertises a newer optional field or feature, and its following no-op Operation succeeds, on `qemu-x86_64` and `hw-h002`.
- [ ] A native Component that omits a field the kernel knows completes the handshake with the newer kernel, and its following Operation succeeds.
- [ ] An Operation before the handshake, or a handshake with an incompatible version, returns the typed error named by D-0006 and allocates no handle.
- [ ] `abi/conformance/cases/000_handshake.rs` exists, is listed as case 0 in `abi/conformance/README.md`, and runs in `pre-merge` on both repositories.

#### Verification
- Unit: `kernel:tests/abi/handshake_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Integration: case 0 retained as the first ABI conformance case on every merge to `main`.
- Demo: handshake success and unknown-field acceptance shown in `os inspect abi` on H-002.

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

Every native kernel object is an `Object<T>` with a type identifier checked on every Operation (§7); user space holds only a Capability to it. `jakeos/abi/object.rs` defines the registry: a `TypeId` (a `u16` allocated from `jakeos/abi/types.rs`, which lists every V0 type: `Component`, `Task`, `TaskGroup`, `Channel`, `Operation`, `MemoryObject`, `ResourceDomain`, `Event`, `Timer`), the `ObjectHeader` every kernel object embeds (type id, generation, owning Component, reference count, live handle count), and `register_type::<T>()` that binds a `TypeId` to the handler table `dispatch.rs` calls. Which types are kernel-resident follows D-0013 (ABI-013); how the type tag sits in the handle word follows D-0007 (ABI-010).

Dispatch checks the type identifier from the CAP-005 table entry against the handler's expected type before any object code runs; a forged handle (no table entry, wrong generation) or a wrong-type handle returns the typed error D-0006 names (`Error::Rights` or its equivalent) and allocates no handle. The inspect provider `jakeos/obs/providers/object.rs` prints type, owner and live handle count for every kind. The fuzz target `tools/jakeos/fuzz/abi_handle/` (the roadmap path `kernel:fuzz/abi_handle`) drives random handle words and kinds through dispatch for an hour nightly without a panic.

<!-- covers: INV-0049, INV-0158 -->

#### Out of scope
Capability rights encoding and derivation (CAP-003, CAP-010). Which types live in kernel versus user services (ABI-013). Channel endpoints (IPC-008). Handle-word packing (ABI-010).

#### Deliverables
- kernel:jakeos/abi/object.rs · `ObjectHeader`, `TypeId`, `register_type`, the handler table dispatch consults.
- kernel:jakeos/abi/types.rs · The V0 type identifier table with a reserved range for later kinds.
- kernel:jakeos/obs/providers/object.rs · The `os inspect object` provider.
- kernel:tools/jakeos/fuzz/abi_handle/ · Fuzz target over handle words and kinds (`kernel:fuzz/abi_handle`).
- kernel:tools/testing/selftests/jakeos/abi/object_registry_*.rs · Selftests and property tests for unforgeability and type mismatch.
- kernel:Documentation/jakeos/abi/objects.md · The registry design: header layout, type id allocation, how an area registers a type.

#### Acceptance criteria
- [ ] Operating on a forged handle returns `Error::Rights` (or the equivalent named by D-0006) and allocates no handle, on `qemu-x86_64` and `hw-h002`.
- [ ] Operating on a live handle with the wrong type identifier returns the typed error and the target object's handler is never entered (asserted by a handler-entry counter).
- [ ] `os inspect object` prints type identifier, owning Component and live handle count for every V0 type in `types.rs`.
- [ ] Property tests over random handle words and type ids on both matrix entries find no path that reaches a handler without a matching type, and `kernel:fuzz/abi_handle` runs one hour nightly without a panic.

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

The §65 rules become one checklist that every Layer 1 change is measured against, applied by CI where a rule is mechanical and by the reviewer where it is not. The checklist `abi/review/checklist.md` in the platform monorepo lists each rule with its mechanical test: a Layer 1 entry must cite an accepted decision (the pull request body carries `Decision: D-NNNN` and the file is `accepted` in the roadmap at the pinned commit); a native header must not name `task_struct`, `mm_struct`, cgroups, namespaces or page-table layout (regex list in `abi/review/forbidden-internals.txt`); an entry justified only by a POSIX, Linux or Windows equivalent is rejected (the reviewer's item, with the ABI-018 lint as the mechanical half); x86-64-only assumptions are flagged (`#[cfg(target_arch)]` in Layer 1 code without an architecture-neutral definition fails); every change carries a deprecation and compatibility-shim plan section; the surface stays Capability-based, Component-oriented, asynchronous and object-based (reviewer items with examples). No Layer 1 surface is frozen in V0 and the checklist says so.

`tools/abi-review-gate/` (crate `jakeos-tools-abi-review-gate`) runs the mechanical items as the `abi-review-gate` job in `pre-merge` of both repositories, for changes touching `jakeos/abi/`, `include/uapi/linux/jakeos/` or `abi/`; the RFC template (GOV-005) embeds the reviewer items verbatim.

<!-- covers: INV-0081, INV-0103, INV-0034, INV-0003, INV-1277, INV-1275, INV-0708, INV-0702, INV-0198, INV-1276, INV-1305, INV-1283, INV-1268 -->

#### Out of scope
Mechanical snapshot diff (ABI-027). RFC process text (GOV-005). Capability rights review (CAP). POSIX-name lint content (ABI-018).

#### Deliverables
- abi:review/checklist.md · The §65 rules, each marked mechanical (with its job) or reviewer (with its examples), and the no-freeze-in-V0 statement.
- abi:review/forbidden-internals.txt · The kernel-internal names a native header may not contain.
- tools:abi-review-gate/ · Crate `jakeos-tools-abi-review-gate`: decision-link check against the roadmap pin, forbidden-internals scan, `target_arch` scan, plan-section presence.
- platform:.github/workflows/pre-merge.yml · The `abi-review-gate` job for changes under `abi/`.
- kernel:.github/workflows/pre-merge.yml · The `abi-review-gate` job for changes under `jakeos/abi/` and `include/uapi/linux/jakeos/`.
- docs:rfc/template.md · The RFC template section that embeds the reviewer items (co-owned with GOV-005).

#### Acceptance criteria
- [ ] A pull request that adds a Layer 1 entry whose only justification is a POSIX, Linux or Windows equivalent fails review against `checklist.md`, and one whose body lacks an accepted `Decision: D-NNNN` fails the `abi-review-gate` job.
- [ ] A pull request that names `task_struct`, `mm_struct`, cgroups, namespaces or page-table layout in a native header, or uses `#[cfg(target_arch)]` in Layer 1 code without an architecture-neutral definition, fails the `abi-review-gate` job.
- [ ] A Layer 1 change without a deprecation and compatibility-shim plan section in its pull request body fails the job.
- [ ] `checklist.md` is consumed by both CI jobs and embedded in `docs:rfc/template.md`, and states that no Layer 1 surface is frozen in V0.

#### Verification
- Unit: `tools:tests/abi_review_gate_*` on CI matrix entry `qemu-x86_64`, one fixture per failing class.
- Integration: pre-merge job on a fixture pull request for each failing class in both repositories.
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

Languages other than Rust must reach Layer 1 without a second Native ABI (§50, I-002). This decision fixes the binding substrate in D-0003: a stable C-compatible header (`abi/include/jakeos/abi.h`, generated from the ABI-017 specification) plus IDL-generated per-language stubs for Layer 2; an IDL-only substrate with no C header; or a Rust-native ABI with other languages bound through Rust. The agent executing this task writes the Options with consequences drawn from ABI-011's layer placement, records the Decision as a rule ("every language reaches Layer 1 through ..."), lists rejected options with the deciding reason, and names follow-ups (the header generator task in SDK, the IDL backend rule in IPC-047).

Acceptance is by review: ABI and SDK leads sign off on the pull request, the decision file's Status becomes `accepted`, and the task is completed with `roadmap done ABI-007 --evidence decision:D-0003 --verified-by <handle>`.

<!-- covers: INV-0945, INV-0944 -->

#### Out of scope
IDL language choice (IPC-018). Rust SDK crate (SDK-003). C wrapper safety (SDK). The header generator itself (SDK, after this decision).

#### Deliverables
- roadmap:decisions/D-0003-decide-binding-substrate.md · Options, Decision, Consequences, Rejected options, Follow-ups filled in; Status `accepted` or `rejected`.

#### Acceptance criteria
- [ ] D-0003 evaluates C-compatible header plus IDL stubs, IDL-only, and Rust-native ABI as named options with consequences.
- [ ] The accepted option states how a non-Rust language reaches Layer 1 without a second Native ABI, and names the header path and the stub generator it relies on.
- [ ] Review records ABI lead and SDK lead sign-off on the pull request, and Evidence carries `decision:D-0003`.

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

How a Component enters the kernel, and how many entry points Layer 1 may ever have, is §65 rule 1. D-0005 chooses among the three ABI-019 prototypes (syscall instruction per Operation, shared submission page with rare doorbell syscalls, vDSO-style trampolines) using the B-009 reports from ABI-001 as Evidence, and records a hard maximum count of kernel entry points that the ABI-006 review gate enforces from then on. Surface S-002 (the entry surface) is listed by the decision and moves to `prototyped`; nothing is frozen.

The executing agent fills the Options from the ABI-019 report (each mechanism's entry-point count, async-only preservation, tagged-memory consequences), cites `reports/benchmarks/B-009/h002.md` in Evidence, writes the Decision as a rule with the entry-point bound as a number in the decision file (the one place a bound may appear), lists rejected options with reasons, and updates `registers/surfaces.md` so S-002's `Decided by` names ABI-008 and its State is `prototyped`.

<!-- covers: GAP-0500, INV-1269, INV-1279 -->

#### Out of scope
Operation ring layout (TSK-007). Implementation of the chosen mechanism (ABI-002). Layer 1 freeze (ABI-049). The measurements (ABI-001).

#### Deliverables
- roadmap:decisions/D-0005-decide-entry-mechanism.md · Options from the ABI-019 report, Decision with the entry-point bound, Evidence citing the B-009 H-002 report, rejected options, follow-ups.
- roadmap:registers/surfaces.md · S-002 `Decided by: ABI-008`, `State: prototyped`.

#### Acceptance criteria
- [ ] D-0005 evaluates syscall-per-Operation, shared submission page with rare doorbell syscalls, and trampoline entry as named options, each with the ABI-019 findings on entry-point count and async-only preservation.
- [ ] The accepted option records the maximum count of kernel entry points and lists the rejected options with the deciding reason for each.
- [ ] D-0005 lists S-002 in `Surfaces`, cites `reports/benchmarks/B-009/h002.md` in Evidence, and S-002's register entry names ABI-008 under `Decided by`.
- [ ] S-002 is `prototyped`, not `frozen`, after the task is done.

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

Every Operation completes with a result, and the failure encoding must be stable across the ABI so that forged handles, denied derivation, deadline expiry, cancellation and exhaustion mean the same thing everywhere (§19). D-0006 chooses among a typed error enum per Operation kind, a uniform error object shared by every kind, and a hybrid of a uniform error class plus a per-kind payload, using the ABI-020 prototype report and the Zircon study (ABI-022) as Evidence. Native errors are not errno values; the GLOSSARY's provisional vocabulary (`Error::Rights`, `Exhausted`, `Cancelled`, `DeadlineExceeded`, `Revoked`, `Disconnected`, `Integrity`) is what the accepted option must name an encoding for. Surface S-004 (the error surface) becomes `prototyped`.

The executing agent writes each option's consequences for the SDK's `Result` type, for IDL transport of errors across a Channel and for `os inspect`, records the Decision as the error vocabulary and its encoding, updates GLOSSARY.md's "Error vocabulary" entry from provisional to decided, and sets S-004's register entry.

<!-- covers: INV-0375, INV-1279 -->

#### Out of scope
Capability denial audit log (CAP-001). Operation completion delivery (TSK-007). Personality errno translation (LNX).

#### Deliverables
- roadmap:decisions/D-0006-decide-error-model.md · Options, Decision naming the encoding for each vocabulary term, Evidence citing `reports/spikes/ABI-020.md`, rejected options, follow-ups.
- roadmap:registers/surfaces.md · S-004 `Decided by: ABI-009`, `State: prototyped`.
- roadmap:GLOSSARY.md · The "Error vocabulary" entry updated from provisional to the decided encoding.

#### Acceptance criteria
- [ ] D-0006 evaluates typed enum per kind, uniform error object, and hybrid class-plus-payload as named options with consequences for the SDK `Result`, Channel transport and `os inspect`.
- [ ] The accepted option names the encoding for forged handle, wrong type, denied rights, deadline expiry, cancellation, revocation, disconnection, integrity failure and resource exhaustion, using the GLOSSARY vocabulary.
- [ ] D-0006 lists S-004, states that native errors are not errno values, and S-004 is `prototyped`, not `frozen`, after the task is done.
- [ ] GLOSSARY.md's error vocabulary entry no longer says provisional.

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

CAP-008 chooses the Capability representation and table design; this decision fixes only how that representation is packed into the 64-bit word user space passes at the Layer 1 boundary (S-001): where the type tag and generation live (inline in the word, or held in the table and looked up), how many bits stay reserved for a future sealed-pointer layout (CAP-012), and what the kernel checks at the boundary before dispatch (§7, §8). Both decisions cite the same prototypes (CAP-013) and CHERI study (CAP-012) so the two are never decided twice.

The executing agent writes at least two packings as options, each with a paragraph on CHERI and tagged memory, records the Decision as the bit layout (a table in the decision file is the one place the layout appears until ABI-017 specifies it), states that user space cannot mint a valid handle and that type tags are checked at the boundary, and sets S-001's register entry to `prototyped` with `Decided by: CAP-008, ABI-010`.

<!-- covers: INV-0186, INV-1271, INV-1276, INV-1279 -->

#### Out of scope
Rights word encoding (CAP-010, S-003). Capability table implementation (CAP-005). Hardware CHERI emulator validation (CAP-038). The representation itself (CAP-008).

#### Deliverables
- roadmap:decisions/D-0007-decide-handle-word.md · At least two packings with CHERI paragraphs, the Decision as a bit-layout table, Evidence citing `reports/spikes/CAP-013.md` and `reports/spikes/CAP-012.md`, rejected options, follow-ups.
- roadmap:registers/surfaces.md · S-001 `Decided by` adds ABI-010, `State: prototyped`.

#### Acceptance criteria
- [ ] D-0007 evaluates at least two packings of the CAP-008 representation (type tag and generation inline in the word; opaque word with the tag held in the table), each with a CHERI and tagged-memory paragraph.
- [ ] The accepted option gives the bit layout as a table, states that user space cannot mint a valid handle, and states that type tags are checked at the kernel boundary before dispatch.
- [ ] D-0007 lists S-001 and the register records S-001 as `prototyped`, not `frozen`, with ABI-010 under `Decided by`.
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

§66 defines stability layers; this decision applies them: it enumerates the Layer 1 primitives and places every public concept of the baseline in Layer 1 or Layer 2, answering Q-047 for the compositor protocol, the Package format and ResourceDomain policy. The options in D-0010 are a minimal Layer 1 (handles, entry, errors, negotiation, object type ids), a Layer 1 that also includes Channel and ResourceDomain as kernel objects, and a Layer 1 that also includes the compositor protocol and the Package format. The accepted option is a table: every V0 kernel primitive and every named public concept with its layer and its S-ID, which `registers/surfaces.md` must agree with.

The executing agent writes the table into the decision, reconciles `registers/surfaces.md` (every S-ID's `Layer` matches the table; a concept in the table without an S-ID gets one added in the same change with `State: open`), marks Q-047 answered, and states that no Layer 1 surface is frozen. ABI-017 turns the table into the specification's stability declaration.

<!-- covers: INV-1284, INV-1290 -->

#### Out of scope
Layer 2 evolution rules (IPC-002). SDK crate surface (SDK-054). Freeze of any Layer 1 surface (ABI-049).

#### Deliverables
- roadmap:decisions/D-0010-decide-layer-1-scope.md · Options, the layer table as the Decision, rejected options, follow-ups.
- roadmap:registers/surfaces.md · Every S-ID's `Layer` matches the table; new S-IDs for concepts the table names that had none.
- roadmap:registers/questions.md · Q-047 `Status: answered`.

#### Acceptance criteria
- [ ] D-0010 evaluates minimal Layer 1, Layer 1 including Channel and ResourceDomain, and Layer 1 including compositor protocol and Package format as named options.
- [ ] The accepted option is a table listing every V0 kernel primitive and every public concept (including compositor protocol, Package format and ResourceDomain policy) as Layer 1 or Layer 2 with its S-ID, and `registers/surfaces.md` agrees with it after the change.
- [ ] Q-047 is marked answered by ABI-011 and needs no second decision.
- [ ] No Layer 1 surface is recorded as `frozen`.

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

How an Operation is invoked on a typed handle is the shape of every native call (§18, §65 rules 4 and 5): no Native ABI entry blocks the caller as its primary mode except explicit wait-for-completion, and Capability and MemoryObject arguments move rather than copy. D-0012 chooses among syscall-per-operation dispatch (one entry per kind through the D-0005 mechanism), ring-indexed dispatch (the entry mechanism carries a ring slot that names handle, kind and arguments), and a hybrid that completes already-ready work inline (interacting with D-0307, TSK-005). It depends on the entry mechanism (D-0005), the handle word (D-0007), the error model (D-0006), the kind set (D-0014) and Operation identity (D-0015), and it names S-002 and S-004 as the dispatch and error surfaces it rests on, both still `prototyped`.

The executing agent writes each option's consequences for the entry-point count bound of D-0005, for the TSK-007 transport, and for the SDK's `submit` signature; records the Decision as two rules (async-only entry; move semantics for handle-bearing arguments) plus the dispatch shape; and lists rejected options with reasons. ABI-002 implements the result.

<!-- covers: INV-1279, INV-1272, INV-1273 -->

#### Out of scope
Operation ring byte layout (TSK-007). Inline-completion signalling details (TSK-005). MemoryObject map and unmap (MEM-006). Entry mechanism (ABI-008).

#### Deliverables
- roadmap:decisions/D-0012-decide-dispatch.md · Options with consequences for the entry bound, the transport and the SDK `submit` signature; the Decision as the two rules plus the dispatch shape; rejected options; follow-ups.

#### Acceptance criteria
- [ ] D-0012 evaluates syscall-per-operation dispatch, ring-indexed dispatch and hybrid-with-inline-completion as named options.
- [ ] The accepted option states that no Native ABI entry blocks the calling execution context as its primary mode except explicit wait-for-completion (I-030).
- [ ] The accepted option states that Capability and MemoryObject arguments use ownership transfer by default (I-056, I-063) and names the exception mechanism, if any, for borrowed arguments.
- [ ] D-0012 lists S-002 and S-004 as the surfaces it depends on, both still `prototyped`, and Review records ABI lead sign-off on the pull request.

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

Not every typed object is a kernel object (§65 rule 2, I-008): some are user-service objects reached through Channels. D-0013 writes the residency criterion and applies it to every V0 `Object<T>` (Component, Task, TaskGroup, Channel, Operation, MemoryObject, ResourceDomain, Event, Timer, and the V0 File and Device shapes STO-001 and HW need). Options: kernel residency only for isolation or privilege (an object is in the kernel only if user space could not enforce its semantics), kernel residency when measured cost requires it (a B-ID report shows the Channel round trip is unaffordable), and kernel residency for every typed object. The output is a table in the decision: every V0 type, kernel-resident or user-service, and the criterion it met; high-level semantics that fail the criterion are recorded as Layer 2 services rather than Layer 1 entry points.

The executing agent draws on the Zircon study (ABI-022: which types Zircon keeps in-kernel and why) and the layer placement of D-0010, writes the table, and updates `registers/surfaces.md` where a user-service object needs an L2 surface that does not yet exist.

<!-- covers: INV-0172, INV-1270 -->

#### Out of scope
User-space driver hosting (SVC-004). Channel implementation (IPC-008). ComputeDevice dispatch (HET-011). Layer placement of concepts (ABI-011).

#### Deliverables
- roadmap:decisions/D-0013-decide-kernel-residency.md · The three criteria as options, the residency table as the Decision, rejected options, follow-ups.
- roadmap:registers/surfaces.md · New L2 surface entries for user-service objects the table names that had none.

#### Acceptance criteria
- [ ] D-0013 evaluates isolation-or-privilege, measured-cost and all-objects-in-kernel as named residency criteria with consequences.
- [ ] The accepted option lists every V0 `Object<T>` as kernel-resident or user-service and cites the criterion each met, with `reports/spikes/ABI-022.md` cited for the Zircon comparison.
- [ ] High-level semantics that fail the criterion are recorded as Layer 2 services with an S-ID, not as Layer 1 entry points.
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

The Operation kind set (Read, Write, Receive, Send, Connect, Accept, Timer, Wait, GPUDispatch, DeviceOperation, StorageTransaction, §18) is an ABI stability property: whether it is a closed kernel enum, an extensible registry user-space services add to, or a closed V0 set with reserved slots decides how a new kind lands after V0 and whether that is a Layer 1 change. D-0014 chooses, states the rule for adding a kind, and records that GPUDispatch, DeviceOperation and StorageTransaction occupy reserved slots (with their numeric values) or are deferred with a reservation plan. ABI-002's `kinds.rs` implements the table.

The executing agent writes the options with consequences for the entry-point bound (D-0005), for the IDL (a kind that user space defines needs a wire form) and for the V4 freeze, and records the kind table with slot numbers in the decision as the one place they appear before ABI-017.

<!-- covers: INV-0357 -->

#### Out of scope
Implementation of each kind (TSK-011, TSK-012). ComputeDevice dispatch (HET-011). Storage durability (STO-038). The kind table code (ABI-002).

#### Deliverables
- roadmap:decisions/D-0014-decide-operation-kinds.md · Options, the Decision with the kind table and reserved slot numbers, the rule for adding a kind, rejected options, follow-ups.

#### Acceptance criteria
- [ ] D-0014 evaluates a closed kernel enum, an extensible user-service registry, and a closed V0 set with reserved slots as named options.
- [ ] The accepted option names how a new kind is added after V0 and whether that addition is a Layer 1 change subject to the ABI-006 gate.
- [ ] GPUDispatch, DeviceOperation and StorageTransaction occupy numbered reserved slots in the decision's table, or are deferred with a reservation plan that names their slots.
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

Cancel and deadline must name an in-flight Operation, and V0's cancellation tests need a stable reference (§19). D-0015 chooses how user space identifies an in-flight Operation: as a `Capability<Operation>` in the holder's table, as an index in the submission ring (TSK-014's shared-ring prototype), or as an opaque handle with a separate cancellation token. Each option states how cancel and deadline name the Operation without any blocking entry other than wait-for-completion, what happens to the identifier after completion (reuse, generation), and what `os inspect operation` prints. TSK-013 implements the chosen identity; TSK owns completion delivery.

<!-- covers: INV-0373 -->

#### Out of scope
Completion ring layout (TSK-007). Cancellation of hardware-committed work (TSK-017, Q-009). The Operation object (TSK-013).

#### Deliverables
- roadmap:decisions/D-0015-decide-operation-identity.md · Options with the TSK-014 findings, the Decision as the identity rule and its reuse and generation semantics, rejected options, follow-ups.

#### Acceptance criteria
- [ ] D-0015 evaluates Capability-to-Operation, ring index, and opaque handle plus cancellation token as named options, citing `reports/spikes/TSK-014.md`.
- [ ] The accepted option states how cancel and deadline name the in-flight Operation without a blocking entry other than wait-for-completion, and what the identifier means after completion.
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

§65 rule 6 requires that the Layer 1 handshake identify ABI version and features at kernel entry. D-0016 chooses what it negotiates: a version word at first entry, a feature bit set, or both, using the ABI-021 prototype report. Each option states how an older Component talks to a newer kernel and a newer Component to an older kernel, where the handshake sits relative to the entry mechanism (D-0005), and what a mismatch returns. S-011 (the negotiation surface) becomes `prototyped`; ABI-004 implements the scheme.

<!-- covers: INV-1274, INV-1279 -->

#### Out of scope
Layer 2 interface version negotiation (IPC-019). Implementation of the handshake (ABI-004). The entry mechanism (ABI-008).

#### Deliverables
- roadmap:decisions/D-0016-decide-negotiation.md · Options citing `reports/spikes/ABI-021.md`, the Decision as the negotiated fields and compatibility rules, rejected options, follow-ups.
- roadmap:registers/surfaces.md · S-011 `Decided by: ABI-016`, `State: prototyped`.

#### Acceptance criteria
- [ ] D-0016 evaluates version word, feature bits, and version word plus feature bits as named options.
- [ ] The accepted option states how an older Component talks to a newer kernel and how a newer Component talks to an older kernel, and what a mismatch returns.
- [ ] D-0016 lists S-011 and the register records it as `prototyped`, not `frozen`, with ABI-016 under `Decided by`.
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
- Depends on: ABI-011, ABI-008, ABI-010, ABI-009, ABI-012, ABI-013, ABI-014, ABI-015, ABI-016, ABI-007, BLD-082
- Baseline: §7, §65, §66
- Risks: R-067
- Invariants: I-040, I-055

The specification is the single source of truth for the Native ABI (§65, §66): the C header, the ABI snapshot the conformance suite diffs (ABI-027) and the first conformance cases are all generated from it. It lives in the platform monorepo as `abi/spec/v0/` (crate `jakeos-abi-spec`): machine-readable TOML files, one per surface (`entry.toml` for the D-0005 mechanism and the entry-point bound, `handles.toml` for the D-0007 word layout, `errors.toml` for the D-0006 vocabulary and encoding, `kinds.toml` for the D-0014 kind table, `objects.toml` for the D-0013 type table with type ids, `negotiation.toml` for the D-0016 fields), plus `stability.toml` carrying the D-0010 layer placement with every surface marked `prototyped`. `abi/tools/render/` (crate `jakeos-abi-render`) renders the TOML into `docs/abi/v0.md` (the human-readable normative text) and into `abi/include/jakeos/abi.h` (the C header, D-0003), and the kernel's `include/uapi/linux/jakeos/` headers are regenerated from the same source by a `pre-merge` check that fails on drift.

The document states the entry-point bound from D-0005, defines every V0 entry point, object type and error code with its numeric value, and records each Layer 1 surface S-001, S-002, S-004 and S-011 as `prototyped`. Freeze-candidate marking is ABI-038; IDL-to-docs generation for Layer 2 is DOC.

<!-- covers: INV-1280, INV-1269 -->

#### Out of scope
IDL-to-docs generation (DOC). SDK crate guide (SDK). Freeze-candidate marking (ABI-038). Snapshot diff tooling (ABI-027).

#### Deliverables
- abi:spec/v0/entry.toml · Entry mechanism, entry points and the count bound.
- abi:spec/v0/handles.toml · Handle word layout from D-0007.
- abi:spec/v0/errors.toml · Error vocabulary and encoding from D-0006.
- abi:spec/v0/kinds.toml · Operation kind table with reserved slots from D-0014.
- abi:spec/v0/objects.toml · Object types, type ids and residency from D-0013.
- abi:spec/v0/negotiation.toml · Version and feature fields from D-0016.
- abi:spec/v0/stability.toml · Layer placement from D-0010 with every surface `prototyped`.
- abi:tools/render/ · Crate `jakeos-abi-render`: TOML to `docs/abi/v0.md` and to `abi/include/jakeos/abi.h`.
- docs:abi/v0.md · The rendered normative specification.
- abi:include/jakeos/abi.h · The rendered C header.
- kernel:scripts/jakeos/check-uapi-drift.sh · Fails `pre-merge` when `include/uapi/linux/jakeos/` differs from the rendered header at `roadmap-pin`.

#### Acceptance criteria
- [ ] `abi/spec/v0/` defines every V0 Layer 1 entry point, object type (with type id), Operation kind (with slot) and error code (with value), and `docs/abi/v0.md` and `abi/include/jakeos/abi.h` are rendered from it with no hand edits (a render in CI produces no diff).
- [ ] Each Layer 1 surface S-001, S-002, S-004 and S-011 is recorded as `prototyped`, not `frozen`, in `stability.toml` and in the rendered document.
- [ ] The document states the entry-point bound from D-0005 and the kernel's `include/uapi/linux/jakeos/` headers match the rendered header, enforced by `check-uapi-drift.sh` in `pre-merge`.
- [ ] Review records ABI lead sign-off on the pull request.

#### Verification
- Unit: `abi:tests/spec/render_*`: rendering is deterministic and a TOML change without a re-render fails.
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

A Layer 1 entry must not exist because POSIX or Linux has an equivalent (§57, §65 rule 9). `tools/abi-posix-lint/` (crate `jakeos-tools-abi-posix-lint`) scans the ABI specification (`abi/spec/`), the rendered header, the kernel's `include/uapi/linux/jakeos/` and `registers/surfaces.md` for the names in `build/lints/posix-names.txt` (`open`, `read`, `write`, `close`, `fork`, `exec`, `ioctl`, `mmap`, `select`, `poll`, `socket`, `bind`, `signal`, `kill`, `pipe`, `dup` and the rest of the list) used as entry-point or object-operation names, for any literal equal to a Linux x86-64 syscall number in an entry definition, and for Wayland object names (`wl_surface`, `wl_seat`, `xdg_toplevel` and the `wl_`/`xdg_` prefixes) on a native surface. A match fails the `abi-posix-lint` job in `pre-merge` (BLD-011 makes it required) unless `build/lints/posix-names-exemptions.toml` lists the symbol with an accepted decision.

The lint is deliberately about names on Layer 1 surfaces; the linking firewall is ABI-003 and the reviewer's judgement about POSIX-shaped semantics is ABI-006.

<!-- covers: INV-1130 -->

#### Out of scope
Linking firewall against libc and the Linux personality (ABI-003). CI execution plumbing (BLD-011). Wayland bridge (LNX-004). Reviewer checklist (ABI-006).

#### Deliverables
- tools:abi-posix-lint/ · Crate `jakeos-tools-abi-posix-lint`: name scan, syscall-number scan, Wayland-name scan, exemption lookup.
- bld:lints/posix-names.txt · The forbidden names, one per line.
- bld:lints/posix-names-exemptions.toml · Exempted symbols with their decision IDs (initially empty).
- platform:.github/workflows/pre-merge.yml · The `abi-posix-lint` job.
- kernel:.github/workflows/pre-merge.yml · The same job over `include/uapi/linux/jakeos/`.
- tools:abi-posix-lint/fixtures/ · A header declaring `open` as an entry, a header embedding syscall number 2, a surface naming `wl_surface`.

#### Acceptance criteria
- [ ] A native header or spec file that declares `open`, `read`, `write`, `fork` or `ioctl` (or any name in `posix-names.txt`) as a Layer 1 entry point or object operation fails the `abi-posix-lint` job on `qemu-x86_64`.
- [ ] A native header or spec file that embeds a Linux x86-64 syscall number as a Native ABI entry value fails the same lint; a Wayland object name on a surface in `registers/surfaces.md` fails it.
- [ ] A matching symbol lands only when `posix-names-exemptions.toml` lists it with a decision that is `accepted`; the three fixtures fail.

#### Verification
- Unit: `tools:tests/abi_posix_lint_*` on CI matrix entry `qemu-x86_64` over the fixtures.
- Integration: pre-merge lint job on `abi/spec/`, the rendered header, the kernel UAPI directory and `registers/surfaces.md`.

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

ABI-008 must not be a paper decision (§65). This spike builds the three candidate entry mechanisms far enough to submit and complete a no-op Operation from a user-space process on `qemu-x86_64` and `hw-h002`: a syscall instruction per Operation (a new syscall number in the x86-64 table behind `CONFIG_JAKEOS_SPIKES`), a shared submission page mapped into the process with a doorbell syscall that the kernel polls or is woken by, and a vDSO-style trampoline where the kernel maps an entry stub whose address the process calls. The prototypes live under `jakeos/spikes/entry/` and are never built into a shipped configuration; a tiny user-space driver `abi/spikes/entry-driver/` exercises each. ABI-001 measures them.

The report `reports/spikes/ABI-019.md` answers, per mechanism: how many kernel entry points it needs (the D-0005 bound input), whether it preserves async-only entry (§65 rule 4), what breaks on a future tagged-memory or CHERI CPU (CAP-012), what a mismatch or bad argument does, and what ABI-001 must measure. Nothing is frozen; S-002 stays `open` or `prototyped`.

<!-- covers: GAP-0500 -->

#### Out of scope
The Decision (ABI-008). io_uring lineage inside TSK (TSK-014). Production entry layer (ABI-002). Measurement (ABI-001).

#### Deliverables
- kernel:jakeos/spikes/entry/syscall.rs · Syscall-per-Operation prototype behind `CONFIG_JAKEOS_SPIKES`.
- kernel:jakeos/spikes/entry/shared_page.rs · Shared submission page plus doorbell prototype.
- kernel:jakeos/spikes/entry/trampoline.rs · vDSO-style trampoline prototype.
- abi:spikes/entry-driver/ · User-space driver that submits a no-op through each prototype and checks completion.
- roadmap:reports/spikes/ABI-019.md · The report with the skeleton headings from `reports/README.md` and the answers above.

#### Acceptance criteria
- [ ] Three prototypes exist under `jakeos/spikes/entry/` behind `CONFIG_JAKEOS_SPIKES` (syscall-per-Operation, shared submission page with doorbell, trampoline entry), and none is built in a shipped configuration.
- [ ] Each prototype submits and completes a no-op Operation from `abi/spikes/entry-driver/` on `qemu-x86_64` and `hw-h002`.
- [ ] `reports/spikes/ABI-019.md` records, per mechanism, the entry-point count, async-only preservation, tagged-memory consequences and failure behaviour, states what each rules out, and recommends the options ABI-008 must evaluate.
- [ ] Surface S-002 remains `open` or `prototyped`, never `frozen`.

#### Verification
- Report: which mechanism preserves async-only entry, how many kernel entry points each needs, what breaks on a future tagged-memory CPU, and what ABI-001 must measure.
- Integration: each prototype boots and completes the no-op on `qemu-x86_64` and `hw-h002`.

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

ABI-009 must choose an error model from running code (§7, §12). This spike prototypes typed kernel-boundary errors under `jakeos/spikes/errors/`: a completion record carrying `Error::Rights`, `Error::Exhausted`, `Error::Disconnected` and `Error::DeadlineExceeded` (the GLOSSARY vocabulary) in each of the three candidate encodings (typed enum per kind, uniform error object, uniform class plus per-kind payload), returned to a user-space driver `abi/spikes/errors-driver/` on `qemu-x86_64` and `hw-h002`, and carried across a prototype Channel message to see which encodings survive transport intact. No errno value appears anywhere in the native path.

The spike also ships the negative fixture crate `abi/fixtures/errno-user/`, a native crate that matches on `errno` values, so that ABI-003's firewall lint has a failing case ready the day it lands. The report `reports/spikes/ABI-020.md` answers which encodings preserve typed errors across a Channel, what errno translation would leak into native crates, and which options ABI-009 must evaluate. S-004 stays `open` or `prototyped`.

#### Out of scope
The Decision (ABI-009). Personality errno translation (LNX). Freeze of S-004 (ABI-051).

#### Deliverables
- kernel:jakeos/spikes/errors/ · The three encodings behind `CONFIG_JAKEOS_SPIKES`, each returning the four vocabulary errors.
- abi:spikes/errors-driver/ · User-space driver that provokes each error and checks the encoding, including across a prototype Channel.
- abi:fixtures/errno-user/ · Negative fixture crate matching on `errno`, for ABI-003 to reject.
- roadmap:reports/spikes/ABI-020.md · The report.

#### Acceptance criteria
- [ ] The prototype returns `Error::Rights`, `Error::Exhausted`, `Error::Disconnected` and `Error::DeadlineExceeded` in each of the three encodings on `qemu-x86_64` and `hw-h002`, and no errno value appears in the native path.
- [ ] `abi/fixtures/errno-user/` exists and is the fixture ABI-003 rejects once that lint lands.
- [ ] `reports/spikes/ABI-020.md` records which encodings survive a Channel transport intact and which options ABI-009 must evaluate.
- [ ] Surface S-004 remains `open` or `prototyped`, never `frozen`.

#### Verification
- Report: which encodings preserve typed errors across a Channel, what errno translation would leak into native crates, and which options ABI-009 must evaluate.
- Integration: the prototype boots and the driver passes on `qemu-x86_64` and `hw-h002`.

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

ABI-016 must be informed by running code (§12, §65 rule 6). This spike prototypes the Layer 1 handshake under `jakeos/spikes/handshake/`: on a process's first entry it exchanges a version word and a feature bit set with the kernel in each of the candidate shapes (version word only, feature bits only, both), and a user-space driver `abi/spikes/handshake-driver/` built in two variants (an "older" one that omits a field and a "newer" one that adds an unknown field) runs against a kernel built in the opposite variant so both compatibility directions are exercised on `qemu-x86_64` and `hw-h002`.

The report `reports/spikes/ABI-021.md` answers where the handshake runs relative to the first Operation, what happens on a mismatch (typed error, no handle), how unknown fields are carried so an older receiver ignores them, and which options ABI-016 must evaluate. S-011 stays `open` or `prototyped`.

#### Out of scope
The Decision (ABI-016). IDL message versioning (IPC-019). Freeze of S-011 (ABI-051). The production handshake (ABI-004).

#### Deliverables
- kernel:jakeos/spikes/handshake/ · The three candidate shapes behind `CONFIG_JAKEOS_SPIKES`, with an "older" and "newer" kernel variant selected by a boot parameter.
- abi:spikes/handshake-driver/ · The driver in "older" and "newer" variants.
- roadmap:reports/spikes/ABI-021.md · The report.

#### Acceptance criteria
- [ ] The prototype handshake identifies ABI version and a feature bit on `qemu-x86_64` and `hw-h002` in each of the three shapes.
- [ ] The "older" driver completes the handshake against the "newer" kernel with an unknown field present, and the "newer" driver completes it against the "older" kernel with a field omitted; a deliberate version mismatch returns a typed error and no handle.
- [ ] `reports/spikes/ABI-021.md` records where the handshake runs, the mismatch behaviour, how unknown fields are carried, and which options ABI-016 must evaluate.
- [ ] Surface S-011 remains `open` or `prototyped`, never `frozen`.

#### Verification
- Report: where the handshake runs relative to the first Operation, what happens on a mismatch, and which options ABI-016 must evaluate.
- Integration: the prototype boots and both driver variants pass on `qemu-x86_64` and `hw-h002`.

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

Fuchsia's Zircon is the closest shipping analogue of the native object model (§58): handles with rights, VMOs, channels, FIDL and the component framework. This written study reads the Zircon kernel sources and documentation at a named revision and answers, for each concept, what the design is, why it was chosen, what it costs, and whether the native model takes or rejects the idea, mapping each conclusion onto S-001 (handle representation), S-004 (errors), S-012 (Channel wire) or a specific adr (ABI-010, ABI-012, ABI-013). It explicitly does not adopt Zircon as the Native ABI and recommends freezing nothing.

The report `reports/spikes/ABI-022.md` follows the spike skeleton and cites sources by path and revision. Ideas that fail §65 rules 7 (no exposed kernel internals) or 9 (no entry justified by an existing equivalent) are listed as rejected with the rule they fail.

<!-- covers: INV-1133 -->

#### Out of scope
seL4 capability study (CAP-015). FIDL versus other IDL choice (IPC-018). NT object manager study (ABI-031). Component framework as a supervision model (SVC).

#### Deliverables
- roadmap:reports/spikes/ABI-022.md · The study with per-concept sections, the take-or-reject mapping to surfaces and adrs, and the source citations.

#### Acceptance criteria
- [ ] `reports/spikes/ABI-022.md` describes Zircon handle tables, rights, VMOs, channels, FIDL and components with citations to a named source revision.
- [ ] The report lists ABI assumptions worth taking and worth rejecting, each mapped to S-001, S-004, S-012 or a named adr (ABI-010, ABI-012, ABI-013), and names the §65 rule any rejected idea fails.
- [ ] The report recommends freezing no Layer 1 surface and is cited by ABI-010 and ABI-013.

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
