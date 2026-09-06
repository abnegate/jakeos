# AUD · Audio
- Prefix: AUD
- Lead: none
- Baseline: §7, §9.1, §22, §32, §33, §47, §49, §54, §61, §62, §63
- Baseline gap: No dedicated audio section; Object<AudioStream>, the restartable audio server, user-space audio devices and PipeWire compatibility are named in §7, §32, §33 and §47 without a mixer, hot-plug, codec-selection or echo-cancellation model.

<!-- roadmap:generated:begin summary -->
Tasks: 30 live, 0 done, 0 in-progress, 30 todo, 0 dropped. Ready: 0. Blocked: 30. Weighted: 0%.
<!-- roadmap:generated:end -->

## Scope
AUD owns the native audio object model and the user-space audio server: Object<AudioStream>, playback and capture rights, the LowLatency capture-to-playback path, the per-application mixer, sample-rate and channel conversion, default-device switching on jack, display and USB hot-plug, stream rebind across server restart and suspend, microphone grant enforcement, Bluetooth A2DP and HFP routing with codec selection, ducking and exclusive focus, the echo-cancellation and noise-suppression graph shared with MED, and the PipeWire and PulseAudio compatibility server that terminates on native streams. Native software holds Capability<AudioStream> only. Personality clients never become the native API. AUD also owns the B-028 and B-029 audio-latency harnesses, AudioStream conformance cases for the V4 Layer 2 lock, and the audio rows of the Tier 1 hardware suite.

## Out of scope
Kernel fork and retained ALSA driver inventory (KRN). User-space driver framework, Object<Device>, USB class matrix, Bluetooth host, pairing and BlueZ-versus-native placement (HW). Supervisor, service manifest and driver-host restart (SVC). Scheduling intent classes and IRQ affinity (SCH). Pinned MemoryObject and DMA properties (MEM). Channel batching, IDL and Layer 2 version lock (IPC). Capability rights encoding, audit log and per-user grant stores (CAP). Inspect CLI (OBS, SDK). Identity, login, Session, consent prompts and permissions UI (SEC). Shell volume widget, media-session chrome, in-use indicators and settings panels (APP, GFX). Camera service, codec Packages, playback clock and screen-recording encode (MED). Linux personality client sockets, portals and corpus scenario scripts (LNX). WASAPI and XAudio2 mapping (WIN). Suspend cycle harness (PWR). Loopback fixtures, radio peers and machine racking (LAB). Benchmark register ownership and cross-OS publication (BEN). HCL publication (REL). Codec patent policy (GOV). Compatibility-guide publishing (DOC). Layer 1 freeze adr (ABI). MIDI, Jack and pro-audio graphs (parked on LATER).

## Tasks

### AUD-001 · Ship PipeWire and PulseAudio compatibility on AudioStream
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: AUD-002, AUD-006, AUD-005, AUD-011, SVC-026, IPC-035
- Baseline: §7, §32, §47
- Threats: T-011
- Invariants: I-006, I-096

V1 requires PulseAudio and PipeWire compatibility so Linux GUI applications play through native audio objects (§47). This task is the AUD-side server: either PipeWire wraps AudioStream as the native server, or a PipeWire and PulseAudio server runs as a Linux-personality client of the native service, matching AUD-002. Native Components still hold only Capability<AudioStream>. LNX hosts the client socket.

<!-- covers: INV-0872, INV-0889 -->

#### Out of scope
Personality client socket and desktop integration (LNX-033). systemd-style daemon hosting (SVC-026). Native playback path (AUD-010). WASAPI mapping (AUD-022, WIN-052).

#### Acceptance criteria
- [ ] A Linux-personality PulseAudio client and a PipeWire client each complete playback and capture through AudioStream on H-002 and H-004.
- [ ] A native Component holding Capability<AudioStream> and a PipeWire client play simultaneously through the mixer for thirty minutes on H-002 with the B-028 glitch counter remaining zero.
- [ ] Killing the compatibility server rebinds personality clients without ending the session, visible in `os inspect service`.
- [ ] A native crate that links a PipeWire or PulseAudio client library fails ABI-003.
- [ ] `os inspect` lists personality streams as holders of AudioStream, not as a second device graph.

#### Verification
- Integration: `runtime:tests/aud/pipewire_pulse_bridge_*` on CI matrix entries `hw-h002` and `hw-h004`.
- Compat: C-003 GUI audio scenarios on H-002 and H-004 score through AudioStream.
- Review: LNX lead confirms the client socket remains inside the personality on the pull request.

#### Evidence
- none

### AUD-002 · Decide native AudioStream service versus PipeWire-as-native
- Type: adr
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: AUD-008, SDK-097
- Baseline: none
- Decision: D-0028
- Invariants: I-009, I-006

BASELINE.md has no audio section. This first adr records the native object model §7 names as Object<AudioStream>: whether PipeWire is the native server wrapping AudioStream, or a native AudioStream service runs with PipeWire as a Linux-personality client. The spike report on B-028 is an input. Replacing a mature audio server without a measured benefit violates I-009. Native software never sees POSIX device nodes.

<!-- covers: GAP-0539, INV-0166 -->

#### Out of scope
User-space versus in-kernel device classes (AUD-016). Personality client hosting (LNX-033). Rights encoding (CAP-010). Kernel object residency criteria (ABI-013).

#### Acceptance criteria
- [ ] Option A (PipeWire as the native server wrapping AudioStream) and Option B (native AudioStream service with PipeWire as a Linux-personality client) are evaluated with consequences drawn from `reports/spikes/AUD-008.md`.
- [ ] The accepted option names where Object<AudioStream> is minted, which process is the restartable audio server (§32), and that native crates never link PipeWire or ALSA.
- [ ] The accepted option lists S-025 as the Layer 2 surface it decides and does not freeze it.
- [ ] Architecture review sign-off is recorded on the pull request.

