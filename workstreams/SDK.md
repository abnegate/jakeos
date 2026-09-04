# SDK · Native SDK and developer tools
- Prefix: SDK
- Lead: none
- Baseline: §50, §52, §64, §66

<!-- roadmap:generated:begin summary -->
Tasks: 96 live, 0 done, 0 in-progress, 96 todo, 0 dropped. Ready: 1. Blocked: 95. Weighted: 0%.
<!-- roadmap:generated:end -->

## Scope

SDK owns the native userspace programming surface and the developer tools that sit on it. Native applications execute against the Native Platform layer (the SDK plus platform services) rather than the kernel directly (§4, §50, §52). The workstream ships the tiny native runtime that multiplexes Tasks over Operation completions, the primary Rust crate, the `#[component]` entry and Context Capability surface, Layer 3 crate layout and semver, the Layer 3 Rust `std` compatibility crate, language bindings in the recorded order, the `os` CLI family (`inspect`, `trace`, `history`, `restore`, `env`, `new`, `test`, `help`, `publish`, `bisect`, `profile`, `call`), debugger and editor integration, the native profiler, package-emission tooling from a cargo build, the host cross-compilation SDK, the Component test harness and Layer 2 compatibility suite, versioned toolchain packages, and the SDK samples and guides. Rust `std` lives only at Layer 3. Inventory prefix CLI is absorbed here.

## Out of scope

Layer 1 entry, handle encoding, error model and the generated C header (ABI). IDL compiler, wire format and generated stubs (IPC). Capability mint, derive, revocation and debug-attach rights (CAP). Component kernel object, graphs and launch path (CMP). Task, TaskGroup and Operation kernel objects (TSK). MemoryObject kernel backing (MEM). ResourceDomain budgets (SCH). Inspect and trace data plane, crash format and profiler sampling (OBS). Package store, manifests, SystemGenerations and the history log (PKG). Chooser authority and UserSelected minting (STO). UI protocol and Layer 4 toolkit (UIP). DevelopmentEnvironment objects (ENV). Semantic interface registry and AI broker (SEM). Wasm runtime host (WASM). Compositor and Surface objects (GFX). Service supervisor (SVC). Exploit-mitigation policy (SEC). Signing, repository and HCL publication (REL). Docs site generation (DOC). CI plumbing and the rustc-in-kernel pin (BLD). Benchmark methodology (BEN). License firewall (GOV). Linux and Windows personalities (LNX, WIN). ComputeDevice dispatch (HET). First-party applications (APP).

## Tasks

### SDK-001 · Implement the #[Component] entry macro and Context Capability Surface
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: SDK-004, SDK-009, CMP-014, CAP-005
- Baseline: §52
- Invariants: I-021

The `#[component]` attribute generates Component bootstrap, runtime start and tracing setup so the secure path is the default (§52). `Context` exposes exactly the Capabilities granted at launch; there is no ambient handle table and no filesystem or network API on Context.

<!-- covers: INV-0963, INV-0964 -->

#### Out of scope
Kernel Component object (CMP-014). Window and chooser methods on Context (SDK-023, SDK-017).

#### Acceptance criteria
- [ ] A crate with `#[component] async fn main(ctx: Context)` links the native runtime and starts tracing without a handwritten bootstrap.
- [ ] `Context` methods resolve only Capabilities present in the launch set; a missing kind returns `Error::Rights` and allocates no handle.
- [ ] Generated bootstrap registers the Component TaskGroup so spawned Tasks are owned (SDK-016).

#### Verification
- Unit: `sdk:tests/component_macro_*` on `qemu-x86_64` and `hw-h002`.
- Integration: V0 ImageDecoder sample starts through the macro on H-001.

#### Evidence
- none

### SDK-002 · Ship ImageDecoder as the V0 typed-service SDK sample
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: SDK-001, SDK-005, SDK-004, IPC-012, CMP-005, SCH-007
- Baseline: §11, §14, §52, §59

ImageDecoder is the reference typed service for the V0 demo: Input of image bytes, Output of a bitmap, ResourceDomain budgets from the §11 example, written against the Rust SDK and generated IDL stubs (§14, §59). The sample is a native Component, not a Linux process.

<!-- covers: INV-0236, INV-0280 -->

#### Out of scope
V0 demo orchestration (CMP-011). Isolated decode library used by PhotoEditor (SDK-013). ResourceDomain enforcement (SCH).

#### Acceptance criteria
- [ ] The sample crate builds for the native JSON target and runs as a Component on H-001 and H-002.
- [ ] Decode takes image bytes on a typed Channel and returns a MemoryObject bitmap by ownership transfer.
- [ ] `os inspect component` on the running sample names ImageDecoder, its Capabilities and the awaited Channel.
- [ ] The sample declares the §11 ResourceDomain budgets; exceeding them yields the SCH typed exhaustion error.

#### Verification
- Integration: `sdk:tests/samples/imagedecoder_*` on `qemu-x86_64` and `hw-h002`.
- Demo: V0-D01 pipeline uses this sample as Component B on H-002.

#### Evidence
- none

### SDK-003 · Declare a no_std rustc JSON target for native userspace
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: ABI-003, BLD-004, BLD-009
- Baseline: §3, §50
- Invariants: I-006, I-049, I-089

Native crates must not compile as Linux binaries or the §3 firewall is void. V0 ships a custom rustc JSON target (`x86_64-unknown-jakeos`) with `alloc` only, not a `std` port. The target is consumed by the SDK crate and the ImageDecoder sample.

<!-- covers: GAP-0090 -->

#### Out of scope
Layer 3 `std` crate (SDK-049). Upstream tier-3 proposal (SDK-090). Kernel rustc pin (BLD-013).

#### Acceptance criteria
- [ ] `sdk/targets/x86_64-unknown-jakeos.json` exists and builds `alloc` plus the SDK crate.
- [ ] CI compiling a native crate with the Linux gnu or musl target fails ABI-003.
- [ ] The target does not provide `std::fs`, `std::net` or `std::process`.

#### Verification
- Unit: `sdk:tests/target/json_target_*` on the Linux-host CI image.
- Integration: ImageDecoder sample builds only with the JakeOS JSON target in BLD-009.

#### Evidence
- none

### SDK-004 · Implement the tiny native runtime that multiplexes Tasks over Operations
- Type: build
- Milestone: V0
- Status: todo
- Size: L
- Owner: none
- Depends on: SDK-010, SDK-011, SDK-009, TSK-018, TSK-020, TSK-023
- Baseline: §18, §20, §52, §59
- Invariants: I-030

V0 exit requires a tiny native runtime with Channel and Capability wrappers sufficient to write the demo in Rust (§59). The executor drives futures from Operation completions and never wraps blocking Linux syscalls. Awaiting a Channel receive or MemoryObject read suspends only the Task, not the execution context (§20).

<!-- covers: INV-0360, INV-0382, INV-0977, INV-1167 -->

#### Out of scope
Kernel Operation ring (TSK-018). Executor Decision (SDK-010). Debugger Task stacks (SDK-038).

#### Acceptance criteria
- [ ] The runtime polls Operation completions and wakes the owning Task without a blocking Linux read on the hot path.
- [ ] Two Tasks in one TaskGroup make progress on one execution context while one Task awaits Receive.
- [ ] Cancelling the TaskGroup completes in-flight Operations with `Error::Cancelled` and starts no new work.
- [ ] Channel and Capability wrappers are the only kernel-entry types the V0 demo links.

#### Verification
- Unit: `runtime:tests/executor_*` on `qemu-x86_64` and `hw-h002`.
- Integration: V0-D01 ImageDecoder round trip runs on this runtime on H-001.
- Review: TSK lead confirms the runtime binds to S-005 without a second ring.

#### Evidence
- none

### SDK-005 · Expose Operation cancel, deadline and await on the Rust SDK
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: SDK-009, TSK-010, TSK-013, ABI-009
- Baseline: §19, §52

§19 is the SDK surface for outstanding work. `operation.cancel()`, `operation.deadline(...)` and `operation.await` are the V0 entry points; cancellation and deadline gates are unusable without them. Await integrates with the native runtime so the owning Task suspends until completion.

<!-- covers: INV-0369, INV-0370, INV-0371 -->

#### Out of scope
Kernel cancel and deadline (TSK-010). Executor (SDK-004).

#### Acceptance criteria
- [ ] `cancel()` on an outstanding Operation yields `Error::Cancelled` and never delivers an Ok result.
- [ ] `deadline(...)` on an outstanding Operation yields `Error::DeadlineExceeded` when the deadline passes first.
- [ ] `await` suspends only the owning Task and resumes on completion on `qemu-x86_64` and `hw-h002`.

#### Verification
- Unit: `sdk:tests/operation/cancel_*`, `deadline_*`, `await_*` on `qemu-x86_64` and `hw-h002`.
- Integration: TSK V0 acceptance suite drives the SDK wrappers rather than raw syscalls.

#### Evidence
- none

### SDK-006 · Implement the os CLI skeleton for inspect and trace
- Type: build
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: none
- Baseline: §64
- Invariants: I-034

§64 requires developer tooling from V0. The `os` binary is the home for inspect and trace and later env, history, restore, new, test, help, publish, bisect, profile and call. This task ships the binary, subcommand dispatch and Capability-gated invocation; subcommands land in sibling tasks.

<!-- covers: INV-1252 -->

#### Out of scope
Inspect rendering (SDK-007). Trace rendering (SDK-008). Package and generation subcommands (PKG).

#### Acceptance criteria
- [ ] `os --help` lists at least `inspect` and `trace` as subcommands from the V0 image.
- [ ] Invoking `os` without an inspect Capability returns `Error::Rights` and prints no object state.
- [ ] Unknown subcommands exit non-zero with a stable error code usable by scripts.

#### Verification
- Unit: `sdk:tests/cli/skeleton_*` on `qemu-x86_64`.
- Integration: V0 image contains the `os` binary on H-001.

#### Evidence
- none

### SDK-007 · Ship os inspect for every V0 Object kind
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: SDK-006, OBS-006, OBS-005, OBS-007
- Baseline: §24, §64

V0 exit prints Component, Task, Channel, Capability, MemoryObject and ResourceDomain state (§64). OBS owns the data plane; this task is the `os inspect` command that renders those records. Tooling does not reconstruct wait edges from scheduler traces when inspect already stores them.

<!-- covers: INV-1253, INV-1254, INV-1255, INV-1256, INV-1257, INV-1258 -->

#### Out of scope
Inspect Interface and providers (OBS-006, OBS-005, OBS-007). ComputeQueue inspect (SDK-065). JSON output (SDK-035).

#### Acceptance criteria
- [ ] `os inspect component|task|channel|capability|memory|resource` prints owner, relationships and the §64 field set for a live object.
- [ ] Inspect of a Task waiting on Receive names the Channel type rather than a numeric opcode alone.
- [ ] A caller without inspect rights receives `Error::Rights` and an empty listing.

#### Verification
- Integration: V0-G10 listing of all six kinds on H-001 and H-002.
- Demo: V0-D01 `os inspect component` on H-002 matches the §64 ImageDecoder example shape.
- Unit: `sdk:tests/cli/inspect_*` on `qemu-x86_64`.

#### Evidence
- none

### SDK-008 · Ship os trace with structured dynamically enabled tracing
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: SDK-006, OBS-011, OBS-003
- Baseline: §24, §64

V0 demo and the tracing-overhead gate need `os trace` over the OBS substrate (§24, §64). Enablement is dynamic and Capability-gated; disabled scopes are OBS's problem, this command only requests and renders them.

<!-- covers: INV-1259 -->

#### Out of scope
Trace ring (OBS-011). Overhead measurement (OBS-001). Offline export (SDK-022).

#### Acceptance criteria
- [ ] `os trace` on the V0 demo shows Channel send and Operation complete events named in primitive terms.
- [ ] Enable and disable take effect on a running session without restarting the traced Component.
- [ ] A caller without trace rights receives `Error::Rights` and no event payload.

