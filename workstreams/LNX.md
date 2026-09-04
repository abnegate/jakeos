# LNX · Linux personality
- Prefix: LNX
- Lead: none
- Baseline: §3, §36, §46, §47, §49, §56.3

<!-- roadmap:generated:begin summary -->
Tasks: 111 live, 0 done, 0 in-progress, 111 todo, 0 dropped. Ready: 1. Blocked: 110. Weighted: 0%.
<!-- roadmap:generated:end -->

## Scope

LNX owns the Linux personality: the product that runs existing Linux software while native software sees none of it (§3, §56.3). Phase A retains the Linux syscall ABI, ELF and glibc loading, and POSIX process, fd, signal, proc, sys, socket, fork, exec, epoll and path surfaces so unmodified userspace boots beside native Components. Later phases make Linux a Personality with its own entry path and translate those surfaces onto native primitives (§6, §46).

The workstream also owns the Linux environment as a product: D-Bus, Wayland and Xwayland hosting (primary selection stays inside the bridge), PipeWire client sockets, xdg-desktop-portal onto native choosers, namespaces, seccomp, overlayfs, FUSE, ptrace gated by a debug Capability, inotify, ia32, OCI and Flatpak, systemd and logind assumptions, PAM/NSS, fontconfig, FHS and XDG views, the Steam runtime sub-corpus, and the L0 through L5 Corpora plus B-026 overhead publication. Compatibility UX (launcher, clipboard, notifications, scaling, file chooser) is integrated here and consumed by APP and UIP (§47, §49).

## Out of scope

Kernel fork, retained-mechanism inventory and divergence phases (KRN). Native ABI entry, dual-world tag and native syscall filter (ABI). Capability encoding, tables and debug-attach rights (CAP). Component creation and the process-mapping Decision (CMP). ResourceDomain accounting (SCH). Channels and IDL (IPC). File, Directory, UserSelected and the personality view API (STO). Package immutability and personality packaging (PKG). Compositor, DRM/KMS, HDR Surfaces and brokered RenderQueue (GFX). Native UI protocol, clipboard Capability and toolkit (UIP). FontMatcher, IME host and fontconfig generation (TXT). Screen reader and AT-SPI consumption (ACC). Native init, supervisor and settings store (SVC). PipeWire/PulseAudio server and AudioStream (AUD). Camera and codec Components (MED). NetworkConnection, netmgr and sshd listen Capability (NET). Power service, battery and inhibitors (PWR). HID, print discovery and Device objects (HW). Wine, Proton and Win32 (WIN). VM manager and guest tools (VIRT). `os env` composition (ENV). Threat model, identity, Session and secrets (SEC). Crash capture format (OBS). Native debugger UI (SDK). Launcher, Notify service, chooser UI and print dialog (APP). Corpus legal review (GOV). CI plumbing and QEMU matrix (BLD). Benchmark register (BEN). Compatibility database publication (REL). Docs site (DOC). Hardware lab (LAB).

## Tasks

### LNX-001 · Retain Linux syscall ABI for unmodified userspace
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: KRN-002, KRN-010, KRN-011, KRN-017
- Baseline: §3, §6, §46, §59
- Risks: R-004
- Invariants: I-006, I-010, I-049

Phase A keeps the Linux syscall ABI, ELF loading including glibc dynamic linking, and the POSIX process, PID, file-descriptor, signal, `/proc`, `/sys`, socket, `fork`, `exec`, epoll and path surfaces so unmodified Linux userspace boots beside native Components (§3, §46, §59). Native software still sees none of these. The retained path is the C-001 detector for native hooks that would otherwise regress Linux behaviour (R-004).

<!-- covers: INV-0004, INV-0082, INV-0083, INV-0084, INV-0085, INV-0086, INV-0087, INV-0088, INV-0089, INV-0090, INV-0091, INV-0092, INV-0093, INV-0105, INV-0849, INV-0858, INV-1168, INV-0865 -->

#### Out of scope
Native ABI entry layer (ABI-002). Translation onto native primitives (LNX-090). L0 scenario scripts (LNX-002). Retained-mechanism inventory (KRN-017).

#### Acceptance criteria
- [ ] An unmodified glibc-linked ELF starts through the Linux syscall ABI on CI matrix entries `qemu-x86_64` and `hw-h002`.
- [ ] `fork`, `exec`, epoll, a listening BSD socket, `/proc/self` and `/sys` are available to that process and not to a native Component started in the same image.
- [ ] A native Component in the same boot has no Linux syscall table and cannot open a POSIX path.

#### Verification
- Integration: `personality:tests/retain/syscall_abi_*` on `qemu-x86_64` and `hw-h002`.
- Compat: C-001 smoke subset on H-001 and H-002.
- Review: KRN lead confirms the retained syscall path is listed in the retained-mechanism inventory.

#### Evidence
- none

### LNX-002 · Run the L0 Corpus with zero regressions
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: LNX-001, BLD-012, KRN-014, BEN-006
- Baseline: §6, §46, §59
- Corpora: C-001
- Risks: R-004
- Invariants: I-096

C-001 is the V0 compatibility firewall: the LTP syscall subset plus busybox, bash, coreutils, python3 and a static Go binary must match the unforked kernel of the same version on the same hardware (§6, §59). Results land under `reports/compat/C-001/`. Native hooks that regress this corpus fail the merge.

<!-- covers: INV-0141 -->

#### Out of scope
Scenario CI plumbing after V0 (BLD-017). L1 GUI scenarios (LNX-007). kselftests for retained subsystems (KRN-014).

#### Acceptance criteria
- [ ] C-001 runs on H-001 and H-002 against the unforked baseline of the same kernel version.
- [ ] The committed report meets the C-001 V0 threshold in the register.
- [ ] A native-hook patch that drops a previously passing C-001 case fails CI.

#### Verification
- Compat: C-001 scenario `compat:linux-L0` on H-001 and H-002.
- Integration: `personality:tests/corpus/l0_*` on `qemu-x86_64` and `hw-h002`.
- Review: KRN lead signs the zero-regression comparison against the unforked baseline.

#### Evidence
- none

### LNX-003 · Decide Linux Personality depth and translation phase
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: LNX-009, LNX-011, KRN-009
- Baseline: §6, §46, §56.3
- Decision: D-0178
- Invariants: I-025

One Decision for per-milestone depth and the Phase B through D move from a direct Linux syscall ABI to translation onto native primitives (§6, §46). Options are measured by LNX-009. The accepted option must precede V1 Phase B and V2 translation.

<!-- covers: GAP-0502, INV-0863 -->

#### Out of scope
Kernel strategy (KRN-002). Dual-world entry tag (ABI-025). Syscall translation implementation (LNX-090).

#### Acceptance criteria
- [ ] Options evaluated include in-kernel retain of the Linux syscall path, in-kernel translation onto native primitives, and a gVisor-style userspace Personality, each mapped onto kernel phases B through D.
- [ ] The accepted option names the depth in force at V1, V2 and V3 and the B-026 report it cites.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: kernel architecture lead and LNX lead sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- none

### LNX-004 · Decide Wayland hosting and X11 via Xwayland
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: LNX-010, GFX-020, LNX-011
- Baseline: §41, §47, §57, §60
- Decision: D-0185
- Risks: R-020
- Invariants: I-048

One Decision for how a Wayland Linux app becomes a native compositor window, and that X11 arrives only via Xwayland (§47, §60). GFX decides whether the compositor speaks Wayland or a bridge translates; this Decision chooses nested versus in-compositor hosting for the Personality and records that Wayland is not the native UI API (§41, §57).

<!-- covers: GAP-0504 -->

#### Out of scope
Compositor serving model (GFX-020). Bridge implementation (LNX-006). Xwayland server (LNX-053). Native UI protocol (UIP).

#### Acceptance criteria
- [ ] Options evaluated include a nested compatibility compositor, in-compositor Wayland serving, and a translating bridge Component, each with an X11-via-Xwayland-only variant and an X11-as-native-UI variant that is rejected.
- [ ] The accepted option states that native software does not see Wayland or X11 objects.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: GFX lead and LNX lead sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- none

### LNX-005 · Bound POSIX authority by Component capabilities
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: LNX-001, LNX-011, CAP-005, CAP-007, CMP-005, SEC-002
- Baseline: §3, §9.1, §46
- Risks: R-025
- Threats: T-011, T-025
- Invariants: I-021, I-072

A Linux process must not reach a resource its enclosing Component was not granted, regardless of uid or file mode (§9.1, §46). Compatibility inherits history; it must not become the ambient-authority hole that makes the Capability model hollow (T-011).

<!-- covers: GAP-0188 -->

#### Out of scope
Default Capability bundle for unmanifested apps (LNX-013). Portal upgrades (LNX-036). Native ambient-authority lint (CAP-016).

#### Acceptance criteria
- [ ] A Linux process whose Component lacks a network Capability cannot create a connected socket, including as uid 0.
- [ ] A Linux process whose Component lacks a File Capability cannot open a path its uid could open on upstream Linux, and the denial is `Error::Rights` at the Personality boundary.
- [ ] `os inspect` shows the enclosing Component Capability set as the bound, not the process uid.

#### Verification
- Unit: `personality:tests/authority/posix_bound_*` on `qemu-x86_64` and `hw-h002`.
- Integration: negative cases for network, file and `/proc` of another Component on `qemu-x86_64`.
- Review: CAP lead confirms the bound is the Component grant set.

#### Evidence
- none

### LNX-006 · Bridge Wayland apps as native compositor windows
- Type: build
- Milestone: V0.5
- Status: todo
- Size: L
- Owner: none
- Depends on: LNX-004, LNX-005, GFX-010, GFX-008, MEM-025, UIP-003, ABI-025, HW-011
- Baseline: §40, §41, §47, §60
- Risks: R-020
- Invariants: I-048

V0.5 proves a Wayland Linux app as a normal compositor window with input and clipboard, using existing Linux mechanisms underneath rather than native integration (§47, §60). Buffers cross as MemoryObjects. Native software still does not speak Wayland.

<!-- covers: INV-0758, INV-0761, INV-0870, INV-0878, INV-1191 -->

#### Out of scope
V1 compositor-native polish (LNX-040). Xwayland (LNX-053). GPU acceleration beyond dma-buf import (LNX-031). Native clipboard policy (UIP-003).

#### Acceptance criteria
- [ ] A Wayland client from C-002 appears as a compositor-managed window on H-002 and on `qemu-virtio-gpu`, with keyboard and pointer input.
- [ ] Copy from that window and paste into a native Text Editor succeeds when both hold the clipboard Capability; a native Component without it cannot read the contents.
- [ ] Native toolkit and compositor crates do not link Wayland client or server libraries.
- [ ] Killing the compositor rebinds the Linux window without the client exiting, using the V0.5 rebind contract.

#### Verification
- Integration: `personality:tests/wayland/window_input_clipboard_*` on `qemu-virtio-gpu` and `hw-h002`.
- Demo: V0.5 Wayland-app-as-native-window demo on H-002.
- Compat: C-002 GUI window/input/clipboard scoring on H-002.

#### Evidence
- none

### LNX-007 · Define and run the L1 Corpus
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: LNX-002, LNX-006, BLD-017
- Baseline: §47, §49, §60
- Corpora: C-002
- Invariants: I-096

C-002 is the V0.5 compatibility gate: named CLI and developer tools plus Wayland GUI apps with window, input and clipboard scoring. Thresholds live in the register; this task scripts the scenarios and commits the report.

#### Out of scope
L2 daily-driver apps (LNX-056). Guest-agent plumbing (BLD-017). Launcher chrome (APP-013).

#### Acceptance criteria
- [ ] C-002 scenario scripts exist under the `compat:linux-L1` harness and name every CLI and GUI entry in the register.
- [ ] The committed report on H-001 and H-002 meets the C-002 V0.5 threshold.
- [ ] C-001 remains at its V0.5 zero-regression threshold on the same runs.

#### Verification
- Compat: C-002 scenario `compat:linux-L1` on H-001, H-002 and H-003.
- Integration: `personality:tests/corpus/l1_*` on `qemu-x86_64` and `qemu-virtio-gpu`.

#### Evidence
- none

### LNX-008 · Inventory Linux environment gaps by application class
- Type: spike
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: LNX-001, LNX-011
- Baseline: §56.3

Running binaries is not a complete Linux environment (§56.3). This spike inventories required environment pieces for CLI, GUI, games, IDEs and containers so V1 product work is scoped from evidence rather than from a syscall list.

<!-- covers: INV-1081 -->

#### Out of scope
glibc and systemd survey of top desktop apps (LNX-059). Depth and translation measurement (LNX-009).

#### Acceptance criteria
- [ ] The report lists environment components per class (CLI, GUI, games, IDEs, containers) and tags each as retained, emulated, bridged or out of V1.
- [ ] The report names the V1 tasks that close each V1-tagged gap.
- [ ] The report is committed at `reports/spikes/LNX-008.md`.

#### Verification
- Report: what stands between a booting ELF and a complete environment per class; which systemd, D-Bus, portal, namespace and FHS pieces V1 must ship; which pieces wait for V2 translation.
- Review: LNX lead records that the V1 task list covers every V1-tagged gap.

#### Evidence
- none

### LNX-009 · Measure Linux Personality implementation depth options
- Type: spike
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: LNX-001, LNX-011, BEN-006, Q-001
- Baseline: §6, §46, §54
- Benchmarks: B-026
- Explores: S-030

GAP-0502 requires measured syscall overhead for in-kernel retain, in-kernel translation and a gVisor-style userspace Personality before the depth Decision. Numbers live only in the B-026 method; this report answers which option V1 can ship.

<!-- covers: GAP-0502 -->

#### Out of scope
The depth Decision (LNX-003). Register ownership (BEN-027). Phase B entry path (LNX-030).

#### Acceptance criteria
- [ ] The report measures all three options with the B-026 method on H-001 and H-002 and cites B-026, with no superiority claim.
- [ ] The report states maintenance and security trade-offs per option against phases B through D.
- [ ] The report is committed at `reports/spikes/LNX-009.md`.

#### Verification
- Report: overhead of retain versus in-kernel translation versus userspace Personality on the B-026 microbenchmark set; which option remains viable for V1 daily-driving; what S-030 must expose for later translation.
- Bench: B-026 method on H-001 and H-002; publish-only.
- Review: BEN lead confirms the method matches the register.

#### Evidence
- none

### LNX-010 · Prototype nested versus in-compositor Wayland hosting
- Type: spike
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: LNX-001, GFX-010, MEM-025
- Baseline: §40, §47, §60
- Explores: S-030
- Risks: R-020

The V0.5 Wayland-app-as-native-window gate cannot switch compositor architecture later. This prototype builds nested and in-compositor hosting far enough to compare input, dma-buf import and crash-rebind, and informs both GFX-020 and LNX-004.

<!-- covers: GAP-0504 -->

#### Out of scope
Hosting Decision (LNX-004). Compositor serving Decision (GFX-020). Production bridge (LNX-006).

