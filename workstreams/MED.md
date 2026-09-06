# MED · Media
- Prefix: MED
- Lead: none
- Baseline: §7, §9.1, §11, §16, §17, §32, §37, §39, §51, §54, §56.5, §57, §66
- Baseline gap: The baseline names Object<Camera>, ImageDecoder isolation, MemoryObject zero-copy and media as an ecosystem need, but specifies no media pipeline, codec packaging, hardware decode path, camera service or protected-content policy.

<!-- roadmap:generated:begin summary -->
Tasks: 42 live, 0 done, 0 in-progress, 42 todo, 0 dropped. Ready: 0. Blocked: 42. Weighted: 0%.
<!-- roadmap:generated:end -->

## Scope

MED owns the native media stack that the baseline left unnamed. Codec work runs as sandboxed Components that exchange MemoryObjects: container demux, royalty-free software decode and encode, and hardware video decode and encode on retained Mesa (VA-API or Vulkan Video) exposed through ComputeDevice rather than as a native GPU stack. Frames move by ownership transfer from storage or NIC through decoder to GPU and scanout (§17). Codecs ship as separately updatable Packages with per-codec Capability sets and licensing metadata.

The Camera service mints per-session `Capability<Camera>` with a persistent in-use signal, virtual Camera sources, UVC hot-plug, and a first-party capture client. Screen-recording and conferencing encode consume Frames only through the explicit screen-capture Capability (GFX) and the Camera grant. A native playback session binds play, pause, seek and now-playing metadata for the shell. Protected-content policy is Widevine L3 through the Linux-personality browser CDM; native CDM and Layer 1 secure-path playback stay out of 1.0.

## Out of scope

Audio graph, mixer, echo cancellation implementation and microphone grants (AUD). Compositor, Buffer, Surface, HDR output, screen-capture Capability and recorder UI (GFX). Object<Device>, UVC enablement and the combined hardware suite (HW). ComputeDevice enumeration and GPU dispatch ABI (HET). Shell in-use indicators, media-key chrome, MPRIS and the first-party application set Decision (APP). Camera and ScreenCast portals, PipeWire clients and the personality browser CDM host (LNX). Codec patent and shipping policy (GOV). MemoryObject backing, map and physical-page identity (MEM). Capability rights encoding, revocation walk and per-user grant stores (CAP). IDL compiler and Layer 2 evolution rules (IPC). Supervisor restart policy (SVC). Package format and content-addressed store (PKG). Isolation denial harness (SEC). ResourceDomain budgets (SCH). Component graphs and spawn (CMP). Benchmark register and cross-OS publication (BEN). Fuzz fleet plumbing (BLD). HCL publication (REL). Docs site (DOC). Physical capture rigs (LAB).

## Tasks

### MED-001 · Decide Camera service model over V4L2 and libcamera
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: MED-006
- Baseline: §7, §9.1, §33
- Decision: D-0186
- Threats: T-014
- Invariants: I-009, I-021

The baseline left media unspecified: no pipeline, codec packaging, hardware decode path, Camera service or protected-content policy. Sibling adrs MED-007 and MED-004 cover pipeline and hardware decode; this Decision is the Camera service model. Object<Camera> is a §7 kernel object, but the baseline never says whether the service talks V4L2, libcamera, or both. Native software holds `Capability<Camera>` and never a V4L2 file descriptor. I-009 forbids rewriting a working sensor stack without a recorded benefit from MED-006.

<!-- covers: EXTRA-057, GAP-0303 -->

#### Out of scope
Camera service implementation (MED-013). UVC Device objects (HW-078). Shell in-use indicator (APP-031).

#### Acceptance criteria
- [ ] Option A (V4L2 as the Camera service mechanism, libcamera only on ISP pipelines the spike shows V4L2 cannot drive), Option B (libcamera as the Camera service on every sensor) and Option C (V4L2 only, no libcamera) are evaluated against the libcamera-spike report, T-014 and I-009.
- [ ] The accepted option names the Component that mints `Capability<Camera>`, the retained mechanism, and that native software never receives a V4L2 device node.
- [ ] Architecture review sign-off is recorded on the pull request.

#### Verification
- Review: MED and HW leads sign off on the pull request that accepts the decision file.
- Report: the Decision file cites `reports/spikes/MED-006.md` in its options.

#### Evidence
- none

### MED-002 · Define codec Package Capability sets and licensing metadata
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: MED-007, GOV-020, PKG-011, PKG-033
- Baseline: §11, §28, §51
- Risks: R-068
- Invariants: I-068

EXTRA-058 requires per-codec Capability sets and licensing metadata before V2 royalty-free codecs ship. This task defines the Package schema: requested Capabilities (input bytes, output Frames, ResourceDomain budget), SPDX license and patent notes, and a stub isolated decoder Package that installs under PKG rules so V2 AV1 and VP9 Packages are schema-compatible. R-068 is carried as metadata here; GOV-020 decides what the default image may redistribute.

<!-- covers: EXTRA-058 -->

#### Out of scope
Royalty-free decoder Components (MED-024). Patent policy (GOV-020). Package store layout (PKG).

#### Acceptance criteria
- [ ] The codec Package manifest schema names per-codec Capability sets, SPDX license, patent class (royalty-free or hardware-passthrough) and ResourceDomain budget fields.
- [ ] A stub isolated decoder Package builds with `os package build`, installs, and `os inspect package` shows those fields.
- [ ] The stub Component declares no network, filesystem, microphone or Camera Capability (I-068 rows are on the userspace allowlist).
- [ ] CI rejects a fixture Package whose patent class is hardware-passthrough but whose Capability set includes a software-decode right.

#### Verification
- Unit: `media:tests/codec_package_schema_*` on CI matrix entries `qemu-x86_64` (H-001) and `hw-h002`.
- Integration: stub Package install and inspect on `qemu-x86_64`.
- Review: GOV licensing and PKG leads confirm the patent-class field matches GOV-020.

#### Evidence
- none

### MED-003 · Expose hardware video decode into MemoryObjects
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: MED-004, MEM-024, GFX-056, MEM-022
- Baseline: §16, §17, §39, §56.5
- Invariants: I-045, I-063

Codecs and hardware decode are MED; a first-party player is APP. After MED-004 chooses VA-API or Vulkan Video, this task exposes that retained Mesa path so decoded Frames land in GPU-compatible MemoryObjects. It is the V1 plumbing INV-1112 needs; V2 wraps the same path in ComputeDevice and compositor transfer. No native GPU driver stack (I-045).

<!-- covers: INV-1112 -->

#### Out of scope
ComputeDevice wrapping (MED-017). Native player (APP). DRM ioctls as a native API (GFX).

#### Acceptance criteria
- [ ] A native Component without a DRM device node submits a compressed AV1 or VP9 bitstream and receives a GPU-compatible MemoryObject Frame on H-002 using the path MED-004 selected.
- [ ] Physical-page identity of the output MemoryObject matches the decoder output on H-002 (I-063).
- [ ] Submitting without `Capability` rights for the decode path returns `Error::Rights` and allocates no Frame.
- [ ] Native crates in the decode path do not link a public libva or Vulkan Video API as the native SDK.

#### Verification
- Integration: `media:tests/hw_decode_memoryobject_*` on `hw-h002`.
- Manual: `os inspect` on H-002 shows the output MemoryObject GPU-compatible property and no DRM fd in the Component.
- Review: GFX lead confirms the path uses retained Mesa inside the Component (GFX-056).

#### Evidence
- none

### MED-004 · Decide VA-API versus Vulkan Video for hardware codecs
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: MED-005
- Baseline: §17, §37, §39, §57
- Decision: D-0187
- Risks: R-068
- Invariants: I-009, I-045