#### Verification
- Review: architecture review recorded on the pull request, with LNX and ABI leads named.
- Manual: the decision file lists at least two options, rejected options and a citation of the spike report.

#### Evidence
- none

### AUD-003 · Deny microphone use without an explicit Capability
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: AUD-006, CAP-001, SEC-009, SEC-006
- Baseline: §9.1
- Threats: T-001, T-014
- Invariants: I-021

A native Component must not automatically receive microphone access (§9.1). Capture Operations on AudioStream check the holder's rights word. A missing capture right returns `Error::Rights`, allocates no stream and writes a denial to the Capability audit log. Prompt UI and the in-use indicator chrome remain SEC, APP and GFX; this task emits the grant check and the in-use event they consume.

<!-- covers: INV-0205 -->

#### Out of scope
Consent prompt UI (APP-025, SEC-044). Shell in-use indicator (APP-031, GFX-082). ImageDecoder denial suite (SEC-021). Camera grants (MED).

#### Acceptance criteria
- [ ] Opening capture without a capture right returns `Error::Rights` and allocates no AudioStream handle, on CI matrix entries `qemu-x86_64` and `hw-h002`.
- [ ] The denial appears in the Capability audit log with Component identity and object type AudioStream.
- [ ] Starting capture with a valid right emits an in-use event; dropping the last capture stream clears it.
- [ ] ImageDecoder as launched in SEC-009 holds no microphone Capability and cannot open capture.

#### Verification
- Unit: `kernel:tests/aud/capture_rights_*` on `qemu-x86_64`.
- Integration: `runtime:tests/aud/mic_grant_denial_*` on `qemu-x86_64` and `hw-h002`.
- Review: SEC lead confirms T-014 and I-021 are enforced by the test, not by prompt chrome.

#### Evidence
- none

### AUD-004 · Build audio round-trip latency and glitch harness
- Type: benchmark
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: AUD-007, AUD-010, LAB-011, OBS-040, BEN-005, BEN-007
- Baseline: §54, §61
- Benchmarks: B-028

V1 publishes native low-latency round trip beside PipeWire on Linux on the same hardware. This harness implements `bench:audio-roundtrip`: loopback capture-to-playback and a ten-minute glitch run on the native path. BEN owns the register; SCH-028 remains the scheduler-jitter sibling. No superiority claim without the report.

#### Out of scope
Register ownership and cross-OS publication (BEN-007). Audio-callback jitter under intent class (SCH-028). Loopback cable and fixture (LAB-011).

#### Acceptance criteria
- [ ] Harness `bench:audio-roundtrip` records loopback p50 and p99 and a ten-minute glitch count on the native path on H-002 and H-004.
- [ ] The same run records a PipeWire-on-Linux baseline on the same machines.
- [ ] A report exists under `reports/benchmarks/B-028/` for H-002 and H-004 meeting the register target kind for V1.
- [ ] No AUD description, criterion or report restates a numeric latency target; the register holds the target.

#### Verification
- Bench: B-028 on H-002 and H-004; target per register.
- Integration: `runtime:tests/aud/bench_roundtrip_*` on `hw-h002` and `hw-h004`.
- Review: BEN lead confirms series names match the B-028 method.

#### Evidence
- none

### AUD-005 · Implement restartable audio server with stream rebind
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: AUD-006, SVC-015, SVC-009, IPC-028, CMP-028, BLD-020
- Baseline: §32
- Benchmarks: B-024
- Invariants: I-037

§32 names the audio server as a critical restartable service. AUD implements that server as a supervised Component whose clients disconnect, rebind and restore streams without ending the session. SVC owns the supervisor; this task supplies the audio restore-state path the V1 daily-driving gate needs. B-024 remains BEN's harness.

<!-- covers: INV-0594 -->

#### Out of scope
Supervisor, restart budgets and interface re-advertisement (SVC-015, SVC-009). Generated client rebind stubs (IPC-028). Fault-injection plumbing (BLD-020). Cross-service kill matrix (SVC-034).

#### Acceptance criteria
- [ ] Killing the audio server 100 consecutive times on H-002 restarts it under the supervisor and restores every open playback and capture stream with no client Component exit.
- [ ] The same kill loop passes 20 consecutive times on H-004.
- [ ] `os inspect service` shows the audio-server restart count increment and remaining budget after each kill.
- [ ] A client that does not implement rebind receives a typed disconnect and does not hang the mixer.

#### Verification
- Integration: `runtime:tests/aud/server_rebind_*` on `hw-h002` and `hw-h004`.
- Bench: B-024 audio-server series on H-002; target per register.
- Review: SVC lead confirms supervision versus stream restore-state split on the pull request.

#### Evidence
- none

### AUD-006 · Implement Object<AudioStream> with play and capture rights
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: AUD-002, ABI-005, CAP-036, CAP-011, OBS-019, IPC-035
- Baseline: §7, §9.1
- Invariants: I-006

V1 scope is native audio objects. After AUD-002, ship Object<AudioStream> and Capability rights for playback, capture and route so native Components hold typed streams rather than ALSA or PipeWire handles. `os inspect` shows live streams, devices and holders. The surface is S-025, prototyped, not frozen.

<!-- covers: INV-0166 -->

#### Out of scope
LowLatency callback path (AUD-007). Mixer gain (AUD-009). Personality bridges (AUD-001). Rights-word encoding (CAP-010). Inspect CLI rendering (SDK-007).

#### Acceptance criteria
- [ ] A native test Component plays and captures through Capability<AudioStream> on H-002 with no ALSA or PipeWire handle visible in `os inspect`.
- [ ] Playback without a play right and capture without a capture right each return `Error::Rights` and allocate no handle.
- [ ] `os inspect` lists live AudioStream objects, backing devices and holder Components on H-002.
- [ ] A forged or wrong-type handle targeting AudioStream fails with the typed error from ABI-009.
- [ ] Native crates in the audio server tree fail ABI-003 if they link libc ALSA or PipeWire client libraries.

