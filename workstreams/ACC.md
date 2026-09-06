# ACC · Accessibility
- Prefix: ACC
- Lead: none
- Baseline: §9, §9.1, §41, §42, §49, §57, §60, §64, §65, §66

<!-- roadmap:generated:begin summary -->
Tasks: 37 live, 0 done, 0 in-progress, 37 todo, 0 dropped. Ready: 0. Blocked: 37. Weighted: 0%.
<!-- roadmap:generated:end -->

## Scope

ACC owns the accessibility tree that every native widget emits, the Decision that fixes its schema and its relation to semantic actions, and the assistive-technology clients that consume it. The workstream covers the tree crate and incremental updates, the AT protocol Decision, `Capability<AccessibilityTree>` issuance with secure-field redaction, focus-order and keyboard-only contracts, the native screen reader and speech Component, magnifier and visual preferences, keyboard and pointer accommodations, AT-SPI2 and UI Automation bridges contained inside personalities, the EN 301 549 / WCAG 2.2 AA checklist and EU Accessibility Act evidence, and the task scripts that verify the V2 and V4 gates. Native software never sees AT-SPI, MSAA, UI Automation or a POSIX accessibility bus; those exist only inside the Linux personality and Windows personality.

## Out of scope

Widget emission of role, name and state, toolkit focus traversal, theming tokens and high-contrast widget application (UIP). Compositor scanout, trusted-UI Surfaces and magnification plumbing in the scene graph (GFX). Semantic interface registry, automation rules and the AI broker (SEM). Shell chrome, Settings panels, greeter, lock, consent UI and the four demo applications (APP). HID devices, libinput and braille-display procurement (HW, LAB). Settings object storage (SVC). `os inspect` CLI rendering (SDK). Inspect provider registration (OBS). D-Bus AT-SPI2 bus hosting (LNX). Wine UIA/MSAA enablement (WIN). Permissions UI and the threat-model document (SEC). Installer and recovery chrome (INS). Licence allowlist and CRA steward Decision (GOV). Docs site publishing (DOC). Benchmark methodology (BEN). AudioStream playback (AUD). Text shaping and IME (TXT).

## Tasks

### ACC-001 · Decide the assistive-technology protocol: AT-SPI compatible, native, or both
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: ACC-004, SEC-002
- Baseline: §9, §41, §42, §57, §65
- Decision: D-0018
- Risks: R-021
- Threats: T-001, T-039
- Invariants: I-021, I-048

Decide how assistive-technology clients reach the accessibility tree: a native typed Channel only, an AT-SPI2 export, or both. The schema Decision is separate; this Decision is the transport and client-facing protocol, and it must be accepted before the V1 UI protocol freeze candidate so S-017 is not an incompatible surprise on S-015. An ambient accessibility bus as the native path is rejected by §9; AT-SPI2, if chosen, lives inside the Linux personality.

<!-- covers: GAP-0554, EXTRA-065 -->

#### Out of scope
Tree schema (ACC-002). Capability grant model (ACC-007). AT-SPI2 bus hosting (LNX-096). V3 bridge implementation (ACC-026).

#### Acceptance criteria
- [ ] The Decision file evaluates at least: (A) native typed Channel only; (B) AT-SPI2 export of the native tree; (C) both, with AT-SPI2 confined to the Linux personality.
- [ ] The accepted option names how a native AT client obtains a tree without D-Bus, Wayland or a POSIX bus in native IDL (I-048).
- [ ] Each option cites T-001 and T-039 and records whether an unprivileged Component can observe another application's tree without a Capability.
- [ ] Review sign-off is recorded on the pull request that accepts the Decision.

#### Verification
- Review: ACC and SEC reviewers sign off on the pull request that accepts the Decision.

#### Evidence
- none

### ACC-002 · Decide the accessibility tree model shared by the toolkit and semantic interfaces
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: ACC-004, UIP-007
- Baseline: §41, §42, §60, §65
- Decision: D-0020
- Risks: R-021
- Invariants: I-040

Decide the accessibility tree contract shared by the toolkit and semantic interfaces: roles, states, properties, actions, live regions and text ranges, and whether AccessKit is that schema. Options are adopt AccessKit, a native schema with AccessKit export, or native only. Acceptance is required before the UI protocol L2 freeze candidate and before V0.5 metadata emission; S-017 stays prototyped (I-040 applies to Layer 1; this Decision does not freeze S-017).

<!-- covers: GAP-0261, EXTRA-065 -->

#### Out of scope
AT protocol transport (ACC-001). One-tree-or-two (ACC-008). Tree crate (ACC-005). Widget emission (UIP-001).

#### Acceptance criteria
- [ ] The Decision file evaluates adopt AccessKit, native schema with AccessKit export, and native only, each citing `reports/spikes/ACC-004.md`.
- [ ] The accepted option enumerates roles, states, properties, actions, live regions and text ranges that V0.5 widgets must emit at least role, name and state from.
- [ ] The accepted option is one of the three measured options and names how UIP-001 derives nodes from the declarative tree.
- [ ] Review sign-off is recorded on the pull request that accepts the Decision.

#### Verification
- Review: ACC and UIP reviewers sign off on the pull request that accepts the Decision.

#### Evidence
- none

### ACC-003 · Add os inspect accessibility tree dump and the four-app tree dump test
- Type: build
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: ACC-005, UIP-001, OBS-019, SDK-007, APP-004, APP-006, APP-003, APP-005
- Baseline: §41, §60, §64
- Risks: R-021
- Threats: T-039
- Invariants: I-034

Add an `os inspect` dump of the accessibility tree so V0.5-G10 is a task, not prose: every widget's role, name and state is inspectable, with a CI tree-dump test on Terminal, File Browser, Text Editor and Image Viewer. OBS owns provider registration; SDK owns the `os inspect` command; this task owns the accessibility-tree provider payload and the four-app dump assertions.

<!-- covers: EXTRA-065, INV-0765 -->

#### Out of scope
Widget emission (UIP-001). Inspect CLI rendering (SDK-007). CI job plumbing (BLD-014). Screen reader (ACC-021).

#### Acceptance criteria
- [ ] `os inspect` on a running Terminal, File Browser, Text Editor and Image Viewer prints a tree in which every interactive widget has role, name and state.
- [ ] A widget that ships without role, name or state fails `acc:tests/inspect/tree_dump_*` on `qemu-virtio-gpu`.
- [ ] A non-trusted application cannot inject nodes into the chooser's dump (T-039).
- [ ] The dump names the producing Component; trusted-UI trees name the trusted-UI Component, not the requesting application.

#### Verification
- Unit: `acc:tests/inspect/tree_dump_*` on CI matrix entries `qemu-x86_64` and `qemu-virtio-gpu`.
- Integration: four-app dump on H-003 and H-002, consumed by UIP-026 and BLD-014.
- Review: OBS reviewer confirms the provider uses the typed inspect interface.

