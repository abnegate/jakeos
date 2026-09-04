# Milestone ladder (design pass output)

## Definition of 1.0

1.0 is the first release the project commits to keep working. Concretely: (1) ABI stability — the Layer 1 native kernel ABI (§66) is frozen and declared stable; every binary and package built against the 1.0 Layer 1 ABI runs unmodified on every 1.x release, and any Layer 1 change requires a new major version. Layer 2 core platform interfaces ship with versioned schemas; the 1.0 interface versions remain served for the entire 1.x line with a published deprecation policy (minimum two minor releases of overlap). Layers 3 and 4 are explicitly allowed to evolve with semver. (2) Support commitment — the 1.x line receives security and critical-bug updates for at least 24 months after 1.0 ships; upstream Linux CVEs rated High/Critical that affect inherited code are fixed in a shipped generation within a published SLA (target 14 days, tracked publicly); a security-response process with a disclosure address, embargo handling, and advisories exists and has been exercised. (3) Update and rollback guarantees — every update is an atomic new system generation; a failed boot automatically returns to the previous generation without user intervention; the previous N generations (N ≥ 3) remain bootable; rollback never loses user data; `os history` and `os restore` cover OS, packages, and configuration; update success rate on Tier 1 hardware is measured at ≥ 99.9%. (4) Hardware coverage — a published Hardware Compatibility List with tiers: Tier 1 machines (at least 10 named configurations across AMD desktop, Intel laptop, AMD laptop, and an NVIDIA desktop) are fully tested every release and every listed feature works, including suspend/resume, Wi-Fi, Bluetooth, external displays, HDR/VRR where the hardware supports it; Tier 2 is community-reported and best-effort; anything unlisted is unsupported. x86-64 only. (5) Compatibility coverage — the Linux corpus L5 (500 applications with scripted scenarios) passes at ≥ 90%, and the Windows corpus W3 (300 titles) reaches ≥ 70% Gold-or-better and ≥ 85% Silver-or-better on Tier 1 hardware, with zero regressions on entries that were Gold at V4; Linux and Windows software integrate with the native taskbar, launcher, clipboard, audio, file chooser, notifications, and scaling. (6) Security posture — no ambient authority for native applications, a permissions UI that shows and revokes every grant, full-disk encryption in the installer, Secure Boot and measured boot on Tier 1, a public threat model, an external security audit with all High/Critical findings closed, continuous syscall and IPC fuzzing with no known open crashers, and reproducible builds for 100% of system-generation packages. (7) Documentation — complete Layer 1 ABI reference (every syscall, object type, and capability right documented), SDK guides for Rust and C, an administrator guide, a compatibility guide, and release notes with a migration path from V3/V4 installs. (8) Quality bar — ≥ 30-day release-candidate soak with no open P0/P1, opt-in telemetry showing ≥ 99.5% crash-free sessions across the beta fleet, and every §54 metric measured and published on Tier 1 hardware against Linux, Windows, and macOS baselines without any unmeasured superiority claim. What 1.0 explicitly does NOT promise: universal PC hardware compatibility; ARM64 or RISC-V; 100% Linux or Windows compatibility, kernel-level anti-cheat titles, or vendor-proprietary DRM; stability of Layer 3/4 SDK and framework APIs beyond semver; a native filesystem or object store (the storage model runs on a mature Linux filesystem); a native GPU driver stack or native browser/IDE (these run via the Linux personality); distributed or remote interfaces; hardware capability enforcement (CHERI-class) — only that the ABI leaves room for it; performance superiority over Linux, Windows, or macOS on any metric not measured and published; and continued upstream-Linux mergeability of the kernel fork.

## V0 — Execution Model

**Purpose.** Prove that the native execution model (components, tasks, capabilities, typed channels, async operations, memory objects, resource domains, tracing) exists inside a booting Linux fork without breaking Linux hardware support.

**Hardware scope.** QEMU/KVM as the primary CI target, plus exactly one designated reference AMD desktop (Zen 4-class CPU, RDNA 3-class discrete GPU, NVMe, wired Ethernet) whose exact SKUs are recorded in hardware/reference.md. No laptops, no Wi-Fi, no Bluetooth, no GPU acceleration requirements.

### Scope in

- Kernel fork established with build, boot, and upstream-tracking policy (KRN, BLD)
- Native ABI v0: object handles, capability table, syscall surface, stability-layer declaration (ABI)
- Capability<T>: creation, derivation, attenuation, transfer over channels, revocation, audit log (CAP)
- Component with own address space, capability set, TaskGroup, ResourceDomain; wrapper over task_struct/mm_struct/cgroups per Phase C (CMP)
- Task and TaskGroup with structured cancellation propagation (CMP)
- Channel<T> with a first IDL, generated Rust stubs, wire format, and version-negotiation header (IPC)
- Small-message fast path and MemoryObject ownership transfer for large payloads (IPC, MEM)
- Operation<Result> submission/completion, deadlines, cancellation (OPS)
- ResourceDomain with CPU share and memory budget enforcement (SCH)
- Tracing of every primitive, `os inspect` and `os trace` first cut (OBS, CLI)
- Tiny native runtime and Rust SDK crate sufficient for the demo (SDK)
- Benchmark harness and CI on QEMU plus reference hardware (BEN, BLD)
- ADRs for every V0 architectural choice with options recorded (GOV)

### Scope out

- Compositor, UI protocol, graphics abstractions
- Packages, system generations, storage model beyond raw block/filesystem passthrough
- Native filesystem or content-addressed store
- Windows personality
- Wasm component integration beyond an ADR
- Heterogeneous compute beyond ComputeDevice enumeration ADR
- Installer, updater, security UI, disk encryption
- Any hardware other than QEMU/KVM and the single reference desktop
- Performance tuning beyond measuring and publishing

### Exit criteria

- Kernel fork boots to a native init on QEMU/KVM and on the reference AMD desktop from a CI-built image; boot is reproducible from a tagged commit
- A native Component can be created, given capabilities, run code, and be destroyed with all resources reclaimed; verified by a leak test creating and destroying 100,000 components with no growth in kernel memory beyond 1%
- Capability<File, ReadWrite> derives Capability<File, Read>; an attempt to derive Admin or to forge a handle fails with a typed error; both covered by tests
- Revoking a capability makes every derived capability fail within one operation; verified by a test with a derivation depth of 8
- Channel<T> carries typed messages generated from the IDL; a message with an unknown newer field is accepted by an older receiver (forward compatibility test) and an older message by a newer receiver (backward compatibility test)
- A MemoryObject capability is transferred between two components without copying the payload; verified by physical-page identity check and by a 1 GiB transfer completing in under 1 ms
- Async Operation submission/completion works for Read, Write, Send, Receive, Timer, Wait; an operation with a deadline completes with a Timeout result; a cancelled operation never delivers a result
- Cancelling a TaskGroup cancels every owned task and child group within a bounded time; a test with 10,000 tasks in a 4-level tree completes cancellation in under 50 ms
- A component in a ResourceDomain with a 64 MiB memory budget cannot exceed it; a CPU share of 25% is enforced within ±5% under contention over a 10 s window
- `os inspect component|task|channel|capability|memory|resource` prints state, ownership, and relationships for every live object; `os trace` shows IPC flow and scheduling delays for the demo with tracing overhead measured at under 3% on the benchmark
- Linux compatibility is intact: the unmodified Linux syscall ABI still works and the L0 corpus passes at the same rate as the unforked baseline kernel on the same hardware
- Every V0 task in the task repository is `done` or `dropped` with a written reason; the generated roll-up shows 100%
- Every V0 architecture decision (handle representation, capability encoding, IDL, fast-path mechanism, wrapper-vs-native for components) has an accepted ADR in decisions/ listing at least two rejected options
- All new kernel and runtime code is Rust unless an ADR exempts a specific file; `cargo clippy -D warnings` and rustfmt are clean in CI

