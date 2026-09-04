# GFX · Graphics and compositor
- Prefix: GFX
- Lead: none
- Baseline: §9.1, §16, §17, §23, §32, §33, §39, §40, §47, §54, §55, §56.1, §57, §60, §61, §62

<!-- roadmap:generated:begin summary -->
Tasks: 98 live, 0 done, 0 in-progress, 98 todo, 0 dropped. Ready: 0. Blocked: 98. Weighted: 0%.
<!-- roadmap:generated:end -->

## Scope

GFX owns the native graphics stack above retained DRM/KMS: Surface, Buffer, RenderQueue, ComputeQueue, Display and Frame; the compositor as a privileged user-space Component; crash rebind of live windows; compositor-protected trusted UI and lock enforcement; colour management, HDR, VRR, overlay planes, direct scanout, hybrid graphics and DisplayPort MST; explicit screen-capture capabilities and the OS-owned capture tools that use them. Native applications never receive DRM ioctls, device nodes or fourcc details. GPU driver residency stays with the inherited Linux DRM modules through 1.0 (I-045).

## Out of scope

Kernel DRM driver code and rebase of amdgpu, i915/xe, nouveau and virtio-gpu (KRN, HW). MemoryObject GPU-compatible flags and dma-buf backing (MEM). UI protocol, toolkit and input-routing contract (UIP). Shell chrome, lock UI, in-use icons and settings panels (APP). HID devices and GPU SKU bring-up (HW). Wayland and X11 personality hosting (LNX). Wine present and HDR passthrough (WIN). ComputeDevice taxonomy (HET). Photodiode rig, HDR display and colorimeter (LAB). Benchmark methodology (BEN). Service supervision (SVC). Energy policy signals (PWR, SCH). Codec encode of captured frames (MED). VM manager and guest tools (VIRT). Capability encoding (CAP). Threat-model document chassis (SEC).

## Tasks

### GFX-001 · Inventory retained DRM/KMS drivers and pin the no-native-GPU-stack rule
- Type: docs
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: KRN-017, HW-003
- Baseline: §5.1, §39, §55, §56.1, §57
- Risks: R-013
- Invariants: I-045, I-054

Collapse every GFX retain-Linux DRM item into one inventory that names amdgpu, i915/xe, nouveau and virtio-gpu as the inherited graphics mechanism under native Surface and Buffer objects. The inventory feeds KRN's retained-mechanism list and records the §57 non-goal that JakeOS does not ship a native GPU driver stack before 1.0.

<!-- covers: INV-0128, INV-1060, INV-1117, INV-0022, INV-0067, INV-0723, INV-0724 -->

#### Out of scope
Kernel DRM code changes (KRN). Per-rebase regression runs (GFX-027).

#### Acceptance criteria
- [ ] The inventory lists amdgpu, i915/xe, nouveau and virtio-gpu with the native object each backs.
- [ ] The inventory states that native applications do not open DRM device nodes or issue DRM ioctls.
- [ ] The inventory is cited by KRN-017 as the graphics row of the retained-mechanism list.
- [ ] A standing review rule rejects a patch that adds a native kernel GPU driver before 1.0 without an accepted decision.

#### Verification
- Review: KRN and GFX leads record inventory sign-off on the pull request.
- Unit: `gfx:tests/inventory/retained_drm_*` on CI matrix entry `qemu-x86_64`.

#### Evidence
- none

### GFX-002 · Benchmark compositor restart-to-rebound time
- Type: benchmark
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: GFX-007, BEN-007, LAB-003
- Baseline: §32, §40, §54
- Benchmarks: B-023
- Risks: R-023

Deliver the harness and published measurement for compositor restart-to-rebound. The V0.5 gate is publish-only; later rungs read targets from B-023. Linux compositors terminate their clients, so this metric is published alone.

#### Out of scope
BEN methodology (BEN-007). Client rebind protocol (GFX-009).

#### Acceptance criteria
- [ ] Harness `bench:compositor-rebound` kills the compositor with ten open windows and records time until every window is presented with input working.
- [ ] A report exists for H-003 and H-002 meeting the B-023 target kind for V0.5.
- [ ] `os inspect` on the rebound session lists the same window objects as before the kill.

#### Verification
- Bench: B-023 on H-003 and H-002; target per register.
- Integration: `gfx:tests/compositor/rebound_bench_*` on CI matrix entries `qemu-virtio-gpu` and `hw-h002`.

#### Evidence
- none

### GFX-003 · Benchmark compositor commit-to-scanout latency against a Wayland compositor
- Type: benchmark
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: GFX-024, GFX-006, BEN-007, LAB-003
- Baseline: §40, §54
- Benchmarks: B-018

Deliver the harness and published measurement for compositor commit-to-scanout latency beside a Wayland compositor on the same hardware. The V0.5 gate is publish-only; numbers live only in B-018 reports.

#### Out of scope
Input-to-photon latency (GFX-004). Frame-deadline miss rate (GFX-060).

#### Acceptance criteria
- [ ] Harness `bench:compositor-frame-latency` timestamps client Surface commit and display-controller vblank on H-002.
- [ ] The same client is measured on a wlroots compositor, KWin and Mutter on H-002 in the same session.
- [ ] A report exists for H-002 meeting the B-018 target kind for V0.5.

#### Verification
- Bench: B-018 on H-002; target per register.
- Integration: `gfx:tests/compositor/frame_latency_bench_*` on CI matrix entry `hw-h002`.

#### Evidence
- none

### GFX-004 · Benchmark GUI Input-to-photon latency on the photodiode rig
- Type: benchmark
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: GFX-031, GFX-006, LAB-001, UIP-011, BEN-007
- Baseline: §40, §54
- Benchmarks: B-020
- Risks: R-022

Deliver the harness that drives LAB's photodiode rig and publishes GUI Input-to-photon latency beside a Linux Wayland desktop on the same machine. The V0.5 gate is publish-only; numbers live only in B-020 reports.

#### Out of scope
Photodiode fixture hardware (LAB-001). Toolkit-stage timestamps (UIP-011).

#### Acceptance criteria
- [ ] Harness `bench:input-to-photon` records physical input to pixel change for a native application and a Linux personality application on H-002.
- [ ] The same rig measures a Linux Wayland desktop on H-002 in the same session.
- [ ] A report exists for H-002 meeting the B-020 target kind for V0.5.

#### Verification
- Bench: B-020 on H-002; target per register.
- Manual: LAB operator confirms the photodiode trigger log matches the harness run id.

#### Evidence
- none

### GFX-005 · Implement Buffer as a GPU-compatible MemoryObject
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: GFX-034, MEM-024, MEM-019, MEM-028
- Baseline: §16, §17, §39
- Invariants: I-045

Define Buffer as the GPU-compatible MemoryObject the compositor and renderers query. MEM owns the kernel property flag; GFX owns Buffer semantics, importability queries and the typed Interface over that flag so native software never sees dma-buf file descriptors.

<!-- covers: INV-0727, INV-0311, INV-0335 -->

#### Out of scope
Kernel GPU-compatible property (MEM-024). Zero-copy import verification (GFX-044).

#### Acceptance criteria
- [ ] Creating a Buffer without GPU-compatible rights returns `Error::Rights` and allocates no handle.
- [ ] `os inspect buffer` reports GPU-accessible and GPU-compatible properties without a DRM fourcc or device node.
- [ ] A native Component holding only `Capability<Buffer>` cannot open a DRM device node.
- [ ] Public Buffer IDL contains no drm_fourcc, drm ioctl or device-node field.

#### Verification
- Unit: `gfx:tests/buffer/gpu_compatible_*` on CI matrix entries `qemu-virtio-gpu` and `hw-h002`.
- Integration: `gfx:tests/buffer/inspect_*` on `qemu-virtio-gpu`.

#### Evidence
- none

### GFX-006 · Compose Surfaces on the GPU with scene graph and damage tracking
- Type: build
- Milestone: V0.5
- Status: todo
- Size: L
- Owner: none
- Depends on: GFX-008, GFX-018, GFX-038, GFX-005, GFX-023
- Baseline: §39, §40, §60
- Risks: R-015

GPU-accelerated composition using the decided render backend. A scene graph with damage tracking is the compositor's only path to scanout for windowed Surfaces, so the idle desktop is not recomposed from scratch every refresh.

<!-- covers: INV-0738, INV-0737, INV-1178 -->

#### Out of scope
KMS output programming (GFX-008). Overlay direct scanout (GFX-079).

#### Acceptance criteria
- [ ] Composition of two client Surfaces plus a cursor is performed by the GPU on H-002 and H-003.
- [ ] An undamaged idle desktop records no composition pass in `os trace` for a full refresh period.
- [ ] The compositor process has no implicit-sync import of client Buffers (I-084).
- [ ] Public compositor IDL names Surfaces and Frames, not DRM framebuffers.

#### Verification
- Integration: `gfx:tests/compositor/gpu_compose_*` on CI matrix entries `qemu-virtio-gpu` and `hw-h002`.
- Demo: two native windows plus software cursor presented on H-002.

#### Evidence
- none

### GFX-007 · Automate compositor kill and verify every client Surface rebinds and renders
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: GFX-009, BLD-020, SVC-002
- Baseline: §32, §40
- Risks: R-023

Permanent regression test for the V0.5 compositor-restart gate. The test kills the compositor and asserts every client Surface rebinds and continues rendering, the number of consecutive times named here, on QEMU and on hardware.

<!-- covers: INV-0756 -->

#### Out of scope
Rebind protocol (GFX-009). Restart-to-rebound benchmark (GFX-002).

#### Acceptance criteria
- [ ] The test passes 100 consecutive compositor kills on H-003 with ten open windows and no client exit.
- [ ] The test passes 20 consecutive compositor kills on H-002 with ten open windows and no client exit.
- [ ] A client that exits during rebind fails the test.
- [ ] The test is a required CI job on every compositor change.

#### Verification
- Integration: `gfx:tests/compositor/kill_rebind_*` on CI matrix entries `qemu-virtio-gpu` and `hw-h002`.
- Fuzz: `gfx:fuzz/compositor_kill_timing` one hour nightly without panic.

#### Evidence
- none

### GFX-008 · Drive a single fixed-refresh display through atomic DRM/KMS from the compositor
- Type: build
- Milestone: V0.5
- Status: todo
- Size: L
- Owner: none
- Depends on: GFX-022, GFX-013, GFX-041, HW-010
- Baseline: §39, §40, §60
- Invariants: I-045

V0.5 exit: the compositor drives the reference GPU via DRM/KMS and presents at the display's fixed refresh. Atomic commits are the only KMS path; native clients never hold DRM master.

<!-- covers: INV-0737, INV-1178 -->

#### Out of scope
GPU scene-graph composition (GFX-006). Multi-monitor (GFX-077).

#### Acceptance criteria
- [ ] The compositor presents a solid-colour Frame at the connector's fixed refresh on H-002.
- [ ] The same path presents through virtio-gpu on H-003.
- [ ] `os inspect display` reports mode and refresh without a DRM connector id in the public Interface.
- [ ] A native application Component holds no DRM master capability.

#### Verification
- Integration: `gfx:tests/compositor/kms_output_*` on CI matrix entries `qemu-virtio-gpu` and `hw-h002`.
- Demo: compositor presents on H-002 from a SystemGeneration.

#### Evidence
- none

### GFX-009 · Implement disconnect, rebind and restore-state for compositor clients
- Type: build
- Milestone: V0.5
- Status: todo
- Size: L
- Owner: none
- Depends on: GFX-039, GFX-010, GFX-019, SDK-012, SVC-009, IPC-028
- Baseline: §32, §40
- Risks: R-023
- Invariants: I-037

§32 client contract in the UI runtime and compositor: applications continue rendering after compositor restart with no exit. Generated clients observe disconnect, re-resolve the compositor Interface and restore the state the restart-state decision named.

<!-- covers: INV-0593, INV-0745 -->

#### Out of scope
Object persistence (GFX-039). Kill-loop CI (GFX-007).

#### Acceptance criteria
- [ ] A client whose compositor Channel closes receives a typed disconnect and does not exit.
- [ ] After supervisor restart the client holds a Channel to the new compositor instance and continues presenting Frames.
- [ ] Restored state matches the accepted restart-state decision for geometry, stacking, focus and workspace.
- [ ] A client built against the previous Interface version still rebinds.

#### Verification
- Integration: `gfx:tests/compositor/rebind_protocol_*` on CI matrix entries `qemu-virtio-gpu` and `hw-h002`.
- Unit: `gfx:tests/compositor/disconnect_*` on `qemu-x86_64`.

#### Evidence
- none

### GFX-010 · Run the compositor as a privileged Component holding Display and DRM master
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: GFX-022, GFX-012, CAP-023, SVC-012
- Baseline: §32, §33, §39
- Risks: R-023

Compositor lifecycle under SVC supervision with DRM master and Display capabilities granted from the SystemGeneration, not ambient root. Native software never sees a root compositor process; `os inspect` shows a Component with those capabilities.

<!-- covers: INV-0732 -->

#### Out of scope
KMS programming (GFX-008). Restart policy (SVC-015).

#### Acceptance criteria
- [ ] The compositor starts from a SystemGeneration as a supervised Component.
- [ ] `os inspect component` on the compositor lists Display and DRM-master capabilities and no wildcard grant.
- [ ] A second Component cannot obtain DRM master while the compositor holds it.
- [ ] Supervisor restart of the compositor does not require a kernel reboot.