#### Verification
- Demo: V0-D01 `os trace` displays the ImageDecoder flow on H-002.
- Integration: `sdk:tests/cli/trace_*` on `qemu-x86_64` and `hw-h002`.
- Bench: B-012 sessions are started with this command; OBS publishes the report.

#### Evidence
- none

### SDK-009 · Implement the primary Rust SDK crate over the Native ABI
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: SDK-003, ABI-007, ABI-009, ABI-002, CAP-003, MEM-005, IPC-010
- Baseline: §4, §16, §50, §52
- Invariants: I-005, I-063

Native applications execute against the SDK rather than the kernel (§4, §50). The crate maps kernel failures to typed errors (`Error::Rights`, `Error::Cancelled`, `Error::Revoked`, `Error::Disconnected`, `Error::DeadlineExceeded`) and defaults MemoryObject APIs to move semantics (§16, §52).

<!-- covers: INV-0937, INV-0104, INV-0982, INV-0316 -->

#### Out of scope
Runtime executor (SDK-004). C binding (SDK-033). `std` facade (SDK-049).

#### Acceptance criteria
- [ ] The crate builds for the native JSON target and is the only userspace dependency of the V0 demo.
- [ ] Moving a MemoryObject invalidates the source handle; a subsequent use returns `Error::Revoked`.
- [ ] Kernel failure codes surface as the typed error enum, never as errno.
- [ ] No public SDK function is a thin wrapper over a Linux syscall (I-005).

#### Verification
- Unit: `sdk:tests/error_model_*`, `memoryobject_move_*` on `qemu-x86_64` and `hw-h002`.
- Integration: ImageDecoder sample links only this crate plus generated IDL stubs.

#### Evidence
- none

### SDK-010 · Decide the userspace executor shape for the native runtime
- Type: adr
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: SDK-011, TSK-009
- Baseline: §18, §20, §52, §58
- Decision: D-0258

V0 tiny-runtime gate needs a recorded choice among a custom executor over Operation completions, a Tokio subset with an Operation reactor, and an embassy-style executor before the runtime is built. CMP owns the kernel and runtime split; this Decision owns only the userspace executor.

#### Out of scope
Kernel Task mapping (TSK-009). Runtime implementation (SDK-004).

#### Acceptance criteria
- [ ] Option A (custom executor), option B (Tokio subset over Operations), and option C (embassy-style executor) are evaluated against SDK-011.
- [ ] The Decision states that the executor never wraps blocking Linux syscalls and names the waker contract TSK-045 later documents.
- [ ] TSK lead records Review sign-off on the pull request.

#### Verification
- Review: TSK and SDK leads sign off on the pull request; Evidence will contain `decision:<D-ID>` when the file is accepted.

#### Evidence
- none

### SDK-011 · Study Rust async and Tokio for mapping Operations onto SDK futures
- Type: spike
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: TSK-014, TSK-016
- Baseline: §18, §20, §58

V0 runtime cannot wrap blocking Linux syscalls. This spike studies Rust Future, Pin and Waker, and Tokio work-stealing, task budgets and cooperative scheduling, as inputs to SDK-010 and the tiny native runtime used by the V0 demo (§58).

<!-- covers: INV-1147, INV-1148 -->

#### Out of scope
Executor Decision (SDK-010). Production runtime (SDK-004).

#### Acceptance criteria
- [ ] `reports/spikes/SDK-011.md` exists with the Spike skeleton headings.
- [ ] The report maps Operation completion onto Waker without a blocking Linux syscall on the wait path.
- [ ] The report compares custom, Tokio-subset and embassy-style executors and recommends an option set without selecting it.

#### Verification
- Report: answers how Future/Pin/Waker map onto Operation completions; whether Tokio work-stealing can run over TSK multiplexing without hidden blocking; which executor options remain after rejecting a blocking-syscall reactor; recommended option set for SDK-010.
- Review: TSK lead records that S-005 is not forked by the prototype.

#### Evidence
- none

### SDK-012 · Implement Channel disconnect, rebind, retry and restore-state
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: SDK-009, IPC-028, SVC-009, TSK-035
- Baseline: §32, §52
- Invariants: I-037

V0.5 compositor-restart gate needs client libraries that detect disconnect, rebind by Interface name, retry idempotent Operations and restore session state (§32). IPC generates the stub hooks; this task is the SDK runtime that applications link.

<!-- covers: INV-0600, INV-0601, INV-0602, INV-0603 -->

#### Out of scope
Generated proxy hooks (IPC-028). Supervisor re-advertise (SVC-009). Compositor Surface rebind (GFX-009).

#### Acceptance criteria
- [ ] Peer death surfaces as `Error::Disconnected` on the next Operation and does not exit the client Component.
- [ ] Rebind resolves a new endpoint by Interface name, not by the dead endpoint identity.
- [ ] Idempotent Operations retry once after rebind with the Interface's declared semantics.
- [ ] Restore-state re-establishes session objects (surfaces, streams, subscriptions) declared by the Interface.

#### Verification
- Integration: compositor kill-and-rebind loop on H-001 and H-003 with the SDK client library.
- Unit: `sdk:tests/rebind_*` on `qemu-x86_64`.

#### Evidence
- none

### SDK-013 · Provide an isolated ImageDecoder library on the SDK decode path
- Type: build
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: SDK-002, SDK-014
- Baseline: §11, §52

§52 `decode(image)` runs as an isolated Component so the PhotoEditor sample does not parse untrusted bytes in the UI Component (§11). The library is the SDK path that spawns or binds that Component and returns a bitmap MemoryObject.

<!-- covers: INV-0967 -->

#### Out of scope
ImageDecoder sample itself (SDK-002). PhotoEditor app chrome (APP-005). In-address-space class (CMP).

#### Acceptance criteria
- [ ] `decode(image)` runs in a Component distinct from the caller; `os inspect` shows two Components.
- [ ] The decoder Component holds Input of image bytes and Output of a bitmap and no network or directory Capability.
- [ ] A malformed image fails the decoder Component without aborting the caller.

#### Verification
- Integration: `sdk:tests/decode_isolated_*` on `qemu-x86_64` and H-003.
- Unit: denial test that the decoder cannot open a Channel it was not granted.

#### Evidence
- none

### SDK-014 · Make SDK spawn create a new Component by default
- Type: build
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: SDK-009, CMP-021
- Baseline: §10, §52
- Invariants: I-029

§10 cheap isolation: SDK defaults spawn a separate Component without an opt-in sandbox flag so isolate-it is the path of least resistance (I-029). Sharing an address space, if the CMP Decision allows it, is an explicit opt-in, never the default.

<!-- covers: INV-0229 -->

#### Out of scope
Component spawn primitive (CMP-009). In-address-space class (CMP-038).

#### Acceptance criteria
- [ ] `spawn` without flags creates a new Component; `os inspect` shows a new object identity.
- [ ] There is no SDK flag named sandbox whose default is off.
- [ ] An opt-in same-address-space spawn, if offered, is a distinct API that the ergonomics lint flags in default templates.

#### Verification
- Unit: `sdk:tests/spawn_default_isolate_*` on `qemu-x86_64`.
- Integration: decode path (SDK-013) uses the default spawn.

#### Evidence
- none

### SDK-015 · Provide a developer command that builds, boots QEMU and runs CI tests
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: SDK-006, BLD-009, BLD-012, BLD-006
- Baseline: §56.5, §64
- Benchmarks: B-050

Contributor onboarding wraps BLD's harness behind one SDK command so a fresh clone builds, boots QEMU and runs the same tests CI uses. The time target lives in B-050, not in this task.

<!-- covers: GAP-0152 -->

#### Out of scope
Hermetic Linux-host build (BLD-009). Onboarding measurement (SDK-032). Native host SDK packages (SDK-039).

#### Acceptance criteria
- [ ] `os dev` (or the recorded subcommand) on a Linux host builds the image, boots H-001 and runs the V0.5 guest tests.
- [ ] The command invokes BLD's harness rather than a second QEMU wrapper.
- [ ] Failure prints the guest-agent test ID; a passing run exits zero.

#### Verification
- Integration: Linux-host CI job runs the command against H-001.
- Manual: a contributor follows the recorded steps from a clean clone.

#### Evidence
- none

### SDK-016 · Lint SDK defaults for Capabilities, Tasks, async I/O and typed IPC
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: SDK-009, ABI-003, ABI-018, TSK-001
- Baseline: §3, §13, §51, §52, §57
- Threats: T-001
- Invariants: I-006, I-013, I-021, I-026, I-029, I-030, I-031, I-046, I-049, I-063

One CI lint and test gate collapses the standing SDK invariants: no ambient APIs, no POSIX-shaped native surface, no forced Wasm, drop-safe handles, automatic tracing and cancellation, move semantics for MemoryObjects, and unsafe remaining possible but isolated (§3, §52, §57). Every ergonomics regression of these defaults is a failed lint, not a documentation note.

<!-- covers: INV-0101, INV-0273, INV-0955, INV-0962, INV-0969, INV-0970, INV-0971, INV-0972, INV-0973, INV-0974, INV-0975, INV-0976 -->

#### Out of scope
Layer 1 POSIX-name lint (ABI-018). Capability ambient-authority lint on system Components (CAP-016).

#### Acceptance criteria
- [ ] A native SDK crate that exposes path-open, blocking I/O, or an untyped send fails the lint in CI.
- [ ] Dropping a Capability handle releases it; a second drop or use-after-revoke returns `Error::Revoked` and panics no kernel path.
- [ ] Machine-code compilation of a native crate remains accepted; a crate that requires Wasm as the only target fails the lint (I-046).
- [ ] `unsafe` is permitted inside an explicit module allowlist and nowhere else in default templates.

#### Verification
- Unit: `sdk:tests/lint/ergonomics_*` on `qemu-x86_64`.
- Integration: CI gate `sdk-ergonomics` fails a fixture crate per banned API class.

#### Evidence
- none

### SDK-017 · Implement files.choose returning a typed UserSelected Capability
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: SDK-001, STO-018
- Baseline: §9.1, §25, §52
- Threats: T-001, T-002
- Invariants: I-021, I-035

V0.5 chooser gate: Image Viewer starts without filesystem authority and receives exactly one UserSelected grant (§9.1, §25). STO owns the chooser mint; SDK owns `ctx.files.choose::<T>()`.

<!-- covers: INV-0486, INV-0966 -->

#### Out of scope
UserSelected mint and audit (STO-034). Chooser UI (APP-002). Isolation test (STO-007).

#### Acceptance criteria
- [ ] `files.choose::<Image>()` returns `UserSelected<Image>` after the user selects an object.
- [ ] The calling Component holds no Capability to the containing directory before or after the call.
- [ ] Cancelling the chooser returns a typed error and grants nothing.
- [ ] A Component without the chooser Context method compiled in cannot invoke it.

#### Verification
- Integration: V0.5 Image Viewer scenario on H-003 and H-002.
- Unit: `sdk:tests/files_choose_*` on `qemu-x86_64`.
- Demo: audit log shows exactly one `Capability<Image, Read>` granted.

#### Evidence
- none

### SDK-018 · Lay out Layer 3 SDK crates under semantic versioning
- Type: build
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: SDK-009, ABI-028, SDK-027
- Baseline: §52, §66

Layer 3 is the native SDK and may evolve quickly (§66). Crate layout and semver numbering start while the API is still explicitly unstable so V0.5 apps depend on named crates rather than a monopath.

<!-- covers: INV-1286 -->

#### Out of scope
V1 semver policy Decision (SDK-054). Symbol classification (ABI-028).

#### Acceptance criteria
- [ ] Published crate names and versions exist for runtime, SDK and macros with a pre-1.0 semver.
- [ ] Each crate README states the API is unstable and not SDK v1.
- [ ] ABI-028 classifies these crates as Layer 3.