### Demos

- Native Component A sends Channel<Request>; Component B decodes, allocates a MemoryObject, transfers ownership back; A reads the result without a copy (§59 demo), shown live with `os trace` displaying the flow
- Cancellation demo: killing the parent TaskGroup tears down A, B, and all in-flight operations deterministically
- Fault demo: B panics; A observes a typed disconnect and rebinds to a restarted B
- Isolation demo: B attempts to open a file it has no capability for and receives a typed denial visible in the audit log
- Linux busybox shell and the L0 corpus running side by side on the same kernel

### Benchmark gates

- Component creation latency: p50 and p99 measured on reference hardware and QEMU and published; target p50 ≤ 200 µs (tens to hundreds of µs per §10); if p50 exceeds 500 µs an ADR documenting root cause and remediation plan must be accepted before V0 closes
- Task creation latency p50/p99 measured and published; target p50 ≤ 2 µs
- IPC round trip (small message, same core and cross core) p50/p99 measured and published against Linux Unix-domain-socket and pipe ping-pong on the same machine; no superiority claim without the published table
- MemoryObject transfer cost measured for 4 KiB, 1 MiB, 1 GiB payloads and published against memcpy and Linux splice/vmsplice
- Memory overhead per idle component and per idle task measured and published
- Context-switch behavior (native task handoff vs Linux thread switch) measured and published
- Tracing overhead measured with tracing enabled vs disabled on the IPC benchmark; must be under 3%
- Benchmark suite runs in CI on every merge to main and publishes results as generated Markdown between markers in BENCHMARKS.md

### Compatibility gates

- L0 corpus defined in compat/corpus/linux-L0.md: a Linux Test Project syscall subset (at least 1,000 tests) plus a busybox root filesystem, bash, coreutils, python3, and a static Go binary
- L0 pass rate on the fork equals the pass rate of the unmodified upstream kernel of the same version on the same hardware (zero regressions)
- Windows: none (explicitly out of scope; a scoping ADR is the only deliverable)

### Risks

- Wrapping task_struct/mm_struct/cgroups may make component creation far slower than the target, forcing earlier native implementation than planned
- Rust-for-Linux toolchain and in-kernel Rust API surface remain unstable; bindings to core subsystems may be missing
- Capability encoding and handle-table design churn delays everything downstream because the ABI must stay tiny yet leave room for hardware enforcement (§8)
- Keeping the Linux syscall path fully intact while inserting native scheduling and completion hooks can silently regress Linux behavior
- IDL and interface-evolution rules (§12) are easy to under-design; getting them wrong becomes a Layer 1 mistake
- Scope creep toward a usable system before the execution model is proven

## V0.5 — Application Model

**Purpose.** Prove that real applications can be built on the native model: a compositor, windows, input, declarative UI, immutable packages, capability-based file selection, service restart, and simple system generations.

**Hardware scope.** QEMU/KVM (virtio-gpu for compositor CI) plus the reference AMD desktop with one display at fixed refresh. An Intel laptop may be used for exploratory work but nothing gates on it.

### Scope in

- Native compositor on DRM/KMS with Surface, Buffer, Display, Frame objects; single display, fixed refresh (GFX)
- Native UI protocol v0 and Rust toolkit: declarative widgets, retained rendering, keyboard and mouse input, basic text, clipboard capability (UIP, TXT)
- Package format: manifest, components, requested capabilities, content-addressed immutable store; install means make-available (PKG, STO)
- Simple system generations: build, switch, rollback at boot menu (PKG, BOOT)
- Capability-based file chooser owned by the OS returning UserSelected<T> (STO, SEC)
- Service restart and client rebind for compositor and one other service (CMP, IPC)
- Storage model over a mature Linux CoW filesystem with snapshots (STO)
- Native applications: Terminal, File Browser, Text Editor, Image Viewer (APP)
- Wayland compatibility bridge so Linux GUI apps appear as native windows, using existing Linux mechanisms underneath (LNX)
- Accessibility metadata in the UI protocol from day one, no assistive-tech client yet (ACC)
- SDK expands to windows, files, packages; SDK API is unstable (SDK)

### Scope out

- Multi-monitor, HDR, VRR, fractional scaling, multi-GPU
- Audio
- Networking beyond wired Ethernet passthrough
- Wi-Fi, Bluetooth, suspend/resume, battery
- Windows personality
- Native package repository or store; packages are installed from local files
- Browser, IDE, Git client as native applications
- Semantic interfaces beyond the interfaces the four apps need
- Screen sharing, remote surfaces
- Full system history and restore of user data

### Exit criteria

- Native compositor starts from a system generation, drives the reference GPU via DRM/KMS with GPU-accelerated composition, and presents at the display's fixed refresh rate with zero dropped frames over a 60 s idle desktop capture
- A native application opens a window, receives keyboard and mouse input, renders declarative UI, and animates at the display refresh rate; measured frame-time p99 below the frame interval in a scripted UI benchmark
- Compositor crash recovery: killing the compositor restarts it and rebinds all open windows with no application exit; verified 100 consecutive times in CI on QEMU and 20 times on hardware
- Immutable packages: installing a package adds it to the content-addressed store without writing outside the store; verified by filesystem diff showing no changes to any shared directory
- Two versions of the same library coexist and are used by two different installed applications simultaneously; deduplication of identical content verified by store size
- System generations: an update produces a new generation; selecting the previous one at boot restores the previous kernel, compositor, and packages; verified by automated boot-menu test
- File chooser: the Image Viewer starts with UI and GPU capabilities only, opens a user-chosen image via UserSelected<Image>, and a test confirms it cannot enumerate or open any other file in the same directory
- Clipboard is a capability: an application without the clipboard capability cannot read clipboard contents; a test confirms the denial
- Terminal, File Browser, Text Editor, and Image Viewer each pass their scripted acceptance scenarios and are dogfooded by the core team for at least 10 sessions each
- A Wayland Linux application (from the L1 corpus) appears as a normal window under the native compositor with working input and clipboard
- Every UI widget emits accessibility metadata (role, name, state) inspectable via `os inspect`; verified by a tree dump test on each of the four apps
- Interface versioning exercised end to end: the UI protocol v0 is bumped to v0.1 with an added optional method and old clients still run (regression test retained permanently)
- Every V0.5 task is `done` or `dropped` with reason; roll-up shows 100%
- Accepted ADRs exist for: filesystem choice, package manifest schema, content-addressing scheme, UI protocol transport, compositor frame scheduling, generation switching mechanism

