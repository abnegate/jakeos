# OBS · Observability and tracing
- Prefix: OBS
- Lead: none
- Baseline: §24, §64

<!-- roadmap:generated:begin summary -->
Tasks: 56 live, 0 done, 0 in-progress, 56 todo, 0 dropped. Ready: 1. Blocked: 55. Weighted: 0%.
<!-- roadmap:generated:end -->

## Scope

OBS owns observability as architecture (§24, §67 principle 11): the tracing substrate and structured semantic event ring; the typed kernel inspection interface and per-object providers; lifecycle, ownership, relationship, resource, latency, queueing, failure, Capability-grant, IPC-flow and scheduling-delay data for every primitive; compositor frame and ComputeQueue events; dynamic enablement and Capability-gated trace access; the persistent journal; the tamper-evident audit log; crash capture format, flight recorder, kernel panic artifacts and local symbolication; profiler sampling data attributed to Task, Component and TaskGroup; leak detection over inspect enumeration; and Personality attribution in traces. SDK owns the `os inspect`, `os trace` and profiler command surfaces; OBS owns the data they print.

## Out of scope

`os inspect` and `os trace` CLI rendering (SDK). Profiler UI and flame graphs (SDK). Crash-report consent and client (INS). Crash and telemetry intake, dashboards and debuginfod hosting (REL, BLD). Capability rights encoding, mint, derive and grant store (CAP). Semantic interface registry and AI broker (SEM). eBPF's native role beyond tracing (KRN). Compositor implementation and Frame emission (GFX). ComputeQueue and ComputeDevice objects (HET). Benchmark harness methodology (BEN). Permissions and Settings UI (APP, SEC). Privacy policy (GOV). Personality ptrace (LNX). Accessibility tree contents (ACC).

## Tasks

### OBS-001 · Measure tracing overhead enabled versus disabled on the IPC benchmark
- Type: benchmark
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: OBS-011, Q-001
- Baseline: §24, §54
- Benchmarks: B-012

Run harness `bench:tracing-overhead` so V0-G17 can publish B-012 on H-001 and H-002: the slowdown of B-004 and B-003 with tracing enabled versus disabled, and the cost of a disabled tracepoint on the hot path. The V0 target is publish-only; later rungs read the ceiling recorded by OBS-003 from the register.

<!-- covers: GAP-0532 -->

#### Out of scope
IPC round-trip publication itself (BEN-003, B-004). Methodology (Q-001).

#### Acceptance criteria
- [ ] A committed B-012 report exists for H-001 and H-002 with tracing enabled and disabled on the same session.
- [ ] The disabled-tracepoint cost is reported on the same B-004 path.
- [ ] No superiority claim appears without the published table (I-061).

#### Verification
- Bench: B-012 on H-001 and H-002; target per register (V0 publish).
- Review: BEN lead confirms the harness matches registers/benchmarks.md.

#### Evidence
- none

### OBS-002 · Trace and inspect Capability grant, derivation, transfer and revocation
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: OBS-008, OBS-006, CAP-001, CAP-003
- Baseline: §7, §24, §64
- Invariants: I-028

Every grant, derivation, transfer, revocation and typed denial emits a structured semantic event and is visible through the inspect interface as holder, target object, rights, derivation chain and revocation state (§7, §24). V0-D04 requires a typed denial in the audit trail. These are live events; the durable tamper-evident store is OBS-044.

<!-- covers: INV-0183, INV-0463, INV-0451 -->

#### Out of scope
Rights encoding and mint/derive (CAP). Durable tamper-evident log (OBS-044). Permissions UI (APP, SEC).

#### Acceptance criteria
- [ ] Grant, derive, transfer and revoke each emit a typed event naming holder, target and rights.
- [ ] `os inspect capability` (SDK) can print holder, target, rights, derivation chain and revocation state from OBS data.
- [ ] A forged or over-wide derive that returns `Error::Rights` is present as a denial event and allocates no handle.
- [ ] Events are named in Capability terms, not as raw kernel internals.