EXTRA-057 requires the hardware-decode Decision before GAP-0300 V2 builds. The choice is which retained Mesa path is native, exposed later through ComputeDevice, not a rewritten GPU stack (I-045, I-009). MED-005 supplies measurements on H-002 and H-004.

<!-- covers: EXTRA-057 -->

#### Out of scope
V2 ComputeDevice decode (MED-017). GPU ComputeDevice backend (HET-003). Codec patent policy (GOV-020).

#### Acceptance criteria
- [ ] Option A (VA-API on retained Mesa), Option B (Vulkan Video on retained Mesa) and Option C (both, selected per GPU) are evaluated against the hw-video-spike report, I-045 and I-009.
- [ ] The accepted option names the Mesa path native software calls through MED Interfaces, and records that DRM and VA-API ioctls are not native APIs.
- [ ] Architecture review sign-off is recorded on the pull request.

#### Verification
- Review: MED, GFX and HET leads sign off on the pull request that accepts the decision file.
- Report: the Decision file cites `reports/spikes/MED-005.md` in its options.

#### Evidence
- none

### MED-005 · Measure VA-API and Vulkan Video decode on reference machines
- Type: spike
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: GFX-001, MEM-024, HW-022
- Baseline: §17, §39, §54
- Invariants: I-045, I-061

V2 hardware decode cannot start without a measured VA-API versus Vulkan Video choice on the V1 AMD desktop and Intel laptop. This spike runs both retained Mesa paths into MemoryObjects on H-002 and H-004, records copies per stage by physical-page identity, and does not claim superiority (I-061). It does not build a native GPU driver stack (I-045).

<!-- covers: EXTRA-057, GAP-0300 -->

#### Out of scope
The Decision (MED-004). ComputeDevice wrapping (MED-017). B-046 publication (MED-028).

#### Acceptance criteria
- [ ] The report records VA-API and Vulkan Video decode of a fixed AV1 clip into MemoryObjects on H-002 and H-004, with copy count by physical-page identity at each stage.
- [ ] The report names which path failed to produce a GPU-compatible Frame on each machine, if any.
- [ ] The report states no performance number in prose; measurements live in the report tables and cite B-046 as the later standing metric.

#### Verification
- Report: `reports/spikes/MED-005.md` answers which path produced GPU-compatible Frames on H-002 and H-004, copies per stage, unsafe surface compared with software decode, and whether a per-GPU selection is required.
- Bench: B-046 method (copy count by physical-page identity) on H-002 and H-004; publish-only, no target.
- Review: GFX lead confirms both paths used retained Mesa, not a new driver.

#### Evidence
- none

### MED-006 · Measure libcamera versus V4L2 on the Intel laptop ISP
- Type: spike
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-008, HW-009
- Baseline: §7, §33, §55
- Invariants: I-009

GAP-0303 requires a libcamera-adoption Decision for laptop ISP pipelines. This spike reports whether V4L2 alone enumerates, streams and produces Frames from the built-in sensor on H-004, and what libcamera adds for that ISP. I-009 requires a recorded benefit before replacing V4L2.

<!-- covers: GAP-0303 -->

#### Out of scope
The Camera service model Decision (MED-001). UVC hot-plug (MED-029). Object<Device> (HW).

#### Acceptance criteria
- [ ] The report records V4L2-only capture from the H-004 built-in sensor: enumerate, start, one Frame, stop.
- [ ] The report records the same sequence through libcamera on H-004 and names ISP features V4L2-only could not drive.
- [ ] The report recommends which option set MED-001 must evaluate, without encoding the Decision.

#### Verification
- Report: `reports/spikes/MED-006.md` answers whether V4L2 alone streams the H-004 ISP, which libcamera pipelines are required, unsafe surface of each stack, and the recommended option list for the adr.
- Manual: capture sequence on H-004 with the sensor covered and uncovered, Frames present only when uncovered.
- Review: HW lead confirms the spike used Object<Device> and did not grant a V4L2 node to a native test Component.

#### Evidence
- none

### MED-007 · Decide native media pipeline versus GStreamer or FFmpeg
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: MED-009, GOV-020, MED-010, SDK-097
- Baseline: §11, §17, §51, §57
- Decision: D-0188
- Risks: R-068
- Threats: T-038
- Invariants: I-009, I-014

This Decision is the media baseline-gap scope: sandboxed decoder Components fed by a native pipeline versus retaining GStreamer or FFmpeg as the pipeline, with codecs as separately updatable Packages (GAP-0299). Media decoders are the flagship §11 isolation case. I-009 forbids replacing a mature pipeline without a measured benefit from MED-009. Native software never sees POSIX decoder APIs.

<!-- covers: GAP-0299, EXTRA-057 -->

#### Out of scope
Per-codec Package schema (MED-002). Hardware path choice (MED-004). Camera service model (MED-001). Patent policy (GOV-020).

#### Acceptance criteria
- [ ] Option A (native pipeline of sandboxed decoder Components exchanging MemoryObjects, codecs as separately updatable Packages), Option B (retain GStreamer as the pipeline with elements wrapped as Components) and Option C (retain FFmpeg as the in-Component library behind the same Package schema) are evaluated against T-038, I-009, I-014 and the pipeline-prototype report.
- [ ] The accepted option names the pipeline owner Component, how a codec Package updates without rebuilding the player, and that libavcodec and GStreamer are not public native APIs.
- [ ] The accepted option records that patent-encumbered codecs follow GOV-020 (R-068).
- [ ] Architecture review sign-off is recorded on the pull request.

#### Verification
- Review: architecture review recorded on the pull request, with MED, SEC and GOV leads named.
- Report: the Decision file lists at least two options, cites `reports/spikes/MED-009.md`, and names rejected options.

#### Evidence
- none

### MED-008 · Define Layer 2 decoder encoder Camera and Frame Interfaces
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: MED-007, MED-004, MED-001, IPC-035, IPC-012
- Baseline: §12, §14, §66
- Invariants: I-041

V1 freeze-candidate Layer 2 Interfaces for decoder, encoder, Camera and Frame must exist before V2 implementation and the V4 Layer 2 lock. They are registered with version identities in IPC-035. State is prototyped; versions lock at V4. Native software talks these Interfaces, not POSIX codec APIs.

<!-- covers: EXTRA-057 -->

#### Out of scope
V2 decoder and encoder Components (MED-024, MED-025). Version lock (MED-039). IDL compiler (IPC).

#### Acceptance criteria
- [ ] IDL files define versioned `Decoder`, `Encoder`, `Camera` and `Frame` Interfaces with methods for submit, complete, cancel and Frame transfer by MemoryObject Capability.
- [ ] Each Interface is listed in the Layer 2 registry with a version identity; CI rejects an unversioned change.
- [ ] Generated Rust stubs compile against IPC-012 and transfer a MemoryObject Capability in a Frame message on `qemu-x86_64`.
- [ ] Native sample code for the Interfaces imports no libavcodec, GStreamer or V4L2 headers.

#### Verification
- Unit: `media:tests/interfaces_*` on `qemu-x86_64`.
- Integration: registry listing and version-bump CI check on `qemu-x86_64`.
- Review: IPC lead confirms the Interfaces follow S-014 evolution rules as prototyped.

#### Evidence
- none

### MED-009 · Prototype sandboxed decoder Components exchanging MemoryObjects
- Type: spike
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: MEM-022, MEM-039, CMP-024, CMP-021, SDK-002
- Baseline: §11, §16, §17, §51
- Threats: T-038
- Invariants: I-014, I-063

GAP-0299 and EXTRA-057 need a measured native pipeline versus retained GStreamer versus retained FFmpeg before V2 codec Components. This spike builds three prototypes that decode a fixed AV1 clip into MemoryObjects, counts copies by physical-page identity, and records the Capability set of the decoder Component (T-038). ImageDecoder remains the §11 pattern; this spike generalises it to video.