#### Acceptance criteria
- [ ] The report runs one Wayland client under nested hosting and under in-compositor hosting on H-003 and records input, buffer import and kill-rebind behaviour for each.
- [ ] The report names which option the V0.5 gate can freeze without a later compositor rewrite.
- [ ] The report is committed at `reports/spikes/LNX-010.md`.

#### Verification
- Report: nested versus in-compositor hosting for input, dma-buf, rebind and test-matrix cost (R-020); whether Xwayland can sit in the same host; what remains a V1 polish item.
- Review: GFX lead records that the prototype does not force Wayland into the native UI protocol.

#### Evidence
- none

### LNX-011 · Treat Linux compatibility as a product
- Type: docs
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: none
- Baseline: §3, §47, §49, §56.3
- Invariants: I-025, I-096

Linux compatibility is a product with an owner, a conformance suite, UX acceptance criteria and a release-note template, not a syscall shim (§56.3). This document also records that all Linux GUI machinery belongs to the Personality, not the native platform (§47), so later Wayland, portal and D-Bus work cannot leak into UIP or GFX.

<!-- covers: INV-1080, INV-0876 -->

#### Out of scope
Corpus scenario scripts (LNX-002). Upstream engagement policy (LNX-028). Five-minute launch guide (LNX-054).

#### Acceptance criteria
- [ ] A committed product brief names the LNX owner, the C-001 through C-006 and C-010 Corpora, UX integration checks from §49, and a release-note template with a compatibility section.
- [ ] The brief states that Wayland, X11, D-Bus, PipeWire clients, portals and FHS views live in LNX and are not native APIs.
- [ ] Native software is recorded as never depending on the Personality.

#### Verification
- Review: architecture lead and LNX lead sign-off recorded on the pull request.
- Manual: the brief is cited by LNX-002 and LNX-006.

#### Evidence
- none

### LNX-012 · Decide the Linux Personality container engine Surface
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: LNX-045, LNX-017, Q-032
- Baseline: §36, §56.3
- Decision: D-0171
- Invariants: I-019

One Decision for which container engine surface the Personality exposes (Docker socket versus podman versus containerd) so the L2 corpus can require an OCI runtime without making containers a native platform feature (§36). Q-032 is answered here.

<!-- covers: INV-0675, INV-0676 -->

#### Out of scope
OCI runtime integration (LNX-039). Native isolation (CMP, SCH). VM fallback (VIRT).

#### Acceptance criteria
- [ ] Options evaluated include a Docker socket API, podman-compatible tooling without a Docker socket, and containerd with a thin CLI, each hosted only inside the Personality.
- [ ] The accepted option states that OCI is never a native platform feature (I-019) and marks Q-032 answered by this task.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: LNX lead and ENV lead sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- none

### LNX-013 · Decide the default Capability bundle for Linux apps
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: LNX-005, LNX-018, CAP-007, SEC-006, SEC-002, Q-039
- Baseline: §9.1, §46, §47
- Decision: D-0172
- Threats: T-011, T-025
- Invariants: I-021, I-072

Unmanifested Linux apps need a default directory, network and device bundle plus portal upgrades. The wrong default either breaks the L2 corpus or hollows §9. Q-039 is answered here.

<!-- covers: GAP-0231, INV-0868, INV-0933 -->

#### Out of scope
Applying the bundle (LNX-025). Portal implementation (LNX-036). Windows default bundle (WIN).

#### Acceptance criteria
- [ ] Options evaluated include a narrow bundle upgraded only via portals, a home-and-network bundle with user-visible grants, and an ambient-home bundle, the last rejected against I-021.
- [ ] The accepted option lists the default directories, network and devices, how a portal request upgrades the set, and marks Q-039 answered by this task.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: CAP lead and LNX lead sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- none

### LNX-014 · Decide /dev, sysfs and udev coexistence with native drivers
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: LNX-008, HW-009, Q-028
- Baseline: §33, §46, §56.3
- Decision: D-0173
- Risks: R-016

One Decision for how a user-space native driver coexists with Personality `/dev`, sysfs and udev for the same device, covering GPU and input at V1. Q-028 is answered here.

<!-- covers: INV-0625 -->

#### Out of scope
Providing nodes and udev (LNX-044). Native Device objects (HW). Brokered RenderQueue (GFX-057).

#### Acceptance criteria
- [ ] Options evaluated include Personality-only nodes with native Device objects beside them, a translating `/dev` over Object<Device>, and shared raw nodes for both worlds, the last rejected for native software.
- [ ] The accepted option states how GPU and input at V1 are enumerated in each world and marks Q-028 answered by this task.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: HW lead, GFX lead and LNX lead sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- none

### LNX-015 · Decide whether ia32 emulation is retained
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: LNX-001, KRN-017, KRN-011
- Baseline: §46, §56.3
- Decision: D-0174
- Risks: R-032

One Decision on whether ia32 emulation is retained, made before any syscall-pruning work, because Steam and many Windows titles depend on it. H-016 holds the choice in CI (R-032).

<!-- covers: EXTRA-028 -->

#### Out of scope
Shipping the ia32 userland (LNX-035). 32-bit Win32/WoW64 (WIN-010). Attack-surface pruning (KRN-044).

#### Acceptance criteria
- [ ] Options evaluated include retain ia32 in the fork and in CI on H-016, drop ia32 from 1.0, and provide ia32 only via VIRT fallback.
- [ ] The accepted option states whether syscall pruning may disable ia32 and what H-016 runs.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: KRN lead and LNX lead sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- none

### LNX-016 · Decide how native applications opt into a Personality
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: LNX-003, ABI-025, CAP-007, Q-002
- Baseline: §3, §46
- Decision: D-0176
- Invariants: I-025

One Decision for how a native application explicitly opts into a compatibility environment (Capability to a Personality, embedded Linux Component, or SDK shim) and what authority that grant carries. Q-002 is answered here. S-030 records the shape.

<!-- covers: INV-0102 -->

#### Out of scope
Incremental native-interface adoption by Linux apps (LNX-062). Dual-world entry implementation (ABI-025).

#### Acceptance criteria
- [ ] Options evaluated include a Capability to the Personality, an embedded Linux Component, and an SDK shim, each with the authority the grant carries.
- [ ] The accepted option states that native software sees no POSIX unless it holds that grant, and marks Q-002 answered by this task.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: ABI lead and LNX lead sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- none

### LNX-017 · Decide first-class Linux packaging formats
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: LNX-023, PKG-047, LNX-008
- Baseline: §28, §36, §47, §56.3
- Decision: D-0177

One Decision for which Linux packaging formats the Personality supports for end-user installation (Flatpak, AppImage, deb/rpm via container, Nix) and which is first-class. The L2 corpus includes Flatpak.

<!-- covers: GAP-0424 -->

#### Out of scope
Immutable packaging mechanics (PKG-047). Shipping Flatpak runtimes (LNX-058). Flatpak/Snap spike beyond V1 (LNX-087).

#### Acceptance criteria
- [ ] Options evaluated include Flatpak first-class, AppImage first-class, deb/rpm via a distro container, and Nix, with a named first-class path.
- [ ] The accepted option states how each format is confined to the Personality and how it satisfies PKG immutability.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: PKG lead and LNX lead sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- none

### LNX-018 · Decide xdg-desktop-portal as the native grant bridge
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: LNX-004, STO-034, UIP-003, SEC-002
- Baseline: §9.1, §25, §47
- Decision: D-0179
- Threats: T-002
- Invariants: I-035

One Decision on whether xdg-desktop-portal is the bridge through which Linux apps reach the native Capability-based file chooser, clipboard and screen-share grants. Portals are the sandbox interface Linux apps already speak (T-002).

<!-- covers: GAP-0505 -->

#### Out of scope
Portal implementation (LNX-036). Chooser UI (APP-002). Native UserSelected minting (STO-034).

#### Acceptance criteria
- [ ] Options evaluated include portals as the sole grant bridge, a second Personality-specific permission model, and ambient home access for Linux GUI apps, the last rejected against I-035.
- [ ] The accepted option names which portal interfaces map onto UserSelected, Notify and screen-share Capabilities.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: STO lead and LNX lead sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- none

### LNX-019 · Decide the POSIX path view of native storage
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: STO-047, STO-042, LNX-013, Q-018
- Baseline: §25, §46
- Decision: D-0180
- Invariants: I-016

One Decision for how the Personality presents native storage objects as POSIX paths while respecting Capability scope. STO owns the objects and the view API; LNX owns the path facade. Q-018 is answered here.

<!-- covers: INV-0493 -->

#### Out of scope
View API (STO-047). Three-view mapping (STO-042). Home and XDG presentation (LNX-047). Mount synthesis (LNX-088).

#### Acceptance criteria
- [ ] Options evaluated include a Capability-scoped path facade, a copy-on-first-use tree, and a global FHS as native storage, the last rejected against I-016.
- [ ] The accepted option states that a path check is never a native authority check and marks Q-018 answered by this task.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: STO lead and LNX lead sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- none

### LNX-020 · Decide X11 primary selection stays inside the bridge
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: LNX-004, UIP-003, SEC-002
- Baseline: §41, §47, §57
- Decision: D-0181
- Threats: T-032
- Invariants: I-048

One Decision that X11 primary selection is emulated inside the Wayland/X11 bridge and never crosses into the native clipboard (T-032, I-048, S-032).

<!-- covers: EXTRA-047 -->

#### Out of scope
Native clipboard Capability (UIP-003). Wayland `wl_data_device` (LNX-006). Clipboard history UI (APP-024).

#### Acceptance criteria
- [ ] Options evaluated include primary selection emulated only inside the bridge, merging primary into the native clipboard, and dropping primary selection.
- [ ] The accepted option states that a native Component without a clipboard Capability cannot read primary-selection contents.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: UIP lead and LNX lead sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- none

### LNX-021 · Decide retention and exposure of seccomp, user namespaces, overlayfs and FUSE
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: LNX-001, KRN-017
- Baseline: §3, §36, §46
- Decision: D-0346
- Risks: R-025, R-032
- Invariants: I-006

V1 GPU-accelerated Linux browser and IDE sandboxes require a Decision on whether seccomp-bpf, user namespaces, overlayfs and FUSE stay in the kernel and how they are exposed (§46). Native software never sees these as the native isolation API (§3, I-006). LNX-061 runs after this Decision.

#### Out of scope
Verification of the retained set (LNX-061). Portal strategy (LNX-018). Native Component isolation (CMP).

#### Acceptance criteria
- [ ] Option A (retain all four, Personality-only), option B (retain a subset and emulate the rest) and option C (expose the four as native objects) are evaluated; option C is available to reject.
- [ ] The accepted option names each of seccomp-bpf, user namespaces, overlayfs and FUSE as retained, Personality-only, or declined.
- [ ] Architecture review sign-off is recorded on the pull request.

#### Verification
- Review: LNX, KRN and SEC leads sign off on the pull request.

#### Evidence
- none

### LNX-022 · Decide terminal-session authority for Linux programs
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: LNX-013, LNX-016, APP-004, SEC-002
- Baseline: §9.1, §35, §46
- Decision: D-0182
- Invariants: I-021, I-072

The V1 developer terminal is the one place users expect ambient Linux authority. One Decision records the authority a terminal session confers on Linux programs it launches and how a developer escalates or attenuates it, so §9 does not erode.

<!-- covers: GAP-0311 -->

#### Out of scope
Native Terminal chrome (APP). Default bundle for GUI apps (LNX-013). `os env` namespaces (ENV).

#### Acceptance criteria
- [ ] Options evaluated include an ambient Linux environment scoped by the terminal's own Capabilities, a per-command grant prompt, and an unbounded uid-0 shell, the last rejected against I-021.
- [ ] The accepted option states how a developer escalates or attenuates the session and how `os inspect` shows the grant.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: SEC lead and LNX lead sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- none

### LNX-023 · Decide source-built versus redistributed Linux userland
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: LNX-011, LNX-008, GOV-031
- Baseline: §50, §56.3
- Decision: D-0183

One Decision on whether the Personality userland is built from source or redistributes an existing distribution's binaries, and the resulting trademark and licence obligations. It locks the V1 userland.

<!-- covers: GAP-0022 -->

#### Out of scope
Trademark in product naming (GOV-051). Corpus redistribution legality (GOV-031). Packaging formats (LNX-017).

#### Acceptance criteria
- [ ] Options evaluated include a source-built userland, redistributing a named distribution's binaries, and a hybrid with a named boundary.
- [ ] The accepted option records trademark and source-offer duties and names the V1 glibc, Mesa and coreutils origin.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: GOV licensing reviewer and LNX lead sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- none

### LNX-024 · Decide glibc /usr/lib interoperation with Packages
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: LNX-023, PKG-047, Q-022
- Baseline: §29, §46, §56.3
- Decision: D-0184
- Invariants: I-020

One Decision for how the native dependency model interoperates with Linux shared libraries that expect a global `/usr/lib` namespace, required for unmodified glibc at V1. Q-022 is answered here.

<!-- covers: INV-0552 -->

#### Out of scope
Providing the view (LNX-051). Native Package store (PKG). FHS as native storage (rejected by I-020).

#### Acceptance criteria
- [ ] Options evaluated include a Personality-only `/usr/lib` view over Package contents, a copied FHS tree, and making FHS the native store, the last rejected against I-020.
- [ ] The accepted option states that native software does not search `/usr/lib` and marks Q-022 answered by this task.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: PKG lead and LNX lead sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- none

### LNX-025 · Apply the default Capability bundle to Linux apps
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: LNX-013, LNX-036, LNX-042, CAP-025
- Baseline: §9.1, §46, §47
- Threats: T-011, T-025
- Invariants: I-072

V1 daily-driving of browser and IDE without ambient home access. This implements the default-bundle Decision and portal upgrades so unmanifested Linux apps start with the decided set and grow it only through portals.

<!-- covers: GAP-0231 -->

#### Out of scope
The bundle Decision (LNX-013). Portal interfaces (LNX-036). Permissions UI (APP, SEC).

#### Acceptance criteria
- [ ] A Chromium-class and an IDE-class process start with exactly the default bundle named by the Decision, visible in `os inspect`.
- [ ] Opening a file through the file-chooser portal adds a File Capability and does not grant the containing directory.
- [ ] A process with the default bundle cannot read `$HOME` paths outside that bundle, including as uid 0.

#### Verification
- Integration: `personality:tests/bundle/default_and_portal_*` on `qemu-x86_64` and `hw-h002`.
- Compat: C-003 browser and IDE entries scored with the bundle in force on H-002 and H-004.

#### Evidence
- none

### LNX-026 · Publish Linux compatibility overhead on L2 workloads
- Type: benchmark
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: LNX-056, BEN-027, Q-001, LNX-030, OBS-038
- Baseline: §46, §54
- Benchmarks: B-026
- Corpora: C-003
- Invariants: I-061