#### Verification
- Unit: `kernel:tests/aud/audiostream_rights_*` on `qemu-x86_64`.
- Integration: `runtime:tests/aud/audiostream_play_capture_*` on `hw-h002`.
- Review: CAP lead confirms play, capture and route bits are in the object-rights registry.

#### Evidence
- none

### AUD-007 · Implement the LowLatency capture-to-playback audio path
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: AUD-006, SCH-026, SCH-042, MEM-038, IPC-043
- Baseline: §7, §22, §61

V1 requires a native low-latency path distinct from the compatibility server. Bind AudioStream callbacks to LowLatency intent (§22) over pinned MemoryObjects and batched Channel Operations. SCH owns the scheduler class; this task is the audio graph that path measures. Needed before the B-028 harness can run.

<!-- covers: INV-0407 -->

#### Out of scope
Intent class implementation (SCH-026, SCH-042). Pinning and reclaim (MEM-038). Channel batching (IPC-043). Callback jitter publication (SCH-028). Round-trip harness (AUD-004).

#### Acceptance criteria
- [ ] A loopback capture-to-playback graph runs under LowLatency intent on H-002 and H-004, with intent class visible in `os inspect`.
- [ ] Buffers on that path are pinned MemoryObjects; unpinning a live buffer fails the stream with a typed error and does not copy.
- [ ] The path is a distinct AudioStream graph from the PipeWire compatibility server, both enumerable in `os inspect`.
- [ ] Missing LowLatency grant falls back to the documented non-elevated class rather than silently promoting.

#### Verification
- Integration: `runtime:tests/aud/low_latency_loopback_*` on `hw-h002` and `hw-h004`.
- Bench: B-028 method smoke (one run, no gate) on H-002 confirming the native path is the one the harness will measure.
- Review: SCH lead confirms intent class tagging on the pull request.

#### Evidence
- none

### AUD-008 · Measure native AudioStream versus PipeWire round-trip latency
- Type: spike
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: KRN-017, HW-001, LAB-011, MEM-038, SCH-026
- Baseline: none
- Explores: S-025
- Invariants: I-009

GAP-0539 requires a latency measurement before choosing PipeWire as the native audio server or as a personality client of a native AudioStream service. V0.5 scopes audio out and V0 is capped at zero AUD tasks, so this spike sits at V1 with the object-model adr. The report compares both topologies on H-002 using the B-028 method. Cite the B-ID, never a number.

<!-- covers: GAP-0539 -->

#### Out of scope
The object-model Decision (AUD-002). Production AudioStream (AUD-006). Register ownership (AUD-004). User-space driver residency (AUD-019).

#### Acceptance criteria
- [ ] A report exists at `reports/spikes/AUD-008.md` comparing PipeWire wrapping AudioStream against a native AudioStream service with PipeWire as client, on H-002.
- [ ] Each topology is measured with the B-028 loopback method and a ten-minute glitch run; the report cites B-028 and states no superiority claim.
- [ ] The report names the mature mechanism each option preserves (I-009) and whether restart-and-rebind is possible without ending the session (§32).
- [ ] S-025 remains `open` or `prototyped`; this spike freezes nothing.

#### Verification
- Report: Which topology does B-028 favor on H-002, and by what published method rather than a number in this task? What glitch count does each topology record? What mature mechanism is preserved in each option? Which topology can restart and rebind without ending the session? What IOMMU or device-node exposure does each option create for native Components?
- Bench: B-028 method on H-002 for both topologies; target per register (publish).
- Review: ABI lead confirms S-025 is explored, not frozen.

#### Evidence
- none

### AUD-009 · Implement per-application volume mixer
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: AUD-006, AUD-005
- Baseline: none

The AUD workstream mixer sentence and EXTRA-022 require per-application gain on AudioStream, inspectable and overridable from settings UI. APP owns the widget; this task is the mixer graph. Include a mix-monitor stream so later screen-recording and echo-cancellation can take a far-end reference.

<!-- covers: EXTRA-022 -->

#### Out of scope
Settings volume widget and OSD (APP-041, APP-042). Ducking and exclusive focus (AUD-015). Echo-cancellation consumer (AUD-020). Screen-recording encode (MED-026).

#### Acceptance criteria
- [ ] Two native playback streams on H-002 have independently set gain; changing one does not change the other, visible in `os inspect`.
- [ ] A mix-monitor stream delivers the mixed far-end reference to a holder of the monitor Capability and is silent for a holder without it.
- [ ] Mixer state survives audio-server restart via the restore-state path in AUD-005.
- [ ] Setting gain on a stream the caller does not hold returns `Error::Rights`.

#### Verification
- Unit: `runtime:tests/aud/mixer_gain_*` on `qemu-x86_64`.
- Integration: `runtime:tests/aud/mixer_monitor_*` on `hw-h002`.
- Review: APP lead confirms the inspectable gain fields the settings widget will bind.

#### Evidence
- none

### AUD-010 · Ship native playback and capture on reference hardware
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: AUD-006, AUD-007, AUD-005, HW-015, HW-001, KRN-017
- Baseline: §7, §61
- Invariants: I-006, I-010

INV-1204 and the V1 developer-preview gate require audio output and input for native applications on the AMD desktop and Intel laptop. Uses retained kernel ALSA devices (KRN inventory) behind AudioStream; does not expose ALSA as a native API.

<!-- covers: INV-1204 -->

#### Out of scope
Retained-mechanism inventory (KRN-017). Laptop SKU bring-up (HW-015). PipeWire coexistence (AUD-001, AUD-014). Bluetooth (AUD-021).