<!-- covers: GAP-0299, EXTRA-057 -->

#### Out of scope
The framework Decision (MED-007). Production AV1/VP9 Packages (MED-024). Hardware decode (MED-005).

#### Acceptance criteria
- [ ] Three prototypes (native Component pipeline, GStreamer pipeline, FFmpeg-in-Component) decode a fixed AV1 clip to a MemoryObject Frame on H-001 and H-002.
- [ ] The report records copy count by physical-page identity and the decoder Capability set for each prototype.
- [ ] Each decoder Component in the native prototype holds no network, filesystem, microphone or Camera Capability (T-038).

#### Verification
- Report: `reports/spikes/MED-009.md` answers copies per stage, Capability set, crash isolation (malformed input kills only the decoder Component), update story for swapping a codec Package, and the option list for MED-007.
- Integration: malformed-input kill of the decoder Component on `qemu-x86_64` leaves the player Component running.
- Review: SEC lead confirms the native prototype matches T-038.

#### Evidence
- none

### MED-010 · Prototype decoder, encoder, Camera and Frame Interfaces
- Type: spike
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: IPC-012, MEM-010, SDK-002
- Baseline: none
- Explores: S-036

Prototype decoder, encoder, Camera and Frame Interfaces as sandboxed Components using MemoryObject payloads so MED-007 is informed by running code. Surface S-036 remains open. Native software never sees a POSIX codec device node.

#### Out of scope
The Decision (MED-007). Hardware video path Decision (MED-004). Freeze of S-036 (MED-039). Personality camera portal (LNX).

#### Acceptance criteria
- [ ] A prototype decoder Component consumes a MemoryObject and produces frames on H-002.
- [ ] A prototype Camera Interface grants a Capability that a Component without it cannot open.
- [ ] Surface S-036 remains `open` or `prototyped`, never `frozen`.

#### Verification
- Report: which objects belong in the native Interface versus a codec Package, how MemoryObject zero-copy is preserved, and which options MED-007 must evaluate.
- Integration: the prototype runs on `hw-h002`.

#### Evidence
- none

### MED-011 · Ship a first-party Camera capture application
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: MED-013, MED-027, MED-008
- Baseline: §7, §9.1, §56.5
- Threats: T-014
- Invariants: I-021

GAP-0303 requires a first-party Camera app. APP-051 decides the broader native set; this task ships the capture client that exercises `Capability<Camera>`: preview Frames, still capture to a UserSelected image, and a denied-grant path that shows no Frames. It runs against the virtual Camera on CI and the built-in sensor on H-004.

<!-- covers: GAP-0303 -->

#### Out of scope
First-party application set Decision (APP-051). Shell indicator chrome (APP-031). Chooser (STO).

#### Acceptance criteria
- [ ] The capture application, holding `Capability<Camera>`, displays preview Frames and writes a still image through a UserSelected grant on H-004 and on the virtual Camera on `qemu-x86_64`.
- [ ] Launching the same application without a Camera grant shows no Frames, records a typed denial, and allocates no sensor session (I-021).
- [ ] `os inspect` lists the application as a Camera holder while preview is running and not after it exits.
- [ ] The application Component imports no V4L2 headers and opens no device node.

#### Verification
- Integration: `media:tests/camera_app_*` on `qemu-x86_64` (virtual Camera) and `hw-h004`.
- Demo: V2 Camera-permission demo on H-004; grant, preview, revoke, preview stops.
- Review: APP lead confirms this client is not a substitute for APP-051.

#### Evidence
- none

### MED-012 · Instrument Camera cold start for the B-045 harness
- Type: benchmark
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: MED-013, BEN-038
- Baseline: §54
- Benchmarks: B-045
- Invariants: I-061

B-045 V2 publish includes Camera cold start. MED owns the Camera service the desktop-essentials harness measures; BEN owns the suite runner. This task instruments grant-to-first-Frame on H-004 and H-005 and emits the B-045 Camera series. No number appears in prose (I-061).

#### Out of scope
Suite publication and cross-OS tables (BEN-038). Camera service (MED-013).

#### Acceptance criteria
- [ ] Harness `bench:desktop-essentials` records Camera cold start as grant-to-first-Frame on H-004 and H-005.
- [ ] A report exists under `reports/benchmarks/B-045/` for those machines meeting the register target kind for V2 (publish).
- [ ] No MED description, criterion or report restates a numeric cold-start target.

#### Verification
- Bench: B-045 Camera series on H-004 and H-005; target per register.
- Integration: stage timestamps appear in the B-045 report skeleton under `reports/benchmarks/B-045/`.
- Review: BEN lead confirms the Camera series is not double-counted against other B-045 items.

#### Evidence
- none

### MED-013 · Mint per-session Camera Capabilities with in-use signalling
- Type: build
- Milestone: V2
- Status: todo
- Size: L
- Owner: none
- Depends on: MED-001, MED-008, CAP-036, CAP-001, HW-008
- Baseline: §7, §9.1, §11
- Threats: T-001, T-014
- Invariants: I-021

The Camera service mints per-session `Capability<Camera>`, never ambient, and emits in-use state for the shell indicator (EXTRA-059, GAP-0303, INV-0167). A Component without the grant receives `Error::Rights` and no Frames (I-021, T-014). Built-in sensors on H-004 and H-005 are in scope; UVC hot-plug waits for V3. Native software never receives a V4L2 node.

<!-- covers: EXTRA-059, GAP-0303, INV-0167 -->

#### Out of scope
Shell indicator rendering (APP-031, GFX-082). UVC Device enablement (HW-078). Personality portal (LNX-080). Virtual Camera (MED-027).

#### Acceptance criteria
- [ ] `Camera.open` mints a per-session `Capability<Camera>` and `os inspect camera` lists the holder, sensor identity and in-use flag on H-004 and H-005.
- [ ] A native Component with no Camera grant receives `Error::Rights`, allocates no handle, and produces no Frames (I-021).
- [ ] Revoking the Capability stops Frames within one Operation and clears the in-use flag.
- [ ] In-use audit events are visible to CAP-001 while any session is open.
- [ ] The service Component is the only native holder of the underlying Device Capability; clients hold only `Capability<Camera>`.

#### Verification
- Unit: `media:tests/camera_service_*` on `qemu-x86_64` and `hw-h004`.
- Integration: grant, stream, revoke on H-004 and H-005; denial path on `qemu-x86_64`.
- Demo: V2 Camera-permission prompt on H-004 with the in-use indicator driven by these events.
- Review: CAP lead confirms Camera rights are in CAP-036.

#### Evidence
- none

### MED-014 · Integrate Camera encode with the audio echo-cancellation path
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: MED-013, MED-025, AUD-020, AUD-009
- Baseline: §11, §17, §22
- Corpora: C-004
- Threats: T-014

L3 corpus video conferencing needs Camera Frames encoded beside AUD echo cancellation. AUD owns the capture and playback graph and mix-monitor reference; MED owns Camera Frames and the encoder Component. A conferencing session Component holds Camera, encoder and AudioStream Capabilities and no filesystem or unrelated Device rights. Required by V3-D05 (Windows game and Linux IDE on the AMD laptop).

#### Out of scope
Echo-cancellation filter (AUD-020). Personality conferencing apps (LNX). Screen-share into a call (MED-035).

#### Acceptance criteria
- [ ] A native conferencing session on H-004 captures Camera Frames, encodes them in the sandboxed encoder, and plays far-end audio through AudioStream while AUD echo cancellation is enabled.
- [ ] The encoder Component holds input Frames and output bytes only; it holds no Camera, microphone, network or filesystem Capability (T-014).
- [ ] Muting Camera revokes the session's Camera Capability and stops encoded video without restarting the audio graph.
- [ ] The L3 conferencing scenario probe records Camera grant, in-use flag and encoder Component identity.