B-026 V1 is publish-only: syscall latency and L2 non-graphics throughput versus upstream Linux on the same hardware. LNX owns the Personality workloads; BEN owns the register. No superiority claim without the table.

#### Out of scope
Register definition (BEN-027). L3 and later republish (LNX-063). Native isolation versus OCI (B-015).

#### Acceptance criteria
- [ ] A report exists under `reports/benchmarks/B-026/` for H-001, H-002 and H-004 meeting the V1 publish target.
- [ ] The report names the C-003 non-graphics scripts and the syscall microbenchmark set.
- [ ] The report states no superiority claim.

#### Verification
- Bench: B-026 on H-001, H-002 and H-004; target per register.
- Review: BEN lead confirms the method matches the register.

#### Evidence
- none

### LNX-027 · Capture Linux core dumps and Crashpad minidumps
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: LNX-049, OBS-029, OBS-026, LNX-042
- Baseline: §24, §46, §61
- Threats: T-023, T-027
- Invariants: I-077

V1 browser and IDE crash capture must work under the Personality. Core dumps and Crashpad minidumps map onto the OBS capture format with Component identity, gated by the debug Capability rather than same-uid attach. Required by V3-G04 (Crash reports are symbolicated and opted in).

#### Out of scope
Capture format Decision (OBS-029). Native Component crashes (OBS-026). Crash-report client (INS-020). ptrace equivalent (LNX-049).

#### Acceptance criteria
- [ ] A crashing Chromium-class and IDE-class process produce an OBS crash record that names the enclosing Component.
- [ ] Crashpad minidump capture succeeds when the debug Capability is held and fails with `Error::Rights` when it is not.
- [ ] The record contains no disk keys or unlocked secrets.

#### Verification
- Integration: `personality:tests/crash/core_and_crashpad_*` on `qemu-x86_64` and `hw-h002`.
- Review: OBS lead confirms the payload matches the capture format.

#### Evidence
- none

### LNX-028 · Define freedesktop, Mesa and systemd engagement policy
- Type: docs
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: LNX-011, LNX-023
- Baseline: §2, §56.3, §57
- Invariants: I-009, I-096

Linux compatibility is a product whose Wayland, PipeWire, portals, Mesa and systemd components must not be forked casually (§2, §56.3). This policy names the upstreams, the patch-first rule, and when a fork requires an accepted Decision.

<!-- covers: GAP-0053 -->

#### Out of scope
Kernel upstream-first policy (KRN-006). Wine upstream policy (WIN-007). Userland origin (LNX-023).

#### Acceptance criteria
- [ ] The policy names Wayland, PipeWire, xdg-desktop-portal, Mesa and systemd as upstreams with a patch-first rule.
- [ ] The policy states the condition under which a long-lived fork is allowed, requiring an accepted Decision.
- [ ] CI documents where Personality patches against those trees live.

#### Verification
- Review: LNX lead and GOV licensing reviewer sign-off recorded on the pull request.

#### Evidence
- none

### LNX-029 · Satisfy systemd assumptions for Linux desktop apps
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: LNX-059, SVC-026, LNX-043, LNX-046, LNX-057
- Baseline: §32, §56.3
- Risks: R-072

sd-bus, socket activation, logind sessions and seats, user units, journald and `XDG_RUNTIME_DIR` are satisfied or emulated for applications that depend on them, at the level the glibc/systemd spike recorded. Native supervision stays in SVC; this is the Personality surface (R-072).

<!-- covers: INV-1083 -->

#### Out of scope
Native supervisor (SVC-015). Personality-as-supervised-daemon adapter (SVC-026). Native journal (SVC-031).

#### Acceptance criteria
- [ ] A C-003 desktop entry that requires sd-bus, `XDG_RUNTIME_DIR` and a logind session starts and reaches its scripted main window.
- [ ] Socket activation of a Personality user unit delivers the listening fd the unit declares.
- [ ] Native services are not started by systemd, and `os inspect service` shows them under the native supervisor.

#### Verification
- Integration: `personality:tests/systemd/sdbus_logind_userunit_*` on `qemu-x86_64` and `hw-h002`.
- Compat: C-003 entries tagged systemd-dependent on H-002 and H-004.
- Review: SVC lead confirms the adapter does not replace native supervision.

#### Evidence
- none

### LNX-030 · Make Linux a Personality rather than the default model
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: LNX-003, LNX-016, LNX-005, ABI-025, ABI-035, LNX-001
- Baseline: §3, §6, §46
- Invariants: I-025, I-049

Phase B: Linux becomes a compatibility Personality with its own entry path rather than the default execution model (§6, §46). Native is default. The long-term layer between ELF binaries and native primitives starts here; translation itself is V2.

<!-- covers: INV-0144, INV-0850, INV-1330 -->

#### Out of scope
Syscall translation (LNX-090). Native syscall filter implementation (ABI-035). Dual-world tag (ABI-025).

#### Acceptance criteria
- [ ] A newly created native Component has the Native ABI world tag and fails Linux syscall numbers with the filter in ABI-035.
- [ ] A Linux ELF starts only through the Personality entry path and is inspectable as a Personality Component.
- [ ] The default boot session's user applications are native unless they hold the opt-in grant from LNX-016.

#### Verification
- Integration: `personality:tests/phase_b/entry_world_*` on `qemu-x86_64` and `hw-h002`.
- Unit: world-tag and filter tests on `qemu-x86_64`.
- Review: ABI lead confirms native Components cannot invoke Linux syscalls.

#### Evidence
- none

### LNX-031 · GPU-accelerate Linux apps via Mesa EGL, Vulkan and dma-buf
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: LNX-014, LNX-044, LNX-006, LNX-040, MEM-025, GFX-020
- Baseline: §39, §47, §56.1, §61
- Risks: R-016, R-025
- Invariants: I-045

V1 exit: GPU-accelerated Linux browser WebGL plus OpenGL and Vulkan samples with dma-buf import into the native compositor (§47, §61). Mesa stays inside the Personality; native applications still do not see DRM.

<!-- covers: INV-0882 -->

#### Out of scope
Brokered RenderQueue (GFX-057). Native GPU driver stack (forbidden by I-045). dma-buf MemoryObject import (MEM-025).

#### Acceptance criteria
- [ ] A Chromium-class WebGL page and an OpenGL sample and a Vulkan sample present through dma-buf import on H-002 and H-004.
- [ ] Native compositor and toolkit crates do not open DRM nodes or issue DRM ioctls.
- [ ] `os inspect` shows the Personality GPU holders and the imported MemoryObjects.

#### Verification
- Integration: `personality:tests/gpu/egl_vulkan_dmabuf_*` on `hw-h002` and `hw-h004`.
- Demo: V1 GPU-accelerated Linux browser on H-002.
- Review: GFX lead confirms DRM nodes stay inside the Personality.

#### Evidence
- none

### LNX-032 · Host development runtimes inside the Personality
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: LNX-030, LNX-045, LNX-057, LNX-039
- Baseline: §35, §36, §46, §61

V1 self-hosting and `os env`: compilers, build systems, CLI tooling and Linux-personality services such as postgres run inside native isolation (§35, §61). ENV composes the environment; LNX hosts the Linux-class runtimes.

<!-- covers: INV-0869, INV-1203 -->

#### Out of scope
environment.yaml and enter path (ENV). Native Packages for runtimes (ENV-019). OCI engine (LNX-039).

#### Acceptance criteria
- [ ] The php-postgres-redis reference environment reaches postgres through a Personality-hosted server with no ambient network Capability on the developer Component.
- [ ] `gcc` or `clang`, `make` and `git` from C-003 run inside a Personality Component under `os env`.
- [ ] Native crates in that environment do not link the Personality.

#### Verification
- Integration: `personality:tests/env/runtime_host_*` on `qemu-x86_64` and `hw-h002`.
- Demo: V1 `os env enter` with postgres reachable on H-002.

#### Evidence
- none

### LNX-033 · Host PipeWire clients inside the Linux Personality
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: AUD-001, AUD-002, LNX-043, LNX-005
- Baseline: §47, §56.3, §61
- Invariants: I-096

V1 audio for Linux apps. AUD owns the PipeWire/PulseAudio server and native stream objects; LNX hosts the client socket inside the Personality so GUI apps play without seeing ALSA as a native API.

<!-- covers: INV-1090 -->

#### Out of scope
PipeWire server and AudioStream (AUD-001). Simultaneous native-plus-PipeWire verification (AUD-014). Bluetooth codecs (AUD, HW).

#### Acceptance criteria
- [ ] A C-003 GUI entry plays through the Personality PipeWire socket and is visible as an AudioStream holder in `os inspect`.
- [ ] A Personality process without an audio Capability cannot connect to the PipeWire socket.
- [ ] Native crates do not link libpipewire.

#### Verification
- Integration: `personality:tests/audio/pipewire_client_*` on `hw-h002` and `hw-h004`.
- Compat: C-003 audio integration scoring on H-002.

#### Evidence
- none

### LNX-034 · Host Wine userspace on the Linux Personality
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: LNX-030, LNX-057, LNX-045, LNX-015, WIN-008
- Baseline: §3, §48, §56.2
- Risks: R-031
- Invariants: I-007

V1 non-gated Wine-on-LNX bring-up from the ladder (R-031). WIN owns the Wine test suite, W1 corpus and hosting Decision; this task is the Linux-personality host a PE binary starts under. Native software sees no Win32.

<!-- covers: EXTRA-069 -->

#### Out of scope
Wine test suite CI (WIN-017). Wine-on-LNX bring-up owned by WIN (WIN-015). Win32 surface (WIN-053).

#### Acceptance criteria
- [ ] A PE binary starts under Wine hosted by the Personality on H-001 and exits with a recorded status, with no V1 pass-rate gate.
- [ ] The Wine process is a Personality Component inspectable in `os inspect`.
- [ ] Native crates do not link Wine or include Win32 headers.

#### Verification
- Integration: `personality:tests/wine/host_pe_start_*` on `qemu-x86_64`.
- Review: WIN lead confirms this host is what WIN-015 consumes.

#### Evidence
- none

### LNX-035 · Ship ia32 userland if emulation is retained
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: LNX-015, LNX-057, LNX-051, LNX-023
- Baseline: §46, §56.3
- Risks: R-032

Implements the ia32 Decision before syscall pruning so 32-bit Steam and Wine titles remain possible at V1 and V2. H-016 holds the configuration in CI. If the Decision rejects ia32, a follow-up drops this task. Required by V2-G18 (L3 corpus meets its threshold): C-010 titles are scored inside that corpus and need the 32-bit path.

#### Out of scope
The ia32 Decision (LNX-015). 32-bit multilib for Steam (LNX-086). Win32 WoW64 (WIN-055).

#### Acceptance criteria
- [ ] An ia32 ELF interpreter and glibc matching the accepted Decision are present in the Personality view on H-016.
- [ ] Nightly CI on H-016 boots an ia32 binary to a zero exit status.
- [ ] Kernel config fragments keep ia32 enabled while the Decision retains it (R-032).

#### Verification
- Integration: `personality:tests/ia32/userland_*` on `qemu-ia32` (H-016).
- Review: KRN lead confirms the config fragment matches the Decision.

#### Evidence
- none

### LNX-036 · Implement xdg-desktop-portal onto native choosers
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: LNX-018, LNX-043, STO-034, APP-002, APP-014, UIP-003
- Baseline: §9.1, §25, §47
- Threats: T-002
- Invariants: I-035

V1 L2 integration check: file chooser, notifications, screenshot and settings portals backed by native Capability choosers (§25, §47). The Personality is a deputy and must not use its own authority (T-002).

<!-- covers: INV-0874, INV-0883, INV-0888, INV-1091 -->

#### Out of scope
Portal strategy Decision (LNX-018). Chooser UI (APP-002). ScreenCast portal (LNX-081). Camera portal (LNX-080). Settings portal polish (LNX-077).

#### Acceptance criteria
- [ ] A C-003 app that opens the file-chooser portal receives a UserSelected File Capability and cannot enumerate the containing directory.
- [ ] Notification and screenshot portal calls reach the native Notify service and screen-capture Capability, and a denial returns the portal error without crashing the app.
- [ ] A portal request served using the host's authority rather than the caller's grant fails the confused-deputy test.

#### Verification
- Integration: `personality:tests/portals/chooser_notify_screenshot_*` on `qemu-virtio-gpu` and `hw-h002`.
- Compat: C-003 file-chooser and notification integration scoring on H-002 and H-004.
- Review: STO lead confirms UserSelected is the returned object.

#### Evidence
- none

### LNX-037 · Integrate LTP, glibc tests and syzkaller into nightly CI
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: LNX-057, LNX-002, BLD-017, BLD-016, GOV-031
- Baseline: §56.3
- Corpora: C-001
- Invariants: I-096

Nightly LTP, glibc tests, differential syzkaller versus upstream Linux, and the application smoke corpus, so compatibility is a product with a regression suite that measures behavioural divergence from real Linux.

<!-- covers: GAP-0116, INV-0865 -->

#### Out of scope
Guest-agent plumbing (BLD-017). Native ABI syzkaller (BLD-016). L2 scenario content (LNX-056).

#### Acceptance criteria
- [ ] Nightly CI runs LTP, the glibc test subset, and differential syzkaller against upstream Linux of the same base version on H-001.
- [ ] A divergence from upstream is recorded as a C-ID artefact, not as a silent pass.
- [ ] The application smoke corpus from C-003 CLI entries runs on the same nightly job.

#### Verification
- Integration: `personality:tests/conformance/ltp_glibc_syzkaller_*` on `qemu-x86_64`.
- Compat: C-001 retained on the nightly job on H-001.
- Review: BLD lead confirms job plumbing matches other corpus jobs.

#### Evidence
- none

### LNX-038 · Integrate Linux GUI apps into the native desktop
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: LNX-040, LNX-036, LNX-020, APP-013, APP-014, UIP-029, UIP-028
- Baseline: §47, §49, §61
- Risks: R-025
- Invariants: I-096

V1 L2 integration check: launcher, task switcher, clipboard both ways, drag/drop, notifications, fractional and per-display scaling so users do not perceive Linux GUI apps as VM applications (§47, §49).

<!-- covers: INV-0877, INV-0879, INV-0880, INV-0884, INV-0885, INV-0886 -->

#### Out of scope
Protocol hooks (UIP-029). Launcher chrome (APP-013). V2 polish on three machines (LNX-079). Primary-selection policy (LNX-020).

#### Acceptance criteria
- [ ] A C-003 GTK, Qt and Electron entry appear in the native launcher and task switcher with no separate desktop.
- [ ] Clipboard and drag/drop succeed in both directions between a native Text Editor and a C-003 GUI entry; primary selection does not enter the native clipboard.
- [ ] Notifications from that entry appear in the native Notify service.
- [ ] Fractional and per-display scale of the native compositor apply to those windows on H-002 and H-004.