#### Acceptance criteria
- [ ] A native test Component plays a fixture tone and captures a loopback buffer on H-002 and H-004 through Capability<AudioStream>.
- [ ] `os inspect` on those machines names the onboard output and input devices and their AudioStream holders.
- [ ] Native software has no ALSA device node, `snd_*` syscall or PipeWire socket in the test Component's inspect dump.
- [ ] Failure to open the device returns a typed error and leaves no leaked handle, on both machines.
- [ ] Native playback and PipeWire-compatibility playback run simultaneously for 60 minutes on H-004 with zero underruns recorded in `os inspect` (the V1-G06 duration).

#### Verification
- Integration: `runtime:tests/aud/native_play_capture_*` on `hw-h002` and `hw-h004`.
- Demo: V1 daily-driving session on H-004 includes native playback.
- Manual: operator confirms tone on internal speakers and headset jack on H-004.

#### Evidence
- none

### AUD-011 · Convert sample rate, format and Channel layout on mix
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: AUD-009, AUD-006
- Baseline: none

V1 native-plus-PipeWire simultaneous playback cannot hold a single hardware format without resampling and channel mapping in the mixer. This is required plumbing for the V1 functional audio gate, not a new codec framework. Required by V1-G06 (Native audio and PipeWire compatibility coexist).

#### Out of scope
Codec Packages and media decode (MED). Mixer gain and mix-monitor (AUD-009). Bluetooth codec selection (AUD-021). Patent policy (GOV-020).

#### Acceptance criteria
- [ ] Two AudioStream clients at different sample rates play simultaneously on H-002 through one hardware device, with the B-028 glitch counter remaining zero over a ten-minute run.
- [ ] Stereo and mono clients mix onto the device's channel layout; a 5.1 client downmixes rather than failing open.
- [ ] Format conversion (integer and float PCM) is visible as a mixer stage in `os inspect` and does not expose a native codec API.
- [ ] Asking the mixer to emit a layout the device cannot accept returns a typed error and allocates no extra handle.

#### Verification
- Unit: `runtime:tests/aud/resample_layout_*` on `qemu-x86_64`.
- Integration: `runtime:tests/aud/mixed_format_playback_*` on `hw-h002`.
- Bench: B-028 ten-minute glitch run with mixed-rate clients on H-002; target per register.

#### Evidence
- none

### AUD-012 · Restore playback and capture after suspend and resume
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: AUD-010, AUD-005, PWR-014, MEM-044, LAB-009
- Baseline: §32, §61
- Invariants: I-037

V1 exit requires audio functional after 200 laptop and 100 desktop suspend cycles. AUD reopens devices and rebinds streams; PWR owns the suspend cycle. No new device stack.

#### Out of scope
Suspend cycle harness and wake-source policy (PWR-014). Pinned buffer survival (MEM-044). Lab automation (LAB-009). Bluetooth reconnect after resume (AUD-021, V2).

#### Acceptance criteria
- [ ] After each of 200 suspend cycles on H-004, playback and capture reopen and a loopback round trip completes without restarting client Components.
- [ ] After each of 100 suspend cycles on H-002, the same check passes.
- [ ] A device that does not return after resume is reported as explicit degraded recovery in `os inspect service` (I-037), not as a hung stream.
- [ ] Pinned audio MemoryObjects remain mapped across the cycle per MEM-044.

#### Verification
- Integration: `runtime:tests/aud/resume_rebind_*` attached to the PWR cycle harness on `hw-h004` and `hw-h002`.
- Manual: operator suspends H-004 with lid close and confirms playback after resume once, in addition to the automated loop.

#### Evidence
- none

### AUD-013 · Switch default device on jack, display and USB hot-plug
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: AUD-010, AUD-005, HW-015
- Baseline: none

Headphone jack, HDMI/DP and USB hot-plug must retarget the default route without restarting clients. Bluetooth codec selection waits for V2 because V1 scopes Bluetooth out. Needed for Intel-laptop daily-driving with USB-C displays.

<!-- covers: EXTRA-022 -->

#### Out of scope
Bluetooth default-route and codec selection (AUD-021). Display mode policy (GFX-048). USB class enablement beyond audio endpoints (HW). Settings device picker chrome (APP-041).

#### Acceptance criteria
- [ ] Inserting a headphone jack on H-004 retargets the default playback route to the jack without restarting open AudioStream clients, observed in `os inspect`.
- [ ] Attaching an HDMI or USB-C display with audio on H-004 retargets default playback to that display; unplug restores the previous default.
- [ ] Attaching a USB audio device on H-002 and H-004 adds it to the device list and can be selected as default; removal does not kill clients, which rebind to the new default.
- [ ] A client holding an explicit device Capability is not moved when the default changes.

#### Verification
- Integration: `runtime:tests/aud/hotplug_default_*` on `hw-h004` and `hw-h002`.
- Manual: operator plugs headphones and a USB-C display on H-004 and confirms the inspect dump and audible route.
- Demo: V1 daily-driving on H-004 includes jack and USB-C display audio.

#### Evidence
- none

### AUD-014 · Verify Linux GUI applications play through native AudioStream
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: AUD-001, AUD-010, AUD-011, LNX-033, LNX-056
- Baseline: §47, §61
- Corpora: C-003
- Invariants: I-096

INV-0881 and V1-G06 require native and PipeWire streams to play together. Script L2 GUI entries so audio is scored through native AudioStream, not a leftover Linux-only path from V0.5.

<!-- covers: INV-0881, INV-1204 -->

#### Out of scope
L2 corpus definition and pass-rate (LNX-056). PipeWire client socket (LNX-033). Native-only playback (AUD-010). Windows audio (AUD-022).

