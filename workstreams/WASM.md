# WASM · WebAssembly components
- Prefix: WASM
- Lead: none
- Baseline: §13

<!-- roadmap:generated:begin summary -->
Tasks: 24 live, 0 done, 0 in-progress, 24 todo, 0 dropped. Ready: 1. Blocked: 23. Weighted: 0%.
<!-- roadmap:generated:end -->

## Scope

WASM owns portable WebAssembly as a Component kind beside machine code (§13). It studies the WebAssembly Component Model and WASI as inputs to the native model, records the Wasm role versus the Native ABI, evaluates and selects a userspace runtime, binds WASI imports to Capability, maps WIT worlds onto Channel, and hosts plugins, sandboxed extensions, portable services, automation modules, downloaded Components and architecture-independent Packages. It names the Wasm artefact layout in Package, AOT-compiles content-addressed images, transfers MemoryObject resources into guests, fuzzes the host import table, versions host Interfaces with the Layer 2 rules, publishes B-047, debugs guests, isolates instances across sessions, ships a WASI subset conformance suite and author guide, and locks S-029 at V4.

The runtime stays in userspace. Wasm is not the Native ABI (I-046). Native machine-code Components remain first-class; this workstream does not force applications into Wasm (§57).

## Out of scope

`Object<Component>` spawn, address space and exit-cause objects (CMP). IDL language, Channel transport and the native-IDL-versus-WIT Decision (IPC). Capability table, mint, derive and revocation (CAP). MemoryObject backing and ownership-transfer enforcement (MEM). Package manifest schema, content store and SystemGenerations (PKG). Service supervisor and restart policy (SVC). Semantic interface registry, automation-rule Decision and AI broker (SEM). Editor chrome, consent UI and store client (APP). TypeScript language surface and the native debugger attach Capability (SDK). License firewall and userspace allowlist (GOV). Fuzz fleet, double-build and independent rebuilders (BLD). Benchmark methodology and publication dashboards (BEN). Signed repository operation (REL). Session and identity objects (SEC). Docs site generation (DOC). Native ABI firewall lint (ABI). ResourceDomain object (SCH). Inspect transport (OBS). NetworkConnection object (NET). UserSelected minting (STO). Linux and Windows personalities (LNX, WIN). VM manager (VIRT).

## Tasks

### WASM-001 · Decide Wasm role versus native machine-code Components
- Type: adr
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: WASM-002
- Baseline: §13, §57
- Decision: D-0334
- Invariants: I-046, I-014

V0 records how Wasm sits next to machine-code Components so later rungs do not treat Wasm as the Native ABI or duplicate WIT and WASI without justification (§13, §57). Native machine code remains a first-class target. This Decision is the V0 Wasm ADR; integration waits for V1.

<!-- covers: INV-0264, INV-1119 -->

#### Out of scope
Runtime crate and host placement (WASM-007). Native IDL versus WIT mapping (IPC-022). Host implementation (WASM-012).

#### Acceptance criteria
- [ ] Option A (Wasm is a first-class Component kind beside machine code, WIT and WASI reused rather than duplicated), option B (Wasm is only an in-process plugin runtime inside a machine-code Component, not a Component kind), and option C (Wasm is the Native ABI and every application is a Wasm module) are evaluated against I-046 and §57.
- [ ] The accepted option keeps native machine-code Components first-class and records that option C is rejected.
- [ ] The Decision states that WIT and WASI are not duplicated under native names without a cited justification.
- [ ] ABI and CMP leads record Review sign-off on the pull request.

#### Verification
- Review: ABI and CMP leads sign off on the pull request; the Decision lists at least two options.

#### Evidence
- none

### WASM-002 · Study Component Model and WASI as native-model inputs
- Type: spike
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: none
- Baseline: §13, §58
- Explores: S-029

V0 L1-informing study in the FIDL class: WIT, resources, interface versioning and WASI capability interfaces feed IPC-006, CMP Component Decisions and WASM-001 (§13, §58). V0 bars Wasm integration beyond an ADR; this study is the research input, not a host.