#### Verification
- Integration: `gfx:tests/compositor/service_component_*` on CI matrix entries `qemu-virtio-gpu` and `hw-h002`.
- Review: SVC lead records that the manifest satisfies the rebind-contract lint.

#### Evidence
- none

### GFX-011 · Write the compositor and trusted-UI section of the threat model
- Type: docs
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: SEC-002
- Baseline: §9.1, §32, §40
- Risks: R-008
- Threats: T-009, T-012, T-013, T-031

Extend SEC's V0 threat model with overlay, spoofing, capture and crash-to-unlock threats before trusted-UI and lock designs are fixed. Every later GFX adr cites the T-IDs named here.

<!-- covers: GAP-0228, INV-0210 -->

#### Out of scope
SEC threat-model chassis (SEC-002). Trusted-UI implementation (GFX-040).

#### Acceptance criteria
- [ ] The section cites T-012, T-013, T-009 and T-031 with the compositor control that addresses each.
- [ ] The section states that a compositor crash while locked must restart locked (I-075).
- [ ] The section states that an application without a capture capability receives a denied or black Surface (I-085).
- [ ] Review records that no GFX adr after this task omits its T-IDs.

#### Verification
- Review: SEC and GFX leads record sign-off on the pull request.

#### Evidence
- none

### GFX-012 · Decide compositor architecture: monolithic or split display/scene/input
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: GFX-033
- Baseline: §32, §40
- Decision: D-0076
- Risks: R-015

Record which process split the compositor uses so crash recovery and Input-to-photon latency are properties of one architecture rather than a later retrofit. The UI protocol depends on this split; §40 crash recovery and §32 rebind assume it is stable before clients exist.

<!-- covers: INV-0752, GAP-0513 -->

#### Out of scope
DRM/KMS plumbing reuse (GFX-013). Shell chrome (APP).

#### Acceptance criteria
- [ ] The decision evaluates monolithic display server plus shell, split display server / window manager / shell, and display-plus-input core with an out-of-process shell.
- [ ] The accepted option states which process holds DRM master and Display capabilities.
- [ ] The accepted option states how a crash of each process is observed by clients and by the supervisor.
- [ ] Review records the accepted option on the pull request.

#### Verification
- Review: GFX and SVC leads record the accepted option on the pull request.

#### Evidence
- none

### GFX-013 · Decide compositor infrastructure reuse versus build-anew
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: GFX-032
- Baseline: §39, §40, §2
- Decision: D-0077

Decide which existing compositor infrastructure, if any, is vendored for DRM/KMS plumbing. The native object model is not Wayland; reuse is of plumbing, not of the native UI API (I-048).

<!-- covers: INV-0753 -->

#### Out of scope
Process architecture (GFX-012). Wayland serving model (GFX-020).

#### Acceptance criteria
- [ ] The decision evaluates Smithay crates, wlroots via FFI, a Mutter or KWin fork, and greenfield Rust.
- [ ] The accepted option states which DRM/KMS, GBM and input objects are vendored and which are rewritten.
- [ ] The accepted option states that native applications do not speak Wayland (I-048).
- [ ] Review records the accepted option on the pull request.

#### Verification
- Review: GFX lead records the accepted option on the pull request.

#### Evidence
- none

### GFX-014 · Decide explicit GPU synchronisation as the only path for native Surfaces
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: GFX-036
- Baseline: §39, §40
- Decision: D-0078
- Invariants: I-084

Standing invariant decided once: timeline semaphores / drm_syncobj are mandatory for native Surfaces, with no implicit-sync path. Retrofitting explicit sync later is the pain Wayland went through; the native protocol does not repeat it.

<!-- covers: GAP-0514 -->

#### Out of scope
Frame object implementation (GFX-023). Personality implicit-sync bridges (LNX, WIN).

#### Acceptance criteria
- [ ] The decision evaluates explicit-only, explicit with an implicit bridge for personalities, and implicit default.
- [ ] The accepted option states whether a native Surface commit without a timeline semaphore returns `Error::Rights`.
- [ ] The accepted option names how personalities import implicit-sync buffers, if at all.
- [ ] Review records the accepted option on the pull request.

#### Verification
- Review: GFX and ABI leads record the accepted option on the pull request.

#### Evidence
- none

### GFX-015 · Decide the compositor frame scheduling model
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: GFX-012
- Baseline: §40, §22
- Decision: D-0079

V0.5 exit requires an accepted decision for compositor frame scheduling so client frame callbacks align to display deadlines. Scheduling intent of the compositor Task is SCH; this decision is the compositor's callback model.

<!-- covers: INV-0748 -->

#### Out of scope
Callback implementation (GFX-024). Deadline intent of the compositor Task (SCH-015).

#### Acceptance criteria
- [ ] The decision evaluates fixed vblank-aligned callbacks, deadline-scheduled callbacks with per-client budgets, and client-driven presentation timing.
- [ ] The accepted option states how a late client is observed (dropped frame, overcommit, or wait).
- [ ] The accepted option states that public Interfaces name Frames and deadlines, not DRM vblank ioctls.
- [ ] Review records the accepted option on the pull request.

#### Verification
- Review: GFX and SCH leads record the accepted option on the pull request.

#### Evidence
- none

### GFX-016 · Decide the GPU userspace strategy from the Mesa-behind-capabilities Spike
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: GFX-036, Q-034
- Baseline: §39, §9.1, §56.1
- Decision: D-0080
- Risks: R-016

One decision: how Mesa is hosted so native Components render without ambient DRM device-node access. Q-034 closes when this decision is accepted.

<!-- covers: GAP-0511, INV-0734 -->

#### Out of scope
Native GPU API choice (GFX-017). Allocation broker implementation (GFX-051).

#### Acceptance criteria
- [ ] The decision evaluates unmodified Mesa with a brokered descriptor, a patched Mesa WSI layer, and Mesa inside a Linux personality helper.
- [ ] The accepted option states which Component holds the DRM render node and that native applications do not.
- [ ] The accepted option states how Vulkan WSI maps onto Surface and Buffer.
- [ ] Review records the accepted option on the pull request.

#### Verification
- Review: GFX and CAP leads record the accepted option on the pull request.

#### Evidence
- none

### GFX-017 · Decide the GPU API native applications render with
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: GFX-036
- Baseline: §39, §50
- Decision: D-0082

Decide the GPU API native applications render with, before RenderQueue implementation and SDK v1. The API is a native Interface over RenderQueue, not a Linux syscall wrapper (I-005).

<!-- covers: INV-0733 -->

#### Out of scope
RenderQueue object (GFX-030). Mesa hosting (GFX-016).

#### Acceptance criteria
- [ ] The decision evaluates Vulkan via Mesa inside the Component, a WebGPU-like native API over RenderQueue, and both with Vulkan as the escape hatch.
- [ ] The accepted option states the IDL types native applications call.
- [ ] The accepted option states that DRM ioctls are not part of the native API.
- [ ] Review records the accepted option on the pull request.

#### Verification
- Review: GFX and SDK leads record the accepted option on the pull request.

#### Evidence
- none

### GFX-018 · Decide compositor rendering backend policy: Vulkan-only or Vulkan plus GL
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: GFX-035, HW-003
- Baseline: §40, §62, §56.1
- Decision: D-0086

Evaluate the compositor rendering backend against the V2 target machine list so older laptops are not excluded silently. A single backend reduces maintenance; a GL fallback may be required for a listed SKU.

<!-- covers: GAP-0515 -->

#### Out of scope
Compositor GPU composition (GFX-006). Target SKU list (HW-003).

#### Acceptance criteria
- [ ] The decision evaluates Vulkan-only, Vulkan plus GL fallback, and Vulkan plus software fallback only.
- [ ] The accepted option names which V2 Reference machines the backend supports.
- [ ] The accepted option states the headless software path used on H-001 and H-003.
- [ ] Review records the accepted option on the pull request.

#### Verification
- Review: GFX and HW leads record the accepted option on the pull request.

#### Evidence
- none

### GFX-019 · Decide which compositor state survives restart and where it is checkpointed
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: GFX-033, Q-027
- Baseline: §32, §40
- Decision: D-0087

Answers the §32 question for geometry, stacking, focus and workspace. Q-027 closes when this decision is accepted. Object persistence (kernel versus broker) is the mechanism this decision picks.

<!-- covers: INV-0606 -->

#### Out of scope
Persistence implementation (GFX-039). Window objects (GFX-042).

#### Acceptance criteria
- [ ] The decision evaluates kernel-owned window objects, a persistent broker Component, and client-replayed state.
- [ ] The accepted option lists which of geometry, stacking, focus and workspace are restored.
- [ ] The accepted option states where the checkpoint lives and how a crash during checkpoint is observed.
- [ ] Review records the accepted option on the pull request.

#### Verification
- Review: GFX and ABI leads record the accepted option on the pull request.

#### Evidence
- none

### GFX-020 · Decide whether Wayland is served by the compositor or by a bridge Component
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: GFX-012
- Baseline: §40, §41, §47
- Decision: D-0088
- Risks: R-020
- Invariants: I-048

Determines whether LNX's Wayland bridge translates into the native UI protocol or the compositor speaks Wayland directly. Native applications never speak Wayland. Must land before the V0.5 Wayland application gate.

<!-- covers: INV-0887 -->

#### Out of scope
Wayland bridge implementation (LNX-006). Native UI protocol (UIP).

#### Acceptance criteria
- [ ] The decision evaluates compositor-as-Wayland-server, a bridge Component translating Wayland into the native UI protocol, and nested Wayland inside the Linux personality only.
- [ ] The accepted option states that native applications do not speak Wayland (I-048).
- [ ] The accepted option states where X11 primary selection is confined (T-032).
- [ ] Review records the accepted option on the pull request.

#### Verification
- Review: GFX and LNX leads record the accepted option on the pull request.

#### Evidence
- none

### GFX-021 · Demonstrate compositor crash, restart, rebind and continued rendering
- Type: build
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: GFX-007, APP-003
- Baseline: §32, §60

§60 service restart/rebind demo: kill the compositor mid-session; all windows come back and the Text Editor keeps its unsaved buffer.

<!-- covers: INV-0604 -->

#### Out of scope
Kill-loop CI (GFX-007). Text Editor application (APP-003).

#### Acceptance criteria
- [ ] On H-002, killing the compositor with the Text Editor open restores every window without the editor process exiting.
- [ ] The restored Text Editor window contains the unsaved buffer that was present before the kill.
- [ ] The procedure is recorded as V0.5-D03.

#### Verification
- Demo: compositor kill keeps the editor buffer on H-002.
- Integration: `gfx:tests/compositor/demo_restart_*` on CI matrix entries `qemu-virtio-gpu` and `hw-h002`.

#### Evidence
- none

### GFX-022 · Define and implement Display with mode, refresh, scaling, HDR and VRR metadata
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: GFX-018
- Baseline: §39, §40, §62

Single display at fixed refresh is exercised in V0.5. Mode, refresh, scaling, HDR and VRR metadata fields are defined now so V2 HDR and VRR do not change the Interface shape.

<!-- covers: INV-0730 -->

#### Out of scope
KMS scanout (GFX-008). HDR output pipeline (GFX-068). VRR enablement (GFX-088).

#### Acceptance criteria
- [ ] `os inspect display` reports mode, refresh, scale, HDR and VRR capability fields on H-002 and H-003.
- [ ] Setting an unsupported mode returns a typed error and leaves the previous mode.
- [ ] Public Display IDL contains no DRM connector, CRTC or encoder identifiers.
- [ ] HDR and VRR fields are present and readable when the hardware does not support them (value false or none).

#### Verification
- Unit: `gfx:tests/display/metadata_*` on CI matrix entries `qemu-virtio-gpu` and `hw-h002`.
- Integration: `gfx:tests/display/inspect_*` on `hw-h002`.

#### Evidence
- none

### GFX-023 · Define and implement Frame with explicit fences and timelines for presentation
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: GFX-014, GFX-005
- Baseline: §39, §40
- Invariants: I-084

Presentation unit carrying timeline semaphores. Implements the explicit-sync-only decision from the first compositor; a native Frame without a timeline is rejected.

<!-- covers: INV-0731, GAP-0514 -->

#### Out of scope
Frame callbacks (GFX-024). Personality implicit-sync import (LNX, WIN).

#### Acceptance criteria
- [ ] Committing a Frame without a timeline semaphore returns `Error::Rights` and presents nothing.
- [ ] `os inspect frame` reports the timeline wait point without a DRM syncobj file descriptor.
- [ ] Two Frames in flight on one Surface complete in timeline order.
- [ ] Public Frame IDL contains no implicit-sync flag.

#### Verification
- Unit: `gfx:tests/frame/explicit_sync_*` on CI matrix entries `qemu-virtio-gpu` and `hw-h002`.
- Fuzz: `gfx:fuzz/frame_timeline` one hour nightly without panic.

#### Evidence
- none

### GFX-024 · Implement client frame callbacks aligned to display deadlines
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: GFX-015, GFX-006, GFX-023, SCH-015
- Baseline: §40, §22

Implements the frame-scheduling decision. Required for the animating-at-refresh V0.5 exit criterion. The compositor Task runs under Deadline intent aligned to vblank (SCH).