#### Verification
- Integration: `media:tests/conferencing_pipeline_*` on `hw-h004`.
- Compat: C-004 conferencing entries score Camera and encode through this path on H-004.
- Demo: V2 conferencing demo on H-004 with Camera preview and encoded outbound video.
- Review: AUD lead confirms mix-monitor reference is consumed, not reimplemented.

#### Evidence
- none

### MED-015 · Isolate container demux Components for MP4 WebM and Matroska
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: MED-007, MED-002, MED-008, CMP-024
- Baseline: §11, §51
- Threats: T-038
- Invariants: I-014

Demux is an isolated Component ahead of decode so container parsers cannot escape into the player (T-038). MP4, WebM and Matroska demux Packages follow the codec schema: input bytes, output elementary-stream MemoryObjects, ResourceDomain budget, no network or filesystem. Malformed containers kill only the demux Component. Required by the design critique: container parsers isolated from the player so a malformed container cannot escape (T-038).

#### Out of scope
Decoder Components (MED-024). Playback clock (MED-021). Fuzz fleet (MED-031).

#### Acceptance criteria
- [ ] Demux Packages for MP4, WebM and Matroska install and, given a matching container MemoryObject, emit elementary-stream MemoryObjects on `qemu-x86_64` and H-002.
- [ ] Each demux Component holds only input bytes, output streams and its ResourceDomain; `os inspect` shows no network, filesystem, Camera or microphone Capability.
- [ ] A malformed container fixture terminates the demux Component with a typed exit cause and leaves the player Component running.
- [ ] An unknown container brand returns a typed error and allocates no demux Component.

#### Verification
- Unit: `media:tests/demux_*` on `qemu-x86_64` and `hw-h002`.
- Integration: malformed MP4/WebM/Matroska fixtures on `qemu-x86_64`.
- Fuzz: `media:fuzz/demux` one hour nightly without panic of any process other than the demux Component.
- Review: SEC lead confirms the Capability set matches T-038.

#### Evidence
- none

### MED-016 · Negotiate pixel format color range and HDR metadata on Frames
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: MED-017, MED-008, GFX-063, GFX-005
- Baseline: §16, §17, §39
- Invariants: I-063, I-084

GAP-0300 zero-copy Frames to the compositor require negotiated pixel format, color range and HDR metadata without copies. V2 HDR and VRR desktop gates consume this metadata; the V4 HDR video path completes it on every HDR-capable Tier 1 machine. Explicit GPU synchronization stays mandatory (I-084).

<!-- covers: GAP-0300 -->

#### Out of scope
Compositor HDR output (GFX-068). V4 HDR video completion (MED-038). Color pipeline Decision (GFX-063).

#### Acceptance criteria
- [ ] A Frame MemoryObject carries pixel format, color range and HDR metadata fields inspectable via `os inspect frame` after hardware decode on H-002.
- [ ] Transferring that Frame to the compositor does not copy pages when format and color range match the Surface (physical-page identity on H-002).
- [ ] A mismatch returns a typed negotiate error and allocates no extra Frame copy unless the caller requests an explicit convert Operation.
- [ ] Native software never sees a DRM fourcc or Wayland color protocol type on the Frame Interface.

#### Verification
- Unit: `media:tests/frame_formats_*` on `qemu-x86_64` and `hw-h002`.
- Integration: decode-to-compositor transfer with matching and mismatching metadata on H-002.
- Review: GFX lead confirms metadata matches GFX-063.

#### Evidence
- none

### MED-017 · Expose hardware video decode through ComputeDevice
- Type: build
- Milestone: V2
- Status: todo
- Size: L
- Owner: none
- Depends on: MED-004, MED-003, MED-008, HET-015, GFX-056, MEM-024
- Baseline: §17, §37, §39, §57
- Invariants: I-045, I-063

GAP-0300 V2 deliverable: VA-API and Vulkan Video on retained Mesa, decoded Frames as MemoryObjects transferred zero-copy to the compositor, not a native GPU stack (I-045). Decode is dispatched through ComputeDevice on V2 target GPUs (H-002, H-004, H-005). Native software holds `Capability<ComputeDevice>` and the decoder Interface, never a DRM node.

<!-- covers: GAP-0300, INV-1112 -->

#### Out of scope
NVIDIA desktop (MED-032). Encode (MED-018). ComputeDevice ABI (HET). Compositor import (GFX).

#### Acceptance criteria
- [ ] Hardware decode of a fixed AV1 clip through ComputeDevice produces GPU-compatible Frames on H-002, H-004 and H-005 using the path MED-004 selected.
- [ ] Physical-page identity is preserved from decoder output to compositor import on H-002 (I-063).
- [ ] Decode without `Capability<ComputeDevice>` returns `Error::Rights` and allocates no Frame.
- [ ] `os inspect` shows the ComputeDevice, decoder Component and Frame objects; no DRM fd is in the client Component.
- [ ] Software 4K decode is not required for this task; absence of a hardware path on a machine is recorded as a skip, not a software fallback rewrite.

#### Verification
- Integration: `media:tests/hw_video_decode_*` on `hw-h002`, `hw-h004` and `hw-h005`.
- Demo: V2 hardware-decode playback on H-002 with Frames reaching the compositor.
- Bench: B-046 copy-count stages on H-002; target per register (publish at V2).
- Review: HET and GFX leads confirm ComputeDevice wrapping of retained Mesa, not a new driver.

#### Evidence
- none

### MED-018 · Expose hardware video encode through ComputeDevice
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: MED-004, MED-008, HET-015, MEM-024
- Baseline: §17, §37, §39
- Invariants: I-045

GAP-0300 encode half and EXTRA-060 screen-recording encode: hardware encode of MemoryObject Frames through ComputeDevice on V2 target GPUs. Retained Mesa encode (VA-API or Vulkan Video per MED-004), not a native GPU stack (I-045).

<!-- covers: GAP-0300, EXTRA-060 -->

#### Out of scope
Screen-capture Capability (GFX-061). Encoder Component packaging (MED-025). Screen-record tool UI (GFX-084).

#### Acceptance criteria
- [ ] Hardware encode of a fixed Frame sequence through ComputeDevice produces a compressed bitstream MemoryObject on H-002 and H-004.
- [ ] Encode without `Capability<ComputeDevice>` returns `Error::Rights` and allocates no bitstream.
- [ ] The encoder path does not grant the caller a DRM node; `os inspect` shows ComputeDevice and encoder Component only.
- [ ] Bitstream container wrapping is not required here; elementary-stream output is sufficient.

#### Verification
- Integration: `media:tests/hw_video_encode_*` on `hw-h002` and `hw-h004`.
- Review: HET lead confirms encode uses the same ComputeDevice backend as decode.

#### Evidence
- none

### MED-019 · Restart the media and Camera services with client rebind
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: MED-013, MED-024, SVC-009
- Baseline: §32
- Invariants: I-037

§32 recovery so V2 conferencing and Camera-permission demos survive a codec or Camera service crash. Clients disconnect, rebind by Interface identity, retry and restore-state. SVC owns the supervisor; MED owns media and Camera restore-state.

#### Out of scope
Supervisor (SVC-009). Compositor rebind (GFX). Audio server rebind (AUD-005).

#### Acceptance criteria
- [ ] Killing the Camera service 20 consecutive times on H-004 restarts it under the supervisor; a client with a Camera grant rebinds and receives Frames without exiting.
- [ ] Killing a decoder Component during playback restarts that Component; the playback session rebinds and continues without exiting the player.
- [ ] `os inspect service` shows restart count increment and remaining budget after each kill.
- [ ] A client without restore-state support receives a typed disconnect and does not crash the session.