<!-- covers: INV-0263, INV-1141, INV-1142 -->

#### Out of scope
IDL selection (IPC-006). Wasm role Decision (WASM-001). Runtime evaluation (WASM-003). Host implementation (WASM-012).

#### Acceptance criteria
- [ ] `reports/spikes/WASM-002.md` exists with the spike skeleton headings.
- [ ] The report lists adopted and rejected ideas from the WebAssembly Component Model (WIT, resources, interface versioning) for the native Interface and Component model.
- [ ] The report scores WASI capability-based interfaces against native Capability and names alignments and mismatches.
- [ ] Findings are cited by IPC-006 and WASM-001.

#### Verification
- Report: which WIT, resource and versioning ideas the native IDL and Component model adopt or reject; how WASI capability interfaces align with native Capability; which questions remain for WASM-001 and IPC-022; confirmation that Wasm is not proposed as the Native ABI.
- Review: IPC and CMP leads sign off on the pull request.

#### Evidence
- none

### WASM-003 · Evaluate userspace Wasm runtimes for host selection
- Type: spike
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: WASM-001, WASM-002, GOV-003
- Baseline: §13
- Invariants: I-046

Measures instantiation cost, memory footprint and Capability binding for Wasmtime, WAMR, Wasmer and a custom userspace option so WASM-007 is evidence-based (§13). V0 has no Wasm integration; this spike stays off the kernel. In-kernel embed is out on license grounds (GAP-0024, I-046).

<!-- covers: GAP-0523 -->

#### Out of scope
Runtime Decision (WASM-007). Host implementation (WASM-012). B-047 harness (WASM-006). License firewall policy (GOV).

#### Acceptance criteria
- [ ] `reports/spikes/WASM-003.md` exists with the spike skeleton headings.
- [ ] The report compares Wasmtime, WAMR, Wasmer and a custom userspace option on instantiation cost, resident footprint and Capability import binding on H-001.
- [ ] The report records in-Component versus shared-service placement costs for each candidate that binds Capability.
- [ ] The report records that in-kernel embed is rejected on GPLv2 incompatibility and names which candidates are acceptable as userspace dependencies.

#### Verification
- Report: which userspace runtime and host placement WASM-007 may accept; how each candidate binds Capability without ambient WASI Preview1 imports; what instantiation and footprint evidence the Decision must cite; why in-kernel embed is not an option.
- Review: GOV lead confirms license conclusions on the pull request.

#### Evidence
- none

### WASM-004 · Specify Wasm Component artefact layout in Package
- Type: docs
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: WASM-001, PKG-011, PKG-012, IPC-022
- Baseline: §13, §28, §34
- Risks: R-017

Names the Wasm Component-kind artefact (module, WIT world, requested Capability) so the V0.5 Package format reserves kind fields before the first immutable install (§28, §34). PKG owns the manifest schema; this document is the Wasm layout PKG stores.

#### Out of scope
Manifest schema implementation (PKG-031). AOT compilation (WASM-005). Precompiled-artefact store objects (PKG-063).

#### Acceptance criteria
- [ ] The specification names module bytes, WIT world identity and requested Capability as reserved Package fields for the Wasm Component kind.
- [ ] The specification states that PKG owns schema and store layout and that WASM produces the artefact.
- [ ] A Review line names who accepts the document.

#### Verification
- Review: PKG and IPC leads sign off on the pull request.

#### Evidence
- none

### WASM-005 · AOT-compile Wasm Components into Package artefacts
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: WASM-012, WASM-004, WASM-007, PKG-038, CMP-017
- Baseline: §13, §34
- Invariants: I-039

Produces content-addressed AOT images from Wasm Component Packages so V1 plugin instantiation can map verified-once pages (§34). PKG stores the objects (PKG-063); WASM compiles them.

