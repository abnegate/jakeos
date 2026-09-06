# SEM · Semantic interfaces, automation, AI
- Prefix: SEM
- Lead: none
- Baseline: §13, §14, §21, §31, §41, §42, §43, §44, §45, §57, §62, §66, §67

<!-- roadmap:generated:begin summary -->
Tasks: 44 live, 0 done, 0 in-progress, 44 todo, 0 dropped. Ready: 0. Blocked: 44. Weighted: 0%.
<!-- roadmap:generated:end -->

## Scope
SEM owns typed Semantic interfaces applications expose for what they do, the registry that discovers them, caller permissioning, the application event model, Automation rules over those interfaces, and the AI broker that obtains typed Capabilities to invoke them. Work lands in dependency order: registry, then discovery and grants, then Automation rules, then the AI broker. Native callers never synthesize mouse or keyboard input and never scrape a GUI. The OS supplies Capability plumbing, not a bundled model runtime. Catalog Interfaces (Terminal, Editor, Workspace, Mail, Download, Meeting, Extractor, Notes) and their Layer 2 version lock, conformance and evolution tests live here.

## Out of scope
IDL language, compiler and Channel transport (IPC). Capability mint, derive, revocation and `Capability<BackgroundExecution>` (CAP). Component spawn and graphs (CMP). Toolkit widget actions and UI protocol (UIP). Accessibility tree schema and screen reader (ACC). Shell chrome, consent UI, Terminal, Editor and Settings implementations (APP). `os env` objects (ENV). ComputeDevice enumeration and dispatch (HET). Wasm host for automation modules (WASM). Audit log store and `os inspect` CLI (OBS, SDK). Grant taxonomy, AI principal and multi-user sessions (SEC). Manifest schema and history storage (PKG). `file.type` and UserSelected minting (STO). Docs site generation (DOC). Fuzz fleet (BLD). Benchmark methodology (BEN). Remote and distributed transports (VIRT, LATER).

## Tasks

### SEM-001 · Gate AI broker crates on a done Semantic Interface registry
- Type: build
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: BLD-011
- Baseline: §44, §57, §67
- Risks: R-043
- Threats: T-017
- Invariants: I-023, I-051

Standing CI lint so AI broker and assistant-host crates cannot land until a Semantic interface registry crate exists and they declare a dependency on it. The assistant is not an omnipotent privileged process: crates tagged as the broker or host fail CI if they request ambient authority or skip the registry. This is the mechanical form of §57 and §67 principle 13 from V0, before any Semantic interface ships.

<!-- covers: INV-0819, INV-0833, INV-1125, INV-1304 -->

#### Out of scope
Registry implementation (SEM-007, SEM-029). Assistant host (SEM-012). AI broker (SEM-010). Input-synthesis lint (SEM-002).

#### Acceptance criteria
- [ ] A planted `ai_broker` or `assistant_host` crate with no dependency on a crate tagged `semantic-registry` fails CI on `qemu-x86_64`.
- [ ] A planted broker crate whose manifest requests a wildcard Capability set fails CI.
- [ ] Native crates that are not tagged broker or host pass the lint on `qemu-x86_64`.

#### Verification
- Unit: `sdk:tests/lint/sem_ai_after_registry_*` fixtures that fail without a registry dependency and pass with one.
- Integration: CI job on every native crate in the V0 image.
- Review: SEM reviewer confirms the tag list covers broker and assistant-host crate names.

#### Evidence
- none

### SEM-002 · Forbid input synthesis and GUI scraping in Semantic Interface code
- Type: build
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: BLD-011
- Baseline: §1, §42, §45, §57, §67
- Threats: T-017, T-018
- Invariants: I-023

Standing CI lint so no Semantic interface, Automation rule or AI caller is implemented by synthesising mouse or keyboard input or by GUI scraping. Banned symbols include uinput, XTest, evdev injection, accessibility-tree-as-input and screenshot OCR in SEM crates and in first-party Semantic interface implementations. This is §67 principle 12 as a merge gate from V0.

<!-- covers: INV-0797, INV-0838, INV-1303, INV-0044 -->

#### Out of scope
Accessibility tree as an AT client (ACC). Toolkit input routing (UIP). Personality input emulation (LNX, WIN). AI crate ordering (SEM-001).

#### Acceptance criteria
- [ ] A planted Semantic interface crate that calls uinput, XTest or evdev inject fails CI on `qemu-x86_64`.
- [ ] A planted caller that drives widgets through the accessibility tree as a substitute for a typed Interface fails CI.
- [ ] First-party native crates pass the lint on `qemu-x86_64`.

#### Verification
- Unit: `sdk:tests/lint/sem_no_input_synthesis_*` fixtures covering the banned-symbol list.
- Integration: CI job on every native crate in the V0 image.
- Review: SEM reviewer confirms the banned list covers input synthesis and GUI scraping.

#### Evidence
- none

### SEM-003 · Prototype one Editor method as a typed Semantic Interface
- Type: spike
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: SEM-002, IPC-012, APP-003
- Baseline: §14, §42, §60
- Explores: S-023

Prototype a single Editor method as a typed Semantic interface over Channel, invoked by a native caller with no GUI input, before V1 discovery permissioning is decided. V0.5 allows interfaces the four apps need; this spike is the measured Editor-method evidence that S-023 and SEM-004 require. Record IDL annotation needs and that the call does not synthesize input. Required by V1-G12 (Semantic interfaces and a Wasm channel prototype).

#### Out of scope
Discovery and caller permissioning (SEM-004). Session-local registry (SEM-007). Terminal.run (SEM-008). IDL compiler (IPC-012).

#### Acceptance criteria
- [ ] A report at `reports/spikes/SEM-003.md` records one Editor method invoked over Channel from a native Component on H-001 and H-003.
- [ ] The prototype IDL contains no mouse, keyboard or accessibility-tree input types.
- [ ] The report names the IDL annotations a registry would need (verb, object type, automation exposure) and which option is ruled out.

#### Verification
- Report: answers (1) whether one Editor method can be invoked over Channel without GUI input, (2) which IDL annotations a registry needs, (3) how the call compares to opening the same document through the Editor UI on B-004's method, (4) what V1 must not freeze about S-023.
- Review: SEM reviewer records that SEM-004 cites this report.

#### Evidence
- none

### SEM-004 · Decide Semantic Interface discovery and caller permissioning
- Type: adr
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: SEM-003, CAP-007, SEC-007
- Baseline: §9.1, §42, §44
- Decision: D-0278
- Risks: R-043
- Threats: T-001, T-002, T-017
- Invariants: I-021

Decide how Semantic interfaces are discovered and how automation and AI callers obtain authority to invoke them, before any Semantic interface ships at V1. Shipping exposure without this choice would grant callers de facto ambient authority. The accepted option names the grant source, the session-local versus system-wide lookup shape, and that a missing grant returns `Error::Rights` and allocates no handle.

<!-- covers: GAP-0551 -->

#### Out of scope
Session-local registry implementation (SEM-007). Cross-app grants (SEM-026). Consent UI (APP-025). Grant taxonomy classes (SEC-007).

