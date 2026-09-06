# UIP · Native UI protocol and toolkit
- Prefix: UIP
- Lead: none
- Baseline: §4, §9, §9.1, §12, §41, §42, §57, §60, §65, §66

<!-- roadmap:generated:begin summary -->
Tasks: 58 live, 0 done, 0 in-progress, 58 todo, 0 dropped. Ready: 1. Blocked: 57. Weighted: 0%.
<!-- roadmap:generated:end -->

## Scope
The native UI protocol between applications and the compositor, and the Layer 4 toolkit that speaks it. This workstream owns window and input protocol messages over Channel, retained declarative widgets, layout, GPU rendering into MemoryObject Buffers, animation, focus and input routing, drag and drop and clipboard as Capabilities, accessibility metadata emission, theming and appearance, high-DPI and adaptive layout, protocol versioning, and the conformance suite that third-party toolkits and personality bridges must pass. Native software never sees Wayland, X11 or POSIX window APIs; those exist only inside personalities.

## Out of scope
Compositor, DRM/KMS, Object<Surface>, Buffer, Display, Frame, trusted-UI overlay protection and lock mode (GFX). HID device enablement, libinput, keyboard layouts and gesture recognition (HW). Shaping, rasterisation, the read-only glyph atlas, IME engines and localisation catalogs (TXT). Accessibility tree schema, screen reader, magnifier and AT bridges (ACC). Semantic interface registry, automation rules and the AI broker (SEM). Shell chrome, launcher, panel, Settings, consent UI and the four demo applications (APP). Wayland, X11, portals and primary-selection emulation (LNX). OLE, Win32 clipboard and DPI awareness (WIN). UserSelected chooser authority (STO). SDK `Context` window packaging (SDK). Tracing format (OBS). Lab photodiode rig (LAB).

## Tasks

### UIP-001 · Emit role, name and state for every widget and expose via os inspect
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: UIP-024, ACC-002, ACC-005, OBS-019
- Baseline: §41, §42, §60
- Risks: R-021
- Threats: T-039

Every toolkit widget emits role, name and state through the accessibility tree crate decided by ACC, inspectable via `os inspect`, from the same declarative element tree the renderer uses. Emission lands at V0.5 so V1 ACC work is not a bolt-on and the four demo applications already dump a tree. Trusted-UI trees are produced by the trusted-UI Component (GFX), never by the requesting application (T-039).

#### Out of scope
Tree schema Decision (ACC-002). `os inspect` dump command (ACC-003). Screen reader (ACC-021).

#### Acceptance criteria
- [ ] Every widget in the V0.5 widget set emits role, name and state derived from the declarative tree with no per-widget hand-written parallel tree.
- [ ] `os inspect` on Terminal, File Browser, Text Editor and Image Viewer prints a tree dump that names every interactive widget.
- [ ] A widget that ships without role, name or state fails `sdk:tests/toolkit/a11y_metadata_*` on `qemu-virtio-gpu`.
- [ ] A non-trusted application cannot inject nodes into the chooser's tree (T-039).

#### Verification
- Unit: `sdk:tests/toolkit/a11y_metadata_*` on CI matrix entries `qemu-x86_64` and `qemu-virtio-gpu`.
- Integration: tree dump of the four demo applications on H-003, consumed by ACC-003.
- Review: ACC reviewer confirms emission matches the accepted tree schema.

#### Evidence
- none

### UIP-002 · Build scripted UI frame-time benchmark and publish p99 against frame interval
- Type: benchmark
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: UIP-010, UIP-024
- Baseline: §41, §54, §60
- Benchmarks: B-018

Build the scripted animating-client harness that records toolkit frame time against the display frame interval so V0.5 can require animation at refresh and later rungs can apply B-018 regression clauses. GFX owns compositor commit-to-scanout (GFX-003); this harness is the toolkit-side animating client B-018 already names. Numbers live only in the register and in `reports/benchmarks/B-018/`. Required by V0.5-G14 (Compositor frame latency published): B-018 needs the animating toolkit client this harness supplies.

#### Out of scope
Compositor commit-to-scanout harness (GFX-003). Input-to-photon (LAB-001, BEN-010). Deadline misses (GFX-060).

#### Acceptance criteria
- [ ] Harness `bench:compositor-frame-latency` drives a native toolkit client for the frame count and refresh named in B-018's method on H-002 and H-003.
- [ ] A V0.5 report exists under `reports/benchmarks/B-018/` for H-002 meeting the register's V0.5 publish target.
- [ ] The same harness is retained so V1 through 1.0 regression clauses of B-018 re-run without a new client.

#### Verification
- Bench: B-018 on H-002 and H-003; target per register.
- Integration: CI job on `qemu-virtio-gpu` records frame timestamps from the toolkit client.

#### Evidence
- none

### UIP-003 · Build clipboard service as Capability with typed lazy MemoryObject transfer
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: UIP-004, UIP-013, MEM-010, CAP-025
- Baseline: §9, §9.1, §41
- Threats: T-001, T-032
- Invariants: I-021

Ship clipboard as `Capability` on S-032: a Component without the clipboard Capability cannot read clipboard contents, writes are allowed, and large payloads move as lazy MemoryObject ownership transfer with typed content negotiation. This is the V0.5 denial gate and the native side of copy-paste with a Wayland application (LNX-006). Paste-gesture gating and `Capability<ClipboardRead>` for managers wait for UIP-028. X11 primary selection never enters this service (T-032, I-048).

<!-- covers: INV-0773, INV-0208 -->

#### Out of scope
Paste-gesture gating and ClipboardRead (UIP-028). Clipboard history UI (APP-024). Wayland `wl_data_device` (LNX-006). OLE (WIN-021).

#### Acceptance criteria
- [ ] A Component launched without the clipboard Capability that calls clipboard read receives `Error::Rights`, allocates no handle, and the denial appears in the capability audit log.
- [ ] A write of a typed payload followed by a read from a holder of the clipboard Capability returns the same type and the payload MemoryObject is transferred lazily, not copied on write.
- [ ] Injecting an X11 primary-selection offer into the native clipboard service is rejected and allocates no native clipboard entry (T-032).
- [ ] Copy-paste between a native demo application and a C-002 Wayland application succeeds in both directions on H-003.

#### Verification
- Unit: `compositor:tests/ui/clipboard_deny_*` on `qemu-x86_64` and `qemu-virtio-gpu`.
- Integration: native-to-Wayland copy-paste scenario on H-003 using C-002.
- Compat: C-002 GUI clipboard scoring on H-002 and H-003.

#### Evidence
- none

### UIP-004 · Decide clipboard authority policy: paste gesture or Capability, no ambient read
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: UIP-019, CAP-007
- Baseline: §9, §9.1, §41
- Decision: D-0324
- Threats: T-001, T-032
- Invariants: I-021

Decide the clipboard authority policy that S-032 will enforce. Silent clipboard reading is a data-exfiltration vector (§9.1, T-001); the baseline lists clipboard Capabilities without specifying the policy. The Decision names how reads are gated, how writes work, where clipboard history lives, and how typed content is negotiated with lazy MemoryObject transfer. Primary selection is not a native concept.

<!-- covers: GAP-0274, GAP-0233, INV-0208 -->

#### Out of scope
Clipboard service implementation (UIP-003). History UI (APP-024). Primary-selection emulation inside the Linux personality (LNX-020).

#### Acceptance criteria
- [ ] The Decision file evaluates at least: (A) reads only on a user paste gesture into the focused Surface, plus `Capability<ClipboardRead>` for clipboard managers, writes always allowed, history a privileged shell feature; (B) every read, including paste, requires an explicit Capability; (C) ambient clipboard read (rejected per §9.1).
- [ ] The accepted option forbids ambient clipboard read (I-021) and keeps X11 primary selection out of S-032 (T-032).
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: UIP and SEC reviewers sign off on the pull request that accepts the Decision.

#### Evidence
- none

### UIP-005 · Decide input routing and focus arbitration model for focused surfaces
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: UIP-019, HW-007
- Baseline: §9, §41, §60
- Decision: D-0326
- Threats: T-012

Decide how pointer, keyboard, touch, pen and gamepad events reach only the focused Surface and how focus is arbitrated among compositor, shell and applications without grabs. Device enablement stays in HW; this Decision is the routing and focus contract the protocol and toolkit implement. Synthetic input into trusted UI and focus stealing are in-scope threats (T-012).

<!-- covers: INV-0770, INV-1180 -->

#### Out of scope
HID service (HW-011). Routing implementation (UIP-012). Global shortcut named-actions Decision (UIP-030).

#### Acceptance criteria
- [ ] The Decision file evaluates at least: (A) compositor-owned focus with delivery only to the focused Surface; (B) shell-owned focus arbitration, compositor as delivery path only; (C) a per-seat input-broker Component.
- [ ] The accepted option forbids an unfocused Surface from observing pointer or key events and forbids global key grabs without a Capability.
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: UIP and GFX reviewers sign off on the pull request that accepts the Decision.

#### Evidence
- none

### UIP-006 · Decide UI protocol model: retained scene tree, client Buffers, or hybrid
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: UIP-017, UIP-019
- Baseline: §12, §41, §65
- Decision: D-0327

Decide the UI protocol transport for S-015: whether an application ships a retained scene or element tree to the compositor, renders into Buffers and ships Surfaces, or a hybrid. V0.5 requires an accepted transport Decision before protocol IDL and the four demo applications start. The spike report is an input; the Decision does not invent a fourth option that the spike did not measure.

<!-- covers: INV-0778, GAP-0516 -->

#### Out of scope
IDL for the chosen model (UIP-013). Toolkit architecture (UIP-007). Wayland hosting (GFX-020).