#### Verification
- Integration: `personality:tests/desktop/launcher_clipboard_dnd_notify_scale_*` on `hw-h002` and `hw-h004`.
- Compat: C-003 integration scoring on H-002 and H-004.
- Demo: V1 Linux browser and IDE on the native desktop on H-002.

#### Evidence
- none

### LNX-039 · Integrate an OCI runtime inside the Personality
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: LNX-012, LNX-045, LNX-061, PKG-047
- Baseline: §36, §56.3
- Invariants: I-019

Pulled to V1 because the L2 corpus requires an OCI container runtime. OCI never becomes a native platform feature. The runtime uses Personality namespaces, cgroups and overlayfs.

<!-- covers: INV-0670, INV-0671, INV-0674 -->

#### Out of scope
Engine surface Decision (LNX-012). Native `os env` (ENV). VM manager (VIRT). Sandbox primitive verification (LNX-061).

#### Acceptance criteria
- [ ] The engine named by LNX-012 runs a minimal OCI image to a zero exit inside the Personality on H-001 and H-002.
- [ ] Native crates and services have no OCI runtime dependency (I-019).
- [ ] The container's net, pid and mount namespaces are Personality namespaces, not a native platform API.

#### Verification
- Integration: `personality:tests/oci/runtime_*` on `qemu-x86_64` and `hw-h002`.
- Compat: C-003 container-runtime entry on H-001 and H-002.

#### Evidence
- none

### LNX-040 · Host Wayland clients through the Personality bridge in the native compositor
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: LNX-006, LNX-053, LNX-004, UIP-029
- Baseline: §47, §56.3, §61
- Risks: R-020
- Invariants: I-048

V1 polish beyond the V0.5 existing-mechanism bridge so Linux GUI apps are compositor-native for daily-driving (§56.3, §61). Wayland remains a Personality protocol, not the native UI API.

<!-- covers: INV-1088 -->

#### Out of scope
V0.5 bridge (LNX-006). Xwayland server (LNX-053). Native UI protocol (UIP).

#### Acceptance criteria
- [ ] C-003 Wayland entries use the V1 hosting path named by LNX-004 and pass window, input and clipboard scoring on H-002 and H-004.
- [ ] Native compositor crates still do not link libwayland-client.
- [ ] Compositor kill-rebind on `qemu-virtio-gpu` keeps the Linux client alive.

#### Verification
- Integration: `personality:tests/wayland/v1_hosting_*` on `qemu-virtio-gpu`, `hw-h002` and `hw-h004`.
- Compat: C-003 GUI window scoring on H-002 and H-004.

#### Evidence
- none

### LNX-041 · Keep L0 and L1 Corpus results green
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: LNX-002, LNX-007, LNX-030
- Baseline: §46, §61
- Corpora: C-001, C-002
- Risks: R-004
- Invariants: I-096

V1 compatibility gate: L0 and L1 pass with zero regressions versus V0.5 while L2 is added. Phase B must not silently regress the retained syscall path (R-004).

#### Out of scope
L2 run (LNX-056). Nightly LTP plumbing (LNX-037).

#### Acceptance criteria
- [ ] C-001 meets its V1 zero-regression threshold versus V0.5 on H-001 and H-002.
- [ ] C-002 meets its V1 zero-regression threshold versus V0.5 on H-001, H-002 and H-003.
- [ ] A Phase B patch that drops a previously passing C-001 case fails CI.

#### Verification
- Compat: C-001 and C-002 on H-001, H-002 and H-003.
- Review: KRN lead signs the zero-regression comparison.

#### Evidence
- none

### LNX-042 · Map Linux processes onto Components and ResourceDomains
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: LNX-060, CMP-036, TSK-043, LNX-030, SCH-007, OBS-006
- Baseline: §10, §23, §46, §61
- Invariants: I-014, I-033

Linux software must be accounted, inspectable and cancellable like native software for V1 daily-driving and `os inspect` (§46). The mapping follows CMP-036 and TSK-043.

<!-- covers: INV-0864 -->

#### Out of scope
Mapping Decision (CMP-036). Thread mapping Decision (TSK-043). Personality tracing (OBS-038). fd-to-Capability map (LNX-078).

#### Acceptance criteria
- [ ] Every Linux process started through the Personality is a member of a Component and a ResourceDomain visible in `os inspect`.
- [ ] Cancelling that Component terminates the process tree the Decision assigned to it.
- [ ] CPU and memory accounting for a C-003 IDE entry appear on the ResourceDomain, not as an unattributed kernel task.

#### Verification
- Integration: `personality:tests/map/component_domain_*` on `qemu-x86_64` and `hw-h002`.
- Review: CMP lead and SCH lead confirm the mapping matches their Decisions.

#### Evidence
- none

### LNX-043 · Provide D-Bus session and system buses
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: LNX-030, LNX-052, SVC-027, LNX-005
- Baseline: §47, §56.3
- Risks: R-025

V1 Linux personality as a product: session and system buses with bridges to native services for L2 browsers and IDEs (§47, §56.3). Buses stay inside the Personality.

<!-- covers: INV-0873, INV-1084 -->

#### Out of scope
Portal implementations (LNX-036). Native Notify (APP-014). AT-SPI bus (LNX-096).

#### Acceptance criteria
- [ ] A session bus and a system bus are available to a C-003 desktop process at the addresses glibc and sd-bus expect.
- [ ] A process whose Component lacks the D-Bus grant cannot connect to either bus.
- [ ] Native crates do not link libdbus.

#### Verification
- Integration: `personality:tests/dbus/session_system_*` on `qemu-x86_64` and `hw-h002`.
- Compat: C-003 browser and IDE D-Bus-dependent scenarios on H-002.

#### Evidence
- none

### LNX-044 · Provide udev and /dev nodes for Linux-Personality devices
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: LNX-014, LNX-050, HW-019, HW-011
- Baseline: §33, §46, §56.3
- Risks: R-016

V1 GPU and input for browser and IDE; precursor to Steam `/dev/input` via udev in the V2 sub-corpus. Nodes exist only inside the Personality; native software holds Object<Device>.

<!-- covers: INV-0625 -->

#### Out of scope
Coexistence Decision (LNX-014). Native Device objects (HW). Steam evdev (LNX-074).

#### Acceptance criteria
- [ ] Mesa, udev and a C-003 browser enumerate GPU and input through Personality `/dev` and udev events on H-002 and H-004.
- [ ] A native Component in the same boot has no `/dev` namespace and cannot open those nodes.
- [ ] udev events for a hot-plugged HID device appear to Personality clients and mint a native InputDevice Capability for native clients.

#### Verification
- Integration: `personality:tests/dev/udev_nodes_*` on `hw-h002` and `hw-h004`.
- Review: HW lead confirms native software still does not open DRM or hidraw nodes.

#### Evidence
- none

### LNX-045 · Provide Linux namespaces inside the Personality
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: LNX-030, LNX-061, LNX-042, NET-016
- Baseline: §36, §46, §56.3
- Invariants: I-019

pid, mount, net, user, uts, ipc and cgroup namespaces for containers and sandboxes, required for the L2 container-runtime entry (§56.3). Native software uses ResourceDomain and Capabilities, not namespaces.

<!-- covers: INV-1087 -->

#### Out of scope
Sandbox primitive verification (LNX-061). Native namespace subsumption (CMP-026). OCI runtime (LNX-039). pidns over Components (LNX-098).

#### Acceptance criteria
- [ ] A Personality process can unshare pid, mount, net, user, uts, ipc and cgroup namespaces and the result is visible in `os inspect`.
- [ ] Native Components have no namespace configuration step and do not write to cgroupfs.
- [ ] User-namespace and overlayfs combinations required by LNX-061 succeed.

#### Verification
- Integration: `personality:tests/ns/unshare_*` on `qemu-x86_64` and `hw-h002`.
- Unit: each namespace kind on `qemu-x86_64`.

#### Evidence
- none

### LNX-046 · Provide PAM, NSS and logind from the native Session
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: SEC-020, SEC-028, LNX-043, SVC-027
- Baseline: §9, §56.3, §61

getpwuid, `$HOME`, `XDG_RUNTIME_DIR` and session lock/idle signals for V1 desktop apps, fed from native identity and Session so Linux apps do not read a shadow file.

<!-- covers: GAP-0225 -->

#### Out of scope
Native identity (SEC-020). Session object (SEC-028). systemd user units (LNX-029).

#### Acceptance criteria
- [ ] `getpwuid` and `$HOME` for the logged-in user match the native identity service, with no readable `/etc/shadow` from the app.
- [ ] `XDG_RUNTIME_DIR` exists per session and is torn down with the Session.
- [ ] logind lock and idle signals on the session bus track SEC-028.

#### Verification
- Integration: `personality:tests/session/pam_nss_logind_*` on `qemu-x86_64` and `hw-h002`.
- Review: SEC lead confirms no ambient shadow or machine-id leak (I-078).

#### Evidence
- none

### LNX-047 · Present native storage as POSIX home and XDG paths
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: LNX-019, STO-047, LNX-052, LNX-025
- Baseline: §25, §46, §47
- Invariants: I-016

V1 L2 corpus home and XDG paths for Git, IDE and browser. The POSIX path namespace starts here and is completed at V2 translation. Paths are a Personality view, not native authority.

<!-- covers: INV-0493, INV-1092 -->

#### Out of scope
Storage view API (STO-047). Path-view Decision (LNX-019). Mount synthesis (LNX-088).

#### Acceptance criteria
- [ ] A C-003 Git and IDE entry resolve `$HOME` and XDG base dirs to Capability-scoped native objects.
- [ ] A path outside the Component grant returns `EACCES` at the Personality boundary and allocates no File handle.
- [ ] Native Components still have no filesystem namespace.

#### Verification
- Integration: `personality:tests/paths/home_xdg_*` on `qemu-x86_64` and `hw-h002`.
- Compat: C-003 Git and IDE file-access scenarios on H-002.

#### Evidence
- none

### LNX-048 · Provide procfs semantics inside the Personality
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: LNX-042, LNX-005, LNX-030
- Baseline: §46, §56.3
- Threats: T-011

§56.3 procfs inside the Personality for L2 apps. V2 later synthesises `/proc` over native inspectables. A process cannot read another Component's `/proc` entries beyond its grant (T-011).

<!-- covers: INV-1085 -->

#### Out of scope
`/proc` synthesis over inspectables (LNX-089). Native `os inspect` (OBS).

#### Acceptance criteria
- [ ] `/proc/self` for a C-003 process reports pid, maps and fds that match the Personality process model.
- [ ] Reading `/proc` of a process in another Component returns `EACCES` unless a debug Capability is held.
- [ ] Native Components have no `/proc` namespace.

#### Verification
- Integration: `personality:tests/proc/self_and_isolation_*` on `qemu-x86_64` and `hw-h002`.
- Compat: C-003 entries that read `/proc/self` on H-001.

#### Evidence
- none

### LNX-049 · Provide a ptrace equivalent gated by a debug Capability
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: CAP-032, SDK-038, LNX-042, SEC-002
- Baseline: §9.1, §46, §61
- Threats: T-027
- Invariants: I-021

gdb, strace, Wine debugging and Crashpad at V1 are gated by an explicit debug Capability rather than same-uid attach (T-027).

<!-- covers: EXTRA-009 -->

#### Out of scope
Debug-attach rights type (CAP-032). Native debugger UI (SDK-038). Crash capture (LNX-027).

#### Acceptance criteria
- [ ] `ptrace` attach and `gdb` attach succeed only when the tracer's Component holds the debug Capability to the tracee.
- [ ] Same-uid attach without that Capability returns `EPERM` and allocates no handle.
- [ ] strace of a C-003 process with the Capability records syscalls attributed to the enclosing Component.

#### Verification
- Integration: `personality:tests/debug/ptrace_cap_*` on `qemu-x86_64` and `hw-h002`.
- Review: CAP lead confirms attach is the debug Capability, not uid.

#### Evidence
- none

### LNX-050 · Provide sysfs semantics inside the Personality
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: LNX-014, LNX-030, HW-009
- Baseline: §33, §56.3

§56.3 sysfs inside the Personality for device enumeration used by Mesa, udev and desktop apps at V1. Native software enumerates Object<Device>, not `/sys`.

<!-- covers: INV-1086 -->

#### Out of scope
Coexistence Decision (LNX-014). Native Device enumeration (HW-009).

#### Acceptance criteria
- [ ] Mesa and udev enumerate the V1 GPU and input devices through Personality `/sys` on H-002 and H-004.
- [ ] A native Component has no `/sys` namespace.
- [ ] Writes to `/sys` that would change native Device state without a Device Capability fail at the Personality boundary.

#### Verification
- Integration: `personality:tests/sys/enumerate_*` on `hw-h002` and `hw-h004`.
- Review: HW lead confirms native Device enumeration does not parse `/sys`.

#### Evidence
- none

### LNX-051 · Provide a glibc /usr/lib view over Package contents
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: LNX-024, LNX-057, PKG-047
- Baseline: §29, §56.3
- Invariants: I-020

V1 unmodified glibc and compiler toolchain in the L2 corpus need a global `/usr/lib` view without making FHS the native storage model.

<!-- covers: INV-0552 -->

#### Out of scope
Namespace Decision (LNX-024). Native Package store (PKG).

#### Acceptance criteria
- [ ] Unmodified glibc and a C-003 compiler toolchain resolve their DSOs through the Personality `/usr/lib` view.
- [ ] The view is constructed from Package contents; installing a native Package does not copy into a shared FHS (I-020).
- [ ] Native Components do not search `/usr/lib`.

#### Verification
- Integration: `personality:tests/usrlib/glibc_toolchain_*` on `qemu-x86_64` and `hw-h002`.
- Compat: C-003 compiler and Git entries on H-002.

#### Evidence
- none

### LNX-052 · Provide XDG and FHS conventions in the Personality
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: LNX-019, LNX-023, STO-047
- Baseline: §47, §56.3
- Invariants: I-016

XDG base dirs, `.desktop` entries, icon themes, MIME database, `/tmp`, `/home` and `/etc` as a Personality view, not the native storage model (§47, §56.3).

<!-- covers: INV-0875, INV-1092 -->

#### Out of scope
Home path Decision (LNX-019). Launcher import of `.desktop` (APP-013). Native settings store (SVC).

#### Acceptance criteria
- [ ] A C-003 desktop entry is found via XDG data dirs and its icon and MIME type resolve in the Personality view.
- [ ] `/tmp`, `/home` and `/etc` exist in the view and writes outside the Component grant fail.
- [ ] Native storage objects are not FHS paths.

#### Verification
- Integration: `personality:tests/xdg/desktop_mime_fhs_*` on `qemu-x86_64` and `hw-h002`.
- Compat: C-003 launcher-visible entries on H-002.

#### Evidence
- none

### LNX-053 · Provide X11 via Xwayland inside the Personality
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: LNX-004, LNX-006, LNX-020
- Baseline: §41, §47, §56.3, §57
- Invariants: I-048