#### Evidence
- none

### ACC-004 · Evaluate AccessKit as accessibility tree schema and bridge against a native schema
- Type: spike
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: UIP-024, UIP-007, GOV-016
- Baseline: §41, §42, §65, §67
- Explores: S-017
- Risks: R-021

Prototype deriving role, name and state for the V0.5 toolkit widgets through AccessKit and through a hand-rolled native schema. The report measures schema fit to the widget set, licence against the userspace allowlist, Rust maturity and AT-SPI/UIA bridge coverage so ACC-002 and ACC-001 are not invented on the UI protocol freeze path.

<!-- covers: GAP-0261, GAP-0554 -->

#### Out of scope
Schema Decision (ACC-002). Protocol Decision (ACC-001). Production tree crate (ACC-005). Toolkit architecture (UIP-007).

#### Acceptance criteria
- [ ] A report at `reports/spikes/ACC-004.md` compares AccessKit and a hand-rolled schema on the V0.5 widget set on H-003.
- [ ] Each option derives role, name and state from the declarative tree with no per-widget parallel tree in the prototype.
- [ ] The report records licence, Rust-maturity and AT-SPI/UIA bridge-coverage findings and names which option is ruled out.
- [ ] Native prototype IDL contains no Wayland, X11 or POSIX accessibility-bus types.

#### Verification
- Report: schema fit of AccessKit versus a native schema on V0.5 widgets; licence status against GOV-016; Rust crate maturity; AT-SPI2 and UIA mapping coverage; which option is ruled out and why; whether S-017 can be a projection of the declarative tree.
- Review: ACC reviewer sign-off recorded on the pull request that lands the report.

#### Evidence
- none

### ACC-005 · Build the accessibility tree crate and derive it from the declarative UI tree
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: ACC-002, UIP-020, IPC-012
- Baseline: §41, §42, §60
- Risks: R-021
- Threats: T-039
- Invariants: I-034

Implement the accepted schema as the crate the UIP toolkit emits through, deriving role, name and state automatically from the declarative widget tree so applications need no per-widget parallel tree. V0.5-G10 requires every widget to emit metadata; widget-side wiring stays in UIP-001. Trusted-UI trees are produced by the trusted-UI Component, never by the requesting application (T-039).

<!-- covers: INV-0782, INV-0765, EXTRA-065 -->

#### Out of scope
Widget emission (UIP-001). Incremental diffs, text ranges and live regions (ACC-013). `os inspect` dump (ACC-003). Screen reader (ACC-021).

#### Acceptance criteria
- [ ] A published crate implements the accepted schema and, given a V0.5 declarative widget tree, produces role, name and state for every interactive widget on `qemu-virtio-gpu`.
- [ ] A fixture application that adds a button without a parallel accessibility node still yields role, name and state from the declarative tree.
- [ ] The crate's public API and IDL contain no AT-SPI, MSAA, Wayland or POSIX types.
- [ ] A non-trusted caller cannot insert nodes into another Component's tree (T-039).

#### Verification
- Unit: `acc:tests/tree/derive_role_name_state_*` on `qemu-x86_64` and `qemu-virtio-gpu`.
- Integration: derivation consumed by UIP-001 on H-003.
- Review: UIP reviewer confirms the crate matches the accepted schema.

#### Evidence
- none

### ACC-006 · Route accessibility actions from AT clients to widgets through the tree
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: ACC-008, ACC-013, UIP-036
- Baseline: §41, §42
- Invariants: I-021

Route activate, focus, set-value, scroll, expand/collapse and custom actions from AT clients to widgets through the tree, per the one-tree Decision. Protocol emission of those actions is UIP-036; this task is the dispatch into widget mutations without synthesising pointer or key events. Required before any V2 AT client can act.

<!-- covers: INV-0766, INV-0802 -->

#### Out of scope
Protocol emission of actions (UIP-036). Screen reader (ACC-021). Semantic registry (SEM-007). UI test driver (UIP-041).

#### Acceptance criteria
- [ ] Invoking `activate`, `focus`, `set-value`, `scroll` and `expand`/`collapse` on a tree node performs the same mutation as the corresponding user gesture, with no synthetic pointer or key event in the input log.
- [ ] An action that the node does not expose returns `Error::Rights` and mutates nothing.
- [ ] A round-trip test on Terminal, File Browser, Text Editor and Image Viewer on `qemu-virtio-gpu` records action in, widget mutation out.
- [ ] Custom actions declared by the accepted schema round-trip on a fixture widget.

#### Verification
- Unit: `acc:tests/tree/action_dispatch_*` on `qemu-virtio-gpu`.
- Integration: action round-trip on H-003, consumed by ACC-014.
- Fuzz: `acc:fuzz/tree_actions` one hour nightly without panic.

#### Evidence
- none

### ACC-007 · Decide assistive-technology access as Capability<AccessibilityTree> with redaction
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: ACC-001, ACC-002, SEC-002, SEC-007, CAP-007
- Baseline: §9, §9.1, §41, §51
- Decision: D-0017
- Risks: R-042
- Threats: T-001, T-013, T-039, T-046
- Invariants: I-021, I-060

Decide that assistive-technology clients reach the tree only through a consent-scoped `Capability<AccessibilityTree>`, with password and secure fields redacted unless an elevated grant is held. Options are per-app grants, a session-wide grant with secure-field redaction, and an ambient bus (rejected per §9). The Decision includes an AT keylogging and screen-scraping addendum to SEC's threat model and must precede the V2 broker and screen reader.

<!-- covers: GAP-0262 -->

#### Out of scope
Broker implementation (ACC-016). Permissions UI chrome (SEC-045, APP-025). Threat-model document body (SEC-002).

#### Acceptance criteria
- [ ] The Decision file evaluates at least: (A) per-app grants; (B) session-wide grant with secure-field redaction; (C) ambient bus.
- [ ] Option C is rejected with a citation of T-001, T-013, T-039 and I-021.
- [ ] The accepted option states how password and secure nodes are redacted without an elevated grant, and how the user issues and revokes the Capability.
- [ ] An addendum names the AT keylogging and screen-scraping vectors and is recorded against SEC-002.
- [ ] Review sign-off is recorded on the pull request that accepts the Decision.

#### Verification
- Review: ACC and SEC reviewers sign off on the pull request that accepts the Decision.

#### Evidence
- none

### ACC-008 · Decide whether semantic actions and accessibility actions share one tree
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: ACC-002, SEM-007, SEM-005
- Baseline: §41, §42, §65
- Decision: D-0019