### Demos

- Cold boot to native desktop showing the four native applications running from immutable packages
- Image Viewer chooses a photo through the OS chooser; audit log shows exactly one Capability<Image, Read> granted
- Kill the compositor mid-session; all windows come back and the Text Editor keeps its unsaved buffer
- Install a package, then roll back to the previous generation at the boot menu; the package is gone and the previous state is intact
- A Linux Wayland application (e.g. a GTK text editor from the L1 corpus) running beside native apps with copy-paste between them

### Benchmark gates

- Native application warm startup (click to first presented frame) measured for each of the four apps and published; target p50 ≤ 20 ms for the smallest (§34) with the number published either way
- Cold startup measured and published
- Compositor frame latency (commit to scanout) p50/p99 measured and published against a Wayland compositor on the same hardware
- Input-to-photon latency measured with a photodiode or high-speed camera rig on reference hardware and published against a Linux Wayland desktop on the same machine
- Package install time and store deduplication ratio measured on a corpus of 20 packages and published
- Generation switch and rollback time measured and published
- Compositor restart-to-rebound time measured; target under 500 ms
- All V0 benchmarks re-run with no regression greater than 10% without an accepted ADR explaining the regression

### Compatibility gates

- L0 corpus still passes with zero regressions
- L1 corpus defined in compat/corpus/linux-L1.md: at least 20 CLI/dev tools (git, gcc, clang, python3, node, rustc/cargo, bash, tmux, vim, curl, ssh, make, tar) each with a scripted scenario, plus 5 Wayland-native GUI applications; L1 CLI pass rate 100%, GUI pass rate ≥ 80% (window, input, clipboard working)
- Windows: none

### Risks

- Building a compositor and a UI toolkit simultaneously is the largest single body of new userspace code in the project; scope creep toward a full desktop shell
- Mesa and DRM userspace assume Linux process/fd semantics; adapting them to native components without forking Mesa may force a hybrid where the compositor is partly a Linux-personality process
- Package manifest and capability request schemas are Layer 2 decisions that are hard to change later
- Generation switching depends on bootloader integration that is easy to underestimate
- Text rendering and input (shaping, IME) is deep; a minimal path may harden into the permanent design
- Keeping the Wayland bridge working while the native protocol evolves doubles the compositor test matrix

## V1 — Developer Preview

**Purpose.** Make the OS useful enough that its developers build the OS while running the OS, with a stable native SDK, self-hosted toolchain, networking, audio, GPU acceleration, power management, and Linux compatibility good enough for daily development.

**Hardware scope.** Reference AMD desktop plus one designated Intel laptop (Intel CPU with integrated GPU, Intel Wi-Fi, NVMe, internal display plus one external display over USB-C). Both recorded in hardware/reference.md. Bluetooth and HDR/VRR are not required to work on either.

### Scope in

- Stable native SDK v1 (Rust) with semver and a compatibility test suite; C bindings for the Layer 1 ABI (SDK)
- Self-hosted build: the kernel, platform, and native apps build on the OS from a native dev environment (BLD, ENV)
- `os env` development environments from environment.yaml with ResourceDomain, storage snapshot, capability and network namespaces (ENV)
- Networking: kernel stack retained, network capabilities, DHCP, DNS, Wi-Fi (Intel laptop), basic firewall policy in userspace (NET)
- Audio: native audio objects, low-latency path, PipeWire/PulseAudio compatibility for Linux apps (AUD)
- GPU acceleration for native and Linux-personality applications (GFX, LNX)
- Power management, suspend/resume, battery reporting on the Intel laptop (HW)
- Native package manager with a signed remote repository and the first update channel (PKG, REL)
- Linux personality as a product: glibc, D-Bus, portals, PipeWire, XWayland, OCI containers, browser and IDE via personality (LNX)
- Debugging and tracing tools: `os trace` with structured export, native debugger support (source-level breakpoints, async task stacks) (OBS, CLI)
- Terminal, editor, and shell mature enough for daily work; Git and a browser (via Linux personality) integrated into launcher, clipboard, file chooser (APP, LNX)
- Identity: single user login, session lock, keyring/secrets service (SEC)
- System history v1: `os history` lists generation, package, and environment events; `os restore` to a previous generation (PKG, CLI)
- Semantic interface v0 for Terminal and Editor as proof (SEM)
- Wasm component runtime ADR and prototype: a Wasm component participates in a native channel (WASM)

### Scope out

- Windows personality (a feasibility spike only, no gate)
- HDR, VRR, multi-GPU, fractional scaling
- Bluetooth
- Installer for end users (developers install from a scripted image)
- Native app store UI
- Native browser or IDE
- Full disk encryption UI (encryption may be configured manually)
- Permissions UI beyond a log viewer
- Accessibility assistive-tech client
- Localization beyond English

### Exit criteria

- Self-hosting: a clean checkout builds the full system image on the OS inside an `os env` environment, and the produced image boots and passes the V0 and V0.5 acceptance suites (reproducibility of the image verified bit-for-bit across two machines)
- Daily-driving: every core team member has used the OS as their primary development machine for at least 20 working days, with issues tracked; no open P0 issues that force a fallback to another OS
- SDK v1 published with a semver policy; the SDK compatibility test suite passes for applications built against v1.0.0 after each subsequent v1.x release during the milestone
- `os env enter` on a cached environment presents a working shell in the environment with services (e.g. Postgres, Redis) reachable; cached startup measured with target p50 under 50 ms (§35) and number published
- Wi-Fi with WPA2/WPA3 connects, roams between two access points, and survives suspend/resume on the Intel laptop; DHCP and DNS work; a component without a network capability cannot open a connection (tested)
- Audio plays through the native path and through PipeWire compatibility simultaneously without glitches over a 10-minute test; native low-latency path round-trip measured and published
- Suspend/resume succeeds in at least 200 of 200 automated cycles on the Intel laptop and 100 of 100 on the AMD desktop, with Wi-Fi, display, and audio functional afterwards
- GPU-accelerated Linux applications (browser with WebGL, an OpenGL and a Vulkan sample) run through the Linux personality with acceleration confirmed via frame-rate comparison to software rendering
- Signed package repository: packages are content-addressed, signed, verified before activation; a tampered package is rejected (tested); an update delivered from the repository creates a new generation
- Native debugger attaches to a component, breaks at a source line inside an async task, and shows the logical task stack; `os trace` exports a session that can be viewed offline
- Rollback: `os restore` to a previous generation restores kernel, packages, and system configuration; verified in CI after a simulated bad update
- Semantic interface: an automation script invokes Terminal.run and Editor.open through typed interfaces without GUI input; recorded as a permanent test
- A Wasm component exchanges typed messages over a native Channel<T> with a machine-code component in the prototype
- Kernel fork tracks upstream: at least one upstream stable release has been rebased or adapted during V1 following the divergence policy, with the L0 corpus still green
- Every V1 task is `done` or `dropped` with reason; roll-up shows 100%
- ADRs accepted for: SDK stability policy, network capability model, audio object model, package signing and trust roots, Linux personality D-Bus/portal strategy, Wasm role