#### Acceptance criteria
- [ ] The Decision file evaluates retained scene tree, client Buffers, and hybrid, each citing `reports/spikes/UIP-017.md`.
- [ ] The accepted option is one of those three and names how accessibility metadata, compositor rebind and Buffer ownership work under it.
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: UIP and GFX reviewers sign off on the pull request that accepts the Decision.

#### Evidence
- none

### UIP-007 · Decide new Rust toolkit versus adapting an existing toolkit and renderer
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: UIP-018, UIP-006
- Baseline: §41, §50, §66
- Decision: D-0328
- Risks: R-015

Decide whether to build a new Rust toolkit and renderer or adopt and adapt an existing one, using an evaluation matrix that covers accessibility-tree output, declarative-model fit, GPU renderer fit, license and the protocol model already decided. Toolkit effort is a multi-year cost (R-015) and must be scoped before toolkit-core and the four demo applications start. Layer 4 remains the toolkit; the SDK stays Layer 3.

<!-- covers: INV-0779, GAP-0517 -->

#### Out of scope
Toolkit core implementation (UIP-020). Layer 4 semver statement (UIP-034). Text stack library choice (TXT-003).

#### Acceptance criteria
- [ ] The Decision file evaluates at least: (A) a new Rust toolkit and renderer; (B) adopt and adapt Slint; (C) adopt and adapt Xilem; each scored on accessibility-tree output, declarative fit, GPU renderer, license and protocol fit using `reports/spikes/UIP-018.md`.
- [ ] The accepted option names the implementation language as Rust with bindings and does not make Wayland the toolkit API.
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: UIP reviewer sign-off recorded on the pull request that accepts the Decision.

#### Evidence
- none

### UIP-008 · Decide server-side versus client-side decorations for native and compat windows
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: UIP-006, GFX-020
- Baseline: §41, §47, §49
- Decision: D-0329

Decide server-side versus client-side window decorations for native windows and for compatibility windows. The choice fixes window Surface roles in protocol v0 and must land before the Wayland bridge shows compatibility windows at V0.5. Native and compatibility windows may differ; the Decision records both.

<!-- covers: INV-0786 -->

#### Out of scope
Shell Surface roles implementation (UIP-016). Wayland bridge (LNX-006). Shell panel chrome (APP).

#### Acceptance criteria
- [ ] The Decision file evaluates at least: (A) server-side decorations for native and compatibility windows; (B) client-side for native and server-side for compatibility; (C) client-side for all.
- [ ] The accepted option names the protocol roles and which side draws shadow, title and close for each of native and compatibility windows.
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: UIP and GFX reviewers sign off on the pull request that accepts the Decision.

#### Evidence
- none

### UIP-009 · Document input-stack ownership split between HW and UIP
- Type: docs
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: UIP-005, HW-007
- Baseline: §41, §33

Record the input-stack ownership split: device enablement (libinput, evdev, HID, keyboard layouts, gesture recognition) in HW; routing and focus in UIP. The split is written into both workstream scope notes and the UI protocol specification so later tasks do not re-litigate EXTRA-042.

<!-- covers: EXTRA-042 -->

#### Out of scope
HID service (HW-011). Routing service (UIP-012).

#### Acceptance criteria
- [ ] The protocol specification contains a section that names HW as owner of device enablement and UIP as owner of routing and focus, with examples for keyboard, pointer, touch, pen and gamepad.
- [ ] The UIP and HW workstream Out of scope paragraphs each name the other prefix for the split.
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: UIP and HW reviewers sign off on the pull request.

#### Evidence
- none

### UIP-010 · Drive toolkit redraw from compositor frame callbacks at display refresh
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: UIP-021, GFX-024, GFX-015
- Baseline: §41, §40, §60

Drive toolkit redraw from compositor frame callbacks aligned to display refresh so a native application animates at the display refresh rate. This is the V0.5 functional half of INV-0769; the animation API with implicit and explicit transitions is UIP-027 at V1. Frame-time publication is UIP-002 on B-018.

<!-- covers: INV-0769 -->

#### Out of scope
Animation API (UIP-027). Compositor frame-scheduling Decision (GFX-015). B-018 harness (UIP-002).

#### Acceptance criteria
- [ ] A native toolkit client registers for frame callbacks and presents a Buffer on each callback at the display's fixed refresh on H-002 and H-003.
- [ ] Redraw is not driven by an unbounded timer independent of compositor frame callbacks.
- [ ] Missing a frame callback is visible in `os trace` as a toolkit event correlated with GFX frame timing.

#### Verification
- Unit: `sdk:tests/toolkit/frame_callback_*` on `qemu-virtio-gpu`.
- Integration: animating client on H-002 presents at the display's fixed refresh for the duration named in UIP-002.
- Demo: a native application animates at display refresh on H-002 (V0.5-G02).

#### Evidence
- none

### UIP-011 · Instrument input pipeline stages with trace timestamps for latency measurement
- Type: build
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: UIP-012, UIP-010, OBS-013
- Baseline: §24, §41, §54
- Benchmarks: B-020
- Risks: R-022

Timestamp routing and toolkit stages of the input pipeline with the OBS trace schema so the LAB photodiode result for B-020 can be decomposed into device, route, toolkit and present. This task does not publish B-020; LAB and BEN do. Without these spans the V0.5 input-to-photon gate is a single opaque number. Required by V0.5-G15 (Input-to-photon latency published): B-020 attributes the photodiode result to route, toolkit and present through these spans.

#### Out of scope
Photodiode rig (LAB-001). B-020 publication (BEN-010, GFX-004). HID timestamps (HW-011).

#### Acceptance criteria
- [ ] Every pointer and key event that reaches a focused Surface carries trace timestamps for route-accept, toolkit-handle and present-submit, visible in `os trace`.
- [ ] The span names are documented in the OBS event schema and do not collide with GFX commit, queue and scanout events.
- [ ] A B-020 run on H-002 can attribute a photodiode sample to these spans without a second instrumentation pass.

#### Verification
- Unit: `compositor:tests/ui/input_trace_spans_*` on `qemu-virtio-gpu`.
- Integration: `os trace` export of a scripted click on H-002 contains the three spans.
- Bench: B-020 on H-002 consumes the spans; target per register (publish at V0.5).

#### Evidence
- none

### UIP-012 · Route pointer, keyboard and touch input only to focused surfaces
- Type: build
- Milestone: V0.5
- Status: todo
- Size: L
- Owner: none
- Depends on: UIP-005, UIP-013, HW-011, HW-012
- Baseline: §9, §41, §60
- Threats: T-012

Route pointer, keyboard and touch from HW `Capability<InputDevice>` only to the focused Surface over the native UI protocol. V0.5 requires a native application to receive keyboard and mouse through the protocol, and requires a test that no unfocused Surface observes that input. Touch, pen and gamepad event types beyond the V0.5 pointer and key path wait for UIP-040. Grabs and named global shortcuts wait for V1.

<!-- covers: INV-0770, INV-1180 -->

#### Out of scope
HID minting (HW-011). Touch, pen and gamepad protocol types (UIP-040). Global shortcuts (UIP-037). Trusted-UI overlay protection (GFX-040).

#### Acceptance criteria
- [ ] A focused native Surface on H-003 receives keyboard and mouse events delivered over the UI protocol, and the four demo applications handle them in scripted scenarios.
- [ ] An unfocused Surface that subscribes to pointer or key events receives none; the test is retained permanently.
- [ ] A Component cannot register a seat-wide grab; the call returns `Error::Rights` and allocates no handle (T-012).
- [ ] Unplug and replug of a USB keyboard or mouse continues delivery to the focused Surface without restarting the compositor (HW-012).
- [ ] Synthetic injection into a trusted-UI Surface from an unprivileged client is rejected (T-012).

#### Verification
- Unit: `compositor:tests/ui/focus_delivery_*` on `qemu-x86_64` and `qemu-virtio-gpu`.
- Integration: four-app keyboard and mouse scenarios on H-003; hot-plug on H-002.
- Fuzz: `compositor:fuzz/ui_input` one hour nightly without panic.

#### Evidence
- none

### UIP-013 · Define UI protocol v0 IDL: surfaces, frames and Buffers over Channel
- Type: build
- Milestone: V0.5
- Status: todo
- Size: L
- Owner: none
- Depends on: UIP-006, UIP-008, IPC-012, IPC-002, GFX-038
- Baseline: §4, §12, §41, §65
- Risks: R-015

Define UI protocol v0 in the native IDL as a user-space service above Channel and MemoryObject: window Surfaces, frames and Buffers, with roles fixed by the decorations Decision. Native applications open a window through this protocol, not through Wayland, X11 or a Linux syscall. S-015 stays prototyped; this task does not freeze it. IME optional messages are reserved by TXT-010 so v0 does not have to be broken later.

<!-- covers: INV-0762, INV-0108 -->

#### Out of scope
Toolkit (UIP-020). Compositor KMS output (GFX-008). SDK `ctx.ui.window` (SDK-023). IME engines (TXT). Wayland (LNX-006).

#### Acceptance criteria
- [ ] IDL for protocol v0 compiles with the IPC compiler and generates Rust stubs that open a window Surface, attach a Buffer and commit a frame over Channel.
- [ ] A native Component using only those stubs appears as a window under the compositor on H-003 and H-002.
- [ ] The IDL and generated stubs contain no Wayland, X11 or POSIX window types (I-048).
- [ ] Window Surface roles match the decorations Decision for native windows.
- [ ] `os inspect` on a live window prints Surface, Buffer and protocol version.