#### Out of scope
Store schema and dedup (PKG-063). Bit-for-bit independent rebuilders (WASM-023). Shader artefacts (GFX).

#### Acceptance criteria
- [ ] Compiling a Wasm Component Package emits a content-addressed AOT object named from the WIT world and module identity.
- [ ] Instantiating from that object maps pages without recompiling the module on the launch path.
- [ ] Identical module and WIT world inputs produce the same content identity on H-001.
- [ ] `os inspect component` names the mapped AOT object for a live Wasm Component.

#### Verification
- Unit: `wasm:tests/aot/compile_*` on CI matrix entry `qemu-x86_64`.
- Integration: Package launch maps the AOT object on H-001.

#### Evidence
- none

### WASM-006 · Measure Wasm instantiation and Channel round trip
- Type: benchmark
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: WASM-012, WASM-013, BEN-005, BEN-007
- Baseline: §13, §54
- Benchmarks: B-047
- Invariants: I-061

V1 publish-only run of B-047 (`bench:wasm-component`) so the Wasm-on-Channel prototype is measured. Targets live in the register. No superiority claim.

#### Out of scope
Methodology and dashboards (BEN). Machine-code creation and Channel baselines (B-001, B-004). 1.0 cross-OS republication (WASM-024).

#### Acceptance criteria
- [ ] B-047 reports for H-001, H-002 and H-004 exist under `reports/benchmarks/B-047/` with instantiation and Channel round-trip p50 and p99.
- [ ] Each report includes the register baselines (B-001, B-004, Linux Wasmtime component instantiation) on the same machine.
- [ ] The V1 target kind in the B-047 register is `publish`.
- [ ] CI invokes `bench:wasm-component` on H-001.

#### Verification
- Bench: B-047 on H-001, H-002 and H-004; target per register.
- Review: BEN methodology sign-off recorded on the pull request.

#### Evidence
- none

### WASM-007 · Decide the userspace Wasm runtime and host placement
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: WASM-003, WASM-001, GOV-003
- Baseline: §13, §51
- Decision: D-0332
- Invariants: I-046

Chooses the userspace runtime and whether it runs in-Component or as a shared host service, using WASM-003 (§13). License forces userspace-only; in-kernel embed is rejected on GPLv2 incompatibility (GAP-0024, I-046).

<!-- covers: INV-0275, GAP-0024 -->

#### Out of scope
WASI import set (WASM-008). Host implementation (WASM-012). License allowlist text (GOV-016).

#### Acceptance criteria
- [ ] Options evaluated include Wasmtime as a shared userspace host service, Wasmtime in-Component, WAMR in-Component, Wasmer as a shared userspace host service, and a custom userspace runtime.
- [ ] The accepted option names the crate, host placement and that the runtime is userspace-only.
- [ ] In-kernel embed is recorded as rejected on GPLv2 incompatibility with the kernel license.
- [ ] GOV and CMP leads record Review sign-off on the pull request.

#### Verification
- Review: GOV and CMP leads sign off on the pull request; the Decision cites WASM-003.

#### Evidence
- none

### WASM-008 · Decide WASI imports bound to native Capability
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: WASM-001, WASM-002, IPC-022, STO-034, NET-007, CAP-005
- Baseline: §13, §9, §9.1
- Decision: D-0333
- Threats: T-001
- Invariants: I-021, I-046

Lists which WASI worlds bind to Capability (clocks, random, UserSelected, NetworkConnection) and which ambient Preview1 imports are forbidden, so WASM-013 has a closed import table (§9.1, §13). Native software never sees POSIX preopens. This Decision lists ABI surface S-029.

#### Out of scope
Channel mapping implementation (WASM-013). NetworkConnection object (NET-014). UserSelected minting (STO). IDL versus WIT mapping (IPC-022).