### Demos

- Live: clone the OS repository on the OS, edit a kernel file, build the image inside `os env`, install as a new generation, reboot into it, roll back
- Developer day: browser (Linux personality), native terminal, editor, Git, and audio playback, all in one session on the Intel laptop on battery with suspend/resume mid-day
- Capability demo: a Linux-personality browser downloads a file; the native file chooser grants the editor access to exactly that file
- Typed automation: a script that opens a project in the editor and runs its tests via semantic interfaces, shown alongside `os trace`
- Wasm plugin loaded into the editor as a sandboxed component with only the capabilities it declares

### Benchmark gates

- All V0 and V0.5 benchmarks re-run on all V1 hardware with no regression greater than 10% without an accepted ADR
- IPC small-message round trip target: p50 ≤ 2 µs same-core and ≤ 5 µs cross-core on the reference desktop; published beside Linux UDS and futex ping-pong numbers
- Component creation target: p50 ≤ 100 µs; published beside fork+exec, clone, and container start (podman run) on the same hardware
- Environment startup: cached `os env enter` p50 ≤ 50 ms, cold measured and published, beside `docker compose up` for an equivalent stack
- Native application warm startup: p50 ≤ 20 ms for the Terminal and Editor; measured and published
- Linux compatibility overhead: syscall latency and throughput for the L2 corpus workloads measured against upstream Linux on the same hardware and published (target within 5% for non-graphics workloads; the number is published regardless)
- Audio round-trip latency (native path) measured and published beside PipeWire on Linux on the same hardware
- Idle power draw and battery runtime on the Intel laptop measured and published beside a mainline Linux distribution on the same machine; no superiority claim
- Build time of the full system image on the OS vs on Linux on the same hardware measured and published

### Compatibility gates

- L0 and L1 corpora pass with zero regressions
- L2 corpus defined in compat/corpus/linux-L2.md: 50 applications with scripted scenarios covering a Chromium-based and a Firefox-based browser, a mainstream IDE, GIMP-class image editor, LibreOffice-class suite, a Qt and a GTK application, an Electron application, XWayland-only application, OCI container runtime, Flatpak, and 30 CLI tools; pass rate ≥ 90%, with 100% for the browser, IDE, container runtime, and Git
- Integration check for every L2 GUI application: appears in launcher and task switcher, clipboard both directions, file chooser via portal, notifications, correct scaling — scored as part of the scenario
- Windows: a feasibility report on running Wine's test suite under the Linux personality; no pass-rate gate

### Risks

- Browser and IDE run through the Linux personality; if D-Bus, portals, or GPU acceleration in the personality are weak, daily-driving fails regardless of native quality
- Self-hosting requires the entire toolchain (Rust, LLVM, kernel build) working under native environments; missing pieces block the gate
- Power management and suspend/resume on laptops are notoriously hardware-specific; the single Intel laptop may hide or expose issues unrepresentatively
- SDK stability pressure conflicts with the desire to still change the ABI after learning from real applications
- Upstream kernel rebases while native hooks grow can consume the whole KRN team
- Signed repository and trust roots are security-critical and easy to get wrong early

## V2 — Desktop Preview

**Purpose.** Deliver a polished desktop with laptops, multi-monitor, HDR, VRR, Bluetooth, Windows applications and a gaming proof-of-concept, rollback and snapshots, a native repository, and semantic automation, on three deliberately constrained target machines.

**Hardware scope.** Exactly three named target machines recorded in hardware/targets.md: one AMD desktop (Zen 4-class, RDNA 3-class GPU, HDR/VRR display), one Intel laptop (integrated Intel GPU, Intel Wi-Fi and Bluetooth), one AMD laptop (Zen 4-class APU, Wi-Fi 6, Bluetooth). One NVIDIA desktop is tracked as experimental with no gate. No promise of universal PC compatibility.

### Scope in

- Polished desktop shell: launcher, taskbar, notifications, settings, session management, lock screen (APP, UIP)
- Multi-monitor, per-display and fractional scaling, HDR, VRR, multi-GPU awareness, external displays hot-plug (GFX)
- Bluetooth (audio, input, file transfer) via user-space driver stack (HW, AUD)
- Laptops: battery, thermal, lid, brightness, suspend/resume, hibernate optional (HW, SCH)
- Windows personality: PE loading, Win32/NT via Wine/Proton-derived layer, registry emulation, DirectX translation (DXVK/VKD3D), gaming proof of concept (WIN)
- Rollback and snapshots surfaced in UI; system history with restore of OS, packages, and configuration (PKG, INS)
- Native repository/app store client with capability review at install (PKG, SEC, APP)
- Permissions UI v1: view and revoke grants, prompts for capability requests (SEC)
- Semantic automation: user-visible rules engine over typed interfaces; AI-assistant integration proof via capabilities (SEM)
- Screen sharing and screen recording as explicit capabilities with indicator (GFX, SEC)
- Accessibility: first assistive-technology client (screen reader prototype) driving semantic tree (ACC)
- Heterogeneous compute: ComputeDevice enumeration and dispatch to CPU and GPU with preferences (HET)
- Localization framework in place; shell translated into at least 3 languages (TXT)
- Hardware target selection ADR: three named machines (HW)

### Scope out

- Installer for the general public (still image-based install with a guided script)
- Crash reporting pipeline and hardware compatibility database
- Public package submission process
- Kernel-level anti-cheat games
- NVIDIA as a gated target (experimental only)
- Distributed/remote interfaces
- Native filesystem
- Certification of accessibility compliance

### Exit criteria