#### Verification
- Unit: `compositor:tests/ui/protocol_v0_open_*` on `qemu-x86_64` and `qemu-virtio-gpu`.
- Integration: native window open on H-002.
- Review: IPC reviewer confirms the IDL follows §12 evolution rules.

#### Evidence
- none

### UIP-014 · Add CI lint rejecting X11, Wayland and primary-selection concepts in native IDL
- Type: build
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: UIP-013, LNX-020
- Baseline: §41, §57, §68
- Threats: T-032
- Invariants: I-048

Enforce the standing non-goals once: native UI protocol IDL and toolkit API may not reference X11 or Wayland objects, and primary selection is not a native concept. Primary selection is emulated only inside the Linux personality bridge (LNX-020). The lint is the mechanical form of I-048 and GAP-0277 so a historical Unix behavior cannot leak into S-015.

<!-- covers: INV-0759, INV-0760, INV-1122, GAP-0277 -->

#### Out of scope
Wayland bridge (LNX-006). Primary-selection policy inside the bridge (LNX-020). Toolkit widgets (UIP-020).

#### Acceptance criteria
- [ ] CI fails a change that introduces Wayland, X11 or primary-selection types, methods or comments into native UI protocol IDL or the toolkit public API.
- [ ] The four demo applications and protocol v0 stubs pass the lint on `qemu-x86_64`.
- [ ] A fixture that adds a `wl_surface` or `primary_selection` symbol to the native IDL is rejected in CI.

#### Verification
- Unit: `sdk:tests/lint/ui_native_firewall_*` on `qemu-x86_64`.
- Review: UIP reviewer confirms the banned-symbol list covers Wayland, X11 and primary selection.

#### Evidence
- none

### UIP-015 · Version UI protocol under §12 rules and bump v0 to v0.1 with old clients running
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: UIP-013, IPC-002
- Baseline: §12, §41, §66

Version the UI protocol under the §12 interface-evolution rules and bump v0 to v0.1 by adding an optional method, with old clients still running. The regression test is retained permanently. This exercises S-014 before it freezes at V1; it does not freeze S-015.

<!-- covers: INV-0783 -->

#### Out of scope
L2 evolution-rule freeze (IPC-042). Protocol v1 freeze candidate (UIP-035). IDL compiler (IPC-012).

#### Acceptance criteria
- [ ] Protocol v0.1 adds one optional method; a v0 client against a v0.1 compositor still opens a window and receives input on H-003.
- [ ] A v0.1 client against a v0 compositor negotiates down and still opens a window.
- [ ] The old-client and new-service pair is a permanent CI regression test on `qemu-virtio-gpu`.

#### Verification
- Unit: `compositor:tests/ui/protocol_version_bump_*` on `qemu-virtio-gpu`.
- Integration: v0 client and v0.1 compositor on H-003.
- Review: IPC reviewer confirms the bump follows the prototyped §12 rules.

#### Evidence
- none

### UIP-016 · Add privileged shell Surface roles to protocol v0 for panel and launcher
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: UIP-013, UIP-025
- Baseline: §41, §60

Add privileged shell Surface roles to protocol v0 so a session host can present a minimal launcher without a full desktop shell. APP builds the session host and later panel; UIP provides layer-shell-class roles gated by a shell Capability. Ordinary `Capability<UI>` cannot mint these roles. Required by V0.5-D01 (Cold boot to the four native applications): the session host places its launcher Surface through the shell role.

#### Out of scope
Session host and four-app launch (APP-007). V2 panel and launcher (APP-043, APP-013). Trusted-UI Surfaces (GFX-040).

#### Acceptance criteria
- [ ] Protocol v0 IDL includes shell Surface roles for panel and launcher, gated by a shell Capability distinct from `Capability<UI>`.
- [ ] A Component holding only `Capability<UI>` that requests a shell role receives `Error::Rights` and allocates no Surface.
- [ ] APP-007 can place a launcher Surface using the shell Capability on H-003.

#### Verification
- Unit: `compositor:tests/ui/shell_role_rights_*` on `qemu-virtio-gpu`.
- Integration: session-host launcher Surface on H-003.
- Demo: cold boot to a native desktop showing the four applications (V0.5-D01).

#### Evidence
- none

### UIP-017 · Prototype UI protocol as scene graph, client buffers and hybrid; measure
- Type: spike
- Milestone: V0.5
- Status: todo
- Size: L
- Owner: none
- Depends on: IPC-010, IPC-012, MEM-005, MEM-010
- Baseline: §12, §41, §65
- Explores: S-015, S-032
- Risks: R-015

Prototype three transports for S-015 over Channel and MemoryObject: a retained scene graph sent to the compositor, client-rendered Buffers presented as Surfaces, and a hybrid that keeps a local retained tree while handing Buffers to the compositor. The protocol shape is nearly as permanent as the Native ABI (§65) and must exist as measured evidence before the V0.5 transport Decision, so the largest V0.5 body of new userspace code is not invented on the critical path. Measure latency, memory and toolkit ergonomics; cite B-018 for frame timing and do not state a number in the report prose.

<!-- covers: GAP-0516 -->

#### Out of scope
Compositor DRM/KMS output (GFX). Toolkit widget set (UIP-020). Wayland hosting (LNX-004).

#### Acceptance criteria
- [ ] A report at `reports/spikes/UIP-017.md` compares retained scene graph, client Buffers and hybrid on H-001 and H-002.
- [ ] Each option is exercised as a native Component talking only Channel and MemoryObject, with no Wayland or X11 types in the prototype IDL.
- [ ] The report records latency, memory and toolkit-ergonomics findings against B-018's method and names which option is ruled out.
- [ ] Accessibility metadata and compositor-restart rebind are scored as qualitative constraints on each option.
- [ ] The report records how clipboard offers and paste requests traverse each protocol model and where the clipboard Capability is checked (S-032).

#### Verification
- Report: which transport wins on measured latency, memory and toolkit ergonomics; which option is ruled out and why; how each option emits role, name and state; how each option rebinds after compositor restart; whether remote-surface remains possible without a kernel change.
- Bench: B-018 method on H-001 and H-002 for the three prototypes; publish-only, no target claimed.
- Review: UIP reviewer sign-off recorded on the pull request that lands the report.

#### Evidence
- none

### UIP-018 · Study Flutter, SwiftUI, Compose, Slint and Xilem declarative models
- Type: spike
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: DOC-001
- Baseline: §41, §58

Study Flutter, SwiftUI, Jetpack Compose, Slint and Xilem declarative models before fixing the native declarative UI model. The report feeds UIP-007 and UIP-020. This is a research study, not an ABI-surface exploration; it does not freeze S-015.

<!-- covers: INV-0784 -->

#### Out of scope
Toolkit architecture Decision (UIP-007). Toolkit core (UIP-020). Protocol transport Decision (UIP-006).

#### Acceptance criteria
- [ ] A report at `reports/spikes/UIP-018.md` covers Flutter, SwiftUI, Jetpack Compose, Slint and Xilem.
- [ ] Each model is scored on retained render tree, accessibility-tree output from the same source of truth, and fit to the decided or candidate protocol model.
- [ ] The report names at least one model property that the native toolkit must keep and one it must not copy.

#### Verification
- Report: how each model maps to a retained render tree; whether accessibility is a second tree; license and Rust-maturity constraints for Slint and Xilem; which properties the native toolkit must keep or reject.
- Review: UIP reviewer sign-off recorded on the pull request that lands the report.

#### Evidence
- none

### UIP-019 · Write UI protocol threat review: focus stealing, input injection, exfiltration
- Type: docs
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: SEC-002
- Baseline: §9, §9.1, §41, §51
- Threats: T-001, T-012, T-013, T-032

Write the UI protocol threat review that precedes the routing, clipboard and UI-capability designs it constrains. Clickjacking, focus stealing, synthetic input and screen-content leaks are the protocol's own threats; they extend the V0 SEC threat model rather than replacing it. Clipboard sniffing (T-001, T-032) and unauthorised screen contents (T-013) are in scope for the review, not for a compositor rewrite. Required by V0.5-G07 (Clipboard is a capability): the clipboard exfiltration threats it enumerates constrain the clipboard authority Decision whose denial that gate tests.

#### Out of scope
SEC V0 threat model (SEC-002). Compositor trusted-UI section (GFX-011). Routing Decision (UIP-005).

#### Acceptance criteria
- [ ] A committed review document enumerates focus stealing, synthetic input, clickjacking of trusted UI, clipboard exfiltration and screen-content leaks, each citing a T-ID.
- [ ] UIP-005, UIP-004 and UIP-025 cite this review in their Decision or Description closure.
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: SEC and UIP reviewers sign off on the pull request.

#### Evidence
- none

### UIP-020 · Build toolkit core: declarative element tree with retained render tree
- Type: build
- Milestone: V0.5
- Status: todo
- Size: L
- Owner: none
- Depends on: UIP-007, UIP-013, UIP-018
- Baseline: §41, §60
- Risks: R-015

Build the toolkit core: a declarative element tree with a retained render tree sufficient for the four V0.5 demo applications. The core speaks protocol v0, does not expose Wayland or X11, and is the Layer 4 framework the architecture Decision selected. Widgets, layout, GPU renderer and text are sibling tasks on this core.

<!-- covers: INV-0763, INV-0764, INV-1181 -->

#### Out of scope
Widget set (UIP-024). Layout engine (UIP-022). GPU renderer (UIP-021). Text widgets (UIP-023). Four demo applications (APP).

#### Acceptance criteria
- [ ] A native Component builds a declarative element tree, obtains a retained render tree, and presents a window through protocol v0 on H-003.
- [ ] Updating one element mutates the retained render tree without rebuilding the whole tree; a unit test asserts the retained node identity.
- [ ] Public toolkit crates contain no Wayland, X11 or POSIX window types.
- [ ] No `unsafe` outside the renderer boundary named by UIP-021.