<!-- covers: INV-0748 -->

#### Out of scope
Latency-aware late present (GFX-054). Deadline intent class (SCH).

#### Acceptance criteria
- [ ] A client that waits on the frame callback presents on the next display deadline on H-002.
- [ ] An animating Surface is presented at the display's fixed refresh for a 60 s capture on H-002.
- [ ] A client that misses its deadline is observed as the accepted scheduling option specifies.
- [ ] Public callbacks name Frame deadlines, not DRM vblank event file descriptors.

#### Verification
- Integration: `gfx:tests/compositor/frame_callbacks_*` on CI matrix entries `qemu-virtio-gpu` and `hw-h002`.
- Bench: B-018 on H-002; target per register.

#### Evidence
- none

### GFX-025 · Build golden-image comparison tolerant of documented rasterisation differences
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: GFX-041, GFX-006, BLD-018
- Baseline: §40
- Risks: R-015

Headless desktop testing needs per-backend tolerance rules so virtio-gpu, software and AMD renders compare against one golden set. Pixel goldens stay reserved for compositor rendering; application GUI tests use the semantic harness (BLD).

<!-- covers: GAP-0119 -->

#### Out of scope
Semantic GUI harness (BLD-027). Virtio-gpu compositor path (GFX-041).

#### Acceptance criteria
- [ ] A documented tolerance table exists per backend (software, virtio-gpu, amdgpu).
- [ ] A compositor scene that matches the golden within tolerance passes on H-003 and H-002.
- [ ] A compositor scene that differs by a moved window fails on every backend.
- [ ] The harness is a CI job on compositor rendering changes.

#### Verification
- Integration: `gfx:tests/golden/compare_*` on CI matrix entries `qemu-virtio-gpu` and `hw-h002`.
- Review: BLD lead records that application tests do not use this pixel harness.

#### Evidence
- none

### GFX-026 · Grant Capability<RenderQueue> at Component creation without other surfaces
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: GFX-030, GFX-038
- Baseline: §9.1, §39
- Threats: T-001
- Invariants: I-021

§9.1 PhotoEditor example: a launched application receives render queues only; screen contents and other Surfaces stay unreachable. GPU access is `Capability<RenderQueue>` granted at Component creation.

<!-- covers: INV-0214, INV-0735 -->

#### Out of scope
RenderQueue object (GFX-030). Isolation denial test (GFX-037).

#### Acceptance criteria
- [ ] A Component launched with UI and GPU grants holds `Capability<RenderQueue>` and no capture, Display or foreign Surface capability.
- [ ] Opening another Component's Surface returns `Error::Rights` and allocates no handle.
- [ ] `os inspect component` lists the granted RenderQueue and no screen-contents grant.
- [ ] Image Viewer launch in CI uses this grant set.

#### Verification
- Integration: `gfx:tests/cap/launch_gpu_only_*` on CI matrix entries `qemu-virtio-gpu` and `hw-h002`.
- Unit: `gfx:tests/cap/renderqueue_grant_*` on `qemu-x86_64`.

#### Evidence
- none

### GFX-027 · Run the retained GPU driver regression matrix on virtio-gpu and the AMD desktop
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: GFX-001, GFX-008, KRN-014, BLD-007
- Baseline: §39, §55, §56.1
- Risks: R-013
- Invariants: I-054, I-098

Second of the two permitted retain-X tasks: kernel DRM regression matrix (amdgpu, i915/xe, nouveau, virtio-gpu) run per kernel rebase, extended per milestone as machines arrive. A native graphics change that regresses a retained DRM kselftest is rejected.

<!-- covers: INV-0022, INV-0067, INV-0723, INV-0724 -->

#### Out of scope
Retained-mechanism inventory (GFX-001). Kernel DRM code (KRN).

#### Acceptance criteria
- [ ] DRM kselftests for amdgpu and virtio-gpu run on every kernel rebase on H-002 and H-003.
- [ ] A failing retained DRM kselftest blocks the compositor change that caused it.
- [ ] The matrix document lists which tests are enabled per Reference machine.
- [ ] i915/xe and nouveau rows exist as skip-until-hardware until those machines are in scope.

#### Verification
- Integration: `gfx:tests/drm/regression_matrix_*` on CI matrix entries `qemu-virtio-gpu` and `hw-h002`.
- Review: KRN lead records that the matrix is a row of KRN-014.

#### Evidence
- none

### GFX-028 · Add CI lint forbidding DRM ioctls, device nodes and fourcc in public graphics IDL
- Type: build
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: GFX-038, GFX-005, GFX-030, GFX-022, GFX-023, GFX-014
- Baseline: §39, §40
- Invariants: I-045, I-084

Enforces the §39 invariant and the explicit-sync-only rule as a review gate over the IDL rather than one task per rule. Public graphics Interfaces are S-024.

<!-- covers: INV-0725, GAP-0514 -->

#### Out of scope
Surface implementation (GFX-038). ABI POSIX-shape lint (ABI).

#### Acceptance criteria
- [ ] CI fails a public graphics IDL file that names a DRM ioctl, `/dev/dri` path, drm_fourcc fourcc or implicit-sync flag.
- [ ] The lint allowlist is empty for native graphics Interfaces.
- [ ] A fixture IDL containing a DRM ioctl is rejected in CI.
- [ ] The lint runs on every compositor and graphics IDL change.

#### Verification
- Unit: `gfx:tests/lint/idl_no_drm_*` on CI matrix entry `qemu-x86_64`.
- Review: ABI lead records that S-024 stays free of DRM identifiers.

#### Evidence
- none

### GFX-029 · Test zero dropped frames over a 60 s idle desktop capture
- Type: build
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: GFX-006, GFX-008
- Baseline: §40, §54

V0.5 exit criterion verified by capture-card or KMS vblank counters on the reference desktop. Damage tracking from GPU composition is what makes an idle desktop drop no frames.

#### Out of scope
Deadline-miss benchmark under load (GFX-060). GPU composition (GFX-006).

#### Acceptance criteria
- [ ] KMS vblank counters record no missed deadlines during a 60 s idle capture on H-002.
- [ ] The same capture on H-003 via virtio-gpu records no missed deadlines.
- [ ] Introducing a forced full-scene damage every refresh fails the test.
- [ ] The test is a required CI job on compositor scheduling changes.

#### Verification
- Integration: `gfx:tests/compositor/idle_vblank_*` on CI matrix entries `qemu-virtio-gpu` and `hw-h002`.

#### Evidence
- none

### GFX-030 · Define and implement the RenderQueue abstraction
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: GFX-017, GFX-016, GFX-005
- Baseline: §39, §9.1

Capability-scoped render submission object per §39. Native applications submit work through RenderQueue, never through a DRM render node.

<!-- covers: INV-0728 -->

#### Out of scope
ComputeQueue (GFX-046). Mesa-in-Component (GFX-056).

#### Acceptance criteria
- [ ] Submitting work on a RenderQueue without the matching capability returns `Error::Rights` and enqueues nothing.
- [ ] `os inspect renderqueue` reports owner Component and outstanding submissions without a DRM render-node path.
- [ ] A Frame wait on a RenderQueue submission uses the explicit timeline from GFX-023.
- [ ] Public RenderQueue IDL matches the accepted native GPU API decision.

#### Verification
- Unit: `gfx:tests/renderqueue/submit_*` on CI matrix entries `qemu-virtio-gpu` and `hw-h002`.
- Integration: `gfx:tests/renderqueue/inspect_*` on `qemu-virtio-gpu`.

#### Evidence
- none

### GFX-031 · Dispatch seat input from the HW input service to the focused Surface
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: GFX-042, HW-011, UIP-012, UIP-005
- Baseline: §40, §41

V0.5 exit: a native application receives keyboard and mouse input. HW owns devices and UIP owns the input protocol; this task is the compositor's focus and hit-test dispatch plus software cursor.

#### Out of scope
HID service (HW-011). Input protocol (UIP-012). Hardware cursor plane (GFX-067).

#### Acceptance criteria
- [ ] A focused native Surface receives keyboard and pointer events on H-002 and H-003.
- [ ] An unfocused Surface observes no input events (tested).
- [ ] A software cursor is presented and tracks pointer motion.
- [ ] Native applications do not open `/dev/input` device nodes.

#### Verification
- Integration: `gfx:tests/seat/dispatch_*` on CI matrix entries `qemu-virtio-gpu` and `hw-h002`.
- Demo: a native application types and clicks on H-002.

#### Evidence
- none

### GFX-032 · Evaluate Smithay, wlroots, Mutter and KWin for DRM/KMS plumbing reuse
- Type: spike
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: GFX-035, GFX-001
- Baseline: §39, §40, §2
- Explores: S-024

Prototype-and-measure study of reusable DRM/KMS, GBM and input plumbing versus building anew. Produces the option matrix the reuse adr needs. Native applications still do not speak Wayland.

<!-- covers: INV-0753 -->

#### Out of scope
Architecture split (GFX-033). Reuse decision (GFX-013).

#### Acceptance criteria
- [ ] The report measures DRM/KMS atomic-commit bring-up cost for Smithay crates, wlroots via FFI, Mutter, KWin and greenfield Rust on H-002 or H-003.
- [ ] The report lists which objects each option would expose to native applications, if any.
- [ ] The report records licence and GPL-boundary consequences of each option.
- [ ] The report is committed at `reports/spikes/GFX-032.md`.

#### Verification
- Report: which plumbing is reusable without making Wayland the native UI API; licence boundary; bring-up cost per option; what must be rewritten to speak Surface, Buffer, Display and Frame.
- Review: GOV lead records that the licence table is complete.

#### Evidence
- none

### GFX-033 · Prototype monolithic versus split compositor and measure crash-rebind and latency
- Type: spike
- Milestone: V0.5
- Status: todo
- Size: L
- Owner: none
- Depends on: GFX-032, LAB-003
- Baseline: §32, §40, §54
- Benchmarks: B-020, B-023
- Explores: S-024
- Risks: R-015

Crash recovery and latency pull in opposite directions. Prototype each candidate split and measure both before the UI protocol depends on the architecture.

<!-- covers: GAP-0513 -->

#### Out of scope
Architecture decision (GFX-012). Production compositor (GFX-006).

#### Acceptance criteria
- [ ] Prototypes exist for monolithic, split display/scene/input, and display-plus-input core with out-of-process shell.
- [ ] Each prototype has a B-023 measurement on H-003 and a B-020 measurement on H-002.
- [ ] The report names which process holds DRM master in each prototype.
- [ ] The report is committed at `reports/spikes/GFX-033.md`.

#### Verification
- Report: crash-rebind and Input-to-photon latency per split; which DRM/KMS objects each process holds; which split the architecture adr prefers and why.
- Bench: B-020 on H-002 and B-023 on H-003 for each prototype; targets per register.

#### Evidence
- none

### GFX-034 · Spike dma-buf interoperability between MemoryObject and DRM drivers
- Type: spike
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: MEM-019, MEM-005, GFX-001
- Baseline: §16, §17, §39

Measure whether a GPU-compatible MemoryObject can be imported by amdgpu, i915 and virtio-gpu as the same physical pages. The result gates the zero-copy import build.

<!-- covers: INV-0337 -->

#### Out of scope
Buffer object (GFX-005). Zero-copy import (GFX-044).

#### Acceptance criteria
- [ ] The prototype imports a MemoryObject into amdgpu or virtio-gpu and records physical-page identity.
- [ ] The report states whether a copy is required on each of amdgpu, i915 and virtio-gpu.
- [ ] Native applications in the prototype do not hold dma-buf file descriptors.
- [ ] The report is committed at `reports/spikes/GFX-034.md`.

#### Verification
- Report: whether GPU import of a MemoryObject is the same physical pages on amdgpu, i915 and virtio-gpu; what MEM backing decision that implies; what the zero-copy build may assume.
- Review: MEM lead records that the page-identity method matches MEM tests.

#### Evidence
- none

### GFX-035 · Assess GPU driver risk and mitigation plan for AMD, Intel and NVIDIA
- Type: spike
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: GFX-001, HW-003
- Baseline: §56.1, §55, §57
- Risks: R-016, R-037, R-050

§56.1 names GPU drivers as the hardest hardware problem. This report-only spike records why, and the mitigation plan for AMD, Intel and NVIDIA, before any V0.5 graphics spike commits to a userspace or compositor path. No code lands.

<!-- covers: INV-1059 -->

#### Out of scope
Mesa-behind-capabilities prototype (GFX-036). NVIDIA 1.0 stance (GFX-064).

#### Acceptance criteria
- [ ] The report names the dominant failure mode per vendor (AMD, Intel, NVIDIA) against capability-mediated GPU access.
- [ ] The report names which Reference machines exercise each vendor at V0, V0.5, V1, V2 and V3.
- [ ] The report lists mitigations that do not require a native GPU driver rewrite (I-045).
- [ ] The report is committed at `reports/spikes/GFX-035.md`.

#### Verification
- Report: why GPU is the hardest hardware problem; the AMD, Intel and NVIDIA mitigation plan; which V0.5 spikes are allowed to assume which vendor path; what remains a 1.0 non-promise.
- Review: GFX and HW leads record that the plan does not contradict I-045.

#### Evidence
- none