#### Verification
- Integration: `media:tests/media_rebind_*` on `hw-h004` and `qemu-x86_64`.
- Demo: V2 Camera-permission demo survives Camera service kill on H-004.
- Review: SVC lead confirms rebind uses Interface identity from SVC-009.

#### Evidence
- none

### MED-020 · Grant Linux-Personality Camera access only via native Capabilities
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: MED-013, LNX-080, LNX-018, SEC-044
- Baseline: §3, §9.1, §46
- Corpora: C-004
- Threats: T-002, T-014
- Invariants: I-021, I-072

V2 demo of a Camera permission prompt for a Linux-personality video app: personality apps receive no V4L2 device node, only a minted `Capability<Camera>` through the portal (I-072). LNX owns the portal; MED mints and enforces the Capability. Denial yields no Frames and no crash (T-002).

<!-- covers: EXTRA-059 -->

#### Out of scope
Portal implementation (LNX-080). Prompt UI (SEC-044). Default Capability bundle (LNX-013).

#### Acceptance criteria
- [ ] A Linux-personality video application that is granted Camera through the portal receives Frames and appears as a Camera holder in `os inspect`.
- [ ] The same application denied Camera receives no Frames, does not crash, and the enclosing Component holds no V4L2 device node (I-072).
- [ ] Ambient `/dev/video` open from that application fails; the only path is the portal-minted Capability (I-021).
- [ ] In-use state is set for the personality application while Frames flow.

#### Verification
- Integration: `media:tests/personality_camera_*` on `hw-h004`.
- Compat: C-004 conferencing and video-call entries exercise grant and deny on H-004.
- Demo: V2 Camera permission prompt for a Linux-personality video app on H-004.
- Review: LNX lead confirms no V4L2 node is in the personality sandbox.

#### Evidence
- none

### MED-021 · Implement media clock and audio-video synchronization
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: MED-024, AUD-006, AUD-010, MED-008
- Baseline: §17, §18

Native media players at V2 need a playback clock that slaved video Frame presentation to AudioStream time. MED owns the clock and Frame timestamps; AUD owns the audio path. Native software never sees POSIX timers as the media clock API. Required by the MED scope: "A native playback session binds play, pause, seek and now-playing metadata for the shell."

#### Out of scope
Audio mixer and resampling (AUD). Shell media controls (APP-034). Hardware decode (MED-017).

#### Acceptance criteria
- [ ] A playback session on H-002 presents decoded Frames against AudioStream time and reports clock source, rate and offset via `os inspect media-clock`.
- [ ] Pausing the session stops Frame presentation and audio together; resume continues from the same media timestamp.
- [ ] Seek to a named timestamp flushes decoder and audio queues and presents the Frame nearest that timestamp.
- [ ] Clock Interface methods are typed Operations; native clients do not call POSIX `clock_gettime` as the media API.

#### Verification
- Integration: `media:tests/playback_clock_*` on `hw-h002` and `qemu-x86_64`.
- Manual: A/V sync procedure on H-002 with a clip that has a clap at a known timestamp.
- Review: AUD lead confirms AudioStream time is the slave source.

#### Evidence
- none

### MED-022 · Expose a native playback session for shell media controls
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: MED-008, MED-021, MED-024
- Baseline: §12, §32, §66

APP GAP-0304 owns shell UI and MPRIS; MED owns the playback session object that play, pause, seek and now-playing metadata bind to. The session is a Layer 2 Interface so the shell and a native player share one object. Personality MPRIS mapping stays in LNX.

<!-- covers: GAP-0304 -->

#### Out of scope
Shell media keys and now-playing UI (APP-034). MPRIS bridge (LNX). Audio focus policy (AUD-015).

#### Acceptance criteria
- [ ] `PlaybackSession` Interface methods play, pause, seek and now-playing are implemented and listed in the Layer 2 registry.
- [ ] A native player publishes a session; `os inspect playback-session` shows state, media title and position while playing on H-002.
- [ ] Two sessions on one user do not share state; pausing one does not pause the other.
- [ ] The session holds no Camera, screen-capture or filesystem Capability of its own.

#### Verification
- Unit: `media:tests/playback_session_*` on `qemu-x86_64` and `hw-h002`.
- Integration: player plus inspect on H-002.
- Review: APP lead confirms the session object is the bind target for APP-034.

#### Evidence
- none

### MED-023 · Decide Widevine L3 Personality path and native CDM non-goals
- Type: adr
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: APP-019, GOV-020
- Baseline: §3, §48, §56.5, §57
- Decision: D-0189
- Invariants: I-071

GAP-0302 placed at V2 so the native player never grows a CDM. Options include personality Widevine L3, no DRM at 1.0, and native or Layer 1 secure-path CDM. I-071 forbids circumventing DRM; L1 secure-path requires vendor trust chains the project cannot obtain. Native CDM is a rejected option for 1.0.

<!-- covers: GAP-0302 -->

#### Out of scope
V3 browser verification (MED-036). 1.0 non-goal publication (MED-041). Browser strategy (APP-019).

#### Acceptance criteria
- [ ] Option A (Widevine L3 only through the Linux-personality browser CDM), Option B (no DRM playback at 1.0) and Option C (native CDM or Layer 1 secure-path) are evaluated against I-071, vendor trust-chain access and APP-019.
- [ ] The accepted option states that the native player and native decoder Components ship no CDM, and that circumventing DRM is not a path (I-071).
- [ ] The accepted option names which 1.0 compatibility statement MED-041 must publish.
- [ ] Architecture and GOV review sign-off is recorded on the pull request.

#### Verification
- Review: MED, LNX, APP and GOV leads sign off on the pull request that accepts the decision file.
- Manual: the Decision file lists at least two options and names rejected options.

#### Evidence
- none

### MED-024 · Ship isolated decoder Components for royalty-free codecs
- Type: build
- Milestone: V2
- Status: todo
- Size: L
- Owner: none
- Depends on: MED-007, MED-002, MED-008, MED-015, CMP-024, SCH-008
- Baseline: §11, §17, §51
- Risks: R-068
- Threats: T-038
- Invariants: I-014, I-021

Implements the GAP-0299 native pipeline and EXTRA-058 isolated Packages for AV1 and VP9 software decode. Each decoder holds only input bytes, output Frames and a ResourceDomain budget per §11. An exploit compromises the decoder, not the player (T-038). Patent-encumbered software decode is not shipped here (R-068).

<!-- covers: EXTRA-058, GAP-0299 -->

#### Out of scope
Hardware decode (MED-017). Encumbered codecs (MED-033). Demux (MED-015). Fuzz program (MED-031).

#### Acceptance criteria
- [ ] AV1 and VP9 software-decode Packages install and decode a fixed clip to MemoryObject Frames on `qemu-x86_64` and H-002.
- [ ] `os inspect` on the decoder Component shows only input bytes, output Frames and ResourceDomain membership; no network, filesystem, Camera, microphone or package-install Capability (I-021, T-038).
- [ ] A malformed bitstream terminates the decoder Component with a typed exit cause and leaves the player running.
- [ ] Exceeding the decoder ResourceDomain memory budget returns a typed error and does not enlarge the player domain.
- [ ] No H.264 or HEVC software decoder Package is present in the default image.

#### Verification
- Unit: `media:tests/sandboxed_decoder_*` on `qemu-x86_64` and `hw-h002`.
- Integration: AV1 and VP9 clips plus malformed fixtures on `qemu-x86_64` and H-002.
- Demo: V2 native playback of an AV1 clip on H-002 through isolated decode.
- Review: SEC and GOV leads confirm Capability set and default-image codec list.