#### Acceptance criteria
- [ ] Options evaluated include at least: (A) per-interface Capability grants; (B) manifest-declared exposure with session grants; (C) a user-consent bind flow.
- [ ] The accepted option names how a caller finds an Interface and how a missing grant fails, and names that no ambient lookup exists.
- [ ] Each option cites SEM-003, T-001 and I-021; rejected options record how they reintroduce ambient authority.
- [ ] Review records CAP and SEC lead sign-off on the pull request.

#### Verification
- Review: CAP lead and SEC lead sign-off recorded on the pull request.
- Manual: decision file lists at least two options with consequences, cites `reports/spikes/SEM-003.md`, and names T-001.

#### Evidence
- none

### SEM-005 · Publish Semantic Interface authoring guidelines
- Type: docs
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: SEM-004, SEM-002, IPC-032
- Baseline: §12, §42, §52

Publish verb and object rules, the no-input-synthesis rule, and Capability-passing idioms for IDL authors of Semantic interfaces. Complements IPC interface-design guidelines with the §42 contract so SDK v1 authors do not invent GUI-scraping Interfaces. Required by V3-G12 (Layer 1 ABI reference pages exist for every entry point).

#### Out of scope
IDL language and evolution rules (IPC). Docs site generation (DOC-010). Public-alpha user guide (SEM-036).

#### Acceptance criteria
- [ ] A page in the V1 SDK docs names verb/object naming, Capability-passing and the ban on input synthesis, with a worked Terminal.run example.
- [ ] The page cites SEM-004 and IPC-032 rather than restating Channel rules.
- [ ] A fixture Interface that documents mouse synthesis is rejected by the authoring lint in CI.

#### Verification
- Review: SEM and IPC reviewers sign off on the pull request that lands the page.
- Unit: `sdk:tests/lint/sem_authoring_*` rejects the planted synthesis example.

#### Evidence
- none

### SEM-006 · Let applications expose typed Semantic interfaces from IDL
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: SEM-004, SEM-002, IPC-051, IPC-012
- Baseline: §14, §42
- Invariants: I-023

Applications expose typed Semantic interfaces defined in the native IDL so Terminal.run and Editor.open can ship as proof Interfaces at V1. Generated stubs carry the semantic annotations IPC owns; SEM owns the expose path, the no-synthesis check and the Capability-shaped arguments. Native software never sees POSIX, Win32 or accessibility-tree input as a Semantic interface.

<!-- covers: INV-0788 -->

#### Out of scope
IDL annotation syntax (IPC-051). Session-local registry (SEM-007). App-side Terminal.run and Editor.open (APP-015).

#### Acceptance criteria
- [ ] An application IDL marked semantic generates stubs that register the Interface with the session-local registry after SEM-007 exists; a missing annotation does not register.
- [ ] Generated argument types are Capability or IDL values; a path-string or widget-handle argument fails the IDL lint.
- [ ] Invoking an exposed method from a native Component on `qemu-x86_64` completes without posting mouse or key events.

#### Verification
- Unit: `sem:tests/expose/idl_semantic_*` on `qemu-x86_64`.
- Integration: Terminal.run and Editor.open stubs generated from IDL on `qemu-x86_64`.
- Review: IPC reviewer confirms annotations match IPC-051.

#### Evidence
- none

### SEM-007 · Ship a session-local Semantic Interface registry
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: SEM-006, SEM-004, PKG-057, OBS-019
- Baseline: §42, §61
- Risks: R-043
- Invariants: I-021, I-051

Ship a session-local registry so a caller in the same session can look up Terminal.run and Editor.open without the V2 system-wide service. Lookup is the S-023 prototype ACC-008 and PKG-057 consume. A Component without a grant from SEM-004 receives `Error::Rights` and no handle.

<!-- covers: INV-0798 -->

#### Out of scope
System-wide live registry (SEM-029). One-tree-or-two Decision (ACC-008). Manifest schema (PKG-057). AI broker (SEM-010).

#### Acceptance criteria
- [ ] `os inspect` of the session registry lists Terminal.run and Editor.open after those applications start on `qemu-x86_64`.
- [ ] Lookup of an Interface the caller was not granted returns `Error::Rights` and allocates no handle.
- [ ] Killing the Editor removes Editor.open from lookup within one Operation; a subsequent lookup returns not-found, not a stale handle.
- [ ] No AI broker crate is part of this change; SEM-001 still fails a planted broker crate that does not depend on this registry crate.

#### Verification
- Unit: `sem:tests/registry/v0_lookup_*` on `qemu-x86_64`.
- Integration: V1 typed-automation path on H-001 listing Terminal.run and Editor.open via `os inspect`.
- Review: ACC reviewer confirms the lookup shape is usable as input to ACC-008.

#### Evidence
- none

### SEM-008 · Ship Terminal.run and Editor.open Semantic Interface v0
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: SEM-007, APP-015, APP-016, APP-011
- Baseline: §42, §61
- Invariants: I-023

Ship Terminal.run and Editor.open as Semantic interface v0 and retain a permanent automation script that invokes both through typed Interfaces with no GUI input. APP implements the two proof Interfaces; SEM owns the v0 contract, the registry binding and the CI scenario that V1-G12 and V1-D04 require.

#### Out of scope
Editor.openProject, searchSymbols and runTests (SEM-022). `os call` CLI (SDK-066). Screen reader as a semantic client (ACC-022).

#### Acceptance criteria
- [ ] An automation script on `qemu-x86_64` looks up Terminal.run and Editor.open in the session registry, invokes both, and posts no mouse or key events.
- [ ] The same script is a permanent CI test on `qemu-x86_64` and is recorded as the V1 typed-automation demo path.
- [ ] A caller without the grant receives `Error::Rights` and allocates no handle; Terminal and Editor still run.

#### Verification
- Integration: `sem:tests/v1/terminal_run_editor_open_*` on `qemu-x86_64`.
- Demo: V1-D04 on H-002, shown beside `os trace`.
- Review: APP reviewer confirms the implementations match the v0 IDL.

#### Evidence
- none

### SEM-009 · Demonstrate Workspace.search to IDE.runTests action graph
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: SEM-010, SEM-035, SEM-022, SEM-033, CAP-041, OBS-042
- Baseline: §44, §62
- Risks: R-043
- Threats: T-017
- Invariants: I-023, I-051

Demonstrate the V2 AI-assistant path: Workspace.search returns a Project Capability, Editor.openProject consumes it, Editor.runTests runs, the full action graph is logged, and one step is revocable mid-run. This is V2-G12 and V2-D05. The graph uses only typed Interfaces; no GUI input is posted.

<!-- covers: INV-0824 -->

#### Out of scope
Revocation walk (CAP-041). Durable audit store (OBS-042). Rule-engine examples (SEM-019).

