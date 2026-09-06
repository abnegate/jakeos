# CMP · Components
- Prefix: CMP
- Lead: none
- Baseline: §10, §11, §34

<!-- roadmap:generated:begin summary -->
Tasks: 55 live, 4 done, 0 in-progress, 51 todo, 0 dropped. Ready: 4. Blocked: 47. Weighted: 5%.
<!-- roadmap:generated:end -->

## Scope

CMP owns the Component isolation primitive: `Object<Component>`, spawn, code mapping, address space construction from Packages, panic and typed exit-cause semantics, component graphs with attenuated per-child authority, the native launch path, and the creation and startup metrics B-001, B-008, B-016 and B-017. Native software is a graph of Components; each Component holds only the authority it was handed. Isolation is cheap enough to be the default (I-029). V0 prototypes `Object<Component>` over retained Linux internals without freezing S-007. V1 names freeze candidates. V4 freezes S-007 with a conformance suite.

## Out of scope

Task and TaskGroup objects and cancellation propagation (TSK). Capability table, mint, derive and revocation (CAP). ResourceDomain budgets and scheduling intent (SCH). Channel transport and IDL (IPC). MemoryObject transfer and mapping primitives (MEM). Package manifest schema, content store and SystemGenerations (PKG). Service supervision, restart policy and native init (SVC). Wasm runtime and plugin host (WASM). Linux and Windows personality process and thread mapping implementations (LNX, WIN). ABI entry layer and handle encoding (ABI). Benchmark methodology and runner (BEN). SDK crate, component entry macro and `os` CLI (SDK). Inspect and trace data plane (OBS).

## Tasks

### CMP-001 · Benchmark Component creation latency p50/p99 on QEMU and reference hardware
- Type: benchmark
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: CMP-005, BEN-005, BEN-007, Q-001
- Baseline: §10, §54, §59
- Benchmarks: B-001
- Risks: R-001, R-009
- Invariants: I-029, I-061

Measure B-001 on H-001 and H-002 through the shared BEN runner so V0-G11 can publish Component creation latency. V0 is publish-only. The same harness is reused by V1, V4 and 1.0 gates. If the published p50 exceeds the V0 advisory band in the register, V0-G11 is satisfied only through its Or adr documenting root cause and remediation.

<!-- covers: INV-1295, GAP-0489 -->

#### Out of scope
Harness runner and methodology (BEN). Fast-path implementation (CMP-034). Linux fork, clone and podman baselines (CMP-035).

#### Acceptance criteria
- [ ] B-001 reports for H-001 and H-002 exist under `reports/benchmarks/B-001/` with p50 and p99.
- [ ] The V0 target kind in the B-001 register is `publish`; no absolute threshold is asserted as a V0 gate.
- [ ] If the published p50 exceeds the V0 advisory band in the B-001 register, an accepted decision documenting root cause and remediation is recorded before V0-G11 uses its Or path.
- [ ] The harness is `bench:component-create` and is invoked from nightly CI.

#### Verification
- Bench: B-001 on H-001 and H-002; target per register.
- Review: BEN methodology sign-off recorded on the pull request.

#### Evidence
- none

### CMP-002 · Benchmark resident memory overhead per idle Component
- Type: benchmark
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: CMP-005, BEN-005, BEN-007, Q-001
- Baseline: §10, §11, §54
- Benchmarks: B-008
- Invariants: I-029, I-061

Measure B-008 so V0-G15 can publish resident kernel plus runtime memory attributable to one idle minimal Component. Component graphs with hundreds of Components are viable only if this fixed cost is bounded; the absolute target lives in the register and is first gated at V1.

<!-- covers: GAP-0491 -->

#### Out of scope
Idle Task memory (TSK). Shared code pages that reduce the number (CMP-029).

#### Acceptance criteria
- [ ] B-008 reports for H-001 and H-002 exist under `reports/benchmarks/B-008/` for idle minimal Components.
- [ ] The method creates many idle Components and divides the resident-memory delta, matching the B-008 register method.
- [ ] The V0 target kind in the B-008 register is `publish`.

#### Verification
- Bench: B-008 on H-001 and H-002; target per register.
- Review: BEN methodology sign-off recorded on the pull request.

#### Evidence
- none

### CMP-003 · Map immutable executable objects as Component Code
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: CMP-014, CMP-005, MEM-006, MEM-008
- Baseline: §10, §34
- Invariants: I-039

A Component contains Code as mapped immutable executable objects (§10). In V0 the objects come from the retained initramfs; Packages arrive at V0.5. Mappings are read-only, executable and shared across instances of the same object. W^X is enforced by MEM; CMP owns the Component-side layout.

<!-- covers: INV-0221 -->

#### Out of scope
Package object mapping (CMP-017). MemoryObject executable and sealed properties (MEM).

#### Acceptance criteria
- [ ] Creating a Component maps its Code objects read-only and executable, and a write into a Code mapping returns a typed fault.
- [ ] Two Components created from the same initramfs Code object share the same physical pages, verified by page identity.
- [ ] Destroying a Component unmaps its Code without unmapping other instances of the same object.
- [ ] `os inspect component` lists the mapped Code objects for a live Component.

#### Verification
- Unit: `kernel:tests/cmp/code_mapping_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Integration: two-instance shared-page check on H-001.

#### Evidence
- none

### CMP-004 · Implement Component exit causes, teardown and the 100k leak test
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: CMP-008, CMP-014, CMP-005, TSK-022, SCH-009
- Baseline: §10, §32, §59
- Risks: R-075
- Invariants: I-037

Typed exit causes (panic, stack overflow, OOM, cancelled, exited) are observable through the Component handle. Destroy reclaims the address space, Capability table, owned TaskGroup and ResourceDomain membership. V0-G02 is verified by a leak test that creates and destroys Components at the scale named in the criteria with no unbounded kernel-memory growth.

<!-- covers: EXTRA-004, INV-0070, INV-0046 -->

#### Out of scope
Panic policy decision (CMP-008). Supervision and restart (SVC). TaskGroup cancellation mechanics (TSK).

#### Acceptance criteria
- [ ] A panicking Component reports exit cause `panic` on its handle and does not unwind across the Native ABI.
- [ ] Stack overflow and OOM are distinct typed exit causes visible to the holder of the Component handle.
- [ ] Destroy reclaims every kernel object the Component owned; `os inspect` lists none of them afterward.
- [ ] A leak test that creates and destroys one hundred thousand Components on H-001 reports no unbounded kernel-memory growth after teardown.
- [ ] Cancelling a Component's TaskGroup yields exit cause `cancelled` and completes only after owned Tasks have terminated.

#### Verification
- Unit: `kernel:tests/cmp/exit_cause_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Integration: leak test `kernel:tests/cmp/leak_create_destroy` on H-001.
- Demo: V0-D03 on H-002.

#### Evidence
- none

### CMP-005 · Implement Component creation as one kernel Operation over Linux internals
- Type: build
- Milestone: V0
- Status: todo
- Size: L
- Owner: none
- Depends on: CMP-010, CMP-014, CMP-009, CAP-005, SCH-007, TSK-023
- Baseline: §6, §10, §53, §59
- Risks: R-001, R-010
- Invariants: I-014, I-057

Phase C initial implementation: a single native operation creates the AddressSpace, Capability set, owned TaskGroup and ResourceDomain membership by wrapping `task_struct`, `mm_struct`, namespaces and cgroups. The Native ABI does not expose those internals. Native Components start from a retained initramfs; native init is not a V0 deliverable (R-010). This is the V0-G02 create, grant, run and destroy path.

<!-- covers: INV-0145, INV-0222, INV-0226, INV-0434, INV-0997, INV-1155 -->

#### Out of scope
Native AddressSpace object (CMP-045). Native membership without namespaces (CMP-046). Package launch (CMP-027). Native init (SVC).