Answer Q-035: whether semantic actions and accessibility actions are one tree or two, and how they stay consistent. Options are a single tree with actions typed by the semantic registry, two trees with a consistency contract, and the accessibility tree as a projection of the semantic tree. The Decision precedes action dispatch and the V1 L2 freeze candidates.

<!-- covers: INV-0802, INV-0766 -->

#### Out of scope
Action dispatch (ACC-006). Semantic registry implementation (SEM-007). Protocol emission (UIP-036).

#### Acceptance criteria
- [ ] The Decision file evaluates at least: (A) single tree with actions typed by the semantic registry; (B) two trees with a published consistency contract; (C) accessibility tree as a projection of the semantic tree.
- [ ] The accepted option states how `activate` on a widget and a semantic verb such as `Editor.open` stay consistent, and which tree an AT client walks.
- [ ] Each option cites SEM-007 as input and records what ACC-006 must implement.
- [ ] Review sign-off is recorded on the pull request that accepts the Decision.

#### Verification
- Review: ACC and SEM reviewers sign off on the pull request that accepts the Decision.

#### Evidence
- none

### ACC-009 · Decide the text-to-speech engine for the native screen reader
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: ACC-012, GOV-016, GOV-003
- Baseline: §41, §67
- Decision: D-0021

Decide the speech engine the native screen reader uses. Options are espeak-ng, Piper, vendor voices, and a pluggable Interface with a named default. Scheduled at V1 so the V2 speech Component is not invented on the screen-reader path; the userspace licence allowlist is an input. No latency number is stated; findings cite the spike report.

<!-- covers: GAP-0263 -->

#### Out of scope
Speech Component (ACC-023). Screen reader (ACC-021). AudioStream (AUD). Licence allowlist (GOV-016).

#### Acceptance criteria
- [ ] The Decision file evaluates espeak-ng, Piper, vendor voices, and pluggable-with-default, each citing `reports/spikes/ACC-012.md`.
- [ ] The accepted option names the default engine Package, whether other engines may register, and that the engine runs as a sandboxed Component with no ambient filesystem or network Capability.
- [ ] The accepted option is on the userspace licence allowlist or records a GOV exception.
- [ ] Review sign-off is recorded on the pull request that accepts the Decision.

#### Verification
- Review: ACC and GOV reviewers sign off on the pull request that accepts the Decision.

#### Evidence
- none

### ACC-010 · Publish accessibility guidelines for native application developers
- Type: docs
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: ACC-002, ACC-015, ACC-011, DOC-014, SDK-056
- Baseline: §41, §42, §52

Publish labelling, role, focus, contrast and reduced-motion guidance so third-party native applications can meet the V3 checklist when SDK v1 ships. Content is owned by ACC and published through DOC's pipeline; the SDK guide links here rather than duplicating the contract. Required by the ACC scope: "the EN 301 549 / WCAG 2.2 AA checklist".

#### Out of scope
Docs site generator (DOC-010). Widget lint (ACC-015). V3 checklist (ACC-028). Toolkit emission (UIP).

#### Acceptance criteria
- [ ] A published page names how a native application supplies name, role, state, focus order, contrast and reduced-motion, with examples against the accepted schema.
- [ ] The page lists no performance number; any latency claim cites a B-ID.
- [ ] The V1 SDK guide links to this page as the accessibility chapter.
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: ACC and DOC reviewers sign off on the pull request.

#### Evidence
- none

### ACC-011 · Define and implement the focus order and keyboard traversal contract
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: ACC-005, UIP-039, UIP-001
- Baseline: §41, §42

Define and implement the focus-order and keyboard-traversal contract that the V2 screen reader tracks and that V4 keyboard-only shell verification consumes. Focus order is derived from the tree; focus-visible indication is mandatory; traversal rules are testable. UIP owns widget-level tab order and the focus ring; ACC owns the tree-derived contract AT clients rely on. Required by V4-G10 (Assistive-technology script completes in full): keyboard-only operation of the entire shell.

#### Out of scope
Widget tab order and focus ring (UIP-039). Screen reader (ACC-021). Keyboard-only shell verification (APP-067). Global shortcuts (UIP-037).

#### Acceptance criteria
- [ ] Focus order of every V0.5 widget gallery matches tree order; a unit test fails when they diverge.
- [ ] The focused node is always present in the tree dump and is marked focus-visible except where the accepted schema names pointer-driven focus as exempt.
- [ ] Keyboard traversal reaches every interactive widget in Terminal, File Browser, Text Editor and Image Viewer on `qemu-virtio-gpu`.
- [ ] A widget that cannot be focused from the tree fails `acc:tests/focus/contract_*`.

#### Verification
- Unit: `acc:tests/focus/contract_*` on `qemu-virtio-gpu`.
- Integration: keyboard-only walk of the four demo applications on H-003 and H-002.

#### Evidence
- none

### ACC-012 · Measure espeak-ng, Piper and vendor voices for latency, quality, footprint, licence
- Type: spike
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: CMP-005, GOV-016, AUD-006
- Baseline: §10, §41, §54
- Invariants: I-061

Prototype espeak-ng, Piper and a vendor-voice option as sandboxed Components and publish time-to-first-audio, CPU and memory, voice-quality notes and licence findings. Must precede ACC-009. Numbers live only in the spike report; prose cites I-061 and does not state a target.

<!-- covers: GAP-0263 -->

#### Out of scope
Engine Decision (ACC-009). Speech service (ACC-023). AudioStream object (AUD-006).

#### Acceptance criteria
- [ ] A report at `reports/spikes/ACC-012.md` runs each engine as a Component with no ambient filesystem or network Capability on H-002.
- [ ] The report publishes time-to-first-audio, CPU and memory, a quality rubric, and licence status against GOV-016 for each engine.
- [ ] The report names which option is ruled out and why, without a numeric target in the Description of later tasks.
- [ ] Playback uses AudioStream; native code does not open an ALSA or PipeWire handle.

#### Verification
- Report: time-to-first-audio, CPU, memory, quality and licence per engine; sandbox Capability set; which option is ruled out; whether a pluggable Interface is feasible as a fourth option.
- Review: ACC and GOV reviewers sign off on the pull request that lands the report.

#### Evidence
- none

### ACC-013 · Complete tree derivation: properties, text ranges, live regions, incremental diffs
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: ACC-005, ACC-002, UIP-001
- Baseline: §41, §42

Extend automatic derivation beyond role, name and state to the full accepted schema: properties, text ranges, live regions, and incremental update notifications so AT clients are not re-sent whole trees. Prerequisite for the V2 screen reader reading the Text Editor.

<!-- covers: INV-0782 -->

#### Out of scope
Role/name/state crate (ACC-005). Action dispatch (ACC-006). Screen reader (ACC-021).