#### Acceptance criteria
- [ ] On H-002 the assistant completes Workspace.search to Editor.openProject to Editor.runTests for the Postgres-branch fixture without posting mouse or key events.
- [ ] `os inspect` of the live graph names every hop, the Capability granted at each hop, and the broker principal.
- [ ] Revoking the Project Capability mid-run causes the next Editor hop to return `Error::Rights`, allocate no new handle, and leave the graph inspectable.
- [ ] The scenario is a permanent integration test on `qemu-x86_64`.

#### Verification
- Integration: `sem:tests/v2/action_graph_postgres_*` on `qemu-x86_64`.
- Demo: V2-D05 on H-002 with the graph and grants displayed.
- Review: CAP reviewer confirms the mid-run revoke matches CAP-041.

#### Evidence
- none

### SEM-010 · Implement the AI broker over typed Capabilities
- Type: build
- Milestone: V2
- Status: todo
- Size: L
- Owner: none
- Depends on: SEM-029, SEM-013, SEM-012, SEM-026, SEM-001, SEC-034
- Baseline: §42, §44, §57, §67
- Risks: R-043
- Threats: T-017
- Invariants: I-023, I-051

Implement the AI broker as the Component through which an assistant obtains typed, permissioned, logged, revocable and scoped Capabilities and composes IDL-only action graphs across applications. The broker is not omnipotent, does not synthesize input, and cannot land until SEM-029 is done. Logging and mid-run revoke are consumed by OBS and CAP; this task is the grant and invoke path.

<!-- covers: INV-0795, INV-0825, INV-0831 -->

#### Out of scope
Assistant host process (SEM-012). Scoped-grant policy (SEC-033). Action-graph audit store (OBS-042). Mid-run revoke walk (CAP-041). Model runtime (SEM-017).

#### Acceptance criteria
- [ ] The broker Component starts with only the Capabilities it was granted; a planted ambient filesystem or input-synthesis Capability in its manifest fails SEM-001 and SEM-002.
- [ ] Compose of an IDL method the broker was not granted returns `Error::Rights` and allocates no handle.
- [ ] A two-hop graph across two applications is submitted only as typed Interface calls listed in the registry; a non-IDL action type is rejected.
- [ ] Every hop emits an inspectable graph node naming the Interface, the Capability and the outcome, consumed by OBS-042.
- [ ] The broker crate depends on the registry crate; CI fails if that dependency is removed.

#### Verification
- Unit: `sem:tests/broker/grant_deny_compose_*` on `qemu-x86_64`.
- Integration: two-application graph on H-002 with `os inspect` of the broker Component.
- Review: SEC reviewer confirms the principal matches SEC-034.

#### Evidence
- none

### SEM-011 · Define the typed application event model
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: SEM-006, SEM-013, STO-018
- Baseline: §42, §45
- Threats: T-018

Applications emit typed events such as Download.completed(file) and Meeting.start over Semantic interfaces. Events carry delegated Capabilities, not paths. STO-018 owns `file.type`; this task is the event IDL, emission and subscription so the rule engine can match without sniffing bytes or scraping windows.

<!-- covers: INV-0839 -->

#### Out of scope
`file.type` registry (STO-018). Rule evaluation (SEM-031). Download UI (APP). Meeting product (APP).

#### Acceptance criteria
- [ ] Download.completed carries a file Capability; a subscriber without that Capability cannot open the bytes and receives `Error::Rights`.
- [ ] Meeting.start carries a meeting Capability usable by Notes.create; no path string is present on the wire.
- [ ] An event whose payload is a filesystem path instead of a Capability fails the IDL lint.
- [ ] `os inspect` of a subscribed Component names the event type and the delegated Capability.

#### Verification
- Unit: `sem:tests/events/download_meeting_*` on `qemu-x86_64`.
- Integration: fixture Download and Meeting emitters on H-002, inspected via `os inspect`.
- Review: STO reviewer confirms `file.type` uses STO-018.

#### Evidence
- none

### SEM-012 · Host the assistant runtime as an ordinary Component
- Type: build
- Milestone: V2
- Status: todo
- Size: L
- Owner: none
- Depends on: SEM-029, SEM-017, SEM-001, SEM-033, SEC-034, CMP-005
- Baseline: §10, §44, §57
- Threats: T-017
- Invariants: I-023, I-051

Host the assistant runtime as an ordinary Component with an explicit Capability set, not a privileged process and not a kernel-resident model runtime. Placement of weights follows SEM-017; the OS still supplies only Capability plumbing. The host cannot land until SEM-029 is done.

<!-- covers: INV-0821, INV-0820 -->

#### Out of scope
Broker grant path (SEM-010). ComputeDevice dispatch (HET). Model-placement Decision (SEM-017). Wasm plugin host (WASM-009).

#### Acceptance criteria
- [ ] The host is a Component visible to `os inspect` with a finite Capability set and no wildcard grant.
- [ ] Destroying the host Component reclaims its Tasks, Channels and Capabilities; a leak test of repeated create and destroy shows no unbounded kernel-object growth.
- [ ] The host crate depends on the registry crate; CI fails if that dependency is removed or if the crate is built before SEM-029 exists.
- [ ] No in-tree crate named as a model runtime is linked into the kernel or into a privileged supervisor; the accepted SEM-017 option is the only execution path.
- [ ] A planted host that posts mouse or key events fails SEM-002.

#### Verification
- Unit: `sem:tests/host/ordinary_component_*` on `qemu-x86_64`.
- Integration: host start and teardown on H-002 with `os inspect` of the Component.
- Review: CMP reviewer confirms the host is a normal Component; SEC reviewer confirms the principal.

#### Evidence
- none

### SEM-013 · Implement the automation layer over Semantic interfaces
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: SEM-029, SEM-026
- Baseline: §42, §45
- Risks: R-043
- Invariants: I-023

Implement the client runtime that automation and scripting callers use to bind and invoke Semantic interfaces with typed Capabilities. This is the layer between the registry and the rule engine, and the native path SDK-066 consumes. Callers never receive ambient authority and never synthesize input.

<!-- covers: INV-0837, INV-0791 -->

#### Out of scope
`os call` CLI (SDK-066). Rule trigger engine (SEM-031). Automation-rule editor UI (APP-059). Screen reader client (ACC-022).

#### Acceptance criteria
- [ ] Bind to a registered Interface with a grant succeeds and returns a typed stub; bind without a grant returns `Error::Rights` and allocates no handle.
- [ ] Invoke of Terminal.run through the layer on `qemu-x86_64` completes with no mouse or key events.
- [ ] Dropping the grant invalidates the stub; the next invoke returns `Error::Rights`.
- [ ] The layer crate has no dependency on an AI broker crate.

#### Verification
- Unit: `sem:tests/automation/bind_invoke_deny_*` on `qemu-x86_64`.
- Integration: scripted Terminal.run through the layer on H-002.
- Review: SEM reviewer confirms the layer does not import broker crates.

#### Evidence
- none

### SEM-014 · Publish Semantic Interface registry lookup latency
- Type: benchmark
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: SEM-029, BEN-005, IPC-008
- Baseline: §42, §54, §62
- Benchmarks: B-004
- Invariants: I-061