#### Verification
- Unit: `sdk:tests/toolkit/element_tree_*` on `qemu-x86_64` and `qemu-virtio-gpu`.
- Integration: hello-window Component on H-003.
- Review: UIP reviewer confirms the implementation matches the architecture Decision.

#### Evidence
- none

### UIP-021 · Build GPU renderer drawing toolkit render tree into MemoryObject Buffers
- Type: build
- Milestone: V0.5
- Status: todo
- Size: L
- Owner: none
- Depends on: UIP-020, GFX-005, MEM-024, GFX-017, MEM-019
- Baseline: §16, §17, §41
- Risks: R-015

Build the GPU renderer that draws the toolkit retained render tree into application-owned MemoryObject Buffers. V0.5 requires GPU-accelerated composition of application windows rendered into Buffers owned by the application. The compositor composes those Buffers (GFX); this task does not drive KMS. Explicit GPU synchronization is mandatory; there is no implicit-sync path in the toolkit renderer.

<!-- covers: INV-0768 -->

#### Out of scope
Compositor GPU composition (GFX-006). Buffer object (GFX-005). Native GPU API Decision (GFX-017).

#### Acceptance criteria
- [ ] The renderer draws the retained render tree into a GPU-compatible MemoryObject Buffer owned by the application Component.
- [ ] Presentation uses explicit fences; an implicit-sync path is absent from the renderer crate and fails CI if added.
- [ ] A UI-only plus GPU-capability Component renders a window on H-002 and H-003 without holding screen-contents rights.
- [ ] `os inspect` on the Buffer prints owner, GPU-compatible property and the presenting Surface.

#### Verification
- Unit: `sdk:tests/toolkit/gpu_renderer_*` on `qemu-virtio-gpu`.
- Integration: GPU-presented toolkit window on H-002.
- Review: GFX reviewer confirms explicit sync and MemoryObject ownership.

#### Evidence
- none

### UIP-022 · Build responsive layout engine with constraints and reflow on resize
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: UIP-020
- Baseline: §41, §60

Build a constraint-based responsive layout engine that reflows the retained render tree on window resize. The four demo applications must reflow in their scripted scenarios. RTL mirroring and adaptive form-factor breakpoints wait for V2.

<!-- covers: INV-0767 -->

#### Out of scope
RTL mirroring (UIP-048). Adaptive form factors (UIP-043). Widget set (UIP-024).

#### Acceptance criteria
- [ ] Constraint layout of row, column and stack produces pixel-stable frames for a fixture tree at three window sizes on `qemu-virtio-gpu`.
- [ ] Resize of a demo application window reflows its widgets in the scripted scenario without clipping interactive controls.
- [ ] A layout cycle is bounded to the dirty subtree; a unit test asserts nodes outside the dirty region keep identity.

#### Verification
- Unit: `sdk:tests/toolkit/layout_reflow_*` on `qemu-x86_64` and `qemu-virtio-gpu`.
- Integration: four-app resize scenarios on H-003.

#### Evidence
- none

### UIP-023 · Integrate TXT shaping into toolkit text display and editing widgets
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: UIP-024, TXT-013, TXT-012, TXT-006, TXT-003
- Baseline: §41, §60
- Risks: R-019

Integrate the TXT shaping and editing model into toolkit text display and editing widgets so Editor and Terminal can pass acceptance. TXT owns shaping, the glyph atlas and IME protocol; UIP owns the widgets that consume them. A minimal ad-hoc text path in the toolkit is forbidden (R-019).

<!-- covers: INV-0771 -->

#### Out of scope
Shaping and rasterisation (TXT-013). Editing model (TXT-012). IME engines (TXT-029). Terminal cell grid (TXT-009).

#### Acceptance criteria
- [ ] Text display and text-field widgets render through the TXT layout stack, not through a toolkit-local font rasteriser.
- [ ] The Text Editor demo types, selects and deletes through the TXT editing model in its scripted scenario on H-003.
- [ ] The Terminal demo displays a grid of glyphs through the TXT monospace fast path in its scripted scenario on H-003.
- [ ] A CI lint fails a toolkit crate that links a second shaping or FreeType path beside the decided TXT stack.

#### Verification
- Unit: `sdk:tests/toolkit/text_widgets_*` on `qemu-virtio-gpu`.
- Integration: Editor and Terminal scripted scenarios on H-003.
- Review: TXT reviewer confirms widgets consume FontMatcher and the editing model.

#### Evidence
- none

### UIP-024 · Build widget set sufficient for Terminal, File Browser, Editor and Viewer
- Type: build
- Milestone: V0.5
- Status: todo
- Size: L
- Owner: none
- Depends on: UIP-020, UIP-022, GFX-040
- Baseline: §41, §60

Build the widget set the four APP demo applications need: buttons, lists, text fields, scroll views, menus, dialogs, splitters and image views. Each widget is a declarative element with a retained render node. Text rendering inside the fields is UIP-023; this task ships the widget shells and composition.

<!-- covers: INV-1181 -->

#### Out of scope
Four demo applications (APP-004, APP-006, APP-003, APP-005). Text shaping (UIP-023). Focus traversal (UIP-039).

#### Acceptance criteria
- [ ] Buttons, lists, text fields, scroll views, menus, dialogs, splitters and image views exist as public toolkit types and render on H-003.
- [ ] Each widget has a scripted unit test that constructs it declaratively and asserts a retained render node.
- [ ] File Browser, Editor, Terminal and Image Viewer can be composed from this set without a private widget crate.
- [ ] Dialogs cannot draw above a trusted-UI Surface (GFX-040).

#### Verification
- Unit: `sdk:tests/toolkit/widgets_v0_*` on `qemu-x86_64` and `qemu-virtio-gpu`.
- Integration: four-app composition on H-003.

#### Evidence
- none

### UIP-025 · Define Capability<UI> granted at launch: window creation without screen contents
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: UIP-013, CAP-007, GFX-026, UIP-019
- Baseline: §9, §9.1, §41
- Threats: T-001, T-013
- Invariants: I-021

Define `Capability<UI>` granted at launch: the right to create windows without screen contents, clipboard, or other-application Surfaces. V0.5 requires the Image Viewer to start with UI and GPU capabilities only, and a test that a UI-only Component cannot read screen contents (T-013). Selecting a photo still goes through UserSelected (STO), not through this Capability.

<!-- covers: INV-0213 -->

#### Out of scope
GPU Capability at launch (GFX-026). Screen-capture Capability (GFX-061). Chooser authority (STO-034). Image Viewer (APP-005).

#### Acceptance criteria
- [ ] A Component launched with `Capability<UI>` and `Capability<RenderQueue>` only can open a window and cannot read another Surface or the framebuffer; the denial is `Error::Rights` and is logged.
- [ ] Image Viewer starts with UI and GPU capabilities only, verified by `os inspect capability` on H-003.
- [ ] `Capability<UI>` does not imply clipboard, capture, or filesystem rights (I-021, T-001, T-013).

#### Verification
- Unit: `compositor:tests/ui/ui_capability_isolation_*` on `qemu-virtio-gpu`.
- Integration: Image Viewer inspect dump on H-003; isolation test shared with GFX-037.
- Review: CAP reviewer confirms rights encoding for `Capability<UI>`.

#### Evidence
- none

### UIP-026 · Automate V0.5 UI gates: window, input, animation, clipboard denial, a11y dump
- Type: build
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: UIP-013, UIP-012, UIP-010, UIP-003, UIP-001, UIP-015, UIP-025, UIP-024, UIP-002, BLD-027, ACC-003
- Baseline: §60

Wire the V0.5 UI exit criteria into CI on QEMU and the reference desktop so the gates are verified by tasks, not by prose: window open, input to the focused Surface, animation at refresh, clipboard denial, protocol version bump, and accessibility tree dump on each of the four apps.

#### Out of scope
Compositor KMS and rebind gates (GFX). Four-app scripted content (APP). Tree-dump command (ACC-003). B-018 publication (UIP-002).

#### Acceptance criteria
- [ ] CI on H-001, H-003 and H-002 runs a single UIP gate job that opens a native window, delivers keyboard and mouse to it, and asserts an unfocused Surface sees no input.
- [ ] The same job asserts clipboard denial without the Capability, the v0-to-v0.1 client still runs, and tree dumps exist for the four apps.
- [ ] A failing criterion fails the job with the task id in the log.

#### Verification
- Integration: UIP V0.5 gate job on `qemu-x86_64`, `qemu-virtio-gpu` and `hw-h002`.
- Review: BLD reviewer confirms the job is in post-merge CI.

#### Evidence
- none

### UIP-027 · Build animation API with frame-scheduled transitions and reduced-motion
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: UIP-010, UIP-020
- Baseline: §41, §42

Build the toolkit animation API with implicit and explicit transitions scheduled from compositor frame callbacks, plus a reduced-motion mode that disables non-essential animation. V0.5 already presents at refresh; this is the developer-facing animation surface INV-0769 names for V1. High-contrast pairing waits for UIP-053.

<!-- covers: INV-0769 -->

#### Out of scope
Frame callbacks (UIP-010). High-contrast themes (UIP-053). Accessibility reduced-motion setting storage (ACC-025).

#### Acceptance criteria
- [ ] Implicit and explicit animations run on frame callbacks and complete in a unit test that counts frames on `qemu-virtio-gpu`.
- [ ] Reduced-motion mode skips non-essential animations; a fixture animation does not advance while the mode is set.
- [ ] Animation does not allocate a new Buffer per frame when only opacity or transform changes; a unit test asserts Buffer identity.