#### Verification
- Review: ABI lead confirms Layer 3 classification.
- Unit: `sdk:tests/layer3_layout_*` checks crate graph and version metadata.

#### Evidence
- none

### SDK-019 · Implement os history listing SystemGeneration events
- Type: build
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: SDK-006, PKG-022, PKG-023
- Baseline: §31, §64

§31 and §64 CLI for history events. PKG records the log; SDK prints timestamped human-readable entries. Restore of user data stays out of V0.5.

<!-- covers: INV-0572, INV-1260 -->

#### Out of scope
History log storage (PKG-022). Generation restore (SDK-044). Environment events (PKG-053).

#### Acceptance criteria
- [ ] `os history` lists recorded events with a timestamp and a human-readable description.
- [ ] Entries include Package install and OS update events emitted by PKG-023.
- [ ] A caller without history-read rights receives `Error::Rights` and no event list.

#### Verification
- Integration: install a local Package, then `os history` shows the event on H-003.
- Unit: `sdk:tests/cli/history_*` on `qemu-x86_64`.

#### Evidence
- none

### SDK-020 · Emit an immutable Package with manifest from a native cargo build
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: SDK-018, SDK-026, PKG-035, PKG-031, PKG-012
- Baseline: §28, §52, §34
- Invariants: I-036, I-039

V0.5 packages are installed from local files. SDK build tooling produces the manifest and content-addressed Package that PKG stores (§28). Cargo integration is the developer path; `os package build` remains PKG's verb.

<!-- covers: INV-0543, INV-0986 -->

#### Out of scope
Store and install (PKG-025). `os package build` (PKG-035). Linking Decision (SDK-026).

#### Acceptance criteria
- [ ] `cargo` build of a native crate emits an immutable Package whose identity matches PKG's content-addressed scheme.
- [ ] The Package manifest lists Components, requested Capabilities and Interfaces.
- [ ] Two builds of identical content yield the same Package identity.
- [ ] Launch maps Package objects; the build emits no path-based loader script (I-039).

#### Verification
- Integration: four V0.5 apps package through this path on H-003.
- Unit: `sdk:tests/package_emit_*` on the Linux-host builder.

#### Evidence
- none

### SDK-021 · Ship the PhotoEditor Capability sample and its denial tests
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: SDK-017, SDK-023, SDK-013, SDK-020, CAP-025
- Baseline: §9.1, §52
- Threats: T-001
- Invariants: I-021

§9.1 worked example must ship as an SDK sample and integration test proving denied Capabilities are absent. PhotoEditor starts with UI, GPU and temporary-storage only; the user grant is exactly one image.

<!-- covers: INV-0219 -->

#### Out of scope
Image Viewer product (APP-005). Chooser mint (STO-034).

#### Acceptance criteria
- [ ] The sample launches with UI, GPU and temporary-storage Capabilities only.
- [ ] After `files.choose::<Image>()`, `os inspect capability` lists exactly one image grant.
- [ ] Attempts to open a sibling file, the network, or the camera return `Error::Rights` and allocate no handle.
- [ ] The denial is present in the Capability audit log.

#### Verification
- Integration: `sdk:tests/samples/photoeditor_denial_*` on H-003 and H-002.
- Demo: V0.5 chooser demo can run this sample in place of Image Viewer for the grant line.

#### Evidence
- none

### SDK-022 · Export os trace timelines of Component, Task, Channel and Operation
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: SDK-008, OBS-015, OBS-025
- Baseline: §24, §64

V0.5 needs a viewer or exporter over OBS traces so developers can inspect IPC and scheduling without a custom GUI yet. SDK ships the exporter; OBS owns the schema Decision.

<!-- covers: INV-0475 -->

#### Out of scope
Export format Decision (OBS-015). Offline session packaging (SDK-051). Trace ring (OBS).

#### Acceptance criteria
- [ ] `os trace export` writes a timeline that names Component, Task, Channel and Operation.
- [ ] The file uses the format chosen by OBS-015.
- [ ] A filtered export contains only the enabled primitives.

#### Verification
- Integration: export of the compositor-rebind session on H-003 opens in the recorded viewer.
- Unit: `sdk:tests/cli/trace_export_*` on `qemu-x86_64`.

#### Evidence
- none

### SDK-023 · Implement ctx.ui.window and window.render on Context
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: SDK-001, UIP-013, GFX-042, GFX-038, SDK-012
- Baseline: §41, §52, §60

V0.5 SDK expands to windows. UIP owns the protocol; GFX owns Surface; SDK exposes the §52 Context surface the four apps call. `window.render(bitmap)` presents a Buffer to a Surface via the native UI protocol, not Wayland.

<!-- covers: INV-0965, INV-0968 -->

#### Out of scope
UI protocol IDL (UIP-013). Object<Surface> (GFX-038). Toolkit widgets (UIP).

#### Acceptance criteria
- [ ] `ctx.ui.window()` returns a window object only when the Component holds `Capability<UI>`.
- [ ] `window.render(bitmap)` presents the bitmap as a Buffer on the Surface and completes as an Operation.
- [ ] Missing UI Capability returns `Error::Rights` and creates no Surface.
- [ ] After compositor restart, the SDK client rebinds the window without the application exiting.

#### Verification
- Integration: Text Editor and Image Viewer open a window through this API on H-003 and H-002.
- Unit: `sdk:tests/ui/window_*` on `qemu-x86_64`.

#### Evidence
- none

### SDK-024 · Decide SDK language binding order and milestones
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: ABI-007
- Baseline: §50
- Decision: D-0255

Records Rust at V0, C at V1, C++ at V2 and remaining languages at V3 so later binding tasks do not invent a different ladder (§50). Options may keep that order, pull C++ into V1, or delay C to V2; the Decision is the order, not a binding implementation.

<!-- covers: INV-0948 -->

#### Out of scope
C binding (SDK-033). Non-Rust ABI mapping (SDK-072). IDL backends (IPC).

#### Acceptance criteria
- [ ] Option A (Rust V0, C V1, C++ V2, others V3), option B (C++ pulled to V1), and option C (C delayed to V2) are evaluated against §50 and the V1 C-binding gate.
- [ ] The accepted option names the milestone of each language this workstream later implements.
- [ ] ABI lead records Review sign-off on the pull request.

#### Verification
- Review: ABI and SDK leads sign off on the pull request.

#### Evidence
- none

### SDK-025 · Decide POSIX-Personality shell versus a native Object-aware shell
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: SDK-006, TSK-023, IPC-010
- Baseline: §3, §21, §42, §64
- Decision: D-0256
- Invariants: I-006

Fixes how the `os` CLI, PTYs and job control map onto Tasks and Channels before V1 daily-driving. A POSIX shell remains a Linux personality, not the native API (I-006). Options: POSIX shell only inside LNX, a native object-aware shell that speaks typed Interfaces, or both with `os` as the native command surface.

<!-- covers: GAP-0309 -->

#### Out of scope
PTY bridge in Terminal (APP-009). Terminal-session authority (LNX-022). Semantic CLI (SDK-066).

#### Acceptance criteria
- [ ] Option A (POSIX shell only in the Linux personality), option B (native object-aware shell over Tasks and Channels), and option C (both, with `os` as the native surface) are evaluated.
- [ ] The Decision states how job control maps onto TaskGroup cancellation and that native software never sees POSIX job-control signals.
- [ ] APP and LNX leads record Review sign-off on the pull request.

#### Verification
- Review: APP, LNX and SDK leads sign off on the pull request.

#### Evidence
- none

### SDK-026 · Decide the native linking model and reject path-based loaders
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: SDK-030, PKG-012
- Baseline: §34, §53
- Decision: D-0259
- Invariants: I-039

§34 and §53 nongoals: no path-based loader lookup and no global constructors on the launch critical path. Options are static linking, content-addressed shared objects, or both. The Decision is accepted before Package emission hardens a loader.

<!-- covers: INV-0551, INV-1010, INV-1012 -->

#### Out of scope
Cost spike (SDK-030). Address-space mapping (CMP-017). Package format (PKG).

#### Acceptance criteria
- [ ] Option A (static only), option B (content-addressed shared objects), and option C (both with recorded defaults) are evaluated against SDK-030 and B-016.
- [ ] The Decision rejects path-based `ld.so`-style lookup as the native launch path (I-039).
- [ ] The Decision states how global constructors are kept off the launch critical path.
- [ ] PKG and CMP leads record Review sign-off on the pull request.

#### Verification
- Review: PKG, CMP and SDK leads sign off on the pull request.

#### Evidence
- none

### SDK-027 · Decide the license of the native SDK, runtime and language bindings
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: GOV-003, ABI-029, IPC-005
- Baseline: §50, §52
- Decision: D-0262

Third-party native applications begin at V0.5. This Decision picks the license of the SDK, runtime and language bindings, states that proprietary native applications are permitted, and sits beside GOV's layer firewall and the ABI header-exception Decision.

<!-- covers: GAP-0006 -->

#### Out of scope
Layer firewall (GOV-003). ABI header exception (ABI-029). IDL output licence (IPC-005).

#### Acceptance criteria
- [ ] Option A (permissive Apache-2.0 or MIT), option B (weak copyleft such as MPL), and option C (GPL with an SDK exception) are evaluated against GOV-003.
- [ ] The accepted option states that proprietary native applications are permitted.
- [ ] GOV lead records Review sign-off on the pull request.

#### Verification
- Review: GOV and SDK leads sign off on the pull request.

#### Evidence
- none

### SDK-028 · Decide that Rust std lives only as a Layer 3 crate
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: ABI-011, SDK-003, ABI-018
- Baseline: §3, §52, §65, §66
- Decision: D-0264
- Invariants: I-013, I-026, I-040

`std` filesystem, net and process APIs must not justify Layer 1 POSIX shapes (I-013, I-026). This Decision precedes the V1 `std` crate and forbids `std`-driven Native ABI entries. Options: Layer 3 facade over the SDK, no `std` on the native target, or a hybrid with explicit allowlisted modules.

<!-- covers: EXTRA-045 -->

#### Out of scope
`std` crate implementation (SDK-049). Layer 1 scope (ABI-011).

#### Acceptance criteria
- [ ] Option A (Layer 3 facade over the SDK), option B (no `std` on the native target), and option C (allowlisted `std` modules with no new L1 entries) are evaluated.
- [ ] The accepted option states that no Layer 1 entry point may be added to make `std`'s POSIX-shaped APIs work.
- [ ] ABI lead records Review sign-off on the pull request.

#### Verification
- Review: ABI and SDK leads sign off on the pull request.

#### Evidence
- none

### SDK-029 · Document the unstable V0.5 SDK used by the four native apps
- Type: docs
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: SDK-023, SDK-017, SDK-020, SDK-018
- Baseline: §52, §60

V0.5 scope says the SDK expands to windows, files and packages and remains unstable. The four apps need a crate-level guide before v1 so APP authors are not reading source as the contract.

#### Out of scope
V1 SDK guide (SDK-056). Docs site (DOC).

#### Acceptance criteria
- [ ] A crate-level guide covers Context, windows, `files.choose`, Packages and the unstable warning.
- [ ] The guide states the API is not SDK v1 and may break.
- [ ] The four V0.5 apps are the worked examples.

#### Verification
- Review: APP and DOC leads sign off on the pull request.

#### Evidence
- none

### SDK-030 · Measure static versus content-addressed shared-Object linking cost
- Type: spike
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: SDK-009, BEN-009, Q-001
- Baseline: §34, §53, §58
- Benchmarks: B-016

The linking Decision needs measured startup sharing before Package emission hardens a loader. This spike compares static linking and content-addressed shared objects on the B-016 path and does not restate a number in prose.

<!-- covers: INV-0551 -->

#### Out of scope
Linking Decision (SDK-026). Warm-startup publication (CMP-019).

#### Acceptance criteria
- [ ] `reports/spikes/SDK-030.md` exists with the Spike skeleton headings.
- [ ] The report publishes B-016 for static and content-addressed shared-object layouts on H-002.
- [ ] The report recommends an option set for SDK-026 without selecting it.