#### Acceptance criteria
- [ ] One native create operation returns `Capability<Component>` whose object has an AddressSpace, a Capability table, a TaskGroup and ResourceDomain membership.
- [ ] Granting a Capability at create makes that Capability usable by the Component's first instruction and by no other Component.
- [ ] Mapped Code runs; destroy reclaims the wrapper resources so `os inspect component` no longer lists the handle.
- [ ] Native crates that create a Component do not call fork, exec, clone, unshare or a container runtime.
- [ ] `os inspect component` prints state, ownership and relationships for a live Component.

#### Verification
- Unit: `kernel:tests/cmp/create_destroy_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Integration: V0-G02 create-grant-run-destroy path on H-001 and H-002.
- Review: ABI review gate confirms no Linux internals leak into the Native ABI.

#### Evidence
- none

### CMP-006 · Decide what replaces PID, parent/child, exit status and process groups
- Type: adr
- Milestone: V0
- Status: done
- Size: M
- Owner: @agent/claude
- Depends on: none
- Baseline: §1, §2, §10
- Decision: D-0062
- Invariants: I-014
- Verified by: @jakebarnby

Component replaces process as the unit of isolation. This decision fixes the native replacements for process identity, lineage, exit status and grouping that every V0 build task assumes. Options are `Object<Component>` handles only, handle plus kernel-visible lineage, and opaque identity with supervisor-held lineage.

<!-- covers: INV-0070, INV-0035, INV-0220 -->

#### Out of scope
Personality process mapping (CMP-036). Task identity (TSK).

#### Acceptance criteria
- [x] At least two options are evaluated, including handles-only and handle plus kernel-visible lineage.
- [x] The accepted option names the native replacement for PID, parent/child, exit status and process groups.
- [x] Native software has no PID, process-group or parent/child API as a result of the accepted option.
- [x] A Review line names who accepts the decision.

#### Verification
- Review: ABI lead and CMP lead sign-off recorded on the pull request.

#### Evidence
- decision:D-0062

### CMP-007 · Decide Component plus ResourceDomain as the native isolation model
- Type: adr
- Milestone: V0
- Status: done
- Size: S
- Owner: @agent/claude
- Depends on: none
- Baseline: §1, §23, §36, §53
- Decision: D-0064
- Invariants: I-019
- Verified by: @jakebarnby

Records that container-based isolation is replaced by Component plus ResourceDomain and that OCI containers live in the Linux personality. The §53 non-goals (no daemon, no namespace step, no overlay mount, no image layers) become the standing rules this decision adopts and CMP-013 enforces.

<!-- covers: INV-0079, INV-0040, INV-0990, INV-0991, INV-0992, INV-0993 -->

#### Out of scope
OCI runtime inside the Linux personality (LNX). Namespace subsumption tests (CMP-026). ResourceDomain object (SCH).

#### Acceptance criteria
- [x] At least two options are evaluated, including Component plus ResourceDomain as the only native isolation model and retaining a native container runtime.
- [x] The accepted option states that native isolation has no runtime daemon, namespace setup step, overlay mount, image layers or fork/exec creation mechanism.
- [x] OCI containers are recorded as Linux-personality compatibility, not a native primitive.
- [x] A Review line names who accepts the decision.

#### Verification
- Review: ABI lead and CMP lead sign-off recorded on the pull request.

#### Evidence
- decision:D-0064

### CMP-008 · Decide Component panic, abort and typed exit-cause semantics
- Type: adr
- Milestone: V0
- Status: done
- Size: M
- Owner: @agent/claude
- Depends on: none
- Baseline: §10, §32
- Decision: D-0066
- Risks: R-075
- Invariants: I-037
- Verified by: @jakebarnby

A Rust panic aborts only its Component. Stack overflow and OOM are typed exit causes reported to the supervisor. No unwinding crosses the Native ABI. Options are abort-only, unwind-to-component-boundary, and per-component policy. V0-D03 requires this decision.

<!-- covers: EXTRA-004 -->

#### Out of scope
Exit-cause implementation (CMP-004). Supervisor restart policy (SVC). Crash capture format (OBS).

#### Acceptance criteria
- [x] At least two options are evaluated, including abort-only and unwind-to-component-boundary.
- [x] The accepted option states that a panic does not unwind across the Native ABI.
- [x] The accepted option names typed exit causes for panic, stack overflow and OOM.
- [x] A Review line names who accepts the decision.

#### Verification
- Review: ABI lead and CMP lead sign-off recorded on the pull request.

#### Evidence
- decision:D-0066

### CMP-009 · Decide the native Component spawn primitive that replaces fork and exec
- Type: adr
- Milestone: V0
- Status: done
- Size: S
- Owner: @agent/claude
- Depends on: none
- Baseline: §2, §10, §53
- Decision: D-0068
- Invariants: I-014, I-019
- Verified by: @jakebarnby

fork has no native equivalent. This decision chooses among spawn-from-immutable-code-object, template-clone, and builder-object-then-start, and records that Unix process startup is never the native creation mechanism.

<!-- covers: INV-0074, INV-0995 -->

#### Out of scope
Wrapper implementation (CMP-005). Personality fork (LNX).

#### Acceptance criteria
- [x] At least two options are evaluated, including spawn-from-immutable-code-object and template-clone.
- [x] The accepted option is a native spawn shape that is not fork, exec, posix_spawn or clone.
- [x] The Native ABI surface for spawn is named for S-007 prototyping.
- [x] A Review line names who accepts the decision.

#### Verification
- Review: ABI lead and CMP lead sign-off recorded on the pull request.

#### Evidence
- decision:D-0068

### CMP-010 · Decide the Phase A Component implementation strategy
- Type: adr
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: CMP-015, CMP-016
- Baseline: §6, §10
- Decision: D-0069
- Risks: R-001
- Invariants: I-009, I-057

V0 exit criteria name the wrapper-versus-native Component decision as a required accepted decision. Options are thin wrapper, wrapper plus prewarmed templates, and early native object, chosen from the two V0 creation spikes. The Native ABI stays free of `task_struct`, `mm_struct`, cgroups and namespaces regardless of option.

<!-- covers: GAP-0489, INV-0145 -->

#### Out of scope
Native replacement of the wrapper (CMP-042). Fast-path tuning (CMP-034).

#### Acceptance criteria
- [ ] At least two options are evaluated, including thin wrapper and wrapper plus prewarmed templates.
- [ ] The accepted option cites `reports/spikes/CMP-016.md` and `reports/spikes/CMP-015.md`.
- [ ] The accepted option states that Linux internals remain an implementation detail, not ABI.
- [ ] A Review line names who accepts the decision.

#### Verification
- Review: ABI lead and CMP lead sign-off recorded on the pull request.
- Report: both cited spike reports exist.

#### Evidence
- none

### CMP-011 · Build the V0 Demo: Component A requests, Component B returns a MemoryObject
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: CMP-005, CMP-004, CMP-008, CAP-006, IPC-010, IPC-011, MEM-010, SDK-002
- Baseline: §16, §32, §59
- Benchmarks: B-013

The §59 demo: A sends `Channel<Request>`, B returns a result by MemoryObject ownership transfer, shown with `os trace`. The same pair runs V0-D03: B panics and A observes a typed disconnect and rebinds to a restarted B.

<!-- covers: INV-1169 -->

#### Out of scope
Channel transport (IPC). MemoryObject transfer primitive (MEM). Service supervisor (SVC). Tracing substrate (OBS).

#### Acceptance criteria
- [ ] Component A sends a typed request to Component B and reads the result from a transferred MemoryObject without copying payload bytes, verified by physical-page identity.
- [ ] `os trace` on H-002 shows the A-to-B-to-MemoryObject flow for a live run of V0-D01.
- [ ] When B panics, A observes a typed disconnect and rebinds to a new B instance that serves a subsequent request (V0-D03).
- [ ] The panic does not unwind into A.

#### Verification
- Integration: `runtime:tests/cmp/demo_request_response` on H-001 and H-002.
- Demo: V0-D01 and V0-D03 on H-002.
- Bench: B-013 on H-001 and H-002; target per register.

#### Evidence
- none

### CMP-012 · Prove Component isolation with negative tests and the isolation Demo
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: CMP-005, CMP-007, CMP-013, CAP-001, SEC-001
- Baseline: §9.1, §10, §51, §59
- Threats: T-001, T-011
- Invariants: I-014, I-021, I-049

Negative tests that a Component cannot reach memory, objects or Capabilities it was not granted, and receives a typed denial visible in the audit log (V0-D04). Native Components do not enter the Linux syscall ABI as a native API; the hard syscall filter is ABI at V1.

<!-- covers: INV-1164, INV-0952, INV-0035, INV-0040 -->

#### Out of scope
Syscall filter on native Components (ABI-035). Capability mint and derive (CAP). Ambient-authority harness (SEC).

#### Acceptance criteria
- [ ] A Component without a file Capability that attempts to open a file receives `Error::Rights` and allocates no handle (V0-D04).
- [ ] The denial is visible in the Capability audit log with Component identity and the denied object type.
- [ ] A Component cannot map another Component's memory or enumerate its Capabilities.
- [ ] Native Component entry points do not expose PID, fork, exec or a Linux syscall number.

#### Verification
- Unit: `kernel:tests/cmp/isolation_negative_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Demo: V0-D04 on H-002.
- Review: SEC threat-model citation of T-001 recorded on the pull request.