#### Verification
- Unit: `sdk:tests/toolkit/animation_*` on `qemu-virtio-gpu`.
- Integration: scripted transition in a demo application on H-002.

#### Evidence
- none

### UIP-028 · Gate clipboard reads on paste gesture and add ClipboardRead for managers
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: UIP-003, UIP-004, UIP-012
- Baseline: §9, §9.1, §41
- Threats: T-001
- Invariants: I-021

Implement the accepted clipboard policy: reads succeed only on a user paste gesture into the focused Surface, or via `Capability<ClipboardRead>` held by the privileged shell clipboard manager. V1 L2 integration checks clipboard both directions for Linux windows (LNX-038). History remains a privileged shell feature (APP-024).

<!-- covers: GAP-0233, INV-0773 -->

#### Out of scope
Clipboard history UI (APP-024). Wayland data-device (LNX). OLE (WIN-021). Unified types (UIP-050).

#### Acceptance criteria
- [ ] A focused application without `Capability<ClipboardRead>` that reads the clipboard without a paste gesture receives `Error::Rights` and the contents are not delivered.
- [ ] A paste gesture into the focused Surface delivers the negotiated type and MemoryObject.
- [ ] A holder of `Capability<ClipboardRead>` reads without a paste gesture; a non-shell Component cannot mint that Capability (`Error::Rights`).
- [ ] Linux-personality clipboard both directions scores on C-003 integration on H-002 and H-004.

#### Verification
- Unit: `compositor:tests/ui/clipboard_paste_gesture_*` on `qemu-virtio-gpu`.
- Integration: paste-gesture and manager-read scenarios on H-002.
- Compat: C-003 clipboard scoring on H-002 and H-004.

#### Evidence
- none

### UIP-029 · Expose decoration, clipboard, drop and scale hooks for compatibility bridges
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: UIP-013, UIP-028, UIP-008, UIP-033, UIP-032
- Baseline: §41, §47, §49
- Risks: R-020

Expose protocol hooks for decoration, clipboard, drop and scale that personality bridges plug into. LNX owns the Wayland and XWayland bridge; UIP owns the native hooks so every GUI application can score clipboard both directions, correct scaling and native chrome at V1. The hooks consume native types only; they do not import Wayland requests into S-015 (R-020, I-048).

<!-- covers: GAP-0276 -->

#### Out of scope
Wayland bridge (LNX-006, LNX-040). XWayland (LNX-053). OLE drop (WIN-037).

#### Acceptance criteria
- [ ] Native protocol methods exist for decoration mode, clipboard offer/accept, drop target, and per-Surface scale, documented for bridge authors.
- [ ] LNX-038 can call those methods without linking Wayland types into the native compositor crate.
- [ ] A C-003 GUI entry scores clipboard both directions, correct scaling and native chrome on H-002 and H-004.

#### Verification
- Unit: `compositor:tests/ui/bridge_hooks_*` on `qemu-virtio-gpu`.
- Compat: C-003 integration scoring on H-002 and H-004.
- Review: LNX reviewer confirms the hooks are sufficient for the V1 Wayland path.

#### Evidence
- none

### UIP-030 · Decide global shortcut model: named actions bound in Settings, no key grabs
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: UIP-005, UIP-019
- Baseline: §9, §41
- Decision: D-0325
- Threats: T-012

Decide the global shortcut model. Applications request named actions bound by the user in Settings and never receive raw global key grabs. Push-to-talk, screenshots and media keys need global keys; the ambient X11 grab model is a keylogger primitive (T-012). Implementation is UIP-045 at V2; the V1 shortcut-focus-model depends on this Decision.

<!-- covers: GAP-0286 -->

#### Out of scope
Shortcut service (UIP-045). Settings binding UI (APP-044). Focus model implementation (UIP-037).

#### Acceptance criteria
- [ ] The Decision file evaluates at least: (A) named actions bound in Settings, no raw grabs; (B) compositor-reserved media and screenshot keys only, applications cannot register; (C) X11-style global key grabs (rejected per T-012).
- [ ] The accepted option forbids a Component from receiving key events while unfocused unless the user bound a named action to it.
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: UIP and SEC reviewers sign off on the pull request that accepts the Decision.

#### Evidence
- none

### UIP-031 · Write UI protocol reference and toolkit SDK guide from IDL at V1
- Type: docs
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: UIP-035, UIP-038, UIP-034, DOC-010, DOC-014
- Baseline: §41, §56.5, §66

Write the UI protocol reference and toolkit SDK guide content that V1 SDK publication and V3 documentation coverage rely on. DOC generates pages from IDL; UIP authors the normative protocol and toolkit prose. Layer 4 evolution is declared by UIP-034, not by this guide. Required by V3-G12 (Layer 1 ABI reference pages exist for every entry point): the toolkit SDK guide is part of the reviewed SDK guide set that gate names.

#### Out of scope
IDL-to-docs generator (DOC-010). Developer guide chassis (DOC-014). Layer 4 semver policy (UIP-034).

#### Acceptance criteria
- [ ] Protocol reference pages exist for every v1 method, generated from IDL plus authored prose, and build in the V1 docs CI.
- [ ] A toolkit SDK guide covers declarative trees, widgets, theming tokens, input, clipboard Capability and accessibility emission.
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: DOC and UIP reviewers sign off on the pull request.
- Integration: docs CI builds the UIP pages with DOC-012.

#### Evidence
- none

### UIP-032 · Implement drag and drop where a drop transfers a Capability to the target
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: UIP-003, UIP-012, STO-034, CAP-006, UIP-004
- Baseline: §9, §25, §41

Implement drag and drop where a drop transfers a Capability to the target, not a path. V1 daily-driving and the chooser demo (exactly one file granted) rely on this. Typed content negotiation reuses the clipboard type story. Cross-personality unification waits for UIP-050.

<!-- covers: INV-0772 -->

#### Out of scope
Unified native/Wayland/OLE types (UIP-050). File Browser chrome (APP-006). Chooser authority minting (STO-034).

#### Acceptance criteria
- [ ] Dropping a file from File Browser onto Image Viewer grants `Capability<Image, Read>` for that object only; `os inspect` shows no directory Capability on the viewer.
- [ ] A drop of an unsupported type is rejected with a typed error and allocates no handle.
- [ ] Cancelling a drag allocates no Capability on the target.
- [ ] A path string is never the drop payload on the native protocol.

#### Verification
- Unit: `compositor:tests/ui/drag_drop_cap_*` on `qemu-virtio-gpu`.
- Integration: File Browser to Image Viewer drop on H-002.
- Demo: chooser grants exactly one file (V0.5-D02 extended to drag).

#### Evidence
- none

### UIP-033 · Support per-Surface high-DPI and mixed-DPI scale across displays in toolkit
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: UIP-022, GFX-049, GFX-022
- Baseline: §41, §40, §61

Support per-Surface high-DPI and mixed-DPI scale across displays in the toolkit. V1 L2 integration scores correct scaling; the Intel laptop internal panel plus an external display differ in DPI (H-004). Fractional scale is computed by the compositor (GFX-049); the toolkit layouts and renders at the scale the Surface is told.

<!-- covers: INV-0777 -->

#### Out of scope
Compositor fractional scaling (GFX-049). V2 hot-plug without window loss (UIP-047). Appearance font scale (UIP-044).

#### Acceptance criteria
- [ ] A toolkit window on H-004's internal display and on an attached external display each render at the scale GFX reports for that Display.
- [ ] Moving a window between those displays relayouts and rerenders at the new scale without a protocol break.
- [ ] C-003 GUI scaling score is recorded for native chrome on H-004.

#### Verification
- Unit: `sdk:tests/toolkit/scale_factor_*` on `qemu-virtio-gpu`.
- Integration: internal plus external display on H-004.
- Compat: C-003 scaling scoring on H-004.

#### Evidence
- none

### UIP-034 · Declare Layer 4 UI toolkit evolution under semantic versioning
- Type: docs
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: UIP-007, SDK-054
- Baseline: §66, §52

Declare that Layer 4 (UI toolkit and application frameworks) evolves under semantic versioning independent of Layer 3 SDK and Layer 2 protocol. UIP owns Layer 4. SDK remains Layer 3 only (SDK-054). This is the V1 policy text; 1.0 restates it in UIP-057.

<!-- covers: INV-1287 -->

#### Out of scope
SDK Layer 3 semver (SDK-054). Protocol L2 freeze candidate (UIP-035). 1.0 stability statement (UIP-057).

#### Acceptance criteria
- [ ] A published policy states Layer 4 is the UI toolkit, evolves with semver, and is not a Layer 1 or Layer 2 freeze.
- [ ] The policy names that native applications depend on toolkit semver separately from SDK v1 crate semver.
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: UIP and SDK reviewers sign off on the pull request.

#### Evidence
- none

### UIP-035 · Publish UI protocol v1 as L2 freeze candidate with client compatibility suite
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: UIP-015, UIP-014, UIP-040, UIP-032, UIP-001, IPC-042, TXT-010, ACC-001
- Baseline: §12, §41, §65, §66
- Invariants: I-040

Publish UI protocol v1 as an L2 freeze candidate with a client compatibility suite proving v0 and v0.1 clients run on v1. L2 evolution rules freeze at V1 (IPC-042); S-015 is not frozen here (I-040 applies to Layer 1; L2 versions lock at V4). IME optional messages and the accessibility tree contract are in the candidate so they are not an incompatible v2 surprise.

<!-- covers: INV-0783 -->

#### Out of scope
L2 version lock (UIP-055). Conformance suite for third parties (UIP-054). Layer 1 freeze (ABI).