- Desktop shell passes the 40-scenario desktop UX script (launch, switch, notify, settings, lock, unlock, external display, scaling change) on all three target machines with zero P0/P1 defects
- Multi-monitor: two displays with different scale factors and refresh rates, hot-plug and unplug 100 times in automation without compositor restart or window loss
- HDR10 output verified on an HDR display with a native HDR test application and a Windows-personality HDR game; VRR verified via display reporting at variable frame rates
- Bluetooth: pairing and reconnect of a headset (A2DP and HFP), a mouse, and a keyboard succeed in 95 of 100 automated pairing cycles; user-space Bluetooth stack crash is recovered without reboot (tested)
- Laptop: 500 automated suspend/resume cycles per laptop with ≥ 99% success and full function after; battery estimate within 10% of measured runtime
- Windows personality runs the W1 corpus at the pass thresholds below; a Windows application launches by double-clicking an .exe, appears in taskbar and launcher, uses the native file chooser and clipboard, with no separate desktop or wizard
- Gaming proof of concept: at least 10 W1 titles rated Gold or better run with GPU acceleration, HDR or VRR where supported, and gamepad input, with FPS measured and published against Linux+Proton on the same hardware
- Snapshots and rollback are in the settings UI; a user can restore a previous generation and a previous package set from the UI; a fault-injected broken update automatically boots the previous generation (tested on all three machines)
- Native repository: install, update, and remove from the store client; capability requests are shown and can be denied at install, with the application still launching in a degraded mode where it declares optional capabilities
- Permissions UI lists every active grant for every application and revocation takes effect immediately (tested with camera, microphone, files, network, screen capture)
- Screen sharing: an application without the screen-capture capability receives a black or denied surface; sharing shows a persistent indicator; tested
- Semantic automation: three shipped example rules (download completed → extract; meeting start → create note; project opened → start environment) run without GUI input; an AI-assistant demo performs a multi-step task through capabilities only, with the full action graph logged and revocable
- Screen reader prototype reads and activates every widget in the four native applications and the shell settings; task completion script passes 100%
- ComputeDevice: a sample workload dispatched with preference Throughput runs on the GPU and with preference LowLatency on the CPU; `os inspect` shows the ComputeQueue
- Every V2 task is `done` or `dropped` with reason; roll-up shows 100%
- ADRs accepted for: target hardware selection, Windows personality architecture (how Wine/Proton maps to native objects), HDR/color pipeline, Bluetooth stack placement, permissions prompt policy, automation rules model

### Demos

- Laptop day: open lid, Wi-Fi reconnects, Bluetooth headset connects, external HDR display plugged in at a different scale, work in native and Linux apps, close lid, resume
- Double-click a Windows .exe from the file browser; it launches with native chrome, uses the native file chooser, and appears in the task switcher
- Gaming: a W1 title at 1440p with VRR and HDR on the AMD desktop, with an on-screen FPS overlay and the published comparison to Linux+Proton
- Update, discover a bug, roll back from Settings with a live view of `os history`
- AI assistant: 'find yesterday's Postgres branch and run its tests' executed through typed interfaces with the action graph and capability grants displayed and one step revoked mid-run
- Screen reader navigating the shell and Text Editor
- A permissions prompt appearing for camera access in a Linux-personality video app and being denied without the app crashing

### Benchmark gates

- All prior benchmarks re-run on all three target machines; no regression greater than 10% without an accepted ADR
- Input-to-photon latency measured at 60 Hz and the display's maximum refresh on each target machine and published beside Linux Wayland and Windows on the same hardware where dual-boot is possible
- Compositor frame deadline misses under a scripted mixed desktop workload measured; target under 0.1% of frames at 60 Hz; published
- Windows compatibility overhead: FPS for each Gold-rated W1 title measured against Linux+Proton and Windows on the same hardware, published, with no superiority claim
- Linux compatibility overhead re-measured on the L3 corpus and published
- Bluetooth audio latency and pairing time measured and published
- Battery runtime under a scripted mixed workload measured and published beside mainline Linux and, where possible, Windows on the same laptop
- Suspend and resume latency measured and published
- Application warm startup for all shipped native applications published; Terminal and Editor hold p50 ≤ 20 ms
- Snapshot creation and restore time measured and published
- Energy use of the idle desktop measured via battery discharge and published

### Compatibility gates

- L0, L1, L2 corpora pass with zero regressions
- L3 corpus defined in compat/corpus/linux-L3.md: 150 applications including Flatpak and Snap-free distro packages, games via Steam on Linux, media players, creative tools, video conferencing; pass rate ≥ 85% with integration scoring (launcher, clipboard, notifications, file chooser, audio, scaling) as part of pass
- W1 corpus defined in compat/corpus/windows-W1.md: 50 titles (35 games without kernel-level anti-cheat drawn from widely played lists, 15 productivity applications) with scenarios rated on a Platinum/Gold/Silver/Bronze/Broken scale; ≥ 50% Gold or better, ≥ 70% Silver or better, at least 10 Gold with HDR or VRR exercised
- Integration check for Windows applications: taskbar, launcher, notifications, clipboard, audio, file chooser, gamepad input

### Risks

- Windows gaming is a research-scale problem: Wine/Proton assume glibc and Linux process semantics, so the personality may initially be Wine running inside the Linux personality, which must still feel native
- Anti-cheat and DRM titles are structurally excluded but users will judge the milestone by them
- NVIDIA support may become a hard user expectation before it is a gated target
- HDR color management across compositor, Mesa, and games is immature everywhere; being correct requires reference displays and measurement hardware
- Bluetooth in user space is a full stack to build or port
- Laptop firmware quirks on two specific machines can consume the HW team for the entire milestone
- Permissions prompts risk becoming a deny-list UX that contradicts the capability model
- Accessibility and localization arriving here risk being treated as bolt-ons

## V3 — Public Alpha

**Purpose.** Open the OS to enthusiasts, developers, researchers, and experimental gamers with an installer, updater, rollback, recovery, crash reporting, hardware compatibility database, signed package repository, permissions UI, and documentation good enough for strangers to succeed.

**Hardware scope.** Tier 1: the three V2 target machines plus one NVIDIA desktop (proprietary or open kernel module path decided by ADR) and one additional laptop of each vendor, six machines total, all fully tested each release. Tier 2: community-reported machines in the compatibility database, best effort. The installer refuses or warns on machines with known-blocking hardware. Still x86-64 only.

### Scope in

- Graphical installer with full-disk encryption, dual-boot detection, hardware compatibility check against the database (INS, SEC, BOOT)
- Updater with channels (alpha, testing), delta updates, automatic rollback on failed boot, recovery environment (INS, REL, PKG)
- Crash reporting: opt-in, privacy-reviewed, symbolicated, deduplicated, linked to issue tracker (INS, OBS)
- Hardware compatibility database with user submissions and automated probe tool (REL, HW)
- Public package repository with submission, review, signing, and revocation; developer publishing tooling (REL, PKG, SDK)
- UEFI Secure Boot with project key or shim, measured boot, generation attestation (BOOT, SEC)
- Permissions UI v2 including per-application audit history and one-time grants (SEC)
- Public documentation: install guide, user guide, SDK guide, compatibility guide, contribution guide, ABI draft reference (GOV, SDK)
- Security response process: disclosure address, advisories, embargo handling, CVE tracking for inherited code (REL, KRN, GOV)
- Multi-user login and sessions; screen lock with hardware-backed secrets where available (SEC)
- Wi-Fi and Bluetooth hardware breadth beyond target machines via inherited Linux drivers and the compatibility database (HW, NET)
- Printing, cameras, webcams, USB mass storage, common USB classes (HW)
- Community infrastructure: forum or chat, issue tracker triage process, RFC process live (GOV)