#### Verification
- Unit: `kernel:tests/obs/capability_audit_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Demo: V0-D04 typed denial visible in the audit trail on H-002.
- Integration: CAP isolation demo records the denial event for a missing `Capability<File>`.

#### Evidence
- none

### OBS-003 · Decide the tracing substrate and its measured overhead ceiling
- Type: adr
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: OBS-010
- Baseline: §24, §58
- Decision: D-0211
- Risks: R-034

One Decision for the V0 tracing substrate and the overhead ceiling that B-012 later verifies: extend inherited ftrace, tracepoints and eBPF; build a native per-Component structured ring; or layer a native semantic schema over eBPF. The Decision lists ABI surface S-010 and records that a disabled tracepoint stays off the hot path. Observability is architecture (§24), so the cost is bounded before every primitive is instrumented.

<!-- covers: GAP-0532, INV-0472 -->

#### Out of scope
eBPF as sched_ext or network policy (KRN-024). Export format (OBS-015). Implementation (OBS-011).

#### Acceptance criteria
- [ ] The Decision evaluates at least two of: extend ftrace/tracepoints/eBPF; native per-Component structured ring; native semantic schema over eBPF.
- [ ] The accepted option names the overhead ceiling that B-012 verifies and whether disabled scopes pay on the hot path.
- [ ] The Decision lists ABI surface S-010 in prototyped state and does not freeze it.
- [ ] A Review line records ABI and kernel lead sign-off on the pull request.

#### Verification
- Review: ABI and kernel leads sign off on the pull request; Evidence will contain `decision:<D-ID>` when the file is accepted.
- Report: Spike OBS-010 is cited as Evidence in the Decision options.

#### Evidence
- none

### OBS-004 · Generate tracing metadata from the IDL so every Channel call is observable by name
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: OBS-011, IPC-012
- Baseline: §14, §24

OBS owns the event-id, method-name and message-type schema that IDL codegen emits so every Channel call appears in `os trace` by Interface and method rather than as an anonymous send (§14, §24). The V0 demo trace must show `Channel<Request>` by name. IPC owns the compiler; this task is the OBS metadata plugin and schema.

<!-- covers: INV-0290 -->

#### Out of scope
IDL compiler front end and Rust stubs (IPC-012). CLI rendering (SDK-008).

#### Acceptance criteria
- [ ] Generated stubs emit a span with Interface name, method name and message type on every Channel call.
- [ ] A Channel whose IDL has no tracing metadata fails the OBS review gate.
- [ ] The V0 demo trace names `Channel<Request>` rather than a numeric opcode alone.

#### Verification
- Unit: `idl:tests/obs_metadata_*` on the V0 demo Interface.
- Integration: V0-D01 pipeline trace on H-001 shows method names from generated metadata.
- Review: IPC lead confirms the plugin does not fork the compiler IR.

#### Evidence
- none

### OBS-005 · Expose Component and Task inspection data including the awaited Operation
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: OBS-006, CMP-005, TSK-021, TSK-020
- Baseline: §20, §24, §64

Deliver the §64 inspect shape for Component and Task: name, state, memory, CPU consumption, held Capabilities, Tasks, owner TaskGroup, and the Operation a suspended Task waits on (for example `Channel<ImageDecodeRequest>`). Tooling does not reconstruct wait edges from scheduler traces when the kernel already stores them.

<!-- covers: INV-1263, INV-0447, INV-0448, INV-0383, INV-0459 -->

#### Out of scope
CLI formatting (SDK-007). TaskGroup tree as its own inspect kind (OBS-018). Channel, Operation, MemoryObject and ResourceDomain providers (OBS-007).

#### Acceptance criteria
- [ ] Inspect of a live Component returns name, state, memory, CPU consumption, held Capabilities and member Tasks.
- [ ] Inspect of a suspended Task returns the awaited Operation identity and type.
- [ ] Destroying the Component removes it from enumeration and reclaims the inspect records.

#### Verification
- Unit: `kernel:tests/obs/inspect_component_*` and `inspect_task_*` on `qemu-x86_64` and `hw-h002`.
- Demo: V0-D01 `os inspect component` on H-002 matches the §64 field set.
- Integration: A Task blocked on Receive reports that Channel as the wait object.

#### Evidence
- none

### OBS-006 · Build the typed kernel inspection Interface for per-Object metadata
- Type: build
- Milestone: V0
- Status: todo
- Size: L
- Owner: none
- Depends on: CMP-005, TSK-021, TSK-013, CAP-005, IPC-010, MEM-005, SCH-007
- Baseline: §7, §24, §64

The Layer 2-shaped inspect Interface that userspace tools call: enumerate live objects by kind and read ownership, relationships, resource use and wait state through typed messages rather than proc-style text (§64). V0-G10 requires `os inspect component|task|channel|capability|memory|resource` to print state, ownership and relationships. SDK renders; OBS supplies the Interface and kernel providers' registration.

<!-- covers: INV-1264, INV-0457, INV-0458 -->

#### Out of scope
Per-kind field sets (OBS-005, OBS-007). CLI (SDK-007). Userspace service providers (OBS-019).

#### Acceptance criteria
- [ ] A Capability to the inspect Interface enumerates every live object of a requested kind.
- [ ] Each record includes owner, relationship edges and resource-use fields required by V0-G10.
- [ ] A caller without inspect rights receives `Error::Rights` and no object list.
- [ ] Adding a kernel object kind without an inspect provider fails OBS-017.

#### Verification
- Unit: `kernel:tests/obs/inspect_interface_*` on `qemu-x86_64` and `hw-h002`.
- Integration: SDK `os inspect` against the Interface lists the six V0 kinds during the V0 demo.
- Fuzz: `kernel:fuzz/obs_inspect` one hour nightly without panic.

#### Evidence
- none

### OBS-007 · Expose Channel, Operation, MemoryObject and ResourceDomain inspection data
- Type: build
- Milestone: V0
- Status: todo
- Size: L
- Owner: none
- Depends on: OBS-006, IPC-009, TSK-013, MEM-005, SCH-007, SCH-009
- Baseline: §16, §19, §23, §24, §64
- Risks: R-073

Providers for the remaining four V0 inspect kinds. Channel: endpoints, message type, queue depth, blocked senders and receivers. Operation: a uniform enumerable set of outstanding work per Task, Component and ResourceDomain with kind, submitter, state, deadline and completion latency. MemoryObject: size, properties, owner and mappings. ResourceDomain: budgets, consumption and members. Transfer history and throttling events wait for V0.5.

<!-- covers: INV-0450, INV-0452, INV-0372, INV-0453, INV-0320, INV-0444, INV-0454, INV-0461, INV-0459 -->

#### Out of scope
Component and Task providers (OBS-005). MemoryObject transfer history (OBS-021). Throttling events (OBS-022). Queue-depth policy (IPC-009).

#### Acceptance criteria
- [ ] `os inspect channel` data includes endpoints, message type, queue depth and blocked parties.
- [ ] Outstanding Operations are enumerable per Task, Component and ResourceDomain with deadline and state.
- [ ] `os inspect memory` data includes size, properties, owner and current mappings.
- [ ] `os inspect resource` data includes budgets, current consumption and member Components.

#### Verification
- Unit: `kernel:tests/obs/inspect_channel_*`, `inspect_operation_*`, `inspect_memory_*`, `inspect_resource_*` on `qemu-x86_64` and `hw-h002`.
- Integration: V0-G10 listing of all six kinds on H-001.
- Demo: V0-D01 inspect of the result MemoryObject shows owner transfer after the handoff.

#### Evidence
- none

### OBS-008 · Emit lifecycle and ownership trace events for every V0 primitive
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: OBS-011, CMP-005, TSK-021, TSK-023, TSK-013, CAP-005, IPC-010, MEM-005, SCH-007
- Baseline: §19, §24, §59

Creation, destruction and owner-change events for Component, Task, TaskGroup, Channel, Capability, Operation, MemoryObject and ResourceDomain, named in primitive terms (Channel send, Operation complete) with typed fields. Operation submit events carry identity, submitter and parent span (§19). These feed `os trace` at V0.

<!-- covers: INV-1166, INV-0456, INV-0457, INV-0468, INV-0366 -->

#### Out of scope
IPC flow graph and scheduling-delay aggregation (OBS-009). IDL method names (OBS-004). Per-domain dynamic enable (OBS-025).

#### Acceptance criteria
- [ ] Create and destroy of each V0 primitive kind emit typed events with object identity and owner.
- [ ] Operation submit events include identity, submitter and parent span.
- [ ] Event names use primitive terms, not free-form strings or raw kernel internals.
- [ ] A primitive kind without create/destroy events fails OBS-017.

#### Verification
- Unit: `kernel:tests/obs/lifecycle_*` on `qemu-x86_64` and `hw-h002`.
- Integration: V0 demo pipeline produces create/destroy events for Component, Channel, Operation and MemoryObject.
- Fuzz: `kernel:fuzz/obs_events` one hour nightly without panic.

#### Evidence
- none

### OBS-009 · Ship the os trace first cut showing IPC flow, scheduling delays and failures
- Type: build
- Milestone: V0
- Status: todo
- Size: L
- Owner: none
- Depends on: OBS-008, OBS-004, OBS-007, SCH-010
- Baseline: §24, §59, §64

V0-G10 and V0-D01 require `os trace` to show IPC flow and scheduling delays for the §59 demo. OBS supplies the flow graph between Components, wakeup-to-run per Task, Operation latency, run-queue wait, and failure events (Operation errors, Component crashes, revocations). SDK owns the command; this task is the queryable trace view and aggregations.

<!-- covers: INV-0464, INV-0465, INV-0460, INV-0461, INV-0462, INV-1209 -->

#### Out of scope
CLI binary (SDK-008). Offline export format (OBS-015). Daily-driving filters and histograms (OBS-037). Per-intent-class histograms (OBS-040).

#### Acceptance criteria
- [ ] A live trace of the V0 demo shows Channel sends between Component A and B as a flow graph.
- [ ] Wakeup-to-run delay is present per Task on that trace.
- [ ] Operation completion latency is present on the same session.
- [ ] An Operation error, a Component panic disconnect and a revocation each appear as failure events.

#### Verification
- Integration: V0-D01 on H-002 with `os trace` displaying the pipeline flow.
- Unit: `kernel:tests/obs/trace_flow_*` and `trace_sched_*` on `qemu-x86_64`.
- Demo: V0-D01 live flow on H-002.

#### Evidence
- none

### OBS-010 · Study eBPF, ftrace, Fuchsia tracing and Perfetto for the native tracing layer
- Type: spike
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: none
- Baseline: §24, §58
- Explores: S-010
- Risks: R-034

Time-boxed comparison of inherited ftrace/tracepoints/eBPF, Fuchsia tracing and Perfetto against a native structured ring, focused on ABI surface S-010 (the tracing event record and enable bit). The report informs OBS-003 and records what retained Linux tooling (perf, bpftrace) can still provide beside native semantic events (§58).

<!-- covers: INV-0476, INV-1145 -->

#### Out of scope
The substrate Decision (OBS-003). eBPF as sched_ext or network policy (KRN-024). Implementation (OBS-011).

#### Acceptance criteria
- [ ] `reports/spikes/OBS-010.md` exists with the Spike skeleton headings.
- [ ] The report compares overhead on the B-004 path, in-kernel filtering safety, semantic-event fit and retained-tooling coverage for each substrate.
- [ ] The report recommends an option set for OBS-003 without selecting it.

#### Verification
- Report: answers overhead on B-004 enabled versus disabled; whether disabled scopes can stay off the hot path; which semantic events must be native versus retained Linux tooling; how Fuchsia and Perfetto schemas map onto Component, Task, Channel and Operation; recommended option set for the substrate Decision.
- Bench: B-012 prototype numbers on H-001 attached as Spike Evidence, not as a Gate.
- Review: OBS and KRN leads record that S-010 was explored.

#### Evidence
- none

### OBS-011 · Implement the structured semantic trace ring with runtime global enable
- Type: build
- Milestone: V0
- Status: todo
- Size: L
- Owner: none
- Depends on: OBS-003
- Baseline: §24, §59
- Risks: R-034
- Invariants: I-034

The V0 tracing substrate chosen by OBS-003: typed-field records named in primitive terms, never free-form strings or raw kernel internals, with a global enable that keeps a disabled tracepoint off the hot path (ABI surface S-010). V0-G10 needs this ring so `os trace` can show the Demo. Per-Component and per-ResourceDomain enable wait for V0.5.

<!-- covers: INV-0467, INV-0468, INV-0471 -->

#### Out of scope
Per-primitive, per-Component and per-ResourceDomain enable (OBS-025). Export format (OBS-015). Access policy (OBS-014). CLI (SDK-008).

#### Acceptance criteria
- [ ] Enabling tracing globally records typed events; disabling it records none on a subsequent B-004 run.
- [ ] Event records have typed fields and primitive names, not free-form strings.
- [ ] A disabled tracepoint does not appear on the B-004 hot path in the B-012 disabled configuration.
- [ ] No `unsafe` outside the substrate module named in the Decision.

#### Verification
- Unit: `kernel:tests/obs/trace_ring_*` on `qemu-x86_64` and `hw-h002`.
- Bench: B-012 on H-001 and H-002 uses this ring as the enabled configuration.
- Integration: V0 demo produces structured events consumed by OBS-009.

#### Evidence
- none

### OBS-012 · Require a Capability to read another Component traces
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: OBS-014, OBS-006, OBS-011
- Baseline: §9.1, §24
- Threats: T-027
- Invariants: I-021

Tracing is security-aware: a Component may read its own traces, and reading another Component requires a Capability rather than a same-user check (§24, T-027). This is the kernel check implementing OBS-014. Payload redaction and ResourceDomain charging of trace buffers are OBS-024. SEC policy cites this substrate.

<!-- covers: INV-0470 -->

#### Out of scope
Policy Decision (OBS-014). Payload redaction and buffer charging (OBS-024). Debugger attach rights (CAP-032). Personality ptrace (LNX-049).

#### Acceptance criteria
- [ ] Reading another Component's traces without the Decision's Capability returns `Error::Rights` and copies no records.
- [ ] A Component can read its own traces without that cross-Component Capability.
- [ ] The V0.5 Image Viewer Component cannot enumerate another application's trace payloads.

#### Verification
- Unit: `kernel:tests/obs/trace_cap_*` on `qemu-x86_64` and `qemu-virtio-gpu`.
- Integration: V0.5 chooser demo: Image Viewer denied traces of File Browser.
- Review: SEC lead confirms T-027 is addressed by the check.

#### Evidence
- none

### OBS-013 · Trace compositor frame timing, queueing and presentation events
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: OBS-008, GFX-010, GFX-024
- Baseline: §24, §40
- Benchmarks: B-018, B-020

OBS owns the semantic event schema for compositor commit, queue and scanout; GFX emits those events from the compositor Component (§40). V0.5 frame-latency and input-to-photon gates consume the timestamps. Multi-display and VRR deadline-miss metrics wait for V2.

<!-- covers: INV-0755 -->

#### Out of scope
Compositor implementation and Frame object (GFX). B-018 and B-020 harnesses (GFX, BEN, LAB). Multi-display deadline misses (OBS-046).

#### Acceptance criteria
- [ ] A client Surface commit emits a typed commit event with Frame identity and timestamp.
- [ ] Queue and scanout events are present for the same Frame on H-002.
- [ ] B-018 and B-020 harnesses can read commit-to-scanout intervals from the trace without parsing DRM debugfs.

#### Verification
- Integration: V0.5 compositor scenario on H-002 and H-003 records commit, queue and scanout for 3,600 frames.
- Bench: B-018 on H-002 consumes these events; target per register.
- Unit: `kernel:tests/obs/frame_events_*` on `qemu-x86_64`.

#### Evidence
- none

### OBS-014 · Decide who may trace and inspect which Components
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: SEC-002, OBS-006, OBS-011
- Baseline: §9.1, §24
- Decision: D-0209
- Threats: T-027
- Invariants: I-021

§24 requires tracing to be security-aware. Unrestricted tracing would defeat the V0.5 file-chooser demo (Image Viewer cannot see other files) by leaking Channel payloads and inspect graphs. Options: ambient debug right per session; `Capability<Trace>` per ResourceDomain subtree; owner-only with elevation. T-027 (debugger attach) is in scope for this Decision.

#### Out of scope
Kernel check (OBS-012). Redaction and charging (OBS-024). Debugger protocol (SDK-052). Threat model document (SEC-002).

#### Acceptance criteria
- [ ] The Decision evaluates at least two of: ambient debug right per session; `Capability<Trace>` per ResourceDomain subtree; owner-only with elevation.
- [ ] The accepted option states whether Channel payloads are visible without rights on that Channel.
- [ ] The Decision cites T-027 and I-021.
- [ ] A Review line records SEC lead sign-off on the pull request.

#### Verification
- Review: SEC lead sign-off recorded on the pull request.
- Integration: The accepted option is the policy OBS-012 implements.

#### Evidence
- none

### OBS-015 · Decide the trace event schema and export format
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: OBS-003, OBS-023
- Baseline: §24
- Decision: D-0210

Choose the event schema and export format so existing analysis tooling can be reused: Perfetto/CTF-compatible, OpenTelemetry mapping, or native binary with a schema registry. V0 gates only show flow live; V1-G10 requires offline export, so V0.5 is the last rung this Decision can occupy.

<!-- covers: GAP-0533, INV-0473 -->

#### Out of scope
Exporter implementation (OBS-041). SDK CLI export command (SDK-051). Substrate choice (OBS-003).

#### Acceptance criteria
- [ ] The Decision evaluates at least two of: Perfetto/CTF-compatible; OpenTelemetry mapping; native binary with schema registry.
- [ ] The accepted option names how an external decoder discovers event types.
- [ ] The Decision does not freeze an L1 surface.
- [ ] A Review line records SDK and OBS lead sign-off on the pull request.

#### Verification
- Review: SDK and OBS leads sign off on the pull request.
- Report: OBS-010 Perfetto findings are cited as option evidence.

#### Evidence
- none

### OBS-016 · Inspect an application's live Component graph via os inspect
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: OBS-006, OBS-005, CMP-024
- Baseline: §11, §24, §64

Using relationship edges from the inspect Interface, expose one application's live graph of Components, Channels and Capabilities (§11). First needed when the four V0.5 applications are graphs rather than single Components.

<!-- covers: INV-0246 -->

#### Out of scope
Graph declaration and instantiation (CMP-024). TaskGroup tree (OBS-018). CLI graph renderer (SDK).

#### Acceptance criteria
- [ ] Inspect of a V0.5 application returns its Components, the Channels between them and Capabilities each holds.
- [ ] A Component not in that application is absent from the graph.
- [ ] Destroying a child Component updates the graph on the next inspect without restart.

#### Verification
- Integration: Image Viewer plus isolated decoder graph on H-003 matches CMP instantiation.
- Unit: `kernel:tests/obs/inspect_graph_*` on `qemu-x86_64`.
- Demo: V0.5 Image Viewer graph via `os inspect` on H-002.

#### Evidence
- none

### OBS-017 · Add the observability review Gate requiring inspect and trace support per primitive
- Type: build
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: OBS-006, OBS-005, OBS-007, OBS-008
- Baseline: §24, §64, §67
- Invariants: I-034

CI and review enforcement for the standing rule that every kernel or platform primitive ships with inspect support and semantic lifecycle events in the same change (§24, §64, §67 principle 11). Tooling must not reconstruct application semantics from raw kernel events when the OS already knows the relationship.

<!-- covers: INV-0446, INV-1266, INV-0471, INV-1302 -->

#### Out of scope
The inspect Interface itself (OBS-006). Userspace provider registration (OBS-019).

#### Acceptance criteria
- [ ] CI fails a kernel object kind that has no inspect provider or no create/destroy events.
- [ ] The pull-request checklist includes an observability item naming inspect and trace.
- [ ] A fixture kind that omits events is rejected on `qemu-x86_64`.

#### Verification
- Unit: `tools:tests/obs/review_gate_*` asserting the lint fails on a fixture without a provider.
- Review: ABI lead confirms the checklist is required on kernel object changes.

#### Evidence
- none

### OBS-018 · Expose the TaskGroup ownership tree with membership and cancellation state
- Type: build
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: OBS-006, OBS-005, TSK-023
- Baseline: §21, §24

TaskGroup is not one of the six V0 inspect kinds. V0.5 supervision and structured cancellation debugging need membership, parent/child hierarchy and cancellation state through inspect (§21).

<!-- covers: INV-0397, INV-0449 -->

#### Out of scope
TaskGroup object and cancel propagation (TSK). Hang detection from wait graphs (OBS-047). Service supervision tree fields (SVC-010).

#### Acceptance criteria
- [ ] Inspect of a TaskGroup lists owned Tasks, child TaskGroups and cancellation state.
- [ ] Cancelling a parent marks descendants cancelled in inspect before they are reaped.
- [ ] A Component's owned TaskGroup is reachable from Component inspect.

#### Verification
- Unit: `kernel:tests/obs/inspect_taskgroup_*` on `qemu-x86_64`.
- Integration: V0.5 service restart scenario shows the supervisor TaskGroup tree on H-003.

#### Evidence
- none

### OBS-019 · Let platform services publish inspection data through the typed inspect Interface
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: OBS-006, GFX-010
- Baseline: §24, §64

INV-1266 extends inspect support to platform primitives, not only kernel objects. Services register providers on the typed inspect Interface so V0.5 can inspect compositor objects and, with ACC, the accessibility tree via `os inspect`. OBS owns registration and schema; services own their records.

<!-- covers: INV-1266 -->

#### Out of scope
Accessibility tree contents (ACC-003). Supervision-tree fields (SVC-010). Compositor internals (GFX).

#### Acceptance criteria
- [ ] A supervised service can register an inspect provider without a kernel patch.
- [ ] Unregistering on service death removes its objects from enumeration.
- [ ] A caller without inspect rights cannot read another service's provider records (`Error::Rights`).

#### Verification
- Integration: compositor objects enumerable via `os inspect` on H-003 after GFX-010 starts.
- Unit: `runtime:tests/obs/provider_registry_*` on `qemu-x86_64`.
- Review: SVC lead confirms death unregisters providers.

#### Evidence
- none

### OBS-020 · Assert zero leaked kernel objects after every integration test via the inspect Interface
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: OBS-007, OBS-005, SCH-009
- Baseline: §24
- Risks: R-074
- Threats: T-016

Use inspect enumeration to assert no leaked Capabilities, MemoryObjects, Channels or Tasks after each integration test in CI. Leaked kernel objects in a long-lived session are the usual cause of gradual degradation. Multi-hour soaks are OBS-036.

<!-- covers: GAP-0133 -->

#### Out of scope
Nightly hardware soak (OBS-036). Kernel-object limit enforcement (SCH-009). CMP 100k create/destroy leak test (CMP-004).

#### Acceptance criteria
- [ ] The integration-test harness fails if inspect reports a Capability, MemoryObject, Channel or Task that outlived its Component.
- [ ] The check runs on `qemu-x86_64` for every merge to main.
- [ ] A fixture that deliberately leaks a Channel fails the harness.

#### Verification
- Integration: `kernel:tests/obs/leak_guard_*` on `qemu-x86_64`.
- Unit: fixture leak is detected and the test is red.

#### Evidence
- none

### OBS-021 · Record MemoryObject transfer history and property-change events
- Type: build
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: OBS-007, OBS-008, MEM-026
- Baseline: §16, §24

V0 inspect shows size, properties, owner and mappings. Transfer history and property-change tracing are required when the compositor and Image Viewer pass buffers across several Components (§16).

<!-- covers: INV-0453, INV-0320 -->

#### Out of scope
Ownership-transfer implementation (MEM). V0 inspect minimum (OBS-007).

#### Acceptance criteria
- [ ] Each ownership transfer appends a history record with previous owner, new owner and timestamp.
- [ ] A property change (seal, map, unmap) emits a typed event.
- [ ] Inspect of a transferred demo buffer shows the A-to-B-to-A path after V0.5 Image Viewer decode.

#### Verification
- Unit: `kernel:tests/obs/memory_history_*` on `qemu-x86_64`.
- Integration: Image Viewer decode path on H-003 lists each transfer in inspect.

#### Evidence
- none

### OBS-022 · Emit ResourceDomain throttling and budget-exhaustion events
- Type: build
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: OBS-008, SCH-016, SCH-014
- Baseline: §23, §24

V0 inspect shows budgets and consumption. Throttling and kernel-object-limit exhaustion become events once SCH enforces them on real applications (§23).

<!-- covers: INV-0454 -->

#### Out of scope
Exhaustion policy (SCH-016). Enforcement (SCH-014). Inspect of budgets (OBS-007).

#### Acceptance criteria
- [ ] Hitting a memory budget emits a typed throttling or exhaustion event naming the ResourceDomain and budget kind.
- [ ] Kernel-object-limit exhaustion emits a typed event and inspect still lists the domain.
- [ ] Events are visible through `os trace` without reading cgroup files.

#### Verification
- Integration: SCH fault-injection of memory and object-limit exhaustion on `qemu-x86_64` produces OBS events.
- Unit: `kernel:tests/obs/throttle_events_*`.

#### Evidence
- none

### OBS-023 · Prototype inspect and trace Layer 2 Interface shape
- Type: spike
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: OBS-010, OBS-006
- Baseline: §24, §64
- Explores: S-035

Prototype the Layer 2 inspect and trace Interface shape so OBS-015 and the later L2 lock are not paper Decisions (§24). Distinct from L1 tracing event S-010. Surface S-035 remains open.

#### Out of scope
L1 tracing substrate Decision (OBS-003). Freeze of S-035 (OBS-052). `os inspect` CLI (SDK-007).

#### Acceptance criteria
- [ ] A prototype inspect Interface dumps one Component and one Channel on H-001.
- [ ] A prototype trace export round-trips a session with object identities intact.
- [ ] Surface S-035 remains `open` or `prototyped`, never `frozen`.

#### Verification
- Report: which inspect fields are kernel-provided versus userspace-provided, how export versioning would work, and which options the export Decision must evaluate.
- Integration: the prototype runs on `qemu-x86_64`.

#### Evidence
- none

### OBS-024 · Gate tracing and inspection behind Capabilities with payload redaction
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: OBS-014, OBS-012, SCH-007
- Baseline: §9.1, §22, §24
- Threats: T-027
- Invariants: I-021

Complete OBS-014: inspect and trace requests carry a Capability, Channel payloads are redacted unless the holder has rights on that Channel, and trace buffers are charged to the requester's ResourceDomain so tracing cannot exhaust another domain (T-016, T-027).

#### Out of scope
The Capability check alone (OBS-012). Debug-attach Capability types (CAP-032). Personality ptrace (LNX).

#### Acceptance criteria
- [ ] A holder of inspect rights without Channel rights sees redacted payloads, not message bodies.
- [ ] Trace buffer memory is charged to the requester's ResourceDomain and over-budget recording returns a typed error.
- [ ] Dropping the Trace Capability stops further record delivery within one Operation.

#### Verification
- Unit: `kernel:tests/obs/trace_redact_*` and `trace_charge_*` on `qemu-x86_64`.
- Integration: V0.5 Image Viewer cannot read File Browser Channel payloads from traces.
- Review: SEC lead confirms redaction matches the Decision.

#### Evidence
- none

### OBS-025 · Enable and disable tracing at runtime per primitive, Component and ResourceDomain
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: OBS-011, OBS-003
- Baseline: §24

Selective tracing first matters on a multi-application desktop. Enable and disable at runtime per primitive kind, per Component and per ResourceDomain without restart, with in-kernel filtering so disabled scopes stay off the hot path (§24).

<!-- covers: INV-0469 -->

#### Out of scope
Global enable (OBS-011). Access policy (OBS-014). CLI filters (OBS-037, SDK).

#### Acceptance criteria
- [ ] Disabling a Component at runtime stops new events from that Component without restart.
- [ ] Disabling a primitive kind (for example Channel send) stops that kind process-wide while others continue.
- [ ] A disabled ResourceDomain does not appear on the B-004 hot path in a B-012-style enabled-vs-disabled pair.

#### Verification
- Unit: `kernel:tests/obs/filter_dynamic_*` on `qemu-x86_64`.
- Integration: Four V0.5 apps running; enabling only Image Viewer records no Terminal events.
- Bench: B-012 disabled-scope configuration on H-002.

#### Evidence
- none

### OBS-026 · Capture Component crashes with typed exit cause, async Task stacks and trace window
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: OBS-029, CMP-004, CMP-008, OBS-032, TSK-038
- Baseline: §10, §24, §61
- Risks: R-075
- Threats: T-023
- Invariants: I-077

Implement the capture format chosen by OBS-029: panic, stack-overflow and OOM-abort exit causes from the supervisor, logical Task stacks, held Capabilities and the flight-recorder window. SDK debugger and INS crash-report client consume this record. Dumps never contain disk keys or unlocked secrets (I-077). Required by V3-G04 (Crash reports are symbolicated and opted in).

#### Out of scope
Exit-cause semantics (CMP-008). Debugger UI (SDK-038). Consent and upload (INS-020). Intake (REL-038). Kernel panic artifacts (OBS-027).

#### Acceptance criteria
- [ ] A panicking Component produces a capture with typed exit cause, logical Task stacks, held Capabilities and the flight-recorder window.
- [ ] Stack overflow and OOM abort produce distinct exit causes in the capture.
- [ ] Captures contain no disk keys or unlocked secrets under the inventory in OBS-050.
- [ ] The capture is readable by the SDK symbolicator without a personality core-file parser when the Decision selected a native record.

#### Verification
- Integration: CMP panic demo on `qemu-x86_64` and H-004 writes a capture matching the Decision.
- Unit: `runtime:tests/obs/crash_capture_*`.
- Review: SEC lead confirms I-077 against the field inventory.

#### Evidence
- none

### OBS-027 · Capture kernel crash artifacts with symbolised Rust panic backtraces in CI and field
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: OBS-029, KRN-041, BLD-008
- Baseline: §24, §61
- Risks: R-011
- Threats: T-023

Retain pstore/kdump and wire them so QEMU CI and reference hardware persist kernel panics with symbolised Rust backtraces. KRN watchdog and lockup detectors feed this path. Full pstore/kdump handoff to the V3 reporter is KRN-048; this task is the V1 capture so stripped daily-driver builds remain diagnosable.

<!-- covers: GAP-0151 -->

#### Out of scope
Debuginfod hosting (REL, BLD-038). Local symbol package (OBS-028). Component crashes (OBS-026). Watchdog policy (KRN-041).

#### Acceptance criteria
- [ ] A triggered kernel panic on H-001 persists an artifact across reboot.
- [ ] The artifact includes a symbolised Rust panic backtrace for in-tree Rust kernel code.
- [ ] CI collects the artifact from `qemu-x86_64` panic jobs.

#### Verification
- Integration: BLD kernel-debug workflow panic job on H-001 retains pstore/kdump output.
- Manual: Injected panic on H-002, reboot, artifact present.
- Review: KRN lead confirms retained pstore/kdump are not disabled in the V1 config.

#### Evidence
- none

### OBS-028 · Package build-id keyed debug symbols and a symbolication tool for every build
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: BLD-026
- Baseline: §61, §63
- Threats: T-023

Produce debuginfo artifacts keyed by build-id and a local symbolicator for stripped release builds so V1 daily-driving and later crash reports can be symbolised without shipping symbols on the image. Hosting a debuginfod-style server is REL/BLD pipeline scope.

<!-- covers: GAP-0151, GAP-0362 -->

#### Out of scope
Symbol server operation (BLD-038, REL). Crash-report client (INS). Capture format (OBS-029).

#### Acceptance criteria
- [ ] Every CI release profile publishes a build-id keyed debuginfo artifact beside the stripped image.
- [ ] A local symbolicator maps a stripped capture to source functions using only those artifacts.
- [ ] The image itself does not contain debuginfo for shipped Packages.

#### Verification
- Integration: symbolicate a V1 stripped panic capture on H-001 using the published artifact.
- Unit: `tools:tests/obs/symbolicate_*`.
- Review: BLD lead confirms the symbols-preserved profile feeds this package.

#### Evidence
- none

### OBS-029 · Decide the Component crash capture format
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: OBS-006, CMP-008
- Baseline: §10, §24, §61
- Decision: D-0207
- Risks: R-048
- Threats: T-023
- Invariants: I-077

OBS scope names crash capture format. Options: minidump-compatible; native typed record with async Task stacks; core-file plus sidecar. Consumers are the SDK debugger (V1 async Task stacks) and the INS crash-report client (V3). The Decision states how I-077 (no disk keys or unlocked secrets) is enforced in the record.

#### Out of scope
Implementation (OBS-026). Kernel panic artifacts (OBS-027). Privacy policy (GOV-061). Consent UI (INS).

#### Acceptance criteria
- [ ] The Decision evaluates at least two of: minidump-compatible; native typed record with async Task stacks; core-file plus sidecar.
- [ ] The accepted option names Task-stack representation and where the flight-recorder window attaches.
- [ ] The Decision cites I-077 and T-023.
- [ ] A Review line records SDK, SEC and OBS lead sign-off on the pull request.

#### Verification
- Review: SDK, SEC and OBS leads sign off on the pull request.

#### Evidence
- none

### OBS-030 · Decide the persistent journal record format and retention model
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: OBS-015, PKG-007
- Baseline: §24, §30
- Decision: D-0208

Persistent structured logs from previous boots, distinct from the tracing pipeline. Options: systemd-journal-compatible export; native typed records over the trace schema; plain structured text. The Decision sets retention, per-boot indexing and how Personality logs are ingested, and respects PKG's generation-boundary Decision that logs are not immutable generation content.

<!-- covers: GAP-0290 -->

#### Out of scope
Store implementation (OBS-035). Redaction and bug-report export (OBS-034). Collection from supervised services (SVC-031).

#### Acceptance criteria
- [ ] The Decision evaluates at least two of: systemd-journal-compatible export; native typed records over the trace schema; plain structured text.
- [ ] The accepted option names retention, per-boot indexing and Personality ingest.
- [ ] The Decision states that journal data is excluded from SystemGeneration rollback.
- [ ] A Review line records OBS and SVC lead sign-off on the pull request.

#### Verification
- Review: OBS and SVC leads sign off on the pull request.

#### Evidence
- none

### OBS-031 · Write the instrumentation guide for semantic events and inspect providers
- Type: docs
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: OBS-008, OBS-019, OBS-025
- Baseline: §24, §52, §64

SDK v1 ships at V1. Third-party Components must know how to emit semantic events and register inspect providers so OBS-017 is achievable outside the core team.

#### Out of scope
IDL-to-docs generation (DOC). SDK crate API (SDK). Review-gate lint (OBS-017).

#### Acceptance criteria
- [ ] A guide in the SDK docs tree names how to emit a semantic event and how to register an inspect provider.
- [ ] The guide cites OBS-017 as the CI check a third-party Package must pass.
- [ ] A Review line records DOC and SDK lead sign-off.

#### Verification
- Review: DOC and SDK leads sign off on the pull request.
- Manual: A sample third-party Component in SDK tests follows the guide and passes the review gate.

#### Evidence
- none

### OBS-032 · Add an always-on flight recorder with trigger-based dumps on failure
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: OBS-025, OBS-009, OBS-035, OBS-001
- Baseline: §24, §61
- Threats: T-023

Field debugging for daily-driving: a low-overhead ring kept on within the B-012 ceiling, dumped to the journal on Component crash, deadline miss or revocation storm. V3 crash reports attach the last trace window from this ring.

<!-- covers: INV-0462 -->

#### Out of scope
Journal format (OBS-030). Crash-record schema (OBS-029). Access policy (OBS-024).

#### Acceptance criteria
- [ ] The recorder is enabled by default on V1 images and stays within the B-012 enabled ceiling on H-004.
- [ ] Component crash, deadline miss and revocation storm each dump the ring into the journal.
- [ ] A dump is charged to the crashing Component's ResourceDomain, not an ambient log domain.

#### Verification
- Integration: induced panic, deadline miss and revocation storm on `qemu-x86_64` each produce a journal dump.
- Bench: B-012 with flight recorder on, H-001 and H-004; target per register.
- Unit: `kernel:tests/obs/flight_recorder_*`.

#### Evidence
- none

### OBS-033 · Define inspection and trace schemas as versioned Layer 2 interfaces
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: OBS-006, IPC-042, IPC-035
- Baseline: §12, §24, §66

V1 freezes L2 evolution rules and ships SDK v1 tools that consume inspection data. Inspect and trace schemas become IDL-defined versioned Interfaces with the forward/backward test applied (§12, §66). L1 tracing event ABI surface S-010 stays prototyped until V4. Required by V4-G02 (Layer 2 interface versions for 1.x are locked).

#### Out of scope
L2 evolution rules Decision (IPC-042). Version lock (OBS-052). CLI (SDK).

#### Acceptance criteria
- [ ] Inspect and trace schemas are IDL Interfaces with version identities in IPC-035.
- [ ] An added optional field is accepted by an older inspect client.
- [ ] A missing optional field is accepted by a newer inspect client.
- [ ] CI fails an inspect schema change that skips a version bump.

#### Verification
- Integration: old-client/new-service and new-client/old-service inspect tests on `qemu-x86_64`.
- Unit: `idl:tests/obs_inspect_version_*`.
- Review: IPC lead confirms evolution-rule compliance.

#### Evidence
- none

### OBS-034 · Add privacy redaction and bug-report export to the journal
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: OBS-035, OBS-030, OBS-032
- Baseline: §24, §63
- Threats: T-023
- Invariants: I-077

Redaction rules per field class and a bug-report bundle (journal, inspect snapshot, flight-recorder dump) that the INS crash-report client later shows and redacts before send. Distinct from live tracing.

<!-- covers: GAP-0290 -->

#### Out of scope
Consent and upload (INS-020, INS-021). Privacy policy (GOV-061). Intake (REL).

#### Acceptance criteria
- [ ] Export applies redaction rules per field class before writing the bundle.
- [ ] A bundle contains journal records, an inspect snapshot and the flight-recorder window.
- [ ] Redacted fields never include disk keys or unlocked secrets.

#### Verification
- Unit: `runtime:tests/obs/journal_redact_*`.
- Integration: bundle from a V1 panic on `qemu-x86_64` passes the I-077 scan.
- Review: SEC lead confirms T-023 redaction classes.

#### Evidence
- none

### OBS-035 · Build the persistent structured journal with retention and per-Component attribution
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: OBS-030, OBS-008, PKG-007
- Baseline: §24, §30, §61

Durable logs from previous boots, distinct from the tracing pipeline, with retention and per-Component attribution. Required for V1 daily-driving issue tracking and later crash reports. SVC-031 collects from supervised services into this store.

<!-- covers: GAP-0290 -->

#### Out of scope
Format Decision (OBS-030). Redaction and export (OBS-034). Service-side collection (SVC-031). Tamper-evident security audit log (OBS-044).

#### Acceptance criteria
- [ ] Records from a previous boot are readable after reboot on H-004.
- [ ] Each record names the emitting Component.
- [ ] Retention drops records older than the Decision's policy and reclaim is visible in ResourceDomain storage accounting.
- [ ] Rolling back a SystemGeneration does not delete journal records (PKG generation boundary).

#### Verification
- Integration: reboot on H-001 and H-004; previous-boot records queryable.
- Unit: `runtime:tests/obs/journal_store_*`.
- Review: PKG lead confirms exclusion from generation trees.

#### Evidence
- none

### OBS-036 · Run nightly multi-hour soak with leak detection on reference hardware
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: OBS-020, LAB-010, BLD-044
- Baseline: §24, §61
- Risks: R-074

Second half of GAP-0133: a multi-hour desktop soak on reference machines with the same leak assertions as CI, scheduled through the lab scheduler. LAB owns calendar and machine uptime; OBS owns the assertions.

<!-- covers: GAP-0133 -->

#### Out of scope
Lab scheduler (LAB-010). CI wiring (BLD-044). 1.0 soak report (OBS-055).

#### Acceptance criteria
- [ ] A nightly job on H-002 and H-004 runs the leak assertions for the soak duration named in the job.
- [ ] A leak fails the job and is visible in CI.
- [ ] The job is scheduled through LAB-010, not a one-off script.

#### Verification
- Integration: nightly soak job on H-002 and H-004 with leak_guard.
- Manual: LAB operator confirms the job appears on the soak calendar.

#### Evidence
- none

### OBS-037 · Make os trace usable for daily performance and correctness debugging
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: OBS-009, OBS-025
- Baseline: §24, §61, §64

V1 scope: debugging and tracing tools for daily-driving developers. Record, filter, aggregate, latency histograms, timeline view and live follow on top of the V0 first cut. Offline export is OBS-041; the CLI remains SDK.

<!-- covers: INV-1209 -->

#### Out of scope
CLI binary (SDK-008). Offline file format (OBS-041). Profiler samples (OBS-039). Personality attribution (OBS-038).

#### Acceptance criteria
- [ ] A developer can record, filter by Component and primitive, and follow live events on H-004.
- [ ] Latency histograms for Channel round trip and Operation completion are queryable from a recorded session.
- [ ] A timeline of the V1 editor-plus-terminal workflow is producible without a custom GUI.

#### Verification
- Integration: daily-driving trace session on H-004 covering Terminal and Editor.
- Unit: `runtime:tests/obs/trace_query_*`.
- Demo: V1-G10 tracing half on H-004.

#### Evidence
- none

### OBS-038 · Attribute Linux-Personality processes and syscalls in os trace and os inspect
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: OBS-037, OBS-006, LNX-042, LNX-030
- Baseline: §3, §24, §46, §61
- Benchmarks: B-026

V1 daily-driving runs browser and IDE via the Linux personality. B-026 needs syscall-translation events attributed to the owning Component. Native software still never sees POSIX; these events live on the Personality path. Windows personality rides on this because Wine runs inside LNX.

#### Out of scope
Personality syscall retention and translation (LNX). ptrace (LNX-049). Native ABI shapes (ABI). Windows-specific tracing (WIN).

#### Acceptance criteria
- [ ] A Linux-personality process is inspectable as its enclosing Component.
- [ ] Syscall-translation events in `os trace` name that Component, not a global kernel thread.
- [ ] B-026 overhead runs can attribute syscall cost to the Personality Component.

#### Verification
- Integration: Linux-personality browser session on H-004; inspect and trace name the enclosing Component.
- Compat: B-026 publish path on H-004 consumes these events.
- Unit: `kernel:tests/obs/personality_attr_*`.

#### Evidence
- none

### OBS-039 · Provide sampling profiles attributed to Task, Component and TaskGroup
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: OBS-011, OBS-008, TSK-038
- Baseline: §20, §21, §24, §61

Kernel sampling with attribution to Task and Component rather than threads, aggregated by TaskGroup, exportable alongside traces. The profiler UI and flame graphs are SDK; this task is the sample stream and attribution. GPU attribution waits for V2 (SDK-071).

<!-- covers: EXTRA-032 -->

#### Out of scope
Profiler UI (SDK-046). Flame graphs (SDK-064). Export format Decision (SDK-053). GPU samples (SDK-071).

#### Acceptance criteria
- [ ] Samples from a native Component name Task, Component and TaskGroup, not a kernel thread id as the primary key.
- [ ] A recorded profile can be exported beside an `os trace` session.
- [ ] Sampling stays within the B-012 enabled ceiling on H-004 when combined with tracing.

#### Verification
- Integration: SDK-061 scenario on H-001 using this sample stream.
- Bench: B-012 with sampling on, H-001 and H-004.
- Unit: `kernel:tests/obs/profile_attr_*`.

#### Evidence
- none

### OBS-040 · Expose wakeup-to-run latency histograms per Task and intent class
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: OBS-009, SCH-042, SCH-026
- Baseline: §22, §24
- Benchmarks: B-010

SCH intent classes complete at V1 (Realtime). The V1 audio round-trip path needs scheduling delay attributed to intent class, not only per Task. B-010 reads these histograms.

<!-- covers: INV-0465, INV-0460 -->

#### Out of scope
Intent class implementation (SCH). Audio path (AUD). V0 per-Task wakeup-to-run (OBS-009).

#### Acceptance criteria
- [ ] Wakeup-to-run histograms exist per Task and per intent class including Realtime.
- [ ] B-010 on H-004 can read Interactive, Background, LowLatency and Realtime series from OBS.
- [ ] A Task that changes intent class is counted under the class that was active at wakeup.

#### Verification
- Bench: B-010 on H-001 and H-004; target per register.
- Unit: `kernel:tests/obs/sched_hist_*`.
- Integration: AUD round-trip session attributes delay to LowLatency or Realtime.

#### Evidence
- none

### OBS-041 · Export os trace sessions in the decided format for offline viewing
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: OBS-015, OBS-009, OBS-004, OBS-037
- Baseline: §24, §61, §64

V1-G10: `os trace` exports a session that can be viewed offline. Implements OBS-015 including the schema registry external tools use to decode events. SDK-051 is the CLI verb.

<!-- covers: INV-0473, GAP-0533 -->

#### Out of scope
Format Decision (OBS-015). CLI verb (SDK-051). Live follow (OBS-037).

#### Acceptance criteria
- [ ] Exporting a V0 demo session writes a file in the decided format that an offline viewer named in the Decision can open.
- [ ] The schema registry needed to decode events is included or referenced from the file.
- [ ] A session recorded on H-004 opens on a separate machine without the live kernel.

#### Verification
- Integration: export of V1 editor-plus-terminal session on H-004 opens offline.
- Unit: `runtime:tests/obs/trace_export_*`.
- Demo: V1-G10 offline export half.

#### Evidence
- none

### OBS-042 · Log and inspect AI assistant action graphs and their Capability grants
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: SEM-029, SEM-010, OBS-044, OBS-006
- Baseline: §24, §44, §57, §64
- Risks: R-043
- Threats: T-017
- Invariants: I-023, I-051

V2 exit: the AI demo's full action graph is logged and revocable. Records land in OBS-044 and appear in `os inspect` and `os trace`. Depends on a done Semantic interface registry and broker; no AI work precedes that model (§44, §57). The assistant never receives ambient authority.

<!-- covers: INV-0827, INV-0829 -->

#### Out of scope
AI broker and registry (SEM). Action revocation mid-run (CAP-041). Inspect providers for rules (SEM-025).

#### Acceptance criteria
- [ ] Every broker-invoked Semantic interface call appends an audit record with Capability grants.
- [ ] `os inspect` shows the live action graph for the V2 AI demo.
- [ ] The graph is present in `os trace` as semantic events, not GUI scrape.
- [ ] Missing SEM-029 in the closure keeps this task from starting.

#### Verification
- Integration: V2 AI demo on H-002 logs the full graph and one inspect snapshot.
- Unit: `runtime:tests/obs/ai_audit_*`.
- Demo: V2 AI action graph logged and inspectable on H-002.
- Review: SEM lead confirms registry-before-broker ordering.

#### Evidence
- none

### OBS-043 · Expose audit log queries via os inspect for Settings and the Permissions UI
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: OBS-044, OBS-006
- Baseline: §9, §24, §64

Typed query Interface over OBS-044 consumed by `os inspect`, the Settings app (APP-029) and Permissions UI v1 (SEC-045).

<!-- covers: GAP-0234 -->

#### Out of scope
Store (OBS-044). Settings UI (APP-029). Permissions UI (SEC). Per-grant last-use history (OBS-048).

#### Acceptance criteria
- [ ] A typed query returns records filtered by Component, grant type and time range.
- [ ] `os inspect` can list authentication failures, elevation grants and Capability escalations from the store.
- [ ] A caller without audit-query rights receives `Error::Rights` and no records.

#### Verification
- Integration: APP-029 and `os inspect` on H-002 against the query Interface.
- Unit: `runtime:tests/obs/audit_query_*`.
- Review: SEC lead confirms query rights.

#### Evidence
- none

### OBS-044 · Build the tamper-evident size-bounded security audit log
- Type: build
- Milestone: V2
- Status: todo
- Size: L
- Owner: none
- Depends on: OBS-002, SEC-020, SEC-015, BOOT-034
- Baseline: §7, §9, §24
- Threats: T-023

Hash-chained, size-bounded records of authentication failures, elevation grants, Capability escalations, Secure Boot state changes and key enrolments for incident response. Distinct from live tracing and from the structured journal. Emitters in SEC and BOOT write; OBS stores and attests.

<!-- covers: GAP-0234 -->

#### Out of scope
Live Capability events (OBS-002). Query Interface (OBS-043). Journal (OBS-035). Identity implementation (SEC). Measured boot (BOOT).

#### Acceptance criteria
- [ ] Records are hash-chained; a truncated tail fails verification.
- [ ] The store is size-bounded and drops oldest records under the Decision's bound without breaking the chain's current head.
- [ ] Authentication failure, elevation grant, Capability escalation, Secure Boot state change and key enrolment each produce a record when those emitters fire.
- [ ] Journal and trace pipelines do not accept these records as substitutes for the chain.

#### Verification
- Unit: `runtime:tests/obs/audit_chain_*`.
- Integration: login failure (SEC) and measured-boot event (BOOT) land in the chain on H-002.
- Fuzz: `runtime:fuzz/obs_audit_chain` one hour nightly without panic.

#### Evidence
- none

### OBS-045 · Trace and inspect ComputeQueue dispatches, latency and completions
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: OBS-006, OBS-008, HET-008, HET-016
- Baseline: §24, §37, §64
- Benchmarks: B-048

V2 exit: `os inspect` shows the ComputeQueue for the ComputeDevice demo. OBS traces pending dispatches, device, queue latency and completions (§37, §24). HET owns the object; SDK adds the subcommand.

<!-- covers: INV-0455, INV-0699 -->

#### Out of scope
ComputeQueue object and dispatch (HET). CLI subcommand (SDK-065). GPU profiler samples (SDK-071). Native GPU driver stack (forbidden before 1.0).

#### Acceptance criteria
- [ ] Inspect of the V2 ComputeDevice demo names the ComputeQueue, pending dispatches and device.
- [ ] Dispatch submit and complete events appear in `os trace` with queue latency.
- [ ] B-048 can read submit-to-complete intervals from OBS events.

#### Verification
- Integration: V2 ComputeDevice demo on H-002; inspect and trace show the queue.
- Bench: B-048 on H-002 consumes these events; target per register.
- Unit: `kernel:tests/obs/computequeue_*`.
- Demo: V2 ComputeQueue visible in `os inspect` on H-002.

#### Evidence
- none

### OBS-046 · Derive per-display frame deadline-miss metrics across multi-monitor and VRR
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: OBS-013, GFX-088
- Baseline: §40, §24
- Benchmarks: B-019

V2-G compositor deadline-miss gate reads B-019. Extend OBS-013 to multiple displays and variable refresh so BEN's harness can count misses from vblank timestamps without DRM debugfs.

#### Out of scope
B-019 harness (GFX-060, BEN). VRR output (GFX-088). Commit/queue/scanout schema (OBS-013).

#### Acceptance criteria
- [ ] Per-display miss counters are queryable at 60 Hz and at the display's maximum refresh.
- [ ] VRR sessions still produce miss events keyed by the display's reported deadline.
- [ ] B-019 on H-002 and H-005 reads these counters.

#### Verification
- Bench: B-019 on H-002 and H-005; target per register.
- Integration: dual-display plus VRR trace on H-002.

#### Evidence
- none

### OBS-047 · Detect hung Tasks and wait cycles from waiting-on relationships and dump them
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: OBS-005, OBS-018, TSK-020, OBS-032
- Baseline: §20, §21, §24

V2 exit demands zero P0/P1 on the 40-scenario desktop script; hangs are the hardest class. Use waiting-on Operation data to detect cycles and deadline overruns and emit a failure event with a wait-graph snapshot plus flight-recorder dump. Required by V2-G01 (Desktop shell passes the UX script on all three machines).

#### Out of scope
Wait-object recording (TSK-020, OBS-005). UX script (APP-048). Supervisor restart (SVC).

#### Acceptance criteria
- [ ] A constructed two-Task wait cycle emits a hang event with a wait-graph snapshot.
- [ ] A Task past its Operation deadline without completion emits a hang event.
- [ ] The event dump includes the flight-recorder window.
- [ ] A healthy V2 desktop script run produces no hang events.

#### Verification
- Unit: `kernel:tests/obs/wait_cycle_*` on `qemu-x86_64`.
- Integration: APP 40-scenario script on H-002, H-004 and H-005 with hang detector enabled.
- Fuzz: `kernel:fuzz/obs_wait_graph` one hour nightly without panic.

#### Evidence
- none

### OBS-048 · Record per-grant usage history for the Permissions UI audit view
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: OBS-044, OBS-002, OBS-043
- Baseline: §9, §24

V3-G08: a user can see for any application every grant and when it was used. Aggregate live Capability events into per-grant last-use records in the audit store for SEC-062 and SEC-062.

#### Out of scope
Permissions UI (SEC-062, SEC-062). One-time grant semantics (CAP-048). Query Interface (OBS-043).

#### Acceptance criteria
- [ ] Each use of a persistent grant updates last-used time in the audit store.
- [ ] Query by application returns every grant and last-used time for V3-G08.
- [ ] Revoked grants remain listed with revocation time and no further last-used updates.

#### Verification
- Integration: SEC-062 on H-006 against this history.
- Unit: `runtime:tests/obs/grant_usage_*`.

#### Evidence
- none

### OBS-049 · Define crash and panic signatures for symbolicated deduplication
- Type: build
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: OBS-026, OBS-028, OBS-029
- Baseline: §63
- Threats: T-023

V3-G04: crash reports are symbolicated and deduplicated within the window named in that gate. The stable signature over symbolised frames is part of the capture format, computed on device where symbols allow. REL-023 groups on this signature.

<!-- covers: GAP-0151 -->

#### Out of scope
Intake dashboard (REL-023). Consent (INS). Symbol server (BLD, REL).

#### Acceptance criteria
- [ ] Two captures with the same symbolised stack produce one signature.
- [ ] Two captures that differ in a symbolised frame produce different signatures.
- [ ] Signature computation on device uses only local debuginfo from OBS-028 when present.

#### Verification
- Unit: `runtime:tests/obs/crash_signature_*`.
- Integration: two induced panics on H-006 collapse to one signature after symbolication.

#### Evidence
- none

### OBS-050 · Document every field in crash dumps, journal exports and telemetry for privacy review
- Type: docs
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: OBS-029, OBS-034, OBS-026, OBS-051
- Baseline: §63
- Risks: R-048
- Threats: T-023
- Invariants: I-077

V3-G04 privacy review is documented. OBS supplies the data inventory that GOV-061 and INS consent screens depend on. Every field is classified (identity, contents, secrets) so I-077 can be checked.

#### Out of scope
Privacy policy (GOV-061). Consent UI (INS). Telemetry intake (REL-042).

#### Acceptance criteria
- [ ] An inventory lists every field in crash captures, journal exports and local telemetry counters with a privacy class.
- [ ] Disk keys and unlocked secrets are classed as forbidden and mapped to the redaction rule that drops them.
- [ ] A Review line records GOV and SEC sign-off used by V3-G04.

#### Verification
- Review: GOV and SEC leads sign off on the inventory pull request.
- Manual: V3-G04 privacy review cites this inventory.

#### Evidence
- none

### OBS-051 · Record crash-free session and kernel panic metrics locally for opt-in telemetry
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: OBS-026, OBS-027, GOV-055
- Baseline: §63
- Benchmarks: B-041, B-042
- Risks: R-057
- Threats: T-023

V3-G13: opt-in telemetry with crash-free session rate measured. OBS defines and records the on-device counters for B-041 and B-042. Consent client is INS; intake and dashboards are REL. Collection follows GOV-055; opt-in only. Required by V3-G13 (Opt-in telemetry meets the crash-free target).

#### Out of scope
Consent (INS). Intake and dashboards (REL-042, REL-055). Policy (GOV-055).

#### Acceptance criteria
- [ ] On-device counters exist for crash-free sessions, Component crashes, compositor restarts and kernel panics.
- [ ] Counters are not uploaded unless the INS consent Capability is present.
- [ ] B-041 and B-042 definitions can be computed from these counters plus REL intake.
- [ ] Counters contain no file contents, identifiers or dump bodies.

#### Verification
- Unit: `runtime:tests/obs/telemetry_counters_*`.
- Integration: opt-in path on H-006 increments counters; opt-out path uploads nothing.
- Bench: B-041 and B-042 V3 publish; target per register.
- Review: GOV lead confirms the counter schema matches the telemetry Decision.

#### Evidence
- none

### OBS-052 · Lock inspection and trace Interface versions for 1.x with evolution tests
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: OBS-033, IPC-062, IPC-068, OBS-023, OBS-015
- Baseline: §12, §66
- Freezes: S-035

V4 exit: Layer 2 interface versions enumerated and locked; old-client/new-service and new-client/old-service tests pass for every core interface. Inspect and trace schemas are among them. L1 ABI surface S-010 freezes with ABI-049, not here.

#### Out of scope
L1 freeze of S-010 (ABI-049). 1.x stability document (OBS-056). Evolution rules (IPC-042).

#### Acceptance criteria
- [ ] Inspect and trace Interface versions served for 1.x are listed in the L2 lock set.
- [ ] Old-client/new-service and new-client/old-service tests pass for those versions on `qemu-x86_64`.
- [ ] A breaking inspect change without a new version fails CI.

#### Verification
- Integration: IPC L2 evolution matrix entries for inspect and trace on `qemu-x86_64`.
- Review: IPC and ABI leads confirm the lock list.
- Unit: `idl:tests/obs_l2_lock_*`.

#### Evidence
- none

### OBS-053 · Attribute kernel panics per SystemGeneration and hardware for the beta fleet dashboard
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: OBS-051, OBS-049, PKG-019
- Baseline: §63
- Benchmarks: B-042
- Risks: R-057

V4 stability program: B-042 kernel panic rate on the beta fleet. Panic reports carry SystemGeneration, H-ID and signature so REL-055 can attribute regressions. OBS attributes; REL publishes.

#### Out of scope
Dashboard (REL-055). Lab soaks (LAB-024). Generation object (PKG).

#### Acceptance criteria
- [ ] Each panic record includes SystemGeneration identity, H-ID and crash signature.
- [ ] REL-055 can group panics by generation and machine from these records.
- [ ] B-042 V4 target is computable from opted-in records.

#### Verification
- Bench: B-042 on the V4 fleet window; target per register.
- Integration: induced panic on H-009 includes generation and H-ID after reboot.
- Review: REL lead confirms dashboard grouping keys.

#### Evidence
- none

### OBS-054 · Ship Layer 1 tracing-event conformance for the freeze
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: OBS-010, OBS-003, OBS-011, ABI-047
- Baseline: §24, §65, §66
- Freezes: S-010
- Invariants: I-040

V4 freezes L1 tracing event S-010 after the substrate spike and accepted Decision. This task is the conformance suite for event identity, timestamps, relationships and the enable bit that keeps a disabled tracepoint off the hot path. Layer 2 inspect and trace versions lock in OBS-052.

#### Out of scope
Layer 2 inspect/trace lock (OBS-052). Layer 1 freeze adr (ABI-049). Benchmark overhead (OBS-001).

#### Acceptance criteria
- [ ] A conformance suite covers event identity, timestamps, relationships and the disabled-tracepoint enable bit on `qemu-x86_64` and `hw-h002`.
- [ ] Surface S-010 is listed as frozen by this task in the surfaces register.
- [ ] A change to S-010 without an accepted superseding Decision fails CI.

#### Verification
- Integration: `obs:tests/l1/trace_event_conformance_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Review: OBS and ABI leads sign off on the pull request that lands the freeze.