#### Verification
- Report: answers relative warm-start cost on B-016; code-page sharing behavior; constructor cost on the launch path; recommended option set for the linking Decision.
- Bench: B-016 prototype numbers on H-002 attached as Spike Evidence, not as a Gate.

#### Evidence
- none

### SDK-031 · Prototype the SDK v1 crate Surface used by daily-driving apps
- Type: spike
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: SDK-018, SDK-009
- Baseline: §50, §52, §66
- Explores: S-031

Prototype the Rust SDK crate surface that Terminal, Files, Editor and Image Viewer actually import so SDK-055 is not a paper Decision (§50, §52). Surface S-031 remains open. Rust `std` is not justified as a Layer 1 entry.

#### Out of scope
The Decision (SDK-055). Semver policy (SDK-054). Freeze of S-031 (SDK-057). C bindings (SDK-033).

#### Acceptance criteria
- [ ] A prototype crate set builds the four V0.5 apps without importing personality libraries.
- [ ] The spike report lists every public module those apps use, mapped to a proposed v1 crate.
- [ ] Surface S-031 remains `open` or `prototyped`, never `frozen`.

#### Verification
- Report: which crates are the v1 candidate, which items are host-only, and which options SDK-055 must evaluate.
- Integration: the four V0.5 apps build against the prototype on `qemu-x86_64`.

#### Evidence
- none

### SDK-032 · Measure timed onboarding from SDK install to a hello-Component
- Type: benchmark
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: SDK-059, SDK-039, SDK-043, SDK-015, DOC-016, Q-001
- Baseline: §52, §54, §56.5
- Benchmarks: B-050
- Invariants: I-061

Onboarding targets live in B-050. This harness times host-OS sessions each release from SDK install to a running hello-Component, and from a fresh clone to a booted QEMU image running the CI tests, without stating a number in prose.

<!-- covers: GAP-0454 -->

#### Out of scope
Methodology (Q-001, BEN-028). Host SDK packaging (SDK-039). Getting-started tutorial (DOC-016).

#### Acceptance criteria
- [ ] A committed B-050 report exists for Linux-host sessions covering SDK-install-to-hello and clone-to-QEMU.
- [ ] macOS and Windows host sessions are measured via the containerised Linux environment required by I-090.
- [ ] No onboarding-time claim appears outside the B-050 report (I-061).

#### Verification
- Bench: B-050 on H-001; target per register (V1 publish).
- Review: BEN lead confirms the harness matches registers/benchmarks.md.

#### Evidence
- none

### SDK-033 · Provide the C SDK binding over the Native ABI
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: SDK-024, SDK-009, ABI-023, IPC-048, TSK-045
- Baseline: §50, §52

V1 scope is SDK v1 plus C bindings for Layer 1. ABI owns the header; SDK owns safe C wrappers and IDL stubs. Native C programs call the Native ABI through these wrappers, never POSIX.

<!-- covers: INV-0938 -->

#### Out of scope
Generated C header (ABI-023). IDL C codegen (IPC-048). Host header packages (SDK-034).

#### Acceptance criteria
- [ ] A C program built against the wrappers creates a Component, submits an Operation and transfers a MemoryObject on H-001.
- [ ] Typed errors are C enums matching the SDK error model, not errno.
- [ ] The wrappers do not expose Linux syscall numbers or POSIX types on the native path.

#### Verification
- Integration: `sdk:tests/c/binding_*` on `qemu-x86_64` and `hw-h002`.
- Unit: error-enum and handle-drop tests in `sdk:tests/c/handles_*`.

#### Evidence
- none

### SDK-034 · Package C headers and stubs for host and native toolchains
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: SDK-033, SDK-039, IPC-048
- Baseline: §50

C bindings are unusable from the host SDK without packaged headers and IDL-generated stubs on Linux, macOS and Windows toolchains. macOS and Windows hosts use the containerised Linux environment (I-090) plus these headers for cross-compilation.

#### Out of scope
C wrapper implementation (SDK-033). Host SDK distribution (SDK-039).

#### Acceptance criteria
- [ ] The host SDK package includes C headers and IDL-generated stubs for the native target.
- [ ] A Linux-host compile of the C hello-Component uses only those headers.
- [ ] Header install paths are identical between host SDK and native JakeOS toolchain packages.

#### Verification
- Integration: host-SDK C hello-Component on the Linux builder.
- Unit: `sdk:tests/c/headers_packaged_*`.

#### Evidence
- none

### SDK-035 · Add machine-readable os CLI output and shell completions
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: SDK-006, SDK-007, SDK-025
- Baseline: §64

§64 scripts and later automation need JSON output and completions on the `os` CLI family. Human text remains the default; JSON is an explicit flag.

<!-- covers: INV-1267 -->

#### Out of scope
Semantic `os call` (SDK-066). Docs CLI reference (DOC-013).

#### Acceptance criteria
- [ ] `os inspect --json` emits a schema-stable object for each V0 kind.
- [ ] `os --completions bash|zsh|fish` writes completion scripts covering shipped subcommands.
- [ ] JSON mode on an error writes a typed error object to stderr and exits non-zero.

#### Verification
- Unit: `sdk:tests/cli/json_*` and `completions_*` on `qemu-x86_64`.
- Integration: a script drives `os inspect --json` during the V1 demo on H-004.

#### Evidence
- none

### SDK-036 · Build the SDK compatibility suite across adjacent generations
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: SDK-054, ABI-033, IPC-042
- Baseline: §66
- Invariants: I-041

V1 SDK v1 gate: applications built against v1.0.0 run on later v1.x generations and the reverse where promised, enforcing Layer 2 versioning. The suite is the Layer 3 half of that proof; ABI owns Layer 1 cases.

<!-- covers: GAP-0100 -->

#### Out of scope
CI job plumbing (BLD-037). Layer 1 conformance (ABI-033). 1.0 soak (SDK-094).

#### Acceptance criteria
- [ ] A fixture application built against SDK v1.0.0 runs on a later v1.x SystemGeneration in CI.
- [ ] A fixture built against a later v1.x SDK runs on the v1.0.0 generation where the policy promises it.
- [ ] A breaking Layer 3 change without a semver major bump fails the suite.

#### Verification
- Integration: BLD-037 runs this suite on H-001.
- Unit: `sdk:tests/compat/adjacent_gen_*`.

#### Evidence
- none

### SDK-037 · Ship a Debug Adapter Protocol adapter for native Components
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: SDK-052, SDK-060, SDK-038, CAP-032
- Baseline: §52, §61

Developers keep their editors. DAP is the V1 integration for breakpoints and Task inspection once the protocol Decision is accepted. Attach remains a Capability, not same-uid ptrace.

<!-- covers: GAP-0457 -->

#### Out of scope
Protocol Decision (SDK-052). Debugger core (SDK-038). Editor remote (SDK-047).

#### Acceptance criteria
- [ ] A DAP client sets a breakpoint at a source line inside an async Task and hits it on H-001.
- [ ] The adapter reports the logical Task stack from TSK-038.
- [ ] Attach without `Capability<Debug>` returns `Error::Rights` and starts no session.

#### Verification
- Integration: `sdk:tests/dap/breakpoint_async_*` on `qemu-x86_64` and H-004.
- Manual: VS Code DAP session against the hello-Component.

#### Evidence
- none

### SDK-038 · Attach a debugger to a Component with async Task stacks
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: SDK-052, SDK-060, TSK-038, CAP-032, OBS-027, BLD-038
- Baseline: §52, §61
- Risks: R-011

V1 exit: debugger attaches, breaks inside an async Task, shows the logical stack, and symbolises crash dumps. Attach is a Capability, not same-uid ptrace. OBS captures dumps; BLD publishes symbols.

<!-- covers: INV-0984, INV-1208 -->

#### Out of scope
DAP adapter (SDK-037). Debug Capability type (CAP-032). Crash format (OBS-027). Personality ptrace (LNX).

#### Acceptance criteria
- [ ] Debugger attach to a native Component stops at a source line inside an async Task on H-001 and H-004.
- [ ] The logical Task stack is shown; a kernel thread listing is not the default view.
- [ ] A crash dump from OBS symbolises against BLD's content-hashed symbols.
- [ ] Attach without debug rights returns `Error::Rights` and does not pause the Component.

#### Verification
- Integration: `sdk:tests/debug/attach_async_*` on `qemu-x86_64` and `hw-h004`.
- Demo: V1 debugger session on H-004.
- Review: CAP lead confirms attach is Capability-gated.

#### Evidence
- none

### SDK-039 · Ship a cross-compilation SDK and emulator harness on host OSes
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: SDK-003, SDK-009, SDK-043, BLD-009, BLD-012, BLD-047
- Baseline: §50, §52
- Invariants: I-090

First native apps are built from Linux, macOS and Windows hosts; the SDK plus QEMU harness must not require installing JakeOS to compile. Native Windows and macOS host toolchains are not provided before 1.0 (I-090); those hosts use the containerised Linux environment plus this SDK.

<!-- covers: GAP-0453 -->

#### Out of scope
Containerised Linux env for non-Linux hosts (BLD-009). Toolchain manager (SDK-083). Remote editor (SDK-047).

#### Acceptance criteria
- [ ] A Linux host installs the SDK and cross-compiles a hello-Component for the native target without a JakeOS install.
- [ ] The package includes a QEMU harness that boots the Component on H-001.
- [ ] macOS and Windows host instructions use the containerised Linux environment and produce the same Package identity.
- [ ] Host-built Packages run on H-004.

#### Verification
- Integration: Linux-host CI builds and boots hello-Component on H-001.
- Manual: recorded macOS and Windows host paths using the containerised environment.
- Review: BLD lead confirms I-090 is preserved.

#### Evidence
- none

### SDK-040 · Ship LSP-based tooling for native SDK projects
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: SDK-009, SDK-043, IPC-012, IPC-049
- Baseline: §52, §56.5

V1 developer preview meets editors via LSP over the native crate and IDL, not a new IDE. 1.0 does not promise a native IDE.

<!-- covers: GAP-0457 -->

#### Out of scope
DAP (SDK-037). Remote development (SDK-047). IDE strategy (APP-020).

#### Acceptance criteria
- [ ] LSP completion and go-to-definition work for SDK Context methods and generated IDL stubs.
- [ ] Diagnostics include ergonomics-lint findings.
- [ ] The server runs on the host SDK and inside `os env`.

#### Verification
- Integration: `sdk:tests/lsp/completion_*` on the Linux-host SDK.
- Manual: VS Code and one JetBrains editor attach to the server.

#### Evidence
- none

### SDK-041 · Implement the os env enter command
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: SDK-006, ENV-013, ENV-018, SDK-025
- Baseline: §35, §64
- Invariants: I-043

V1 self-hosting and env-startup gates. ENV materialises the environment; SDK drops the developer into it via `os env enter` (§35). Native environments do not require Docker, a Linux VM or overlayfs (I-043).

<!-- covers: INV-0654 -->

#### Out of scope
DevelopmentEnvironment object (ENV-013). Lifecycle verbs (SDK-042). B-025 publication (BEN-025).

#### Acceptance criteria
- [ ] `os env enter` on the php-postgres-redis reference presents a working shell with declared services reachable.
- [ ] Enter does not start Docker, a Linux VM or an overlay filesystem.
- [ ] A second enter on a cached environment reuses ENV's cache.

#### Verification
- Integration: V1 env demo on H-004.
- Unit: `sdk:tests/cli/env_enter_*` on `qemu-x86_64`.
- Bench: B-025 sessions are started with this command; BEN publishes.

#### Evidence
- none

### SDK-042 · Implement os env list, status, leave, destroy and rebuild
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: SDK-041, ENV-012
- Baseline: §35, §64

The `os env` family is the developer CLI over ENV objects. list, status, leave, destroy and rebuild complete enter. ENV owns the object operations; SDK owns the verbs.