#### Evidence
- none

### MED-025 · Ship isolated encoder Components with per-codec rights
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: MED-007, MED-002, MED-008, CMP-024
- Baseline: §11, §51
- Threats: T-038
- Invariants: I-021

Screen-recording and conferencing encode need isolated encoder Packages with no network or filesystem rights (EXTRA-058). Each encoder holds input Frames, output bytes and a ResourceDomain budget. Hardware encode is a ComputeDevice Capability the encoder may hold; it still has no Camera or screen-capture grant of its own.

<!-- covers: EXTRA-058 -->

#### Out of scope
Hardware encode path (MED-018). Screen capture (GFX). Conferencing session (MED-014).

#### Acceptance criteria
- [ ] An AV1 encoder Package accepts Frame MemoryObjects and emits a compressed bitstream on `qemu-x86_64` and H-002.
- [ ] The encoder Component holds no network, filesystem, Camera, microphone or screen-capture Capability (I-021, T-038).
- [ ] Encoding without an input-Frame Capability returns `Error::Rights` and allocates no bitstream.
- [ ] Malformed Frame metadata terminates the encoder Component and leaves the caller running.

#### Verification
- Unit: `media:tests/sandboxed_encoder_*` on `qemu-x86_64` and `hw-h002`.
- Integration: conferencing and screen-record callers on H-002 once those tasks land; encoder-only fixtures on `qemu-x86_64` now.
- Review: SEC lead confirms the Capability set.

#### Evidence
- none

### MED-026 · Encode screen-capture MemoryObjects through codec Components
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: MED-025, MED-018, GFX-061
- Baseline: §9.1, §17, §40
- Threats: T-014
- Invariants: I-085

EXTRA-060: encode Frames obtained only through the explicit screen-capture Capability. GFX owns capture and tools; MED owns the encoder path. An encoder invoked without a capture Capability receives `Error::Rights` and no Frames (I-085).

<!-- covers: EXTRA-060 -->

#### Out of scope
Capture Capability and recorder UI (GFX-061, GFX-084, APP-038). Live conferencing share (MED-035).

#### Acceptance criteria
- [ ] Frames minted by a screen-capture Capability encode through the sandboxed encoder to a bitstream MemoryObject on H-002.
- [ ] Invoking encode without a screen-capture Capability returns `Error::Rights`, allocates no bitstream, and produces a black or denied Frame if capture was also missing (I-085).
- [ ] The encoder Component does not itself hold screen-capture; it receives Frames already captured.
- [ ] `os inspect` shows capture holder and encoder as distinct Components while recording.

#### Verification
- Integration: `media:tests/screen_record_encode_*` on `hw-h002`.
- Demo: V2 screen recording on H-002 produces a playable bitstream through MED encode.
- Review: GFX lead confirms capture remains GFX-owned (S-034).

#### Evidence
- none

### MED-027 · Provide a virtual Camera source for conferencing and tests
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: MED-013
- Baseline: §7, §9.1
- Threats: T-014
- Invariants: I-021

GAP-0303 virtual Camera support: a capability-scoped virtual source for conferencing tests and CI without a physical sensor. The virtual source still requires `Capability<Camera>`; it is not an ambient loopback device (I-021).

<!-- covers: GAP-0303 -->

#### Out of scope
Physical sensors (MED-013). Personality v4l2loopback (LNX). Conferencing encode (MED-014).

#### Acceptance criteria
- [ ] A virtual Camera Device can be created in tests, minted as `Capability<Camera>`, and produces Frames from a supplied MemoryObject on `qemu-x86_64`.
- [ ] Opening the virtual Camera without a grant returns `Error::Rights` and no Frames (I-021).
- [ ] In-use state is set while a virtual session is open, matching physical Camera signalling.
- [ ] Native tests do not depend on a physical sensor for Camera grant, deny and revoke paths.

#### Verification
- Unit: `media:tests/virtual_camera_*` on `qemu-x86_64`.
- Integration: camera-app and camera-service denial suites use the virtual source on `qemu-x86_64`.
- Review: SEC lead confirms virtual Camera is not an ambient bypass of T-014.

#### Evidence
- none

### MED-028 · Publish the Zero-copy media path harness for B-046
- Type: benchmark
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: MED-017, MED-024, MEM-039
- Baseline: §17, §54
- Benchmarks: B-046
- Invariants: I-061, I-063

B-046 V2 publish gate. Measures copies per stage on the §17 storage-or-NIC to decoder to GPU to scanout path by physical-page identity. MED owns the media-path stages; BEN owns the runner and cross-OS tables. No superiority claim without the report (I-061). Required by V4-G17 (Prior benchmarks within the V4 regression band).

#### Out of scope
Register ownership and Linux dma-buf baseline tables (BEN-045). Hardware decode feature (MED-017).

#### Acceptance criteria
- [ ] Harness `bench:zero-copy-media` records copy count by physical-page identity at NIC-or-storage, decoder, GPU and scanout stages on H-002.
- [ ] A report exists under `reports/benchmarks/B-046/` for H-002 meeting the register target kind for V2 (publish).
- [ ] The report cites B-046 and states no superiority claim.

#### Verification
- Bench: B-046 on H-002; target per register.
- Integration: stage series appear in the B-046 report skeleton.
- Review: BEN lead confirms stage names match the register method.

#### Evidence
- none

### MED-029 · Survive UVC Camera hot-plug without restarting the service
- Type: build
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: MED-013, HW-078
- Baseline: §7, §32, §33
- Invariants: I-037

V3 cameras, webcams and USB classes: the Camera service enumerates and drops UVC devices on hot-plug without losing other sessions. HW enables UVC as Object<Device>; MED mints and revokes `Capability<Camera>` as devices appear and disappear.

<!-- covers: GAP-0303 -->

#### Out of scope
UVC Device objects (HW-078). Multi-user isolation (MED-034).

#### Acceptance criteria
- [ ] Plugging a UVC camera while another Camera session is open mints a new `Capability<Camera>` for the new device without restarting the service on H-004.
- [ ] Unplugging a UVC camera revokes its outstanding Capabilities, stops those Frames, and leaves other sessions running.
- [ ] `os inspect camera` lists only currently attached sensors after each plug and unplug cycle.
- [ ] Native clients never see a V4L2 add/remove event; they see Capability mint and revoke.

#### Verification
- Integration: `media:tests/camera_hotplug_*` on `hw-h004` with a UVC fixture from HW-078.
- Manual: 20 plug/unplug cycles on H-004; other session Frames continue.
- Review: HW lead confirms Device add/remove is the only input to MED.

#### Evidence
- none

### MED-030 · Run Camera capture on every V3 Tier 1 machine
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: MED-013, MED-029, HW-078, MED-027
- Baseline: §7, §62
- Invariants: I-021

V3 cameras and webcams across six Tier 1 machines: the Camera service opens built-in and UVC sensors and mints Capabilities on each. Machines without a sensor use the virtual Camera and record a skip for physical capture, not a silent pass.

<!-- covers: GAP-0303, EXTRA-059 -->

#### Out of scope
V4 RC suite (MED-037). HCL publication (REL-048).

#### Acceptance criteria
- [ ] Camera grant, one Frame, revoke and denial paths pass on H-002, H-004, H-005, H-006, H-007 and H-008, using built-in or UVC sensors where present.
- [ ] A machine with no physical sensor runs the virtual Camera paths and records `sensor: none` in the per-machine report; it does not report physical capture as passed.
- [ ] `os inspect camera` on each machine shows no ambient V4L2 node in native Components (I-021).
- [ ] Per-machine results are files the V4 HCL suite can consume.