### GFX-036 · Spike Mesa/Vulkan inside a native Component without ambient /dev/dri access
- Type: spike
- Milestone: V0.5
- Status: todo
- Size: L
- Owner: none
- Depends on: GFX-035, GFX-034, CAP-005
- Baseline: §39, §9.1, §56.1
- Explores: S-024
- Risks: R-016

Prove Surface, Buffer and RenderQueue can map onto DRM render nodes, GBM and Vulkan WSI with the DRM descriptor held by a broker, not the application. The V0.5 risk register names Mesa's descriptor assumptions as the top graphics risk.

<!-- covers: GAP-0511, INV-0734, INV-1065 -->

#### Out of scope
GPU userspace decision (GFX-016). Mesa-in-Component production path (GFX-056).

#### Acceptance criteria
- [ ] A native Component renders a Vulkan triangle without opening `/dev/dri`.
- [ ] The broker Component is the only holder of the DRM render-node capability.
- [ ] The report names which Mesa WSI entry points required a patch, a broker, or a personality helper.
- [ ] The report is committed at `reports/spikes/GFX-036.md`.

#### Verification
- Report: whether unmodified Mesa, a patched WSI layer, or a Linux personality helper is viable; which DRM objects the broker must hold; what Q-034 records.
- Integration: `gfx:tests/spike/mesa_no_dev_dri_*` on CI matrix entries `qemu-virtio-gpu` and `hw-h002`.

#### Evidence
- none

### GFX-037 · Test that a UI+GPU Component cannot read other Surfaces or the framebuffer
- Type: build
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: GFX-026, GFX-038, GFX-006
- Baseline: §9.1, §39
- Threats: T-001, T-013
- Invariants: I-021, I-085

Enforces the §9.1 no-screen-contents invariant with a denial test rather than a standalone rule task. A UI+GPU Component cannot read other Surfaces or the framebuffer.

<!-- covers: INV-0210, INV-0725 -->

#### Out of scope
Capture capability (GFX-061). Launch grants (GFX-026).

#### Acceptance criteria
- [ ] Mapping another Component's Surface returns `Error::Rights` and copies no pixels.
- [ ] Mapping the scanout framebuffer from a UI+GPU Component returns `Error::Rights`.
- [ ] The denial is visible in the capability audit log.
- [ ] The test is a required CI job on graphics capability changes.

#### Verification
- Integration: `gfx:tests/cap/surface_isolation_*` on CI matrix entries `qemu-virtio-gpu` and `hw-h002`.
- Unit: `gfx:tests/cap/framebuffer_deny_*` on `qemu-virtio-gpu`.

#### Evidence
- none

### GFX-038 · Define and implement Object<Surface> with typed interfaces
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: GFX-017, GFX-014
- Baseline: §7, §39, §40

Core §39 abstraction. Every window, chooser and bridge Surface is built on it. Public Interfaces are S-024.

<!-- covers: INV-0162, INV-0726 -->

#### Out of scope
Window policy (GFX-042). Buffer backing (GFX-005).

#### Acceptance criteria
- [ ] Creating a Surface without the matching capability returns `Error::Rights` and allocates no handle.
- [ ] `os inspect surface` reports owner, size and attached Buffer without a DRM framebuffer id.
- [ ] Attaching a Buffer from another Component without a transfer returns `Error::Rights`.
- [ ] Public Surface IDL contains no Wayland protocol name.

#### Verification
- Unit: `gfx:tests/surface/create_*` on CI matrix entries `qemu-virtio-gpu` and `hw-h002`.
- Integration: `gfx:tests/surface/inspect_*` on `qemu-virtio-gpu`.

#### Evidence
- none

### GFX-039 · Keep Surface and window objects alive across compositor restart
- Type: build
- Milestone: V0.5
- Status: todo
- Size: L
- Owner: none
- Depends on: GFX-019, GFX-042, SVC-009
- Baseline: §32, §40
- Invariants: I-037

Implements the restart-state-checkpoint decision: objects owned by the kernel or a persistent broker so the restarted compositor re-attaches them. Applications do not recreate their Surfaces on compositor crash.

<!-- covers: INV-0605 -->

#### Out of scope
Rebind protocol (GFX-009). Restart-state decision (GFX-019).

#### Acceptance criteria
- [ ] After compositor kill, `os inspect` still lists the pre-crash Surface and window objects.
- [ ] The restarted compositor re-attaches those objects without the client allocating a new Surface.
- [ ] Object identity observed by the client is unchanged across the restart.
- [ ] Implementation matches the accepted restart-state decision (kernel-owned, broker, or client replay).

#### Verification
- Integration: `gfx:tests/compositor/persist_objects_*` on CI matrix entries `qemu-virtio-gpu` and `hw-h002`.
- Unit: `gfx:tests/compositor/object_identity_*` on `qemu-virtio-gpu`.

#### Evidence
- none

### GFX-040 · Render choosers, prompts and elevation in compositor-protected trusted Surfaces
- Type: build
- Milestone: V0.5
- Status: todo
- Size: L
- Owner: none
- Depends on: GFX-038, GFX-006, GFX-031, GFX-011
- Baseline: §9.1, §25, §40
- Threats: T-012
- Invariants: I-035

The V0.5 UserSelected chooser grants authority; applications must not overlay, spoof, resize or inject input into it. Chooser chrome is APP; GFX enforces compositor-protected Surfaces.

<!-- covers: GAP-0228 -->

#### Out of scope
Chooser chrome (APP-002). UserSelected authority minting (STO).

#### Acceptance criteria
- [ ] A client Surface cannot stack above a trusted Surface (tested).
- [ ] Pointer and keyboard events targeting a trusted Surface are not delivered to the requesting application.
- [ ] Resizing or moving a trusted Surface from an application Channel returns `Error::Rights`.
- [ ] A screenshot of a trusted Surface from an application without capture rights is denied or black.

#### Verification
- Integration: `gfx:tests/trusted/no_overlay_*` on CI matrix entries `qemu-virtio-gpu` and `hw-h002`.
- Integration: `gfx:tests/trusted/no_input_inject_*` on `qemu-virtio-gpu`.

#### Evidence
- none

### GFX-041 · Provide software-rendered and virtio-gpu compositor paths for QEMU CI
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: GFX-022, GFX-018, BLD-028
- Baseline: §39, §40
- Risks: R-015

V0.5 hardware scope names virtio-gpu for compositor CI. Without a GPU-less rendering path every UI change needs physical hardware.

<!-- covers: GAP-0119 -->

#### Out of scope
Golden-image comparison (GFX-025). QEMU matrix plumbing (BLD-028).

#### Acceptance criteria
- [ ] The compositor presents on H-003 through virtio-gpu.
- [ ] The compositor presents on H-001 through the software backend.
- [ ] Both paths implement the same Surface, Buffer, Display and Frame Interfaces as H-002.
- [ ] Compositor CI jobs on `qemu-virtio-gpu` do not require H-002.

#### Verification
- Integration: `gfx:tests/compositor/virtio_gpu_*` on CI matrix entry `qemu-virtio-gpu`.
- Integration: `gfx:tests/compositor/sw_rast_*` on CI matrix entry `qemu-x86_64`.

#### Evidence
- none

### GFX-042 · Define window objects as Capability<Surface> handles over typed interfaces
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: GFX-038, GFX-019
- Baseline: §40, §60

§60 window objects created and managed through typed Interfaces, not a Wayland-shaped protocol. A window is a Capability to a Surface plus the geometry and stacking the shell policy Interface consumes.

<!-- covers: INV-0757, INV-1179 -->

#### Out of scope
Window-management policy Interface (GFX-043). Surface object (GFX-038).

#### Acceptance criteria
- [ ] Creating a window returns `Capability<Surface>` plus a window handle inspectable via `os inspect window`.
- [ ] Destroying the window releases the Surface; a subsequent present returns a typed error.
- [ ] Public window IDL contains no Wayland xdg-shell name.
- [ ] Window objects survive compositor restart as the restart-state decision specifies.

#### Verification
- Unit: `gfx:tests/window/create_*` on CI matrix entries `qemu-virtio-gpu` and `hw-h002`.
- Integration: `gfx:tests/window/inspect_*` on `qemu-virtio-gpu`.

#### Evidence
- none

### GFX-043 · Define the window management policy Interface between compositor and shell
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: GFX-042, GFX-012
- Baseline: §40, §60

Typed Interface for placement, stacking, focus and workspace policy so APP's shell can own policy without DRM master. The compositor enforces; the shell decides.

<!-- covers: INV-0757 -->

#### Out of scope
Shell chrome (APP). DRM master (GFX-010).

#### Acceptance criteria
- [ ] A shell Component without DRM master can set placement, stacking, focus and workspace through the policy Interface.
- [ ] A non-shell Component calling the policy Interface returns `Error::Rights`.
- [ ] Focus changes are visible in `os inspect` and match seat dispatch.
- [ ] Public policy IDL contains no DRM or Wayland identifier.

#### Verification
- Integration: `gfx:tests/wm/policy_interface_*` on CI matrix entries `qemu-virtio-gpu` and `hw-h002`.
- Unit: `gfx:tests/wm/rights_*` on `qemu-x86_64`.

#### Evidence
- none

### GFX-044 · Import a MemoryObject into the GPU driver without copying
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: GFX-034, GFX-005, MEM-024
- Baseline: §16, §17, §39
- Invariants: I-063

Renderer hands a MemoryObject to the GPU as the same physical pages via dma-buf. Verified by page-identity test. Native applications never hold the dma-buf descriptor.

<!-- covers: INV-0325, INV-0337 -->

#### Out of scope
dma-buf spike (GFX-034). MEM allocation (MEM-024).

#### Acceptance criteria
- [ ] Importing a GPU-compatible MemoryObject into the GPU driver preserves physical-page identity on H-002 and H-003.
- [ ] A MemoryObject that is not GPU-compatible returns a typed error and copies nothing.
- [ ] The importing Component is not granted a dma-buf file descriptor.
- [ ] `os inspect` reports GPU-imported without a copy flag.

#### Verification
- Integration: `gfx:tests/buffer/page_identity_*` on CI matrix entries `qemu-virtio-gpu` and `hw-h002`.
- Unit: `gfx:tests/buffer/import_rights_*` on `qemu-virtio-gpu`.

#### Evidence
- none

### GFX-045 · Implement compositor-owned lock mode inhibiting input and overlay
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: GFX-040, GFX-009, GFX-031
- Baseline: §32, §40, §61
- Threats: T-009
- Invariants: I-075

V1 session lock: APP owns lock UI later; GFX enforces input inhibition, notification hiding policy and the no-overlay guarantee on idle, lid close and suspend. A compositor crash while locked restarts locked.

<!-- covers: GAP-0216 -->

#### Out of scope
Lock-screen chrome (APP-033). Crash-while-locked test (GFX-055). Identity (SEC).

#### Acceptance criteria
- [ ] Entering lock mode inhibits all input to application Surfaces.
- [ ] No application Surface can stack above the lock Surface (tested).
- [ ] Notification content is hidden according to the lock policy.
- [ ] Lock mode is entered on idle timeout, lid close and suspend signals from HW/PWR.

#### Verification
- Integration: `gfx:tests/lock/inhibit_*` on CI matrix entries `qemu-virtio-gpu` and `hw-h004`.
- Integration: `gfx:tests/lock/no_overlay_*` on `hw-h002`.

#### Evidence
- none

### GFX-046 · Define and implement the ComputeQueue abstraction shared with HET
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: GFX-030, GFX-056, HET-003
- Baseline: §37, §39

§39/§37 shared object. V1 Mesa-in-Component needs compute submission; HET's V2 ComputeDevice demo consumes it. A ComputeQueue is an Object like any other, inspectable through `os inspect`.

<!-- covers: INV-0729 -->

#### Out of scope
ComputeDevice taxonomy (HET). RenderQueue (GFX-030).

#### Acceptance criteria
- [ ] Submitting compute work without `Capability<ComputeQueue>` returns `Error::Rights` and enqueues nothing.
- [ ] `os inspect computequeue` reports owner and outstanding submissions without a DRM render-node path.
- [ ] A ComputeQueue derived from a RenderQueue capability cannot present Frames.
- [ ] Public ComputeQueue IDL matches the accepted HET GPU-backend decision.

#### Verification
- Unit: `gfx:tests/computequeue/submit_*` on CI matrix entries `qemu-virtio-gpu` and `hw-h002`.
- Integration: `gfx:tests/computequeue/inspect_*` on `hw-h002`.

#### Evidence
- none

### GFX-047 · Decide the proprietary GPU kernel driver policy
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: KRN-027, GFX-035
- Baseline: §51, §56.1, §57
- Decision: D-0084
- Risks: R-050
- Invariants: I-067

Covers GPL-only symbol exposure, tainting and support commitments for proprietary GPU kernel modules versus open kernel modules, Nouveau and NVK. Coordinates with HW's NVIDIA Secure Boot decision without duplicating it.

<!-- covers: GAP-0016 -->

#### Out of scope
NVIDIA 1.0 userspace stance (GFX-064). Module signing under Secure Boot (KRN-027). NVIDIA SKU bring-up (HW).

#### Acceptance criteria
- [ ] The decision evaluates open kernel modules plus NVK, proprietary module tolerated with taint and no support, and unsupported.
- [ ] The accepted option states whether GPL-only native symbols are exported to out-of-tree GPU modules.
- [ ] The accepted option states the support commitment when the module taints the kernel.
- [ ] Review records the accepted option on the pull request.