<!-- covers: INV-0655, INV-1262 -->

#### Out of scope
ENV object API (ENV-012). Enter (SDK-041). Rebuild-on-change (ENV-027).

#### Acceptance criteria
- [ ] `os env list` and `os env status` show ENV-provided identity, ResourceDomain and snapshot state.
- [ ] `os env leave` ends the session without destroying the environment.
- [ ] `os env destroy` tears down Components and drops the snapshot through ENV-012.
- [ ] `os env rebuild` recreates from environment.yaml plus lock.

#### Verification
- Integration: `sdk:tests/cli/env_lifecycle_*` on H-001 and H-004.
- Unit: each verb maps to the ENV operation and no other.

#### Evidence
- none

### SDK-043 · Implement os new scaffolding for a Component project
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: SDK-001, SDK-020, SDK-016, PKG-031
- Baseline: §52

V1 onboarding and the published SDK need a generator that emits manifest, requested Capabilities and build config. Templates pass the ergonomics lint.

<!-- covers: INV-0979 -->

#### Out of scope
Host SDK distribution (SDK-039). Test runner (SDK-045).

#### Acceptance criteria
- [ ] `os new component` emits a crate with `#[component]`, a Package manifest and requested Capabilities.
- [ ] The generated project builds with the host SDK and runs as hello-Component on H-001.
- [ ] Generated code passes SDK-016.

#### Verification
- Integration: B-050 hello-Component is produced by this command.
- Unit: `sdk:tests/cli/new_*` on the Linux-host SDK.

#### Evidence
- none

### SDK-044 · Implement os restore to a previous SystemGeneration
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: SDK-019, PKG-060, PKG-048
- Baseline: §30, §31, §64

Ladder moves generation restore to V1 (V0.5 is boot-menu rollback only). PKG performs the switch; SDK exposes `os restore` for a previous SystemGeneration.

<!-- covers: INV-1261 -->

#### Out of scope
PKG restore mechanism (PKG-060). History-point restore (SDK-069). Boot-menu rollback (BOOT).

#### Acceptance criteria
- [ ] `os restore --generation <id>` requests PKG restore of that SystemGeneration.
- [ ] After restore, `os history` records the restore event.
- [ ] A caller without restore rights receives `Error::Rights` and the current generation is unchanged.

#### Verification
- Integration: simulated bad update then `os restore` on H-001 and H-004.
- Unit: `sdk:tests/cli/restore_generation_*`.

#### Evidence
- none

### SDK-045 · Implement os test running the Component harness
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: SDK-050, SDK-006
- Baseline: §52

V1 SDK testing story needs a single command that runs the mock-Capability harness the same way CI does.

#### Out of scope
Harness implementation (SDK-050). CI plumbing (BLD).

#### Acceptance criteria
- [ ] `os test` in a scaffolded project runs the Component harness and exits zero on pass.
- [ ] Failures print the test name and the typed error.
- [ ] The command is the same entry CI uses for SDK unit jobs.

#### Verification
- Integration: `sdk:tests/cli/os_test_*` on the Linux-host SDK and H-001.
- Unit: fixture project with one passing and one failing test.

#### Evidence
- none

### SDK-046 · Implement the native CPU profiler attributed to Task and Component
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: SDK-053, SDK-061, OBS-039, TSK-038
- Baseline: §24, §52, §64

Sampling CPU profiles are attributed to Task and Component, not threads, and are exportable with traces. OBS owns sampling; SDK owns the profiler that developers run.

<!-- covers: EXTRA-032 -->

#### Out of scope
Sampling data plane (OBS-039). GPU attribution (SDK-071). Flame graphs (SDK-064). Format Decision (SDK-053).

#### Acceptance criteria
- [ ] A profiling session attributes samples to Task and Component identities from inspect.
- [ ] Export uses the format chosen by SDK-053 and can accompany an `os trace` export.
- [ ] Profiles do not reconstruct Linux threads as the primary grouping.

#### Verification
- Integration: profile of ImageDecoder on H-001 and H-004.
- Unit: `sdk:tests/profiler/cpu_attr_*`.

#### Evidence
- none

### SDK-047 · Integrate remote development into a JakeOS VM or device
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: SDK-039, SDK-040, SDK-037, SDK-041
- Baseline: §52, §56.5

VS Code and JetBrains remote-development into a JakeOS VM or device is required so host-built apps can be debugged on target. This is editor integration, not a native IDE.

<!-- covers: GAP-0457 -->

#### Out of scope
LSP server (SDK-040). DAP adapter (SDK-037). IDE strategy (APP-020). VM manager (VIRT).

#### Acceptance criteria
- [ ] VS Code remote attaches to a JakeOS QEMU guest and edits, builds and debugs a Component.
- [ ] One JetBrains remote path attaches to the same guest.
- [ ] Attach uses debug and env Capabilities; no ambient root shell is required.

#### Verification
- Manual: recorded VS Code and JetBrains sessions against H-001.
- Integration: guest-side LSP and DAP listen only with the granted Capabilities.

#### Evidence
- none

### SDK-048 · Enable userspace exploit mitigations in the native runtime
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: SDK-004, SEC-002
- Baseline: §51
- Invariants: I-060

ASLR, NX, CET where the CPU supports them, guard pages and a hardened allocator are default runtime behavior and Component manifest requirements. SEC owns policy; SDK enables them in the runtime. Language and Capability safety remain the primary model (§51).

<!-- covers: GAP-0187 -->

#### Out of scope
Threat model (SEC-002). Mitigation overhead bench (SEC-@mitigation-overhead). Kernel hardening (KRN).

#### Acceptance criteria
- [ ] Native Components run with ASLR, NX and guard pages on H-001 and H-004.
- [ ] CET shadow stacks and IBT are on when the CPU reports them (H-002, H-004).
- [ ] A Component manifest can require these mitigations; a runtime that cannot provide them refuses to start.
- [ ] The hardened allocator is the default; a debug allocator is opt-in.

#### Verification
- Integration: `runtime:tests/mitigations_*` on `qemu-x86_64`, `hw-h002` and `hw-h004`.
- Review: SEC lead confirms defaults match the threat model.

#### Evidence
- none

### SDK-049 · Implement the Layer 3 Rust std compatibility crate
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: SDK-028, SDK-009, SDK-003, ABI-006
- Baseline: §3, §50, §52, §66
- Risks: R-028
- Invariants: I-013, I-026

`std` and `alloc` for the native target are a Layer 3 facade over the SDK. No Layer 1 entry may be added to make `std`'s POSIX shapes work. Modules that cannot be expressed with Capabilities are absent or return typed unsupported errors.

<!-- covers: GAP-0090, EXTRA-045 -->

#### Out of scope
Layer 3 Decision (SDK-028). JSON target (SDK-003). ABI review gate (ABI-006).

#### Acceptance criteria
- [ ] `std` on the native target is a Layer 3 crate; ABI-028 classifies it as L3.
- [ ] `std::fs` and `std::net` do not introduce Layer 1 POSIX syscalls; they sit on SDK Capabilities or are omitted per the Decision.
- [ ] A proposed Native ABI entry whose rationale is `std` compatibility fails ABI-006.
- [ ] Maximum-performance crates continue to compile to machine code without this crate.

#### Verification
- Integration: hello-Component builds with and without `std` on H-001.
- Review: ABI lead confirms no L1 entry was added for `std`.
- Unit: `sdk:tests/std_facade_*`.

#### Evidence
- none

### SDK-050 · Provide a Component test harness with mock capabilities
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: SDK-009, SDK-001, CAP-003, IPC-030
- Baseline: §52

SDK-level tests need mock Capabilities and mock services so Components can be exercised without the full desktop. The harness is what `os test` and CI run.

<!-- covers: INV-0983 -->

#### Out of scope
`os test` command (SDK-045). Semantic UI harness (BLD-027). In-process transport (IPC-030).

#### Acceptance criteria
- [ ] A test can mint mock Capabilities and bind mock Interface servers in-process.
- [ ] Denied Capabilities return `Error::Rights` inside the harness without a kernel object.
- [ ] The PhotoEditor denial tests run on this harness on the Linux-host builder.

#### Verification
- Unit: `sdk:tests/harness/mock_caps_*` on the Linux-host SDK.
- Integration: PhotoEditor sample tests use the harness in CI.

#### Evidence
- none

### SDK-051 · Export an os trace session that can be viewed offline
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: SDK-022, OBS-041
- Baseline: §24, §61, §64

V1 debugger and tracing gate: `os trace` exports a session viewable offline using the OBS-chosen format. OBS owns the schema; SDK writes the file a developer copies off-device.

#### Out of scope
OBS export implementation (OBS-041). Live `os trace` (SDK-008).

#### Acceptance criteria
- [ ] `os trace export --offline` writes a self-contained session file.
- [ ] The file opens in the viewer recorded by OBS-015 without a running JakeOS.
- [ ] Payload redaction from OBS-024 is preserved in the file.

#### Verification
- Integration: export from H-004 opens on a Linux host viewer.
- Unit: `sdk:tests/cli/trace_offline_*`.

#### Evidence
- none

### SDK-052 · Decide DAP versus a native debugger protocol
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: SDK-060, CAP-032
- Baseline: §52, §61
- Decision: D-0257

V1 debugger gate and editor integration need a recorded protocol choice before the adapter is built. Options include DAP, a native protocol, or DAP over a native control Channel. Attach is a Capability in every option.

<!-- covers: GAP-0457 -->

#### Out of scope
Spike (SDK-060). Adapter (SDK-037). Debugger core (SDK-038).

#### Acceptance criteria
- [ ] Option A (DAP), option B (native protocol), and option C (DAP over a native control Channel) are evaluated against SDK-060.
- [ ] The Decision states that attach requires `Capability<Debug>` and is not same-uid ptrace.
- [ ] APP lead records Review sign-off on the pull request.

#### Verification
- Review: SDK and APP leads sign off on the pull request.

#### Evidence
- none

### SDK-053 · Decide profiler export format and Task attribution
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: SDK-061, OBS-015
- Baseline: §24, §64
- Decision: D-0261

Native profiles are attributed to Task, Component and TaskGroup, not threads. The Decision picks pprof, Firefox Profiler, or a native format folded into traces.

<!-- covers: EXTRA-032 -->

#### Out of scope
Attribution spike (SDK-061). Profiler implementation (SDK-046). OBS sampling (OBS-039).

#### Acceptance criteria
- [ ] Option A (pprof), option B (Firefox Profiler), and option C (native format folded into traces) are evaluated against SDK-061.
- [ ] The accepted option names Task, Component and TaskGroup as attribution keys and rejects thread as the primary key.
- [ ] OBS lead records Review sign-off on the pull request.

#### Verification
- Review: OBS and SDK leads sign off on the pull request.

#### Evidence
- none

### SDK-054 · Decide the Layer 3 SDK semver and deprecation policy
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: SDK-018, ABI-039, ABI-037
- Baseline: §52, §66
- Decision: D-0263
- Risks: R-028
- Invariants: I-040

V1 gate requires an accepted SDK stability-policy Decision. Layer 3 evolves with semver and deprecation windows, not a Layer 1 freeze (I-040, R-028). Surface S-031 is named as a V1 freeze candidate, not frozen.

<!-- covers: INV-0980 -->

#### Out of scope
Compat suite (SDK-036). Per-layer deprecation document (ABI-039). L1 freeze (ABI).

#### Acceptance criteria
- [ ] Option A (semver with recorded deprecation windows), option B (lockstep with Layer 2 interface versions), and option C (freeze Layer 3 at V1) are evaluated; option C is available to reject.
- [ ] The accepted option states S-031 is a freeze candidate at V1 and is not frozen before the Layer 3 policy allows.
- [ ] ABI lead records Review sign-off on the pull request.

#### Verification
- Review: ABI and SDK leads sign off on the pull request.

#### Evidence
- none