#### Evidence
- none

### CMP-013 · Add the ABI review Gate rejecting process, thread and container shapes
- Type: build
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: CMP-007, CMP-009, CMP-006, ABI-018
- Baseline: §1, §3, §53, §57
- Invariants: I-006, I-014, I-019, I-025, I-049

Standing rules become a lint, not one task per rule. CI fails when a native Component entry point exposes PID, fork/exec, process-group, namespace, overlay-mount, image-layer or daemon-dependent shapes.

<!-- covers: INV-0035, INV-0040, INV-0990, INV-0991, INV-0992, INV-0993, INV-0995 -->

#### Out of scope
POSIX-shaped name lint on ABI symbols (ABI). Personality syscall retention (LNX).

#### Acceptance criteria
- [ ] CI fails a native crate that exports a PID, fork, exec, waitpid, unshare, overlay-mount or container-runtime API.
- [ ] CI fails a native Component create path that invokes a container daemon.
- [ ] The lint allowlist is empty for native crates; exemptions require an accepted decision cited on the symbol.
- [ ] The gate runs on every merge to main.

#### Verification
- Unit: `tools:tests/cmp/lint_no_process_shapes` on the lint fixtures.
- Review: ABI review-gate checklist includes the CMP process-shape items.

#### Evidence
- none

### CMP-014 · Define Object<Component> in the native Object model
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: CMP-006, CMP-009, CMP-007, ABI-005, ABI-013
- Baseline: §7, §10, §65, §66
- Invariants: I-014, I-040

Specifies the Component object, its lifecycle states, rights and handle semantics at surface state prototyped. No Layer 1 freeze in V0 (I-040). This is the Native Platform layer entry for Components (S-007).

<!-- covers: INV-0168, INV-0046, INV-1313 -->

#### Out of scope
Capability rights encoding (CAP). Freeze-candidate review (CMP-033). L1 freeze (CMP-052).

#### Acceptance criteria
- [ ] `Object<Component>` is registered in the typed object registry with a type identifier checked on every Operation.
- [ ] Lifecycle states include at least created, running, exited and destroyed, and a wrong-type Operation returns a typed error.
- [ ] S-007 remains `open` or `prototyped`; it is not `frozen`.
- [ ] `os inspect component` can name a live Component by handle.

#### Verification
- Unit: `kernel:tests/cmp/object_component_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Review: ABI object-registry sign-off recorded on the pull request.

#### Evidence
- none

### CMP-015 · Measure the dominant costs of Component creation on the Linux wrapper
- Type: spike
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: none
- Baseline: §6, §10
- Explores: S-007
- Risks: R-001

Prototype Component creation over `task_struct`, `mm_struct` and cgroups and attribute cost to address-space setup, page-table population, Capability table and scheduler registration. The breakdown feeds CMP-010 and the V0-G11 Or path if B-001 exceeds the register advisory band.

<!-- covers: INV-0230 -->

#### Out of scope
Strategy comparison (CMP-016). Permanent B-001 harness (CMP-001).

#### Acceptance criteria
- [ ] `reports/spikes/CMP-015.md` exists with the spike skeleton headings.
- [ ] The report attributes B-001 cost on H-001 and H-002 to named stages.
- [ ] The report states which stages remain if page tables are prewarmed.

#### Verification
- Report: which of address-space setup, page-table population, Capability-table fill and scheduler registration dominates B-001 on the Linux wrapper; what share of B-001 is paid before the first instruction of the new Component runs; which costs remain if page tables are prewarmed.
- Bench: B-001 instrumentation on H-001 and H-002; target per register.

#### Evidence
- none

### CMP-016 · Benchmark wrapper, prewarmed-template and native Component strategies
- Type: spike
- Milestone: V0
- Status: todo
- Size: L
- Owner: none
- Depends on: CMP-015
- Baseline: §6, §10
- Explores: S-007
- Risks: R-001

Compare a thin wrapper, prewarmed address-space templates and a dedicated native kernel object against the B-001 band on H-001 and H-002. CMP-010 is not accepted without these numbers. Publish-only; no superiority claim.

<!-- covers: GAP-0489 -->

#### Out of scope
Accepting the implementation strategy (CMP-010). Permanent harness (CMP-001).

#### Acceptance criteria
- [ ] `reports/spikes/CMP-016.md` exists with the spike skeleton headings.
- [ ] The report publishes B-001 p50 and p99 for all three strategies on H-001 and H-002.
- [ ] The report names ABI-visible differences among the three strategies, or records that there are none.

#### Verification
- Report: how thin wrapper, prewarmed address-space templates and a dedicated native kernel object compare on B-001 on H-001 and H-002; which strategy can meet the B-001 V1 absolute target without exposing Linux internals on the Native ABI; what ABI-visible differences exist among the three strategies.
- Bench: B-001 on H-001 and H-002; target per register.

#### Evidence
- none

### CMP-017 · Map immutable Package objects directly into the Component AddressSpace
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: CMP-003, PKG-037, PKG-045
- Baseline: §10, §34, §53
- Invariants: I-039

Verified-once, shared, position-fixed Package objects are mapped without a loader stage so the Component launches immediately after mapping. PKG supplies the load map; CMP performs the mapping.

<!-- covers: INV-1015, INV-0221 -->

#### Out of scope
Load-map construction (PKG). Shared verified pages across applications (PKG-039). Linking model (SDK).

#### Acceptance criteria
- [ ] Creating a Component from a Package maps the load-map objects and returns a runnable Component with no path lookup on the launch path.
- [ ] A trace of launch shows no loader stage and no dependency resolution after mapping.
- [ ] Mapped Package objects are immutable; a write returns a typed fault.
- [ ] Two Components from the same Package generation share Code pages.

#### Verification
- Unit: `kernel:tests/cmp/package_map_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Integration: launch-path trace on H-001 showing map-then-schedule.

#### Evidence
- none

### CMP-018 · Benchmark native application cold startup
- Type: benchmark
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: BEN-005, BEN-007, BEN-016, Q-001, Q-029
- Baseline: §34, §54
- Benchmarks: B-017
- Invariants: I-061

V0.5 benchmark gate for cold startup is a distinct metric from warm startup. Content is evicted from the page cache before each run. The visible-UI boundary is Q-029. Publish-only at V0.5.

<!-- covers: INV-0642 -->

#### Out of scope
Warm startup (CMP-019). Per-app APP harness (APP). Methodology (BEN).

#### Acceptance criteria
- [ ] B-017 reports for H-001, H-002 and H-003 exist under `reports/benchmarks/B-017/` for each of the four V0.5 native applications.
- [ ] Each run drops the Package from the page cache before launch, matching the B-017 register method.
- [ ] The V0.5 target kind in the B-017 register is `publish`.

#### Verification
- Bench: B-017 on H-001, H-002 and H-003; target per register.
- Review: BEN methodology sign-off recorded on the pull request.

#### Evidence
- none

### CMP-019 · Benchmark native application warm startup from click to first presented frame
- Type: benchmark
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: BEN-005, BEN-007, BEN-016, Q-001, Q-029
- Baseline: §34, §54
- Benchmarks: B-016
- Invariants: I-042, I-061