#### Acceptance criteria
- [ ] Option A (documented WASI Preview2 world subset bound to Capability for clocks, random, UserSelected and NetworkConnection, with Preview1 ambient imports forbidden), option B (full WASI Preview2 including filesystem and socket worlds), and option C (no WASI worlds; only native IDL imports) are evaluated against T-001 and I-021.
- [ ] The accepted option names every allowed import and states that undeclared imports fail closed with `Error::Rights` and allocate no handle.
- [ ] Preview1 preopens are recorded as rejected as POSIX-shaped ambient authority.
- [ ] CAP and IPC leads record Review sign-off on the pull request.

#### Verification
- Review: CAP and IPC leads sign off on the pull request; the Decision lists S-029 and at least two options.

#### Evidence
- none

### WASM-009 · Load a sandboxed Wasm plugin with declared Capability rights
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: WASM-012, WASM-013, WASM-008, APP-011, PKG-028, CAP-005, CMP-021
- Baseline: §13, §9.1
- Threats: T-001
- Invariants: I-021

V1 demo: a Wasm plugin in the Text Editor holds only declared Capability (§13, §9.1). APP owns editor chrome; WASM owns the plugin host and denial tests. In-address-space class, if adopted, is CMP-038.

<!-- covers: INV-0266, INV-0267 -->

#### Out of scope
Editor chrome and buffer UI (APP-011). In-address-space Component class (CMP). Consent prompt UI (APP-025).

#### Acceptance criteria
- [ ] The daily-driving Text Editor loads a Wasm plugin Package whose manifest lists only the declared Capability set, and the plugin exchanges typed messages with the editor over Channel.
- [ ] A plugin that requests an undeclared Capability receives `Error::Rights`, allocates no handle, and the editor continues.
- [ ] `os inspect component` shows the plugin as a Wasm Component whose Capability table matches the manifest.
- [ ] The denial is visible in the audit log.

#### Verification
- Integration: `wasm:tests/plugin/editor_*` on H-001 and H-002.
- Demo: V1 Wasm plugin in the editor on H-002.
- Review: APP lead confirms chrome remains in APP.

#### Evidence
- none

### WASM-010 · Fuzz the Wasm host ABI and Capability import table
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: WASM-012, WASM-013, WASM-008, BLD-042
- Baseline: §13, §51
- Risks: R-051
- Threats: T-003, T-006
- Invariants: I-028

The Wasm host is a trust boundary once the V1 prototype exists. A harness mutates WIT canonical ABI and import tables; panics and Capability forgeries fail closed. Feeds the V3 continuous-fuzzing gate.

#### Out of scope
Fuzz fleet, dedup and bisection (BLD-035). Native ABI syzkaller (BLD-016). WASI subset documentation (WASM-020).

#### Acceptance criteria
- [ ] Harness `wasm:fuzz/host_abi` mutates WIT canonical ABI payloads and import tables against the host.
- [ ] A forged Capability in an import table returns `Error::Rights`, allocates no handle, and does not panic.
- [ ] An undeclared import fails closed with a typed error and no host process abort.
- [ ] The harness is invoked from BLD nightly userspace fuzzing.

#### Verification
- Fuzz: `wasm:fuzz/host_abi` one hour nightly without panic.
- Unit: `wasm:tests/host/fail_closed_*` on CI matrix entry `qemu-x86_64`.

#### Evidence
- none

### WASM-011 · Transfer MemoryObjects into Wasm guest resources
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: WASM-013, MEM-010, MEM-005, MEM-007
- Baseline: §13, §16, §17

Large Channel payloads move as MemoryObject resources rather than copies through linear memory, so Wasm Components participate in the object ecosystem (§13, §16). MEM owns MemoryObject; WASM maps the guest resource.

#### Out of scope
MemoryObject kernel object and transfer enforcement (MEM). Inline small-message threshold (IPC-007). DMA and GPU properties (MEM).