#### Verification
- Review: GFX, KRN and GOV leads record the accepted option on the pull request.

#### Evidence
- none

### GFX-048 · Move display management policy into a user-space service
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: GFX-008, GFX-022, SVC-015
- Baseline: §33, §40

Mode selection, hot-plug policy and per-display preferences live outside the compositor core so V2 multi-monitor policy can restart independently. The compositor keeps DRM master; the policy service holds Display capabilities.

<!-- covers: INV-0620 -->

#### Out of scope
Compositor KMS commits (GFX-008). Multi-monitor hot-plug (GFX-077). Shader cache broker (GFX-051).

#### Acceptance criteria
- [ ] Mode selection and connector policy are served by a supervised Component distinct from the compositor.
- [ ] Killing the policy service does not tear down existing scanout; clients receive a typed degraded notice.
- [ ] `os inspect` lists the policy service separately from the compositor.
- [ ] The policy Interface contains no DRM connector identifiers.

#### Verification
- Integration: `gfx:tests/display/policy_service_*` on CI matrix entries `qemu-virtio-gpu` and `hw-h002`.
- Integration: `gfx:tests/display/policy_kill_*` on `hw-h002`.

#### Evidence
- none

### GFX-049 · Implement fractional scaling in the compositor
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: GFX-006, GFX-022, GFX-053
- Baseline: §40, §61, §62

V1 Intel laptop with a HiDPI internal display. L2 integration check scores correct scaling. Per-display scale factors wait for V2.

<!-- covers: INV-0742 -->

#### Out of scope
Per-display scale factors (GFX-080). Integer-only scale on V0.5 (already in Display metadata).

#### Acceptance criteria
- [ ] A native Surface on H-004 at a fractional scale factor presents sharp text relative to integer downscale (side-by-side capture).
- [ ] `os inspect display` reports the fractional scale in use.
- [ ] A Linux personality window on the same display uses the same scale factor.
- [ ] Public scale fields are rational scale factors, not DRM plane properties.

#### Verification
- Integration: `gfx:tests/scale/fractional_*` on CI matrix entry `hw-h004`.
- Manual: HiDPI internal panel screenshot compared with integer scale on H-004.

#### Evidence
- none

### GFX-050 · Report explicit degraded recovery to users and clients after GPU reset
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: GFX-056, GFX-009
- Baseline: §32, §40
- Invariants: I-037

§32: where driver constraints prevent seamless recovery the system reports it rather than failing silently. Needed once real Mesa workloads run at V1.

<!-- covers: INV-0607 -->

#### Out of scope
Compositor crash rebind (GFX-009). GPU budget (GFX-066).

#### Acceptance criteria
- [ ] A fault-injected GPU reset notifies every attached client with a typed degraded-recovery error.
- [ ] `os inspect` reports the compositor in a degraded-recovery state until the next successful present.
- [ ] The user-visible notice is not a silent black screen.
- [ ] Clients that continue after reset do so only after acknowledging the degraded event.

#### Verification
- Integration: `gfx:tests/reset/degraded_recovery_*` on CI matrix entries `hw-h002` and `hw-h004`.
- Manual: inject a GPU reset on H-002 and record the client and UI notices.

#### Evidence
- none

### GFX-051 · Run shader cache and GPU allocation broker as user-space services
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: GFX-016, GFX-010, GFX-030
- Baseline: §33, §39, §56.1
- Risks: R-016

§33 higher-level GPU services above retained DRM/KMS. The allocation broker holds the DRM descriptor on behalf of Components per the GPU userspace strategy decision.

<!-- covers: INV-0620 -->

#### Out of scope
Display policy service (GFX-048). Restartability (GFX-083). Mesa-in-Component (GFX-056).

#### Acceptance criteria
- [ ] Shader cache and allocation broker run as supervised Components distinct from the compositor.
- [ ] Native application Components hold no DRM render-node capability; the broker does.
- [ ] A Component can allocate a Buffer through the broker and submit on its RenderQueue.
- [ ] `os inspect` lists broker-held DRM rights and application-held RenderQueue rights separately.

#### Verification
- Integration: `gfx:tests/gpu/userspace_broker_*` on CI matrix entries `qemu-virtio-gpu` and `hw-h002`.
- Integration: `gfx:tests/gpu/no_dev_dri_app_*` on `hw-h002`.

#### Evidence
- none

### GFX-052 · Review Surface, Buffer, RenderQueue, Display and Frame as SDK v1 freeze candidates
- Type: docs
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: GFX-038, GFX-005, GFX-030, GFX-022, GFX-023, GFX-028
- Baseline: §65, §66
- Invariants: I-040

V1 SDK v1 marks Layer 2 Interfaces as freeze candidates. Nothing Layer 1 freezes before V4. S-024 is recorded as a freeze candidate, not frozen.

#### Out of scope
Locking versions at V4 (GFX-095). SDK crate API (SDK).

#### Acceptance criteria
- [ ] A review record lists Surface, Buffer, RenderQueue, Display and Frame as Layer 2 freeze candidates.
- [ ] The record states that S-024 is not frozen and that no Layer 1 graphics surface exists to freeze.
- [ ] Each candidate cites its spike and decision in the dependency closure.
- [ ] Review records SDK v1 sign-off on the pull request.

#### Verification
- Review: GFX, ABI and SDK leads record freeze-candidate status on the pull request.

#### Evidence
- none

### GFX-053 · Bring up i915/xe internal panel and USB-C external display on the Intel laptop
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: GFX-027, HW-022, HW-015, LAB-007
- Baseline: §55, §61, §62

V1 hardware scope adds an Intel laptop with internal display plus one external display over USB-C. Extends the driver regression matrix. HDR/VRR are not required to work at V1.

#### Out of scope
Kernel i915/xe enablement (HW-022). Fractional scaling (GFX-049). Multi-monitor arrangement (GFX-065).

#### Acceptance criteria
- [ ] The compositor presents on the H-004 internal panel at the panel's native mode.
- [ ] An external display over USB-C presents without compositor restart.
- [ ] The GPU driver regression matrix includes i915/xe rows on H-004.
- [ ] Native applications still hold no DRM device node on H-004.

#### Verification
- Integration: `gfx:tests/bringup/intel_panel_*` on CI matrix entry `hw-h004`.
- Integration: `gfx:tests/bringup/intel_usbc_*` on `hw-h004`.

#### Evidence
- none

### GFX-054 · Present as late as possible before scanout to minimise latency
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: GFX-024, GFX-003, GFX-004
- Baseline: §40, §54

§40 latency-aware presentation. Measured by the Input-to-photon and frame-latency benchmarks. Does not restate targets; B-018 and B-020 hold the numbers.

<!-- covers: INV-0749 -->

#### Out of scope
Frame callbacks (GFX-024). Immediate/tearing present (GFX-074).

#### Acceptance criteria
- [ ] The compositor schedules present at the latest deadline that still meets scanout on H-002 and H-004.
- [ ] B-018 and B-020 reports for V1 exist on H-002 and H-004.
- [ ] `os trace` shows commit-to-scanout waiting until the accepted deadline rather than presenting at commit.
- [ ] A client that opts out of late present still meets the frame-scheduling decision.

#### Verification
- Bench: B-018 and B-020 on H-002 and H-004; targets per register.
- Integration: `gfx:tests/present/late_present_*` on `hw-h002`.

#### Evidence
- none

### GFX-055 · Test that a compositor crash while locked restarts into the locked state
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: GFX-045, GFX-007
- Baseline: §32, §40
- Threats: T-009
- Invariants: I-075

Pulled from V2 to V1 because the lock exists at V1 and crash-to-unlock is a known bypass class. Standing invariant enforced by a permanent regression test.

<!-- covers: GAP-0217 -->

#### Out of scope
Lock mode (GFX-045). Unlock UI (APP, SEC).

#### Acceptance criteria
- [ ] Killing the compositor while locked restarts into lock mode with application Surfaces still inhibited.
- [ ] The test fails if any application Surface is visible or receives input after rebind.
- [ ] The test is a required CI job on compositor and lock changes.
- [ ] The denial is visible in the capability audit log.

#### Verification
- Integration: `gfx:tests/lock/crash_stays_locked_*` on CI matrix entries `qemu-virtio-gpu` and `hw-h002`.

#### Evidence
- none

### GFX-056 · Run Mesa Vulkan and OpenGL inside native Components with Capability-mediated GPU
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: GFX-016, GFX-036, GFX-051, GFX-030
- Baseline: §39, §56.1, §61
- Risks: R-016
- Invariants: I-045

V1 GPU acceleration for native applications. Implements the GPU userspace strategy decision on top of the V0.5 spike. Native applications still do not open DRM device nodes.

<!-- covers: INV-1066, INV-1205 -->

#### Out of scope
Linux personality GPU path (GFX-057). ComputeQueue (GFX-046).

#### Acceptance criteria
- [ ] A native Component renders a Vulkan sample and an OpenGL sample through RenderQueue on H-002 and H-004.
- [ ] The application Component holds no DRM render-node capability.
- [ ] Frame submission uses explicit timelines (I-084).
- [ ] `os inspect` shows GPU work attributed to the application Component.

#### Verification
- Integration: `gfx:tests/mesa/native_vulkan_*` on CI matrix entries `hw-h002` and `hw-h004`.
- Integration: `gfx:tests/mesa/native_gl_*` on `hw-h002`.
- Integration: `gfx:tests/mesa/no_dev_dri_*` on `hw-h002`.

#### Evidence
- none

### GFX-057 · Mediate Linux Personality GPU access through RenderQueue capabilities
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: GFX-056, GFX-020, LNX-031, LNX-014
- Baseline: §39, §47, §61
- Risks: R-025

V1 exit: browser WebGL, GL and Vulkan samples accelerated through the personality. LNX owns the personality; GFX owns the brokered render-node access so personality processes do not receive ambient DRM master.

<!-- covers: INV-1205 -->

#### Out of scope
Linux personality EGL/Vulkan stack (LNX-031). Native Mesa (GFX-056).

#### Acceptance criteria
- [ ] A Linux personality WebGL, OpenGL and Vulkan sample present through the compositor on H-002 and H-004.
- [ ] The personality Component holds a RenderQueue capability, not DRM master.
- [ ] Acceleration is confirmed by a frame-rate comparison to software rendering in the LNX job.
- [ ] Native applications still cannot open the personality's render node.

#### Verification
- Integration: `gfx:tests/personality/gpu_render_*` on CI matrix entries `hw-h002` and `hw-h004`.
- Compat: C-002 GPU samples on H-002.

#### Evidence
- none

### GFX-058 · Spike GPU time and memory budgets via DRM scheduler and cgroup GPU controllers
- Type: spike
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: GFX-056, SCH-031
- Baseline: §23, §39

Measure enforceability of GPU time and memory budgets on amdgpu, i915/xe and NVIDIA before the V2 ResourceDomain GPU budget build. Native software must not use vendor-specific GPU control Interfaces.

<!-- covers: INV-0443 -->

#### Out of scope
ResourceDomain GPU budget (GFX-066). DRM scheduler in the kernel (KRN).

#### Acceptance criteria
- [ ] The report measures whether DRM scheduler and cgroup GPU controllers can cap time and memory on amdgpu and i915/xe.
- [ ] The report records NVIDIA enforceability or the gap that the V2 NVIDIA stance must address.
- [ ] The report names what a native ResourceDomain GPU budget Interface can promise without vendor ioctls.
- [ ] The report is committed at `reports/spikes/GFX-058.md`.

#### Verification
- Report: enforceability per vendor; whether V2 can replace vendor GPU controls; what SCH-031 may assume.
- Review: SCH lead records that the native budget Interface does not expose cgroup paths.

#### Evidence
- none

### GFX-059 · Bring up the AMD APU internal panel and external displays on the AMD laptop
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: GFX-027, GFX-053, HW-039, LAB-018
- Baseline: §55, §62

V2 hardware scope adds an AMD laptop. Extends the driver regression matrix and hot-plug tests to H-005.

#### Out of scope
Kernel amdgpu laptop enablement (HW-039). Multi-monitor policy (GFX-077).

#### Acceptance criteria
- [ ] The compositor presents on the H-005 internal panel at the panel's native mode.
- [ ] An external display presents without compositor restart.
- [ ] The GPU driver regression matrix includes amdgpu rows on H-005.
- [ ] Native applications hold no DRM device node on H-005.

#### Verification
- Integration: `gfx:tests/bringup/amd_laptop_panel_*` on CI matrix entry `hw-h005`.
- Integration: `gfx:tests/bringup/amd_laptop_external_*` on `hw-h005`.

#### Evidence
- none

### GFX-060 · Benchmark compositor frame deadline misses under a mixed desktop workload
- Type: benchmark
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: GFX-006, GFX-024, BEN-007, LAB-018
- Baseline: §40, §54, §62
- Benchmarks: B-019

V2 benchmark gate: deadline misses under a scripted mixed workload. Harness plus published measurement; the target lives in B-019.

#### Out of scope
Idle dropped-frame test (GFX-029). BEN methodology (BEN).