### SDK-055 · Decide the SDK v1 crate API freeze candidate
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: SDK-031, SDK-054
- Baseline: §50, §52, §66
- Decision: D-0347
- Invariants: I-040

V1 names S-031 as a freeze candidate: the Rust SDK v1 crate API with semver (§52, §66). This Decision names the crates and public items that constitute that candidate. Layer 3 evolves; it is not a Layer 1 freeze (I-040).

#### Out of scope
Semver and deprecation windows (SDK-054). Freeze execution (SDK-057). `std` Layer 3 placement (SDK-028).

#### Acceptance criteria
- [ ] Option A (named crate set as freeze candidate), option B (semver-only with no freeze candidate) and option C (freeze Layer 3 with Layer 1) are evaluated; option C is available to reject.
- [ ] The accepted option lists S-031 as a V1 freeze candidate and does not freeze a Layer 1 surface.
- [ ] ABI and SDK leads record Review sign-off on the pull request.

#### Verification
- Review: ABI and SDK leads sign off on the pull request.

#### Evidence
- none

### SDK-056 · Write the V1 SDK guide for the published Rust crate
- Type: docs
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: SDK-062, SDK-054, SDK-023, SDK-017, SDK-020, DOC-010, DOC-014
- Baseline: §52, §56.5, §61

V1 published-SDK gate and DOC IDL-to-docs at V1 need a crate guide covering Context, Operations, Packages and the chooser. SDK authors the crate guide; DOC publishes the site.

<!-- covers: INV-0978 -->

#### Out of scope
Site generation (DOC-007). Developer guide pipeline (DOC-014). Reference tutorial (SDK-062).

#### Acceptance criteria
- [ ] The V1 SDK guide covers Context, Operations, Packages, the chooser and the semver policy.
- [ ] The guide cites B-IDs for any performance discussion and states no unmeasured superiority claim.
- [ ] Broken-link check against the generated IDL pages is recorded.

#### Verification
- Review: DOC lead sign-off recorded on the pull request.

#### Evidence
- none

### SDK-057 · Freeze the SDK v1 crate API candidate
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: SDK-031, SDK-055, SDK-059, SDK-036
- Baseline: §50, §52, §66
- Freezes: S-031
- Invariants: I-040

V1 freeze of Layer 3 surface S-031 after the crate-API spike and accepted Decision. Semver and deprecation windows from SDK-054 remain the evolution rule. This is not a Layer 1 freeze (I-040).

#### Out of scope
Semver policy (SDK-054). Compat suite contents (SDK-036). Layer 1 freeze (ABI-049).

#### Acceptance criteria
- [ ] Surface S-031 is listed as frozen by this task in the surfaces register.
- [ ] The published SDK v1 crate set matches the accepted Decision's enumerated crates.
- [ ] A breaking public-item change without a semver bump and an accepted Decision fails CI.

#### Verification
- Integration: `sdk:tests/v1/crate_api_freeze_*` on `qemu-x86_64`.
- Review: SDK and ABI leads sign off on the pull request that lands the freeze.

#### Evidence
- none

### SDK-058 · Fuzz SDK handle wrappers for use-after-revoke and double-drop
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: SDK-009, CAP-004, TSK-010, BLD-042
- Baseline: §7, §19, §52
- Threats: T-003, T-005

Layer 3 handle safety is a V1 developer-preview concern. Harnesses exercise Capability drop, revoke and Operation cancel without kernel panics.

#### Out of scope
Kernel Capability fuzz (CAP-042). Userspace parser fuzz infra (BLD-042).

#### Acceptance criteria
- [ ] `sdk:fuzz/handles` runs in nightly CI without panic on double-drop, use-after-revoke and cancel-after-complete.
- [ ] Use-after-revoke returns `Error::Revoked` and allocates no handle.
- [ ] Crashes file through BLD's fuzz pipeline with a minimized repro.

#### Verification
- Fuzz: `sdk:fuzz/handles` one hour nightly without panic.
- Unit: deterministic fixtures for double-drop and revoke in `sdk:tests/handles_fuzz_oracle_*`.

#### Evidence
- none

### SDK-059 · Publish SDK v1 with semver policy and compatibility suite
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: SDK-054, SDK-036, SDK-039, SDK-033, SDK-049, SDK-056, SDK-043, REL-007
- Baseline: §52, §56.5, §61, §66
- Risks: R-028

V1 developer-preview gate: a published stable native SDK with documented versioning and deprecation, verified by the compatibility suite. S-031 is the freeze-candidate crate API; Layer 1 stays unfrozen (R-028).

<!-- covers: INV-1109, INV-1195 -->

#### Out of scope
Semver Decision (SDK-054). Compat suite content (SDK-036). Signed repository (REL-007).

#### Acceptance criteria
- [ ] SDK v1.0.0 is published as signed Packages on the developer repository.
- [ ] The release notes state the Layer 3 semver policy and that Layer 1 is not frozen.
- [ ] The compatibility suite from SDK-036 is green on that artifact.
- [ ] A hello-Component from `os new` builds against the published v1.0.0 artifact.

#### Verification
- Integration: host SDK install from the signed repository builds hello-Component on H-001.
- Review: ABI lead confirms S-031 is a freeze candidate, not frozen.
- Demo: V1 developer-preview SDK install on H-004.

#### Evidence
- none

### SDK-060 · Prototype DAP breakpoints inside async Tasks
- Type: spike
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: TSK-038, SDK-004, CAP-032
- Baseline: §20, §52, §58, §61

V1 requires breaking at a source line inside an async Task and showing the logical Task stack. The spike explores runtime hooks before the adapter lands and feeds SDK-052.

<!-- covers: GAP-0457 -->

#### Out of scope
Protocol Decision (SDK-052). Production adapter (SDK-037).

#### Acceptance criteria
- [ ] `reports/spikes/SDK-060.md` exists with the Spike skeleton headings.
- [ ] A prototype hits a breakpoint inside an awaited Operation on H-001.
- [ ] The report compares DAP, a native protocol and DAP-over-Channel without selecting one.

#### Verification
- Report: answers whether logical Task stacks from TSK-038 are sufficient for DAP; what runtime hooks are required; how attach Capabilities bind; recommended option set for the protocol Decision.

#### Evidence
- none

### SDK-061 · Prototype CPU sample attribution to Task and Component
- Type: spike
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: OBS-039, TSK-038, OBS-011
- Baseline: §24, §58, §64

Sampling must not reconstruct threads. The spike measures attribution cost on the OBS trace substrate before the profiler ships and feeds SDK-053.

<!-- covers: EXTRA-032 -->

#### Out of scope
Format Decision (SDK-053). Profiler implementation (SDK-046).

#### Acceptance criteria
- [ ] `reports/spikes/SDK-061.md` exists with the Spike skeleton headings.
- [ ] Prototype samples on H-002 are grouped by Task and Component.
- [ ] The report compares pprof, Firefox Profiler and a native folded format without selecting one.

#### Verification
- Report: answers attribution cost on the OBS substrate; whether thread grouping can be avoided; how profiles fold into traces; recommended option set for the format Decision.

#### Evidence
- none

### SDK-062 · Write the window, chooser, decode and render SDK tutorial
- Type: docs
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: SDK-021, SDK-023, SDK-017, SDK-013, DOC-016
- Baseline: §52

SDK owns the reference example and API tutorial for window, choose image, decode and render. DOC publishes the site. This is the worked §52 listing used by SDK v1.

<!-- covers: INV-0978 -->

#### Out of scope
Getting-started clone path (DOC-016). Crate guide (SDK-056). PhotoEditor sample code (SDK-021).

#### Acceptance criteria
- [ ] The tutorial walks window, `files.choose::<Image>()`, isolated decode and `window.render` against the PhotoEditor sample.
- [ ] Each step names the Capability granted or denied.
- [ ] The tutorial builds with SDK v1 on the host SDK.

#### Verification
- Review: DOC lead sign-off recorded on the pull request.
- Integration: tutorial snippets compile in docs CI.

#### Evidence
- none

### SDK-063 · Provide the C++ SDK binding
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: SDK-024, SDK-074, SDK-033, IPC-057
- Baseline: §50

§50 language order: C++ at V2 once C exists. V4 ecosystem gate needs C plus one additional language. The binding sits on the C ABI and IDL stubs; it does not invent a second Native ABI.

<!-- covers: INV-0939 -->

#### Out of scope
C++ shape spike (SDK-074). IDL C++ backend (IPC-057). C++ guide (SDK-073).

#### Acceptance criteria
- [ ] A C++ program creates a Component, awaits an Operation and moves a MemoryObject on H-001.
- [ ] The binding links only the C ABI and generated stubs; nm shows no second syscall surface.
- [ ] Drop of a handle releases it; use-after-move is a compile error or a typed runtime error.

#### Verification
- Integration: `sdk:tests/cpp/binding_*` on `qemu-x86_64` and H-005.
- Unit: move and error tests in `sdk:tests/cpp/handles_*`.

#### Evidence
- none

### SDK-064 · Render flame graphs by TaskGroup exportable with traces
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: SDK-046, SDK-068, SDK-053
- Baseline: §24, §64

Completes the native profiler: flame graphs grouped by TaskGroup and folded into exported traces.

<!-- covers: EXTRA-032 -->

#### Out of scope
CPU profiler (SDK-046). `os profile` command (SDK-068).

#### Acceptance criteria
- [ ] A profile session renders a flame graph grouped by TaskGroup.
- [ ] The graph exports with the trace in the chosen format.
- [ ] Switching grouping to Component is a supported view.

#### Verification
- Integration: flame graph of a V2 desktop session on H-002.
- Unit: `sdk:tests/profiler/flamegraph_*`.

#### Evidence
- none

### SDK-065 · Add os inspect support for ComputeQueue
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: SDK-007, HET-008, OBS-019
- Baseline: §37, §64

V2 ComputeDevice demo requires `os inspect` to show the ComputeQueue. OBS owns events; HET owns the object; SDK adds the subcommand rendering.

#### Out of scope
ComputeQueue object (HET-008). Preference-routing demo (HET-013).

#### Acceptance criteria
- [ ] `os inspect compute-queue` prints owner, device, depth and the in-flight Operation set.
- [ ] The V2 preference-routing demo's queues are visible through this command on H-002.
- [ ] Missing inspect rights returns `Error::Rights`.

#### Verification
- Demo: HET-013 inspect line on H-002.
- Unit: `sdk:tests/cli/inspect_computequeue_*`.

#### Evidence
- none

### SDK-066 · Invoke Semantic interfaces from the os CLI
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: SDK-025, SDK-035, SEM-007, SEM-008, SEM-013
- Baseline: §42, §64

Semantic interfaces must support scripting clients. The `os` CLI is the shell invocation path; SEM owns registry and permissioning. This is not an AI broker.

<!-- covers: INV-0793 -->

#### Out of scope
Registry (SEM-007). Automation engine (SEM-031). AI broker (SEM-010).

#### Acceptance criteria
- [ ] `os call Terminal.run` and `os call Editor.open` invoke the V1 Semantic interfaces without GUI input.
- [ ] Missing interface Capability returns `Error::Rights` and performs no call.
- [ ] JSON output matches SDK-035 schemas.

#### Verification
- Integration: V1 typed-automation demo driven by `os call` on H-004.
- Unit: `sdk:tests/cli/call_*`.

#### Evidence
- none

### SDK-067 · Implement os help serving offline on-device SDK documentation
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: SDK-006, DOC-020, DOC-018
- Baseline: §64, §56.5

Offline on-device docs. DOC owns content; SDK implements the `os help` CLI. The docs Package rolls back with the SystemGeneration.

#### Out of scope
Docs corpus (DOC-020). Docs Package (DOC-018).

#### Acceptance criteria
- [ ] `os help` renders the shipped docs Package with no network.
- [ ] `os help <subcommand>` shows the page for that command.
- [ ] After rolling back a generation, `os help` matches that generation's docs Package.

#### Verification
- Integration: offline `os help` on H-005 with network disabled.
- Unit: `sdk:tests/cli/help_*`.