#### Acceptance criteria
- [ ] Text Editor caret and selection appear as text ranges on the tree; a unit test types into the editor and asserts range updates without a full-tree replace.
- [ ] A live-region fixture announces a change as an incremental event, not a whole-tree snapshot.
- [ ] Properties required by the accepted schema are present on V0.5 widgets; a missing property fails `acc:tests/tree/schema_full_*`.
- [ ] An AT client subscribed to diffs receives only changed nodes for a single-widget mutation.

#### Verification
- Unit: `acc:tests/tree/incremental_*` and `acc:tests/tree/text_ranges_*` on `qemu-virtio-gpu`.
- Integration: Text Editor range updates on H-003.
- Fuzz: `acc:fuzz/tree_diffs` one hour nightly without panic.

#### Evidence
- none

### ACC-014 · Build a scripted test driver that operates applications through the tree
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: ACC-006, ACC-013, BLD-027
- Baseline: §41, §42

Build a scripted driver that navigates and activates applications through the accessibility tree rather than synthetic input. The V2 task-completion script and the V4 fifty-task script consume it; APP acceptance scenarios can exercise accessibility without a second harness. UIP-041 is the toolkit and CI sibling; this driver is the AT-oriented face.

<!-- covers: GAP-0118 -->

#### Out of scope
UIP toolkit driver (UIP-041). Semantic GUI harness plumbing (BLD-027). Pixel goldens (GFX-025). Task-completion scripts (ACC-017).

#### Acceptance criteria
- [ ] The driver activates, focuses, sets values and scrolls toolkit widgets through tree actions on `qemu-virtio-gpu` with no synthetic pointer path.
- [ ] A widget missing an `activate` action fails the driver run rather than falling back to coordinates.
- [ ] Each of the four demo applications has at least one driver-scripted scenario in CI.
- [ ] The driver consumes the tree crate over Channel; it does not import AT-SPI or UIA types.

#### Verification
- Unit: `acc:tests/driver/tree_driver_*` on `qemu-virtio-gpu`.
- Integration: four-app driver scenarios on H-003.

#### Evidence
- none

### ACC-015 · Add CI lint failing on unlabelled or roleless interactive widgets
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: ACC-005, UIP-001, BLD-011
- Baseline: §41
- Invariants: I-034

Turn the standing rule that every UI element carries accessibility metadata into an enforced gate at SDK v1: toolkit and first-party applications fail CI on missing role, name or state.

<!-- covers: INV-0765 -->

#### Out of scope
Tree dump test (ACC-003). Developer guidelines prose (ACC-010). CI platform (BLD).

#### Acceptance criteria
- [ ] A first-party or toolkit change that lands an interactive widget without role, name or state fails `acc:lint/widget_label_*` in pre-merge CI.
- [ ] The four demo applications pass the lint on `qemu-virtio-gpu`.
- [ ] The lint runs on toolkit widgets and first-party APP crates; a documented allowlist exists only for non-interactive decoration.

#### Verification
- Unit: `acc:lint/widget_label_*` on `qemu-x86_64`.
- Review: BLD reviewer confirms the lint is in the pre-merge gate.

#### Evidence
- none

### ACC-016 · Implement the Capability<AccessibilityTree> broker with secure-field redaction
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: ACC-007, ACC-013, CAP-037, SEC-045, SEC-044, APP-025
- Baseline: §9, §9.1, §41
- Risks: R-042
- Threats: T-001, T-012, T-013, T-039
- Invariants: I-021, I-060

Implement the V1 access Decision: grant issuance via consent, per-app scoping, revocation through SEC's Permissions UI v1, and tree filtering that redacts secure nodes for an AT without an elevated grant. Tests prove a non-elevated AT cannot read a password field. The broker is a supervised Component; it is not an ambient bus.

<!-- covers: GAP-0262 -->

#### Out of scope
Access Decision (ACC-007). Permissions UI chrome (SEC-045). Consent prompt Surface (APP-025, GFX-040). Screen reader (ACC-021).

#### Acceptance criteria
- [ ] An AT Component without `Capability<AccessibilityTree>` that requests a tree receives `Error::Rights` and allocates no handle.
- [ ] A non-elevated grant's tree omits password and secure-field values; an elevated grant includes them; both cases are permanent tests.
- [ ] Revoking the grant through Permissions UI v1 makes the next tree Operation fail with `Error::Rights` and allocates no handle.
- [ ] A per-app grant does not expose another application's tree (T-001, T-039).
- [ ] Consent is issued through the trusted consent UI; the broker refuses a client-supplied tree and a non-trusted application cannot overlay the prompt (T-012, T-039).

#### Verification
- Unit: `acc:tests/broker/grant_redaction_*` on `qemu-x86_64` and `qemu-virtio-gpu`.
- Integration: consent, redaction and revoke on H-002 with SEC-045.
- Fuzz: `acc:fuzz/broker_grants` one hour nightly without panic.

#### Evidence
- none

### ACC-017 · Write the assistive-technology Task script for the four apps and shell settings
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: ACC-014, ACC-021, ACC-023, APP-040, APP-004, APP-006, APP-003, APP-005
- Baseline: §41, §42, §62
- Risks: R-042

Write the assistive-technology task script that verifies V2-G13: the screen reader reads and activates every widget in the four native applications and shell Settings, and the script completes in full. Uses the tree test driver plus speech capture. Seeds the V4 fifty-task script.

<!-- covers: GAP-0263 -->

#### Out of scope
Fifty-task extension (ACC-031). Screen reader implementation (ACC-021). Settings chrome (APP-040).

#### Acceptance criteria
- [ ] A committed script lists tasks covering every interactive widget in Terminal, File Browser, Text Editor, Image Viewer and the Settings chassis.
- [ ] Running the script with the screen reader on H-002 completes every listed task; speech capture matches the expected announcement for each.
- [ ] A failing widget names the task id in the log and fails the job.
- [ ] The script drives the tree test driver; it does not synthesise pointer events.

#### Verification
- Integration: script on H-002, H-004 and H-005.
- Demo: V2-D06 screen reader navigating the shell and Text Editor on H-002.

#### Evidence
- none

### ACC-018 · Benchmark focus-change to speech-onset and tree update latency
- Type: benchmark
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: ACC-021, ACC-023, ACC-013, BEN-038
- Baseline: §41, §54
- Benchmarks: B-045
- Invariants: I-050, I-061

Build the harness that measures focus-change to AT notification and to first audio, published beside Orca on Linux on the same hardware. Any responsiveness statement about the screen reader lives in a register report, not in prose (I-061). Results are published under B-045's p50/p99 method as desktop-essentials items with a V2 publish target and no absolute claimed.

#### Out of scope
Screen reader behaviour (ACC-021). Desktop-essentials suite ownership (BEN-038). Input-to-photon (B-020, GFX).