#### Acceptance criteria
- [ ] Protocol v1 IDL is published as the L2 freeze candidate and listed against S-015 in the candidate review.
- [ ] A compatibility suite proves v0 and v0.1 clients open a window, receive input and paste on a v1 compositor on H-003 and H-002.
- [ ] No Layer 1 surface is marked frozen by this task (I-040).

#### Verification
- Unit: `compositor:tests/ui/protocol_v1_compat_*` on `qemu-virtio-gpu`.
- Integration: v0 client on v1 compositor on H-002.
- Review: IPC reviewer confirms L2 candidate status and that S-015 is not frozen.

#### Evidence
- none

### UIP-036 · Add semantic actions (activate, expand, scroll-to) to the a11y tree in protocol
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: UIP-001, ACC-008, SEM-007
- Baseline: §41, §42

Add semantic actions (activate, expand, scroll-to) to the accessibility tree in the protocol so AT and the UI test driver can act without synthesising input. V2 screen-reader prototype and SEM's Terminal.run / Editor.open proof need actionable nodes on the tree emitted at V0.5. Action dispatch to widgets is ACC-006; this task is the protocol and toolkit emission of the actions.

<!-- covers: INV-0766 -->

#### Out of scope
Action dispatch from AT clients (ACC-006). Semantic interface registry (SEM-007). UI test driver (UIP-041). Input synthesis lint (SEM-002).

#### Acceptance criteria
- [ ] Every interactive V0.5 widget exposes at least `activate` on its tree node; lists expose `scroll-to` and expandable nodes expose `expand`.
- [ ] Invoking `activate` on a button through the tree performs the same mutation as a primary click, without a synthetic pointer event.
- [ ] A permanent test invokes Editor and Terminal widget actions through the tree on `qemu-virtio-gpu`.

#### Verification
- Unit: `sdk:tests/toolkit/semantic_actions_*` on `qemu-virtio-gpu`.
- Integration: action round-trip on H-003, consumed by ACC-006.

#### Evidence
- none

### UIP-037 · Implement shortcut and focus model forbidding global interception without Capability
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: UIP-030, UIP-012, UIP-039
- Baseline: §9, §41
- Threats: T-012

Implement the shortcut and focus model that forbids global key interception without a Capability. A Component cannot register a global key grab; focus follows the routing Decision. Named-action registration for user-bound shortcuts is the V2 service; this task is the V1 denial and in-window shortcut dispatch.

<!-- covers: INV-0787 -->

#### Out of scope
Named-action service (UIP-045). Settings bindings UI (APP-044). Widget focus ring (UIP-039).

#### Acceptance criteria
- [ ] A Component that calls a global key-grab API receives `Error::Rights` and allocates no handle; the test is retained permanently.
- [ ] In-window shortcuts fire only while the Surface is focused.
- [ ] Focus moves according to the routing Decision; `os inspect` names the focused Surface and Component.

#### Verification
- Unit: `compositor:tests/ui/no_global_grab_*` on `qemu-x86_64` and `qemu-virtio-gpu`.
- Integration: focus and in-window shortcut scenarios on H-002.

#### Evidence
- none

### UIP-038 · Build theming and styling system with tokens and runtime theme switch
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: UIP-024, SVC-013
- Baseline: §41

Build a theming and styling system with tokens and runtime theme switch for native applications. This is the base UIP-044 builds on at V2. Tokens cover color, type scale and spacing; light and dark are switchable at runtime without restarting the Component.

<!-- covers: INV-0785 -->

#### Out of scope
Light/dark, accent, icon, cursor, wallpaper and font scale as typed settings (UIP-044). High contrast (UIP-053). Settings app chrome (APP-040).

#### Acceptance criteria
- [ ] All V0.5 widgets take color, type and spacing from theme tokens, not from hard-coded colors.
- [ ] Switching light to dark at runtime relayouts and repaints an open window without Component restart.
- [ ] A unit test asserts a fixture widget's token-resolved color changes when the theme switches.

#### Verification
- Unit: `sdk:tests/toolkit/theme_tokens_*` on `qemu-virtio-gpu`.
- Integration: runtime theme switch on H-002.

#### Evidence
- none

### UIP-039 · Build keyboard focus traversal, focus ring and access keys across all widgets
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: UIP-024, UIP-001, ACC-002
- Baseline: §41, §42

Build keyboard focus traversal, a visible focus ring and access keys across all toolkit widgets. V4 keyboard-only operation of the shell and the V2 screen-reader prototype both need every toolkit widget reachable and activatable by keyboard. ACC owns the AT client and the focus-order contract; UIP owns widget traversal. Required by V4-G10 (Assistive-technology script completes in full): that gate verifies keyboard-only operation of the entire shell.

#### Out of scope
Focus-order contract (ACC-011). Screen reader (ACC-021). Keyboard-only shell verification (APP-067).

#### Acceptance criteria
- [ ] Tab and shift-tab visit every interactive widget in each V0.5 widget, in tree order, on `qemu-virtio-gpu`.
- [ ] The focused widget draws a focus ring that is not suppressed except on explicit pointer-driven focus where the schema allows.
- [ ] Access keys activate the labelled widget; a missing label fails the a11y metadata tests.
- [ ] A widget that cannot be focused by keyboard fails CI.

#### Verification
- Unit: `sdk:tests/toolkit/focus_traversal_*` on `qemu-virtio-gpu`.
- Integration: keyboard-only walk of each demo application on H-003.

#### Evidence
- none

### UIP-040 · Add touch, pen and gamepad event types to the UI protocol
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: UIP-012, UIP-013
- Baseline: §41

Add touch, pen and gamepad event types to the UI protocol so the V1 freeze-candidate review sees the full input surface INV-0770 lists. V2 gaming and target laptops consume the types; HW enablement of touchscreens, pens and gamepads may arrive later. Unfocused Surfaces still do not observe these events.

<!-- covers: INV-0770 -->

#### Out of scope
Touchscreen and touchpad gesture handling (UIP-049). Gamepad HID minting (HW-049). Pen digitizer enablement (HW-045).

#### Acceptance criteria
- [ ] Protocol v1 IDL includes touch, pen and gamepad event types with focus delivery rules identical to pointer and key.
- [ ] A fixture client on `qemu-virtio-gpu` receives injected touch, pen and gamepad events only while focused.
- [ ] An unfocused Surface observes none of those events; the test is retained permanently.

#### Verification
- Unit: `compositor:tests/ui/touch_pen_gamepad_*` on `qemu-virtio-gpu`.
- Review: protocol v1 freeze-candidate review includes these types.

#### Evidence
- none

### UIP-041 · Build UI test driver that scripts widgets through the a11y tree and actions
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: UIP-036, UIP-001, BLD-027
- Baseline: §41, §42

Build a UI test driver that scripts widgets through the accessibility tree and semantic actions rather than pixel automation. V0.5 app scenarios, the V2 40-scenario desktop UX script and the V4 50-task a11y script all need a driver that is not GUI pixel automation. ACC-014 is the AT-oriented sibling; this driver is the toolkit and CI face.

<!-- covers: GAP-0118 -->

#### Out of scope
ACC tree test driver (ACC-014). Semantic GUI harness plumbing (BLD-027). Pixel goldens (GFX-025).

#### Acceptance criteria
- [ ] The driver activates, types into and scrolls toolkit widgets through tree actions on `qemu-virtio-gpu` with no synthetic pointer path.
- [ ] Each of the four demo applications has at least one driver-scripted scenario in CI.
- [ ] A widget missing an `activate` action fails the driver run rather than falling back to coordinates.

#### Verification
- Unit: `sdk:tests/toolkit/ui_driver_*` on `qemu-virtio-gpu`.
- Integration: four-app driver scenarios on H-003.

#### Evidence
- none

### UIP-042 · Automate V1 UI gates: L2 integration scoring and frame-time regression
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: UIP-035, UIP-028, UIP-033, UIP-029, UIP-002
- Baseline: §54, §61
- Benchmarks: B-018
- Corpora: C-003

Automate the V1 UI gates: L2 integration scoring (clipboard both directions, correct scaling, native chrome) and B-018 regression versus V0.5 on H-002 and H-004. LNX owns corpus entries; UIP owns the native hooks and the toolkit regression client.

#### Out of scope
L2 corpus content (LNX-056). Compositor-side B-018 (GFX-003). SDK compatibility suite (SDK-036).

#### Acceptance criteria
- [ ] CI records C-003 clipboard, scaling and chrome scores for native hooks on H-002 and H-004.
- [ ] B-018 V1 regression versus V0.5 is measured on H-002 and H-004; exceedance without an accepted Decision fails the job.
- [ ] Protocol v1 compatibility suite is in the same job.

#### Verification
- Compat: C-003 integration scoring on H-002 and H-004.
- Bench: B-018 on H-002 and H-004; target per register.
- Integration: protocol v1 compat suite on `qemu-virtio-gpu`.

#### Evidence
- none

### UIP-043 · Support adaptive layouts for desktop, laptop and external display in toolkit
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: UIP-022, UIP-033
- Baseline: §41, §62

Support adaptive layouts for desktop, laptop and external display in the toolkit. The V2 laptop-day demo plugs an external HDR display at a different scale; shell UX scripts include external display and scaling change. On-screen keyboard, tablet mode and convertible UI stay out of 1.0 (UIP-058).

<!-- covers: INV-0776 -->

#### Out of scope
Hot-plug without window loss (UIP-047). Tablet mode and on-screen keyboard (UIP-058). HDR pipeline (GFX).