#### Acceptance criteria
- [ ] A machine-code Component transfers a MemoryObject Capability to a Wasm Component over Channel and the sender loses access, verified by a subsequent map from the sender returning a typed error.
- [ ] The guest resource names the same MemoryObject; payload bytes are not copied into linear memory on that path, verified by physical-page identity.
- [ ] Guest drop or Component exit reclaims the resource without leaking the MemoryObject, verified by `os inspect memory`.
- [ ] A guest that lacks the MemoryObject Capability cannot map it.

#### Verification
- Unit: `wasm:tests/guest/memory_object_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Integration: page-identity check on H-001.

#### Evidence
- none

### WASM-012 · Host Wasm Components in userspace beside machine code
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: WASM-007, WASM-001, WASM-003, WASM-004, CMP-014, CMP-005, CMP-004, CMP-027, SCH-007, PKG-031, GOV-003, BLD-023, ABI-003, OBS-005, IPC-011
- Baseline: §13, §10, §34
- Risks: R-075
- Invariants: I-046, I-014, I-037

V1 scope: portable WebAssembly Components as a Component kind next to machine code (§13). Instantiates from Package, charges ResourceDomain, maps traps to typed disconnect, and CI-rejects linking the runtime into the kernel (GAP-0024, I-046).

<!-- covers: INV-0265 -->

#### Out of scope
WASI and Channel mapping (WASM-013). AOT images (WASM-005). In-address-space class (CMP-038). License policy (GOV).

#### Acceptance criteria
- [ ] Instantiating a Wasm Component Package creates an `Object<Component>` charged to a ResourceDomain, with kind visible in `os inspect component`.
- [ ] A guest trap reports a typed exit cause on the Component handle; the peer observes a typed disconnect and the Native ABI does not unwind.
- [ ] Linking the selected runtime into a kernel crate fails CI on `qemu-x86_64`.
- [ ] A machine-code Component still instantiates through the CMP launch path on the same image.
- [ ] Destroy reclaims the instance; `os inspect` lists none of its guest resources afterward.

#### Verification
- Unit: `wasm:tests/host/instantiate_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Integration: trap-to-disconnect case on H-001.
- Review: GOV lead confirms the kernel-link lint matches the license firewall.

#### Evidence
- none

### WASM-013 · Map WASI resources and WIT worlds onto Capability
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: WASM-012, WASM-008, IPC-022, IPC-010, IPC-013, CAP-005, CAP-011
- Baseline: §13, §12, §14
- Threats: T-001, T-003
- Invariants: I-021, I-028

V1 exit: a Wasm Component exchanges typed messages over Channel with a machine-code Component (§13). Capabilities are WASI resources and WIT interfaces are Channel after IPC-022.

<!-- covers: INV-0274 -->

#### Out of scope
IDL versus WIT Decision (IPC-022). Import-set Decision (WASM-008). MemoryObject guest resources (WASM-011). Layer 2 evolution tests (WASM-014).

#### Acceptance criteria
- [ ] A Wasm Component and a machine-code Component exchange a typed request and reply over `Channel<T>` generated from the same WIT world or mapped IDL.
- [ ] A WASI resource held by the guest is a Capability; a forged handle returns `Error::Rights` and allocates no handle.
- [ ] An import outside the accepted WASI subset fails closed at instantiate and creates no Component.
- [ ] `os inspect channel` shows both endpoints and the Interface identity.