#### Acceptance criteria
- [ ] Harness `bench:desktop-essentials` records focus-change to AT notification and focus-change to first audio on H-002 using B-045's trial count and statistics.
- [ ] A V2 report exists under `reports/benchmarks/B-045/` for H-002 that includes the Orca-on-Linux baseline on the same machine.
- [ ] Tree-update notification latency is published in the same report.
- [ ] No superiority claim appears without the published table (I-050, I-061).

#### Verification
- Bench: B-045 on H-002; target per register (V2 publish).
- Review: BEN lead confirms the harness matches registers/benchmarks.md.

#### Evidence
- none

### ACC-019 · Add sticky, slow and bounce keys, mouse keys and dwell click to input routing
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: UIP-012, HW-011, SVC-013, APP-040
- Baseline: §41, §49
- Risks: R-042

Keyboard and pointer accommodations implemented in UIP's input routing layer under ACC settings so they apply to native, Linux personality and Windows personality windows alike. Required by the V3 checklist and the V4 script. Settings storage is SVC; the Settings panel chrome is APP.

<!-- covers: GAP-0264 -->

#### Out of scope
HID minting (HW-011). Focus routing (UIP-012). Settings chrome (APP-040). Switch access (ACC-037).

#### Acceptance criteria
- [ ] Sticky keys, slow keys and bounce keys are typed settings; enabling each changes key delivery to focused Surfaces of native, Linux personality and Windows personality windows on H-002.
- [ ] Mouse keys move the pointer from the keyboard; dwell click issues a primary activate after the configured dwell without a physical click.
- [ ] An unfocused Surface observes none of the transformed events; the denial test is retained permanently.
- [ ] Settings persist across restart via the settings store and appear in the Settings accessibility panel.

#### Verification
- Unit: `acc:tests/input/sticky_slow_bounce_*` and `acc:tests/input/mouse_keys_dwell_*` on `qemu-virtio-gpu`.
- Integration: accommodations on H-002, H-004 and H-005 against a native window and a Linux personality window.

#### Evidence
- none

### ACC-020 · Build the screen magnifier with focus and caret tracking
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: ACC-013, ACC-011, GFX-006, GFX-010, APP-040, SVC-013
- Baseline: §40, §41, §62
- Risks: R-042

Compositor-level magnification driven by focus and caret positions from the tree. V4-G10 lists the magnifier; the V3 checklist requires it. GFX owns composition; ACC owns tracking, zoom settings and the follow-focus/follow-caret policy.

<!-- covers: GAP-0264 -->

#### Out of scope
Compositor GPU composition (GFX-006). High-contrast tokens (UIP-053). Settings chrome (APP-040).

#### Acceptance criteria
- [ ] A magnification factor from the accessibility settings zooms the focused region's scanout on H-002 without granting applications a global overlay Capability.
- [ ] Follow-focus and follow-caret modes keep the focused widget or caret inside the magnified view, driven by tree focus and text-range events.
- [ ] Toggling the magnifier at runtime updates the output without Component restart of open applications.
- [ ] `os inspect` names the magnifier Component and the current zoom factor.

#### Verification
- Unit: `acc:tests/magnifier/follow_focus_*` on `qemu-virtio-gpu`.
- Integration: magnifier follow-focus and follow-caret on H-002.
- Demo: included in V2-D06 as an optional pass on H-002.

#### Evidence
- none

### ACC-021 · Build the native screen reader: focus tracking, navigation modes, review cursor
- Type: build
- Milestone: V2
- Status: todo
- Size: L
- Owner: none
- Depends on: ACC-016, ACC-023, ACC-006, ACC-013, ACC-011, ACC-014
- Baseline: §41, §42, §62
- Risks: R-042
- Threats: T-039
- Invariants: I-021

First AT client consuming the tree through the Capability broker: announces focus and state changes, live regions, text ranges in the editor, browse and focus modes, and a review cursor. Verified by V2-G13: it reads and activates every widget in the four native applications and shell Settings.

<!-- covers: GAP-0263 -->

#### Out of scope
Speech engine (ACC-023). Capability broker (ACC-016). Semantic verbs (ACC-022). Braille (ACC-027). Task script (ACC-017).

#### Acceptance criteria
- [ ] Holding `Capability<AccessibilityTree>`, the reader announces focus, name, role and state changes for every interactive widget in Terminal, File Browser, Text Editor, Image Viewer and Settings on H-002.
- [ ] Browse and focus modes and a review cursor navigate the tree; activate through the tree performs the widget action with no synthetic pointer event.
- [ ] Live regions and Text Editor text ranges are announced from incremental tree events.
- [ ] Without the Capability the reader receives `Error::Rights` and produces no speech (I-021).
- [ ] The reader does not inject nodes into trusted-UI trees (T-039).
- [ ] Native crates contain no AT-SPI, MSAA or POSIX accessibility-bus types.

#### Verification
- Unit: `acc:tests/reader/focus_modes_*` on `qemu-virtio-gpu`.
- Integration: four-app plus Settings read-and-activate on H-002, H-004 and H-005.
- Demo: V2-D06 on H-002.

#### Evidence
- none

### ACC-022 · Make the screen reader an accessibility client of semantic interfaces
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: ACC-021, ACC-008, SEM-007, SEM-008, SEM-002
- Baseline: §41, §42
- Threats: T-046

Use SEM's V1 registry so the screen reader can list and invoke semantic actions such as `Editor.open` and `Terminal.run` without simulating input, proving §42's accessibility consumer. Depends on semantic interface v0; this is not an AI broker.

<!-- covers: INV-0792 -->

#### Out of scope
Registry service (SEM-007). AI broker (SEM-010). Action dispatch to widgets (ACC-006). Input-synthesis lint (SEM-002).

#### Acceptance criteria
- [ ] The screen reader lists semantic actions exposed by Terminal and Text Editor from the session registry and invokes `Terminal.run` and `Editor.open` without a synthetic pointer or key event.
- [ ] A missing grant returns `Error::Rights` and allocates no handle.
- [ ] The invocation is logged as a semantic call, not as GUI scraping (SEM-002).
- [ ] A permanent CI test on `qemu-virtio-gpu` completes both invocations through the reader.

#### Verification
- Unit: `acc:tests/reader/semantic_client_*` on `qemu-virtio-gpu`.
- Integration: Terminal.run and Editor.open through the reader on H-002.

#### Evidence
- none

### ACC-023 · Build the speech Component with voice selection, queueing and interruption
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: ACC-009, ACC-012, AUD-006, AUD-010, SVC-015, SVC-013, IPC-012
- Baseline: §10, §32, §41
- Risks: R-042