#### Acceptance criteria
- [ ] Harness `bench:compositor-deadline-misses` runs the mixed desktop workload (video, animating window, text input, background build) on H-002, H-004 and H-005.
- [ ] Misses are counted from vblank timestamps.
- [ ] A report exists for each in-scope machine meeting the B-019 target kind for V2.

#### Verification
- Bench: B-019 on H-002, H-004 and H-005; target per register.

#### Evidence
- none

### GFX-061 · Implement per-Surface and per-Display screen capture capabilities
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: GFX-040, GFX-037, GFX-011
- Baseline: §9.1, §40
- Threats: T-013
- Invariants: I-085

V2 exit: an application without the capture capability receives a black or denied Surface (tested). Capture is S-034.

<!-- covers: INV-0747, INV-0210 -->

#### Out of scope
OS screenshot tool (GFX-086). Screen-share picker (GFX-085). Personality portals (LNX).

#### Acceptance criteria
- [ ] A Component without `Capability<Capture>` that requests another Surface's contents receives a denied or black Surface and no pixels.
- [ ] A Component with a per-Surface capture capability receives only that Surface's frames.
- [ ] A Component with a per-Display capture capability receives that Display's composition, excluding trusted Surfaces unless separately granted.
- [ ] Revoking the capability stops delivery within one Frame.

#### Verification
- Integration: `gfx:tests/capture/deny_*` on CI matrix entries `qemu-virtio-gpu` and `hw-h002`.
- Integration: `gfx:tests/capture/scoped_*` on `hw-h002`.
- Unit: `gfx:tests/capture/revoke_*` on `qemu-virtio-gpu`.

#### Evidence
- none

### GFX-062 · Support colour-managed Surfaces with declared colour spaces
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: GFX-063, GFX-038
- Baseline: §40, §62
- Risks: R-038

Surfaces declare primaries and transfer function so composition converts correctly under the accepted colour pipeline.

<!-- covers: GAP-0306 -->

#### Out of scope
HDR output (GFX-068). ICC profiles (GFX-073). Tone mapping (GFX-070).

#### Acceptance criteria
- [ ] A Surface commit includes primaries and transfer-function fields; omitting them uses the Display's default.
- [ ] Two Surfaces with different colour spaces on one Display are converted per the accepted pipeline.
- [ ] Public colour fields are colour-space names, not DRM property blobs.
- [ ] `os inspect surface` reports the declared colour space.

#### Verification
- Integration: `gfx:tests/colour/surface_spaces_*` on CI matrix entry `hw-h002`.
- Unit: `gfx:tests/colour/defaults_*` on `qemu-virtio-gpu`.

#### Evidence
- none

### GFX-063 · Decide the HDR and colour management pipeline
- Type: adr
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: GFX-022, LAB-017
- Baseline: §40, §62
- Decision: D-0081
- Risks: R-038

V2 exit requires an accepted HDR/colour pipeline decision before HDR10 output and tone mapping land.

<!-- covers: GAP-0306, INV-0741 -->

#### Out of scope
HDR output implementation (GFX-068). Colorimeter fixture (LAB-017).

#### Acceptance criteria
- [ ] The decision evaluates scRGB linear compositing, PQ/HLG passthrough with per-surface transforms, and hybrid with per-plane hardware LUTs.
- [ ] The accepted option states how mixed SDR and HDR Surfaces on one Display are converted.
- [ ] The accepted option names the metadata native Surfaces and personalities consume.
- [ ] Review records the accepted option on the pull request.

#### Verification
- Review: GFX and LAB leads record the accepted option on the pull request.

#### Evidence
- none

### GFX-064 · Decide the NVIDIA support stance for 1.0
- Type: adr
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: GFX-087, GFX-047
- Baseline: §56.1, §62
- Decision: D-0083
- Risks: R-037, R-050

V3 Tier 1 adds an NVIDIA desktop decided by this adr. Proprietary userspace drivers assume ambient device access and may be impossible to fit behind capabilities.

<!-- covers: GAP-0512 -->

#### Out of scope
Kernel-module policy (GFX-047). NVIDIA desktop bring-up (HW-052). Secure Boot signing (KRN, HW).

#### Acceptance criteria
- [ ] The decision evaluates open kernel modules plus NVK/Mesa, proprietary userspace via the Linux personality, and unsupported.
- [ ] The accepted option states what 1.0 promises and does not promise for NVIDIA.
- [ ] The accepted option states how capability-mediated GPU access is preserved on the chosen path.
- [ ] Review records the accepted option on the pull request.

#### Verification
- Review: GFX, HW and GOV leads record the accepted option on the pull request.

#### Evidence
- none

### GFX-065 · Implement display arrangement and per-display configuration persistence
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: GFX-077, GFX-048
- Baseline: §40, §62

Arrangement, primary display and per-display mode persist per connector fingerprint. APP settings surfaces the configuration; GFX stores and applies it.

<!-- covers: INV-0754, INV-1214 -->

#### Out of scope
Settings chrome (APP-041). Hot-plug (GFX-077).

#### Acceptance criteria
- [ ] Arrangement, primary Display and per-display mode are restored after compositor restart and after reboot.
- [ ] Persistence keys on connector fingerprint, not on a volatile DRM connector index.
- [ ] A missing display after restore does not drop windows; they migrate per policy.
- [ ] `os inspect` reports the saved arrangement.

#### Verification
- Integration: `gfx:tests/display/arrangement_persist_*` on CI matrix entries `hw-h002` and `hw-h004`.
- Integration: `gfx:tests/display/fingerprint_*` on `hw-h005`.

#### Evidence
- none

### GFX-066 · Enforce ResourceDomain GPU time and memory budgets
- Type: build
- Milestone: V2
- Status: todo
- Size: L
- Owner: none
- Depends on: GFX-058, SCH-031, GFX-030, GFX-046
- Baseline: §23, §39
- Invariants: I-033

§23 GPU budget replaces vendor-specific GPU control Interfaces for native software. Exhaustion returns a typed error.

<!-- covers: INV-0441 -->

#### Out of scope
Budget spike (GFX-058). ResourceDomain object (SCH).

#### Acceptance criteria
- [ ] A Component that exceeds its GPU memory budget receives a typed exhaustion error and allocates no additional Buffer.
- [ ] A Component that exceeds its GPU time budget is throttled on subsequent RenderQueue submissions (observed in `os inspect`).
- [ ] Native software has no vendor GPU-control Interface.
- [ ] Exhaustion is charged to the owning ResourceDomain.

#### Verification
- Integration: `gfx:tests/budget/gpu_memory_*` on CI matrix entries `hw-h002` and `hw-h004`.
- Integration: `gfx:tests/budget/gpu_time_*` on `hw-h002`.
- Unit: `gfx:tests/budget/no_vendor_ioctl_*` on `qemu-x86_64`.

#### Evidence
- none

### GFX-067 · Use the hardware cursor plane for the pointer
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: GFX-006, GFX-031
- Baseline: §40
- Threats: T-031

Removes cursor motion from the composition path. Prerequisite for direct scanout. Plane assignment does not grant a client another client's buffer (T-031).

<!-- covers: EXTRA-024 -->

#### Out of scope
Overlay scanout of fullscreen Surfaces (GFX-079). Software cursor (GFX-031).

#### Acceptance criteria
- [ ] Pointer motion updates the hardware cursor plane without a composition pass on H-002, H-004 and H-005.
- [ ] A client cannot sample the cursor plane contents of another client.
- [ ] Fallback to software cursor is reported in `os inspect` when no cursor plane exists.
- [ ] Direct scanout of a fullscreen Surface leaves the cursor plane intact.

#### Verification
- Integration: `gfx:tests/cursor/hw_plane_*` on CI matrix entries `hw-h002`, `hw-h004` and `hw-h005`.
- Integration: `gfx:tests/cursor/no_leak_*` on `hw-h002`.

#### Evidence
- none

### GFX-068 · Output HDR10 from the compositor with a native HDR test application
- Type: build
- Milestone: V2
- Status: todo
- Size: L
- Owner: none
- Depends on: GFX-063, GFX-070, GFX-073, LAB-017
- Baseline: §40, §62
- Risks: R-038

V2 exit: HDR10 verified on the HDR display with a native HDR test application. LAB owns the reference display and colorimeter.

<!-- covers: INV-0741, INV-1215 -->

#### Out of scope
Tone mapping (GFX-070). Personality HDR (GFX-069). Colorimeter (LAB).

#### Acceptance criteria
- [ ] A native HDR test application presents HDR10 on the reference display attached to H-002.
- [ ] Colorimeter measurement of the test pattern is recorded in the V2 HDR report.
- [ ] `os inspect display` reports HDR10 output active.
- [ ] SDR Surfaces on the same Display are tone-mapped per the accepted pipeline.

#### Verification
- Integration: `gfx:tests/hdr/output_pipeline_*` on CI matrix entry `hw-h002`.
- Manual: colorimeter reading of the native HDR test pattern on the LAB reference display.
- Demo: native HDR test application on H-002.

#### Evidence
- none

### GFX-069 · Expose HDR Surfaces to the Wayland bridge and Windows Personality
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: GFX-068, GFX-020
- Baseline: §40, §47, §48, §62

V2 exit requires a Windows personality HDR game. LNX and WIN consume the HDR Surface metadata GFX provides; they do not extend the native ABI.

<!-- covers: INV-1215 -->

#### Out of scope
Wine HDR passthrough (WIN-032). Native HDR output (GFX-068).

#### Acceptance criteria
- [ ] A Wayland client that declares HDR metadata presents through an HDR Surface.
- [ ] A Windows personality HDR title receives HDR Surface metadata through the Frame Interface.
- [ ] Native applications still do not speak Wayland or DXGI.
- [ ] Metadata fields match the accepted colour-pipeline decision.

#### Verification
- Integration: `gfx:tests/hdr/wayland_metadata_*` on CI matrix entry `hw-h002`.
- Compat: Windows personality HDR title on H-002 as consumed by WIN-032.

#### Evidence
- none

### GFX-070 · Implement HDR-to-SDR and SDR-in-HDR tone mapping
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: GFX-063, GFX-062
- Baseline: §40, §62
- Risks: R-038

Mixed SDR and HDR Surfaces on one Display require tone mapping under the decided colour pipeline.

<!-- covers: GAP-0306, INV-0741 -->

#### Out of scope
HDR10 output (GFX-068). Night-light (GFX-078).

#### Acceptance criteria
- [ ] An HDR Surface on an SDR Display is tone-mapped and presented without clipping to a solid colour.
- [ ] An SDR Surface on an HDR Display is mapped into the HDR output per the accepted pipeline.
- [ ] `os inspect surface` reports the tone-mapping path in use.
- [ ] Tone mapping is a compositor transform, not an application DRM LUT.

#### Verification
- Integration: `gfx:tests/hdr/tone_map_hdr_to_sdr_*` on CI matrix entry `hw-h002`.
- Integration: `gfx:tests/hdr/tone_map_sdr_in_hdr_*` on `hw-h002`.

#### Evidence
- none

### GFX-071 · Automate 100 hot-plug and unplug cycles including an MST daisy-chained dock
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: GFX-075, GFX-077, LAB-018
- Baseline: §40, §62
- Risks: R-044

V2 exit criterion, extended to cover DisplayPort MST docks. Windows must not be lost.

<!-- covers: INV-0754, EXTRA-026 -->

#### Out of scope
MST topology (GFX-075). Arrangement persistence (GFX-065).

#### Acceptance criteria
- [ ] 100 hot-plug and unplug cycles of two displays complete on H-004 and H-005 without compositor restart.
- [ ] The cycle set includes an MST daisy-chained dock.
- [ ] No window is lost; windows migrate per arrangement policy.
- [ ] A failed cycle is a CI failure.

#### Verification
- Integration: `gfx:tests/hotplug/cycle_*` on CI matrix entries `hw-h004` and `hw-h005`.
- Manual: LAB dock fixture attached for the MST subset.

#### Evidence
- none

### GFX-072 · Support render offload selection and mux switching across both personalities
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: GFX-076, HW-048
- Baseline: §40, §62
- Risks: R-044

Per-application GPU selection surfaced through RenderQueue capabilities and passed through the Wayland bridge and Wine. V2 implements the compositor path; hybrid SKUs H-011 and H-012 gate at V4.

<!-- covers: EXTRA-025 -->

#### Out of scope
Multi-GPU scanout (GFX-076). ACPI mux (HW-048).

#### Acceptance criteria
- [ ] An application can be launched with a RenderQueue bound to the discrete GPU while scanout stays on the integrated GPU.
- [ ] Mux switch is visible in `os inspect` and does not drop windows.
- [ ] Linux personality and Windows personality clients receive the selected RenderQueue, not ambient DRM master.
- [ ] Native applications still do not open vendor mux device nodes.

#### Verification
- Integration: `gfx:tests/hybrid/offload_*` on CI matrix entries that expose iGPU+dGPU.
- Integration: `gfx:tests/hybrid/mux_switch_*` on those machines.

#### Evidence
- none

### GFX-073 · Apply per-display ICC profiles in the compositor
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: GFX-063, LAB-017, GFX-022
- Baseline: §40, §62
- Risks: R-038

Colour management baseline for creative users. Validated with the colorimeter.

<!-- covers: GAP-0306 -->

#### Out of scope
Colour-managed Surfaces (GFX-062). Night-light (GFX-078). Settings chrome (APP).