V0.5 benchmark gate: warm startup measured for each of the four apps and published. The B-016 absolute target for Terminal and Editor is a measurement target, never a public guarantee (I-042). The same harness serves V1, V2, V4 and 1.0 gates.

#### Out of scope
Cold startup (CMP-018). Extending the harness to every shipped app (CMP-047). Visible-UI boundary decision (BEN).

#### Acceptance criteria
- [ ] B-016 reports for H-001, H-002 and H-003 exist under `reports/benchmarks/B-016/` for Terminal, File Browser, Text Editor and Image Viewer.
- [ ] The measurement boundary is the compositor's first presentation of a non-blank frame, matching Q-029.
- [ ] The V0.5 target kind in the B-016 register is `publish`; the V1 absolute is not asserted as a V0.5 gate.
- [ ] Public material produced by this task does not state the B-016 figure as a guarantee.

#### Verification
- Bench: B-016 on H-001, H-002 and H-003; target per register.
- Review: BEN methodology sign-off recorded on the pull request.

#### Evidence
- none

### CMP-020 · Verify the native isolation path needs no daemon, namespace, overlay or fork
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: CMP-027, CMP-007, CMP-013, BEN-020
- Baseline: §23, §36, §53
- Invariants: I-019

Acceptance suite over the Package-based launch path proving the §53 native isolation path (create ResourceDomain, create Component, attach Capabilities, map Package, schedule) involves no runtime daemon, namespace setup, overlay mount, image layers or fork/exec.

<!-- covers: INV-0439, INV-0990, INV-0992, INV-0993, INV-0995 -->

#### Out of scope
OCI runtime in the Linux personality (LNX). Isolation-versus-OCI benchmark (BEN).

#### Acceptance criteria
- [ ] A traced native launch performs create ResourceDomain, create Component, attach Capabilities, map Package and schedule, and no other isolation steps.
- [ ] The trace contains no dockerd, containerd, unshare, overlay mount, image-layer unpack or fork/exec.
- [ ] CI fails if a later change reintroduces any of those steps on the native path.
- [ ] The suite runs on H-001 and H-002.

#### Verification
- Integration: `runtime:tests/cmp/isolation_path_trace` on H-001 and H-002.
- Review: CMP-013 still fails the forbidden shapes.

#### Evidence
- none

### CMP-021 · Decide whether every Component owns a hardware address space
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: CMP-032
- Baseline: §10, §51
- Decision: D-0061
- Invariants: I-014

Determines whether an intra-address-space Component class exists for plugins and decoders. V0 requires accepted decisions for architectural choices; this one bounds isolation cost before applications are architected as component graphs. The spike report is the evidence. Options are hardware address space for every Component, an in-address-space class for fine-grained sandboxes, or hardware address space by default with an opt-in in-address-space class.

<!-- covers: INV-0231, GAP-0490, INV-0952 -->

#### Out of scope
Implementing the in-address-space class (CMP-038). Wasm runtime (WASM).

#### Acceptance criteria
- [ ] At least two options are evaluated, including hardware-address-space-only and an in-address-space class.
- [ ] The accepted option states whether plugins and decoders can be isolated without a hardware address space on H-002.
- [ ] The decision cites `reports/spikes/CMP-032.md`.
- [ ] A Review line names who accepts the decision.

#### Verification
- Review: ABI lead and CMP lead sign-off recorded on the pull request.
- Report: the cited spike report exists and answers the isolation-cost questions.

#### Evidence
- none

### CMP-022 · Decide static manifest graphs versus dynamic child instantiation
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: CMP-031
- Baseline: §11
- Decision: D-0063

Decides whether component graphs are declared in the Package manifest, instantiated dynamically by a parent, or both with constraints on dynamic children. Precedes CMP-024. S-019 is prototyped, not frozen.

<!-- covers: INV-0245 -->

#### Out of scope
Graph instantiation (CMP-024). Dynamic-child enforcement (CMP-023). Package manifest schema (PKG).

#### Acceptance criteria
- [ ] At least two options are evaluated, including static manifest graphs only and both static graphs and constrained dynamic children.
- [ ] The accepted option names how a parent obtains attenuated Capabilities for each child.
- [ ] The decision cites `reports/spikes/CMP-031.md`.
- [ ] A Review line names who accepts the decision.

#### Verification
- Review: ABI lead and CMP lead sign-off recorded on the pull request.
- Report: the cited spike report exists.

#### Evidence
- none

### CMP-023 · Enforce constraints on dynamically created child Components
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: CMP-022, CMP-024, SCH-009
- Baseline: §11, §23
- Invariants: I-021

Implements the constraint half of CMP-022: children receive only attenuated parent Capabilities, nest inside the parent's ResourceDomain, and are bounded by SCH kernel-object limits. Tested with a child attempting to exceed its parent's authority.

<!-- covers: INV-0245 -->

#### Out of scope
Kernel-object limits (SCH). Capability derive (CAP). Static graph wiring (CMP-024).

#### Acceptance criteria
- [ ] A dynamically created child that requests a Capability the parent does not hold receives `Error::Rights` and is not created.
- [ ] A child's ResourceDomain is nested in the parent's domain; exceeding the parent budget returns a typed exhaustion error.
- [ ] A child cannot escape the parent's TaskGroup ownership.
- [ ] `os inspect` shows parent, child, attenuated Capabilities and domain membership.

#### Verification
- Unit: `kernel:tests/cmp/dynamic_child_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Integration: authority-exceeded child on H-001.

#### Evidence
- none

### CMP-024 · Instantiate declared Component graphs with attenuated per-child authority
- Type: build
- Milestone: V0.5
- Status: todo
- Size: L
- Owner: none
- Depends on: CMP-022, CMP-025, CAP-024
- Baseline: §11, §10
- Invariants: I-021, I-029

Applications are component graphs where each child gets only the authority it requires (§11). The parent creates children per the accepted declaration model, wires Channels between Inputs and Outputs, and attenuates Capabilities per child.

<!-- covers: INV-0245 -->

#### Out of scope
Dynamic-child constraints (CMP-023). Capability lint over graphs (CAP). IDL endpoint types (IPC).

#### Acceptance criteria
- [ ] A declared two-Component graph launches with Channels wired from declared Outputs to Inputs.
- [ ] Each child holds only the Capabilities listed for it in the declaration; a sibling's Capability is absent.
- [ ] ImageDecoder in the Image Viewer graph has no network Capability and no arbitrary filesystem Capability.
- [ ] `os inspect` renders the live graph of Components, Channels and Capabilities for the application.

#### Verification
- Integration: `runtime:tests/cmp/graph_instantiate_*` on H-001 and H-003.
- Unit: `kernel:tests/cmp/graph_attenuate_*` on `qemu-x86_64`.

#### Evidence
- none

### CMP-025 · Bind Component Inputs<T> and Outputs<T> declarations to Channels at launch
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: CMP-014, IPC-031, PKG-031
- Baseline: §10, §12
- Invariants: I-039

§10 lists Inputs<T> and Outputs<T> as Component contents. The V0.5 four apps need declared typed endpoints wired by the launch path rather than discovered ad hoc. IPC generates the endpoint types; CMP binds them at launch. S-019 is prototyped.

<!-- covers: INV-0224, INV-0225 -->

#### Out of scope
IDL codegen for endpoint bundles (IPC). Graph wiring of many children (CMP-024). Package schema (PKG).

#### Acceptance criteria
- [ ] A Component whose manifest declares `Inputs<T>` and `Outputs<T>` receives those Channel endpoints at launch.
- [ ] A missing required Input returns a typed error and the Component is not scheduled.
- [ ] Endpoints are typed; sending the wrong message type returns a typed error.
- [ ] The four V0.5 apps launch with declared endpoints, not ad hoc discovery.

#### Verification
- Unit: `kernel:tests/cmp/inputs_outputs_bind_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Integration: Terminal, File Browser, Text Editor and Image Viewer launch on H-003.

#### Evidence
- none