Text-to-speech service on the decided engine, exposed as a typed Interface. Utterance queue, priority interruption, rate, pitch and voice settings. Required by the V2 screen-reader prototype gate. Playback is AudioStream; the engine is a sandboxed Component.

<!-- covers: GAP-0263 -->

#### Out of scope
Engine Decision (ACC-009). Screen reader (ACC-021). AudioStream object (AUD-006). Mixer UI (APP).

#### Acceptance criteria
- [ ] A native Component speaks a fixture utterance through the Interface on H-002 using AudioStream, with no ALSA or PipeWire handle in native code.
- [ ] Voice, rate and pitch settings are typed objects in the settings store; changing them affects the next utterance without Component restart.
- [ ] A higher-priority utterance interrupts the queue; `os inspect` shows queue depth and the speaking Component.
- [ ] The engine Component holds no ambient filesystem or network Capability; a denial test is retained.

#### Verification
- Unit: `acc:tests/speech/queue_interrupt_*` on `qemu-x86_64`.
- Integration: utterance playback on H-002.
- Bench: consumed by ACC-018 (B-045) for first-audio.

#### Evidence
- none

### ACC-024 · Prototype consuming AT-SPI2 and Wine UIA/MSAA trees into the native tree
- Type: spike
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: ACC-001, ACC-005, ACC-016, LNX-043, LNX-006, WIN-015
- Baseline: §3, §9, §49
- Threats: T-011, T-039
- Invariants: I-006, I-007

Feasibility on one GTK application and one Wine application: node mapping fidelity, event latency, D-Bus containment inside the Linux personality, and the security boundary between compatibility trees and the Capability broker. Precedes the V3 bridge builds. Native software still does not see D-Bus, AT-SPI or UIA.

<!-- covers: GAP-0265, INV-0929 -->

#### Out of scope
Production AT-SPI2 bridge (ACC-026). Production UIA/MSAA bridge (ACC-030). Bus hosting (LNX-096). Wine core (WIN).

#### Acceptance criteria
- [ ] A report at `reports/spikes/ACC-024.md` maps one GTK application's AT-SPI2 tree and one Wine application's UIA or MSAA tree into the native schema on H-002.
- [ ] D-Bus and AT-SPI types appear only inside the Linux personality crate; UIA/MSAA types appear only inside the Windows personality crate.
- [ ] The report records node-mapping fidelity, event-latency method (no target claimed), and whether a compat tree can bypass the Capability broker (T-039).
- [ ] The report names which mapping losses would fail V3 read-and-activate.

#### Verification
- Report: mapping fidelity per role; event-latency method; D-Bus containment; broker bypass or not; which losses are acceptable for V3; whether AT-SPI2 export of the native tree is still warranted under the protocol Decision.
- Review: ACC, LNX and WIN reviewers sign off on the pull request that lands the report.

#### Evidence
- none

### ACC-025 · Enforce high contrast, large text, reduced motion, colour filters and cursor size
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: SVC-013, SVC-028, UIP-038, UIP-044, UIP-027, APP-040
- Baseline: §41, §62
- Risks: R-042

Accessibility settings stored via SVC, surfaced in the V2 Settings application, and consumed as toolkit theme tokens so every native application complies without per-app work. Prerequisite for the V3 checklist and the V4 high-contrast-across-shell-and-apps gate. UIP-053 applies the token set to widgets at V3.

<!-- covers: GAP-0264 -->

#### Out of scope
High-contrast widget tokens (UIP-053). Appearance model (UIP-044). Settings chrome (APP-040). Magnifier (ACC-020).

#### Acceptance criteria
- [ ] Typed settings exist for high contrast, large text, reduced motion, colour filters and cursor size, readable by a native Component through the settings client.
- [ ] Changing large text or cursor size at runtime updates open native windows without Component restart.
- [ ] Reduced-motion is the setting UIP-027 reads; there is not a second store.
- [ ] The Settings accessibility panel toggles each setting on H-002.

#### Verification
- Unit: `acc:tests/prefs/visual_settings_*` on `qemu-virtio-gpu`.
- Integration: runtime toggle on H-002, H-004 and H-005.
- Review: UIP reviewer confirms token inputs match these settings.

#### Evidence
- none

### ACC-026 · Bridge AT-SPI2 trees of Linux-Personality applications into the native tree
- Type: build
- Milestone: V3
- Status: todo
- Size: L
- Owner: none
- Depends on: ACC-024, ACC-001, ACC-016, ACC-021, LNX-096, LNX-043, LNX-084
- Baseline: §3, §47, §49
- Corpora: C-004, C-005
- Threats: T-011, T-039
- Invariants: I-006, I-048

Screen reader reads and activates Linux personality GUI applications through an AT-SPI2 consumer contained in the Linux personality. D-Bus stays inside LNX's bridge. If ACC-001 chose both, this task also exports the native tree as AT-SPI2 inside the personality. §49 requires compatibility applications to integrate with accessibility.

<!-- covers: GAP-0265, INV-0929 -->

#### Out of scope
Bus hosting (LNX-096). UIA/MSAA (ACC-030). Corpus scenarios (LNX-100). Native tree schema (ACC-002).

#### Acceptance criteria
- [ ] The native screen reader reads and activates a GTK and a Qt application from C-004 through the bridge on H-002, with D-Bus confined to the Linux personality crate.
- [ ] Native IDL and SDK crates contain no AT-SPI or D-Bus types (I-006, I-048).
- [ ] A Linux personality tree reaches the broker as `Capability<AccessibilityTree>`; a native AT without that Capability receives `Error::Rights` (T-039).
- [ ] If the protocol Decision selected both, a fixture AT-SPI2 client inside the personality reads the native four-app tree; if not, the export path is absent and documented.
- [ ] Mapping losses named by the spike as V3-blocking are closed.

#### Verification
- Integration: GTK and Qt read-and-activate on H-002.
- Compat: C-004 accessibility integration scoring on H-002; C-005 entries inherit the same probe.
- Review: LNX reviewer confirms D-Bus does not leak into native crates.

#### Evidence
- none

### ACC-027 · Add braille display output with BRLTTY-equivalent drivers to the screen reader
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: ACC-021, LAB-020, HW-011, HW-037
- Baseline: §33, §41
- Risks: R-042

Braille mode of the screen reader over USB and Bluetooth braille displays. V2-G13 needs speech only; braille lands at V3 so V4-G10 is not the first braille milestone and so LAB can attach a display.

<!-- covers: GAP-0263 -->

#### Out of scope
Display procurement (LAB-020). Bluetooth host (HW-035). Speech (ACC-023).

#### Acceptance criteria
- [ ] With a USB braille display attached through LAB USB switching, the screen reader writes focus name, role and review-cursor text to the display on a lab machine that has the device.
- [ ] A Bluetooth braille display that HW enumerates as `Capability<InputDevice>` receives the same output after pairing.
- [ ] Native code does not open `/dev/ttyUSB*` or a BlueZ socket; the path is HID or the Bluetooth Device Capability.
- [ ] Disabling braille mode stops output without stopping speech.