#### Verification
- Integration: `media:tests/camera_tier1_*` on each V3 Tier 1 machine.
- Manual: physical capture on every laptop in the V3 set; UVC fixture on at least one desktop.
- Review: HW lead confirms Device objects exist for each reported sensor.

#### Evidence
- none

### MED-031 · Fuzz isolated decoder Components in continuous CI
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: MED-024, MED-015, BLD-042
- Baseline: §11, §51
- Threats: T-038

V3 continuous-fuzzing applied to §11 decoder isolation. Demux and decoder Packages consume in-tree corpora of AV1, VP9, MP4, WebM and Matroska. A crasher kills only the decoder or demux Component; the fuzzer supervisor stays up (T-038). BLD owns fleet plumbing.

#### Out of scope
Fuzz fleet and crasher SLA (BLD-063). Native ABI syzkaller (BLD).

#### Acceptance criteria
- [ ] Harnesses `media:fuzz/decoder` and `media:fuzz/demux` run in continuous CI on the BLD fleet with in-tree corpora.
- [ ] A crasher in decoder or demux is attributed to that Component; the supervisor and player fixtures do not abort.
- [ ] Corpus replay of every unique crasher is a regression test under `media:tests/decoder_fuzz_regression_*`.
- [ ] No decoder fuzz target opens network, filesystem, Camera or microphone Capabilities.

#### Verification
- Fuzz: `media:fuzz/decoder` and `media:fuzz/demux` on the BLD nightly fleet; duration per BLD-063.
- Unit: regression fixtures from unique crashers on `qemu-x86_64`.
- Review: BLD lead confirms the harnesses are registered in the continuous fleet.

#### Evidence
- none

### MED-032 · Enable hardware video decode on the NVIDIA Tier 1 desktop
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: MED-017, HET-023
- Baseline: §17, §37, §39, §56.1
- Invariants: I-045

GAP-0300 on the V3 NVIDIA Tier 1 desktop (H-006): hardware decode through the chosen VA-API or Vulkan Video path once NVIDIA joins the gated fleet. Uses the HW NVIDIA driver Decision and HET ComputeDevice enumeration, not a native NVIDIA driver rewrite (I-045).

<!-- covers: GAP-0300 -->

#### Out of scope
NVIDIA ComputeDevice enumeration (HET-023). Driver module signing (HW). V2 AMD/Intel decode (MED-017).

#### Acceptance criteria
- [ ] Hardware decode of a fixed AV1 clip through ComputeDevice produces GPU-compatible Frames on H-006.
- [ ] Decode without `Capability<ComputeDevice>` returns `Error::Rights` on H-006.
- [ ] `os inspect` shows the NVIDIA ComputeDevice and decoder Component; the client holds no DRM node.
- [ ] The path is the one MED-004 selected, or a recorded per-GPU selection allowed by that Decision.

#### Verification
- Integration: `media:tests/hw_video_decode_*` on `hw-h006`.
- Review: HW and HET leads confirm retained NVIDIA userspace, not a rewritten stack.

#### Evidence
- none

### MED-033 · Restrict patent-encumbered codecs to hardware decode Packages
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: MED-002, MED-017, GOV-020
- Baseline: §11, §28
- Risks: R-068
- Invariants: I-068

Implements GOV codec-shipping policy for the V3 public image so H.264 and HEVC arrive only as hardware decode Packages (EXTRA-058, R-068). Software decode Packages for those codecs are absent from the default image. Royalty-free software decode remains MED-024.

<!-- covers: EXTRA-058 -->

#### Out of scope
Patent policy Decision (GOV-020). Default-image allowlist (GOV). Hardware decode path (MED-017).

#### Acceptance criteria
- [ ] The V3 default image SBOM lists H.264 and HEVC only as hardware-passthrough Packages; no software-decode Package for those codecs is installed.
- [ ] Installing a fixture software H.264 decoder Package is rejected by the patent-class check from MED-002.
- [ ] Hardware H.264 decode through ComputeDevice succeeds on at least H-002 when the GPU provides it; otherwise the Package is absent and inspect records `unavailable`.
- [ ] License metadata for those Packages is on the userspace allowlist (I-068).

#### Verification
- Integration: SBOM and patent-class CI check on the V3 image build.
- Manual: `os inspect package` on H-002 lists patent class hardware-passthrough for any H.264/HEVC Package present.
- Review: GOV licensing sign-off recorded on the pull request.

#### Evidence
- none

### MED-034 · Scope Camera grants per user session for multi-user
- Type: build
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: MED-013, CAP-049
- Baseline: §9.1
- Threats: T-014
- Invariants: I-021

V3 multi-user exit criterion of separate capability stores: Camera in-use state and grants must not leak across user sessions. CAP owns the stores; MED scopes Camera sessions and in-use signals per session.

<!-- covers: EXTRA-059 -->

#### Out of scope
Per-user grant stores (CAP-049). Audio session isolation (AUD-024). Identity (SEC).

#### Acceptance criteria
- [ ] Two concurrent user sessions on H-004 each mint Camera Capabilities; `os inspect camera` in session A does not list session B holders.
- [ ] In-use state for session A does not appear in session B's indicator events.
- [ ] Revoking Camera in session A does not stop session B Frames.
- [ ] A Component in session A cannot open session B's Camera Capability (I-021).

#### Verification
- Integration: `media:tests/multi_user_camera_*` on `hw-h004`.
- Review: CAP lead confirms Camera sessions key off per-user grant stores.

#### Evidence
- none

### MED-035 · Encode screen-share Frames into a conferencing session
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: MED-014, MED-026, GFX-085
- Baseline: §9.1, §17, §40
- Corpora: C-004
- Invariants: I-085

EXTRA-060 live path and V3 demo of screen shared into a video call through the explicit screen-share Capability. Capture stays GFX; encode and the conferencing session stay MED. No capture Capability means no shared Frames (I-085).

<!-- covers: EXTRA-060 -->

#### Out of scope
Screen-share picker (GFX-085). Echo cancellation (AUD). Personality ScreenCast portal (LNX-081).

#### Acceptance criteria
- [ ] A conferencing session on H-004 encodes screen-share Frames obtained through the screen-share Capability into the outbound encoder.
- [ ] Denying screen-share leaves Camera (if granted) flowing and encodes no screen Frames (I-085).
- [ ] The encoder Component still holds no screen-capture Capability of its own.
- [ ] In-use state for screen share is distinct from Camera in-use in `os inspect`.

#### Verification
- Integration: `media:tests/screenshare_call_encode_*` on `hw-h004`.
- Compat: C-004 conferencing entries that exercise screen share on H-004.
- Demo: V3 screen shared into a video call on H-004.
- Review: GFX lead confirms Frames come from S-034 capture only.

#### Evidence
- none

### MED-036 · Verify Widevine L3 playback through the Linux-Personality browser
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: MED-023, LNX-056, APP-023, APP-019
- Baseline: §3, §46, §56.5
- Corpora: C-003
- Invariants: I-071

GAP-0302 V3 compatibility: Widevine L3 through the Linux-personality browser CDM is available when MED-023 accepted that option; native CDM is not shipped. Verification uses the personality browser from APP-019. Circumvention is not a path (I-071).

<!-- covers: GAP-0302 -->

#### Out of scope
Native CDM (rejected by MED-023). Browser product (APP, LNX). 1.0 non-goal text (MED-041).

#### Acceptance criteria
- [ ] If the accepted Decision is Option A, a C-003 Chromium-based browser entry plays a Widevine L3 test stream on H-002 through the personality CDM.
- [ ] No native decoder, player or Package on the image contains a CDM library.
- [ ] If the accepted Decision is Option B (no DRM), this task records that result and the L3 stream is documented as unsupported rather than forced green.
- [ ] No test decrypts Widevine content outside the vendor CDM (I-071).