Publish registry lookup latency on V2 reference machines beside B-004 so V2 semantic automation never states an unmeasured number. The harness uses B-004's method for a lookup-sized typed message against the system registry; SEM does not claim B-004's absolute target. Numbers live only in the register and in `reports/benchmarks/B-004/`.

#### Out of scope
IPC round-trip harness (IPC-008). Rule-dispatch latency (SEM-015). Methodology (BEN-007).

#### Acceptance criteria
- [ ] Harness `bench:sem-registry-lookup` runs on H-002, H-004 and H-005 using B-004's method.
- [ ] A V2 report exists under `reports/benchmarks/B-004/` for each of those H-IDs naming the lookup measurement beside the Channel round trip, with no superiority claim in the prose.
- [ ] The harness is retained so later B-051 sessions re-run it without a new client.

#### Verification
- Bench: B-004 method on H-002, H-004 and H-005; SEM lookup published beside the B-004 table; no SEM absolute claimed.
- Integration: CI job on `qemu-x86_64` records lookup samples from the harness.

#### Evidence
- none

### SEM-015 · Publish Automation rule dispatch latency
- Type: benchmark
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: SEM-031, SEM-019, BEN-005
- Baseline: §45, §54, §62
- Benchmarks: B-009
- Invariants: I-061

Publish event-to-action latency of the rule engine so the V2 automation gate never states an unmeasured number. The harness uses B-009's method from event delivery to action Operation completion on the Download.completed to Extractor.extract fixture. SEM does not claim a B-009 absolute; numbers live only in the register and reports.

#### Out of scope
Rule engine (SEM-031). Registry lookup harness (SEM-014). Methodology (BEN).

#### Acceptance criteria
- [ ] Harness `bench:sem-rule-dispatch` runs on H-002, H-004 and H-005 using B-009's method on the Download.completed fixture.
- [ ] A V2 report exists under `reports/benchmarks/B-009/` for each of those H-IDs naming the dispatch measurement, with no superiority claim in the prose.
- [ ] The harness is retained so later B-051 sessions re-run it.

#### Verification
- Bench: B-009 method on H-002, H-004 and H-005; SEM dispatch published beside the B-009 table; no SEM absolute claimed.
- Integration: CI job on `qemu-x86_64` records dispatch samples from the harness.

#### Evidence
- none

### SEM-016 · Prove cross-app integration over Semantic interfaces
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: SEM-029, SEM-026, SEM-027, SEM-022
- Baseline: §42, §62
- Invariants: I-023

Two distinct applications discover each other through the system registry and complete a typed call with a transferred Capability. This is the standing proof that Semantic interfaces support cross-app integration, retained as a permanent test on `qemu-x86_64` and the V2 desktop.

<!-- covers: INV-0794 -->

#### Out of scope
Mail product sync (APP). Editor implementation (APP-011). AI broker graphs (SEM-010).

#### Acceptance criteria
- [ ] Mail fixture and Editor discover each other through the system registry on `qemu-x86_64` and on H-002.
- [ ] Mail.compose transfers a draft Capability that Editor.open consumes; page identity of any MemoryObject payload is preserved.
- [ ] Without the cross-app grant the same call returns `Error::Rights` and allocates no handle.
- [ ] The scenario is a permanent integration test on `qemu-x86_64`.

#### Verification
- Integration: `sem:tests/v2/cross_app_mail_editor_*` on `qemu-x86_64` and H-002.
- Review: APP reviewer confirms neither side synthesizes input.

#### Evidence
- none

### SEM-017 · Decide where assistant models execute
- Type: adr
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: SEM-032, HET-001, SEM-029
- Baseline: §37, §44, §57
- Decision: D-0279
- Invariants: I-051

Decide where assistant models execute: local ComputeDevice or NPU, a remote service, or user-selectable both. Reject an OS-bundled model runtime. The OS provides Capability plumbing; HET owns device enumeration. The spike report is the evidence this adr must cite.

<!-- covers: INV-0834 -->

#### Out of scope
ComputeDevice ABI (HET). Assistant host Component (SEM-012). Remote shell (NET).

#### Acceptance criteria
- [ ] Options evaluated include at least: (A) local ComputeDevice or NPU only; (B) remote service only; (C) user-selectable both.
- [ ] Every option states that the OS does not bundle a model runtime; an option that ships weights as a privileged system service is rejected.
- [ ] Each option cites `reports/spikes/SEM-032.md` and HET-001.
- [ ] Review records HET lead sign-off on the pull request.

#### Verification
- Review: HET lead sign-off recorded on the pull request.
- Manual: decision file lists at least two options with consequences and names that a bundled model runtime is rejected.

#### Evidence
- none

### SEM-018 · Decide the Automation rule format
- Type: adr
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: SEM-034
- Baseline: §13, §45, §62
- Decision: D-0280
- Invariants: I-046

Decide the Automation rule format for the V2 automation-rules-model Decision: declarative rules, a scripting language, or Wasm automation modules. WASM owns a Wasm host if that option wins. The spike report is the evidence this adr must cite.

<!-- covers: INV-0844 -->

#### Out of scope
Rule engine (SEM-031). Wasm host (WASM-015). Rule editor UI (APP-059).

#### Acceptance criteria
- [ ] Options evaluated include at least: (A) declarative when-event if-condition action rules; (B) a sandboxed scripting language; (C) Wasm automation modules.
- [ ] The accepted option names how a rule holds only delegated Capabilities and how a background rule requires `Capability<BackgroundExecution>`.
- [ ] Each option cites `reports/spikes/SEM-034.md`; the Wasm option names WASM-015 as the host and does not make Wasm the Native ABI.
- [ ] Review records WASM lead sign-off if option C remains in play.

#### Verification
- Review: SEM lead sign-off recorded on the pull request; WASM lead sign-off if option C is accepted or kept.
- Manual: decision file lists at least two options with consequences and cites the spike report.

#### Evidence
- none

### SEM-019 · Demonstrate Download.completed to Extractor.extract
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: SEM-031, SEM-030, SEM-024, SEM-011, STO-018
- Baseline: §45, §62
- Threats: T-018
- Invariants: I-023

First of three shipped example rules: Download.completed with `file.type` Archive delegates the file Capability to Extractor.extract and runs without GUI input. STO owns `file.type`; SEM owns the rule and the demo.

<!-- covers: INV-0841 -->

#### Out of scope
Extractor fixture (SEM-024). Archive product (APP). `file.type` (STO-018).

#### Acceptance criteria
- [ ] On H-002 a Download.completed event with `file.type` Archive causes Extractor.extract to run with the delegated file Capability and no mouse or key events.
- [ ] A Download.completed whose type is not Archive does not invoke Extractor.extract.
- [ ] The rule holds only the delegated file Capability; a planted ambient home grant in the rule fails SEM-030.

#### Verification
- Integration: `sem:tests/v2/demo_download_extract_*` on `qemu-x86_64` and H-002.
- Demo: V2-G12 first example rule on H-002.

#### Evidence
- none

### SEM-020 · Demonstrate Meeting.start to Notes.create
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: SEM-031, SEM-024, SEM-011
- Baseline: §45, §62
- Invariants: I-023