#### Verification
- Integration: USB braille on the machine LAB-020 racks; Bluetooth braille on H-004 or H-005 when the peer is present.
- Manual: operator confirms cells match the current review cursor for a Text Editor fixture.
- Review: HW reviewer confirms Device rights used.

#### Evidence
- none

### ACC-028 · Author the EN 301 549 / WCAG 2.2 AA checklist and assess EU Accessibility Act duties
- Type: docs
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: ACC-010, ACC-025, ACC-011, ACC-019
- Baseline: §41, §63
- Risks: R-042

Define the checklist every first-party surface is audited against and, with GOV, map EU Accessibility Act obligations for operating systems to the conformity evidence 1.0 must publish. V3 scopes out conformance claims; this is the internal instrument, not a public claim.

<!-- covers: GAP-0266, GAP-0077 -->

#### Out of scope
Running the audit (ACC-034). Publishing 1.0 evidence (ACC-036). CRA steward versus manufacturer (GOV-070). Widget-level defects (UIP-056).

#### Acceptance criteria
- [ ] A committed checklist maps EN 301 549 / WCAG 2.2 AA criteria to first-party surfaces (shell, installer, greeter, lock, recovery, every native application) with an observable test per row.
- [ ] A companion note lists EU Accessibility Act duties that apply to a consumer operating system and names the 1.0 evidence artefacts, without claiming conformity.
- [ ] Keyboard-only, high contrast, screen reader, magnifier and input accommodations each appear as rows.
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: ACC and GOV reviewers sign off on the pull request.

#### Evidence
- none

### ACC-029 · Make installer, greeter, lock screen and recovery operable by the screen reader
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: ACC-021, ACC-023, INS-027, INS-030, INS-041, INS-034, APP-030, APP-033
- Baseline: §41, §63
- Risks: R-042

GAP-0266 requires the screen reader from the first installer screen. The graphical installer, greeter and recovery environment arrive at V3, so the speech service and reader must be present in the live image and start on a keystroke.

<!-- covers: GAP-0266 -->

#### Out of scope
Installer chrome (INS-027). Recovery environment (INS-041). Greeter and lock chrome (APP). Localisation of installer strings (TXT, INS-030).

#### Acceptance criteria
- [ ] The live installer image starts the speech service and screen reader on a documented keystroke from the first screen, before a user account exists.
- [ ] Greeter, lock screen and recovery environment are read and activated by the screen reader on H-002 and one V3 laptop.
- [ ] A tree dump of each of those surfaces contains role, name and state for every interactive widget.
- [ ] The reader in the live image is the same Component as the desktop reader, not a second engine.

#### Verification
- Integration: live-image first-screen keystroke on H-002; greeter, lock and recovery on H-002 and H-004.
- Manual: operator completes an encrypted install using only the screen reader and keyboard on one Tier 1 machine.
- Review: INS reviewer confirms the live image Package set includes speech and reader.

#### Evidence
- none

### ACC-030 · Bridge UI Automation and MSAA exposed by Wine into the native tree
- Type: build
- Milestone: V3
- Status: todo
- Size: L
- Owner: none
- Depends on: ACC-024, ACC-016, ACC-021, WIN-054, WIN-071, WIN-053
- Baseline: §3, §48, §49
- Corpora: C-008
- Threats: T-011, T-039
- Invariants: I-007

Screen reader reads W2 corpus applications via Wine's UIA/MSAA implementation mapped into the native schema. Coordinated with WIN and gated on V3 W2 integration checks. Native software never sees UIA, MSAA or Win32.

<!-- covers: GAP-0265, INV-0929 -->

#### Out of scope
Wine core (WIN-054). W2 corpus definition (WIN-070). AT-SPI2 (ACC-026). Native schema (ACC-002).

#### Acceptance criteria
- [ ] The native screen reader reads and activates a W2 productivity title that exposes UIA or MSAA, through the bridge on H-002.
- [ ] Native IDL and SDK crates contain no UIA, MSAA or Win32 types (I-007).
- [ ] A Windows personality tree reaches the broker as `Capability<AccessibilityTree>`; a native AT without that Capability receives `Error::Rights` (T-039).
- [ ] Mapping losses named by the spike as V3-blocking are closed.
- [ ] W2 integration scoring records an accessibility probe for titles whose toolkit exposes UIA or MSAA.

#### Verification
- Integration: one W2 UIA title read-and-activate on H-002.
- Compat: C-008 accessibility probe on H-002.
- Review: WIN reviewer confirms UIA/MSAA types stay inside the personality.

#### Evidence
- none

### ACC-031 · Extend the assistive-technology script to 50 tasks across shell and every native app
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: ACC-017, ACC-021, ACC-020, ACC-025, ACC-014, ACC-029, APP-040, APP-043, APP-061, UIP-056, APP-058, UIP-039
- Baseline: §41, §42
- Risks: R-042

Verify V4-G10: the fifty-task script completes in full with the screen reader across shell, Settings and every shipped native application, and also runs under magnifier and high contrast. APP-067 owns making those applications pass; this task owns the script and the AT-client run.

<!-- covers: GAP-0266, GAP-0263 -->

#### Out of scope
Application defect closure (APP-067). Widget gallery (UIP-056). Conformance audit write-up (ACC-034). User study (ACC-032).

#### Acceptance criteria
- [ ] A committed fifty-task script covers shell, Settings and every shipped native application named by APP-061.
- [ ] Running the script with the screen reader on H-002 completes every task; the same script completes under magnifier and under high contrast.
- [ ] Keyboard-only operation of the entire shell is recorded as a distinct pass of the script with speech capture optional.
- [ ] A failing task names the application and widget in the log and fails the job.

#### Verification
- Integration: fifty-task script on H-002 and one V4 laptop.
- Demo: V4-D04 assistive-technology user completes daily tasks, using this script as the lab procedure.

#### Evidence
- none

### ACC-032 · Run and report an assistive-technology user study on daily tasks
- Type: docs
- Milestone: V4
- Status: todo
- Size: S
- Owner: none
- Depends on: ACC-031, ACC-021, ACC-034
- Baseline: §41

V4-D04: an assistive-technology user completes daily tasks across the shell and native applications. A moderated study with blind and low-vision participants produces a report that feeds the RC fix list and the 1.0 evidence dossier. No calendar date is recorded; the report is evidence for the demo.

#### Out of scope
Fifty-task lab script (ACC-031). 1.0 evidence publication (ACC-036). APP defect closure (APP-067).