#### Acceptance criteria
- [ ] Each Display can have an ICC profile applied; composition uses it.
- [ ] Colorimeter verification of a known patch is recorded on H-002.
- [ ] Removing the profile restores the Display default.
- [ ] Native applications do not load the profile through a DRM colour-management ioctl.

#### Verification
- Integration: `gfx:tests/colour/icc_apply_*` on CI matrix entry `hw-h002`.
- Manual: colorimeter patch reading on the LAB reference display.

#### Evidence
- none

### GFX-074 · Support tearing and immediate presentation for fullscreen Surfaces
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: GFX-079, GFX-023
- Baseline: §40, §56.2

Fullscreen games opt into tearing/async presentation. Exposed to the Windows personality via the Frame Interface. Explicit timelines remain mandatory (I-084).

<!-- covers: EXTRA-024 -->

#### Out of scope
Direct scanout (GFX-079). Wine present (WIN-031).

#### Acceptance criteria
- [ ] A fullscreen Surface that opts into immediate presentation is presented without waiting for vblank.
- [ ] A windowed Surface requesting immediate presentation is rejected with a typed error.
- [ ] Immediate presentation still carries an explicit timeline.
- [ ] The Frame Interface exposes the opt-in to the Windows personality without a DXGI type.

#### Verification
- Integration: `gfx:tests/present/immediate_fullscreen_*` on CI matrix entry `hw-h002`.
- Integration: `gfx:tests/present/immediate_windowed_deny_*` on `qemu-virtio-gpu`.

#### Evidence
- none

### GFX-075 · Support DisplayPort MST topologies and daisy-chained docks
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: GFX-077, HW-059, HW-057
- Baseline: §40, §62
- Risks: R-044

Docking stations are the dominant laptop multi-monitor path. MST topology changes must not lose windows.

<!-- covers: EXTRA-026, INV-1222 -->

#### Out of scope
Hot-plug cycle test (GFX-071). Dock USB/Ethernet (HW).

#### Acceptance criteria
- [ ] An MST daisy-chained dock presents two Displays without compositor restart on H-004 and H-005.
- [ ] Unplugging one MST stream migrates windows per arrangement policy; no window is destroyed.
- [ ] `os inspect` lists each MST Display with a stable fingerprint.
- [ ] Native applications do not see DRM MST connector names.

#### Verification
- Integration: `gfx:tests/mst/topology_*` on CI matrix entries `hw-h004` and `hw-h005`.
- Manual: LAB dock fixture for the daisy-chain case.

#### Evidence
- none

### GFX-076 · Render on one GPU and scan out on another
- Type: build
- Milestone: V2
- Status: todo
- Size: L
- Owner: none
- Depends on: GFX-006, GFX-051, GFX-044
- Baseline: §40, §62
- Risks: R-044

§40 multi-GPU awareness. Hybrid laptops route dGPU renders to iGPU-attached panels. Buffer import across GPUs uses the MemoryObject GPU-compatible path.

<!-- covers: INV-0743, EXTRA-025 -->

#### Out of scope
Offload selection UX (GFX-072). ACPI mux (HW).

#### Acceptance criteria
- [ ] A Frame rendered on GPU A is scanned out on GPU B without an extra CPU copy on hardware that permits shared physical memory.
- [ ] `os inspect frame` reports render GPU and scanout GPU.
- [ ] Failure to import across GPUs returns a typed error and falls back to a copy that is visible in `os inspect`.
- [ ] Native applications do not name DRM card indices.

#### Verification
- Integration: `gfx:tests/multigpu/render_scanout_*` on CI matrix entries that expose two GPUs.
- Integration: `gfx:tests/multigpu/copy_fallback_*` on `hw-h002` when a second GPU is absent.

#### Evidence
- none

### GFX-077 · Support multi-monitor hot-plug, docks and USB-C DisplayPort Alt Mode
- Type: build
- Milestone: V2
- Status: todo
- Size: L
- Owner: none
- Depends on: GFX-022, GFX-048, GFX-008, GFX-042, HW-059
- Baseline: §40, §62

V2 exit: two displays with different scales and refresh rates, hot-plug without compositor restart or window loss.

<!-- covers: INV-0754, INV-1222 -->

#### Out of scope
MST daisy-chain (GFX-075). Per-display scale (GFX-080). Arrangement persistence (GFX-065).

#### Acceptance criteria
- [ ] Two Displays with different scale factors and refresh rates present simultaneously on H-002, H-004 and H-005.
- [ ] Hot-plug and unplug of the second Display does not restart the compositor or destroy windows.
- [ ] USB-C DisplayPort Alt Mode is a supported connector class on H-004 and H-005.
- [ ] `os inspect` lists both Displays as separate objects.

#### Verification
- Integration: `gfx:tests/monitor/dual_*` on CI matrix entries `hw-h002`, `hw-h004` and `hw-h005`.
- Integration: `gfx:tests/monitor/hotplug_*` on `hw-h004`.

#### Evidence
- none

### GFX-078 · Implement night-light colour temperature mode
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: GFX-062, GFX-073
- Baseline: §40, §62

Desktop essential listed in GAP-0306. Implemented as a compositor output transform, scheduled by APP settings.

<!-- covers: GAP-0306 -->

#### Out of scope
Settings schedule chrome (APP). ICC profiles (GFX-073).

#### Acceptance criteria
- [ ] Enabling night-light applies a colour-temperature transform on every Display.
- [ ] Disabling night-light restores the previous colour pipeline.
- [ ] The transform is visible in `os inspect display`.
- [ ] Applications cannot disable night-light for their own Surfaces.

#### Verification
- Integration: `gfx:tests/colour/night_light_*` on CI matrix entries `qemu-virtio-gpu` and `hw-h002`.

#### Evidence
- none

### GFX-079 · Scan out fullscreen Surfaces directly via overlay planes
- Type: build
- Milestone: V2
- Status: todo
- Size: L
- Owner: none
- Depends on: GFX-067, GFX-006, GFX-023
- Baseline: §40, §56.2
- Threats: T-031

V2 gaming proof of concept and HDR video need direct scanout to avoid a composition pass. Plane assignment does not grant a client the contents of another client's buffer.

<!-- covers: EXTRA-024 -->

#### Out of scope
Hardware cursor (GFX-067). Immediate presentation (GFX-074).

#### Acceptance criteria
- [ ] A fullscreen Surface that meets format and size constraints is scanned out on an overlay plane with no composition pass.
- [ ] `os trace` records direct scanout rather than a compose-and-scanout path.
- [ ] A client cannot read another client's overlay buffer (T-031).
- [ ] Falling back to composition when constraints fail is reported in `os inspect`.

#### Verification
- Integration: `gfx:tests/overlay/direct_scanout_*` on CI matrix entry `hw-h002`.
- Integration: `gfx:tests/overlay/no_leak_*` on `hw-h002`.

#### Evidence
- none

### GFX-080 · Implement per-display scale factors
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: GFX-077, GFX-049
- Baseline: §40, §62

Different scale factors per Display is an explicit V2 exit criterion.

<!-- covers: INV-0744, INV-1214 -->

#### Out of scope
Fractional scaling on one Display (GFX-049). Arrangement (GFX-065).

#### Acceptance criteria
- [ ] Two Displays can run at different scale factors simultaneously.
- [ ] A window moved from one Display to the other is rescaled without client restart.
- [ ] `os inspect display` reports each Display's scale.
- [ ] Linux personality windows follow the Display they occupy.

#### Verification
- Integration: `gfx:tests/scale/per_display_*` on CI matrix entries `hw-h002`, `hw-h004` and `hw-h005`.

#### Evidence
- none

### GFX-081 · Skip idle frames and lower refresh on battery under SCH energy intent
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: GFX-006, GFX-029, SCH-045
- Baseline: §22, §40, §62

§40 power-aware rendering for the V2 battery runtime benchmarks. PWR supplies the policy signal; SCH owns energy intent; GFX skips idle frames and lowers refresh.

<!-- covers: INV-0750 -->

#### Out of scope
Energy policy (SCH, PWR). Idle dropped-frame test (GFX-029). VRR (GFX-088).

#### Acceptance criteria
- [ ] An idle desktop under EnergyEfficient intent skips composition of undamaged Frames.
- [ ] On battery, refresh is lowered when the Display supports it, visible in `os inspect display`.
- [ ] Interactive intent restores the previous refresh without compositor restart.
- [ ] B-031 energy reports for V2 exist on H-004 and H-005 (published by PWR/BEN; this task supplies the compositor path).

#### Verification
- Integration: `gfx:tests/power/skip_idle_*` on CI matrix entries `hw-h004` and `hw-h005`.
- Integration: `gfx:tests/power/refresh_on_battery_*` on `hw-h005`.

#### Evidence
- none

### GFX-082 · Provide a non-suppressible indicator layer for camera, microphone, capture and location
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: GFX-061, GFX-040, CAP-001
- Baseline: §9.1, §40
- Threats: T-013, T-014

Compositor-protected layer no Surface can cover, fed by CAP audit events with per-application attribution. APP's shell renders the icons.

<!-- covers: GAP-0230 -->

#### Out of scope
Indicator chrome (APP-031). Camera service (MED-013). Capture capability (GFX-061).

#### Acceptance criteria
- [ ] While camera, microphone, capture or location capabilities are exercised, the indicator layer is presented.
- [ ] No application Surface can stack above the indicator layer (tested).
- [ ] `os inspect` attributes each indicator to the exercising Component.
- [ ] Stopping the capability removes that application's indicator.

#### Verification
- Integration: `gfx:tests/indicator/layer_*` on CI matrix entries `qemu-virtio-gpu` and `hw-h002`.
- Integration: `gfx:tests/indicator/no_cover_*` on `hw-h002`.

#### Evidence
- none

### GFX-083 · Make shader cache, allocation broker and display policy restartable
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: GFX-051, GFX-048, GFX-009
- Baseline: §32, §33
- Invariants: I-037

§32 selected GPU user-space services restartable where driver support permits. Tested with kill-and-rebind like the compositor.

<!-- covers: INV-0599 -->

#### Out of scope
Compositor rebind (GFX-009). GPU reset degraded recovery (GFX-050).

#### Acceptance criteria
- [ ] Killing the shader cache, allocation broker or display policy service rebinds clients without compositor restart.
- [ ] In-flight RenderQueue submissions fail with a typed error or complete; none hang.
- [ ] `os inspect` reports each service as restartable with a restart count.
- [ ] Where the driver cannot survive the kill, clients receive the degraded-recovery notice from GFX-050.

#### Verification
- Integration: `gfx:tests/gpu/service_rebind_*` on CI matrix entries `hw-h002` and `hw-h004`.

#### Evidence
- none

### GFX-084 · Build the OS-owned screen recorder over the capture Capability
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: GFX-061, GFX-085, MED-026, MED-018
- Baseline: §40
- Invariants: I-085

Records via capture capability with encode delegated to MED's hardware encoder Component. §40 forbids unrestricted capture, so the OS supplies the recorder.

<!-- covers: GAP-0305 -->

#### Out of scope
Encode (MED-026). Capture capability (GFX-061). Recorder chrome (APP-038).

#### Acceptance criteria
- [ ] The recorder obtains frames only through a capture capability minted by the picker.
- [ ] Stopping capture stops the encoder input within one Frame.
- [ ] A Component without capture rights cannot start a recording.
- [ ] The indicator layer is active for the duration of the recording.

#### Verification
- Integration: `gfx:tests/capture/record_*` on CI matrix entries `qemu-virtio-gpu` and `hw-h002`.
- Integration: `gfx:tests/capture/record_deny_*` on `qemu-virtio-gpu`.

#### Evidence
- none

### GFX-085 · Build the OS-owned screen-share picker with per-app scoping for portals
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: GFX-061, GFX-040
- Baseline: §9.1, §40
- Threats: T-012, T-013
- Invariants: I-085

Trusted-UI picker minting capture capabilities. LNX's xdg-desktop-portal ScreenCast calls into it. Applications cannot spoof the picker.

<!-- covers: INV-0747, GAP-0305 -->

#### Out of scope
Portal D-Bus (LNX). Capture capability (GFX-061). Consent chrome policy (SEC, APP).

#### Acceptance criteria
- [ ] The picker is a trusted Surface; applications cannot overlay or inject input into it.
- [ ] Selecting one Surface or Display mints a capture capability for only that object.
- [ ] Cancelling the picker mints nothing.
- [ ] A Linux personality ScreenCast request is satisfied only through this picker.

#### Verification
- Integration: `gfx:tests/capture/picker_scope_*` on CI matrix entries `qemu-virtio-gpu` and `hw-h002`.
- Integration: `gfx:tests/capture/picker_trusted_*` on `hw-h002`.

#### Evidence
- none

### GFX-086 · Build the OS-owned screenshot tool with region selection and annotation
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: GFX-061, GFX-085
- Baseline: §40
- Invariants: I-085

§40 forbids unrestricted capture so the OS must supply the tool users would otherwise install. Region selection and annotation run in a trusted Surface.

<!-- covers: GAP-0305 -->

#### Out of scope
Shell chrome (APP-038). Capture capability (GFX-061).

#### Acceptance criteria
- [ ] A screenshot of a region is taken only after the picker or a trusted region-select Surface grants capture.
- [ ] Annotation draws on a copy; source Surfaces are not writable by the tool.
- [ ] A Component without capture rights cannot invoke the tool to read other Surfaces.
- [ ] The resulting image is a MemoryObject the user can save through UserSelected.