### Scope out

- Layer 1 ABI freeze (freeze candidate happens in V4)
- Support commitments and SLAs
- Localization beyond 5 languages
- Accessibility conformance claims
- Native browser or IDE
- ARM64
- Enterprise features (domain join, MDM, fleet management)
- Kernel-level anti-cheat

### Exit criteria

- Installer completes on all Tier 1 machines in automated runs with full-disk encryption enabled, with a success rate ≥ 98% over 200 runs across the fleet, and shows an accurate compatibility verdict for at least 20 Tier 2 machines
- A user without project involvement installs the OS from public media on a Tier 1 machine following only the public documentation, in a moderated usability study of at least 10 participants with ≥ 8 succeeding unaided
- Updater delivers at least 6 consecutive alpha releases over the milestone through the update channel; automatic rollback on a fault-injected failing generation works on every Tier 1 machine; recovery environment restores a system whose current generation is corrupted (tested)
- Crash reports from the alpha fleet are received, symbolicated, deduplicated, and appear in the tracker within 24 hours; privacy review documented; opt-in only
- Hardware compatibility database has entries for all Tier 1 machines and at least 100 community-submitted machines with probe data; the probe tool runs from the live installer
- Public package repository accepts a third-party submission, reviews it, signs it, publishes it, and revokes it in a rehearsed drill; at least 20 packages from outside the core team are published
- Secure Boot: all Tier 1 machines boot with Secure Boot enabled using the documented key or shim path; measured boot records generation identity in the TPM event log; verified
- Permissions UI: a user can see, for any application, every grant and when it was used, and revoke or convert it to one-time; usability study task completion ≥ 80%
- Security response: process documented and exercised via at least two real or simulated advisories with public postmortems; inherited-Linux High/Critical CVEs over the trailing quarter fixed in a shipped generation with median under 21 days and the distribution published
- Kernel fuzzing (native syscall surface and IPC) runs continuously in CI; no known open crasher older than 14 days at gate time
- Multi-user: two users with separate sessions, separate capability stores, and separate encrypted data; switching sessions preserves state (tested)
- Documentation coverage: 100% of Layer 1 ABI entry points have reference pages; install, user, SDK, and compatibility guides published and reviewed; documentation build is part of CI
- Alpha telemetry (opt-in) from at least 500 distinct machines over 60 days with crash-free session rate ≥ 97% measured and published
- Public RFC process has processed at least 5 external RFCs to decision; governance document, license, and trademark policy published
- Every V3 task is `done` or `dropped` with reason; roll-up shows 100%
- ADRs accepted for: installer disk layout and encryption, update channel policy, crash reporting privacy, Secure Boot key strategy, repository review and revocation policy, multi-user session model

### Demos

- Public install on a fresh Tier 1 laptop from USB with Secure Boot on and disk encryption, to a working desktop with Wi-Fi, in under 15 minutes wall clock
- Break an update deliberately; the machine boots the previous generation automatically and the updater reports the failure with the crash report link
- A third-party developer publishes a native package to the repository from the SDK and a user installs it from the store client with capability review
- Compatibility database probe from the live installer producing a verdict for an unknown machine
- Windows game and Linux IDE session on the AMD laptop, on battery, with Bluetooth headset, screen shared into a video call through the explicit screen-share capability

### Benchmark gates

- All prior benchmarks re-run across Tier 1; no regression greater than 10% without an accepted ADR; benchmark dashboards public
- Install time, first-boot time, and update apply time measured on every Tier 1 machine and published
- Delta update size for a typical alpha release measured and published; rollback time measured
- Boot time to login on each Tier 1 machine measured and published beside a mainline Linux distribution and Windows where dual-boot exists
- Crash-free session rate and mean time between kernel panics from opt-in telemetry published monthly
- Linux and Windows compatibility overhead re-measured on L4 and W2 corpora and published
- Energy and battery runtime re-measured on both laptops and published
- Full §54 list has a public number for every item on at least one Tier 1 machine; any missing item blocks the gate

### Compatibility gates

- L0–L3 corpora pass with zero regressions on Gold entries
- L4 corpus defined in compat/corpus/linux-L4.md: 300 applications drawn from Flathub and distribution popularity data plus the Steam-on-Linux top list without kernel anti-cheat; pass rate ≥ 85%
- W2 corpus defined in compat/corpus/windows-W2.md: 150 titles (110 games without kernel anti-cheat, 40 applications); ≥ 60% Gold or better, ≥ 80% Silver or better; a public per-title report with the rating scale published
- Community-submitted compatibility reports accepted into the database with a documented verification process

### Risks

- Installer edge cases (firmware quirks, existing partitions, dual boot, RAID/Optane) dominate support load
- Secure Boot signing via shim requires an external review process with uncertain timelines
- Crash reporting and telemetry create privacy and legal obligations that require governance before code
- Repository review at scale needs process and people, not just tooling
- NVIDIA driver strategy may force uncomfortable kernel-tree or licensing decisions
- Public exposure surfaces security issues in the young capability system faster than they can be fixed
- Documentation debt from V0–V2 becomes a wall

## V4 — Beta

**Purpose.** Freeze the Layer 1 ABI, complete every 1.0 feature, widen hardware, harden security through external audit, and demonstrate release-quality stability on a real beta fleet so that 1.0 is a declaration rather than a scramble.

**Hardware scope.** Tier 1: at least 10 named machines — AMD desktops (at least two GPU generations), Intel laptops (at least two generations), AMD laptops (at least two), one NVIDIA desktop, one Intel desktop — all fully tested each RC. Tier 2: compatibility-database machines with community verification and a documented promotion path to Tier 1. x86-64 only; ARM64 explicitly out.

### Scope in

- Layer 1 ABI freeze candidate: final review, removal of deprecated entry points, compatibility test suite, versioned Layer 2 interfaces locked for 1.x (ABI, IPC, GOV)
- Feature completion for everything promised in the 1.0 definition; feature freeze after the first release candidate (all workstreams)
- External security audit of kernel capability enforcement, IPC, personalities, installer, updater, and repository; all High/Critical fixed (SEC, KRN, LNX, WIN)
- Kernel hardening: exploit mitigations enabled and measured, unsafe-code inventory published, unsafe authority minimized per §51 (KRN, SEC)
- Hardware widening: Tier 1 to at least 10 machines including NVIDIA; Tier 2 process mature (HW, REL)
- Stability program: opt-in telemetry, crash-free targets, kernel panic tracking, flaky-test elimination (OBS, BLD)
- Upgrade path from V3 installs to V4 via generation update with data preserved (INS, PKG)
- Accessibility: screen reader, magnifier, keyboard-only operation, high contrast across shell and native apps; scripted task suite (ACC)
- Localization: shell and native apps in at least 10 languages; input methods for CJK (TXT)
- Documentation complete and reviewed for 1.0; API references generated from source in CI (GOV, SDK)
- Third-party ecosystem: developer program, at least 100 native packages from outside the core team, SDK bindings for C and one additional language (SDK, GOV)
- Release engineering: stable channel infrastructure, LTS branch policy, reproducible builds for all generation packages, signing key rotation drill (REL, BLD)
- Support policy drafts: support window, CVE SLA, HCL tiers, ABI stability statement (GOV, REL)