#### Acceptance criteria
- [ ] A committed report lists tasks drawn from ACC-031, participant class (blind and low-vision), and every defect filed as a task id.
- [ ] Defects classed P0 or P1 are closed or tracked against the RC soak; the report names the remaining ids.
- [ ] The report states no performance number; any latency remark cites B-045.
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: ACC and GOV reviewers sign off on the pull request that lands the report.
- Demo: V4-D04 on a Tier 1 machine using the study procedure.

#### Evidence
- none

### ACC-033 · Add accessibility scoring to L5 and W3 Corpus integration checks
- Type: build
- Milestone: V4
- Status: todo
- Size: S
- Owner: none
- Depends on: ACC-026, ACC-030, WIN-080
- Baseline: §49
- Corpora: C-006, C-009
- Invariants: I-006, I-007

The 1.0 compatibility gate requires accessibility where the toolkit exposes it for every passing Linux and Windows entry. Adds a screen-reader read-and-activate check to LNX and WIN scenario scoring using the bridges.

<!-- covers: INV-0929, GAP-0265 -->

#### Out of scope
Corpus definitions (LNX-107, WIN-079). Bridges (ACC-026, ACC-030). Pass-rate thresholds (registers/corpora.md).

#### Acceptance criteria
- [ ] C-006 and C-009 integration scoring includes a read-and-activate probe when the title's toolkit exposes AT-SPI2, UIA or MSAA.
- [ ] A title whose toolkit exposes no accessibility API is scored not-applicable, not fail, with the reason recorded.
- [ ] The probe uses the native screen reader through the personality bridge; native crates still contain no AT-SPI or UIA types.
- [ ] Reports under `reports/compat/C-006/` and `reports/compat/C-009/` include the accessibility column.

#### Verification
- Compat: C-006 and C-009 accessibility column on H-002.
- Review: LNX and WIN reviewers confirm the probe is in the scenario harness.

#### Evidence
- none

### ACC-034 · Audit shell, installer, greeter, lock screen and every native app against the checklist
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: ACC-028, ACC-031, ACC-029, ACC-025, ACC-019, ACC-011, UIP-053, UIP-056
- Baseline: §41, §42
- Risks: R-042

Run the V3 checklist over every first-party surface, track fixes to closure, and verify keyboard-only operation of the entire shell and high-contrast rendering across shell and applications as V4-G10 requires.

<!-- covers: GAP-0266, GAP-0264 -->

#### Out of scope
Checklist text (ACC-028). Widget-level CI (UIP-056). APP script pass (APP-067). 1.0 publication (ACC-036).

#### Acceptance criteria
- [ ] Every checklist row for shell, installer, greeter, lock, recovery and each shipped native application is pass or is tracked by a non-P0/P1 task id.
- [ ] Keyboard-only operation of the entire shell is recorded as pass on H-002.
- [ ] High-contrast rendering is recorded as pass across shell and native applications on H-002.
- [ ] Open P0 or P1 accessibility defects fail this task.

#### Verification
- Integration: checklist run on H-002 and one V4 laptop.
- Review: ACC reviewer records the signed checklist in the pull request.

#### Evidence
- none

### ACC-035 · Freeze the accessibility tree model
- Type: build
- Milestone: V4
- Status: todo
- Size: S
- Owner: none
- Depends on: ACC-004, ACC-002, ACC-034, UIP-056
- Baseline: §41, §42, §49
- Freezes: S-017

V4 freezes Layer 2 accessibility tree S-017 after the AccessKit spike and accepted tree-schema Decision (§41, §42). Role, name and state emitted by every widget, and the relation to semantic actions, are the frozen contract.

#### Out of scope
The Decision (ACC-002). Screen-reader implementation (ACC-021). UI protocol freeze (UIP-055).

#### Acceptance criteria
- [ ] Surface S-017 is listed as frozen by this task in the surfaces register.
- [ ] The V4 a11y conformance audit runs against the frozen tree schema on H-002.
- [ ] A breaking tree-schema change without an accepted superseding Decision fails CI.

#### Verification
- Integration: `acc:tests/tree/freeze_conformance_*` on `hw-h002`.
- Review: ACC and UIP leads sign off on the pull request that lands the freeze.

#### Evidence
- none

### ACC-036 · Publish the accessibility conformity Evidence and hold V4 gates on the release build
- Type: docs
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: ACC-031, ACC-034, ACC-032, ACC-033, GOV-070, GOV-083
- Baseline: §41, §49, §70

1.0 exit requires the V4 accessibility gates to hold on the release build, and GAP-0077 requires conformity evidence at 1.0: re-run the fifty-task script and keyboard-only verification, then publish the EN 301 549 conformance report and EU Accessibility Act evidence with GOV.

<!-- covers: GAP-0077, GAP-0266 -->

#### Out of scope
CRA steward Decision (GOV-070). Docs site snapshot (DOC-040). Holding shell quality (APP-067).

#### Acceptance criteria
- [ ] The fifty-task script and keyboard-only verification pass on the 1.0 release candidate on every in-scope Tier 1 machine named by the hardware register for 1.0.
- [ ] A published EN 301 549 conformance report and EU Accessibility Act evidence pack exist and are linked from the 1.0 documentation set.
- [ ] The pack claims only what the V4 audit and the re-run support; remaining not-applicable rows are listed.
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: ACC and GOV reviewers sign off on the pull request.
- Integration: fifty-task re-run on the 1.0 release candidate on H-002.

#### Evidence
- none

### ACC-037 · Prototype voice control and switch access over semantic actions
- Type: build
- Milestone: LATER
- Status: todo
- Size: L
- Owner: none
- Depends on: ACC-014, ACC-006, ACC-008, SEM-029, SEM-008
- Baseline: §41, §42

Parking rung for alternative input modalities not in the 1.0 definition. Voice control and switch access build on semantic actions and the tree test driver rather than synthetic input, so they are cheap once SEM and ACC infrastructure exists. Split when leaving LATER: voice control over semantic verbs, and switch access over tree actions. Required by the ACC scope: "the assistive-technology clients that consume it".

#### Out of scope
On-screen keyboard and tablet mode (UIP-058). Speech dictation as a 1.0 non-goal (APP-068). AI broker (SEM-010). Input accommodations already in 1.0 (ACC-019).

#### Acceptance criteria
- [ ] A design note in this task's Evidence names voice control and switch access as post-1.0 work with a split into those two pieces.
- [ ] No 1.0 toolkit or ACC crate exports APIs whose only purpose is voice control or switch access.
- [ ] A prototype, when built, invokes semantic actions and tree actions without synthesising pointer events, and holds only granted Capabilities.

#### Verification
- Review: ACC reviewer confirms 1.0 crates do not export the parked APIs and that the design note names the split, Capability grants, and the SEM-029 dependency.

#### Evidence
- none