#### Verification
- Integration: `gfx:tests/capture/screenshot_region_*` on CI matrix entries `qemu-virtio-gpu` and `hw-h002`.
- Integration: `gfx:tests/capture/screenshot_deny_*` on `qemu-virtio-gpu`.

#### Evidence
- none

### GFX-087 · Evaluate NVIDIA open modules with NVK and proprietary userspace against capabilities
- Type: spike
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: GFX-047, GFX-036, HW-052
- Baseline: §56.1, §9.1
- Risks: R-037, R-050

V2 tracks an NVIDIA desktop as experimental. The spike measures each stack behind brokered device access before the stance adr.

<!-- covers: GAP-0512 -->

#### Out of scope
NVIDIA 1.0 stance (GFX-064). Tier 1 NVIDIA bring-up (HW-052).

#### Acceptance criteria
- [ ] The report measures open kernel modules plus NVK/Mesa behind the GPU broker.
- [ ] The report measures proprietary userspace behind the Linux personality and records which DRM objects it requires.
- [ ] The report states whether proprietary userspace can run without ambient device-node access.
- [ ] The report is committed at `reports/spikes/GFX-087.md`.

#### Verification
- Report: viability of NVK versus proprietary userspace against the capability model; which option the stance adr prefers; what remains unsupported.
- Review: HW lead records that H-006 experimental bring-up matches the measured stacks.

#### Evidence
- none

### GFX-088 · Support variable refresh rate output with display-reported Verification
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: GFX-022, GFX-008, GFX-024
- Baseline: §40, §62

V2 exit: VRR verified via display reporting at variable frame rates.

<!-- covers: INV-0740, INV-1216 -->

#### Out of scope
HDR output (GFX-068). Power-aware refresh lowering (GFX-081).

#### Acceptance criteria
- [ ] A client presenting at a variable frame rate is scanned out at a matching refresh on a VRR Display attached to H-002.
- [ ] Display-reported refresh tracks the client frame rate across the tested range.
- [ ] `os inspect display` reports VRR active.
- [ ] Native applications do not program DRM VRR properties directly.

#### Verification
- Integration: `gfx:tests/vrr/output_*` on CI matrix entry `hw-h002`.
- Manual: display-reported refresh log on the LAB VRR panel.

#### Evidence
- none

### GFX-089 · Decide whether RDP/VNC clients and a remote-desktop server are in 1.0 scope
- Type: adr
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: GFX-092
- Baseline: §40, §43, §57
- Decision: D-0085
- Invariants: I-047

Pulled from V4 to V3 so the decision precedes the V4 feature freeze. 1.0 excludes distributed Interfaces as a kernel concern; this decision is whether clients and a userspace server over remote Surfaces are in product scope.

<!-- covers: GAP-0440 -->

#### Out of scope
Network transport for remote Surfaces (GFX-098). VM guest Surfaces (GFX-092).

#### Acceptance criteria
- [ ] The decision evaluates out of scope for 1.0, clients only via the Linux personality, and a native server over remote Surfaces.
- [ ] The accepted option states that distribution is not a kernel concern (I-047).
- [ ] The accepted option lists follow-up tasks that stay in LATER if 1.0 is out of scope.
- [ ] Review records the accepted option on the pull request.

#### Verification
- Review: GFX, IPC and GOV leads record the accepted option on the pull request.

#### Evidence
- none

### GFX-090 · Build the per-machine display test suite for Tier 1 hardware
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: GFX-071, GFX-068, GFX-088, GFX-077, LAB-021
- Baseline: §62, §63

V3 six Tier 1 machines fully tested each release. The Hardware Compatibility List needs display hot-plug, HDR/VRR and multi-monitor results per machine. Runs every release candidate through V4 and 1.0.

#### Out of scope
Hardware Compatibility List publication (HW, REL). Lab fleet (LAB).

#### Acceptance criteria
- [ ] The suite runs display hot-plug, HDR/VRR where the SKU supports it, and multi-monitor on H-002, H-004, H-005, H-006, H-007 and H-008.
- [ ] Each machine produces a per-feature pass/fail record consumed by the Hardware Compatibility List.
- [ ] A failing display case is a release-qualification failure.
- [ ] NVIDIA rows follow HW-070.

#### Verification
- Integration: `gfx:tests/hcl/display_suite_*` on CI matrix entries `hw-h002`, `hw-h004`, `hw-h005`, `hw-h006`, `hw-h007` and `hw-h008`.
- Review: HW lead records that the suite feeds the Hardware Compatibility List schema.

#### Evidence
- none

### GFX-091 · Add fuzz targets for compositor and graphics interfaces
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: GFX-038, GFX-005, GFX-023, GFX-043, BLD-035
- Baseline: §40, §51

V3 gate: IPC fuzzing runs continuously. BLD owns the fuzz infrastructure; GFX owns compositor and graphics Interface targets.

#### Out of scope
Fuzz infrastructure (BLD-035). Open-crasher gate (BLD-063).

#### Acceptance criteria
- [ ] Structure-aware fuzz targets exist for Surface, Buffer, Frame, Display, RenderQueue and the window-policy Interface.
- [ ] Targets are wired into BLD's continuous fuzzing fleet.
- [ ] A known crasher in these targets is filed as a GFX task and counted by the V3 crasher-age gate.
- [ ] Native fuzz executors do not open DRM device nodes.

#### Verification
- Fuzz: `gfx:fuzz/surface_buffer_frame` and `gfx:fuzz/wm_policy` running continuously in CI.
- Review: BLD lead records that the targets are in the V3 crasher-age set.

#### Evidence
- none

### GFX-092 · Support Surfaces produced by a KVM guest as remote surfaces
- Type: build
- Milestone: V3
- Status: todo
- Size: L
- Owner: none
- Depends on: GFX-009, GFX-038, VIRT-003, IPC-025
- Baseline: §40, §43
- Invariants: I-047

§40 remote Surfaces for producers inside a VM. VIRT's VM manager presents guest virtio-gpu output as native windows. Distribution is not a kernel concern; the compositor consumes a remote-Surface transport supplied by IPC/VIRT.

<!-- covers: INV-0746 -->

#### Out of scope
Guest GPU (VIRT-011). Cross-machine network transport (GFX-098). Guest tools (VIRT).

#### Acceptance criteria
- [ ] A KVM guest virtio-gpu output appears as a native Surface/window on the host compositor.
- [ ] Guest resize is reflected without compositor restart.
- [ ] The host application does not hold the guest's DRM objects.
- [ ] Disconnect of the guest is a typed Surface failure, not a compositor crash.

#### Verification
- Integration: `gfx:tests/remote/vm_guest_surface_*` on CI matrix entry `qemu-nested` (H-015).
- Demo: guest window beside native windows on H-002.

#### Evidence
- none

### GFX-093 · Fix compositor and trusted-UI findings from the external security audit
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: SEC-070, GFX-040, GFX-045, GFX-061
- Baseline: §40, §51
- Risks: R-055
- Threats: T-012, T-013, T-009

V4 exit requires all High/Critical audit findings fixed and re-verified. The compositor enforces trusted UI, lock and capture and is in the audit scope.

#### Out of scope
Audit commissioning (SEC-070). Auditor re-verification (SEC-069).

#### Acceptance criteria
- [ ] Every High and Critical finding whose asset is the compositor, trusted UI, lock or capture is fixed.
- [ ] Each fix has a regression test that fails without the fix.
- [ ] Re-verification evidence is recorded against SEC-069.
- [ ] Medium compositor findings are triaged with public tracking.

#### Verification
- Integration: `gfx:tests/audit/regression_*` on CI matrix entries `qemu-virtio-gpu` and `hw-h002`.
- Review: auditor re-verification recorded on the pull request (SEC-069).

#### Evidence
- none

### GFX-094 · Tune the presentation path to meet the input-to-photon p99 target
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: GFX-004, GFX-060, GFX-054, GFX-067
- Baseline: §40, §54, §65
- Benchmarks: B-019, B-020

V4 benchmark gate: Input-to-photon p99 and deadline misses on every Tier 1 machine. Targets live in B-020 and B-019; this task tunes the presentation path until those reports meet the V4 kind.

#### Out of scope
Harnesses (GFX-004, GFX-060). Photodiode rig (LAB).

#### Acceptance criteria
- [ ] B-020 reports for V4 exist on every Tier 1 machine in the V4 hardware scope and meet the V4 target kind.
- [ ] B-019 reports for V4 exist on those machines and meet the V4 target kind.
- [ ] Tuning changes are documented as compositor present-path changes, not as restated numbers in this task.
- [ ] Native software still uses explicit Frame timelines.

#### Verification
- Bench: B-020 and B-019 on every V4 Tier 1 machine; targets per register.
- Integration: `gfx:tests/present/v4_path_*` on `hw-h002`.

#### Evidence
- none

### GFX-095 · Lock Surface, Buffer, RenderQueue, Display and Frame Interface versions for 1.x
- Type: build
- Milestone: V4
- Status: todo
- Size: S
- Owner: none
- Depends on: GFX-052, GFX-036, GFX-014, GFX-017, GFX-038, GFX-005, GFX-030, GFX-022, GFX-023, IPC-042, GFX-032, GFX-012
- Baseline: §65, §66
- Freezes: S-024
- Invariants: I-040

V4 exit: Layer 2 Interface versions enumerated and locked with the old/new client evolution test passing for every core graphics Interface. S-024 freezes here. Layer 1 graphics surfaces do not exist and are not frozen.

#### Out of scope
Layer 1 ABI freeze (ABI). Screen-capture capability freeze (S-034 remains prototyped unless a later task freezes it). SDK crate API (SDK).

#### Acceptance criteria
- [ ] Surface, Buffer, RenderQueue, Display and Frame versions for 1.x are enumerated.
- [ ] The old-client/new-service and new-client/old-service evolution test passes for each.
- [ ] S-024 is recorded frozen by this task after its spike and decision closure.
- [ ] No Layer 1 graphics identifier is listed as frozen.

#### Verification
- Integration: `gfx:tests/abi/l2_evolution_*` on CI matrix entries `qemu-virtio-gpu` and `hw-h002`.
- Review: ABI lead records S-024 frozen on the pull request.

#### Evidence
- none

### GFX-096 · Publish the 1.0 GPU support statement and non-promises
- Type: docs
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: GFX-064, GFX-001, HW-070
- Baseline: §56.1, §57, §39
- Invariants: I-045

1.0 explicitly does not promise a native GPU driver stack. Publishes the retained DRM/Mesa posture and the NVIDIA stance for the support window.

<!-- covers: INV-1117, INV-1060 -->

#### Out of scope
Hardware Compatibility List rows (HW, REL). Native GPU driver rewrite (forbidden, I-045).

#### Acceptance criteria
- [ ] The statement lists retained DRM/KMS plus Mesa as the 1.0 GPU stack.
- [ ] The statement repeats that a native GPU driver stack is not a 1.0 promise.
- [ ] The statement cites the accepted NVIDIA stance.
- [ ] The statement is part of the 1.0 non-promise list.

#### Verification
- Review: GFX, HW and REL leads record publication sign-off on the pull request.

#### Evidence
- none

### GFX-097 · Verify external display, HDR and VRR on every HCL Tier 1 machine per release
- Type: build
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: GFX-090, HW-089
- Baseline: §62, §63

1.0 Hardware Compatibility List lists external displays and HDR/VRR as working on every Tier 1 machine that has the hardware. Runs the V3 display test suite as a release gate.

#### Out of scope
Hardware Compatibility List publication (HW-088). Suite implementation (GFX-090).

#### Acceptance criteria
- [ ] The V3 display suite passes on every 1.0 Tier 1 machine for the features that machine lists.
- [ ] External display, HDR and VRR rows in the Hardware Compatibility List match the suite results.
- [ ] A mismatch is a 1.0 release-qualification failure.

#### Verification
- Integration: `gfx:tests/hcl/display_suite_*` on every 1.0 Tier 1 machine.
- Review: HW and REL leads record that Hardware Compatibility List display rows match.

#### Evidence
- none

### GFX-098 · Support Surfaces produced on another machine over a network transport
- Type: build
- Milestone: LATER
- Status: todo
- Size: L
- Owner: none
- Depends on: GFX-089, GFX-092, IPC-025
- Baseline: §40, §43, §57
- Invariants: I-047

The cross-machine half of §40 remote Surfaces. 1.0 excludes distributed Interfaces as a kernel concern, so this is parked pending the remote-desktop scope decision and IPC's distribution transport. Capabilities, identity, encryption and explicit user policy still apply.

<!-- covers: INV-0746 -->

#### Out of scope
VM guest Surfaces (GFX-092). Kernel distribution (forbidden, I-047). RDP/VNC product scope (GFX-089).

#### Acceptance criteria
- [ ] A Surface produced on another machine presents on the local compositor through the accepted IPC transport.
- [ ] The local compositor does not hold the remote machine's DRM objects.
- [ ] Disconnect is a typed Surface failure.
- [ ] The path is used only if GFX-089 accepted a native server.

#### Verification
- Integration: `gfx:tests/remote/network_surface_*` on a pair of lab machines once the transport exists.
- Review: IPC lead records that the kernel is not the transport.

#### Evidence
- none