### Scope out

- New features after the first release candidate
- New Layer 1 ABI entry points
- ARM64/RISC-V ports
- Native filesystem or object store
- Native GPU driver rewrites
- Distributed interfaces
- Kernel anti-cheat support
- Enterprise management features

### Exit criteria

- Layer 1 ABI frozen: the ABI reference document is complete, every entry point has a conformance test, and a compatibility suite proves binaries built against the freeze candidate run on every subsequent beta build; the freeze ADR is accepted
- Layer 2 interface versions for 1.x are enumerated and locked; the interface-evolution test (old client/new service and new client/old service) passes for every core interface
- Feature freeze declared at RC1; every feature in the 1.0 definition is `done`; zero open P0/P1 at RC1, RC2, and the final beta
- External security audit report received; all High and Critical findings fixed and re-verified by the auditor; Medium findings triaged with public tracking; summary published
- Kernel fuzzing and IPC fuzzing have zero known open crashers for 30 consecutive days before the gate; unsafe-code inventory published with justification per block
- Beta fleet: at least 2,000 distinct opt-in machines over 60 days; crash-free session rate ≥ 99.5%; kernel panic rate below 1 per 1,000 machine-days; both published
- Two release-candidate cycles each with at least 14 days of soak on the full Tier 1 fleet with no P0 found; any P0 restarts the cycle
- In-place upgrade: at least 100 V3 alpha installs from the community upgraded to V4 through the update channel with user data preserved and rollback to V3 possible; success rate ≥ 99% measured
- Hardware: Tier 1 has at least 10 named machines (including at least one NVIDIA desktop and one laptop per vendor); every Tier 1 machine passes the full hardware test suite (display hot-plug, suspend/resume 500 cycles ≥ 99%, Wi-Fi, Bluetooth, audio, camera, USB classes, printing) each RC
- Accessibility: the 50-task assistive-technology script completes 100% with the screen reader across shell, settings, and every shipped native application; keyboard-only operation of the entire shell verified
- Localization: 10 languages at ≥ 95% string coverage for shell and native apps, CJK input methods functional in native and Linux-personality applications (tested)
- Reproducible builds: 100% of system-generation packages reproduce bit-for-bit on two independent builders; a public verifier tool is available
- Signing key rotation drill executed on the testing channel without breaking updates
- Documentation: user, admin, SDK (Rust and C), compatibility, security, and ABI references complete; a documentation review sign-off recorded; broken-link and outdated-example checks pass in CI
- Ecosystem: at least 100 externally published native packages; at least 25 external contributors merged in the milestone; developer program documentation published
- Support policy documents (support window, CVE SLA, HCL tiers, ABI stability statement, deprecation policy) drafted, reviewed via RFC, and accepted
- Every V4 task is `done` or `dropped` with reason; roll-up shows 100%

### Demos

- Compatibility suite run: a binary built against RC1 executes unchanged on the final beta, with the ABI conformance report displayed
- Fleet dashboard: live crash-free rate, panic rate, and update success across the beta fleet
- Upgrade a real V3 alpha laptop to the final beta over the air and roll it back
- Accessibility session: an assistive-technology user completes daily tasks across the shell and native apps
- Ten-machine hardware lab run of the full test suite with published per-machine results
- A native package published by an external developer in a non-Rust SDK language, installed with capability review, running with tracing visible

### Benchmark gates

- All prior benchmarks re-run on all Tier 1 machines; any regression greater than 5% versus V3 blocks the gate unless an accepted ADR explains it
- Every §54 metric published for every Tier 1 machine with comparison to Linux and, where dual-boot exists, Windows; macOS comparison published for the metrics where a comparable machine class exists
- Component creation p50 ≤ 100 µs, task creation p50 ≤ 2 µs, IPC same-core p50 ≤ 2 µs, cross-core p50 ≤ 5 µs on the reference desktop; regressions block
- Application warm startup p50 ≤ 20 ms for Terminal and Editor; input-to-photon p99 ≤ one frame interval plus 4 ms at 60 Hz and at maximum refresh on every Tier 1 machine; compositor frame deadline misses under 0.1%
- Environment startup cached p50 ≤ 50 ms
- Update apply time, rollback time, install time published per Tier 1 machine
- Energy: idle and mixed-workload battery runtime published for every Tier 1 laptop beside mainline Linux on the same machine
- Security mitigation overhead measured (mitigations on vs off) and published
- Public benchmark dashboards updated on every RC

### Compatibility gates

- L0–L4 corpora pass with zero regressions on Gold entries
- L5 corpus defined in compat/corpus/linux-L5.md: 500 applications; pass rate ≥ 90% with integration scoring; 100% for the browser, IDE, container runtime, Steam client, and office suite entries
- W3 corpus defined in compat/corpus/windows-W3.md: 300 titles (220 games without kernel anti-cheat, 80 applications); ≥ 70% Gold or better, ≥ 85% Silver or better; per-title public reports
- Zero Gold-to-lower regressions on W2 entries versus V3
- Compatibility corpora and results are published in machine-readable form and the pass rates are reproducible by third parties using the published scenario scripts

### Risks

- ABI freeze pressure versus late discoveries from the beta fleet; the freeze must hold even when a cleaner design appears
- External audit findings in capability enforcement or personalities can require deep changes late
- Long-tail hardware bugs across 10 machines multiply the test matrix and consume release engineering
- Fleet size and crash-free targets depend on community adoption that cannot be scheduled
- Documentation and localization are labor-heavy and easy to defer until they block the gate
- Ecosystem thresholds (100 external packages, 25 contributors) depend on external interest
- Reproducible builds across the entire inherited Linux toolchain are harder than for new code

## 1.0 — Public Stable

**Purpose.** Declare the first supported release: frozen Layer 1 ABI, published support window and security SLA, tiered hardware compatibility, measured compatibility coverage, guaranteed atomic update and rollback, and complete documentation, with every claim backed by a published measurement.

**Hardware scope.** Tier 1: the published HCL of at least 10 named x86-64 machines (AMD and Intel desktops and laptops, at least one NVIDIA desktop) with every listed feature tested each release. Tier 2: community-verified machines in the compatibility database, best effort. Everything else unsupported; the installer warns. No ARM64, no RISC-V.