#### Evidence
- none

### SDK-068 · Implement the os profile command over the native profiler
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: SDK-046, SDK-006, SDK-053
- Baseline: §64

The profiler is incomplete without an `os` CLI that records a session, attributes samples and writes the chosen export format.

#### Out of scope
Profiler implementation (SDK-046). Flame graphs (SDK-064).

#### Acceptance criteria
- [ ] `os profile record` captures a session attributed to Task and Component.
- [ ] `os profile export` writes the chosen format.
- [ ] Missing profile rights returns `Error::Rights` and writes no file.

#### Verification
- Integration: `os profile` of Terminal on H-002 and H-005.
- Unit: `sdk:tests/cli/profile_*`.

#### Evidence
- none

### SDK-069 · Implement os restore to a chosen history event
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: SDK-044, PKG-077, PKG-071, PKG-078
- Baseline: §31, §64

V2 snapshots and rollback UI consume history-point restore. SDK CLI invokes PKG restore of OS, packages and configuration for a chosen history event.

<!-- covers: INV-0578 -->

#### Out of scope
Generation restore (SDK-044). PKG package-set restore (PKG-077). Settings rollback UI (INS-014). Application-state checkpoint (SDK-093).

#### Acceptance criteria
- [ ] `os restore <event>` requests PKG restore of the state classes recorded at that history event.
- [ ] Selective restore follows PKG-071 and refuses inconsistent combinations.
- [ ] The restore appears as a new history event.

#### Verification
- Integration: restore a previous package set from `os history` on H-002 and H-005.
- Unit: `sdk:tests/cli/restore_event_*`.

#### Evidence
- none

### SDK-070 · Run the same SDK integration tests through C and C++
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: SDK-033, SDK-063, SDK-036
- Baseline: §50, §66

Language bindings must not drift: one integration corpus compiled as Rust, C and C++ against the compatibility suite.

#### Out of scope
Conformance at V3 (SDK-075). C++ binding (SDK-063).

#### Acceptance criteria
- [ ] The shared integration corpus passes as Rust, C and C++ on H-001.
- [ ] A behavior change in one binding that the others do not share fails CI.
- [ ] Corpus cases include Operation cancel, MemoryObject move and Channel disconnect.

#### Verification
- Integration: `sdk:tests/polyglot/*` on `qemu-x86_64` and H-005.
- Unit: corpus runner reports per-language results.

#### Evidence
- none

### SDK-071 · Attribute GPU samples to Task and Component
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: SDK-046, HET-019, GFX-030, OBS-039
- Baseline: §24, §37, §64

V2 ComputeDevice and gaming gates need GPU profiles on the same Task and Component model as CPU. HET and GFX own devices; SDK owns the profiler.

<!-- covers: EXTRA-032 -->

#### Out of scope
CPU profiler (SDK-046). GPUDispatch (HET-019). RenderQueue (GFX).

#### Acceptance criteria
- [ ] GPU samples during a ComputeQueue workload attribute to the submitting Task and Component.
- [ ] `os profile` can record a combined CPU and GPU session.
- [ ] Profiles do not use vendor thread names as the primary key.

#### Verification
- Integration: GPU profile of the V2 ComputeDevice demo on H-002.
- Unit: `sdk:tests/profiler/gpu_attr_*`.

#### Evidence
- none

### SDK-072 · Decide how non-Rust bindings map onto Layer 1 and IDL stubs
- Type: adr
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: SDK-024, SDK-033, IPC-047
- Baseline: §50, §65
- Decision: D-0260

Swift, Kotlin, C# and TypeScript at V3 need a recorded mapping onto the C ABI and IDL before those bindings start. Options are C-only, per-language codegen, or both.

#### Out of scope
V3 bindings (SDK-081 and siblings). IDL backends (IPC-057).

#### Acceptance criteria
- [ ] Option A (C ABI only), option B (per-language IDL codegen), and option C (both) are evaluated against IPC-047.
- [ ] The Decision forbids a second Native ABI for any language.
- [ ] IPC lead records Review sign-off on the pull request.

#### Verification
- Review: IPC and SDK leads sign off on the pull request.

#### Evidence
- none

### SDK-073 · Document the C++ SDK binding
- Type: docs
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: SDK-063
- Baseline: §50, §52

C++ at V2 needs a binding guide so the V4 extra-language ecosystem gate is not the first C++ documentation.

#### Out of scope
C++ binding (SDK-063). V3 multi-language guide (SDK-086). Site (DOC).

#### Acceptance criteria
- [ ] A C++ guide covers handles, Operations, move semantics and error enums.
- [ ] The polyglot corpus cases appear as worked examples.
- [ ] The guide states Layer 3 semver applies to the C++ crate or headers.

#### Verification
- Review: DOC lead sign-off recorded on the pull request.

#### Evidence
- none

### SDK-074 · Prototype C++ binding shape over the C ABI and IDL stubs
- Type: spike
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: SDK-033, IPC-047
- Baseline: §50, §58

C++ must not invent a second Native ABI. The spike compares header-only wrappers, generated C++ stubs, and C-only plus thin C++ before the binding ships.

<!-- covers: INV-0939 -->

#### Out of scope
Production C++ binding (SDK-063). Non-Rust mapping Decision (SDK-072).

#### Acceptance criteria
- [ ] `reports/spikes/SDK-074.md` exists with the Spike skeleton headings.
- [ ] Three prototypes (header-only, generated stubs, thin C++) build a hello-Component.
- [ ] The report recommends an option set without selecting it.

#### Verification
- Report: answers whether header-only wrappers hide move and cancel; whether generated stubs share the IDL backend; whether a thin C++ layer over C is enough; recommended option set for SDK-063.

#### Evidence
- none

### SDK-075 · Run binding conformance tests for every shipped language
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: SDK-070, SDK-081, SDK-078, SDK-077, SDK-085
- Baseline: §50, §66

V3 multi-language SDK must not silently diverge. One conformance corpus per binding against the same Layer 2 Interfaces.

#### Out of scope
IDL language backends (IPC-057). Polyglot C/C++ corpus (SDK-070).

#### Acceptance criteria
- [ ] Rust, C, C++ and every V3 language pass the shared conformance corpus on H-001.
- [ ] A missing method or divergent error mapping fails that language's job.
- [ ] Corpus coverage includes cancel, deadline, rebind and MemoryObject move.

#### Verification
- Integration: `sdk:tests/conformance/*` on `qemu-x86_64` and H-006.
- Unit: per-language runner reports.

#### Evidence
- none

### SDK-076 · Integrate os publish with GitHub Actions and GitLab CI
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: SDK-080, REL-021
- Baseline: §52, §56.5

Developers publish from CI. Hosted integrations wrap `os publish` so catalog growth is not gated on desktop signing.

<!-- covers: GAP-0465 -->

#### Out of scope
`os publish` itself (SDK-080). Publisher pipeline (REL-021).

#### Acceptance criteria
- [ ] A GitHub Actions workflow builds, signs with the publisher key and submits via `os publish`.
- [ ] A GitLab CI job does the same.
- [ ] Secrets never enter the Package; missing signing rights fail the job before upload.

#### Verification
- Integration: fixture repositories on both hosts complete SDK-082.
- Review: REL lead confirms the jobs call the publisher pipeline only.

#### Evidence
- none

### SDK-077 · Provide the C# SDK binding
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: SDK-072, SDK-033, IPC-057
- Baseline: §50

§50 V3 language: C# binding over IDL stubs. Windows personality remains a separate workstream; this binding is for native Components.

<!-- covers: INV-0942 -->

#### Out of scope
Windows personality (WIN). Non-Rust mapping Decision (SDK-072).

#### Acceptance criteria
- [ ] A C# program creates a Component and awaits an Operation on H-001.
- [ ] The binding uses the mapping chosen by SDK-072 and adds no Native ABI entry.
- [ ] Handle dispose releases the Capability.

#### Verification
- Integration: `sdk:tests/csharp/binding_*` on `qemu-x86_64`.
- Unit: dispose and error tests.

#### Evidence
- none

### SDK-078 · Provide the Kotlin SDK binding
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: SDK-072, SDK-033, IPC-057
- Baseline: §50

§50 V3 language: Kotlin binding for native Components, not a JVM personality.

<!-- covers: INV-0941 -->

#### Out of scope
A JVM personality (none; out of 1.0). Non-Rust mapping Decision (SDK-072).

#### Acceptance criteria
- [ ] A Kotlin program creates a Component and awaits an Operation on H-001.
- [ ] The binding uses the mapping chosen by SDK-072 and adds no Native ABI entry.
- [ ] Handle close releases the Capability.

#### Verification
- Integration: `sdk:tests/kotlin/binding_*` on `qemu-x86_64`.
- Unit: close and error tests.

#### Evidence
- none

### SDK-079 · Implement os bisect across SystemGenerations
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: SDK-019, SDK-044, PKG-019, BLD-006
- Baseline: §31, §64

Immutable generations make regression bisection cheap. The CLI steps generations to the introducing change for public-alpha bug reports.

<!-- covers: GAP-0388 -->

#### Out of scope
Generation objects (PKG-019). Restore (SDK-044). CI bisection of fuzz crashes (BLD).

#### Acceptance criteria
- [ ] `os bisect` accepts a test command and walks SystemGenerations to the first failing generation.
- [ ] The result names the generation identity and the history event that introduced it.
- [ ] Bisect does not mutate user data volumes.

#### Verification
- Integration: injected failing generation is found on H-001 and H-007.
- Unit: `sdk:tests/cli/bisect_*`.

#### Evidence
- none

### SDK-080 · Implement os publish to build, sign and submit Packages
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: SDK-020, REL-021, REL-025, GOV-054
- Baseline: §28, §52

V3 public repository demo: a third-party developer publishes a native Package from the SDK. REL owns review and signing keys; SDK is the submit command.

<!-- covers: GAP-0465 -->

#### Out of scope
Publisher pipeline (REL-021). CI wrappers (SDK-076). Store client (APP).

#### Acceptance criteria
- [ ] `os publish` builds, signs with the publisher key and submits to REL's pipeline.
- [ ] Capability requests from the manifest are included in the submission metadata.
- [ ] Unsigned or identity-mismatched submissions are rejected before upload.

#### Verification
- Integration: SDK-082 uses this command.
- Unit: `sdk:tests/cli/publish_*`.

#### Evidence
- none

### SDK-081 · Provide the Swift SDK binding
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: SDK-072, SDK-033, IPC-057
- Baseline: §50

§50 V3 language: Swift binding generated from IDL over the recorded non-Rust mapping.

<!-- covers: INV-0940 -->

#### Out of scope
Non-Rust mapping Decision (SDK-072). IDL Swift backend (IPC).

#### Acceptance criteria
- [ ] A Swift program creates a Component and awaits an Operation on H-001.
- [ ] The binding uses the mapping chosen by SDK-072 and adds no Native ABI entry.
- [ ] `deinit` releases the Capability.

#### Verification
- Integration: `sdk:tests/swift/binding_*` on `qemu-x86_64`.
- Unit: deinit and error tests.

#### Evidence
- none

### SDK-082 · Prove a third-party native Package can be published from the SDK
- Type: build
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: SDK-080
- Baseline: §52, §56.5, §63

V3 demo and repository drill: an external developer publishes from the SDK and a user installs with Capability review.

#### Out of scope
Publish command (SDK-080). REL drill (REL-033). Developer-program terms (GOV).

#### Acceptance criteria
- [ ] A Package built outside the core team is submitted with `os publish` and appears in the public repository.
- [ ] Install shows requested Capabilities and can deny an optional grant.
- [ ] `os trace` of the installed Package is visible to the user who granted inspect.

#### Verification
- Demo: V3 public-repository demo on H-006.
- Integration: REL-033 consumes this Package.

#### Evidence
- none

### SDK-083 · Implement a single command that installs versioned toolchains
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: SDK-084, SDK-006
- Baseline: §50, §52