### CMP-026 · Show ResourceDomain plus capabilities subsume mount, pid, net, user and ipc namespaces
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: CMP-007, CMP-027, SCH-023
- Baseline: §23, §53
- Invariants: I-019

Per-namespace mapping with tests: mount to capability-scoped storage, pid to `Object<Component>` handles, net to network Capabilities, user to no ambient uid, ipc to Channels. Native software needs no namespace setup step.

<!-- covers: INV-0438, INV-0991 -->

#### Out of scope
Linux namespaces inside the personality (LNX). ResourceDomain controllers (SCH). Network Capabilities (NET).

#### Acceptance criteria
- [ ] A native Component has no mount, pid, net, user or ipc namespace setup step on its launch trace.
- [ ] Process identity for native software is the Component handle, not a pid namespace.
- [ ] Native IPC is a Channel; a native crate that opens a POSIX ipc object fails CI via CMP-013.
- [ ] A native Component has no ambient uid; authority is the Capability set it was handed.

#### Verification
- Integration: `runtime:tests/cmp/namespace_subsumption_*` on H-001 and H-002.
- Review: SCH domain-subsumes-cgroups tests remain green.

#### Evidence
- none

### CMP-027 · Implement the native launch path from Package entry to scheduled Component
- Type: build
- Milestone: V0.5
- Status: todo
- Size: L
- Owner: none
- Depends on: CMP-005, CMP-009, SCH-007
- Baseline: §34, §53, §60
- Invariants: I-021, I-039

§34 and §53: resolve the Package generation entry, create ResourceDomain, create Component, attach declared Capabilities, map immutable Package objects, schedule. Required by the V0.5 cold-boot-to-desktop demo and the four apps. Native init consumes this path; it does not own it.

<!-- covers: INV-0641, INV-1015 -->

#### Out of scope
Native init (SVC). Package install (PKG). Warm-start metadata cache (CMP-040). Optional-capability degraded launch (CMP-043).

#### Acceptance criteria
- [ ] Launching a Package entry creates a ResourceDomain, a Component, attached declared Capabilities and mapped Package objects, then schedules the TaskGroup, as one launch path.
- [ ] A Component whose required Capabilities are missing is not scheduled and returns a typed error.
- [ ] Terminal, File Browser, Text Editor and Image Viewer each launch from an immutable Package on H-003.
- [ ] The launch path performs no file lookup or dependency resolution after mapping.

#### Verification
- Integration: `runtime:tests/cmp/native_launch_*` on H-001, H-002 and H-003.
- Demo: V0.5 cold boot to native desktop showing the four apps.

#### Evidence
- none

### CMP-028 · Preserve Component identity across restart so clients can rebind
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: CMP-004, CMP-030
- Baseline: §32, §60
- Invariants: I-037

V0.5 compositor crash recovery rebinds all windows. §32 needs a stable Component slot identity with an instance generation counter on `Object<Component>`. SVC owns supervision; CMP owns the identity and exit-cause surface it consumes.

<!-- covers: INV-1185 -->

#### Out of scope
Supervisor restart policy (SVC). Client disconnect/rebind stubs (IPC, SDK). Compositor surface rebind (GFX).

#### Acceptance criteria
- [ ] `Object<Component>` carries a stable slot identity and an instance generation counter that increments on restart.
- [ ] After a supervised restart, a client holding the slot identity observes the new instance generation and a typed disconnect for the old instance.
- [ ] Exit cause of the old instance remains readable on the handle after restart.
- [ ] One hundred consecutive compositor-kill cycles on H-003 rebind without application exit, using this identity.