#### Acceptance criteria
- [ ] Every C-003 GUI entry that declares audio scores that audio through AudioStream on H-002 and H-004, recorded in the corpus report.
- [ ] Simultaneous native playback and a C-003 browser or media-player entry run for thirty minutes on H-004 with the B-028 glitch counter remaining zero.
- [ ] No C-003 scenario remaining in the V1 gate uses a raw ALSA device node inside the personality that bypasses AudioStream.
- [ ] `os inspect` during a C-003 browser playback names the personality Component as an AudioStream holder.

#### Verification
- Compat: C-003 audio integration scoring on H-002 and H-004.
- Integration: `runtime:tests/aud/l2_gui_through_audiostream_*` on `hw-h004`.
- Review: LNX lead signs off that leftover V0.5 Linux-only paths are absent from the V1 gate scripts.

#### Evidence
- none

### AUD-015 · Apply ducking and exclusive-mode audio focus
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: AUD-009, AUD-007, SCH-048
- Baseline: none

V2 desktop preview and L3 conferencing need interruption policy on the mixer: duck, pause and exclusive LowLatency for games and calls. APP media-session UI consumes this; AUD enforces it on AudioStream. Required by the AUD scope: "ducking and exclusive focus".

#### Out of scope
Media-session shell controls (APP-034). Playback-session object (MED-022). Elevated-intent Capability UI (SCH-048). Echo-cancellation (AUD-020).

#### Acceptance criteria
- [ ] A documented duck policy lowers gain of background streams when a foreground stream with duck-others focus starts, restored when it ends, on H-002.
- [ ] Exclusive LowLatency focus pauses other playback streams and fails a second exclusive request with a typed error until the holder drops it.
- [ ] Focus changes are visible in `os inspect` and do not restart clients.
- [ ] A Component without the focus Capability cannot duck or exclude others (`Error::Rights`).

#### Verification
- Unit: `runtime:tests/aud/focus_duck_exclusive_*` on `qemu-x86_64`.
- Integration: `runtime:tests/aud/focus_call_game_*` on `hw-h002` and `hw-h005`.
- Review: APP lead confirms the media-session UI can bind the inspectable focus state.

#### Evidence
- none

### AUD-016 · Decide which audio device classes run in user space
- Type: adr
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: AUD-019, HW-016
- Baseline: §33
- Decision: D-0029
- Invariants: I-008, I-097

INV-0616 is evaluate-then-implement. Record which classes (USB, HDMI, Bluetooth offload, onboard) stay on retained ALSA versus a user-space driver hosted by SVC, using the spike report. Options must include keep-in-kernel for 1.0 and user-space USB. Kernel residency remains acceptable where latency, DMA safety or inherited-driver compatibility require it (I-008).

<!-- covers: INV-0616 -->

#### Out of scope
The measured prototype (AUD-019). Shipping the USB path (AUD-018). Driver framework (HW-029). Bluetooth host placement (HW-040).

#### Acceptance criteria
- [ ] Option A (keep every in-scope audio class in-kernel through 1.0) and Option B (user-space USB audio, other classes in-kernel) are evaluated against `reports/spikes/AUD-019.md` and I-008.
- [ ] The accepted option names each class (USB, HDMI, Bluetooth offload, onboard), its residency and the B-028 budget the implement task must not exceed.
- [ ] The accepted option states that AUD-018 is dropped if user-space USB is rejected.
- [ ] Architecture review sign-off is recorded on the pull request, with HW and SVC leads named.

#### Verification
- Review: architecture review recorded on the pull request.
- Manual: the decision file lists at least two options, per-class residency and a citation of the spike report.

#### Evidence
- none

### AUD-017 · Measure Bluetooth audio latency on the native path
- Type: benchmark
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: AUD-021, LAB-011, HW-037
- Baseline: §54, §62
- Benchmarks: B-029
- Risks: R-039

V2 publishes Bluetooth audio latency on the native path. Pairing cycles stay with HW-033. This harness records loopback latency per negotiated codec on the Intel and AMD laptops and publishes beside BlueZ plus PipeWire on Linux. No superiority claim without the report. Required by V2-G16 (Prior benchmarks show no unexplained regression): B-029 is published in that session.

#### Out of scope
Pairing and reconnect cycle counts (HW-033, HW-036). Register ownership (HW-033). Host stack (HW-035).

#### Acceptance criteria
- [ ] Harness `bench:bluetooth` audio-latency series records loopback p50 and p99 per negotiated codec on H-004 and H-005.
- [ ] The same run records a BlueZ-plus-PipeWire-on-Linux baseline on those machines.
- [ ] A report exists under `reports/benchmarks/B-029/` for H-004 and H-005 meeting the register target kind for V2, covering audio latency only.
- [ ] No AUD description restates a numeric latency target.

#### Verification
- Bench: B-029 audio-latency series on H-004 and H-005; target per register.
- Integration: `runtime:tests/aud/bench_bt_latency_*` on `hw-h004` and `hw-h005`.
- Review: BEN and HW leads confirm pairing series stay in the HW harness.

#### Evidence
- none

### AUD-018 · Host USB audio in user space within the latency budget
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: AUD-016, AUD-007, SVC-022, HW-029, HW-026
- Baseline: §33
- Invariants: I-008, I-038, I-097

INV-0616 implement-where-acceptable clause. If AUD-016 accepts user-space USB audio, ship it as a Capability-scoped Component on the V2 machines and keep B-028 inside the budget the adr records. If the adr rejects it, this task is dropped with that reason rather than shipping a rejected path.

<!-- covers: INV-0616 -->

#### Out of scope
The residency Decision (AUD-016). Driver framework and IOMMU DMA (HW-029, HW-026). Onboard and HDMI residency. USB class matrix beyond audio (HW-075).

#### Acceptance criteria
- [ ] A USB audio device on H-002 and H-004 is hosted by a supervised user-space Component holding Capability<Device>, not by an ambient device node in native software.
- [ ] B-028 on that path meets the budget recorded in AUD-016 on H-002 and H-004.
- [ ] Killing the USB-audio driver Component rebinds the stream or reports explicit degraded recovery; the session does not reboot.
- [ ] With the IOMMU off, user-space DMA is not enabled (I-038) and the device either refuses or falls back to the in-kernel path documented by the adr.