#### Verification
- Unit: `wasm:tests/map/channel_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Integration: V1 Wasm-on-Channel prototype on H-001 and H-002.
- Demo: V1 Wasm Component on a native Channel on H-002.

#### Evidence
- none

### WASM-014 · Version WIT worlds across native Channel endpoints
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: WASM-013, IPC-042, IPC-038, IPC-052
- Baseline: §13, §12, §66
- Risks: R-005
- Invariants: I-041

V1 freezes Layer 2 evolution rules. Wasm Components honor the same forward and backward Channel compatibility tests as machine-code endpoints so WIT is not a second versioning story (§12, §66). IPC owns the rules; WASM exercises them on the host.

#### Out of scope
Evolution-rule freeze (IPC-042). Diff tool (IPC-052). V4 host version lock (WASM-022).

#### Acceptance criteria
- [ ] A newer Wasm guest with an unknown field is accepted by an older machine-code receiver; an older guest message is accepted by a newer machine-code receiver.
- [ ] The same pair of revisions passes when the Wasm side is the older endpoint.
- [ ] A breaking WIT world change is classified as breaking by IPC-052 and fails CI.
- [ ] Optional methods missing on one side return a typed unsupported result.

#### Verification
- Unit: `wasm:tests/versioning/wit_world_*` on CI matrix entry `qemu-x86_64`.
- Integration: forward and backward cases on H-001.

#### Evidence
- none

### WASM-015 · Host Wasm automation modules over typed Interface
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: WASM-012, WASM-013, WASM-008, SEM-018, SEM-013, SEM-007
- Baseline: §13, §45
- Risks: R-043
- Threats: T-001
- Invariants: I-021

V2 first-class automation modules: a Wasm module invoked over typed Interface holds only delegated Capability (§13, §45). SEM owns the rules Decision and registry; WASM hosts the module. Not an AI broker.

<!-- covers: INV-0269 -->

#### Out of scope
Automation-rule format Decision and registry (SEM). AI broker (SEM-010). Automation editor UI (APP-059).

#### Acceptance criteria
- [ ] A Wasm automation module bound to `Terminal.run` and `Editor.open` runs with only the delegated Capability set from SEM-013.
- [ ] A module that reaches for an undeclared Capability receives `Error::Rights` and allocates no handle.
- [ ] The module is a Wasm Component visible in `os inspect component` with kind and Capability table.
- [ ] No path from this host grants the AI broker Capabilities.

#### Verification
- Integration: `wasm:tests/automation/module_*` on H-002 and H-004.
- Review: SEM lead confirms registry-then-automation order and that the AI broker is not a dependency.

#### Evidence
- none

### WASM-016 · Instantiate downloaded Wasm Components from the store
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: WASM-012, WASM-013, APP-045, PKG-064, PKG-028, REL-007
- Baseline: §13, §28
- Threats: T-006
- Invariants: I-021

Instantiates downloaded Wasm Components once the V2 store client and signed repository exist (§13). Unsigned or undeclared-Capability modules are refused; grants match Package requested Capability. REL and APP own store UX; WASM instantiates.

<!-- covers: INV-0270 -->

#### Out of scope
Store client UI (APP-045). Repository protocol and signing (REL, PKG). Optional-capability degraded launch (CMP-043).

#### Acceptance criteria
- [ ] A signed Wasm Component Package from the repository instantiates with the requested Capability set from its manifest.
- [ ] An unsigned module is refused, allocates no Component, and the refusal is visible in the audit log.
- [ ] A module whose declared Capability set does not match the grant is refused with `Error::Rights`.
- [ ] `os inspect component` names the Package identity of a live downloaded instance.

#### Verification
- Integration: `wasm:tests/download/store_*` on H-002.
- Review: REL lead confirms signature verification stays in REL/PKG.

#### Evidence
- none

### WASM-017 · Run portable services as supervised Wasm Components
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: WASM-012, WASM-013, SVC-015, SVC-009, SVC-005, CMP-028
- Baseline: §13, §32
- Invariants: I-037

A long-running Wasm Component is SVC-supervised with crash-rebind of Channel clients, matching the V0.5 compositor pattern (§13, §32). SVC owns supervision; WASM owns the service Component kind.

<!-- covers: INV-0268 -->

#### Out of scope
Supervisor and restart policy (SVC). Client rebind stubs (IPC). Compositor service (GFX).

#### Acceptance criteria
- [ ] A supervised Wasm service is started from a service manifest as a Component with declared Capability.
- [ ] Killing the service causes clients to observe a typed disconnect, rebind by Interface identity and continue.
- [ ] Restart does not require clients to hold a new Capability for the rebound Interface.
- [ ] `os inspect service` names the Wasm Component instance and its restart generation.

#### Verification
- Integration: `wasm:tests/service/rebind_*` on H-002 and H-004.
- Review: SVC lead confirms restart policy is unchanged from machine-code services.

#### Evidence
- none

### WASM-018 · Ship architecture-independent Wasm Component Packages
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: WASM-012, WASM-004, PKG-031
- Baseline: §13, §38
- Invariants: I-001, I-046

§13 cross-architecture workloads as a portable format: one Wasm Component Package instantiates on every V2 x86-64 machine without per-arch native binaries. ARM64 native remains LATER; this does not port the kernel (I-001).

<!-- covers: INV-0271 -->

#### Out of scope
ARM64 kernel or userspace port (BLD, KRN). AOT image identity across rebuilders (WASM-023). Personality binaries (LNX, WIN).

#### Acceptance criteria
- [ ] One Wasm Component Package with no architecture-specific native binary instantiates on H-002, H-004 and H-005.
- [ ] The Package identity is the same on each machine.
- [ ] A Package that also ships only an x86-64 native Component is not required for the Wasm path to run.

#### Verification
- Integration: `wasm:tests/portable/cross_machine_*` on H-002, H-004 and H-005.

#### Evidence
- none

### WASM-019 · Debug Wasm Components with source-level breakpoints
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: WASM-012, WASM-005, SDK-038
- Baseline: §13, §64

V3 TypeScript/Wasm SDK bindings need a guest debug path. The native debugger is V1; this adds DWARF and name-section breakpoints inside a Wasm Component. Attach is a Capability. Native software does not see POSIX ptrace.

#### Out of scope
Native debugger attach Capability and async Task stacks (SDK-038). TypeScript language surface (SDK-085). Personality ptrace (LNX).

#### Acceptance criteria
- [ ] The debugger attaches to a Wasm Component through the same attach Capability as a machine-code Component and breaks on a DWARF source line in guest code.
- [ ] A name-section breakpoint binds to an exported WIT function and stops before the guest returns.
- [ ] No public native API names ptrace, pid or a POSIX process tracer.
- [ ] `os inspect component` shows the attached debugger Capability on the guest.

#### Verification
- Integration: `wasm:tests/debug/breakpoint_*` on H-002.
- Review: SDK lead confirms attach remains the V1 debugger Capability.

#### Evidence
- none

### WASM-020 · Publish WASI subset conformance for third-party Components
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: WASM-008, WASM-013, WASM-012
- Baseline: §13
- Risks: R-051

V3 public repository and third-party Packages. A documented WASI/WIT subset with a runnable suite so external Wasm Components instantiate under the host import Decision.

#### Out of scope
Public repository operation (REL-050). Author guide prose (WASM-021). Host version lock (WASM-022).

#### Acceptance criteria
- [ ] A published subset document lists every allowed WASI import and WIT world from WASM-008.
- [ ] The suite instantiates a third-party fixture Component that uses only the subset and exchanges a Channel message with a machine-code peer.
- [ ] A fixture that uses a forbidden Preview1 import fails instantiate with a typed error.
- [ ] CI runs the suite on H-001.

#### Verification
- Unit: `wasm:tests/conformance/wasi_subset_*` on CI matrix entry `qemu-x86_64`.
- Integration: third-party fixture on H-002.
- Review: IPC lead confirms the subset matches the import Decision.

#### Evidence
- none

### WASM-021 · Document Wasm Component authoring against native Interface
- Type: docs
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: WASM-004, WASM-008, WASM-013, WASM-020, DOC-032
- Baseline: §13, §52, §63
- Risks: R-052

V3 documentation gate for strangers shipping Packages. Layer 2/3 authoring guide for WIT worlds, Capability requests and Package artefacts. DOC owns L1 ABI pages; WASM owns this host guide.

#### Out of scope
L1 ABI reference pages (DOC, ABI). SDK TypeScript binding (SDK-085). Package format reference (PKG-061).

#### Acceptance criteria
- [ ] The guide documents WIT world authoring, requested Capability, Package artefact fields and the published WASI subset.
- [ ] The guide states that Wasm is not the Native ABI and that machine-code Components remain first-class.
- [ ] A Review line names who accepts the document.

#### Verification
- Review: DOC and SDK leads sign off on the pull request.

#### Evidence
- none

### WASM-022 · Lock Wasm host Interface versions with a conformance suite
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: WASM-002, WASM-003, WASM-001, WASM-007, WASM-008, WASM-014, WASM-020, IPC-022, IPC-068
- Baseline: §13, §66
- Freezes: S-029
- Invariants: I-046

V4 Layer 2 versions locked. Wasm host imports are Layer 2, not Layer 1, so this is not an L1 freeze. The suite proves binaries against the lock candidate still instantiate on later V4 builds. Spike and Decisions sit in the dependency closure.

#### Out of scope
Channel L1 freeze (IPC-064). Layer 2 evolution-rule freeze (IPC-042). ABI Layer 2 version register (ABI).

#### Acceptance criteria
- [ ] S-029 is `frozen` and names this task.
- [ ] Every locked host import and WIT world in the WASI subset has a conformance test; a binary built against the lock candidate instantiates on a later V4 build without rebuild.
- [ ] The closure contains WASM-002, WASM-008 and IPC-022.
- [ ] Adding a host import after lock fails CI without a superseding Decision.

#### Verification
- Unit: `wasm:tests/conformance/host_lock_*` on CI matrix entry `qemu-x86_64`.
- Integration: freeze-candidate binary replay on a subsequent V4 image.
- Review: ABI lead sign-off that S-029 freeze cites this task, the exploring spike and the import Decision.

#### Evidence
- none

### WASM-023 · Reproduce Wasm AOT artefacts bit-for-bit
- Type: build
- Milestone: V4
- Status: todo
- Size: S
- Owner: none
- Depends on: WASM-005, BLD-041, BLD-074
- Baseline: §13, §34
- Risks: R-060

V4 reproducible-builds gate applied to Wasm AOT images in Package. Two independent builders emit identical content-addressed artefacts for the same WIT world and module.

#### Out of scope
Independent rebuilder infrastructure (BLD-074). Store dedup (PKG). Public verifier tool (BLD-075).

#### Acceptance criteria
- [ ] Two independent builders emit the same content identity for one Wasm Component Package AOT artefact.
- [ ] A mismatch fails the V4 reproducibility job that consumes BLD-074.
- [ ] The artefact identity is the Package object identity PKG stores.

#### Verification
- Integration: double-build of a fixture Wasm Package on the V4 rebuilder pair.
- Review: BLD lead confirms the AOT path is in the generation reproducibility set.

#### Evidence
- none

### WASM-024 · Publish Wasm instantiation metrics against Linux Wasmtime
- Type: benchmark
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: WASM-006, BEN-060
- Baseline: §13, §54
- Benchmarks: B-047
- Invariants: I-061

1.0 requires every tracked metric published with no unmeasured claim. Re-runs B-047 on Tier 1 against the register baselines, including Linux Wasmtime. The harness landed at V1.

#### Out of scope
Harness implementation (WASM-006). Cross-OS dashboard publication (BEN-060). Methodology pack (BEN-063).

#### Acceptance criteria
- [ ] B-047 reports exist under `reports/benchmarks/B-047/` for every 1.0 hardware-scope H-ID.
- [ ] Each report includes B-001, B-004 and Linux Wasmtime component instantiation on the same machine.
- [ ] No 1.0 announcement cites a Wasm number except by B-047.
- [ ] The 1.0 target kind in the B-047 register is met (regression versus V4 per register).

#### Verification
- Bench: B-047 on every 1.0 hardware-scope H-ID; target per register.
- Review: BEN lead sign-off recorded on the pull request.

#### Evidence
- none