#### Verification
- Unit: `kernel:tests/cmp/restart_identity_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Integration: compositor kill/rebind loop on H-003 (SVC gate consumes this).

#### Evidence
- none

### CMP-029 · Share verified code pages across Component instances of one Package
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: CMP-017, CMP-002, MEM-016, PKG-039
- Baseline: §34
- Benchmarks: B-008
- Invariants: I-039

§34: executables are verified once, shared and deduplicated. Sharing Code pages across instances is the main lever that keeps per-Component resident memory under the B-008 register target.

<!-- covers: GAP-0491 -->

#### Out of scope
Content-store page sharing primitive (MEM). Store-side verified pages (PKG).

#### Acceptance criteria
- [ ] Two Components of the same Package share Code pages, verified by physical-page identity.
- [ ] B-008 for an idle instance of a shared Package is no worse than the V0.5 regression band versus V0 on H-001 and H-002.
- [ ] Unmapping one instance does not unmap the other's Code pages.
- [ ] `os inspect` reports the shared mapping.

#### Verification
- Unit: `kernel:tests/cmp/shared_code_pages_*` on `qemu-x86_64`.
- Bench: B-008 on H-001 and H-002; target per register.

#### Evidence
- none

### CMP-030 · Study Erlang/BEAM supervision and restart strategies for Component recovery
- Type: spike
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: none
- Baseline: §32, §58

Informs Component-side restart identity and exit-cause reporting consumed by SVC supervision at V0.5. Non-gating research study scheduled at the rung where restart and rebind first matter. SVC owns the service-side study.

<!-- covers: INV-1146 -->

#### Out of scope
SVC supervision strategies (SVC-014). Restart policy decision (SVC).

#### Acceptance criteria
- [ ] `reports/spikes/CMP-030.md` exists with the spike skeleton headings.
- [ ] The report names which BEAM strategies map onto Component restart identity and typed exit causes.
- [ ] The report names which BEAM ideas are rejected because they assume shared-heap processes.

#### Verification
- Report: which BEAM supervision strategies map onto Component restart identity and typed exit causes; what `Object<Component>` must expose so SVC can implement one-for-one and rest-for-one restart without CMP owning the supervisor; which BEAM ideas are rejected because they assume shared-heap processes.

#### Evidence
- none

### CMP-031 · Study Genode composition, session routing and resource budgets
- Type: spike
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: none
- Baseline: §11, §23, §58
- Explores: S-019

Research study informing CMP-022 and SCH budget nesting. Moved out of V0 because it does not inform a V0 surface.

<!-- covers: INV-1136 -->

#### Out of scope
Graph declaration decision (CMP-022). ResourceDomain nesting (SCH).

#### Acceptance criteria
- [ ] `reports/spikes/CMP-031.md` exists with the spike skeleton headings.
- [ ] The report compares Genode session routing with declared Inputs, Outputs and attenuated Capabilities.
- [ ] The report recommends constraints on dynamically created children, or recommends rejecting Genode's constraints, with reasons.

#### Verification
- Report: how Genode session routing compares with declared Inputs and Outputs plus attenuated Capabilities; what constraints on dynamically created children Genode enforces that CMP adopts or rejects; how Genode resource budgets nest relative to ResourceDomain membership owned by SCH.

#### Evidence
- none

### CMP-032 · Measure hardware address-space cost against SFI-style in-address-space isolation
- Type: spike
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: none
- Baseline: §10, §51
- Explores: S-007

Measures the lower bound of isolation cost (address-space switch, TLB, page-table footprint) versus an SFI or Wasm-style in-address-space sandbox on H-002 so CMP-021 rests on data. Wasm runtime work stays in WASM.

<!-- covers: GAP-0490, INV-0231 -->

#### Out of scope
The address-space decision (CMP-021). Wasm host (WASM).

#### Acceptance criteria
- [ ] `reports/spikes/CMP-032.md` exists with the spike skeleton headings.
- [ ] The report publishes switch, TLB and page-table footprint costs for a hardware address space and for an SFI or Wasm-style sandbox on H-002.
- [ ] The report states whether plugins and decoders can be isolated on H-002 without a hardware address space.

#### Verification
- Report: what is the measured address-space switch, TLB and page-table footprint cost of a hardware address space versus an SFI or Wasm-style in-address-space sandbox on H-002; can plugins and decoders be isolated without a hardware address space on that machine; which costs are paid per instance versus per mapping.

#### Evidence
- none

### CMP-033 · Bring the Component L1 Surface to freeze-candidate state with tests
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: CMP-014, CMP-004, CMP-003, CMP-009, CMP-008, ABI-034
- Baseline: §65, §66
- Invariants: I-040

L1 surfaces become freeze candidates at V1 with SDK v1 and freeze only at V4. Delivers the surface review and candidate conformance tests for `Object<Component>`, spawn, exit causes and Code mapping under the ABI freeze process. S-007 is not frozen.

<!-- covers: INV-0168 -->

#### Out of scope
L1 freeze and conformance suite (CMP-052). ABI-wide freeze-candidate review (ABI).

#### Acceptance criteria
- [ ] S-007 is listed as a freeze candidate in the surfaces register and remains not `frozen`.
- [ ] Candidate conformance tests cover create, start, destroy, panic/abort exit causes and Code mapping.
- [ ] Each candidate test cites the spike and decision in its closure for S-007.
- [ ] Binaries built against the V0.5 Component surface still run on the V1 kernel where promised.

#### Verification
- Unit: `kernel:tests/cmp/conformance_candidate_*` on CI matrix entries `qemu-x86_64`, `hw-h002` and `hw-h004`.
- Review: ABI freeze-candidate review records S-007.

#### Evidence
- none

### CMP-034 · Implement the Component creation fast path to the Register target
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: CMP-010, CMP-005, CMP-001
- Baseline: §10, §53
- Benchmarks: B-001
- Risks: R-001
- Invariants: I-029

V1 benchmark gate sets the B-001 absolute. Implements the mechanism chosen by CMP-010 (prewarmed templates, cached page tables, batched Capability-table setup) and verifies it with CMP-001.

<!-- covers: INV-1295, GAP-0489 -->

#### Out of scope
Linux baselines (CMP-035). Native AddressSpace replacement (CMP-045).

#### Acceptance criteria
- [ ] B-001 on H-002 meets the V1 absolute target in the register.
- [ ] The implementation matches the option accepted by CMP-010.
- [ ] Native ABI create, start and destroy remain unchanged; the V0 acceptance suite still passes.
- [ ] `os inspect` still prints Component state after the fast-path change.

#### Verification
- Bench: B-001 on H-001, H-002 and H-004; target per register.
- Integration: V0 create-grant-run-destroy suite on H-001 and H-002.

#### Evidence
- none

### CMP-035 · Publish Component creation beside fork+exec, clone and podman run
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: CMP-001, BEN-006
- Baseline: §53, §54
- Benchmarks: B-001
- Invariants: I-050, I-061

V1 benchmark gate requires creation latency published beside fork+exec, clone and container start on the same hardware. Extends the V0 harness with the Linux baselines under BEN methodology. No superiority claim.

<!-- covers: INV-1295 -->

#### Out of scope
OCI isolation cost as a separate metric (BEN B-015). Fast-path implementation (CMP-034).

#### Acceptance criteria
- [ ] B-001 reports on H-002 and H-004 include Linux fork+exec of a static binary, clone(CLONE_VM), posix_spawn and podman run of a minimal OCI image.
- [ ] The table is published with methodology; no claim of superiority appears without the table.
- [ ] Baselines run on the same machine, same session, matching B-001 method.

#### Verification
- Bench: B-001 on H-002 and H-004; target per register.
- Review: BEN methodology sign-off recorded on the pull request.

#### Evidence
- none

### CMP-036 · Decide how Personality processes map onto Components
- Type: adr
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: CMP-006, LNX-060, Q-010
- Baseline: §3, §10, §46, §48
- Decision: D-0067
- Invariants: I-014, I-025

V1 daily-driving through the Linux personality and Wine bring-up require a decided mapping of Linux and Windows processes to Components (one Component per process, per process tree, or a personality ResourceDomain hosting plain tasks) and how PID and exit status surface through `Object<Component>`. TSK owns thread-to-Task mapping.

<!-- covers: INV-0070 -->

#### Out of scope
Personality thread mapping (TSK-043). LNX mapping implementation (LNX-042). Windows process semantics (WIN).

#### Acceptance criteria
- [ ] At least two options are evaluated, including one Component per process and a personality ResourceDomain hosting plain tasks.
- [ ] The accepted option states how PID and exit status surface through `Object<Component>` without becoming native APIs.
- [ ] The decision cites `reports/spikes/LNX-060.md`.
- [ ] A Review line names who accepts the decision.

#### Verification
- Review: CMP, LNX and WIN lead sign-off recorded on the pull request.
- Report: the cited LNX spike report exists.

#### Evidence
- none

### CMP-037 · Write the Component design guidelines for SDK v1
- Type: docs
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: CMP-024, CMP-008, CMP-022
- Baseline: §11, §52

V1 exit publishes SDK v1. Developers need guidance on when to split an application into Components, graph patterns from §11, Capability attenuation per child, and panic/exit-cause handling. Required by V3-G12 (Layer 1 ABI reference pages exist for every entry point).

#### Out of scope
SDK crate guide (SDK). Generated L1 reference pages (DOC). Browser-shaped sample graph (CMP-041).

#### Acceptance criteria
- [ ] A published guide names when to split an application into Components, with the §11 graph patterns as examples.
- [ ] The guide states that children receive only attenuated parent Capabilities and that a panic aborts only its Component.
- [ ] The guide cites B-001 and B-008 rather than stating performance numbers.
- [ ] SDK v1 links the guide from the developer documentation.

#### Verification
- Review: SDK lead and DOC lead sign-off recorded on the pull request.

#### Evidence
- none

### CMP-038 · Implement the in-address-space Component class if the adr adopts it
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: CMP-021, CMP-014, WASM-012
- Baseline: §10, §13
- Invariants: I-014

Conditional on CMP-021: gives fine-grained sandboxes (the V1 Wasm editor plugin demo) a kernel-visible Component identity, Capability set and ResourceDomain membership without a hardware address space. WASM owns the runtime. If the decision rejects the class, this task is dropped with reason descoped.

<!-- covers: GAP-0490, INV-0231 -->

#### Out of scope
Wasm runtime (WASM). Editor plugin chrome (APP). Address-space decision (CMP-021).

#### Acceptance criteria
- [ ] If the accepted decision records an in-address-space class, a Component of that class has kernel-visible identity, a Capability set and ResourceDomain membership without a hardware address space.
- [ ] If that class exists, a denied Capability still returns `Error::Rights` and allocates no handle.
- [ ] If the accepted decision records no in-address-space class, evidence records the decision and no kernel object of that class exists.
- [ ] Wasm guest isolation remains a userspace runtime concern (I-046).

#### Verification
- Unit: `kernel:tests/cmp/in_address_space_*` on `qemu-x86_64` if the class is adopted.
- Review: WASM and CMP lead sign-off recorded on the pull request.
- Demo: V1 Wasm editor plugin, if the class is adopted.

#### Evidence
- none

### CMP-039 · Publish the Component isolation boundary assurance report
- Type: docs
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: CMP-012, SEC-002
- Baseline: §9.1, §51, §59
- Threats: T-001, T-011
- Invariants: I-021, I-049

Before daily-driving, enumerate every retained Linux kernel path reachable from a native Component and show each is gated by a Capability or unreachable. Feeds the SEC threat model and the V4 external audit.

<!-- covers: INV-1164 -->

#### Out of scope
Syscall filter implementation (ABI). External audit (SEC). Closing audit findings (CMP-053).

#### Acceptance criteria
- [ ] A published report lists retained Linux kernel paths reachable from a native Component.
- [ ] Each listed path is marked Capability-gated or unreachable, with the test that proves it.
- [ ] The report cites T-001 and T-011.
- [ ] SEC acknowledges the report on the threat-model pull request.

#### Verification
- Review: SEC lead sign-off recorded on the pull request.
- Integration: CMP-012 still pass on H-001, H-002 and H-004.

#### Evidence
- none

### CMP-040 · Cache Component launch metadata across launches
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: CMP-027, CMP-019, PKG-044
- Baseline: §34
- Benchmarks: B-016
- Invariants: I-039

Warm start caches the Component graph, Capability set and relocation state. CMP owns launch; PKG supplies the store objects.

<!-- covers: INV-0631 -->

#### Out of scope
Package verification cache (PKG). Warm-startup harness (CMP-019). Linking model (SDK).

#### Acceptance criteria
- [ ] A second launch of the same Package reuses cached graph, Capability set and relocation state; a trace shows no rebuild of those structures.
- [ ] B-016 for Terminal and Editor on H-002 meets the V1 absolute target in the register.
- [ ] Invalidating the Package identity discards the cache; the next launch rebuilds metadata.
- [ ] Cached metadata is not used across Package identity or publisher changes.

#### Verification
- Integration: `runtime:tests/cmp/launch_cache_*` on H-001 and H-002.
- Bench: B-016 on H-002 and H-004; target per register.

#### Evidence
- none

### CMP-041 · Document a browser-shaped Component graph as the composition example
- Type: docs
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: CMP-024, CMP-037
- Baseline: §11

§11 example graph (UI, Network, per-Tab Renderer) is CMP composition guidance for SDK samples, not an SDK runtime feature.

<!-- covers: INV-0234 -->

#### Out of scope
A shipped native browser (APP, LNX). SDK sample runtime (SDK).

#### Acceptance criteria
- [ ] A published sample describes a browser-shaped graph with UI, Network and per-Tab Renderer Components and their Capability sets.
- [ ] Each child in the sample holds only the authority named for it in §11.
- [ ] The sample is linked from CMP-037.

#### Verification
- Review: SDK lead sign-off recorded on the pull request.

#### Evidence
- none

### CMP-042 · Decide the order and equivalence tests for replacing the Component wrapper
- Type: adr
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: CMP-010, CMP-034
- Baseline: §6
- Decision: D-0065
- Invariants: I-009, I-057

Phase C later: chooses which wrapper pieces (`mm_struct`, namespaces, cgroups, `task_struct`) are replaced first and the ABI-equivalence strategy. Options include address-space-first, membership-first, and big-bang replacement.

<!-- covers: INV-0146 -->

#### Out of scope
AddressSpace implementation (CMP-045). Membership implementation (CMP-046). Equivalence suite (CMP-048).

#### Acceptance criteria
- [ ] At least two options are evaluated, including address-space-first and membership-first.
- [ ] The accepted option names the ABI-equivalence tests that must stay green during replacement.
- [ ] The accepted option states that the Component ABI does not change.
- [ ] A Review line names who accepts the decision.

#### Verification
- Review: ABI lead and CMP lead sign-off recorded on the pull request.

#### Evidence
- none

### CMP-043 · Launch Components in degraded mode when optional capabilities are denied
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: CMP-027, CMP-022, PKG-075, SEC-047, CMP-031, CMP-021
- Baseline: §9.1, §34
- Freezes: S-019
- Invariants: I-021

V2 exit criterion: an application whose optional Capability requests are denied at install still launches in a declared degraded mode. The launch path distinguishes required from optional declared Capabilities. S-019 freezes at V2.

<!-- covers: INV-0641 -->

#### Out of scope
Manifest optional-versus-required schema (PKG). Install-time review UI (SEC, APP).

#### Acceptance criteria
- [ ] A Package with a denied optional Capability still launches; required missing Capabilities still refuse launch with a typed error.
- [ ] The Component can observe which optional Capabilities are absent without being granted them.
- [ ] S-019 is `frozen` by this task once its spike and decision are done.
- [ ] A regression test covers denied-optional versus missing-required.

#### Verification
- Integration: `runtime:tests/cmp/degraded_launch_*` on H-002, H-004 and H-005.
- Review: PKG and SEC sign-off recorded on the pull request.

#### Evidence
- none

### CMP-044 · Scale-test 500-Component graphs for creation, memory and teardown
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: CMP-024, CMP-001, CMP-002, SCH-009
- Baseline: §11
- Benchmarks: B-001, B-008
- Invariants: I-029

§11 browser-style graphs need hundreds of Components per application. Verifies bounded per-Component memory and creation latency under contention on all three V2 target machines.

<!-- covers: GAP-0491 -->

#### Out of scope
B-001 and B-008 harnesses (CMP benches). Kernel-object limits (SCH).

#### Acceptance criteria
- [ ] A graph of five hundred Components is created, inspected and torn down on H-002, H-004 and H-005.
- [ ] B-001 and B-008 on that graph meet the V2 register targets on those machines.
- [ ] Teardown reclaims every object; `os inspect` lists none of the five hundred afterward.
- [ ] Kernel-object limits return typed exhaustion errors if the graph exceeds the domain budget, rather than an unbounded allocation.

#### Verification
- Integration: `runtime:tests/cmp/graph_scale_500` on H-002, H-004 and H-005.
- Bench: B-001 and B-008 on H-002, H-004 and H-005; target per register.

#### Evidence
- none

### CMP-045 · Implement the native AddressSpace Object replacing mm_struct wrapping
- Type: build
- Milestone: V2
- Status: todo
- Size: L
- Owner: none
- Depends on: CMP-042, CMP-017
- Baseline: §6, §10
- Invariants: I-057

First step of the native Component implementation: page-table templates and prewarmed address spaces owned by the native object while the Component ABI stays unchanged.

<!-- covers: INV-0146 -->

#### Out of scope
Namespace and cgroup replacement (CMP-046). Equivalence suite (CMP-048). MemoryObject mapping (MEM).

#### Acceptance criteria
- [ ] Component address spaces are native objects; create no longer wraps `mm_struct` as the ABI-visible mechanism.
- [ ] The V0, V0.5 and V1 Component acceptance suites still pass.
- [ ] `os inspect component` still prints address-space membership without Linux mm fields.
- [ ] B-001 on H-002 meets the V2 regression band versus V1.

#### Verification
- Unit: `kernel:tests/cmp/native_address_space_*` on CI matrix entries `qemu-x86_64`, `hw-h002` and `hw-h004`.
- Bench: B-001 on H-002; target per register.
- Integration: V0 through V1 Component suites on H-001.

#### Evidence
- none

### CMP-046 · Replace namespace and cgroup wrapping with native Component membership
- Type: build
- Milestone: V2
- Status: todo
- Size: L
- Owner: none
- Depends on: CMP-042, CMP-045, SCH-023
- Baseline: §6, §23, §53
- Invariants: I-019, I-057

Removes namespace and cgroup creation from the native path so Component and ResourceDomain membership are native kernel state. The Linux personality keeps namespaces for compatibility software only.

<!-- covers: INV-0146, INV-0991 -->

#### Out of scope
Personality namespaces (LNX). ResourceDomain accounting (SCH). Equivalence suite (CMP-048).

#### Acceptance criteria
- [ ] Native Component create does not create Linux namespaces or cgroup directories.
- [ ] ResourceDomain membership is native kernel state visible in `os inspect`.
- [ ] Linux-personality processes still have namespaces inside the personality.
- [ ] The V0 through V1 Component acceptance suites still pass.

#### Verification
- Unit: `kernel:tests/cmp/native_membership_*` on `qemu-x86_64`.
- Integration: native launch trace with no unshare or cgroupfs write on H-002.
- Review: LNX confirms personality namespaces remain available.

#### Evidence
- none

### CMP-047 · Extend the warm-startup harness to every shipped native application
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: CMP-019
- Baseline: §34, §54
- Benchmarks: B-016
- Invariants: I-042, I-061

V2 benchmark gate: application warm startup for all shipped native applications published, with Terminal and Editor holding the register target. Extends CMP-019 to the shell and store client.

#### Out of scope
Harness methodology (BEN). Per-app APP instrumentation (APP).

#### Acceptance criteria
- [ ] B-016 reports exist for every shipped native application on H-002, H-004 and H-005.
- [ ] Terminal and Editor meet the V2 B-016 absolute target in the register on those machines.
- [ ] Shell and store client are included in the published table.

#### Verification
- Bench: B-016 on H-002, H-004 and H-005; target per register.

#### Evidence
- none

### CMP-048 · Prove the native Component implementation keeps the ABI unchanged
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: CMP-045, CMP-046, CMP-001, ABI-044
- Baseline: §6, §65
- Benchmarks: B-001
- Invariants: I-057

Runs the V0, V0.5 and V1 acceptance suites and B-001 on the native implementation. V2 benchmark gate allows no regression beyond the register band without an accepted decision.

<!-- covers: INV-0146 -->

#### Out of scope
ABI conformance content (ABI). Fast-path work (CMP-034).

#### Acceptance criteria
- [ ] V0, V0.5 and V1 Component acceptance suites pass on the native implementation on H-001, H-002, H-004 and H-005.
- [ ] B-001 meets the V2 regression target versus V1 on those machines, or an accepted decision explains the exception.
- [ ] Native ABI snapshots for S-007 match the V1 freeze-candidate snapshot.

#### Verification
- Integration: Component acceptance suites on H-001, H-002, H-004 and H-005.
- Bench: B-001 on H-002, H-004 and H-005; target per register.
- Review: ABI dual-implementation conformance sign-off recorded on the pull request.

#### Evidence
- none

### CMP-049 · Add fuzz targets for the Component spawn, exit and code-mapping Surface
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: CMP-033, BLD-035
- Baseline: §51, §65
- Invariants: I-040

V3 scope introduces continuous fuzzing (BLD infrastructure). CMP supplies the fuzz targets for its ABI surface so the 1.0 clean-window criterion is measurable. Required by V3-G10 (Kernel and IPC fuzzing has no stale open crasher).

#### Out of scope
Fuzzing fleet (BLD). L1 freeze (CMP-052).

#### Acceptance criteria
- [ ] Structure-aware fuzz targets exist for Component spawn, exit-cause delivery and Code mapping.
- [ ] Targets run on BLD continuous fuzzing and file crashes against CMP.
- [ ] A panic in the harness is a test failure; no known open crasher is older than the V3 register window at V3 exit.
- [ ] Oracles include: no handle issued on `Error::Rights`, no unwind across the ABI, W^X on Code mappings.

#### Verification
- Fuzz: `kernel:fuzz/cmp_spawn_exit_map` on the BLD fleet.
- Review: BLD fuzz-infra sign-off recorded on the pull request.

#### Evidence
- none

### CMP-050 · Complete the Component L1 reference documentation
- Type: docs
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: CMP-033
- Baseline: §7, §10, §11, §66

V3 exit requires 100 percent L1 reference pages. DOC generates signatures from the IDL; CMP writes the lifecycle, exit-cause, graph and isolation semantics prose. Required by V3-G12 (Layer 1 ABI reference pages exist for every entry point).

#### Out of scope
IDL-to-docs generator (DOC). ABI entry-point catalogue (ABI).

#### Acceptance criteria
- [ ] Every S-007 entry point has a reference page with lifecycle, exit-cause, graph and isolation semantics.
- [ ] Pages are generated from IDL plus CMP-authored prose; signatures are not hand-typed.
- [ ] The V3 documentation coverage gate includes these pages.

#### Verification
- Review: DOC lead sign-off recorded on the pull request.

#### Evidence
- none

### CMP-051 · Isolate Components across user sessions with negative tests
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: CMP-012, SEC-060, WASM-012, SEC-028, SEC-064
- Baseline: §9.1, §51, §63
- Threats: T-026
- Invariants: I-021

V3 introduces multi-user. Components created in one user session inherit that session's ResourceDomain and identity, and negative tests show no cross-user reach to memory, objects or Capabilities.

<!-- covers: INV-1164 -->

#### Out of scope
Session objects and identity (SEC). Per-user grant stores (CAP).

#### Acceptance criteria
- [ ] A Component created in session A cannot map memory, list Capabilities or send on Channels of a Component in session B.
- [ ] Cross-session attempts return `Error::Rights` and allocate no handle.
- [ ] Each session's Components inherit that session's ResourceDomain root.
- [ ] Tests run with two concurrent sessions on H-002.

#### Verification
- Integration: `runtime:tests/cmp/multi_user_isolation_*` on H-002, H-004 and H-005.
- Review: SEC multi-user sign-off recorded on the pull request.

#### Evidence
- none

### CMP-052 · Ship the conformance suite for the frozen Component L1 Surface
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: CMP-033, CMP-049, CMP-009, CMP-008, CMP-010, CMP-016, ABI-049, CMP-015, CMP-021, CMP-045, CMP-046
- Baseline: §65, §66
- Freezes: S-007
- Invariants: I-040

V4 exit: L1 frozen with a conformance suite. No L1 freeze happens before V4. The freeze-time conformance suite for `Object<Component>`, spawn and exit causes lands here and freezes S-007.

#### Out of scope
ABI freeze decision (ABI). Layer 2 component manifest freeze (CMP-043).

#### Acceptance criteria
- [ ] S-007 is `frozen` and names this task.
- [ ] Every S-007 entry point has a conformance test; binaries built against the freeze candidate run on every subsequent V4 build.
- [ ] The closure contains the S-007 spike and the spawn, panic and wrapper-versus-native decisions.
- [ ] Deprecated Component entry points are absent from the frozen surface.

#### Verification
- Unit: `kernel:tests/cmp/conformance_l1_*` on every V4 hardware-scope entry used by CI.
- Review: ABI freeze sign-off recorded on the pull request.

#### Evidence
- none

### CMP-053 · Close external audit findings against the Component isolation boundary
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: CMP-039, CMP-012, SEC-067, SEC-070
- Baseline: §51, §63
- Risks: R-055
- Threats: T-001
- Invariants: I-021

V4 exit requires the external audit closed. Findings against Component isolation, exit handling or Code mapping are fixed with regression tests and re-verified by CMP-012.

#### Out of scope
Commissioning the audit (SEC). Capability-enforcement findings (CAP).

#### Acceptance criteria
- [ ] Every High and Critical finding against Component isolation, exit handling or Code mapping is fixed.
- [ ] Each fix has a regression test that fails without the fix and passes with it.
- [ ] CMP-012 pass on the post-fix kernel on H-002.
- [ ] The auditor re-verifies High and Critical CMP findings.

#### Verification
- Integration: isolation negative tests on H-002 after each fix.
- Review: auditor re-verification recorded as `https://` evidence on the pull request.