#### Verification
- Integration: `runtime:tests/aud/userspace_usb_*` on `hw-h002` and `hw-h004`.
- Bench: B-028 on the user-space USB path on H-002 and H-004; target per the adr-recorded budget.
- Review: HW lead confirms Device Capability and IOMMU posture on the pull request.

#### Evidence
- none

### AUD-019 · Measure user-space audio-driver latency against ALSA
- Type: spike
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: AUD-004, HW-029, HW-014, SVC-022
- Baseline: §33
- Invariants: I-008, I-038

INV-0616 and §33 require a measured latency budget before moving audio devices out of the kernel. Compare user-space USB audio against in-kernel ALSA on B-028. HW's USB-driver spike is the sibling, not a substitute.

<!-- covers: INV-0616 -->

#### Out of scope
The residency Decision (AUD-016). Production USB host (AUD-018). USB HID spike (HW-014). Driver-class criteria adr (HW-016).

#### Acceptance criteria
- [ ] A report exists at `reports/spikes/AUD-019.md` comparing user-space USB audio to in-kernel ALSA on H-002 using the B-028 method.
- [ ] The report cites B-028, states no superiority claim and names whether each class (USB, HDMI, Bluetooth offload, onboard) meets I-008 criteria.
- [ ] The report records IOMMU-off behavior for the user-space path (I-038).
- [ ] The spike freezes nothing.

#### Verification
- Report: Does user-space USB audio stay inside a budget AUD-016 can record versus in-kernel ALSA on H-002? Which classes fail DMA, interrupt or isolation criteria? What happens when the IOMMU is off? What SVC supervision cost is attributable to the driver Component versus the mixer?
- Bench: B-028 on H-002 for both paths; target per register (publish).
- Review: HW lead confirms the USB Device path used is the V1 framework, not a one-off.

#### Evidence
- none

### AUD-020 · Provide echo-cancellation and noise-suppression pipeline
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: AUD-009, AUD-007, AUD-015
- Baseline: none
- Corpora: C-004

Conferencing in the L3 corpus needs acoustic echo cancellation and noise suppression shared with MED. AUD owns the capture and playback graph and the mix-monitor far-end reference; MED owns video encode and the camera service.

<!-- covers: EXTRA-023 -->

#### Out of scope
Camera service and conferencing encode (MED-013, MED-014). Playback clock (MED-021). Mix-monitor minting (AUD-009). Screen-share Capability (SEC, GFX).

#### Acceptance criteria
- [ ] A capture stream can enable echo cancellation that takes the mix-monitor as far-end reference, on H-002 and H-005.
- [ ] Noise suppression is a separate, inspectable stage that can be enabled independently of echo cancellation.
- [ ] A conferencing client without the cancellation Capability receives raw capture and cannot enable the stage (`Error::Rights`).
- [ ] Enabling the pipeline does not grant camera, network or filesystem rights to the audio graph.
- [ ] MED-014 can attach to the mix-monitor without copying PCM into a POSIX pipe.

#### Verification
- Integration: `runtime:tests/aud/aec_ns_*` on `hw-h002` and `hw-h005`.
- Compat: C-004 conferencing entries that declare audio exercise the cancellation path on H-002.
- Review: MED lead confirms mix-monitor attachment versus camera encode split on the pull request.

#### Evidence
- none

### AUD-021 · Route Bluetooth audio with per-device codec selection
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: AUD-010, AUD-013, HW-040, HW-037, HW-035, HW-038, GOV-020
- Baseline: §32, §62
- Risks: R-039
- Invariants: I-097

V2 scope is Bluetooth audio via the user-space stack. HW owns BlueZ-versus-native placement and pairing. AUD routes A2DP and HFP onto AudioStream and exposes codec selection (SBC, AAC, aptX, LDAC, LC3) from EXTRA-022. Headset reconnect is a V2 Bluetooth gate.

<!-- covers: EXTRA-022 -->

#### Out of scope
Bluetooth host, pairing chooser and profile implementation (HW-035, HW-036, HW-037). Codec patent policy (GOV-020). Pairing-time publication (HW-033). Media keys and AVRCP chrome (APP-034).

#### Acceptance criteria
- [ ] An A2DP headset and an HFP headset each appear as AudioStream devices on H-004 and H-005 after HW pairing, with playback (A2DP) and capture-plus-playback (HFP) completing.
- [ ] Codec selection among the codecs the device and GOV-020 allow is inspectable and settable per device; an unavailable codec returns a typed error.
- [ ] Power-on of a known headset reconnects and restores the previous AudioStream route without restarting clients, for the reconnect cycles named in HW-038.
- [ ] Native software has no BlueZ socket or HCI device node; the holder is Capability<AudioStream> plus the per-device Bluetooth Capability HW mints.

#### Verification
- Integration: `runtime:tests/aud/bt_a2dp_hfp_*` on `hw-h004` and `hw-h005`.
- Demo: V2 laptop-day demo on H-004 or H-005 includes headset connect and audio.
- Review: HW lead confirms profile versus route split on the pull request.

#### Evidence
- none

### AUD-022 · Serve native AudioStream to the Windows Personality
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: AUD-006, AUD-007, AUD-001, LNX-034
- Baseline: §49, §56.2
- Corpora: C-007
- Threats: T-011
- Invariants: I-007, I-096

Windows applications must integrate with native audio (§49). AUD exposes AudioStream so WIN can map WASAPI and XAudio2 (WIN-052). Needed for W1 integration scoring and the V2 gaming proof. Native software still never sees Win32.

<!-- covers: INV-0925 -->