V1 L2 corpus includes an XWayland-only application. X11 never becomes the native UI API (§47, §57).

<!-- covers: INV-0871, INV-1089 -->

#### Out of scope
Hosting Decision (LNX-004). Native UI protocol (UIP). Primary-selection policy (LNX-020).

#### Acceptance criteria
- [ ] The C-003 XWayland-only entry presents as a compositor window with input on H-002.
- [ ] Native compositor and toolkit crates do not link X11 libraries.
- [ ] Primary selection of that client stays inside the bridge and does not enter the native clipboard.

#### Verification
- Integration: `personality:tests/xwayland/x11_app_*` on `qemu-virtio-gpu` and `hw-h002`.
- Compat: C-003 XWayland entry on H-002.

#### Evidence
- none

### LNX-054 · Publish a five-minute Linux application launch guide
- Type: docs
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: LNX-011, LNX-038, LNX-017
- Baseline: §49, §56.3, §61

Recommended first developer experience: run an existing Linux application via the Personality. Showing existing software working is the fastest credibility win before anyone writes native code.

<!-- covers: GAP-0459 -->

#### Out of scope
Docs site pipeline (DOC). Full compatibility guide (LNX-099). Packaging Decision (LNX-017).

#### Acceptance criteria
- [ ] A committed guide walks from a booted V1 image to a running C-003 GUI application in five minutes of operator steps.
- [ ] The guide names the first-class packaging path from LNX-017.
- [ ] The guide states that native software is not required to use the Personality.

#### Verification
- Review: DOC lead and LNX lead sign-off recorded on the pull request.
- Manual: a reviewer follows the guide on H-002 and reaches the scripted window.

#### Evidence
- none

### LNX-055 · Retain inotify for Linux IDE file watching
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: LNX-047, STO-035, LNX-005
- Baseline: §25, §46, §61

File-watch/inotify is pulled to V1 for VS Code. STO owns the native change-notification Operation and its bridge; LNX retains inotify inside the Personality and wires it to that Operation for granted paths only.

#### Out of scope
Native change-notification Operation (STO-035). IDE chrome (APP). Fanotify extras beyond the L2 IDE.

#### Acceptance criteria
- [ ] A C-003 IDE entry receives inotify events for files inside its grant set and not for files outside it.
- [ ] inotify watches on a path without a File or Directory Capability fail at the Personality boundary.
- [ ] Native Components use the change-notification Operation, not inotify.

#### Verification
- Integration: `personality:tests/inotify/granted_paths_*` on `qemu-x86_64` and `hw-h002`.
- Compat: C-003 IDE file-watch scenario on H-002.

#### Evidence
- none

### LNX-056 · Define and run the L2 Corpus
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: LNX-041, LNX-038, LNX-039, LNX-058, LNX-031, LNX-061, LNX-053, GOV-031, BLD-017, LNX-029, LNX-046, LNX-048, TXT-019
- Baseline: §47, §49, §56.3, §61
- Corpora: C-003
- Risks: R-025
- Invariants: I-096

V1 compatibility gate: C-003 including Chromium, Firefox, IDE, GIMP-class, LibreOffice-class, Qt, GTK, Electron, XWayland, OCI and Flatpak. Thresholds, including the required-pass entries, live in the register.

<!-- covers: INV-1093, INV-0892, INV-1103, INV-1105, INV-1198, INV-1199, INV-0865 -->

#### Out of scope
L3 corpus (LNX-084). Legal redistribution review (GOV-031). Integration implementation (LNX-038).

#### Acceptance criteria
- [ ] C-003 scenario scripts exist under `compat:linux-L2` and cover every class named in the register.
- [ ] The committed report on H-001, H-002 and H-004 meets the C-003 V1 threshold, including the register's required-pass entries for browser, IDE, container runtime and Git.
- [ ] Integration scoring covers launcher, task switcher, clipboard, file chooser via portal, notifications and scaling.

#### Verification
- Compat: C-003 scenario `compat:linux-L2` on H-001, H-002 and H-004.
- Integration: `personality:tests/corpus/l2_*` on `qemu-x86_64` and `hw-h002`.
- Review: GOV lead confirms redistributable entries match the legal review.

#### Evidence
- none

### LNX-057 · Run unmodified glibc and musl in the Personality
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: LNX-030, LNX-023, LNX-001
- Baseline: §46, §56.3

§56.3: glibc and musl must run unmodified; vDSO and vsyscall are retained. V1 L2 browsers and toolchains depend on this.

<!-- covers: INV-1082 -->

#### Out of scope
Userland origin Decision (LNX-023). `/usr/lib` view (LNX-051). Syscall translation (LNX-090).

#### Acceptance criteria
- [ ] Unmodified glibc and musl dynamic linkers from the userland Decision start a hello binary to a zero exit on H-001 and H-002.
- [ ] vDSO and vsyscall remain available to those processes.
- [ ] No glibc or musl patch lives in the Personality tree without an accepted Decision.

#### Verification
- Integration: `personality:tests/libc/glibc_musl_unmodified_*` on `qemu-x86_64` and `hw-h002`.
- Unit: vDSO and vsyscall presence on `qemu-x86_64`.
- Review: LNX lead confirms the binaries match upstream checksums recorded by the userland Decision.

#### Evidence
- none

### LNX-058 · Ship Flatpak runtimes inside the Linux Personality
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: LNX-017, LNX-061, LNX-036, LNX-045, PKG-047
- Baseline: §36, §47, §56.3
- Invariants: I-019

V1 L2 corpus includes Flatpak. bubblewrap, user namespaces and overlayfs come from the sandbox-primitives task. Flatpak is a Personality package source, not a native store.

<!-- covers: GAP-0424, INV-0891 -->

#### Out of scope
Packaging format Decision (LNX-017). Immutable packaging mechanics (PKG). Flatpak/Snap evaluation beyond V1 (LNX-087).

#### Acceptance criteria
- [ ] The C-003 Flatpak entry installs and runs inside the Personality on H-002.
- [ ] The sandbox uses Personality user namespaces and overlayfs verified by LNX-061.
- [ ] Native Package install paths do not invoke Flatpak.

#### Verification
- Integration: `personality:tests/flatpak/runtime_*` on `qemu-x86_64` and `hw-h002`.
- Compat: C-003 Flatpak entry on H-002.

#### Evidence
- none

### LNX-059 · Survey glibc and systemd needs of top desktop apps
- Type: spike
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: LNX-008, LNX-057
- Baseline: §56.3

Evidence for real systemd versus a compatible service manager versus none, from the top desktop applications. Precedes systemd emulation so scope is set from evidence (§56.3).

<!-- covers: GAP-0506 -->

#### Out of scope
systemd surface implementation (LNX-029). Native supervisor (SVC).

#### Acceptance criteria
- [ ] The report surveys the top desktop applications named in the spike and records actual glibc and systemd dependencies (sd-bus, socket activation, logind, user units, journald, `XDG_RUNTIME_DIR`).
- [ ] The report recommends one of: real systemd, a compatible service manager, or none, with the application count that would break under each.
- [ ] The report is committed at `reports/spikes/LNX-059.md`.

#### Verification
- Report: which systemd APIs the surveyed set actually calls; whether a compatible manager covers C-003; what remains a V2 gap.
- Review: SVC lead records that the recommendation does not replace native supervision.

#### Evidence
- none

### LNX-060 · Prototype Linux process to Component mapping
- Type: spike
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: LNX-030, CMP-005, SCH-007
- Baseline: §10, §23, §46
- Explores: S-030

Prototypes mapping a Linux process to a native Component so CMP-036 can choose first-class Components versus a Personality-managed construct, and so ResourceDomain, tracing and permissions apply uniformly.

<!-- covers: GAP-0503 -->

#### Out of scope
Mapping Decision (CMP-036). Mapping implementation (LNX-042). Thread mapping (TSK-043).

#### Acceptance criteria
- [ ] The report prototypes one-Component-per-process, one-Component-per-process-tree, and a Personality ResourceDomain hosting plain tasks, with inspect and accounting results for each.
- [ ] The report states how PID and exit status surface through Object<Component>.
- [ ] The report is committed at `reports/spikes/LNX-060.md`.

#### Verification
- Report: which mapping keeps ResourceDomain, tracing and permissions uniform; cost of each option on C-003 IDE process counts; what S-030 must expose.
- Review: CMP lead confirms the report is the input to CMP-036.

#### Evidence
- none

### LNX-061 · Verify seccomp, user namespaces, overlayfs and FUSE
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: LNX-001, KRN-011, KRN-017, LNX-030, LNX-021
- Baseline: §46, §56.3
- Risks: R-025, R-032

Chromium sandbox, Flatpak/bubblewrap and podman at V1 require these retained mechanisms verified from the Personality, not merely left enabled in a config fragment.

<!-- covers: EXTRA-008 -->

#### Out of scope
Attack-surface pruning that must keep these symbols (KRN-044). Namespace product surface (LNX-045). OCI runtime (LNX-039).

#### Acceptance criteria
- [ ] Chromium-class sandbox, bubblewrap and a podman/crun run each exercise seccomp-bpf, user namespaces, overlayfs and FUSE from the Personality on H-001 and H-002.
- [ ] CI fails if a kernel fragment disables a symbol these runs require.
- [ ] Native Components cannot open `/dev/fuse` or load seccomp filters as a native API.

#### Verification
- Integration: `personality:tests/sandbox/seccomp_userns_overlay_fuse_*` on `qemu-x86_64` and `hw-h002`.
- Review: KRN lead confirms the required symbols are in the retained-mechanism inventory.

#### Evidence
- none

### LNX-062 · Decide incremental native-Interface adoption
- Type: adr
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: LNX-016, LNX-036, LNX-018
- Baseline: §3, §42, §46
- Decision: D-0175
- Invariants: I-025

One Decision on whether a Linux-personality app may adopt native file chooser, semantic interfaces and Capabilities without eroding the §3 firewall. An all-or-nothing rewrite requirement means no existing application ever becomes native.

<!-- covers: GAP-0456 -->

#### Out of scope
Bridge implementation (LNX-072). Semantic registry (SEM). Native opt-in of native apps (LNX-016).

#### Acceptance criteria
- [ ] Options evaluated include an explicit incremental bridge, all-or-nothing rewrite, and silent mixing of POSIX and native APIs in one Component, the last rejected against I-025.
- [ ] The accepted option lists which native interfaces a Linux app may call and the limits that keep the §3 firewall.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: ABI lead and LNX lead sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- none

### LNX-063 · Publish Linux compatibility overhead on L3 workloads
- Type: benchmark
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: LNX-026, LNX-084, Q-001
- Baseline: §54, §62
- Benchmarks: B-026
- Corpora: C-004
- Invariants: I-061

B-026 V2 publish on L3 workloads versus upstream Linux on the three target machines. No superiority claim without the table.

<!-- covers: INV-1032, INV-0866 -->

#### Out of scope
Register ownership (BEN-027). L4 republish (LNX-092).

#### Acceptance criteria
- [ ] A report exists under `reports/benchmarks/B-026/` for H-002, H-004 and H-005 meeting the V2 publish target.
- [ ] The report names the C-004 non-graphics scripts.
- [ ] The report states no superiority claim.

#### Verification
- Bench: B-026 on H-002, H-004 and H-005; target per register.
- Review: BEN lead confirms the method matches the register.

#### Evidence
- none

### LNX-064 · Bridge native IME via text-input and IBus
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: TXT-029, TXT-030, LNX-043, LNX-040
- Baseline: §41, §47, §49
- Invariants: I-096

zwp_text_input_v3 plus IBus/Fcitx D-Bus so one active IME types into Linux windows. TXT owns the native IME mapping; LNX hosts the Personality protocols. Wine IMM32/TSF is WIN.

<!-- covers: GAP-0255 -->

#### Out of scope
IME host Component (TXT-029). Wayland text-input mapping (TXT-030). Wine IMM32 (WIN-060). CJK gate (TXT-044).

#### Acceptance criteria
- [ ] Typing through the native IME into a GTK and a Qt C-004 window commits the same preedit and commit sequence as a native Text Editor.
- [ ] An IBus-compatible D-Bus endpoint exists inside the Personality and does not leak key events to Components without TextInputFocus.
- [ ] Native crates do not link IBus or Fcitx.

#### Verification
- Integration: `personality:tests/ime/text_input_ibus_*` on `hw-h002` and `hw-h004`.
- Review: TXT lead confirms a single IME host serves both worlds.

#### Evidence
- none

### LNX-065 · Bridge org.freedesktop.Notifications to native
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: APP-014, LNX-043, LNX-036
- Baseline: §47, §49, §62

V2 notification center and do-not-disturb for Linux apps via Notifications and portal D-Bus. Windows toast is WIN.

<!-- covers: GAP-0273 -->

#### Out of scope
Native Notify service (APP-014). Windows toast (WIN-047). Portal file chooser (LNX-036).

#### Acceptance criteria
- [ ] An org.freedesktop.Notifications call from a C-004 app appears in the native notification center with the same do-not-disturb policy.
- [ ] A process without a Notify Capability receives the D-Bus error and does not crash.
- [ ] Native crates do not link libnotify.

#### Verification
- Integration: `personality:tests/notify/fdo_and_portal_*` on `hw-h002` and `hw-h004`.
- Compat: C-004 notification integration scoring on H-002, H-004 and H-005.

#### Evidence
- none

### LNX-066 · Define the Steam runtime sub-Corpus
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: LNX-056, LNX-015, LNX-017
- Baseline: §47, §49, §56.3, §62
- Corpora: C-010

V2 L3 Steam-on-Linux: pressure-vessel, gamescope, userns, 32-bit multilib, `/dev/input` via udev, SDL evdev, with scenario scripts. C-010 is a named sub-corpus scored inside C-004.

<!-- covers: EXTRA-027 -->

#### Out of scope
pressure-vessel host (LNX-071). gamescope host (LNX-070). Gamepad evdev (LNX-074). 32-bit multilib (LNX-086). Proton launch (WIN-046).

#### Acceptance criteria
- [ ] C-010 scenario scripts exist under `compat:steam-runtime` and name pressure-vessel, gamescope, userns, 32-bit, `/dev/input` and SDL evdev.
- [ ] Each script records input, overlay, audio and GPU-acceleration integration checks from the register.
- [ ] Titles requiring kernel-level anti-cheat are excluded and listed as out of this sub-corpus.

#### Verification
- Review: LNX lead and WIN lead sign the C-010 script list on the pull request.
- Compat: C-010 scripts parse in the `compat:steam-runtime` harness on H-001.

#### Evidence
- none

### LNX-067 · Emulate Linux uid, gid, DAC, seccomp and LSM hooks
- Type: build
- Milestone: V2
- Status: todo
- Size: L
- Owner: none
- Depends on: LNX-061, LNX-005, LNX-042, LNX-090
- Baseline: §9.1, §46, §51
- Threats: T-011
- Invariants: I-072