Second shipped example rule: Meeting.start triggers Notes.create(meeting) through typed Capabilities with the action logged. No meeting or notes product is invented; fixtures expose the Interfaces.

<!-- covers: INV-0842 -->

#### Out of scope
Notes fixture (SEM-024). Calendar product (APP). History events (SEM-038).

#### Acceptance criteria
- [ ] On H-002 Meeting.start causes Notes.create with the meeting Capability and no mouse or key events.
- [ ] The action appears in `os inspect` of the live rule graph with the delegated Capability named.
- [ ] Without the meeting Capability Notes.create returns `Error::Rights` and allocates no handle.

#### Verification
- Integration: `sem:tests/v2/demo_meeting_notes_*` on `qemu-x86_64` and H-002.
- Demo: V2-G12 second example rule on H-002.

#### Evidence
- none

### SEM-021 · Demonstrate Project opened to environment start
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: SEM-031, SEM-022, ENV-029
- Baseline: §35, §45, §62
- Invariants: I-023

Third shipped example rule: when Editor.openProject succeeds, invoke ENV via a Semantic interface to start the project environment. ENV owns `os env` and startEnvironment; SEM owns the rule.

#### Out of scope
Environment objects (ENV). Editor.openProject (SEM-022). Restore of environments (ENV-028).

#### Acceptance criteria
- [ ] On H-002 Editor.openProject on the fixture Project causes ENV startEnvironment with the Project Capability and no mouse or key events.
- [ ] A Project opened without the environment grant does not start an environment and returns `Error::Rights` on the ENV hop.
- [ ] The rule is listed beside the other two V2-G12 examples in `os inspect`.

#### Verification
- Integration: `sem:tests/v2/demo_project_env_*` on `qemu-x86_64` and H-002.
- Demo: V2-G12 third example rule on H-002.

#### Evidence
- none

### SEM-022 · Ship the reference Editor Semantic Interface in the native editor
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: SEM-008, SEM-028, SEM-029, APP-011
- Baseline: §42, §60, §61, §62

Extend V1 Editor.open with Editor.openProject(Project), searchSymbols(Query) returning Symbol[], and runTests(TestSelection) returning TestRun on the native editor. APP owns the editor; SEM owns the Semantic interface used by the V2 action-graph demo.

<!-- covers: INV-0789 -->

#### Out of scope
Native IDE product (APP-020). Project Capability type (SEM-028). Workspace.search (SEM-035).

#### Acceptance criteria
- [ ] Editor.openProject accepts a Project Capability and rejects a path string at the IDL boundary.
- [ ] searchSymbols and runTests are registered in the system registry and invocable through SEM-013 on `qemu-x86_64`.
- [ ] Invoking runTests posts no mouse or key events; results return as TestRun.
- [ ] A caller without the Editor grant receives `Error::Rights` and allocates no handle.

#### Verification
- Unit: `sem:tests/editor/open_search_tests_*` on `qemu-x86_64`.
- Integration: native Editor on H-002 exercising the three methods from a script.
- Review: APP reviewer confirms the native editor implements the IDL.

#### Evidence
- none

### SEM-023 · Expose Semantic interfaces from shipped native UI applications
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: SEM-029, SEM-026, APP-040, APP-043
- Baseline: §41, §42, §45
- Invariants: I-023

Shipped native UI applications (shell, the four apps, Settings) expose Semantic interfaces for automation rather than widget scraping. Toolkit-level actions stay in UIP; this task is the per-application catalog of open, search, settings and notification Interfaces registered with the system registry.

<!-- covers: INV-0774 -->

#### Out of scope
Toolkit widget actions (UIP). Shell chrome (APP). Accessibility actions (ACC). Automation-rule editor (APP-059).

#### Acceptance criteria
- [ ] Shell, Settings, Terminal, File Browser, Text Editor and Image Viewer each register at least one Semantic interface visible in the system registry on H-002.
- [ ] A scripted caller invokes one method per application with no mouse or key events.
- [ ] An application crate that implements a Semantic interface by posting input fails SEM-002.
- [ ] Denying the grant leaves the application running and returns `Error::Rights` to the caller.

#### Verification
- Integration: `sem:tests/v2/shell_app_interfaces_*` on `qemu-x86_64` and H-002.
- Review: APP reviewer confirms each shipped app registers Interfaces rather than scraping widgets.

#### Evidence
- none

### SEM-024 · Ship Extractor and Notes fixture Components
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: SEM-006, SEM-029
- Baseline: §11, §45

Ship Extractor.extract(file) and Notes.create(meeting) as fixture Components so V2 example rules do not invent a mail or archive product. Fixtures are ordinary Components with explicit Capability sets, registered in the system registry. Required by V2-G12 (Semantic automation and an AI-assistant demo).

#### Out of scope
Archive manager product (APP-051). Notes or mail client (APP). Rule engine (SEM-031).

#### Acceptance criteria
- [ ] Extractor.extract accepts a file Capability and rejects a path string; Notes.create accepts a meeting Capability.
- [ ] Both fixtures appear in the system registry on `qemu-x86_64`.
- [ ] Neither fixture crate posts mouse or key events; SEM-002 passes on them.

#### Verification
- Unit: `sem:tests/fixtures/extractor_notes_*` on `qemu-x86_64`.
- Integration: registry listing of both fixtures on H-002.

#### Evidence
- none

### SEM-025 · Expose registry, rules, and action graphs to os inspect
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: SEM-029, SEM-031, SEM-010, SEM-009, OBS-019, SDK-007
- Baseline: §24, §44, §64
- Invariants: I-034

Register `os inspect` providers for the Semantic interface registry, Automation rules and live action graphs so the V2 AI demo's graph is inspectable. OBS owns the inspect interface and audit storage; SEM emits the records.

<!-- covers: INV-0829 -->

#### Out of scope
Inspect CLI rendering (SDK-007). Durable audit log (OBS-044). AI action audit schema (OBS-042).

#### Acceptance criteria
- [ ] `os inspect` of the registry lists live Interfaces, grant holders and denying callers on H-002.
- [ ] `os inspect` of Automation rules lists each shipped example rule, its trigger and its held Capabilities.
- [ ] `os inspect` of a live AI graph during SEM-009 names every hop and the Capability at that hop.

#### Verification
- Integration: inspect dumps on H-002 for registry, rules and a live graph.
- Review: OBS reviewer confirms providers match OBS-019.

#### Evidence
- none

### SEM-026 · Require a Capability to invoke another app's Semantic Interface
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: SEM-029, SEM-004, CAP-003, SEC-006
- Baseline: §7, §9.1, §42
- Threats: T-001, T-002, T-017
- Invariants: I-021

Access to another application's Semantic interface requires a Capability granted by the user or policy. This implements SEM-004 for cross-app callers: missing grant returns `Error::Rights` and allocates no handle. Consent UI stays in APP; this task is the check.

<!-- covers: INV-0799 -->

#### Out of scope
Consent prompt UI (APP-025). Grant taxonomy (SEC-007). Session-local v0 lookup (SEM-007).