GAP-0106 requires one toolchain-manager command so strangers do not reproduce the project's build environment by hand. The command installs Packages from SDK-084.

<!-- covers: GAP-0106 -->

#### Out of scope
Toolchain Packages (SDK-084). rustc pin (BLD-046).

#### Acceptance criteria
- [ ] One command installs a named toolchain version on Linux, on macOS via the containerised environment, and natively on JakeOS.
- [ ] `os toolchain list` shows installed versions and their Package identities.
- [ ] Switching versions does not mutate other toolchains' Packages.

#### Verification
- Integration: install and switch on H-001 and a Linux host.
- Unit: `sdk:tests/cli/toolchain_*`.

#### Evidence
- none

### SDK-084 · Package versioned developer toolchains as immutable Packages
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: SDK-059, PKG-038, BLD-046, BLD-047
- Baseline: §28, §50
- Invariants: I-089

V3 third-party developers install rustc target, IDL compiler, image tools and QEMU harness as immutable Packages on Linux, macOS and JakeOS. No custom compiler or forked LLVM (I-089).

<!-- covers: GAP-0106 -->

#### Out of scope
Toolchain manager CLI (SDK-083). rustc pin (BLD-046).

#### Acceptance criteria
- [ ] rustc native target, IDL compiler, image tools and QEMU harness each have a versioned Package.
- [ ] Packages install on Linux hosts and on JakeOS; macOS uses the containerised Linux environment (I-090).
- [ ] Two builds of the same toolchain version yield the same Package identity.

#### Verification
- Integration: toolchain Packages install from the public repository on a Linux host and H-007.
- Unit: identity check against the content-addressed store.

#### Evidence
- none

### SDK-085 · Provide the TypeScript SDK binding via Wasm components
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: SDK-072, WASM-012, WASM-013, IPC-057
- Baseline: §13, §50
- Invariants: I-046

§13 and §50 TypeScript binding uses Wasm component integration. WASM owns the runtime; SDK owns the language surface. This does not force native apps into Wasm (I-046).

<!-- covers: INV-0943 -->

#### Out of scope
Wasm host (WASM-012). Capability mapping (WASM-013). Native machine-code path (SDK-009).

#### Acceptance criteria
- [ ] A TypeScript Wasm Component talks over a Channel to a machine-code Component on H-001.
- [ ] A native Rust crate still compiles to machine code without this binding.
- [ ] The binding adds no Layer 1 entry and uses the recorded non-Rust mapping.

#### Verification
- Integration: `sdk:tests/ts_wasm/binding_*` on `qemu-x86_64`.
- Review: WASM lead confirms the host is userspace-only.

#### Evidence
- none

### SDK-086 · Publish SDK guides covering Rust, C and one extra language
- Type: docs
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: SDK-056, SDK-033, SDK-073, DOC-032
- Baseline: §50, §52, §56.5

V3 documentation gate includes the SDK guide. Public alpha strangers need Rust, C and the first extra language documented. DOC publishes; SDK authors.

#### Out of scope
V3 site editorial (DOC-032). C++ guide source (SDK-073).

#### Acceptance criteria
- [ ] Published guides cover Rust, C and the extra language selected by SDK-024.
- [ ] Each guide states Layer 3 semver and that Layer 1 is not frozen.
- [ ] Worked examples build against the V3 toolchain Packages.

#### Verification
- Review: DOC lead sign-off recorded on the pull request.

#### Evidence
- none

### SDK-087 · Prove C SDK binaries against the Layer 1 freeze candidate
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: SDK-033, ABI-047, ABI-049
- Baseline: §50, §66
- Invariants: I-040

V4 L1 freeze: C SDK binaries built against the freeze candidate run on every subsequent beta. ABI owns L1 tests; SDK owns the C crate run.

#### Out of scope
L1 compatibility suite (ABI-047). Freeze Decision (ABI-049).

#### Acceptance criteria
- [ ] C binaries built against the freeze-candidate SDK run on every subsequent V4 beta image in CI.
- [ ] Failures are reported as ABI suite cases, not as skipped SDK tests.
- [ ] No Layer 1 entry is added by the C crate (I-040).

#### Verification
- Integration: ABI-047 includes the C SDK run on H-001 and H-006.
- Unit: `sdk:tests/c/freeze_candidate_*`.

#### Evidence
- none

### SDK-088 · Ship a non-Rust native Package used in the V4 Demo
- Type: build
- Milestone: V4
- Status: todo
- Size: S
- Owner: none
- Depends on: SDK-075, SDK-082, SDK-086
- Baseline: §50, §56.5

V4 demo: a native Package published by an external developer in a non-Rust SDK language, installed with Capability review and tracing visible.

#### Out of scope
Bindings (SDK-063 and V3 languages). Publish path (SDK-080).

#### Acceptance criteria
- [ ] The demo Package is written in C or the extra language, not Rust.
- [ ] Install shows Capability review; `os trace` shows the Component after launch.
- [ ] The Package is not authored by the core team identity used in REL drills.

#### Verification
- Demo: V4 ecosystem demo on a Tier 1 machine in hardware scope.
- Integration: the Package passes SDK-075.

#### Evidence
- none

### SDK-089 · Lint deprecated Layer 3 APIs with a deprecation window
- Type: build
- Milestone: V4
- Status: todo
- Size: S
- Owner: none
- Depends on: SDK-054, SDK-059
- Baseline: §52, §66

Layer 3 semver needs mechanical deprecation detection before 1.0 so SDK v1 APIs that will not survive 1.x are flagged in beta.

#### Out of scope
Semver policy (SDK-054). ABI deprecated-entry detector (ABI-043).

#### Acceptance criteria
- [ ] Using a Layer 3 API marked deprecated fails CI after the recorded window, and warns during the window.
- [ ] The lint names the replacement API and the semver version that removes it.
- [ ] The lint does not apply to Layer 1 symbols (ABI owns those).

#### Verification
- Unit: `sdk:tests/lint/deprecation_*` on the Linux-host SDK.
- Integration: a fixture crate using a deprecated v1 API warns in V4 CI.

#### Evidence
- none

### SDK-090 · Propose the native rustc target for upstream tier-3 inclusion
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: SDK-003, SDK-049, SDK-059, BLD-046
- Baseline: §50
- Invariants: I-089

GAP-0090 later step: the custom JSON target is proposed for rustc tier-3 once SDK v1 and the Layer 3 `std` crate have been used in the beta fleet. Patches stay upstream-bound; no forked LLVM (I-089).

<!-- covers: GAP-0090 -->

#### Out of scope
JSON target (SDK-003). `std` crate (SDK-049). Custom compiler (forbidden by I-089).

#### Acceptance criteria
- [ ] A rustc target proposal document exists with the target triple, std/alloc status and test results from the beta fleet.
- [ ] Patches are against upstream rustc, not a project-owned LLVM fork.
- [ ] CI still builds with the in-tree JSON target until upstream inclusion lands.

#### Verification
- Review: BLD and SDK leads sign off on the proposal.
- Integration: beta fleet images still build if upstream has not accepted the target.

#### Evidence
- none

### SDK-091 · Make SDK and binding packages bit-for-bit reproducible
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: SDK-084, BLD-041, BLD-077
- Baseline: §27, §50
- Threats: T-007

V4 reproducible-builds gate includes SDK and binding Packages so third-party verifiers can rebuild developer toolchains.

#### Out of scope
SystemGeneration Package reproducibility (BLD-077). Double-build CI (BLD-041).

#### Acceptance criteria
- [ ] Two independent builds of SDK, C, C++ and V3 binding Packages match identities.
- [ ] Mismatches fail the V4 qualification job.
- [ ] Rebuild instructions use only published toolchain Packages.

#### Verification
- Integration: BLD independent rebuilder includes SDK Packages.
- Unit: `sdk:tests/repro/identity_*`.

#### Evidence
- none

### SDK-092 · Complete Rust and C SDK references for the beta review
- Type: docs
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: SDK-086, DOC-038
- Baseline: §52, §56.5

V4 exit: user, admin, SDK (Rust and C) references complete with review sign-off and broken-link checks. SDK authors remaining crate pages; DOC owns the editorial pass.

#### Out of scope
Editorial completion (DOC-035). C guide (DOC-038). Extra-language guide (DOC-037).

#### Acceptance criteria
- [ ] Rust and C SDK references cover every public Layer 3 crate exported by SDK v1.
- [ ] Broken-link and outdated-example CI from DOC-034 is green for SDK pages.
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: DOC lead sign-off recorded on the pull request.

#### Evidence
- none

### SDK-093 · Restore opted-in application state through os restore
- Type: build
- Milestone: 1.0
- Status: todo
- Size: M
- Owner: none
- Depends on: SDK-069, PKG-069, PKG-079, Q-056
- Baseline: §31

§31 long-term restore of application state where apps opt in to checkpointing. SDK exposes the checkpoint API and `os restore` consumes it. Scope follows PKG-069 and Q-056.

<!-- covers: INV-0585 -->

#### Out of scope
Whether 1.0 includes app-state restore (PKG-069, Q-056). Generation restore (SDK-044).

#### Acceptance criteria
- [ ] An opt-in Component writes a checkpoint object that `os restore` reloads after a history-point restore.
- [ ] A Component that did not opt in is unchanged by application-state restore.
- [ ] Missing checkpoint rights returns `Error::Rights` and writes no state.

#### Verification
- Integration: opted-in Editor buffer round-trip on a Tier 1 machine after `os restore`.
- Unit: `sdk:tests/checkpoint_*`.

#### Evidence
- none

### SDK-094 · Run the SDK compatibility suite through the 1.0 soak
- Type: build
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: SDK-036, SDK-087, BLD-079, ABI-052
- Baseline: §66

The 1.0 soak includes SDK v1 binaries on the release candidate. The suite is the Layer 3 half of the compatibility-proof demo.

#### Out of scope
Soak fleet (BLD-079). ABI 1.0 conformance run (ABI-052). Compat suite content (SDK-036).

#### Acceptance criteria
- [ ] SDK v1 binaries from V4 run unmodified on the 1.0 release candidate in the soak matrix.
- [ ] The compatibility suite is green on every in-scope H-ID of 1.0.
- [ ] A failure blocks 1.0 qualification.

#### Verification
- Integration: soak job runs `sdk:tests/compat/*` on the 1.0 hardware scope.
- Compat: suite report committed under the 1.0 candidate identity.

#### Evidence
- none

### SDK-095 · Publish the Layer 3 semver stability statement for 1.x
- Type: docs
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: SDK-054, ABI-053, SDK-089
- Baseline: §66

1.0 ABI stability statement covers Layer 1. Layer 3 still evolves with semver and must say so explicitly beside the 1.x support window.

#### Out of scope
Layer 1 stability declaration (ABI-053). Semver Decision (SDK-054).

#### Acceptance criteria
- [ ] A published statement names Layer 3 semver, deprecation windows and that L3 is not frozen with Layer 1.
- [ ] The statement cites SDK-054 and lists supported crate versions for 1.x.
- [ ] No performance number appears without a B-ID.

#### Verification
- Review: ABI and DOC leads sign off on the pull request.

#### Evidence
- none

### SDK-096 · Complete SDK guides for Rust and C on the 1.0 release
- Type: docs
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: SDK-092, SDK-095, DOC-040
- Baseline: §52, §56.5

1.0 documentation gate: SDK guides for Rust and C complete with the rest of the 1.0 doc set. DOC owns site publishing and the completeness checklist.

#### Out of scope
Docs completeness checklist (DOC-041). Snapshot (DOC-040). Layer 3 statement (SDK-095).

#### Acceptance criteria
- [ ] Rust and C SDK guides are in the 1.0 snapshot set.
- [ ] The guides match the Layer 3 stability statement.
- [ ] DOC-041 records these guides as present.

#### Verification
- Review: DOC lead sign-off recorded on the pull request.

#### Evidence
- none