Linux security semantics as needed for Chromium, Flatpak and containers, always bounded by the enclosing Component Capability set (§46). uid 0 inside the Personality is not host authority.

<!-- covers: INV-0857 -->

#### Out of scope
Sandbox primitive retain (LNX-061). Native Capability model (CAP). LSM rewrite (not a native API).

#### Acceptance criteria
- [ ] uid, gid, DAC modes, capabilities(7) and seccomp behave as Linux for a C-004 Chromium and Flatpak entry.
- [ ] A uid-0 Personality process still cannot exceed the enclosing Component grant (I-072).
- [ ] Native Components have no uid and no LSM hook as an API.

#### Verification
- Integration: `personality:tests/security/uid_dac_seccomp_*` on `qemu-x86_64` and `hw-h002`.
- Compat: C-004 Chromium sandbox and Flatpak entries on H-002.

#### Evidence
- none

### LNX-068 · Emulate NetworkManager D-Bus and resolv.conf
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: NET-015, NET-019, LNX-043, LNX-005
- Baseline: §47, §49, §62

Browsers, chat clients and package managers query NM and nsswitch. Windows proxy/WinINet is WIN. Native software uses NetworkConnection, not NM.

<!-- covers: GAP-0295 -->

#### Out of scope
Native netmgr (NET-015). Resolver (NET-019). Windows proxy (WIN). System proxy at V3 (NET-032).

#### Acceptance criteria
- [ ] A C-004 browser queries NetworkManager D-Bus and `/etc/resolv.conf` and connects using the native netmgr state.
- [ ] A process without a network Capability sees disconnected NM state and cannot open a socket.
- [ ] Native crates do not link libnm.

#### Verification
- Integration: `personality:tests/nm/dbus_resolv_*` on `hw-h002`, `hw-h004` and `hw-h005`.
- Compat: C-004 browser and chat network-state scenarios on H-004.

#### Evidence
- none

### LNX-069 · Emulate UPower and logind power D-Bus APIs
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: PWR-013, PWR-010, PWR-012, LNX-046, LNX-043
- Baseline: §22, §47, §62

Battery, inhibitors, session and seat queries for Linux apps backed by native power and Session services on V2 laptops. Native software never sees logind.

<!-- covers: GAP-0314 -->

#### Out of scope
Native Power Component (PWR-013). Inhibit Capabilities (PWR-012). Native Session (SEC-028).

#### Acceptance criteria
- [ ] A C-004 browser or media player reads battery and inhibitor state from UPower and logind D-Bus that matches `os inspect` power state on H-004 and H-005.
- [ ] Taking an inhibitor through logind holds Capability<InhibitIdle> or Capability<InhibitSuspend> on the enclosing Component.
- [ ] Native crates do not link libupower or call logind.

#### Verification
- Integration: `personality:tests/power/upower_logind_*` on `hw-h004` and `hw-h005`.
- Review: PWR lead confirms inhibitors are the native Capabilities.

#### Evidence
- none

### LNX-070 · Host gamescope for Steam and Linux games
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: LNX-066, LNX-040, LNX-031, GFX-069
- Baseline: §47, §56.3, §62

V2 Steam runtime sub-corpus and L3 games via Steam: gamescope nested compositor inside the Personality, not a native GPU stack rewrite.

<!-- covers: EXTRA-027 -->

#### Out of scope
HDR Surfaces (GFX-069). Steam corpus scripts (LNX-066). Proton (WIN).

#### Acceptance criteria
- [ ] gamescope hosts a C-010 title as a compositor window with input and GPU acceleration on H-002.
- [ ] Native compositor crates do not link gamescope.
- [ ] Killing gamescope does not kill the native compositor.

#### Verification
- Integration: `personality:tests/steam/gamescope_host_*` on `hw-h002`.
- Compat: C-010 gamescope scenarios on H-002.

#### Evidence
- none

### LNX-071 · Host pressure-vessel for the Steam runtime
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: LNX-066, LNX-045, LNX-061, LNX-058
- Baseline: §36, §46, §62

V2 Steam runtime sub-corpus: pressure-vessel container on Personality namespaces and overlayfs.

<!-- covers: EXTRA-027 -->

#### Out of scope
Corpus scripts (LNX-066). OCI engine (LNX-039). 32-bit multilib (LNX-086).

#### Acceptance criteria
- [ ] pressure-vessel starts a C-010 title inside Personality user and mount namespaces on H-002.
- [ ] overlayfs used by pressure-vessel is the verified Personality overlayfs.
- [ ] Native `os env` does not invoke pressure-vessel.

#### Verification
- Integration: `personality:tests/steam/pressure_vessel_*` on `hw-h002`.
- Compat: C-010 pressure-vessel scenarios on H-002.

#### Evidence
- none

### LNX-072 · Implement the incremental native-Interface bridge
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: LNX-062, LNX-036, LNX-016
- Baseline: §3, §42, §46
- Invariants: I-025

V2 migration path so existing Linux apps are not all-or-nothing rewrites. Limits keep the §3 firewall: a Linux app that opts into a named native interface still cannot see POSIX as a native API.

#### Out of scope
Adoption Decision (LNX-062). Semantic registry (SEM). Native opt-in of native apps (LNX-016).

#### Acceptance criteria
- [ ] A Linux app that opts into the native file chooser receives UserSelected and cannot open POSIX paths outside that grant.
- [ ] Mixing a native Channel call with an undeclared POSIX ambient path fails the firewall lint and the runtime check.
- [ ] Native software that does not opt in still sees no Personality types.

#### Verification
- Integration: `personality:tests/bridge/incremental_native_*` on `qemu-x86_64` and `hw-h002`.
- Review: ABI lead confirms the §3 firewall still holds.

#### Evidence
- none

### LNX-073 · Keep L0 through L2 Corpus results green
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: LNX-041, LNX-056, LNX-090
- Baseline: §46, §62
- Corpora: C-001, C-002, C-003
- Risks: R-064
- Invariants: I-096

V2 compatibility gate: L0, L1 and L2 pass with zero regressions versus V1 while L3 is added. Translation must not silently regress daily-driving.

#### Out of scope
L3 run (LNX-084). Overhead publish (LNX-063).

#### Acceptance criteria
- [ ] C-001, C-002 and C-003 meet their V2 zero-regression thresholds versus V1 on H-001, H-002 and H-004.
- [ ] A translation patch that drops a previously passing C-003 browser or IDE case fails CI.

#### Verification
- Compat: C-001, C-002 and C-003 on H-001, H-002 and H-004.
- Review: LNX lead signs the zero-regression comparison.

#### Evidence
- none

### LNX-074 · Expose SDL game-controller evdev to Linux games
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: LNX-066, LNX-044, HW-049
- Baseline: §47, §49, §62

V2 Steam runtime sub-corpus: SDL game-controller evdev and `/dev/input` via udev inside the Personality. Native software holds Capability<InputDevice>, not evdev nodes.

#### Out of scope
Native gamepad Capabilities (HW-049). XInput (WIN-056). udev nodes (LNX-044).

#### Acceptance criteria
- [ ] An SDL C-010 title sees the lab gamepad through `/dev/input` and udev on H-002.
- [ ] A Personality process without an input grant has no `/dev/input` nodes for that device.
- [ ] Native Components do not open evdev nodes.

#### Verification
- Integration: `personality:tests/input/sdl_evdev_*` on `hw-h002`.
- Compat: C-010 input scoring on H-002.

#### Evidence
- none

### LNX-075 · Present HDR and VRR to Linux fullscreen applications
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: GFX-069, GFX-068, LNX-070, LNX-031
- Baseline: §40, §47, §62
- Risks: R-038

V2 HDR/VRR gates for Linux/Steam fullscreen via the Personality and gamescope, not a native GPU stack rewrite.

#### Out of scope
HDR output pipeline (GFX-068). Personality HDR Surfaces (GFX-069). Colorimeter (LAB-017).

#### Acceptance criteria
- [ ] A C-010 or C-004 fullscreen title presents HDR metadata on the H-002 HDR display through the Personality.
- [ ] VRR is exercised on that path when the display advertises it.
- [ ] Native applications still do not see DRM HDR properties.

#### Verification
- Integration: `personality:tests/hdr/linux_fullscreen_*` on `hw-h002`.
- Demo: V2 Linux/Steam fullscreen HDR on H-002.
- Review: GFX lead confirms metadata comes from GFX-069.

#### Evidence
- none

### LNX-076 · Support per-display scaling for Linux applications
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: GFX-077, GFX-080, UIP-047, LNX-038
- Baseline: §40, §47, §49, §62

V2 multi-monitor hot-plug with different scale factors. Linux windows follow native per-display and fractional scaling.

<!-- covers: INV-0886 -->

#### Out of scope
Compositor hot-plug (GFX-077). Toolkit relayout (UIP-047). V1 scaling (LNX-038).

#### Acceptance criteria
- [ ] A C-004 window follows native per-display scale when moved between two displays on H-004 and H-005.
- [ ] Hot-plug of a second display 100 times does not exit the Linux client.
- [ ] Native compositor crates still do not link Wayland.

#### Verification
- Integration: `personality:tests/display/per_display_scale_hotplug_*` on `hw-h004` and `hw-h005`.
- Compat: C-004 scaling integration scoring on H-004 and H-005.

#### Evidence
- none

### LNX-077 · Bridge appearance through the settings portal
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: UIP-044, LNX-036, SVC-013
- Baseline: §47, §49, §62

V2 desktop polish: light/dark, accent and font scale to GTK/Qt via the settings portal so Linux windows match the shell.

<!-- covers: GAP-0281 -->

#### Out of scope
Native appearance model (UIP-044). Settings appearance panel (APP). Portal core (LNX-036).

#### Acceptance criteria
- [ ] Changing native light/dark, accent and font scale updates a GTK and a Qt C-004 window through the settings portal.
- [ ] The portal reads the same settings objects native apps read; there is no second store.
- [ ] Native crates do not link gsettings as a native API.

#### Verification
- Integration: `personality:tests/settings/appearance_portal_*` on `hw-h002` and `hw-h004`.
- Review: UIP lead confirms a single settings object.

#### Evidence
- none

### LNX-078 · Map file descriptors onto native capabilities
- Type: build
- Milestone: V2
- Status: todo
- Size: L
- Owner: none
- Depends on: LNX-042, LNX-090, LNX-005, CAP-005
- Baseline: §7, §46
- Invariants: I-015, I-072

§46 fd namespace over native Capabilities so Linux fds cannot outlive or exceed the enclosing Component grant. Native software still holds typed handles, not fds.

<!-- covers: INV-0853 -->

#### Out of scope
Process mapping (LNX-042). POSIX path view (LNX-047). Native handle table (ABI, CAP).

#### Acceptance criteria
- [ ] Each live fd in a Personality process names a Capability in the enclosing Component table, inspectable in `os inspect`.
- [ ] Closing the Component invalidates every fd; a subsequent use returns `EBADF` and allocates no handle.
- [ ] An fd cannot be used to reach an object outside the Component grant.

#### Verification
- Integration: `personality:tests/fd/cap_map_*` on `qemu-x86_64` and `hw-h002`.
- Fuzz: `personality:fuzz/fd_map` without panic on `qemu-x86_64`.
- Review: CAP lead confirms fds never outlive the table.

#### Evidence
- none

### LNX-079 · Polish Linux desktop integration on target machines
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: LNX-038, LNX-076, LNX-065, LNX-077, APP-032
- Baseline: §47, §49, §62
- Invariants: I-096

V2 three-machine desktop: taskbar, clipboard, drag/drop and notifications for Linux apps at preview quality (§62).

<!-- covers: INV-1223 -->

#### Out of scope
V1 integration (LNX-038). Launcher search (APP-032). Windows desktop integration (WIN-024).

#### Acceptance criteria
- [ ] C-004 integration scoring for launcher, taskbar, clipboard, drag/drop and notifications meets the register threshold on H-002, H-004 and H-005.
- [ ] Linux apps appear with `.desktop` icons in the V2 launcher and no separate desktop.
- [ ] A reviewer following the V2 desktop script on H-005 records pass for those checks.

#### Verification
- Compat: C-004 integration scoring on H-002, H-004 and H-005.
- Demo: V2 Linux applications on the native desktop on H-005.
- Manual: V2 desktop UX script on H-002, H-004 and H-005.

#### Evidence
- none

### LNX-080 · Bridge camera portal to native camera Capability
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: LNX-036, APP-025, APP-031, SEC-044
- Baseline: §9.1, §47, §62
- Threats: T-014
- Invariants: I-021

V2 permissions demo: a Linux-personality video app denied camera access does not crash and receives no frames. Personality apps receive no V4L2 node, only a minted Capability.

#### Out of scope
Native camera service (MED-020). Consent UI (APP-025). V4L2 as a native API (forbidden).

#### Acceptance criteria
- [ ] A Linux video app granted camera access receives frames through the portal and shows the shell in-use indicator.
- [ ] The same app denied camera access receives a portal error, allocates no V4L2 fd, and does not crash.
- [ ] Native Components never see `/dev/video*`.

#### Verification
- Integration: `personality:tests/portals/camera_grant_deny_*` on `hw-h004`.
- Demo: V2 camera permission prompt for a Linux video app on H-004.

#### Evidence
- none

### LNX-081 · Bridge ScreenCast portal to native screen-share
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: LNX-036, APP-025, APP-031, SEC-044
- Baseline: §9.1, §40, §47, §62
- Threats: T-013
- Invariants: I-085

V2 screen-share Capability and L3 conferencing: Linux apps get a granted Surface or a denial, with a persistent indicator. Screen capture is S-034.

<!-- covers: INV-0888 -->

#### Out of scope
Native screen-capture Capability (GFX, S-034). Consent UI (APP-025). Encode of captured frames (MED).

#### Acceptance criteria
- [ ] A granted ScreenCast portal call delivers a Surface and shows the persistent indicator.
- [ ] A denied call returns a black or denied surface and the app does not crash.
- [ ] An app without the screen-capture Capability cannot sample another client's buffer.

#### Verification
- Integration: `personality:tests/portals/screencast_grant_deny_*` on `hw-h002` and `hw-h004`.
- Demo: V2 Linux conferencing screen-share on H-002.
- Compat: C-004 conferencing entries on H-002.

#### Evidence
- none

### LNX-082 · Provide signals inside the Linux Personality
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: LNX-083, LNX-042, TSK-043
- Baseline: §46
- Invariants: I-018

POSIX signals as Personality-provided semantics. V3 later reimplements delivery over native primitives. Native software uses Operations and Channels, not signals.

<!-- covers: INV-0854 -->

#### Out of scope
Signal delivery over native primitives (LNX-101). Native cancellation (TSK). Process model (LNX-083).

#### Acceptance criteria
- [ ] SIGTERM, SIGCHLD, SIGWINCH and a handled SIGINT behave as Linux for C-003 and C-004 CLI and desktop entries.
- [ ] Native Components have no signal table and cannot raise a POSIX signal.
- [ ] `os inspect` attributes a delivered signal to the Personality process, not to a native Task.