#### Acceptance criteria
- [ ] Invoke of another application's Interface without a grant returns `Error::Rights` and allocates no handle, recorded in the Capability audit log.
- [ ] A user or policy grant allows the same invoke; revoke makes the next invoke fail within one Operation.
- [ ] The callee is not a confused deputy: it acts only with the caller's attenuated Capability, not its own extra rights (T-002).
- [ ] Same-application invoke of a self-exposed Interface still requires the grant named by SEM-004.

#### Verification
- Unit: `sem:tests/grants/cross_app_deny_*` on `qemu-x86_64`.
- Integration: grant, invoke, revoke on H-002 with audit records visible to `os inspect`.
- Review: CAP reviewer confirms denial allocates no handle.

#### Evidence
- none

### SEM-027 · Define the reference Mail Semantic Interface
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: SEM-006, SEM-029
- Baseline: §42

Catalogue IDL for Mail.search, compose and reply with a fixture Component. APP parks mail sync for 1.0; this is the Interface hook, not a mail client.

<!-- covers: INV-0790 -->

#### Out of scope
Mail sync and client (APP). Cross-app demo (SEM-016).

#### Acceptance criteria
- [ ] Mail.search, compose and reply are generated from IDL and registered by the fixture on `qemu-x86_64`.
- [ ] compose and reply take Draft and Message Capabilities, not paths or widget handles.
- [ ] The fixture is not a shipped mail product; APP first-party set does not list it as a native mail client.

#### Verification
- Unit: `sem:tests/mail/search_compose_reply_*` on `qemu-x86_64`.
- Review: APP reviewer confirms this is an Interface hook, not a mail client.

#### Evidence
- none

### SEM-028 · Define a transferable Project Capability type
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: CAP-003, CAP-004, SEM-004
- Baseline: §7, §44
- Invariants: I-028

Project is a Capability type transferable among Workspace, Editor and assistant Components with attenuation and revocation. Required before Workspace.search and the action-graph demo. Deriving extra rights fails with `Error::Rights` and allocates no handle.

<!-- covers: INV-0823 -->

#### Out of scope
Workspace.search (SEM-035). Editor.openProject (SEM-022). Environment attach (ENV-024).

#### Acceptance criteria
- [ ] A holder of `Capability<Project, ReadWrite>` derives `Capability<Project, Read>`; deriving Admin returns `Error::Rights` and allocates no handle.
- [ ] Transfer of a Project Capability from Workspace to Editor leaves exactly one owner; the sender cannot use it afterward.
- [ ] Revoking the parent invalidates derived Project Capabilities within one Operation.

#### Verification
- Unit: `sem:tests/project/derive_transfer_revoke_*` on `qemu-x86_64`.
- Integration: Workspace to Editor transfer on H-002 with `os inspect` of holders.
- Review: CAP reviewer confirms attenuation and revoke match CAP primitives.

#### Evidence
- none

### SEM-029 · Implement the system Semantic Interface registry service
- Type: build
- Milestone: V2
- Status: todo
- Size: L
- Owner: none
- Depends on: SEM-007, SEM-001, PKG-057, SVC-009
- Baseline: §32, §42, §57
- Risks: R-043
- Invariants: I-037, I-051

Implement the system discovery service so one Component finds another application's exposed Interfaces. Extends SEM-007 with live registration, crash rebind and manifest ingest. This is the Semantic interface registry task AI work depends on. IPC service discovery for compositor rebind stays in IPC and SVC.

<!-- covers: INV-0798 -->

#### Out of scope
IPC service naming (IPC-023). Supervisor rebind (SVC-009). Cross-app grants (SEM-026). AI broker (SEM-010).

#### Acceptance criteria
- [ ] Live registration: starting an application adds its manifest-declared Semantic interfaces to lookup; exit removes them within one Operation.
- [ ] Killing and restarting the registry service rebinds clients by Interface identity with no caller restart, matching SVC-009.
- [ ] Manifest ingest: a Package whose Interfaces section names a Semantic interface is listed after install without a hand-written registry entry.
- [ ] Lookup without a grant returns `Error::Rights` and allocates no handle.
- [ ] SEM-001 passes for broker crates that depend on this registry crate and fails if that dependency is removed.

#### Verification
- Unit: `sem:tests/registry/live_rebind_manifest_*` on `qemu-x86_64`.
- Integration: kill and restart of the registry service on H-002 with clients rebound.
- Review: SVC reviewer confirms rebind matches the supervisor contract.

#### Evidence
- none

### SEM-030 · Hold only delegated Capabilities in Automation rules
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: SEM-031, CAP-017, CAP-003
- Baseline: §9.1, §21, §45
- Threats: T-018
- Invariants: I-021, I-031

Rules hold delegated typed Capabilities from events rather than ambient authority. Background rules require `Capability<BackgroundExecution>` from CAP; a missing grant returns `Error::Rights` and allocates no handle. This is the authority half of the rule engine.

<!-- covers: INV-0843, INV-0847 -->

#### Out of scope
Background-execution type (CAP-017). Rule evaluation (SEM-031). Event model (SEM-011).

#### Acceptance criteria
- [ ] A rule whose action uses a Capability not present on the triggering event returns `Error::Rights` and allocates no handle.
- [ ] A rule that runs after the owning session ends without `Capability<BackgroundExecution>` returns `Error::Rights` and does not start a Task.
- [ ] The three V2 example rules hold only delegated Capabilities; a planted home-directory grant in a rule fixture fails CI.

#### Verification
- Unit: `sem:tests/rules/delegation_background_*` on `qemu-x86_64`.
- Integration: background-deny and event-delegation cases on H-002.
- Review: CAP reviewer confirms the BackgroundExecution check matches CAP-017.

#### Evidence
- none

### SEM-031 · Implement the Automation rule trigger engine
- Type: build
- Milestone: V2
- Status: todo
- Size: L
- Owner: none
- Depends on: SEM-018, SEM-011, SEM-013
- Baseline: §45, §62
- Risks: R-043
- Threats: T-018
- Invariants: I-023

Evaluate when-event if-condition action rules over the application event model. This is the V2 user-visible rules engine. Format follows SEM-018; Wasm hosting, if chosen, is WASM-015. Automation lands before the AI broker.

<!-- covers: INV-0840, INV-1229 -->

#### Out of scope
Rule format Decision (SEM-018). Wasm module host (WASM-015). Rule editor UI (APP-059). Capability delegation checks (SEM-030). AI broker (SEM-010).

#### Acceptance criteria
- [ ] A when-event if-condition action rule matching Download.completed and Archive type invokes Extractor.extract through SEM-013.
- [ ] A rule whose condition does not match does not invoke the action.
- [ ] Rules are listed in `os inspect` with trigger, condition and action Interface names.
- [ ] The engine crate has no dependency on an AI broker crate.
- [ ] Evaluation posts no mouse or key events; SEM-002 passes on the engine crate.

#### Verification
- Unit: `sem:tests/rules/engine_match_miss_*` on `qemu-x86_64`.
- Integration: user-visible rule list on H-002 via `os inspect`.
- Review: SEM reviewer confirms no broker dependency.