#### Acceptance criteria
- [ ] Toolkit layout breakpoints for desktop, laptop-internal and external-display widths reflow a fixture application on H-002, H-004 and H-005.
- [ ] Plugging an external display at a different scale on H-004 relayouts the focused native window to the external breakpoint.
- [ ] No toolkit API for tablet-mode chrome or on-screen keyboard ships in this task.

#### Verification
- Unit: `sdk:tests/toolkit/adaptive_breakpoints_*` on `qemu-virtio-gpu`.
- Integration: external display attach on H-004 and H-005.
- Demo: laptop-day external display on H-004.

#### Evidence
- none

### UIP-044 · Expose light/dark, accent, icon, cursor, wallpaper and font scale as typed settings
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: UIP-038, SVC-028, SVC-013
- Baseline: §41, §49, §62

Expose light/dark, accent color, icon and cursor themes, wallpaper and font scale as typed settings consumed by native applications. LNX projects the same settings through the settings portal and GTK/Qt platform hints (LNX-077). APP owns the Settings appearance panel chrome.

<!-- covers: GAP-0281 -->

#### Out of scope
Settings appearance panel (APP-040). Linux settings portal (LNX-077). High contrast (UIP-053). Wallpaper compositor rendering (GFX).

#### Acceptance criteria
- [ ] Typed settings exist for light/dark, accent, icon theme, cursor theme, wallpaper and font scale, readable by a native Component through the settings client.
- [ ] Changing light/dark or font scale at runtime updates open native windows without restart.
- [ ] The settings objects are the values LNX-077 reads; native and portal paths do not have a second store.

#### Verification
- Unit: `sdk:tests/toolkit/appearance_settings_*` on `qemu-virtio-gpu`.
- Integration: runtime appearance change on H-002, H-004 and H-005.
- Review: LNX reviewer confirms portal mapping inputs.

#### Evidence
- none

### UIP-045 · Implement named-action global shortcuts bound by the user in Settings
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: UIP-037, UIP-030, SVC-013
- Baseline: §9, §41, §62
- Threats: T-012

Implement named-action global shortcuts bound by the user in Settings. Push-to-talk, screenshot and media keys are needed by V2 screen-sharing, conferencing and gaming. Applications request named actions; they never receive raw global key grabs (T-012). APP-044 is the Settings binding chrome.

<!-- covers: GAP-0286 -->

#### Out of scope
Settings binding UI (APP-044). Screenshot UI (APP-038). Media session chrome (APP-034).

#### Acceptance criteria
- [ ] An application that registers a named action does not receive key events until the user binds that action in Settings; unbound, `Error::Rights` on a grab attempt.
- [ ] A user-bound action delivers one event to the registrant even if unfocused; unbinding stops delivery within one event.
- [ ] Two applications cannot bind the same named action to the same chord; the second bind is rejected.
- [ ] `os inspect` lists bound actions, holders and chords.

#### Verification
- Unit: `compositor:tests/ui/named_shortcuts_*` on `qemu-virtio-gpu`.
- Integration: screenshot and media-key bindings on H-004 and H-005.

#### Evidence
- none

### UIP-046 · Provide locale-aware date, number, currency and collation APIs to native apps
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: TXT-031, TXT-016, SVC-025
- Baseline: §41

Provide locale-aware date, number, currency and collation APIs to native applications, consumed by the toolkit. TXT owns the Locale object and data source; UIP exposes formatting through toolkit widgets (lists, tables, text fields). The translated-language set and percent-translated gate are TXT's V4 localisation gate, not this task.

<!-- covers: GAP-0259 -->

#### Out of scope
Locale object (TXT-031). Translation catalogs (TXT-032). Ten-language gate (TXT-046). RTL mirroring (UIP-048).

#### Acceptance criteria
- [ ] Toolkit text and list widgets format dates, numbers and currency through the TXT Locale object, not through a toolkit-local formatter.
- [ ] Collation order of a fixture list changes when the session Locale changes, without Component restart.
- [ ] A unit test for at least one RTL locale and one non-Gregorian date format passes on `qemu-x86_64`.

#### Verification
- Unit: `sdk:tests/toolkit/locale_format_*` on `qemu-x86_64`.
- Integration: locale switch in a demo application on H-002.
- Review: TXT reviewer confirms widgets call Locale, not a second CLDR copy.

#### Evidence
- none

### UIP-047 · Handle per-display scale and refresh changes and hot-plug without window loss
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: UIP-033, UIP-043, GFX-065
- Baseline: §41, §40, §62

Handle per-display scale and refresh changes and hot-plug in the toolkit and protocol so windows migrate without loss. V2 requires two displays with different scale factors and refresh rates, hot-plugged the count named in the verifying GFX task, without compositor restart or window loss. GFX owns the compositor side; UIP owns Surface migration and toolkit relayout.

<!-- covers: INV-0777, INV-0776 -->

#### Out of scope
Compositor hot-plug and MST (GFX). Display arrangement persistence (GFX-065). HDR/VRR (GFX).

#### Acceptance criteria
- [ ] A native window moved or hot-plugged onto a second display at a different scale and refresh relayouts and presents without the Component exiting.
- [ ] Disconnecting the display that held a window restores that window on a remaining display; `os inspect` still names the Surface.
- [ ] The V2 hot-plug loop named in the GFX verifying task loses zero native toolkit windows on H-002, H-004 and H-005.

#### Verification
- Integration: dual-display hot-plug on H-002, H-004 and H-005.
- Unit: `sdk:tests/toolkit/surface_migrate_*` on `qemu-virtio-gpu`.

#### Evidence
- none

### UIP-048 · Implement right-to-left layout mirroring as a toolkit property
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: UIP-022, UIP-046, TXT-038
- Baseline: §41

Implement right-to-left layout mirroring as a toolkit property so RTL is architectural rather than a translation afterthought. TXT owns bidi resolution of text runs; UIP mirrors widget geometry, scroll direction and focus traversal. V2 three-language shell includes one RTL language (TXT-033).

<!-- covers: GAP-0259 -->

#### Out of scope
Bidi of text runs (TXT-038). Translation catalogs (TXT). Appearance settings (UIP-044).

#### Acceptance criteria
- [ ] Setting locale direction to RTL mirrors row, splitter and scroll-view geometry for a fixture tree; a pixel-stable test on `qemu-virtio-gpu` asserts the mirror.
- [ ] Focus traversal in RTL visits widgets in mirrored visual order.
- [ ] Mixed LTR text runs inside an RTL layout are ordered as the Unicode Bidirectional Algorithm reference implementation orders them, via TXT bidi and not toolkit-local reordering.

#### Verification
- Unit: `sdk:tests/toolkit/rtl_mirror_*` on `qemu-virtio-gpu`.
- Integration: RTL locale on a demo application on H-002.
- Review: TXT reviewer confirms bidi stays in the text stack.

#### Evidence
- none

### UIP-049 · Ship basic touchscreen and touchpad gestures in toolkit and routing
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: UIP-040, HW-058, HW-045
- Baseline: §41, §62

Ship basic touchscreen and touchpad gestures: tap, scroll, pinch and swipe on the V2 target laptops' touchpads and touchscreens. HW owns libinput gesture recognition; UIP routes gesture events to the focused Surface and handles them in widgets. On-screen keyboard, tablet mode and convertible UI stay out (GAP-0287).

<!-- covers: GAP-0287 -->

#### Out of scope
On-screen keyboard, tablet mode, convertible UI (UIP-058). Gesture recognition (HW-058). Pen digitizer besides basic events (HW-045).

#### Acceptance criteria
- [ ] Tap, scroll, pinch and swipe on H-004 and H-005 reach the focused toolkit Surface and scroll or scale the target widget.
- [ ] An unfocused Surface observes no gesture events.
- [ ] No on-screen keyboard or tablet-mode chrome is shipped by this task.

#### Verification
- Integration: touchpad gesture scenarios on H-004 and H-005.
- Unit: `compositor:tests/ui/gestures_focus_*` on `qemu-virtio-gpu`.

#### Evidence
- none

### UIP-050 · Unify drag-and-drop and clipboard content types across native, Wayland and OLE
- Type: build
- Milestone: V2
- Status: todo
- Size: L
- Owner: none
- Depends on: UIP-032, UIP-028, UIP-029, LNX-038, WIN-021
- Baseline: §9, §41, §47, §48, §49
- Corpora: C-003, C-007

Unify drag-and-drop and clipboard content types across native, Wayland (`wl_data_device`, XDND) and Windows (OLE) so a drop grants the target a Capability to the dropped object rather than a path. V2 requires Windows applications to use the native clipboard and file chooser; dragging a file from a native File Browser into a Windows application and the reverse is the litmus test.

<!-- covers: GAP-0276 -->

#### Out of scope
Wayland data-device implementation (LNX). OLE implementation (WIN-037, WIN-021). Native drop protocol (UIP-032).

#### Acceptance criteria
- [ ] A native-to-Linux, Linux-to-native, native-to-Windows and Windows-to-native file drop each grants the target a Capability to the object, not a path, on H-002.
- [ ] Clipboard types negotiated across the three personalities round-trip a fixture text and a fixture image type.
- [ ] A drop that cannot mint a Capability is rejected with a typed error in every personality.
- [ ] C-003 and C-007 clipboard and chooser integration scores include these paths on H-002.

#### Verification
- Integration: four-direction drop matrix on H-002.
- Compat: C-003 and C-007 clipboard and chooser scoring on H-002.
- Review: LNX and WIN reviewers confirm bridges consume the unified type registry.

#### Evidence
- none

### UIP-051 · Automate V2 desktop UX script and appearance consistency on three machines
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: UIP-044, UIP-047, UIP-041, UIP-050, APP-048, BLD-056
- Baseline: §62