### Scope in

- Release-candidate soak, final signing, stable channel launch, release notes and migration guide (REL)
- Layer 1 ABI declared stable with the 1.x compatibility guarantee; Layer 2 interface versions declared supported for 1.x (ABI, GOV)
- Support commitments in force: 24-month security support window, CVE SLA, HCL tiers, deprecation policy (GOV, REL, KRN)
- Security response operating publicly with advisories and a disclosure program (SEC, REL)
- Final HCL publication and hardware test-lab results per Tier 1 machine (HW, REL)
- Final compatibility reports for L5 and W3 (LNX, WIN)
- Update and rollback guarantees verified by fault injection on every Tier 1 machine (INS, PKG, BOOT)
- 1.x maintenance branch, backport policy, and the 2.0 planning process opened (GOV, KRN)

### Scope out

- Any feature not already complete at V4 feature freeze
- ARM64/RISC-V
- Kernel anti-cheat, vendor DRM
- Native filesystem, native GPU stack, native browser, native IDE
- Distributed/remote interfaces
- Hardware capability enforcement (CHERI-class) — only ABI room is preserved
- Performance superiority claims not backed by published measurements
- Universal PC compatibility

### Exit criteria

- Final release candidate has soaked at least 30 days on the full Tier 1 fleet and the beta fleet with zero open P0/P1 and no P2 without a documented workaround in the release notes
- Layer 1 ABI stability statement signed off: the ABI reference, conformance suite, and compatibility suite pass; the ABI freeze ADR is amended to 'stable for 1.x'; a public policy states that Layer 1 changes require a major version
- Layer 2 supported interface versions listed in a published compatibility document; deprecation policy (minimum two minor releases of overlap) published
- Support window (≥ 24 months of security and critical fixes), CVE SLA (High/Critical inherited CVEs shipped within 14 days target), and HCL tiers published; the trailing 90 days of CVE response meet the SLA for ≥ 90% of applicable CVEs with the distribution published
- Security: external audit summary published; all High/Critical closed; no known open fuzzer crasher for 30 days; disclosure address and advisory feed live; at least one advisory has been published through the process
- Update and rollback guarantee verified: on every Tier 1 machine a fault-injected failing generation boots back to the previous generation automatically; N ≥ 3 previous generations remain bootable; rollback preserves user data; update success rate from the beta fleet ≥ 99.9% published
- Installer with full-disk encryption and Secure Boot succeeds on every Tier 1 machine; HCL publishes Tier 1 (≥ 10 machines, all features listed working) and Tier 2 (community) with a documented promotion path
- Compatibility: L5 ≥ 90% pass, W3 ≥ 70% Gold-or-better and ≥ 85% Silver-or-better on Tier 1, zero regressions on V4 Gold entries; per-title reports and scenario scripts published
- Stability: opt-in telemetry crash-free session rate ≥ 99.5% and kernel panic rate below 1 per 1,000 machine-days over the final 30-day soak, published
- Documentation complete: Layer 1 ABI reference (100% coverage), SDK guides for Rust and C, user guide, administrator guide, compatibility guide, security guide, release notes with migration path from V3 and V4; documentation CI green
- Reproducible builds: 100% of 1.0 system-generation packages reproduce bit-for-bit on two independent builders; a third party has reproduced the release image and the result is published
- Every §54 metric published for every Tier 1 machine against Linux, Windows (where dual-boot exists), and macOS (where a comparable class exists); the release announcement contains no performance claim without a link to the measurement
- Accessibility and localization gates from V4 hold on the release build (50-task assistive script 100%, 10 languages ≥ 95%)
- Governance: license, trademark, contribution, RFC, and security policies published; 1.x maintenance branch created with backport rules; a 2.0 planning RFC opened listing the explicitly deferred items
- Every 1.0 task is `done` or `dropped` with reason; roll-up shows 100%; all earlier milestones show 100%

### Demos

- Release ceremony: signed 1.0 image reproduced independently by a third party, checksums matching, then installed on a Tier 1 laptop with Secure Boot and disk encryption
- Compatibility proof: a native application built against the V4 ABI freeze runs unmodified on 1.0, shown with the conformance report
- Guarantee proof: pull the power during an update on a Tier 1 desktop; the machine boots the previous generation, then completes the update on retry
- Coverage proof: the public HCL, L5 and W3 dashboards, benchmark dashboards, and security advisory feed shown live
- A full working day on 1.0: native development in `os env`, Linux IDE and browser, a Windows game with HDR and VRR, Bluetooth audio, screen-shared meeting, AI-assisted semantic automation with visible capability grants, then rollback of a package change from `os history`

### Benchmark gates

- No regression greater than 5% versus V4 final on any tracked benchmark on any Tier 1 machine; any exception requires an accepted ADR referenced in the release notes
- Component creation p50 ≤ 100 µs, task creation p50 ≤ 2 µs, IPC same-core p50 ≤ 2 µs and cross-core p50 ≤ 5 µs on the reference desktop, published
- Terminal and Editor warm startup p50 ≤ 20 ms; cached environment startup p50 ≤ 50 ms; published per Tier 1 machine
- Input-to-photon p99 ≤ one frame interval plus 4 ms at 60 Hz and maximum refresh; compositor deadline misses under 0.1%; published per Tier 1 machine
- Linux compatibility overhead (L5 workloads) and Windows compatibility overhead (W3 Gold titles FPS) published beside upstream Linux, Linux+Proton, and Windows on the same hardware
- Energy: idle and mixed-workload battery runtime published for every Tier 1 laptop beside mainline Linux and Windows where possible
- Boot, install, update, and rollback times published per Tier 1 machine
- The benchmark methodology, raw data, and scripts are published so third parties can reproduce every number

### Compatibility gates

- L0–L5 corpora at V4 thresholds with zero regressions on Gold entries; L5 ≥ 90%
- W3 ≥ 70% Gold or better, ≥ 85% Silver or better; zero Gold-to-lower regressions versus V4
- Integration requirements hold for every passing Linux and Windows entry: launcher, taskbar, notifications, clipboard, audio, file chooser, input, scaling, and accessibility where the toolkit exposes it
- Published statement of what is not supported: kernel-level anti-cheat, vendor DRM, and any title or application marked Broken, with reasons

### Risks

- Support commitments convert engineering into a permanent obligation (§56.4); the team must be staffed for 24 months of maintenance before declaring
- Upstream CVE cadence may exceed the SLA if the fork has diverged in affected subsystems
- Last-minute driver or firmware refreshes for Tier 1 machines can regress hardware tests during the soak
- Compatibility thresholds depend on external software that changes under the project (Steam, browsers, Wine upstream)
- Reproducibility of the full inherited toolchain may break on a late dependency update
- Governance and legal items (trademark, licensing of inherited code, signing keys) can block the declaration independently of engineering readiness