#### Verification
- Integration: `personality:tests/signals/posix_delivery_*` on `qemu-x86_64` and `hw-h002`.
- Compat: C-003 CLI job-control scenarios on H-001.

#### Evidence
- none

### LNX-083 · Provide the POSIX process model in the Personality
- Type: build
- Milestone: V2
- Status: todo
- Size: L
- Owner: none
- Depends on: LNX-042, CMP-036, LNX-090
- Baseline: §3, §46
- Invariants: I-014

PIDs, parent/child, wait and exit codes as Personality behaviour over native Components, not as native abstractions (§46).

<!-- covers: INV-0852 -->

#### Out of scope
fork as native (forbidden). fork over native primitives (LNX-095). Component mapping Decision (CMP-036).

#### Acceptance criteria
- [ ] fork/exec/wait and exit codes match Linux for C-003 job-control and C-004 IDE child processes.
- [ ] Each Personality PID maps to the Component the mapping Decision named, visible in `os inspect`.
- [ ] Native software has no PID namespace and cannot call wait(2).

#### Verification
- Integration: `personality:tests/procmodel/pid_wait_exit_*` on `qemu-x86_64` and `hw-h002`.
- Compat: C-003 and C-004 process-model scenarios on H-001 and H-002.

#### Evidence
- none

### LNX-084 · Define and run the L3 Corpus
- Type: build
- Milestone: V2
- Status: todo
- Size: L
- Owner: none
- Depends on: LNX-073, LNX-066, LNX-071, LNX-070, LNX-086, LNX-079, LNX-081, LNX-068, LNX-069, LNX-074, LNX-075, LNX-082, LNX-085, LNX-088, LNX-089, TXT-035
- Baseline: §47, §49, §62
- Corpora: C-004, C-010
- Risks: R-064
- Invariants: I-096

V2 compatibility gate: C-004 including Flatpak, Steam-on-Linux, media players, creative tools and conferencing, with integration scoring. C-010 is scored inside this corpus.

#### Out of scope
Public notes publication (DOC-021). Compatibility database (REL-015). L4 corpus (LNX-100).

#### Acceptance criteria
- [ ] C-004 scenario scripts exist under `compat:linux-L3` and include the C-010 Steam sub-corpus.
- [ ] The committed report on H-002, H-004 and H-005 meets the C-004 V2 threshold.
- [ ] Integration scoring covers launcher, clipboard, notifications, file chooser, audio and scaling.

#### Verification
- Compat: C-004 and C-010 on H-002, H-004 and H-005.
- Integration: `personality:tests/corpus/l3_*` on `hw-h002`.
- Review: REL lead confirms results feed the published corpus.

#### Evidence
- none

### LNX-085 · Shim org.freedesktop.secrets onto native secrets
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: SEC-027, SEC-026, LNX-043, LNX-025
- Baseline: §9, §51, §47
- Threats: T-025

Linux libsecret / org.freedesktop.secrets with per-application scoping. Windows Credential Manager/DPAPI is WIN. Browsers and mail clients must not share a password store.

<!-- covers: GAP-0224 -->

#### Out of scope
Native secrets service (SEC-027). Windows DPAPI (WIN-025). Isolation suite (SEC-026).

#### Acceptance criteria
- [ ] A C-004 browser stores a secret through libsecret and another Personality app cannot read it.
- [ ] The secret is a Capability-scoped object under SEC-027.
- [ ] Native crates do not link libsecret.

#### Verification
- Integration: `personality:tests/secrets/libsecret_scope_*` on `qemu-x86_64` and `hw-h002`.
- Review: SEC lead confirms per-application scoping.

#### Evidence
- none

### LNX-086 · Ship 32-bit multilib for the Steam runtime
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: LNX-015, LNX-035, LNX-066, LNX-071
- Baseline: §46, §62
- Risks: R-032

V2 Steam runtime sub-corpus 32-bit multilib, contingent on the V1 ia32 Decision remaining retain. If that Decision rejected ia32, a follow-up drops this task.

<!-- covers: EXTRA-027 -->

#### Out of scope
ia32 Decision (LNX-015). ia32 userland (LNX-035). WoW64 (WIN-055).

#### Acceptance criteria
- [ ] A C-010 32-bit title loads the 32-bit dynamic linker and SDL from the Personality multilib view on H-016 and H-002.
- [ ] Kernel config on H-016 still enables ia32 while the Decision retains it.
- [ ] Native Components have no 32-bit syscall path.

#### Verification
- Integration: `personality:tests/steam/multilib_32_*` on `qemu-ia32` and `hw-h002`.
- Compat: C-010 32-bit entries on H-002.

#### Evidence
- none

### LNX-087 · Evaluate Flatpak and Snap as Personality Package sources
- Type: spike
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: LNX-058, LNX-036, LNX-017
- Baseline: §47, §56.3

Runtime, portals and sandbox fidelity of Flatpak and Snap as Linux-personality package sources beyond the V1 Flatpak ship. Informs whether Snap becomes a V3 source.

<!-- covers: INV-0891 -->

#### Out of scope
V1 Flatpak ship (LNX-058). Packaging format Decision (LNX-017). Native Package store (PKG).

#### Acceptance criteria
- [ ] The report runs one Flatpak and one Snap application through portals and sandbox checks and records fidelity versus upstream Linux.
- [ ] The report recommends Snap as a V3 source, as out of 1.0, or as a documented gap.
- [ ] The report is committed at `reports/spikes/LNX-087.md`.

#### Verification
- Report: portal and sandbox fidelity of Flatpak versus Snap on C-004-class apps; licence and maintenance cost; whether Snap is in 1.0.
- Review: PKG lead records the recommendation against PKG immutability.

#### Evidence
- none

### LNX-088 · Synthesise POSIX mounts over native storage
- Type: build
- Milestone: V2
- Status: todo
- Size: L
- Owner: none
- Depends on: LNX-019, LNX-047, STO-047, LNX-090
- Baseline: §25, §46
- Invariants: I-016, I-044

§46 mounts and POSIX path namespace over the native storage model; not a native filesystem. Foreign and network filesystems stay STO.

<!-- covers: INV-0856 -->

#### Out of scope
Native filesystem (forbidden by I-044). View API (STO-047). Foreign volumes (STO-074). Network filesystems (STO-059).

#### Acceptance criteria
- [ ] `mount` and `/proc/mounts` inside the Personality show Capability-scoped views, not a global FHS native store.
- [ ] A mount outside the Component grant fails at the Personality boundary.
- [ ] Native Components have no mount table.

#### Verification
- Integration: `personality:tests/mounts/posix_view_*` on `qemu-x86_64` and `hw-h002`.
- Review: STO lead confirms mounts are views over native objects.

#### Evidence
- none

### LNX-089 · Synthesise /proc over native inspectables
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: LNX-048, LNX-042, OBS-038, LNX-090
- Baseline: §24, §46, §64
- Threats: T-011

§46 `/proc` as a synthesised Personality view over native Component/Task inspectables rather than ambient kernel proc.

<!-- covers: INV-0855 -->

#### Out of scope
V1 procfs retain (LNX-048). Native inspect interface (OBS-006). Personality tracing (OBS-038).

#### Acceptance criteria
- [ ] `/proc/self` and `/proc/<pid>` for Personality processes are served from inspectables after translation is on.
- [ ] `/proc` of another Component remains `EACCES` without a debug Capability.
- [ ] Native Components still have no `/proc` namespace.

#### Verification
- Integration: `personality:tests/proc/synthesised_*` on `qemu-x86_64` and `hw-h002`.
- Review: OBS lead confirms fields come from inspect data.

#### Evidence
- none

### LNX-090 · Translate Linux syscalls onto native primitives
- Type: build
- Milestone: V2
- Status: todo
- Size: L
- Owner: none
- Depends on: LNX-003, LNX-030, LNX-042, ABI-041
- Baseline: §6, §46
- Risks: R-025
- Invariants: I-025, I-049

Long-term Personality layer: syscall translation onto native primitives per the V0.5 depth Decision and Phase C (§6, §46). Native software still does not see Linux syscalls.

<!-- covers: INV-0851 -->

#### Out of scope
Depth Decision (LNX-003). Dual-implementation conformance (ABI-044). fork/epoll/signals/pidns over native (V3 LNX tasks).

#### Acceptance criteria
- [ ] The depth Decision's V2 translation path is the default for new Personality processes on H-001 and H-002.
- [ ] A translated process still passes C-003 browser and IDE scenarios on H-002.
- [ ] Native Components continue to fail Linux syscall numbers.

#### Verification
- Integration: `personality:tests/translate/syscall_to_native_*` on `qemu-x86_64` and `hw-h002`.
- Compat: C-003 browser and IDE on the translation path on H-002.
- Review: ABI lead confirms every translated object terminates in a native Object<T>.

#### Evidence
- none

### LNX-091 · Accept community Linux compatibility reports
- Type: build
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: LNX-100, LNX-099
- Baseline: §56.3, §63
- Corpora: C-005
- Threats: T-042

V3 community-submitted compatibility reports with a documented verification process into the compatibility database. REL collects; LNX verifies Linux entries. Submissions are opt-in and contain no serial numbers (T-042).

#### Out of scope
Intake pipeline (REL-022). Windows reports (WIN). HCL hardware reports (HW, REL).

#### Acceptance criteria
- [ ] A documented verification process exists for a community Linux report: reproduce against C-005 scripts or record why it cannot be reproduced.
- [ ] An accepted report appears in the compatibility database with Personality version and result.
- [ ] Rejected submissions that contain serial numbers or network identifiers are dropped before publish.

#### Verification
- Manual: process a fixture community report through the verification checklist.
- Review: REL lead confirms the Linux path matches REL-022.

#### Evidence
- none

### LNX-092 · Publish Linux compatibility overhead on L4 workloads
- Type: benchmark
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: LNX-063, LNX-100, Q-001
- Baseline: §54, §63
- Benchmarks: B-026
- Corpora: C-005
- Invariants: I-061

B-026 V3 publish on L4 workloads across Tier 1. No superiority claim without the table.

#### Out of scope
Register ownership (BEN). L5 republish (LNX-102).

#### Acceptance criteria
- [ ] A report exists under `reports/benchmarks/B-026/` for every V3 Tier 1 machine in hardware scope meeting the V3 publish target.
- [ ] The report names the C-005 non-graphics scripts.
- [ ] The report states no superiority claim.

#### Verification
- Bench: B-026 on every V3 Tier 1 H-ID; target per register.
- Review: BEN lead confirms the method matches the register.

#### Evidence
- none

### LNX-093 · Expose the native print service as a CUPS-compatible IPP socket
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-071, HW-041, APP-064, LNX-005
- Baseline: §33, §47, §49, §63

Linux personality print bridging. Wine spooler follows WIN. Native print dialog stays APP. The socket is inside the Personality and mints per-job print Capabilities.

<!-- covers: GAP-0271 -->

#### Out of scope
Native print service (HW-071). Print dialog (APP-064). Wine spooler (WIN).

#### Acceptance criteria
- [ ] A C-005 GTK app prints through a CUPS-compatible IPP socket to a printer discovered by HW-071 on a V3 Tier 1 machine.
- [ ] The job is a Capability<PrintJob> for that job only; the app cannot enumerate other jobs.
- [ ] Native crates do not link libcups.

#### Verification
- Integration: `personality:tests/print/cups_ipp_*` on `hw-h002`.
- Review: HW lead confirms the native print service is the backend.

#### Evidence
- none

### LNX-094 · Implement epoll over native Operations
- Type: build
- Milestone: V3
- Status: todo
- Size: L
- Owner: none
- Depends on: LNX-090, TSK-011, TSK-012, LNX-078
- Baseline: §18, §46
- Invariants: I-018, I-030

epoll as Personality behaviour over native Operations. The native API stays free of POSIX readiness (§46).

<!-- covers: INV-0860 -->

#### Out of scope
Native Operation ring (TSK). Native readiness API (forbidden). fork over native (LNX-095).

#### Acceptance criteria
- [ ] epoll_wait for a Personality process completes when the underlying Operations complete, for files, sockets and timers used by C-005.
- [ ] Native Components have no epoll fd and submit Operations instead.
- [ ] A cancelled Operation never appears as a spurious epoll event.

#### Verification
- Integration: `personality:tests/epoll/over_operations_*` on `qemu-x86_64` and `hw-h002`.
- Compat: C-005 entries that use epoll on H-002.
- Review: TSK lead confirms no POSIX readiness in the Native ABI.

#### Evidence
- none

### LNX-095 · Implement fork as Personality behaviour
- Type: build
- Milestone: V3
- Status: todo
- Size: L
- Owner: none
- Depends on: LNX-090, LNX-083, LNX-078, CMP-046
- Baseline: §3, §10, §46
- Invariants: I-014

fork() becomes compatibility-layer behaviour over native primitives, never a native abstraction (§46). Correctly late.

<!-- covers: INV-0859 -->

#### Out of scope
POSIX process model retain (LNX-083). Native Component create (CMP). pid namespaces (LNX-098).

#### Acceptance criteria
- [ ] fork in a Personality process creates the child mapping named by CMP-036 without a native fork ABI.
- [ ] C-005 entries that fork still meet their scripts on H-002.
- [ ] Native Components cannot call fork and have no copy-on-write process primitive.

#### Verification
- Integration: `personality:tests/fork/over_native_*` on `qemu-x86_64` and `hw-h002`.
- Compat: C-005 fork-heavy entries on H-002.
- Review: CMP lead confirms fork is not a native abstraction.

#### Evidence
- none

### LNX-096 · Host AT-SPI2 for native screen-reader bridging
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: LNX-043, ACC-001, ACC-024, LNX-084
- Baseline: §42, §49, §63
- Invariants: I-096

V3 Linux apps must be accessible. ACC owns the native reader and AT-SPI bridge; LNX hosts the bus inside the Personality so D-Bus stays in the bridge.

<!-- covers: GAP-0265 -->

#### Out of scope
AT-SPI tree consumption (ACC-026). Native screen reader (ACC-021). Protocol Decision (ACC-001).

#### Acceptance criteria
- [ ] An AT-SPI2 bus is available inside the Personality to C-005 GTK and Qt entries.
- [ ] Native AT clients do not connect to that bus except through ACC-026.
- [ ] A process without an accessibility grant cannot enumerate other apps' AT-SPI trees.

#### Verification
- Integration: `personality:tests/a11y/atspi_bus_*` on `hw-h002`.
- Review: ACC lead confirms the bus is Personality-only.

#### Evidence
- none

### LNX-097 · Keep L0 through L3 Gold Corpus entries green
- Type: build
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: LNX-073, LNX-084, LNX-095
- Baseline: §46, §63
- Corpora: C-001, C-002, C-003, C-004
- Invariants: I-096