#### Verification
- Compat: C-003 browser scenario for Widevine L3 on H-002, or an explicit skip citing MED-023 Option B.
- Integration: image scan that native Packages contain no CDM shared object.
- Review: LNX and GOV leads confirm the CDM is personality-only.

#### Evidence
- none

### MED-037 · Add Camera to the Tier 1 hardware test suite every RC
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: MED-030, HW-086, REL-048
- Baseline: §62, §63

V4 hardware gate: every Tier 1 machine passes Camera in the full hardware test suite each RC, including the NVIDIA desktop and both vendor laptops. HW owns the combined suite; MED supplies Camera cases (grant, Frame, revoke, denial, hot-plug where UVC is present).

#### Out of scope
Combined suite runner (HW-086). HCL publication (REL-048).

#### Acceptance criteria
- [ ] Camera cases from MED-030 are registered in HW-086 and run on every V4 Tier 1 machine each RC.
- [ ] H-006, H-007, H-008, H-009, H-010, H-011 and H-012 produce Camera results (physical or recorded `sensor: none`).
- [ ] A failing Camera case fails the suite for that machine; it is not skipped unless `sensor: none` is recorded.
- [ ] Per-machine Camera results are files REL-048 can attach.

#### Verification
- Integration: suite invocation on the V4 Tier 1 set includes Camera cases.
- Review: HW lead confirms Camera is a named class in the combined suite report.

#### Evidence
- none

### MED-038 · Deliver HDR decoded Frames Zero-copy to the compositor
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: MED-016, MED-017, GFX-068
- Baseline: §17, §39
- Invariants: I-063, I-084

GAP-0300 feature completion for V4 freeze: HDR decoded Frames reach the compositor by MemoryObject transfer on every HDR-capable Tier 1 machine. Pixel format, color range and HDR metadata are already negotiated (MED-016). Explicit GPU sync remains mandatory (I-084).

<!-- covers: GAP-0300 -->

#### Out of scope
Compositor HDR10 output (GFX-068). Reference display and colorimeter (LAB). Frame metadata schema (MED-016).

#### Acceptance criteria
- [ ] Hardware decode of a fixed HDR10 clip transfers Frames to the compositor by MemoryObject ownership on every HDR-capable V4 Tier 1 machine, with physical-page identity preserved (I-063).
- [ ] `os inspect frame` shows HDR metadata on those Frames matching the clip.
- [ ] Machines without HDR output record `hdr: none` and still decode the clip to SDR without claiming HDR pass.
- [ ] Native software never sees Wayland color-management protocol types on this path.

#### Verification
- Integration: `media:tests/hdr_video_path_*` on HDR-capable Tier 1 machines (H-002 with LAB HDR display, and peers the GFX HDR pipeline names).
- Demo: V4 HDR video playback on H-002.
- Review: GFX lead confirms zero-copy import of MED Frames into the HDR pipeline.

#### Evidence
- none

### MED-039 · Lock Layer 2 media Interface versions for the 1.x line
- Type: build
- Milestone: V4
- Status: todo
- Size: S
- Owner: none
- Depends on: MED-008, MED-022, MED-013, IPC-068, IPC-062, MED-010, MED-007
- Baseline: §66
- Freezes: S-036
- Invariants: I-040

V4 Layer 2 Interface versions locked: decoder, encoder, Camera, Frame and playback-session Interfaces are enumerated and locked for 1.x. IPC owns the lock mechanics; MED enumerates the media set. Layer 1 is not frozen here (I-040).

#### Out of scope
IPC version lock (IPC-068). Evolution test matrix (IPC-062). Layer 1 freeze (ABI).

#### Acceptance criteria
- [ ] A committed enumeration lists decoder, encoder, Camera, Frame and PlaybackSession Interface identities and versions served for 1.x.
- [ ] Old-client/new-service and new-client/old-service cases from IPC-062 pass for each listed Interface.
- [ ] CI rejects a change to a locked media Interface that does not bump the version per S-014.
- [ ] The enumeration names no Layer 1 surface as frozen by this task (I-040).

#### Verification
- Integration: evolution matrix entries for the five Interfaces on `qemu-x86_64`.
- Review: IPC lead confirms the enumeration is consumed by IPC-068.

#### Evidence
- none

### MED-040 · Re-run B-046 on every Tier 1 machine for V4 regression
- Type: benchmark
- Milestone: V4
- Status: todo
- Size: S
- Owner: none
- Depends on: MED-028, MED-038, BEN-059
- Baseline: §17, §54
- Benchmarks: B-046
- Invariants: I-061, I-088

V4 benchmark gate re-runs prior metrics including B-046. Regression versus V3 is the register target, not a prose number (I-088). MED re-runs the media-path harness on every Tier 1 machine; BEN owns the regression band.

#### Out of scope
Fleet-wide regression runner (BEN-059). Harness definition (MED-028).

#### Acceptance criteria
- [ ] B-046 reports exist under `reports/benchmarks/B-046/` for every V4 Tier 1 machine meeting the register target kind for V4 (regression versus V3).
- [ ] Machines that cannot share physical memory record `copies: n/a` with the reason; they do not invent a pass.
- [ ] No MED text restates the regression percentage; the register holds it (I-088).

#### Verification
- Bench: B-046 on every V4 Tier 1 machine; target per register.
- Review: BEN lead confirms reports satisfy the V4 regression clause.

#### Evidence
- none

### MED-041 · Publish native CDM and Layer 1 secure-path as 1.0 non-goals
- Type: docs
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: MED-023, MED-036, DOC-028
- Baseline: §56.5, §57
- Invariants: I-071

GAP-0302 and the 1.0 compatibility statement of what is not supported: vendor DRM, native CDM and Layer 1 secure-path playback stay out of 1.0. This document publishes that statement beside the personality Widevine L3 result. Circumvention remains forbidden (I-071).

<!-- covers: GAP-0302 -->

#### Out of scope
Compatibility guide hosting (DOC-028). Other 1.0 non-goals (APP-068). Widevine verification (MED-036).

#### Acceptance criteria
- [ ] A published 1.0 compatibility page states that native CDM, vendor hardware DRM and Layer 1 secure-path playback are not supported.
- [ ] The same page states the personality Widevine L3 result from MED-023 and MED-036.
- [ ] The page states that circumventing DRM is not provided (I-071).
- [ ] DOC-028 links this page as the media non-support section.

#### Verification
- Review: MED, LNX, GOV and DOC leads sign off on the pull request.
- Manual: the 1.0 compatibility guide contains the three non-support statements.

#### Evidence
- none

### MED-042 · Publish B-046 versus Linux and Windows on every Tier 1 machine
- Type: benchmark
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: MED-040, BEN-060
- Baseline: §17, §54
- Benchmarks: B-046
- Invariants: I-061, I-088

1.0 gate that every §54 metric is published versus Linux, Windows and macOS. B-046 is the media-path metric with GStreamer dma-buf as the Linux baseline. MED supplies the media-path reports; BEN owns the cross-OS pack. No superiority claim without the table (I-061).

#### Out of scope
Cross-OS pack and third-party repro scripts (BEN-063, BEN-060). Harness (MED-028).

#### Acceptance criteria
- [ ] B-046 reports exist for every 1.0 Tier 1 machine meeting the register target kind for 1.0 (regression versus V4) and include Linux GStreamer dma-buf and Windows comparison rows where dual-boot exists.
- [ ] macOS comparison is present where BEN-046 provides a comparable class, otherwise recorded as `class: none`.
- [ ] No MED announcement text contains a performance number; claims cite B-046 (I-088).

#### Verification
- Bench: B-046 on every 1.0 Tier 1 machine; target per register.
- Review: BEN lead confirms the tables are in the 1.0 publication pack.

#### Evidence
- none