Automate the V2 desktop UX script and appearance consistency checks on H-002, H-004 and H-005. APP owns the shell scenarios; UIP owns the toolkit-level harness and theming consistency so native and compatibility windows share light/dark, accent and font scale. Zero P0 or P1 in the forty-scenario script is the APP gate; this task fails when toolkit theming or window migration is the defect.

#### Out of scope
Forty-scenario content (APP-048). Harness plumbing (BLD-056). Shell panel (APP).

#### Acceptance criteria
- [ ] The UI test driver runs the toolkit and theming subset of the forty-scenario script on H-002, H-004 and H-005.
- [ ] Appearance consistency checks fail the job when a native window ignores the session light/dark or font scale setting.
- [ ] Multi-display scale-change scenarios in the script are attributed to UIP when the Surface is lost.

#### Verification
- Integration: toolkit subset of the UX script on H-002, H-004 and H-005.
- Review: APP reviewer confirms scenario ownership split.

#### Evidence
- none

### UIP-052 · Fuzz UI protocol decoders and compositor-facing endpoints continuously
- Type: build
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: UIP-035, BLD-035, IPC-029
- Baseline: §41, §51

Fuzz UI protocol decoders and compositor-facing endpoints continuously. V3 requires continuous fuzzing; the protocol decoder is the largest untrusted-input surface between applications and the compositor. Harnesses are IDL-generated where IPC-029 supports it.

<!-- covers: GAP-0130 -->

#### Out of scope
Fuzz infrastructure (BLD-035). IDL mutators (IPC-029). Kernel syscall fuzz (BLD).

#### Acceptance criteria
- [ ] Continuous fuzz targets exist for protocol decode, clipboard offer parse and drop-target parse, running in BLD's fleet.
- [ ] A known crasher in these targets is filed with a task id and does not remain open past the V3 fuzz SLA BLD enforces.
- [ ] Corpus seeds include v0, v0.1 and v1 messages plus truncated and oversize MemoryObject handles.

#### Verification
- Fuzz: `compositor:fuzz/ui_protocol` in the V3 continuous fleet with no known open crasher older than the BLD register window.
- Review: BLD reviewer confirms the targets are scheduled.

#### Evidence
- none

### UIP-053 · Add high-contrast themes and reduced-motion mode across all toolkit widgets
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: UIP-038, UIP-027, ACC-025, UIP-044
- Baseline: §41, §42

Add high-contrast themes and reduced-motion mode across all toolkit widgets. V4 requires high contrast across shell and native apps; building at V3 leaves a milestone to verify rather than to start. ACC-025 owns the settings; UIP applies tokens so every native app complies without per-app work.

<!-- covers: GAP-0264 -->

#### Out of scope
Accessibility settings service (ACC-025). Shell chrome (APP). Conformance audit (ACC-034).

#### Acceptance criteria
- [ ] A high-contrast token set covers every V0.5 and V1 widget; a fixture gallery renders only high-contrast tokens when the setting is on.
- [ ] Reduced-motion mode disables the non-essential animations named by UIP-027 across those widgets.
- [ ] Switching either setting at runtime updates open native windows without Component restart.

#### Verification
- Unit: `sdk:tests/toolkit/high_contrast_*` and `sdk:tests/toolkit/reduced_motion_*` on `qemu-virtio-gpu`.
- Integration: settings toggle on H-002.
- Review: ACC reviewer confirms tokens match the V3 checklist contrast requirements.

#### Evidence
- none

### UIP-054 · Publish UI protocol conformance suite for third-party toolkits and bridges
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: UIP-035, UIP-029, UIP-052
- Baseline: §12, §41, §66

Publish a UI protocol conformance suite for third-party toolkits and bridges. V3's public repository means non-core toolkits talk the protocol; the suite is also the precondition for the V4 L2 version lock. Cases cover window open, input focus, clipboard rights, drop Capability transfer, scale, decorations and old-client/new-service. Required by the UIP scope: "the conformance suite that third-party toolkits and personality bridges must pass".

#### Out of scope
L2 version lock (UIP-055). Personality bridges (LNX, WIN). SDK language bindings (SDK).

#### Acceptance criteria
- [ ] A public suite binary, built against protocol v1, passes against the V3 compositor on H-001, H-002 and H-003.
- [ ] The suite includes old-client/new-service and new-client/old-service cases retained from UIP-015.
- [ ] A third-party toolkit fixture in the V3 public repository is gated on this suite in CI.

#### Verification
- Integration: suite on `qemu-x86_64`, `qemu-virtio-gpu` and `hw-h002`.
- Review: IPC reviewer confirms the suite is sufficient for the V4 lock.

#### Evidence
- none

### UIP-055 · Lock UI protocol L2 version for 1.x and remove deprecated messages
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: UIP-054, UIP-035, UIP-017, UIP-006, UIP-004
- Baseline: §12, §41, §65, §66
- Freezes: S-015, S-032

Lock the UI protocol L2 version for 1.x, remove deprecated messages, and freeze S-015. The compatibility suite proves V1-era clients run on the locked version. Layer 1 is not frozen by this task. Clipboard capability (S-032) stays a distinct surface; this lock covers the protocol messages that carry it.

<!-- covers: INV-0783 -->

#### Out of scope
Layer 1 freeze (ABI). Toolkit Layer 4 semver (UIP-034). IME surface lock (TXT-045). Accessibility tree lock (ACC).

#### Acceptance criteria
- [ ] S-015 is `frozen` by this task; protocol versions served for 1.x are enumerated.
- [ ] Deprecated messages present in v1 and removed here have a conformance-suite case that the locked compositor still handles from a V1 client, or an accepted Decision documents the break.
- [ ] Old-client/new-service and new-client/old-service pass for every locked method on H-002 and H-003.

#### Verification
- Integration: conformance suite against the locked compositor on H-002 and H-003.
- Review: IPC reviewer confirms S-015 freeze closure (spike, Decision, this task).

#### Evidence
- none

### UIP-056 · Verify every shipped widget passes a11y metadata and keyboard conformance
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: UIP-001, UIP-039, UIP-053, ACC-028, UIP-041
- Baseline: §41, §42

Verify every shipped toolkit widget passes accessibility metadata and keyboard conformance so V4's 50-task assistive-technology script and keyboard-only shell can complete. ACC runs the script and owns the checklist; UIP fixes widget-level defects the script surfaces and fails CI when a shipped widget regresses metadata, focus or contrast.

#### Out of scope
50-task script (ACC-031). Conformance audit of shell and apps (ACC-034). Screen reader (ACC).

#### Acceptance criteria
- [ ] Every public toolkit widget passes metadata, keyboard-focus and high-contrast unit tests on `qemu-virtio-gpu`.
- [ ] A new public widget that omits role, name, state, `activate` or keyboard focus fails CI.
- [ ] Defects ACC-034 files against toolkit widgets are closed or tracked with a widget-level regression test.

#### Verification
- Unit: `sdk:tests/toolkit/a11y_conformance_*` on `qemu-virtio-gpu`.
- Integration: widget gallery driven by UIP-041 on H-002.
- Review: ACC reviewer confirms the widget gallery covers the checklist's toolkit rows.

#### Evidence
- none

### UIP-057 · Publish UI protocol and toolkit API stability statement for 1.x
- Type: docs
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: UIP-055, UIP-034
- Baseline: §41, §66, §70

Publish the UI protocol and toolkit API stability statement for 1.x: the locked protocol version, toolkit semver policy, and the explicit non-promises that on-screen keyboard, tablet mode and convertible form-factor UI are not in 1.0 (GAP-0287). This is the 1.0 ABI-adjacent statement for Layer 2 UI and Layer 4 toolkit, beside the Layer 1 ABI statement ABI owns.

<!-- covers: GAP-0287 -->

#### Out of scope
Layer 1 ABI stability statement (ABI). Layer 3 SDK statement (SDK-095). Parked tablet/OSK implementation (UIP-058).

#### Acceptance criteria
- [ ] A published 1.0 statement names the locked S-015 version, Layer 4 semver policy, and that on-screen keyboard, tablet mode and convertible UI are not promised.
- [ ] The statement lists no performance number; any latency claim cites a B-ID.
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: UIP and GOV reviewers sign off on the pull request.

#### Evidence
- none

### UIP-058 · Park on-screen keyboard, tablet mode and convertible form-factor UI
- Type: build
- Milestone: LATER
- Status: todo
- Size: L
- Owner: none
- Depends on: UIP-043, UIP-049, HW-051
- Baseline: §41, §62

Park on-screen keyboard, tablet mode and convertible form-factor UI after 1.0. GAP-0287 excludes them from 1.0; INV-0776 names future tablet. Adaptive-form-factors must not be stretched into these before 1.0. Basic touchscreen and touchpad gestures remain in UIP-049.

<!-- covers: GAP-0287, INV-0776 -->

#### Out of scope
Basic gestures (UIP-049). Adaptive desktop/laptop/external layouts (UIP-043). 1.0 non-promise text (UIP-057).

#### Acceptance criteria
- [ ] A design note in this task's Evidence names on-screen keyboard, tablet mode and convertible chrome as post-1.0 work with a split into those three pieces.
- [ ] No 1.0 toolkit crate exports APIs whose only purpose is on-screen keyboard, tablet mode or convertible hinge state.
- [ ] Convertible hinge events from HW-051 are ignored by 1.0 toolkit layout.

#### Verification
- Review: UIP reviewer confirms 1.0 crates have no OSK, tablet-mode or convertible APIs.
- Unit: `sdk:tests/toolkit/no_tablet_apis_*` on `qemu-x86_64`.

#### Evidence
- none