V3 compatibility gate: L0 through L3 pass with zero regressions on Gold entries versus V2 while L4 is added.

#### Out of scope
L4 run (LNX-100). Overhead publish (LNX-092).

#### Acceptance criteria
- [ ] C-001 through C-003 meet their V3 zero-regression thresholds versus V2.
- [ ] C-004 Gold entries meet the V3 zero-regression-on-Gold threshold versus V2 on V3 Tier 1 machines.

#### Verification
- Compat: C-001 through C-004 Gold on every V3 Tier 1 H-ID in scope.
- Review: LNX lead signs the Gold-hold comparison.

#### Evidence
- none

### LNX-098 · Implement pid namespaces over Components
- Type: build
- Milestone: V3
- Status: todo
- Size: L
- Owner: none
- Depends on: LNX-045, LNX-095, LNX-042, LNX-039
- Baseline: §10, §36, §46
- Invariants: I-014, I-019

pid namespaces inside the Personality over native Components and ResourceDomains for containers at alpha quality (§46). Native software still has no pid namespace.

<!-- covers: INV-0862 -->

#### Out of scope
Namespace retain (LNX-045). OCI runtime (LNX-039). Native isolation (CMP, SCH).

#### Acceptance criteria
- [ ] A Personality pid namespace isolates PIDs of an OCI container to that Component graph, visible in `os inspect`.
- [ ] Native Components have no pid namespace and enumerate Object<Component> instead.
- [ ] C-005 container-runtime entries still pass on H-002.

#### Verification
- Integration: `personality:tests/pidns/over_components_*` on `qemu-x86_64` and `hw-h002`.
- Compat: C-005 container-runtime entries on H-002.
- Review: CMP lead confirms pidns is Personality-only.

#### Evidence
- none

### LNX-099 · Publish the Linux compatibility guide
- Type: docs
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: LNX-100, LNX-054, DOC-021, LNX-028
- Baseline: §56.3, §56.5, §63
- Invariants: I-096

V3 public documentation: compatibility guide for strangers installing Linux software. DOC owns the book; LNX authors the Linux personality chapters. 1.0 docs consume this.

<!-- covers: INV-1248 -->

#### Out of scope
Docs site (DOC-028). Windows chapters (WIN-081). Five-minute guide (LNX-054).

#### Acceptance criteria
- [ ] A committed guide covers how to install Linux software via the first-class packaging path, what the Personality provides, and what deliberately does not exist.
- [ ] The guide cites C-005 and does not restate pass-rate numbers in prose.
- [ ] Trademark wording matches GOV policy.

#### Verification
- Review: DOC lead, LNX lead and GOV licensing reviewer sign-off recorded on the pull request.

#### Evidence
- none

### LNX-100 · Define and run the L4 Corpus
- Type: build
- Milestone: V3
- Status: todo
- Size: L
- Owner: none
- Depends on: LNX-097, LNX-096, LNX-093, BLD-017, LNX-094, LNX-098, LNX-101
- Baseline: §49, §56.3, §63
- Corpora: C-005
- Invariants: I-096

V3 public-alpha quality: C-005 applications from Flathub, distro and Steam-on-Linux popularity, against the conformance suite.

<!-- covers: INV-1245 -->

#### Out of scope
Community report verification (LNX-091). L5 corpus (LNX-107). Docs guide (LNX-099).

#### Acceptance criteria
- [ ] C-005 scenario scripts exist under `compat:linux-L4`.
- [ ] The committed report on every V3 Tier 1 machine meets the C-005 V3 threshold.
- [ ] Accessibility is scored where the toolkit exposes it, using the hosted AT-SPI bus.

#### Verification
- Compat: C-005 scenario `compat:linux-L4` on every V3 Tier 1 H-ID.
- Integration: `personality:tests/corpus/l4_*` on `hw-h002`.

#### Evidence
- none

### LNX-101 · Implement signal delivery over native primitives
- Type: build
- Milestone: V3
- Status: todo
- Size: L
- Owner: none
- Depends on: LNX-082, LNX-090, TSK-013, LNX-095
- Baseline: §18, §21, §46
- Invariants: I-018

Signal delivery reimplemented over native primitives inside the Personality. Correctly late per critique. Native software still uses Operations and TaskGroup cancellation, not signals.

<!-- covers: INV-0861 -->

#### Out of scope
V2 signal retain (LNX-082). Native cancellation (TSK). epoll over Operations (LNX-094).

#### Acceptance criteria
- [ ] SIGTERM, SIGCHLD and handled SIGINT for Personality processes are delivered via native Operations and TaskGroup cancellation, not a native signal ABI.
- [ ] C-005 entries that use those signals still pass on H-002.
- [ ] Native Components have no signal table.

#### Verification
- Integration: `personality:tests/signals/over_native_*` on `qemu-x86_64` and `hw-h002`.
- Compat: C-005 signal-using entries on H-002.
- Review: TSK lead confirms no native signal ABI.

#### Evidence
- none

### LNX-102 · Publish Linux compatibility overhead on L5 workloads
- Type: benchmark
- Milestone: V4
- Status: todo
- Size: S
- Owner: none
- Depends on: LNX-092, LNX-107, Q-001
- Baseline: §54, §63
- Benchmarks: B-026
- Corpora: C-006
- Invariants: I-061

B-026 V4 publish on L5 workloads on every Tier 1 machine. No superiority claim without the table.

#### Out of scope
1.0 republish (LNX-108). Register ownership (BEN).

#### Acceptance criteria
- [ ] A report exists under `reports/benchmarks/B-026/` for every V4 Tier 1 H-ID meeting the V4 publish target.
- [ ] The report names the C-006 non-graphics scripts.
- [ ] The report states no superiority claim.

#### Verification
- Bench: B-026 on every V4 Tier 1 H-ID; target per register.
- Review: BEN lead confirms the method matches the register.

#### Evidence
- none

### LNX-103 · Close High findings from the Personality audit
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: SEC-070, LNX-005, LNX-067
- Baseline: §51, §63
- Risks: R-055
- Threats: T-011
- Invariants: I-072

V4 external security audit of personalities. All High and Critical Linux-personality findings are fixed and re-verified. WIN owns Windows-personality findings.

#### Out of scope
Audit commissioning (SEC-070, SEC-070). Kernel capability findings (CAP-050). Windows findings (WIN-073).

#### Acceptance criteria
- [ ] Every High and Critical finding tagged Linux personality is fixed with a regression test.
- [ ] The auditor re-verifies those findings as closed.
- [ ] Medium findings tagged Linux personality are triaged with public tracking.

#### Verification
- Review: auditor re-verification recorded on the pull request; SEC lead signs the Linux-personality subset.
- Integration: new regression tests from findings on `qemu-x86_64` and `hw-h002`.

#### Evidence
- none

### LNX-104 · Keep L0 through L4 Gold Corpus entries green
- Type: build
- Milestone: V4
- Status: todo
- Size: S
- Owner: none
- Depends on: LNX-097, LNX-100
- Baseline: §46, §63
- Corpora: C-001, C-002, C-003, C-004, C-005
- Invariants: I-096

V4 compatibility gate: L0 through L4 pass with zero regressions on Gold entries versus V3 while L5 is added.

#### Out of scope
L5 run (LNX-107). Overhead publish (LNX-102).

#### Acceptance criteria
- [ ] C-001 through C-003 meet their V4 zero-regression thresholds versus V3.
- [ ] C-004 and C-005 Gold entries meet the V4 zero-regression-on-Gold threshold versus V3 on V4 Tier 1 machines.

#### Verification
- Compat: C-001 through C-005 Gold on every V4 Tier 1 H-ID in scope.
- Review: LNX lead signs the Gold-hold comparison.

#### Evidence
- none

### LNX-105 · Lock Linux Personality product contracts for 1.x
- Type: docs
- Milestone: V4
- Status: todo
- Size: S
- Owner: none
- Depends on: LNX-003, LNX-016, LNX-018, LNX-107, LNX-090, LNX-009
- Baseline: §46, §56.3, §65, §66
- Freezes: S-030
- Invariants: I-040, I-096

V4 feature freeze: Personality product contracts (translation present, portals, corpora) locked for 1.x. POSIX is not an L1 freeze. S-030 is the L2 Personality interface surface.

#### Out of scope
Layer 1 freeze (ABI-049). Windows contracts (WIN-076). Corpus data publish (LNX-106).

#### Acceptance criteria
- [ ] A committed contract lists translation presence, portal set, corpus set C-001 through C-006 and C-010, and nongoals (kernel-level anti-cheat, native POSIX).
- [ ] S-030 is recorded frozen by this task and names the exploring spikes LNX-009, LNX-060 and LNX-010 plus the Decisions of LNX-016 and LNX-003.
- [ ] The contract states that no L1 surface is frozen here (I-040).

#### Verification
- Review: ABI lead and LNX lead sign-off recorded on the pull request that freezes S-030.

#### Evidence
- none

### LNX-106 · Publish Corpora and results as machine-readable data
- Type: docs
- Milestone: V4
- Status: todo
- Size: S
- Owner: none
- Depends on: LNX-107, REL-015, LNX-104
- Baseline: §49, §56.3, §63
- Corpora: C-001, C-002, C-003, C-004, C-005, C-006, C-010
- Invariants: I-088

V4 gate: compatibility corpora and results published machine-readable so third parties can reproduce pass rates. Numbers live in the register and in reports, not in prose.

#### Out of scope
Windows ratings export (WIN-074). Database product (REL). Scenario authorship (the run-lN tasks).

#### Acceptance criteria
- [ ] Machine-readable exports of C-001 through C-006 and C-010, plus the V4 result set, are published with the scenario scripts.
- [ ] A third party can recompute the V4 pass rates from those files without a prose table of numbers.
- [ ] Gold-hold comparisons versus V3 are included.

#### Verification
- Review: REL lead and LNX lead sign-off recorded on the pull request.
- Manual: recompute C-006 V4 pass rate from the published files on one H-ID.

#### Evidence
- none

### LNX-107 · Define and run the L5 Corpus
- Type: build
- Milestone: V4
- Status: todo
- Size: L
- Owner: none
- Depends on: LNX-104, ACC-033, BLD-017
- Baseline: §49, §56.3, §63
- Corpora: C-006, C-010
- Risks: R-064
- Invariants: I-096

V4 compatibility gate: C-006 with the register's required-pass entries for browser, IDE, container runtime, Steam client and office suite.

#### Out of scope
1.0 hold (LNX-110). Machine-readable export (LNX-106). Windows W3 (WIN).

#### Acceptance criteria
- [ ] C-006 scenario scripts exist under `compat:linux-L5` and include C-010 with the Steam client clause from the register.
- [ ] The committed report on every V4 Tier 1 machine meets the C-006 V4 threshold, including the register's required-pass entries for browser, IDE, container runtime, Steam client and office suite.
- [ ] Integration scoring includes accessibility where the toolkit exposes it and audio.

#### Verification
- Compat: C-006 and C-010 on every V4 Tier 1 H-ID.
- Integration: `personality:tests/corpus/l5_*` on `hw-h002`.

#### Evidence
- none

### LNX-108 · Publish Linux compatibility overhead at 1.0
- Type: benchmark
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: LNX-102, LNX-110, Q-001
- Baseline: §54
- Benchmarks: B-026
- Corpora: C-006
- Invariants: I-061

B-026 1.0 publish of L5 workload overhead beside upstream Linux on every Tier 1 machine. No superiority claim without the table.

#### Out of scope
Register ownership (BEN). Claim audit (BEN-062).

#### Acceptance criteria
- [ ] A report exists under `reports/benchmarks/B-026/` for every 1.0 Tier 1 H-ID meeting the 1.0 publish target.
- [ ] The report sits beside the upstream Linux baseline on the same machines.
- [ ] The report states no superiority claim.

#### Verification
- Bench: B-026 on every 1.0 Tier 1 H-ID; target per register.
- Review: BEN lead confirms the 1.0 table matches the register method.

#### Evidence
- none

### LNX-109 · Document unsupported Linux software classes
- Type: docs
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: LNX-106, LNX-105, WIN-083
- Baseline: §48, §56.3, §57
- Invariants: I-071, I-096

1.0 statement of what is not supported: kernel-level anti-cheat, vendor DRM, and Broken corpus entries with reasons. Bypass remains a nongoal.

#### Out of scope
Windows unsupported matrix (WIN-083). Anti-cheat policy (WIN-002). VM fallback (VIRT).

#### Acceptance criteria
- [ ] A committed statement lists kernel-level anti-cheat, vendor DRM, and Broken C-006 entries with reasons.
- [ ] The statement cites I-071 and does not promise those titles.
- [ ] Release notes link this statement.

#### Verification
- Review: LNX lead, WIN lead and GOV lead sign-off recorded on the pull request.

#### Evidence
- none

### LNX-110 · Hold L5 pass rate with zero Gold regressions
- Type: build
- Milestone: 1.0
- Status: todo
- Size: M
- Owner: none
- Depends on: LNX-107, LNX-104, LNX-103, AUD-029, ACC-033
- Baseline: §49, §56.3
- Corpora: C-006, C-010
- Risks: R-064
- Invariants: I-096

1.0 compatibility gate: L5 holds the register threshold, zero regressions on V4 Gold entries, and integration requirements hold for every passing Linux entry.

#### Out of scope
Final report authorship (LNX-111). Overhead table (LNX-108). Windows hold (WIN-085).

#### Acceptance criteria
- [ ] C-006 on the 1.0 candidate meets the register 1.0 threshold on every Tier 1 machine, including the required-pass entries for browser, IDE, container runtime, Steam client and office suite.
- [ ] Zero Gold regressions versus V4 on C-006 Gold entries.
- [ ] Integration scoring (launcher, taskbar, notifications, clipboard, audio, file chooser, input, scaling, accessibility where exposed) holds for every passing entry.

#### Verification
- Compat: C-006 and C-010 on every 1.0 Tier 1 H-ID.
- Review: LNX lead signs the Gold-hold and integration-hold comparison.

#### Evidence
- none

### LNX-111 · Publish the final L5 compatibility report
- Type: docs
- Milestone: 1.0
- Status: todo
- Size: M
- Owner: none
- Depends on: LNX-110, LNX-106, LNX-109, LNX-108
- Baseline: §49, §56.3, §56.5
- Corpora: C-006, C-010
- Invariants: I-061, I-088

1.0 final compatibility reports for L5 with per-title results and scenario scripts. Pass rates are in the reports and the register, not restated in marketing prose.

#### Out of scope
Docs site pipeline (DOC). Windows final report (WIN). Claim audit (BEN-062).

#### Acceptance criteria
- [ ] A committed final report includes per-title C-006 and C-010 results, scenario script paths, and a pointer to B-026.
- [ ] The report states no performance number except by citing B-026.
- [ ] Broken and unsupported titles point at LNX-109.

#### Verification
- Review: LNX lead, DOC lead and BEN lead sign-off recorded on the pull request.

#### Evidence
- none