#### Evidence
- none

### SEM-032 · Prototype local ComputeDevice versus remote model placement
- Type: spike
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: SEM-001, HET-009, HET-015, SEM-007
- Baseline: §37, §44, §57

Prototype a local ComputeDevice or NPU path versus a remote-service path for assistant model execution and record that the OS supplies Capability plumbing, not a model runtime. Must precede SEM-017. Do not bundle weights as a privileged system service.

<!-- covers: INV-0834 -->

#### Out of scope
Model-execution Decision (SEM-017). NPU class definition (HET-020). Assistant host (SEM-012).

#### Acceptance criteria
- [ ] A report at `reports/spikes/SEM-032.md` exercises a local ComputeDevice path and a remote-service path on H-002.
- [ ] The report states that neither path links a model runtime into the kernel or a privileged supervisor.
- [ ] The report names which option is ruled out for 1.0 and what remains user-selectable.

#### Verification
- Report: answers (1) whether a local ComputeDevice path can run a small assistant workload under a Capability, (2) what a remote path requires in grants and identity, (3) why an OS-bundled runtime is rejected, (4) what SEM-017 must record.
- Review: HET reviewer records that SEM-017 cites this report.

#### Evidence
- none

### SEM-033 · Prototype the Postgres-branch test-run AI scenario
- Type: spike
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: SEM-029, SEM-022, SEM-035, ENV-018
- Baseline: §35, §44
- Risks: R-043
- Invariants: I-051

Validate the capability-based AI model on "find the Postgres backup branch and run its tests" before SEM-012 and SEM-010. Uses Workspace.search, a Project Capability, Editor.openProject and Editor.runTests against the ENV php-postgres-redis reference stack. No GUI input.

<!-- covers: INV-0836 -->

#### Out of scope
Assistant host (SEM-012). AI broker (SEM-010). Environment objects (ENV).

#### Acceptance criteria
- [ ] A report at `reports/spikes/SEM-033.md` records a scripted, non-broker walk of Workspace.search to Editor.runTests on H-002 against the reference environment.
- [ ] The walk uses only registry lookup and typed Capabilities; the report lists every grant.
- [ ] The report names gaps the broker must close (logging, mid-run revoke, principal) without implementing the broker.

#### Verification
- Report: answers (1) whether the action graph is expressible with shipped Interfaces, (2) which grants are required, (3) what fails without the registry, (4) what SEM-010 must add.
- Review: SEM reviewer records that SEM-012 and SEM-010 cite this report.

#### Evidence
- none

### SEM-034 · Prototype declarative, script, and Wasm Automation rule formats
- Type: spike
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: SEM-008, SEM-013, WASM-012
- Baseline: §13, §45
- Invariants: I-046

Prototype declarative rules, a scripting language and Wasm automation modules on Terminal.run and Editor.open before SEM-018. V2 exit requires that adr. Wasm is one format option, not the Native ABI. WASM-015 remains the host if that option wins.

<!-- covers: INV-0844 -->

#### Out of scope
Rule format Decision (SEM-018). Production Wasm automation host (WASM-015). Rule engine (SEM-031).

#### Acceptance criteria
- [ ] A report at `reports/spikes/SEM-034.md` runs the same Terminal.run and Editor.open automation in declarative, script and Wasm forms on H-002.
- [ ] Each form holds only delegated Capabilities; the report records how a missing grant fails.
- [ ] The report names which form is ruled out and whether Wasm remains a module format rather than a Native ABI.

#### Verification
- Report: answers (1) authoring cost of each form, (2) how each holds Capabilities, (3) Wasm host coupling, (4) the recommendation SEM-018 must cite.
- Review: WASM reviewer records the Wasm option's host implications.

#### Evidence
- none

### SEM-035 · Define Workspace.search returning Project Capabilities
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: SEM-028, SEM-029, ENV-013
- Baseline: §35, §44

Workspace.search is the first hop of the V2 AI action graph and returns Project Capabilities consumed by Editor.openProject. ENV owns `os env`; SEM owns this Semantic interface.

<!-- covers: INV-0822 -->

#### Out of scope
`os env` objects (ENV). Editor.openProject (SEM-022). Desktop search (APP-027).

#### Acceptance criteria
- [ ] Workspace.search(Query) returns Project Capabilities, not paths, and registers in the system registry on `qemu-x86_64`.
- [ ] A caller without the Workspace grant receives `Error::Rights` and allocates no handle.
- [ ] A returned Project Capability is transferable to Editor.openProject as defined by SEM-028.

#### Verification
- Unit: `sem:tests/workspace/search_project_*` on `qemu-x86_64`.
- Integration: search against the V1 reference environment on H-002.
- Review: ENV reviewer confirms search does not duplicate `os env` objects.

#### Evidence
- none

### SEM-036 · Publish the public-alpha automation and Semantic Interface guide
- Type: docs
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: SEM-031, SEM-010, SEM-005, DOC-014
- Baseline: §42, §44, §45, §63

Author the public-alpha user and SDK pages for Semantic interfaces, Automation rules and the AI broker. DOC publishes the site; SEM authors the normative pages strangers need for V3-G12.

#### Out of scope
Docs site (DOC). Authoring guidelines already shipped (SEM-005). Automation-rule editor UI (APP-059).

#### Acceptance criteria
- [ ] Pages cover lookup, grants, writing a rule, and how the broker shows Capabilities, with no input-synthesis examples.
- [ ] The pages are linked from the V3 SDK guide and the desktop user guide.
- [ ] A documentation review records SEM and DOC sign-off on the pull request.

#### Verification
- Review: SEM and DOC reviewers sign off on the pull request.
- Manual: a reader following only these pages binds Terminal.run and installs one example rule in the V3 image on H-002.

#### Evidence
- none

### SEM-037 · Fuzz the registry and Automation rule engine
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: SEM-029, SEM-031, SEM-026, BLD-035, BLD-042
- Baseline: §42, §45, §51, §63
- Threats: T-018

Extend continuous fuzzing to userspace SEM services: registry lookup, grant checks and rule evaluation have harnesses with no known open crasher older than the window named by BLD-063.

#### Out of scope
Fuzz fleet (BLD-035). Kernel ABI fuzz (BLD-016). Crasher-age gate (BLD-063).

#### Acceptance criteria
- [ ] Harnesses exist for registry lookup, grant denial and rule evaluation and run in the BLD continuous fleet.
- [ ] A malformed lookup, grant blob or rule does not panic the registry or engine Components; failures return typed errors.
- [ ] At V3 gate time there is no known open SEM crasher older than the window named by BLD-063.

#### Verification
- Fuzz: `sem:fuzz/registry_lookup`, `sem:fuzz/grants` and `sem:fuzz/rules` on the BLD fleet.
- Review: BLD reviewer confirms the harnesses are scheduled with the userspace parser set.

#### Evidence
- none

### SEM-038 · Record Automation rule changes as system history events
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: SEM-031, PKG-022, PKG-059, PKG-086
- Baseline: §31, §45