#### Evidence
- none

### CMP-054 · Publish reproducible Component creation, memory and startup metrics
- Type: docs
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: CMP-001, CMP-002, CMP-019, CMP-018, BEN-063
- Baseline: §54, §70
- Benchmarks: B-001, B-008, B-016, B-017
- Invariants: I-042, I-050, I-061

1.0 benchmark gates require every §54 metric published against Linux, Windows and macOS with methodology, raw data and scripts. CMP delivers the reproducible package for its three metric families under BEN methodology.

<!-- covers: INV-1295 -->

#### Out of scope
Cross-OS baseline pinning (BEN). Claim lint (BEN).

#### Acceptance criteria
- [ ] B-001, B-008, B-016 and B-017 reports exist for every Tier 1 machine in 1.0 hardware scope.
- [ ] Methodology, raw data and scripts are in the BEN reproduction pack so a third party can replay the numbers.
- [ ] No 1.0 announcement states a Component performance number that is not in those reports.

#### Verification
- Bench: B-001, B-008, B-016 and B-017 on every 1.0 hardware-scope H-ID; target per register.
- Review: BEN lead sign-off recorded on the pull request.

#### Evidence
- none

### CMP-055 · Write the Component section of the ABI stability statement
- Type: docs
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: CMP-052, ABI-050
- Baseline: §65, §66, §70
- Invariants: I-040, I-059

1.0 exit publishes the ABI stability statement and explicit non-promises. The Component section states what is frozen, the status of the in-address-space class, and what stays personality-only.

#### Out of scope
ABI-wide stability declaration (ABI). Capability non-promises (CAP).

#### Acceptance criteria
- [ ] The 1.0 ABI stability statement includes a Component section naming the frozen S-007 operations.
- [ ] The section states whether an in-address-space class is frozen, prototyped or absent.
- [ ] The section states that PID, fork, exec, process groups, namespaces and container runtimes are personality-only.
- [ ] The section states that Layer 1 Component changes require a new major version.

#### Verification
- Review: ABI lead sign-off recorded on the pull request.

#### Evidence
- none