#### Evidence
- none

### OBS-055 · Publish the 30-day soak leak and hang report for the release candidate
- Type: docs
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: OBS-036, OBS-047, LAB-025
- Baseline: §24, §63

1.0 exit: release-candidate soak with zero open P0/P1. OBS runs leak detection and hang detection across that soak on the Tier 1 fleet and publishes the result. LAB executes the fleet soak; BLD owns the matrix jobs.

#### Out of scope
Fleet soak execution (LAB-025). Channel launch (REL). Hang detector (OBS-047).

#### Acceptance criteria
- [ ] A committed report covers leak assertions and hang events for every Tier 1 machine in the 1.0 soak.
- [ ] The report lists zero open P0/P1 leak or hang findings, or names the accepted Decision that defers a finding.
- [ ] A Review line records LAB and OBS lead sign-off.

#### Verification
- Review: LAB and OBS leads sign off on the report pull request.
- Integration: leak_guard and hang detector attached to LAB-025 jobs.

#### Evidence
- none

### OBS-056 · Publish the 1.x stability statement for inspection and trace interfaces
- Type: docs
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: OBS-052, IPC-070
- Baseline: §66

1.0 exit: Layer 2 supported interface versions listed in a published compatibility document with deprecation policy. This task authors the inspect and trace sections of that document.

#### Out of scope
The document chassis and deprecation policy (IPC-070). L1 ABI stability statement (ABI-053). Version lock (OBS-052).

#### Acceptance criteria
- [ ] The published 1.x compatibility document lists inspect and trace Interface versions with deprecation policy.
- [ ] The text states that Layer 1 tracing event ABI surface S-010 changes require a new major version, citing ABI-053.
- [ ] A Review line records IPC, ABI and OBS lead sign-off.

#### Verification
- Review: IPC, ABI and OBS leads sign off on the pull request.

#### Evidence
- none