#### Out of scope
WASAPI and XAudio2 mapping (WIN-052). W1 gate pass-rate (WIN-051). Wine host (LNX-034). Exclusive-mode focus policy (AUD-015).

#### Acceptance criteria
- [ ] A Windows-personality fixture holding only the audio Capability granted to its enclosing Component plays and captures through AudioStream on H-002.
- [ ] A Windows-personality Component without that Capability receives `Error::Rights` at the native terminus and opens no WASAPI device inside the personality that bypasses AudioStream.
- [ ] W1 integration scoring on H-002 records audio through AudioStream for titles that declare audio.
- [ ] `os inspect` names the personality Component as the AudioStream holder during a W1 playback.

#### Verification
- Integration: `runtime:tests/aud/windows_terminus_*` on `hw-h002`.
- Compat: C-007 audio integration scoring on H-002.
- Review: WIN lead confirms WASAPI mapping remains in WIN-052.

#### Evidence
- none

### AUD-023 · Document native and Personality audio mapping
- Type: docs
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: AUD-001, AUD-022, AUD-005, AUD-003
- Baseline: §47, §49
- Invariants: I-096

V3 public documentation includes the compatibility guide. AUD writes how AudioStream, PipeWire, PulseAudio and WASAPI map, which grants are required, and how restart and rebind behave. DOC and SDK own publishing and IDL pages.

#### Out of scope
Compatibility-guide publishing (DOC-028). IDL page generation (DOC-010). Linux personality product guide (LNX-099). WASAPI implementation (WIN-052).

#### Acceptance criteria
- [ ] A mapping page exists covering AudioStream rights, PipeWire and PulseAudio as personality clients or wrapper (per AUD-002), and WASAPI/XAudio2 as WIN mappings.
- [ ] The page names the Capabilities required for playback, capture, route, mix-monitor and focus, and the `Error::Rights` cases.
- [ ] Restart, rebind and degraded-recovery behavior are documented with a pointer to `os inspect service`.
- [ ] No performance number appears; B-028 and B-029 are cited by ID.

#### Verification
- Review: DOC and LNX/WIN leads sign off on the pull request that lands the mapping page.
- Manual: the page is linked from the V3 compatibility guide outline and contains no POSIX-as-native instruction.

#### Evidence
- none

### AUD-024 · Isolate audio routing per login session
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: AUD-005, AUD-003, SEC-060, CAP-049, SVC-039
- Baseline: §9.1, §63
- Threats: T-001, T-014
- Invariants: I-021

V3 multi-user gate: two sessions with separate Capability stores must not share capture or playback. Route AudioStream and the mixer per login session; SEC owns identity, AUD owns the graph.

#### Out of scope
Session objects and login (SEC-060, SEC-028). Per-user grant stores (CAP-049). Session supervision trees (SVC-039). Switcher chrome (APP-063). Camera session isolation (MED-034).

#### Acceptance criteria
- [ ] Two concurrent graphical sessions on H-004 have separate mixer graphs; a playback in session A is inaudible in session B's capture and mix-monitor.
- [ ] A capture Capability minted in session A cannot be used from session B (`Error::Rights`, no handle).
- [ ] Switching sessions preserves each session's routes and in-use state; `os inspect` shows session identity on each AudioStream.
- [ ] Default-device hot-plug applies to the focused session and does not move the background session's exclusive devices.

#### Verification
- Integration: `runtime:tests/aud/session_isolation_*` on `hw-h004`.
- Manual: operator logs in two users on H-004, plays a tone in each, and confirms no cross-session mix.
- Review: SEC lead confirms grant-store versus mixer-graph split on the pull request.

#### Evidence
- none

### AUD-025 · Verify playback and capture on every V3 Tier 1 machine
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: AUD-010, AUD-013, AUD-021, HW-070, HW-062, HW-039
- Baseline: §62, §63

V3 hardware breadth and the public-alpha demo (Bluetooth headset, video call) require jack, HDMI/DP, USB and Bluetooth audio on all six Tier 1 machines, including the NVIDIA desktop. Probe output feeds REL's HCL.

#### Out of scope
HCL publication (REL-048, REL-035). NVIDIA GPU bring-up (HW-070, HW-052). Camera (MED-030). Combined hardware suite ownership (HW-086 at V4).

#### Acceptance criteria
- [ ] Native playback and capture complete on H-002, H-004, H-005, H-006, H-007 and H-008 through AudioStream.
- [ ] Jack or USB-C headset, HDMI/DP audio where the SKU has it, USB audio and Bluetooth A2DP are recorded per machine in the probe output.
- [ ] NVIDIA HDMI audio on H-006 is included in that probe output.
- [ ] A machine that fails a class records a typed degraded reason, not a silent skip, for REL's HCL row.

#### Verification
- Integration: `runtime:tests/aud/tier1_probe_*` on `hw-h002`, `hw-h004`, `hw-h005`, `hw-h006`, `hw-h007` and `hw-h008`.
- Demo: V3-D05 Bluetooth headset and video-call audio on H-005 or H-008.
- Review: REL lead confirms probe fields match the HCL schema.

#### Evidence
- none

### AUD-026 · Ship AudioStream conformance tests for the freeze
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: AUD-008, AUD-002, AUD-006, AUD-005, AUD-009, AUD-003, AUD-001, AUD-022, ABI-049
- Baseline: §7, §65, §66
- Freezes: S-025
- Invariants: I-040

V4 locks Layer 2 interface versions. S-025 is the Layer 2 audio-objects surface explored by AUD-008 and decided by AUD-002. This task is the conformance suite for rights, rebind, mixer and personality bridges. ABI and IPC own the freeze adrs; AUD owns the audio cases. S-025 is L2; this task does not freeze a Layer 1 surface.

#### Out of scope
Layer 1 freeze adr (ABI-049). Layer 2 version lock register (IPC-068, IPC-068). Channel conformance (IPC-065). Capability conformance (CAP-051).