Automation rule creation and changes appear in `os history` and are restorable with configuration. PKG owns history storage; SEM emits the events.

<!-- covers: INV-0845 -->

#### Out of scope
History log (PKG-022). Configuration restore (PKG-086). Rule editor UI (APP-059).

#### Acceptance criteria
- [ ] Creating, updating and deleting a rule each append a typed history event visible to `os history` on H-002.
- [ ] Restoring configuration to a previous history event restores the rule set; a deleted rule does not remain registered.
- [ ] History payloads contain Interface names and grant ids, not event bytes or secrets.

#### Verification
- Integration: `sem:tests/v3/history_rule_events_*` on `qemu-x86_64` and H-002.
- Review: PKG reviewer confirms event types match PKG-022.

#### Evidence
- none

### SEM-039 · Isolate the Semantic Interface registry per user
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: SEM-029, SEM-031, SEC-042, SEC-060, CMP-051
- Baseline: §9.1, §42, §63
- Threats: T-001
- Invariants: I-021

Two users have separate Capability stores; the registry and Automation rules must not leak another user's Interfaces or grants across sessions. SEC owns sessions; SEM scopes registry state.

#### Out of scope
Session objects (SEC-060). Component isolation tests (CMP-051). Greeter chrome (APP-063).

#### Acceptance criteria
- [ ] User A's registry listing on a two-session V3 image does not include User B's Interfaces or grants.
- [ ] A rule belonging to User A does not fire on User B's events.
- [ ] Lookup of User B's Interface from User A's session returns `Error::Rights` or not-found and allocates no handle.

#### Verification
- Integration: two-session leak tests on H-002 and H-004.
- Review: SEC reviewer confirms isolation matches SEC-060.

#### Evidence
- none

### SEM-040 · Build the Semantic Interface catalog conformance suite
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: SEM-022, SEM-008, SEM-027, SEM-035, SEM-024, SEM-026, SEM-023
- Baseline: §42, §65, §66

Build the conformance suite proving catalog methods, Capability passing and grant denial so binaries built against the freeze candidate keep running on later beta builds. Layer 2 versions lock in SEM-042; this suite is the proof. Required by V4-G02 (Layer 2 interface versions for 1.x are locked).

#### Out of scope
Version lock (SEM-042). Layer 2 evolution tests (SEM-041). Layer 1 freeze (ABI).

#### Acceptance criteria
- [ ] Every catalog Interface (Editor, Terminal, Workspace, Mail, Download, Meeting, Extractor, Notes) has a conformance test for methods, Capability passing and grant denial.
- [ ] A binary built against the V4 freeze candidate still passes the suite on a later V4 beta image on H-002.
- [ ] Grant denial cases return `Error::Rights` and allocate no handle.

#### Verification
- Integration: `sem:tests/conformance/catalogue_*` on `qemu-x86_64` and H-002.
- Review: SEM reviewer confirms every catalog Interface is named in the suite.

#### Evidence
- none

### SEM-041 · Run Layer 2 evolution tests on catalog Semantic interfaces
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: SEM-022, SEM-008, IPC-042
- Baseline: §12, §42, §66

Old-client/new-service and new-client/old-service pass for Editor and Terminal catalog Interfaces, retained as permanent CI. IPC owns evolution rules; SEM retains the Editor and Terminal tests V4-G02 requires for this catalog. Required by V4-G02 (Layer 2 interface versions for 1.x are locked).

#### Out of scope
Evolution-rule freeze (IPC-042). Full catalog conformance (SEM-040).

#### Acceptance criteria
- [ ] Editor and Terminal old-client against new-service and new-client against old-service pass on `qemu-x86_64`.
- [ ] The pairs are permanent CI tests on `qemu-x86_64`.
- [ ] A breaking required-field add without negotiation fails the suite.

#### Verification
- Integration: `sem:tests/evolution/editor_terminal_*` on `qemu-x86_64`.
- Review: IPC reviewer confirms the tests apply S-014 as frozen by IPC-042.

#### Evidence
- none

### SEM-042 · Lock Semantic Interface catalog versions for 1.x
- Type: build
- Milestone: V4
- Status: todo
- Size: S
- Owner: none
- Depends on: SEM-040, SEM-041
- Baseline: §42, §66
- Freezes: S-023

Enumerate and lock Layer 2 interface versions served for 1.x for Editor, Terminal, Workspace, Mail, Download, Meeting, Extractor and Notes. This is SEM's half of V4-G02; it does not freeze Layer 1.

#### Out of scope
Layer 1 freeze (ABI). Conformance suite (SEM-040). 1.x catalog document (SEM-043).

#### Acceptance criteria
- [ ] A locked version list names Editor, Terminal, Workspace, Mail, Download, Meeting, Extractor and Notes with the versions served for 1.x.
- [ ] Adding a new required method to a locked Interface without a new version fails CI.
- [ ] The lock file is the input SEM-043 publishes.

#### Verification
- Unit: `sem:tests/lock/catalogue_versions_*` on `qemu-x86_64`.
- Review: SEM reviewer confirms the list matches the conformance suite.

#### Evidence
- none

### SEM-043 · Publish the 1.x supported Semantic Interface catalog
- Type: docs
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: SEM-042, DOC-028, DOC-040
- Baseline: §42, §66, §70

Publish Layer 2 supported Semantic interface versions in the 1.0 compatibility document with the two-minor-release deprecation policy. SEM authors the Semantic interface pages; DOC publishes.

#### Out of scope
Docs snapshot machinery (DOC-040). Compatibility guide chassis (DOC-028). Version lock (SEM-042).

#### Acceptance criteria
- [ ] The 1.0 compatibility document lists every Interface locked by SEM-042 and the deprecation overlap policy.
- [ ] The pages contain no unmeasured performance claim.
- [ ] Review records SEM and DOC sign-off on the pull request.

#### Verification
- Review: SEM and DOC reviewers sign off on the pull request.
- Manual: the published 1.0 compatibility document names the locked catalog.

#### Evidence
- none

### SEM-044 · Demonstrate 1.0 AI-assisted automation with visible Capability grants
- Type: build
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: SEM-009, SEM-025, PKG-059, SDK-069, SEM-029
- Baseline: §31, §44, §45, §70
- Invariants: I-023

A working-day demo on a Tier 1 machine of the release candidate: AI-assisted semantic automation with visible Capability grants, then rollback of a Package change from `os history`. Reuses the V2 action graph; this task is the 1.0 hold.

#### Out of scope
History CLI (PKG-059). Restore (SDK-069). Catalog document (SEM-043).

#### Acceptance criteria
- [ ] On H-002 the release-candidate image repeats Workspace.search to Editor.runTests with grants visible in `os inspect` and no mouse or key events.
- [ ] After a Package update, `os history` shows the change and `os restore` rolls it back without dropping the user's rules.
- [ ] The V2 example rules still run on the same image.

#### Verification
- Demo: working-day path on H-002 of the release candidate.
- Integration: V2 example rules and action-graph tests green on the 1.0 image.

#### Evidence
- none