#### Acceptance criteria
- [ ] A conformance suite covers play and capture rights, `Error::Rights` denials, mixer gain, mix-monitor, server rebind, hot-plug default switch, and personality holders for PipeWire and WASAPI termini.
- [ ] Every case runs on H-002 and is wired into V4 RC CI.
- [ ] S-025 is listed as frozen by this task only after the spike report and accepted object-model Decision are in the closure.
- [ ] No Layer 1 AudioStream entry point is marked frozen before ABI-049 (I-040).

#### Verification
- Integration: `runtime:tests/aud/conformance_*` on `hw-h002` and in the V4 RC matrix.
- Review: ABI and IPC leads confirm S-025 freeze versus L1 non-freeze on the pull request.
- Manual: surfaces register records S-025 frozen by this task.

#### Evidence
- none

### AUD-027 · Verify audio on every V4 Tier 1 machine
- Type: build
- Milestone: V4
- Status: todo
- Size: S
- Owner: none
- Depends on: AUD-025, AUD-026, HW-081, HW-080, HW-086, LAB-023
- Baseline: §62, §63

V4 hardware gate: the full suite including audio on at least ten Tier 1 machines each RC. Extends the V3 matrix across two GPU generations and NVIDIA HDMI audio. No new architecture.

#### Out of scope
Combined suite ownership (HW-086). Fleet racking (LAB-023). Second-generation bring-up (HW-081, HW-080). Camera and print rows (MED, HW).

#### Acceptance criteria
- [ ] Native playback and capture complete on H-002, H-004, H-005, H-006, H-007, H-008, H-009, H-010, H-011, H-012, H-013 and H-014 through AudioStream each RC.
- [ ] HDMI audio is recorded on every desktop SKU that exposes it, including H-006 and the second AMD GPU generation.
- [ ] The AUD cases in HW-086 are the cases this task supplies; a failure fails the RC.
- [ ] Conformance suite from AUD-026 is included in that RC run on H-002.

#### Verification
- Integration: `runtime:tests/aud/tier1_probe_*` extended across the V4 hardware matrix.
- Review: HW lead confirms AUD cases are the audio rows of the combined suite.
- Demo: V4 ten-machine hardware-suite run includes audio per machine.

#### Evidence
- none

### AUD-028 · Publish audio round-trip versus Linux, Windows and macOS
- Type: benchmark
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: AUD-004, AUD-017, AUD-027
- Baseline: §54
- Benchmarks: B-028, B-029

1.0 requires every §54 metric published on Tier 1 against Linux, Windows and macOS. Re-run B-028 and B-029 audio latency with the existing harnesses. No superiority claim without the report.

#### Out of scope
Umbrella 1.0 publication (BEN-060). Pairing-time series (HW-033). New harness work.

#### Acceptance criteria
- [ ] B-028 reports exist for every 1.0 hardware-scope machine that has a loopback fixture, with Linux, Windows and macOS baselines named in the register method.
- [ ] B-029 audio-latency reports exist for every 1.0 laptop in hardware scope with a Bluetooth peer.
- [ ] Reports meet the register target kind for 1.0 and state no superiority claim.
- [ ] No AUD text restates a numeric target.

#### Verification
- Bench: B-028 and B-029 on the 1.0 hardware scope; target per register.
- Review: BEN lead confirms baselines match the register method.

#### Evidence
- none

### AUD-029 · Score audio integration on L5 and W3 Corpus entries
- Type: build
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: AUD-014, AUD-022, AUD-027
- Baseline: §47, §49
- Corpora: C-006, C-009
- Invariants: I-096

1.0 compatibility gates require audio in the integration score for every passing Linux and Windows entry. AUD supplies native-object probes used by LNX and WIN scenario scripts.

#### Out of scope
L5 and W3 pass-rates (LNX-110, WIN-082). Scenario script authorship (LNX, WIN). Probe publication format (REL-015).

#### Acceptance criteria
- [ ] Every passing C-006 entry that declares audio has an AudioStream integration score recorded in the corpus report on Tier 1.
- [ ] Every passing C-009 entry that declares audio has an AudioStream integration score recorded in the corpus report on Tier 1.
- [ ] A passing entry whose audio bypasses AudioStream is reported as a failed integration check, not as a silent pass.
- [ ] Probes are the same native-object checks used at V1 and V2, not a personality-only path.

#### Verification
- Compat: C-006 and C-009 audio integration scoring on Tier 1.
- Review: LNX and WIN leads confirm scripts call the AUD probes.

#### Evidence
- none

### AUD-030 · Park MIDI and pro-audio until after 1.0
- Type: docs
- Milestone: LATER
- Status: todo
- Size: S
- Owner: none
- Depends on: AUD-002
- Baseline: none
- Invariants: I-093

MIDI and pro-audio are explicit 1.0 non-goals so they appear on the published non-goal list. APP-069 covers the combined declaration; this parks the AUD-owned remainder (MIDI ports, Jack and pro-audio graphs) on LATER.

<!-- covers: EXTRA-038 -->

#### Out of scope
Combined non-goal publication (APP-069). Native AudioStream mixer (in-rung AUD tasks). Codec Packages (MED).

#### Acceptance criteria
- [ ] The published non-goal list names MIDI ports, Jack and pro-audio graphs as post-1.0, citing I-093.
- [ ] No non-dropped AUD task before LATER implements MIDI ports or a Jack-compatible graph.
- [ ] A follow-up note in this task lists MIDI ports and Jack-compatible graphs as the pieces a later split would allocate.

#### Verification
- Review: GOV and APP leads confirm the AUD remainder is listed beside EXTRA-038 on the published non-goal page.
- Manual: `roadmap check` shows this task at LATER with no pre-1.0 dependent outside LATER.

#### Evidence
- none
