# APP · Native applications and shell
- Prefix: APP
- Lead: none
- Baseline: §9, §9.1, §11, §25, §41, §42, §45, §49, §56.5, §60, §61, §62, §63

<!-- roadmap:generated:begin summary -->
Tasks: 69 live, 0 done, 0 in-progress, 69 todo, 0 dropped. Ready: 0. Blocked: 69. Weighted: 0%.
<!-- roadmap:generated:end -->

## Scope
APP owns the native shell and the first-party native applications that dogfood Components, Capabilities and UserSelected storage. The shell sub-scope is panel and taskbar, launcher, notifications, lock, greeter, quick settings, status indicators, workspaces, overview, window switcher and on-screen displays. The applications sub-scope is Terminal, File Browser, Text Editor, Image Viewer, Settings, store client, consent UI, migration assistant, desktop search, grant surfaces, print dialog, backup UI, automation-rule editor and the native subset of the first-party set. First-party applications are Component graphs: isolated decoders, choosers and PTY helpers hold only the authority they need (§11). Native software never sees Wayland, X11, POSIX window APIs or a home-directory namespace.

## Out of scope
Compositor, trusted-UI overlay protection, lock enforcement and screen-capture Capabilities (GFX). UI protocol, toolkit, clipboard policy, drag and drop and named-action shortcuts (UIP). UserSelected minting, content store, snapshots and the search index provider (STO). Grant taxonomy, identity, Session objects, secrets and permission policy (SEC). Semantic registry, rule engine and AI broker (SEM). Linux and Windows personalities, portals, MPRIS, tray clients and PE icons (LNX, WIN). Text stack, IME protocol, glyph atlas and localization pipeline (TXT). Assistive-technology clients and tree schema (ACC). Installer, import engines, recovery generation and crash-report client (INS). Packages, generations, repository fetch and running-app update mechanics (PKG, REL). Supervision, settings store, default-application registry and safe-mode session (SVC). Power, battery, lid and inhibit Capabilities (PWR). Camera service and encode (MED). Print discovery, Bluetooth host and HID keys (HW). Audio objects and mixer (AUD). Benchmark methodology (BEN). SDK Context and `os` CLI (SDK). Capability encoding and audit export (CAP).

## Tasks

### APP-001 · Publish warm and cold startup for the four native applications
- Type: benchmark
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: APP-004, APP-003, APP-005, APP-006, Q-029, BEN-005
- Baseline: §34, §54, §60
- Benchmarks: B-016, B-017
- Invariants: I-042, I-061

Instrument warm and cold launch of Terminal, File Browser, Text Editor and Image Viewer so the V0.5 gate can publish B-016 and B-017. APP owns the per-application harness that drives `os run` to the compositor's first non-blank frame (Q-029); BEN owns methodology and publication. Warm-startup figures are measurement targets, never public guarantees (I-042).

<!-- covers: INV-1187, INV-1188, INV-1189, INV-1190 -->

#### Out of scope
Methodology and CI publication (BEN-009). Input-to-photon (LAB, GFX-004). V1 absolute target (BEN-030).

#### Acceptance criteria
- [ ] Harness `bench:app-startup-warm` launches each of the four applications and records first non-blank frame per Q-029 on H-002 and H-003.
- [ ] Harness `bench:app-startup-cold` repeats the same launches after package content is evicted from the page cache.
- [ ] A V0.5 report exists under `reports/benchmarks/B-016/` and `reports/benchmarks/B-017/` for H-002 meeting the register's publish target.
- [ ] Prose in the report cites B-016 and B-017 and states no public guarantee (I-042).

#### Verification
- Bench: B-016 and B-017 on H-002 and H-003; target per register.
- Integration: CI job on `qemu-virtio-gpu` records first-frame timestamps from the four applications.

#### Evidence
- none

### APP-002 · Build the OS-owned file chooser UI minting UserSelected capabilities
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: STO-034, GFX-040, UIP-024, UIP-025, SEC-007, CAP-029, SDK-017
- Baseline: §9.1, §25, §60
- Risks: R-015
- Threats: T-001, T-002, T-012, T-039
- Invariants: I-016, I-021, I-035

Build the privileged chooser chrome that the OS owns and that applications cannot overlay or spoof (T-012). The chooser holds broad storage authority, the user picks an object, and STO mints `UserSelected<T>` so the caller receives a Capability to exactly that object with no path-based follow-up check (§25, I-035). Image Viewer is the first consumer: it starts with UI and GPU Capabilities only.

<!-- covers: INV-0487 -->

#### Out of scope
UserSelected minting and isolation test (STO-034, STO-007). SDK `files.choose` (SDK-017). Trusted-UI overlay protection (GFX-040). Drag and drop (UIP-032).

#### Acceptance criteria
- [ ] Image Viewer on H-003 opens a user-chosen image via the chooser and holds no Directory Capability for the parent.
- [ ] Enumerate or Open of a sibling of the chosen object returns `Error::Rights` and allocates no handle.
- [ ] A non-trusted application cannot overlay, resize or inject input into the chooser Surface (T-012).
- [ ] The accessibility tree of the chooser is produced by the trusted-UI Component, not by the requesting application (T-039).
- [ ] `os inspect` on the grant records exactly one `Capability<Image, Read>` (or ReadWrite) minted for the chosen object.

#### Verification
- Unit: `apps:tests/chooser/userselected_*` on `qemu-x86_64` and `qemu-virtio-gpu`.
- Integration: Image Viewer chooser scenario on H-003; isolation paired with STO-007.
- Demo: Image Viewer chooses a photo through the OS chooser on H-002; audit log shows one grant.

#### Evidence
- none

### APP-003 · Build the Demo native Text Editor
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: UIP-024, TXT-012, STO-024, APP-008
- Baseline: §11, §41, §60

Ship the V0.5 native Text Editor as a Component graph with declarative UI, accessibility metadata and a scripted acceptance scenario (§60). Kernel-scale editing, diagnostics and IDE features wait for V1. The editor maps immutable Package objects at launch and holds no ambient filesystem namespace (I-021, I-039).

<!-- covers: INV-1189 -->

#### Out of scope
Daily-driving kernel tree (APP-011). IDE strategy (APP-020). Semantic `Editor.open` (APP-015). Spell service (TXT-034).

#### Acceptance criteria
- [ ] The Editor Package launches a window, types, selects, undoes and saves via UserSelected or ApplicationData on H-003.
- [ ] `os inspect` prints role, name and state for every interactive widget in the Editor tree dump.
- [ ] The Editor Component graph matches APP-008: no private widget crate and no ambient File Capability at start.
- [ ] Killing the compositor and rebinding leaves the unsaved buffer intact (GFX-021).
- [ ] Two core team members each use the Text Editor for ten one-hour sessions spread over at least ten working days on H-002, every defect is filed against this task id, and the session log is attached as Evidence (the V0.5-G08 dogfooding count).

#### Verification
- Unit: `apps:tests/editor/demo_*` on `qemu-virtio-gpu`.
- Integration: scripted Editor scenario on H-003; tree dump consumed by ACC-003.
- Demo: Text Editor on the V0.5 four-application desktop on H-002.

#### Evidence
- none

### APP-004 · Build the Demo native Terminal on the system text stack
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: APP-009, APP-008, UIP-024, UIP-023, TXT-009, UIP-001, SDK-023, SDK-025
- Baseline: §11, §41, §56.5, §60

Ship the V0.5 demo Terminal that renders through the system text stack and hosts Linux-personality CLI tools inside a native window via the PTY bridge, without giving the Terminal ambient filesystem or network authority. Tabs, splits, OSC 8, Sixel and Kitty graphics wait for V1 daily driving.

<!-- covers: INV-1107, INV-1187 -->

#### Out of scope
Daily-driving feature bar (APP-016). Terminal-session authority Decision (LNX-022). Glyph atlas service (TXT-020). POSIX as a native API (LNX).

#### Acceptance criteria
- [ ] The Terminal Package opens a window and displays a grid of glyphs through TXT-009 on H-003.
- [ ] A Linux-personality CLI tool runs inside the Terminal window through the PTY bridge and the Terminal Component still holds no ambient File or network Capability.
- [ ] `os inspect` prints role, name and state for every interactive widget in the Terminal tree dump.
- [ ] The Terminal scripted acceptance scenario passes on H-003.
- [ ] Two core team members each use the Terminal for ten one-hour sessions spread over at least ten working days on H-002, every defect is filed against this task id, and the session log is attached as Evidence (the V0.5-G08 dogfooding count).

#### Verification
- Unit: `apps:tests/terminal/demo_*` on `qemu-virtio-gpu`.
- Integration: PTY-hosted CLI scenario on H-003; tree dump consumed by ACC-003.
- Demo: Terminal on the V0.5 four-application desktop on H-002.

#### Evidence
- none

### APP-005 · Build the Demo Image Viewer with an isolated decoder Component
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: APP-002, STO-034, GFX-026, UIP-025, SDK-013, STO-007
- Baseline: §9.1, §11, §60
- Threats: T-001, T-038
- Invariants: I-021, I-029

Ship the V0.5 Image Viewer that starts with UI and GPU Capabilities only, opens a UserSelected image through the OS chooser, and decodes in an isolated Component with no network, no arbitrary filesystem and no process enumeration (§11, T-038). This is the PhotoEditor-shaped proof that cheap isolation is the default.

<!-- covers: INV-1190 -->

#### Out of scope
Chooser chrome (APP-002). Decoder library (SDK-013). GPU grant at launch (GFX-026). First-party document viewer Decision (APP-051).

#### Acceptance criteria
- [ ] At launch the Viewer Capability table contains UI and RenderQueue only; `os inspect` shows no File, Directory or network handle.
- [ ] Opening a UserSelected image decodes in a child Component whose Capabilities are Input of image bytes and Output of a bitmap only.
- [ ] A crafted image that panics the decoder terminates only the decoder Component; the Viewer window stays up and shows a typed decode error.
- [ ] The Viewer cannot enumerate the directory that contained the chosen image (STO-007).
- [ ] Two core team members each use the Image Viewer for ten one-hour sessions spread over at least ten working days on H-002, every defect is filed against this task id, and the session log is attached as Evidence (the V0.5-G08 dogfooding count).

#### Verification
- Unit: `apps:tests/viewer/demo_*` on `qemu-virtio-gpu`.
- Integration: chooser-plus-decode scenario on H-003; decoder panic isolation on H-001.
- Fuzz: `apps:fuzz/viewer_decode` malformed images without Viewer abort.
- Demo: Image Viewer chooses a photo through the OS chooser on H-002.

#### Evidence
- none

### APP-006 · Build the privileged File Browser that hands out storage capabilities
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: UIP-024
- Baseline: §11, §25, §60
- Threats: T-001, T-002
- Invariants: I-016, I-021, I-035

Ship the V0.5 native File Browser as a privileged holder of user storage authority that lists Collections the user may see and hands Capabilities to other applications, rather than exposing a universal filesystem namespace (§25, §60). Drag-and-drop transfer waits for the V1 UIP protocol; V0.5 hands authority through open and the chooser.

<!-- covers: INV-0494, INV-1188 -->

#### Out of scope
Chooser chrome (APP-002). Drag and drop (UIP-032). Foreign-format open Decision (APP-052). Personality path views (LNX, WIN).

#### Acceptance criteria
- [ ] The File Browser lists a user Collection and opens a File by minting `Capability<File>` to the target application with no path-keyed check.
- [ ] A freshly started File Browser child preview Component holds only the Capability it was handed and cannot enumerate siblings.
- [ ] `os inspect` on a handed File shows the Browser as granter and the receiving application as holder.
- [ ] `os inspect` prints role, name and state for every interactive widget in the File Browser tree dump.
- [ ] Two core team members each use the File Browser for ten one-hour sessions spread over at least ten working days on H-002, every defect is filed against this task id, and the session log is attached as Evidence (the V0.5-G08 dogfooding count).

#### Verification
- Unit: `apps:tests/files/demo_*` on `qemu-virtio-gpu`.
- Integration: hand-Capability-to-Editor scenario on H-003.
- Demo: File Browser on the V0.5 four-application desktop on H-002.

#### Evidence
- none

### APP-007 · Build a minimal session host that launches the four Demo applications
- Type: build
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: APP-004, APP-006, APP-003, APP-005, GFX-010, UIP-016, SVC-015, PKG-031
- Baseline: §32, §60
- Risks: R-015

Boot a native session that starts the compositor and places Terminal, File Browser, Text Editor and Image Viewer on a desktop. This is a session host, not a polished shell: no panel, launcher search, lock or greeter (R-015). Privileged shell Surface roles come from UIP; ordinary `Capability<UI>` cannot mint them.

<!-- covers: INV-1187, INV-1188, INV-1189, INV-1190 -->

#### Out of scope
Thin panel (APP-017). Launcher (APP-013). Lock (APP-033). Greeter (APP-030). Compositor (GFX).

#### Acceptance criteria
- [ ] Cold boot of the V0.5 generation on H-003 presents the four demo applications as native windows with input.
- [ ] The session host holds a shell Capability; a demo application requesting a panel role receives `Error::Rights` and allocates no Surface.
- [ ] Restarting the compositor rebinds the four windows without the session host exiting.
- [ ] No panel, lock or greeter chrome is present in the V0.5 session.

#### Verification
- Integration: four-application session on H-003 and H-002.
- Demo: cold boot to a native desktop showing the four applications on H-002.

#### Evidence
- none

### APP-008 · Document first-party applications as Component graphs
- Type: docs
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: CMP-024
- Baseline: §11, §60
- Invariants: I-029

Write the V0.5 application-graph guide so Terminal, File Browser, Text Editor and Image Viewer are declared as Component graphs with attenuated per-child authority (isolated decoder, chooser, PTY helper) instead of monoliths (§11). The four demo tasks treat this document as the shape they implement.

Required by the APP scope: "First-party applications are Component graphs: isolated decoders, choosers and PTY helpers hold only the authority they need".

#### Out of scope
Graph instantiation runtime (CMP-024). Capability-set lint (CAP-024). SDK samples (SDK-021).

#### Acceptance criteria
- [ ] A committed guide names the Component graph, Inputs, Outputs and Capability set for Terminal, File Browser, Text Editor and Image Viewer.
- [ ] Each graph lists at least one isolated child (decoder, PTY or chooser helper) with a Capability set that is a strict subset of the parent's.
- [ ] The four demo application manifests are reviewed against the guide on the pull request that lands each app.

#### Verification
- Review: CMP and APP reviewers sign off that the four V0.5 manifests match the guide.

#### Evidence
- none

### APP-009 · Prototype a PTY bridge that scopes Linux Personality tools to Terminal
- Type: spike
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: LNX-005, UIP-023, SDK-025
- Baseline: §3, §9.1, §46, §60
- Threats: T-001, T-011
- Invariants: I-006, I-021, I-072

Prototype hosting Linux-personality CLI tools inside a native Terminal window without granting the Terminal ambient filesystem or network authority. The V0.5 demo Terminal consumes the report; LNX-022 decides the V1 policy. Native software still never sees POSIX; the PTY exists only as a bridge into the personality (I-006).

<!-- covers: GAP-0310 -->

#### Out of scope
Demo Terminal chrome (APP-004). V1 terminal-session authority Decision (LNX-022). POSIX as a native API (LNX).

#### Acceptance criteria
- [ ] A report at `reports/spikes/APP-009.md` compares at least two bridge shapes (personality PTY Component versus in-process emulator talking to a scoped personality) on H-003.
- [ ] Each prototype runs a Linux-personality CLI tool inside a native window while the Terminal Capability table still contains no ambient File or network handle.
- [ ] The report records which option leaks personality authority into the Terminal Component and which option is ruled out.
- [ ] Native crates in the prototype contain no POSIX PTY types in public APIs.

#### Verification
- Report: which bridge shape preserves I-072; what Capability the PTY helper holds; how resize, exit and signals stay inside the personality; what V1 LNX-022 must decide.
- Review: LNX reviewer sign-off recorded on the pull request that lands the report.

#### Evidence
- none

### APP-010 · Integrate Linux Personality Git, browser and IDE into launcher and chooser
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: APP-013, APP-002, APP-019, APP-020, LNX-038, UIP-028
- Baseline: §49, §56.5, §61
- Risks: R-025
- Invariants: I-096

Wire Git, the chosen browser and the chosen IDE from the Linux personality into the native launcher, clipboard and file chooser so V1 daily driving does not require a separate desktop (§61, §49). Availability of those programs is LNX; this task is the APP chrome half.

<!-- covers: INV-1103, INV-1105, INV-1198, INV-1199 -->

#### Out of scope
L2 corpus pass rates (LNX-056). Browser and IDE strategy (APP-019, APP-020). Portals (LNX-036). Native IDE (not promised for 1.0).

#### Acceptance criteria
- [ ] Git, the Decision's browser and the Decision's IDE appear as launchable entries in the V1 launcher on H-004.
- [ ] Copy-paste of plain text and of an image round-trips between the personality browser and the native Text Editor in both directions, verified by a scripted scenario on H-004.
- [ ] The native chooser can grant the personality IDE exactly one UserSelected file, and the IDE cannot enumerate the parent directory.
- [ ] Native software still has no POSIX path API; personality entries are imported, not in-process.

#### Verification
- Integration: launcher, clipboard and chooser scenarios for Git, browser and IDE on H-004 and `qemu-virtio-gpu`.
- Compat: L2 integration check consumed by LNX-038.
- Review: LNX reviewer confirms personality programs are Components with the default Capability bundle.

#### Evidence
- none

### APP-011 · Extend Text Editor for kernel and platform development on the OS
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: APP-003, APP-020, APP-002, TXT-020, SDK-040, ACC-015
- Baseline: §41, §61

Extend the native Text Editor so it can open the OS tree, search and jump to diagnostics for kernel and platform development on the OS (§61). Linux-personality IDEs remain LNX; this task is the native editor half of self-hosting. The IDE strategy Decision records when a personality IDE is the daily driver instead.

<!-- covers: INV-1197 -->

#### Out of scope
IDE strategy (APP-020). Personality IDEs (LNX-032). Semantic `Editor.open` (APP-015). Debugger attach (SDK-038).

#### Acceptance criteria
- [ ] The Editor opens a UserSelected directory tree of the OS checkout and searches file contents without a home-namespace Capability.
- [ ] Diagnostics from `os test` or the SDK LSP adapter are jumpable from the Editor on H-004.
- [ ] Unlabelled interactive widgets fail `sdk:tests/toolkit/a11y_metadata_*` via ACC-015.
- [ ] Warm startup of the Editor is measured by B-016; this task states no number.

#### Verification
- Unit: `apps:tests/editor/daily_*` on `qemu-virtio-gpu`.
- Integration: open OS tree, search, jump-to-diagnostic on H-004.
- Bench: B-016 Editor warm path on H-002; target per register.

#### Evidence
- none

### APP-012 · Build a grant log viewer for Capability audit events
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: CAP-030, CAP-001, APP-007, OBS-006, SEC-006
- Baseline: §7, §9, §61
- Threats: T-001

Ship a read-only grant log viewer so V1 daily driving can inspect Capability grants, derivations, revocations and denials without a full permissions Settings surface. V1 scope excludes permissions UI beyond this log; revocation, expiry and timeline wait for APP-029.

Required by V1-G02 (Core team daily-drives the OS): daily driving needs a way to inspect grants and denials while the permissions Settings surface waits for V2.

#### Out of scope
Grant Settings surface (APP-029). Audit export (CAP-030). Grant taxonomy Decision (SEC-007). Tamper-evident store (OBS-044).

#### Acceptance criteria
- [ ] The viewer lists grant, derive, revoke and deny events for the current session from CAP-030 on H-003.
- [ ] A Component without a log-read Capability receives `Error::Rights` and sees no events.
- [ ] The viewer cannot revoke a grant; revoke controls are absent.
- [ ] `os inspect` on the viewer Component shows no storage or network Capability.

#### Verification
- Unit: `apps:tests/grants/log_viewer_*` on `qemu-x86_64`.
- Integration: denial from the Image Viewer isolation test appears in the log on H-003.

#### Evidence
- none

### APP-013 · Build the native launcher importing Linux Personality .desktop entries
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: APP-007, LNX-052, PKG-031, UIP-016, CAP-025
- Baseline: §47, §49, §61
- Invariants: I-096

Build a native launcher that starts native Packages and imports Linux-personality `.desktop` entries so Git and a browser appear as launchable native-chrome applications (§47, §61). Search and Personality icon polish wait for V2. The launcher is a privileged shell Surface; ordinary applications cannot mint that role.

<!-- covers: INV-0890 -->

#### Out of scope
Search and PE/.desktop icon polish (APP-032). Thin panel (APP-017). XDG view (LNX-052). Default-application registry (SVC-019).

#### Acceptance criteria
- [ ] Native Terminal, File Browser, Text Editor and Image Viewer are launchable from the V1 launcher on H-003.
- [ ] Installed Linux-personality packages with `.desktop` entries appear in the same launcher list without a separate desktop.
- [ ] A Component holding only `Capability<UI>` that requests the launcher shell role receives `Error::Rights`.
- [ ] Launch attaches only the Package's granted Capability set (CAP-025).

#### Verification
- Unit: `apps:tests/launcher/v1_*` on `qemu-virtio-gpu`.
- Integration: native plus one L1/L2 `.desktop` launch on H-004.

#### Evidence
- none

### APP-014 · Build the Notify service with Capability-gated posting and history
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: IPC-012, SVC-015, GFX-009, SEM-007, CAP-025, UIP-024
- Baseline: §9.1, §12, §32, §42, §61
- Threats: T-001
- Invariants: I-021

Ship a typed Notify Interface that requires `Capability<Notifications>` to post, binds actions to Semantic interfaces rather than callbacks, supports do-not-disturb and persistent history, and survives compositor restart (§32). Linux and Windows notification bridges wait for V2. Native software never sees a D-Bus notification bus.

<!-- covers: GAP-0272 -->

#### Out of scope
Linux `org.freedesktop.Notifications` (LNX-065). Windows toast (WIN-047). Lock-screen notification privacy (APP-033). Shell popover chrome beyond a V1 history list (APP-036).

#### Acceptance criteria
- [ ] Posting without `Capability<Notifications>` returns `Error::Rights` and allocates no notification object.
- [ ] A notification action invokes a named Semantic interface through SEM-007 and does not synthesise pointer or key input.
- [ ] Do-not-disturb suppresses display while history still records the post.
- [ ] Killing the compositor and rebinding redisplays persistent history without dropping entries (GFX-009).
- [ ] `os inspect` lists live notifications, holders and the do-not-disturb flag.

#### Verification
- Unit: `apps:tests/notify/service_*` on `qemu-x86_64` and `qemu-virtio-gpu`.
- Integration: compositor kill-rebind with history on H-003.
- Review: SEM reviewer confirms actions are Semantic interface invocations.

#### Evidence
- none

### APP-015 · Expose Semantic interfaces for Terminal.run and Editor.open
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: SEM-006, SEM-007, SEM-004, SEM-002, APP-016, APP-011
- Baseline: §42, §61
- Invariants: I-023, I-051

Implement `Terminal.run` and `Editor.open` on the native Terminal and Text Editor so an automation script can invoke them through typed Interfaces without GUI input (§42, §61). SEM owns registry and IDL; APP implements the two proof Interfaces on the applications. No AI broker work is in this task (I-051).

#### Out of scope
Registry service (SEM-007). Automation-rule editor (APP-059). AI broker (SEM-010). Input synthesis (SEM-002).

#### Acceptance criteria
- [ ] A script invokes `Terminal.run` and `Editor.open` through the registry and completes without generating pointer or key events.
- [ ] A caller without the Interface Capability receives `Error::Rights` and allocates no handle.
- [ ] The scenario is a permanent CI test on `qemu-x86_64`.
- [ ] `os inspect` shows the two Interfaces registered to the Terminal and Editor Components.

#### Verification
- Unit: `apps:tests/semantic/terminal_run_editor_open_*` on `qemu-x86_64`.
- Integration: typed-automation demo path consumed by SEM-008 on H-003.
- Review: SEM reviewer confirms no input synthesis.

#### Evidence
- none

### APP-016 · Extend Terminal for daily development with tabs, splits and graphics
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: APP-004, APP-009, LNX-022, TXT-020
- Baseline: §41, §61
- Risks: R-026

Extend Terminal to the V1 daily-driving feature bar: GPU text via the system atlas, tabs and splits, OSC 8 hyperlinks, Sixel/Kitty graphics, true color, and the PTY bridge under the accepted terminal-session authority Decision (§61). This is the self-hosted terminal gate, not the V0.5 demo.

<!-- covers: GAP-0310, INV-1196 -->

#### Out of scope
V0.5 demo Terminal (APP-004). Authority Decision (LNX-022). Semantic `Terminal.run` (APP-015). POSIX job control as a native API (SDK-025).

#### Acceptance criteria
- [ ] Tabs and splits host separate PTY helper Components; closing a tab cancels only that helper's TaskGroup.
- [ ] OSC 8 hyperlinks, Sixel or Kitty graphics and true color pass the V1 Terminal scripted scenario on H-004.
- [ ] The Terminal Component still holds no ambient File or network Capability after hosting a Linux-personality toolchain command.
- [ ] Warm startup is measured by B-016; this task states no number.

#### Verification
- Unit: `apps:tests/terminal/daily_*` on `qemu-virtio-gpu`.
- Integration: tabs, splits, OSC 8 and PTY toolchain scenario on H-004.
- Bench: B-016 Terminal warm path on H-002; target per register.

#### Evidence
- none

### APP-017 · Build a thin panel with a running-application list for daily driving
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: APP-007, APP-013, UIP-016, LNX-038
- Baseline: §41, §61

Ship a running-application list sufficient for V1 daily work. Status indicators, quick settings and Personality taskbar polish wait for the V2 panel. The thin panel uses the privileged shell Surface role from UIP.

<!-- covers: INV-0885 -->

#### Out of scope
Polished panel and taskbar (APP-043). Window switcher (APP-018). Quick settings (APP-036).

#### Acceptance criteria
- [ ] The thin panel lists every running native application window and activates one on H-003.
- [ ] Linux-personality windows that LNX-038 maps also appear in the list.
- [ ] A Component without the shell Capability cannot mint the panel Surface (`Error::Rights`).
- [ ] No network, battery, volume or tray indicators are present in the V1 panel.

#### Verification
- Unit: `apps:tests/shell/thin_panel_*` on `qemu-virtio-gpu`.
- Integration: running-app list with native and one personality window on H-004.

#### Evidence
- none

### APP-018 · Build a window switcher usable for daily development
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: APP-017, GFX-010, UIP-013
- Baseline: §41, §61

Ship a window switcher so V1 L2 integration can include the task switcher and developers can move between Terminal, Editor and personality windows. Virtual workspaces and overview wait for V2.

<!-- covers: INV-0885 -->

#### Out of scope
Workspaces and overview (APP-050). Panel taskbar (APP-043). Wayland switcher protocols (LNX).

#### Acceptance criteria
- [ ] A named shortcut cycles windows and focuses the selected Surface on H-003.
- [ ] Native and Linux-personality windows appear in one switcher list.
- [ ] The switcher is keyboard operable and emits role, name and state for each row.
- [ ] Closing the switcher without a selection leaves focus unchanged.

#### Verification
- Unit: `apps:tests/shell/window_switcher_*` on `qemu-virtio-gpu`.
- Integration: L2 task-switcher check consumed by LNX-038 on H-004.

#### Evidence
- none

### APP-019 · Decide browser strategy for 1.0 and the native WebView Component
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: APP-021, LNX-031, Q-044
- Baseline: §56.5, §61
- Decision: D-0022
- Risks: R-025

§56.5 makes a browser existential and V1 daily driving needs one. This Decision records whether 1.0 ships Chromium, Firefox or both through the Linux personality, whether a native port is in scope (it is not promised), and whether applications get a sandboxed native WebView Component. The spike's WebView prototype is an input; LNX productizes the personality browser.

<!-- covers: GAP-0307, INV-1104 -->

#### Out of scope
WebView implementation (APP-049). Personality GPU browser (LNX-031). URL dispatch and downloads (APP-023). Widevine (MED, LNX).

#### Acceptance criteria
- [ ] Options evaluated include Chromium through the Linux personality, Firefox through the Linux personality, both personality browsers, and a native port, each with 1.0 promise versus non-promise.
- [ ] The Decision states whether in-app web content is a Capability-scoped native WebView or personality-only, citing APP-021.
- [ ] The Decision names the default browser for V1 daily driving.
- [ ] LNX and APP leads record Review sign-off on the pull request.

#### Verification
- Review: LNX and APP leads sign off on the pull request that accepts the Decision file.

#### Evidence
- none

### APP-020 · Decide IDE strategy for 1.0 and the criteria for a native port
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: APP-003, LNX-032, SDK-040, Q-045
- Baseline: §56.5, §61
- Decision: D-0025
- Risks: R-025, R-026

Record whether V1 self-hosting uses VS Code or JetBrains through the Linux personality, the native Text Editor, or both, and the criteria for later investing in a native IDE. 1.0 does not promise a native IDE. V1 still needs a usable IDE for kernel and platform development on the OS (§61).

<!-- covers: INV-1106 -->

#### Out of scope
Native Editor daily features (APP-011). Personality runtimes (LNX-032). Debugger (SDK-038).

#### Acceptance criteria
- [ ] Options evaluated include VS Code through the Linux personality, a JetBrains IDE through the Linux personality, the native Text Editor as daily driver, and a native IDE port, with 1.0 promise versus non-promise.
- [ ] The Decision lists switch criteria that would justify a native IDE after 1.0 (self-hosting gaps, Capability model, not taste).
- [ ] The Decision names the V1 daily-driving IDE path used by APP-010.
- [ ] SDK and LNX leads record Review sign-off on the pull request.

#### Verification
- Review: SDK and LNX leads sign off on the pull request that accepts the Decision file.

#### Evidence
- none

### APP-021 · Prototype a sandboxed native WebView Component for in-app web content
- Type: spike
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: CMP-024, GFX-026, CAP-025, LNX-031
- Baseline: §11, §56.5
- Threats: T-001, T-011
- Invariants: I-021, I-029

Prototype in-app web content as a Capability-scoped Component so APP-019 can choose a native WebView versus personality-only web content. The prototype is not a native browser. Engine reuse through the Linux personality is in bounds as a measured option; native software still never sees POSIX APIs.

<!-- covers: GAP-0307 -->

#### Out of scope
Browser strategy Decision (APP-019). WebView ship (APP-049). Default browser (LNX). Widevine (MED, LNX).

#### Acceptance criteria
- [ ] A report at `reports/spikes/APP-021.md` compares at least two options (personality web content only; sandboxed native WebView Component, with or without a personality engine helper) on H-003.
- [ ] The WebView prototype starts with UI and GPU Capabilities only and cannot open network or File handles unless granted.
- [ ] The report records attack surface, GPU path and whether a native WebView can be a child Component of a native app.
- [ ] No native public API in the prototype is POSIX-shaped.

#### Verification
- Report: which option is ruled out; what Capability set a WebView child holds; how navigation, cookies and downloads are granted or denied; impact on APP-019.
- Review: SEC and LNX reviewers sign off on the pull request that lands the report.

#### Evidence
- none

### APP-022 · Publish notification, lock and launcher latencies against desktop baselines
- Type: benchmark
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: APP-033, APP-014, APP-032, APP-030, BEN-035
- Baseline: §54, §62
- Benchmarks: B-016, B-033, B-045
- Invariants: I-061

Instrument the V2 shell so B-016 (warm startup of every shipped native application), B-033 (unlock-to-interactive-desktop) and B-045 (notification post-to-display, lock-to-unlock, launcher) can publish against desktop baselines. APP owns shell instrumentation; BEN owns methodology.

#### Out of scope
Harness methodology (BEN-038, BEN-035). Input-to-photon (B-020). Camera cold start (MED-012).

#### Acceptance criteria
- [ ] Shell probes emit timestamps for notification post-to-display, lock-to-unlock, greeter unlock-to-desktop and launcher-to-first-frame on H-002, H-004 and H-005.
- [ ] Warm startup runs for every shipped native application under B-016 on those machines.
- [ ] Reports exist under `reports/benchmarks/B-016/`, `reports/benchmarks/B-033/` and `reports/benchmarks/B-045/` for H-002 meeting the V2 publish targets.
- [ ] Report prose cites B-IDs and states no superiority claim without the table.

#### Verification
- Bench: B-016, B-033 and B-045 on H-002, H-004 and H-005; target per register.
- Integration: probes consumed by BEN-038.

#### Evidence
- none

### APP-023 · Integrate the default browser with URL dispatch, downloads and chooser
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: APP-019, APP-026, APP-002, LNX-036, LNX-080, MED-013, AUD-003
- Baseline: §9.1, §49, §62
- Threats: T-014, T-025
- Invariants: I-021, I-096

Integrate the Decision's default browser with native default-application URL dispatch, downloads into capability-scoped locations, WebRTC camera and microphone through native Capabilities, and the file-chooser portal (§49). Widevine CDM stays MED/LNX. A personality browser without these bridges is the first public-alpha report.

<!-- covers: GAP-0308 -->

#### Out of scope
Browser strategy (APP-019). Widevine (MED, LNX). Native WebView (APP-049). Camera service (MED-013).

#### Acceptance criteria
- [ ] A native link open dispatches to the default browser via SVC-019 and grants a Capability to the URL target only.
- [ ] A download lands as a UserSelected or ApplicationData object; the browser cannot enumerate the downloads directory.
- [ ] Camera or microphone use without the corresponding Capability produces a denial and no frames or samples (T-014).
- [ ] The native chooser serves the personality file-chooser portal for one file without directory authority.

#### Verification
- Integration: URL dispatch, download, chooser and camera-deny scenarios on H-004.
- Compat: L3 integration scoring consumed by LNX-079.

#### Evidence
- none

### APP-024 · Build clipboard history with sensitive-item exclusion
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: UIP-028, APP-043
- Baseline: §9.1, §41
- Threats: T-001, T-032
- Invariants: I-021

Ship clipboard history as a privileged shell feature. Sources mark entries ephemeral so password managers and secure fields are excluded from history and sync (GAP-0275). UIP owns clipboard Capability policy and paste-gesture gating; X11 primary selection never enters this history (T-032).

<!-- covers: GAP-0275 -->

#### Out of scope
Clipboard Capability policy (UIP-003, UIP-028). Primary selection (LNX-020). OLE (WIN-021).

#### Acceptance criteria
- [ ] The shell history UI lists recent clipboard entries only for a Component that holds `Capability<ClipboardRead>`.
- [ ] An entry marked ephemeral is absent from history and from any sync path immediately after paste.
- [ ] A Component without clipboard rights cannot read history (`Error::Rights`).
- [ ] Primary-selection payloads never appear in native history (T-032).

#### Verification
- Unit: `apps:tests/shell/clipboard_history_*` on `qemu-virtio-gpu`.
- Integration: secure-field exclusion and personality copy on H-004.

#### Evidence
- none

### APP-025 · Build the trusted consent prompt UI for Capability requests
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: SEC-043, SEC-044, GFX-040, APP-029, CAP-037
- Baseline: §9, §9.1, §63
- Risks: R-041
- Threats: T-012, T-014
- Invariants: I-021, I-060

Ship the trusted consent chrome for camera, microphone, files, network and screen-capture requests so deny does not crash the application and prompts cannot be spoofed (T-012). SEC owns prompt policy and grant recording; APP owns the compositor-protected UI. Prompts must not become a deny-list UX (R-041, I-060).

#### Out of scope
Prompt policy Decision (SEC-043). Prompt runtime (SEC-044). Trusted-UI protection (GFX-040). Grant Settings (APP-029).

#### Acceptance criteria
- [ ] A camera, microphone, files, network or screen-capture request from a native or personality application shows the trusted prompt on H-004.
- [ ] Deny returns a typed error to the caller, allocates no handle and leaves the application running.
- [ ] A non-trusted application cannot overlay or inject input into the prompt Surface (T-012).
- [ ] Accept records a grant in the persistent store keyed on package identity plus publisher.
- [ ] The prompt tree is produced by the trusted-UI Component (T-039).

#### Verification
- Unit: `apps:tests/consent/prompt_*` on `qemu-virtio-gpu`.
- Integration: deny-without-crash for camera in a Linux-personality video app on H-004.
- Demo: permissions prompt for camera denied without crash on H-004.

#### Evidence
- none

### APP-026 · Build the default-application picker over the system registry
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: SVC-017, SVC-019, APP-040, PKG-056
- Baseline: §11, §25, §62

Build the Settings picker that binds a typed kind or URL scheme to a handler Package through SVC's default-application registry. Opening mints a Capability to just that object; the picker does not grant a directory or a wildcard handler right.

<!-- covers: GAP-0279, INV-0832 -->

#### Out of scope
Headless registry (SVC-019). Manifest handler declarations (PKG-056). Linux xdg-open (LNX). URL dispatch in the browser (APP-023).

#### Acceptance criteria
- [ ] The Default apps panel lists handler candidates per typed kind and records the user choice in the settings store.
- [ ] Opening an object of that kind launches the chosen handler with a Capability to only that object.
- [ ] Clearing the default returns the next open to an ask-once prompt rather than an ambient last-used path.
- [ ] The panel Component holds only settings and registry Capabilities, not storage namespace.

#### Verification
- Unit: `apps:tests/settings/default_apps_*` on `qemu-virtio-gpu`.
- Integration: set default, open object, clear default on H-003.

#### Evidence
- none

### APP-027 · Build desktop search as a Background-intent Component
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: APP-055, STO-065, APP-013, SCH-010, CAP-017
- Baseline: §9.1, §22, §25
- Threats: T-001
- Invariants: I-016, I-021

Ship desktop search as a Background-intent Component over storage Capabilities so launcher search does not require ambient home access (§9.1). STO owns the consent-scoped index provider; APP owns the search Component and launcher results. Persistent background execution requires `Capability<BackgroundExecution>`.

<!-- covers: EXTRA-044 -->

#### Out of scope
Store-side index (STO-065). Scope spike (APP-055). Launcher icon polish (APP-032). Semantic Workspace.search (SEM-035).

#### Acceptance criteria
- [ ] The search Component runs with Background intent and `Capability<BackgroundExecution>`; missing that Capability returns `Error::Rights` and the indexer does not start.
- [ ] Results include only objects from Collections the user granted; a sibling outside those Collections never appears.
- [ ] The search Component holds no home-directory Capability (`os inspect`).
- [ ] Launcher queries return indexed native files and imported personality names without a second desktop.

#### Verification
- Unit: `apps:tests/search/desktop_*` on `qemu-x86_64`.
- Integration: grant-scoped index versus denied sibling on H-004.
- Review: STO reviewer confirms queries hit only the consent-scoped provider.

#### Evidence
- none

### APP-028 · Build the emoji picker as an input-method protocol client
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: TXT-022, TXT-010, APP-044, UIP-045
- Baseline: §41
- Invariants: I-048

Ship an emoji and special-character picker invoked by a global named shortcut that commits text through the input-method protocol (S-016). TXT owns the protocol and engines; this is the system picker client, not a second IME.

<!-- covers: GAP-0256 -->

#### Out of scope
IME protocol and engines (TXT-029, TXT-028). Shortcut service (UIP-045). Toolkit preedit (TXT-022).

#### Acceptance criteria
- [ ] A user-bound named shortcut opens the picker over the focused text field on H-003.
- [ ] Choosing a character commits through the IME protocol; the picker never injects raw key events into another application.
- [ ] The picker holds `Capability<TextInputFocus>` for the focused field only and receives no global key stream.
- [ ] Closing the picker without a commit leaves the field unchanged.

#### Verification
- Unit: `apps:tests/shell/emoji_picker_*` on `qemu-virtio-gpu`.
- Integration: commit into native Editor and a Linux-personality text field on H-004.

#### Evidence
- none

### APP-029 · Build the Settings grant Surface with revocation, expiry and timeline
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: APP-012, APP-040, SEC-045, CAP-037, CAP-044, OBS-043
- Baseline: §7, §9, §62
- Risks: R-041, R-078
- Threats: T-005
- Invariants: I-060

Ship permissions UI v1: every persistent Capability grant per application, with revocation, expiry and a usage timeline from the audit trail (§7). Policy is SEC; this is the user-facing surface. Revocation takes effect on the next Operation for camera, microphone, files, network and screen capture.

<!-- covers: GAP-0229 -->

#### Out of scope
One-time grants and v2 history (SEC-062). Immediate-revocation matrix (CAP-044). Prompt chrome (APP-025). SEC UI-level proof (SEC-036).

#### Acceptance criteria
- [ ] Settings lists every persistent grant per application including type, rights, expiry and last use from OBS-043.
- [ ] Revoking camera, microphone, files, network or screen capture makes the next Operation fail with `Error::Rights` and no leftover mapping.
- [ ] Setting an expiry records the deadline; after expiry the grant is absent from the active list.
- [ ] The panel Component holds query and revoke Capabilities only, not the objects those grants name.

#### Verification
- Unit: `apps:tests/settings/grants_v1_*` on `qemu-virtio-gpu`.
- Integration: revoke matrix consumed by CAP-044 on H-004.
- Demo: permissions list and revoke on H-004.

#### Evidence
- none

### APP-030 · Build the login greeter UI over the identity service
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: SEC-020, SEC-028, GFX-045, APP-007, SVC-027
- Baseline: §51, §62
- Threats: T-010, T-012

Ship compositor-integrated greeter chrome over the SEC identity service for password login at V2. Fingerprint and FIDO2 authenticators are consumed when SEC ships them; this chrome must not hard-code a single factor. The greeter is trusted UI; applications cannot overlay it.

<!-- covers: GAP-0283, GAP-0215 -->

#### Out of scope
Identity service (SEC-020). Fingerprint and FIDO2 (SEC-057, SEC-056). Session lock chrome (APP-033). Multi-user switcher (APP-063).

#### Acceptance criteria
- [ ] The greeter accepts a password via SEC-020 and starts the session supervision tree on H-004.
- [ ] A failed authentication leaves the greeter visible and records an audit event; no session Capabilities are minted.
- [ ] A non-trusted Surface cannot render above the greeter (GFX-045).
- [ ] Optional auto-login, when enabled in settings, starts the single configured session without showing the password field.

#### Verification
- Unit: `apps:tests/session/greeter_*` on `qemu-virtio-gpu`.
- Integration: password login and failed-auth on H-004.
- Bench: B-032 greeter-ready path is consumed by BEN; this task states no number.

#### Evidence
- none

### APP-031 · Build persistent in-use indicators for camera, microphone and screen share
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: APP-043, MED-013, GFX-061, AUD-003
- Baseline: §9.1, §40
- Threats: T-013, T-014
- Invariants: I-085

Ship persistent shell indicators while camera, microphone or screen share is granted and in use. APP owns the chrome; GFX/SEC own screen-capture Capability; MED owns the camera service; AUD owns microphone grants.

#### Out of scope
Capture Capability (GFX-061). Camera service (MED-013). Screenshot UI (APP-038). Microphone grant test (AUD-003).

#### Acceptance criteria
- [ ] While a camera session is live the shell shows a persistent camera indicator that cannot be hidden by the capturing application.
- [ ] Microphone and screen-share sessions each show a distinct persistent indicator.
- [ ] Ending the session or revoking the Capability clears the indicator within one Operation.
- [ ] An application without the corresponding Capability cannot cause the indicator to light (`Error::Rights`, no session).

#### Verification
- Unit: `apps:tests/shell/in_use_indicators_*` on `qemu-virtio-gpu`.
- Integration: camera, microphone and screen-share indicator on H-004.
- Demo: screen sharing shows a persistent indicator on H-002.

#### Evidence
- none

### APP-032 · Extend the launcher with search and Personality icons
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: APP-013, APP-027
- Baseline: §49, §62
- Invariants: I-096

Extend the V1 launcher with search over the desktop-search Component and with extracted Personality icons (PE resources, `.desktop` icons) so Windows and Linux applications appear in the native launcher with no separate desktop (§49).

<!-- covers: INV-0922, GAP-0282 -->

#### Out of scope
Search indexer (APP-027). PE icon extraction internals (WIN-024). `.desktop` import (APP-013). Tray (APP-046).

#### Acceptance criteria
- [ ] Launcher search returns native Packages and indexed UserSelected names from APP-027 on H-004.
- [ ] A Windows-personality application appears with its PE icon and a Linux-personality application with its `.desktop` icon in the same list.
- [ ] There is no separate compatibility desktop or wizard for ordinary launches.
- [ ] A query that matches no granted object returns an empty list, not a home listing.

#### Verification
- Unit: `apps:tests/launcher/search_icons_*` on `qemu-virtio-gpu`.
- Integration: native, Linux and Windows entries on H-002.
- Demo: double-click `.exe` path consumes launcher identity from WIN-048.

#### Evidence
- none

### APP-033 · Build the compositor-enforced lock screen UI
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: GFX-045, GFX-055, SEC-028, SEC-020, APP-014, PWR-011
- Baseline: §32, §40, §62
- Threats: T-009, T-012
- Invariants: I-075

Ship lock-screen chrome for idle-triggered lock, lock-before-suspend and notification privacy, authenticating through the SEC identity service. GFX owns compositor enforcement that no application Surface renders above the lock and that a compositor crash while locked restarts locked (I-075). Fingerprint and smartcard are authenticators SEC may add; the chrome consumes the identity Interface rather than embedding a factor.

<!-- covers: GAP-0284 -->

#### Out of scope
Lock mode enforcement (GFX-045). Crash-while-locked test (GFX-055). Identity (SEC-020). Greeter (APP-030). Fingerprint (SEC-057).

#### Acceptance criteria
- [ ] Idle timeout and lid-close lock show the lock Surface and inhibit input to application Surfaces on H-004.
- [ ] Notifications on the lock screen show sender or app identity only when the user has enabled lock-screen details; default is hidden body.
- [ ] Successful authentication reaches an interactive desktop; failed authentication leaves the lock Surface up.
- [ ] A compositor kill while locked restarts locked (GFX-055).
- [ ] Lock-before-suspend is the default; a Component without `Capability<InhibitSuspend>` cannot prevent it.

#### Verification
- Unit: `apps:tests/session/lock_*` on `qemu-virtio-gpu`.
- Integration: idle, lid-close and crash-while-locked on H-004 and H-005.
- Bench: B-033 unlock-to-desktop on H-004; target per register.
- Demo: lock and unlock on a V2 laptop day.

#### Evidence
- none

### APP-034 · Build the media session shell controls bound to media keys
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: APP-043, MED-022, AUD-015, UIP-045, HW-037
- Baseline: §42, §62

Surface play, pause, next and now-playing metadata in the shell, bound to hardware media keys and Bluetooth AVRCP. MED owns the playback session object; AUD owns audio focus; HW/AUD own AVRCP; LNX owns any MPRIS bridge. This is a Semantic interface used by the shell, not GUI scraping.

<!-- covers: GAP-0304 -->

#### Out of scope
Playback session object (MED-022). Audio focus (AUD-015). MPRIS bridge (LNX). AVRCP host (HW-037). First-party media player (APP-061).

#### Acceptance criteria
- [ ] A native playback session advertises now-playing on the panel and lock screen on H-004.
- [ ] Named media-key actions invoke play, pause and next on the foreground session without granting apps a global key grab.
- [ ] A Bluetooth headset AVRCP play/pause reaches the same session.
- [ ] An application that does not expose the playback-session Interface does not appear in the shell controls.

#### Verification
- Unit: `apps:tests/shell/media_session_*` on `qemu-virtio-gpu`.
- Integration: media keys and AVRCP on H-004.
- Demo: headset media keys on the V2 laptop day.

#### Evidence
- none

### APP-035 · Build power and battery UX mapped to ResourceDomain energy policy
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: PWR-010, PWR-019, PWR-016, PWR-021, APP-040, APP-036
- Baseline: §22, §23, §62

Ship battery estimates, power profiles mapped to ResourceDomain energy policy, per-application energy attribution, low-battery actions, lid-close behavior and charge limits as Settings and quick-settings chrome. PWR owns the mechanisms; users judge laptop power management through this UI.

<!-- covers: GAP-0312 -->

#### Out of scope
Battery reporting (PWR-010). Charge profiles (PWR-019). Energy attribution meters (PWR-016). Lid policy (PWR-021). Indicator glyph only (APP-036).

#### Acceptance criteria
- [ ] Settings shows charge fraction, rate, AC/DC and remaining-time estimate from PWR on H-004 and H-005.
- [ ] Performance, balanced and power-saver profiles select the corresponding ResourceDomain energy policy and are visible in `os inspect`.
- [ ] Per-application energy attribution lists ResourceDomains with a non-zero sample after a scripted workload.
- [ ] Low-battery actions (notify, dim, suspend) and charge start/stop limits are configurable and persist in the settings store.
- [ ] Lid-close behavior (blank versus suspend), with and without external display) is configurable and matches PWR-021.

#### Verification
- Unit: `apps:tests/settings/power_*` on `qemu-x86_64`.
- Integration: profile switch, low-battery action and lid-close on H-004 and H-005.
- Manual: remaining-time display during a B-031 discharge; this task states no accuracy number (PWR-023).

#### Evidence
- none

### APP-036 · Build quick settings and status indicators
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: APP-043, NET-021, PWR-010, AUD-009, HW-035, TXT-029
- Baseline: §62
- Invariants: I-021

Ship quick settings and status indicators for network, battery, volume, Bluetooth and input method as V2 shell chrome. Each indicator talks to the owning service over a typed Interface; the panel does not receive adapter-wide or ambient device authority.

<!-- covers: GAP-0282 -->

#### Out of scope
Device Settings panels (APP-041). OSD overlays (APP-042). Bluetooth pairing chooser (HW-036). Wi-Fi stack (NET-021).

#### Acceptance criteria
- [ ] Indicators for network, battery, volume, Bluetooth and active IME render on the panel on H-004 and H-005.
- [ ] Quick settings can toggle Wi-Fi radio, set volume, open Bluetooth pairing and switch IME without opening full Settings.
- [ ] The quick-settings Component holds only the indicator Interfaces; `os inspect` shows no adapter-wide scan Capability.
- [ ] Clicking an indicator opens the corresponding Settings panel when installed.

#### Verification
- Unit: `apps:tests/shell/quick_settings_*` on `qemu-virtio-gpu`.
- Integration: laptop-day indicators on H-004 and H-005.
- Demo: open lid, Wi-Fi and Bluetooth indicators on the V2 laptop day.

#### Evidence
- none

### APP-037 · Build the safe-mode recovery shell with settings, logs and restore
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: SVC-036, SVC-035, PKG-060, APP-040
- Baseline: §32, §62
- Risks: R-045
- Invariants: I-077

Ship the recovery chrome SVC starts when the shell or compositor exhausts its restart budget: settings reset, log bundle export and `os restore` to a previous generation. This is not the separately booted recovery environment (INS).

#### Out of scope
Safe-mode session policy (SVC-036, SVC-035). Booted recovery generation (INS-013). Text console last resort (SVC). Crash-report client (INS-020).

#### Acceptance criteria
- [ ] When SVC starts the safe-mode session the recovery chrome offers settings reset, log export and `os restore` on H-003.
- [ ] Restore invokes PKG-060 and does not mutate the running generation in place.
- [ ] Log export produces a bundle without disk keys or unlocked secrets (I-077).
- [ ] The recovery chrome runs on a minimal compositor configuration and does not load third-party Packages.

#### Verification
- Integration: fault-injected shell restart-budget exhaustion on H-002 and H-003.
- Review: SVC reviewer confirms the chrome is the session SVC starts, not a second supervisor.

#### Evidence
- none

### APP-038 · Build screenshot and screen-recording UI over screen-capture Capability
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: GFX-061, GFX-086, GFX-084, MED-026, APP-031, APP-044
- Baseline: §9.1, §40
- Threats: T-013
- Invariants: I-085

Ship user-facing screenshot and screen-recording chrome over S-034. GFX owns the Capability and OS-owned capture tools; MED owns encode; APP owns region selection, save-via-chooser and the recording indicator. An application without the Capability receives a denied or black surface.

#### Out of scope
Capture Capability (GFX-061). OS-owned capture tools (GFX-086, GFX-084). Encode (MED-026). Screen-share picker for portals (GFX-085).

#### Acceptance criteria
- [ ] A named shortcut opens region, window or display capture and saves via the OS chooser as UserSelected.
- [ ] Starting a recording shows the persistent in-use indicator and requires `Capability<ScreenCapture>`.
- [ ] An application without that Capability that requests frames receives a denied or black surface (T-013).
- [ ] Encoded recordings are written only to the UserSelected or ApplicationData object the user picked.

#### Verification
- Unit: `apps:tests/shell/screenshot_*` on `qemu-virtio-gpu`.
- Integration: region capture and recording-with-indicator on H-002.
- Demo: screen sharing with persistent indicator on H-002.

#### Evidence
- none

### APP-039 · Build logout, shutdown, inhibitors, auto-login and session restore
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: APP-056, APP-030, PWR-012, PWR-013, SEC-028, PKG-022
- Baseline: §21, §31, §62
- Risks: R-084

Ship logout, shutdown and reboot with inhibitors, optional auto-login, fast user switching chrome and session restore of windows from system history to the extent APP-056 and PKG-069 accept. Identity and FDE are SEC; generation restore UX is INS.

<!-- covers: GAP-0283 -->

#### Out of scope
Session restore spike (APP-056). Application-state restore Decision (PKG-069). Inhibit Capabilities (PWR-012). Generation restore Settings (INS-014). Multi-user stores (APP-063).

#### Acceptance criteria
- [ ] Logout, shutdown and reboot offer an inhibitor list from `Capability<InhibitIdle>` and `Capability<InhibitSuspend>` holders and proceed or cancel on the user's choice.
- [ ] Auto-login, when enabled, starts the configured session from the greeter path without a password prompt.
- [ ] Fast user switching to a second logged-in identity is either implemented against SEC-028 or explicitly absent with the V3 multi-user task named.
- [ ] If the restore spike accepts window restore, a logged-out session reopens the previous application windows from history; if it rejects, Settings shows restore as unavailable and no criterion claims it.

#### Verification
- Unit: `apps:tests/session/flows_*` on `qemu-virtio-gpu`.
- Integration: shutdown with inhibitor, auto-login and restore-or-unavailable on H-004.
- Review: PKG reviewer confirms restore behavior matches the accepted application-state Decision.

#### Evidence
- none

### APP-040 · Build the Settings application chassis as per-panel Components
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: SVC-013, SVC-028, UIP-044, CMP-024, APP-017
- Baseline: §11, §62
- Invariants: I-029

Ship Settings as a chassis of per-panel Components each holding only that panel's Capabilities (§11). This task is chassis plus appearance, keyboard, region, date/time and a stub About that shows generation identity. Device panels, grant surface, default apps, shortcuts and the notices-filled About are sibling tasks.

<!-- covers: GAP-0279 -->

#### Out of scope
Device panels (APP-041). Grants (APP-029). Default apps (APP-026). Shortcuts (APP-044). Notices About (APP-057). Appearance model (UIP-044).

#### Acceptance criteria
- [ ] Settings launches as a graph of panel Components; `os inspect` shows each panel's Capability set is a subset of the chassis.
- [ ] Appearance, keyboard, region and date/time panels read and write typed settings through SVC-028.
- [ ] A panel cannot read another panel's settings scope (`Error::Rights`).
- [ ] About shows the running SystemGeneration identity from PKG history.

#### Verification
- Unit: `apps:tests/settings/chassis_*` on `qemu-virtio-gpu`.
- Integration: appearance and region change round-trip on H-003.
- Demo: Settings in the 40-scenario UX script.

#### Evidence
- none

### APP-041 · Build Settings panels for displays, network, Bluetooth, sound and printers
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: APP-040, GFX-065, NET-021, HW-036, AUD-009
- Baseline: §11, §62

Ship Settings panels that consume GFX, NET, HW and AUD device services as Components with only that panel's Capabilities. Printers show an empty or discovery-pending state until HW-071; this V2 task does not implement print jobs.

<!-- covers: GAP-0279 -->

#### Out of scope
Print dialog and IPP discovery (APP-064, HW-071). Display arrangement mechanics (GFX-065). Wi-Fi stack (NET). Bluetooth host (HW). Mixer (AUD-009).

#### Acceptance criteria
- [ ] Displays panel shows arrangement and per-display scale from GFX-065 and persists a change on H-002.
- [ ] Network panel lists Wi-Fi networks and connects via NET-021 on H-004 without a network Capability leaking to other panels.
- [ ] Bluetooth panel opens the pairing chooser and lists paired devices from HW-036.
- [ ] Sound panel sets per-application volume via AUD-009.
- [ ] Printers panel lists zero or more discovered printers and does not mint `Capability<PrintJob>` (that is APP-064).

#### Verification
- Unit: `apps:tests/settings/devices_*` on `qemu-virtio-gpu`.
- Integration: display, Wi-Fi, Bluetooth and volume on H-004 and H-002.
- Demo: external display and Bluetooth in the V2 laptop day.

#### Evidence
- none

### APP-042 · Build on-screen display overlays for volume, brightness and lock
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: APP-043, AUD-009, PWR-018, UIP-045, APP-033
- Baseline: §41, §62
- Threats: T-012

Show OSD overlays for hardware volume, brightness and lock keys without granting applications a global overlay Capability. The OSD is a privileged shell Surface.

#### Out of scope
Mixer (AUD-009). Brightness policy (PWR-018). Named-action service (UIP-045). Lock chrome (APP-033).

#### Acceptance criteria
- [ ] Volume and brightness keys show a transient OSD on H-004 without any application holding an overlay Capability.
- [ ] An application requesting a global overlay role receives `Error::Rights` and no Surface.
- [ ] Lock key shows the lock Surface via APP-033, not an application-drawn fake lock (T-012).
- [ ] OSDs emit accessibility role, name and state.

#### Verification
- Unit: `apps:tests/shell/osd_*` on `qemu-virtio-gpu`.
- Integration: volume and brightness keys on H-004.

#### Evidence
- none

### APP-043 · Build the desktop panel and taskbar with Personality integration
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: APP-017, UIP-016
- Baseline: §49, §62
- Invariants: I-096

Replace the V1 thin panel with the V2 taskbar so Windows and Linux applications integrate with native chrome and no separate desktop (§49, §62). Sibling tasks own launcher search, workspaces, quick settings and tray.

<!-- covers: GAP-0282, INV-0921 -->

#### Out of scope
Launcher search (APP-032). Workspaces (APP-050). Quick settings (APP-036). Tray host (APP-046). Personality window identity (WIN-024, LNX-079).

#### Acceptance criteria
- [ ] Native, Linux-personality and Windows-personality windows appear as taskbar entries on H-002.
- [ ] Activating an entry focuses the corresponding Surface; closing it from the taskbar requests a close on that Surface.
- [ ] There is no separate compatibility desktop for ordinary Windows or Linux applications.
- [ ] The panel holds the shell Capability; ordinary apps cannot mint it.

#### Verification
- Unit: `apps:tests/shell/panel_*` on `qemu-virtio-gpu`.
- Integration: native plus L3 and W1 windows on H-002.
- Demo: Windows `.exe` appears in the taskbar (WIN-048).

#### Evidence
- none

### APP-044 · Build the Settings UI for named global shortcut bindings
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: UIP-045, APP-040
- Baseline: §41
- Threats: T-012

Ship the Settings binding UI for named global actions (media, screenshot, picker, lock, launcher). Applications request named actions and never receive raw global key grabs (UIP GAP-0286, T-012).

#### Out of scope
Shortcut service (UIP-045). Screenshot chrome (APP-038). Emoji picker (APP-028). Media keys (APP-034).

#### Acceptance criteria
- [ ] Settings lists named actions and records user key bindings in the settings store.
- [ ] Binding a key to screenshot, launcher or lock invokes those shell actions on H-003.
- [ ] An application requesting a raw global key grab receives `Error::Rights` and no grab.
- [ ] Conflicting bindings are rejected with a typed error and the previous binding remains.

#### Verification
- Unit: `apps:tests/settings/shortcuts_*` on `qemu-virtio-gpu`.
- Integration: bind screenshot and launcher on H-004.

#### Evidence
- none

### APP-045 · Build the store client with install-time Capability review
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: REL-012, REL-013, PKG-064, PKG-075, CAP-043, APP-025
- Baseline: §9, §28, §62
- Risks: R-030, R-078
- Threats: T-006, T-033
- Invariants: I-036

Ship the native repository client: browse, install, update and remove from REL metadata, showing declared Capabilities so the user can deny optional ones at install while the application still launches degraded (§62). REL/PKG own repository and packages; APP owns the client chrome.

#### Out of scope
Repository server and signing (REL-007). Metadata API (REL-013). Optional Capability schema (PKG-075). Publisher continuity (CAP-043).

#### Acceptance criteria
- [ ] The client lists Packages from REL-013 with publisher, declared Capabilities and SBOM identity.
- [ ] Denying an optional Capability at install still launches the application in the degraded mode the Package declares.
- [ ] Denying a required Capability aborts install and writes nothing outside the content store.
- [ ] Update of a Package with the same publisher keeps grants; a publisher change revokes them (T-033, CAP-043).

#### Verification
- Unit: `apps:tests/store/client_*` on `qemu-x86_64`.
- Integration: install, optional-deny degraded launch, publisher-change revoke on H-002.
- Demo: store install with Capability review on H-002.

#### Evidence
- none

### APP-046 · Build the status-tray host according to the tray Decision
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: APP-054, APP-043, LNX-079, WIN-024
- Baseline: §49, §62
- Invariants: I-096

Implement the accepted tray Decision. Linux and Windows tray clients either appear in the native host or are explicitly denied; there is no silent ignore. Native software never speaks StatusNotifierItem; that protocol exists only inside personalities if the Decision accepts it.

#### Out of scope
Tray policy Decision (APP-054). Personality tray clients (LNX, WIN). Quick settings (APP-036).

#### Acceptance criteria
- [ ] If the Decision accepts StatusNotifierItem compatibility, a Linux tray client appears in the native host on H-002; if it rejects, that client is denied with a typed error and a documented Settings explanation.
- [ ] A Windows tray or balloon client is either hosted or denied per the same Decision.
- [ ] The host Component is privileged shell chrome; an ordinary native app cannot inject a tray icon without a tray Capability.
- [ ] Denied clients do not crash (typed error, application continues).

#### Verification
- Unit: `apps:tests/shell/tray_host_*` on `qemu-virtio-gpu`.
- Integration: one Linux and one Windows tray client on H-002 per the accepted Decision.
- Review: LNX and WIN reviewers confirm host-or-deny matches the Decision.

#### Evidence
- none

### APP-047 · Build the running-application update prompt for Generation switches
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: PKG-066, APP-014
- Baseline: §30, §34
- Risks: R-079
- Threats: T-034
- Invariants: I-022

When a Package is replaced by a new SystemGeneration, show restart or defer so no running Component observes a mixed-version tree (T-034). PKG owns the no-mixed-version test; APP owns the prompt.

#### Out of scope
Running-app update mechanics (PKG-066). Updater client (INS-045). Store client (APP-045).

#### Acceptance criteria
- [ ] After a generation that replaces a running Package is activated, each affected application shows restart or defer.
- [ ] Defer keeps the old objects mapped; `os inspect` shows no mixed-version Package tree for that Component.
- [ ] Restart launches the Component from the new Package objects only.
- [ ] The prompt is Notify chrome with a Semantic restart action, not a callback into the old binary.

#### Verification
- Unit: `apps:tests/shell/update_prompt_*` on `qemu-x86_64`.
- Integration: generation switch with a running Editor on H-003; mixed-version test owned by PKG-066.

#### Evidence
- none

### APP-048 · Run the 40-scenario desktop UX script on the three target machines
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: APP-043, APP-032, APP-014, APP-040, APP-033, APP-050, APP-036, APP-041, GFX-059
- Baseline: §62
- Invariants: I-096

Author and land the 40-scenario desktop UX script (launch, switch, notify, settings, lock, unlock, external display, scaling) and run it on H-002, H-004 and H-005 with zero P0/P1. BLD owns harness plumbing; APP owns scenario content. This verifies the polished shell (§62).

<!-- covers: INV-1213 -->

#### Out of scope
Harness plumbing (BLD-056). Compositor multi-monitor (GFX). Individual shell features (sibling APP tasks).

#### Acceptance criteria
- [ ] Forty named scenarios covering launch, switch, notify, settings, lock, unlock, external display and scaling exist as harness inputs.
- [ ] The script passes on H-002, H-004 and H-005 with zero open P0/P1 against those scenarios.
- [ ] A failed scenario names the machine, the step and the inspecting `os inspect` snippet.
- [ ] The script is retained as a V2 gate run, not a one-shot manual list.

#### Verification
- Integration: BLD-056 on H-002, H-004, H-005 and `qemu-virtio-gpu`.
- Demo: polished shell on the three target machines.

#### Evidence
- none

### APP-049 · Build the sandboxed native WebView Component
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: APP-019, APP-021, CMP-024, CAP-025, GFX-026
- Baseline: §11, §56.5
- Threats: T-001, T-011
- Invariants: I-021, I-029

Ship in-app web content as a Capability-scoped Component after the V1 browser Decision and spike. This is not a native browser. If the Decision rejects a native WebView, this task is dropped with that reason rather than shipping a personality-only shim under a native name.

<!-- covers: GAP-0307 -->

#### Out of scope
Browser strategy (APP-019). Spike (APP-021). Default browser (LNX). URL dispatch (APP-023).

#### Acceptance criteria
- [ ] If the Decision accepts a native WebView, a parent application embeds it as a child Component with UI and GPU Capabilities only unless network or File grants are added.
- [ ] Navigation to a URL without a network Capability returns `Error::Rights` and does not open a connection.
- [ ] A decoder or renderer panic aborts only the WebView child.
- [ ] If the Decision rejects a native WebView, the task is `dropped` with `Dropped because: descoped:` naming the Decision, and no native WebView crate is published.

#### Verification
- Unit: `apps:tests/webview/sandbox_*` on `qemu-virtio-gpu` (or drop review if descoped).
- Integration: embed-in-Settings-help or equivalent on H-003.
- Review: SEC reviewer confirms the Capability set matches the Decision.

#### Evidence
- none

### APP-050 · Build virtual workspaces, overview and window layout
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: APP-018, APP-043, GFX-019
- Baseline: §41, §62

Ship virtual workspaces, overview and window layout for the V2 shell so the 40-scenario UX script can launch and switch on three machines. Window switcher already exists at V1; this task adds workspaces and overview. Restore of layout across compositor restart follows GFX-019.

<!-- covers: GAP-0282 -->

#### Out of scope
V1 switcher (APP-018). Compositor checkpoint Decision (GFX-019). UX script (APP-048).

#### Acceptance criteria
- [ ] At least two workspaces exist; moving a window and switching workspace focuses the expected Surface on H-003.
- [ ] Overview shows live window thumbnails and activates one.
- [ ] Layout after compositor rebind matches the checkpoint Decision (geometry and workspace).
- [ ] The named workspace-switch shortcut moves focus to the target workspace with no pointer input and emits the workspace-changed accessibility state event.

#### Verification
- Unit: `apps:tests/shell/workspaces_*` on `qemu-virtio-gpu`.
- Integration: launch-and-switch scenarios on H-002, H-004 and H-005.

#### Evidence
- none

### APP-051 · Decide the first-party application set native versus bundled at 1.0
- Type: adr
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: APP-005, APP-006
- Baseline: §56.5, §60, §62, §63
- Decision: D-0023

Decide which of document/PDF viewer, media player, archive manager, calculator, camera, disk utility and system monitor are native Components versus bundled personality applications at 1.0. Each native choice is a multi-month commitment; the native subset must stay splittable into one task per application. Media codecs stay MED; camera capture client may already exist as MED-011.

<!-- covers: GAP-0318, INV-1112 -->

#### Out of scope
Native implementation (APP-061). PDF library (APP-053). Camera service (MED-013). Codecs (MED).

#### Acceptance criteria
- [ ] Options evaluated include all-native, all-bundled-compat, and a mixed set, with a per-application table of native versus LNX/WIN bundle.
- [ ] The Decision names the V3 native subset so APP-061 can split mechanically if more than two applications are native.
- [ ] Media player native versus bundle is explicit; codecs remain MED either way.
- [ ] APP and LNX leads record Review sign-off on the pull request.

#### Verification
- Review: APP and LNX leads sign off on the pull request that accepts the Decision file.

#### Evidence
- none

### APP-052 · Decide which foreign Package formats map to Personality launches
- Type: adr
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: APP-006, LNX-017, WIN-001
- Baseline: §49
- Decision: D-0024
- Invariants: I-096

Record how `.exe`, `.msi`, `.AppImage`, `.deb`, `.rpm` and Flatpak map to Personality launches from File Browser so double-click feels native with no compatibility wizard (§49). WIN-027 implements the `.exe` path; this Decision is the format map.

<!-- covers: INV-0930 -->

#### Out of scope
`.exe` launch implementation (WIN-027). Linux packaging formats (LNX-017). File Browser chrome (APP-006). Installer-to-launcher registration (WIN-061).

#### Acceptance criteria
- [ ] Options evaluated include open-selected-formats-only, open-all-listed-formats via personality, and ask-once-per-format, each with a per-format table (`.exe`, `.msi`, `.AppImage`, `.deb`, `.rpm`, Flatpak).
- [ ] The Decision states that ordinary opens show no compatibility wizard and no separate desktop.
- [ ] Formats explicitly out of 1.0 are named rather than implied.
- [ ] WIN and LNX leads record Review sign-off on the pull request.

#### Verification
- Review: WIN and LNX leads sign off on the pull request that accepts the Decision file.

#### Evidence
- none

### APP-053 · Decide the shared PDF renderer for viewer, thumbnails and print preview
- Type: adr
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: APP-051
- Baseline: §11, §51
- Decision: D-0026
- Threats: T-038
- Invariants: I-029, I-067, I-068

Choose one shared sandboxed PDF renderer (pdfium, poppler or a Rust renderer) for document viewer, File Browser thumbnails and print preview so four copies of the same attack surface are not shipped. The renderer is an isolated Component (T-038). License allowlist is GOV; this Decision does not put GPLv2-incompatible code in the kernel.

<!-- covers: GAP-0270 -->

#### Out of scope
Document viewer implementation (APP-061). Print dialog (APP-064). File Browser chrome (APP-006). License policy (GOV).

#### Acceptance criteria
- [ ] Options evaluated include pdfium, poppler and a Rust renderer, each with license, isolation and thumbnail/print-preview sharing consequences.
- [ ] The Decision requires the renderer to run as an isolated Component with no network and no arbitrary filesystem.
- [ ] Userspace allowlist (I-068) is cited; AGPL/SSPL/BUSL options are rejected or confined per GOV.
- [ ] APP and GOV leads record Review sign-off on the pull request.

#### Verification
- Review: APP and GOV leads sign off on the pull request that accepts the Decision file.

#### Evidence
- none

### APP-054 · Decide status-tray policy: StatusNotifierItem compatibility versus none
- Type: adr
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: APP-017, LNX-052
- Baseline: §49, §62
- Decision: D-0027
- Invariants: I-048, I-096

Decide whether the V2 shell hosts StatusNotifierItem (and Windows tray) clients in a native host or refuses them with a documented denial. The Decision precedes APP-046. Native software never speaks those protocols.

<!-- covers: GAP-0282 -->

#### Out of scope
Tray host (APP-046). Quick settings (APP-036). Personality implementations (LNX, WIN).

#### Acceptance criteria
- [ ] Options evaluated include native host for StatusNotifierItem plus Windows tray, native host for one personality only, and no tray with explicit deny plus Settings copy.
- [ ] The Decision states how a denied client fails (typed error, no crash) and how users discover the policy.
- [ ] Native public APIs named in the Decision contain no StatusNotifierItem or TrayWnd types.
- [ ] LNX and WIN leads record Review sign-off on the pull request.

#### Verification
- Review: LNX and WIN leads sign off on the pull request that accepts the Decision file.

#### Evidence
- none

### APP-055 · Prototype Capability-scoped desktop search without ambient home access
- Type: spike
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: STO-034, CAP-017, SCH-010
- Baseline: §9.1, §22, §25
- Threats: T-001
- Invariants: I-016, I-021

Measure what EXTRA-044 may index before APP-027 ships. An indexer with ambient home-directory access would violate §9. STO-065 is the store-side counterpart. The report bounds Collections, change-notification and Background intent.

Required by the APP scope: "desktop search"; the report bounds what APP-027 may index.

#### Out of scope
Search Component (APP-027). Store index (STO-065). Semantic Workspace.search (SEM).

#### Acceptance criteria
- [ ] A report at `reports/spikes/APP-055.md` compares at least two index scopes (granted Collections only; granted Collections plus opt-in roots) on H-003.
- [ ] Each prototype runs as Background intent and holds no home-directory Capability.
- [ ] The report records what leaks if change-notification is too broad and which option is ruled out.
- [ ] Native APIs in the prototype are not path-keyed authority checks (I-016).

#### Verification
- Report: what EXTRA-044 may index; how revocation drops documents; cost of reindex on Collection grant; relation to STO-065.
- Review: STO and CAP reviewers sign off on the pull request that lands the report.

#### Evidence
- none

### APP-056 · Prototype session restore of application windows from system history
- Type: spike
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: PKG-079, PKG-022, Q-056
- Baseline: §31, §62
- Risks: R-084

Prototype reopening application windows from system history before APP-039 commits to restore. PKG-069 may park application-state restore as a 1.0 non-goal; this spike supplies window-level evidence so V2 does not promise infeasible restore (R-084, Q-056).

<!-- covers: GAP-0283 -->

#### Out of scope
Session flows (APP-039). Application-state Decision (PKG-069). Generation restore (INS-014).

#### Acceptance criteria
- [ ] A report at `reports/spikes/APP-056.md` compares at least two options (reopen Surfaces from compositor checkpoint; reopen from application-cooperative state; no restore in 1.0) on H-003.
- [ ] The report states whether Terminal and Editor windows can reopen without a mixed-version Package tree (T-034).
- [ ] If restore is infeasible, the report recommends Settings show unavailable rather than a partial promise.
- [ ] Q-056 is referenced; the report does not answer PKG's application-state question for PKG.

#### Verification
- Report: which window restore option survives compositor restart and logout; what APP-039 may claim; interaction with Q-056 and PKG-069.
- Review: PKG reviewer sign-off recorded on the pull request that lands the report.

#### Evidence
- none

### APP-057 · Build the Settings About panel with notices and license texts
- Type: build
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: APP-040, REL-049, INS-037
- Baseline: §63
- Invariants: I-067, I-068

Fill the Settings About panel with the notices bundle REL publishes and the GPL written offer INS requires in installer and Settings. APP hosts the panel; REL owns the bundle; INS owns installer rendering.

<!-- covers: GAP-0279 -->

#### Out of scope
Notices bundle (REL-049). Installer About (INS-037). Generation identity stub (APP-040).

#### Acceptance criteria
- [ ] About lists license texts for first-party and bundled third-party Packages from the generation notices bundle.
- [ ] The GPL written offer is visible as a distinct entry and can be saved via the OS chooser.
- [ ] The panel holds no network Capability; notices are PackageData from the running generation.
- [ ] A missing notices bundle fails CI rather than shipping an empty legal panel.

#### Verification
- Unit: `apps:tests/settings/about_*` on `qemu-x86_64`.
- Review: GOV reviewer confirms GPL offer and notices match REL-049.

#### Evidence
- none

### APP-058 · Polish the desktop shell and applications for external alpha users
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: APP-048, APP-061, SEC-062, APP-063
- Baseline: §63

Close remaining P0/P1 in shell and first-party applications on Tier 1 so the V3 polished desktop is ready for enthusiasts, developers and researchers (§63). This is not a new feature task; it is the alpha-quality pass over shipped APP chrome.

<!-- covers: INV-1247 -->

#### Out of scope
New shell features (APP-065). Localisation (APP-066). A11y script (APP-067). Installer (INS).

#### Acceptance criteria
- [ ] Zero open P0/P1 against shell and first-party applications on the V3 Tier 1 set (H-002, H-004, H-005 and the V3 added machines in hardware scope).
- [ ] The V2 40-scenario UX script still passes on those machines.
- [ ] Public-alpha bugs filed against APP chrome during the milestone are either fixed or documented as known with a task ID.
- [ ] No new shell or first-party feature lands in this task's pull requests.

#### Verification
- Integration: 40-scenario script plus first-party smoke on V3 Tier 1.
- Review: APP lead sign-off that P0/P1 are closed on the alpha image.

#### Evidence
- none

### APP-059 · Build the automation-rule editor over Semantic interfaces
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: SEM-031, SEM-029, SEM-018, SEM-025, APP-040
- Baseline: §42, §45
- Risks: R-043
- Threats: T-018
- Invariants: I-023, I-051

Ship the user-facing automation editor so people create and inspect rules over Semantic interfaces without GUI scraping (§45). Depends on the done SEM registry and rules engine. No AI broker is included (I-051, R-043). Background rules still require `Capability<BackgroundExecution>`.

<!-- covers: INV-0846 -->

#### Out of scope
Rule engine (SEM-031). Registry (SEM-029). AI broker (SEM-010). Example rules (SEM-019).

#### Acceptance criteria
- [ ] A user can create, list, disable and delete a rule that binds a typed event to a typed action through the registry.
- [ ] The editor cannot save a rule that synthesises pointer or key input (SEM-002).
- [ ] A background rule without `Capability<BackgroundExecution>` fails save with `Error::Rights`.
- [ ] `os inspect` shows the rule graph the editor created (SEM-025).
- [ ] No AI-broker crate is introduced by this task.

#### Verification
- Unit: `apps:tests/automation/editor_*` on `qemu-x86_64`.
- Integration: create the download-extract example rule without GUI input on H-002.
- Review: SEM reviewer confirms the editor is a client of the rule engine, not a second engine.

#### Evidence
- none

### APP-060 · Build the backup and restore UI over snapshot-based user-data backup
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: STO-071, APP-040, APP-002
- Baseline: §27, §31, §63

Ship schedule, progress and restore chrome for user-data backup to external or network targets. STO owns encrypted snapshot backup; APP owns the Settings UI. Secrets stay in SEC; this chrome does not display raw keys.

#### Out of scope
Backup service (STO-071). Secrets (SEC). Network stack (NET). Machine-to-machine restore (INS-050).

#### Acceptance criteria
- [ ] Settings can schedule a backup to a UserSelected external or network Collection and show progress.
- [ ] Restore from a listed snapshot writes user data through STO-071 and records a history event.
- [ ] The UI Component holds backup and chooser Capabilities only; `os inspect` shows no disk-encryption keys.
- [ ] Cancelling an in-flight backup cancels the Operation and leaves the previous snapshot intact.

#### Verification
- Unit: `apps:tests/settings/backup_*` on `qemu-x86_64`.
- Integration: schedule, progress, restore on H-002.
- Bench: B-036 snapshot path is STO/BEN; this task states no number.

#### Evidence
- none

### APP-061 · Implement the native first-party applications selected by the set Decision
- Type: build
- Milestone: V3
- Status: todo
- Size: L
- Owner: none
- Depends on: APP-051, APP-053, CMP-024, UIP-024, APP-008, ACC-015
- Baseline: §11, §56.5, §63
- Threats: T-038
- Invariants: I-029

Implement the native subset named by APP-051 (document/PDF viewer, media player, archive manager, calculator, camera, disk utility, system monitor). Compat-bundled remainder is LNX/WIN. If the accepted set names more than two native applications, split into one task per application before leaving todo. Each native app is a Component graph with isolated decoders where parsing untrusted bytes (PDF, media, archives).

<!-- covers: GAP-0318, INV-1112 -->

#### Out of scope
Set Decision (APP-051). PDF library choice (APP-053). Camera service (MED-013). Codecs (MED). Personality bundles (LNX, WIN).

#### Acceptance criteria
- [ ] Every application the Decision marks native ships as a Package with a Component graph documented against APP-008.
- [ ] PDF, media or archive parsing runs in an isolated child Component with no network and no arbitrary filesystem (T-038).
- [ ] Applications the Decision marks bundled are absent from the native first-party set and are launched via personality entries instead.
- [ ] Each native app passes a scripted acceptance scenario on H-002 and `qemu-virtio-gpu`.
- [ ] Unlabelled interactive widgets fail CI (ACC-015).

#### Verification
- Unit: `apps:tests/first_party/*` on `qemu-virtio-gpu`.
- Integration: scripted scenarios for each native app on H-002.
- Review: APP lead confirms the shipped set matches the Decision table.

#### Evidence
- none

### APP-062 · Build the migration assistant UI over the installer import engine
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: INS-049, INS-033, INS-036, APP-040, APP-002
- Baseline: §63

Ship migration-assistant chrome for choosing sources, destinations, progress and undo. INS owns Windows-profile and Linux-home import engines and the pre-import snapshot; APP owns the UI. Each imported item lands as a Collection or ApplicationData object, not a path grant.

#### Out of scope
Windows import engine (INS-049). Linux home import (INS-033). Undo snapshot (INS-036). Browser profile secrets (INS-018, SEC).

#### Acceptance criteria
- [ ] The assistant lists INS-discovered Windows and Linux sources and lets the user pick per-item destinations.
- [ ] Progress and resumability are visible; cancel stops the import Operation.
- [ ] Undo invokes the pre-import snapshot via INS-036 and is listed in `os history`.
- [ ] The UI Component holds chooser and import-client Capabilities only.

#### Verification
- Unit: `apps:tests/migration/assistant_*` on `qemu-x86_64`.
- Integration: Windows-profile and Linux-home import with undo on H-002.
- Review: INS reviewer confirms the UI does not reimplement import.

#### Evidence
- none

### APP-063 · Build multi-user session switching with separate Capability stores
- Type: build
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: SEC-060, SEC-064, APP-030, CAP-049
- Baseline: §63
- Threats: T-026
- Invariants: I-092

Ship switcher and greeter chrome for a second user so two users have separate sessions, Capability stores and encrypted data, and switching preserves state (§63, T-026). SEC owns session objects; APP owns chrome. Enterprise directory and guest sessions remain 1.0 non-goals (I-092).

<!-- covers: GAP-0283 -->

#### Out of scope
Multi-user sessions (SEC-060). State preservation (SEC-064). Per-user grant stores (CAP-049). MDM and guest (I-092).

#### Acceptance criteria
- [ ] Two users can be logged in; switching via greeter chrome shows the second session's Surfaces and not the first (T-026).
- [ ] `os inspect` on each session shows a distinct Capability store.
- [ ] Switching preserves the first session's windows (SEC-064).
- [ ] Fast user switch does not require a reboot.

#### Verification
- Integration: two-user switch on H-004.
- Review: SEC reviewer confirms chrome consumes session objects rather than minting them.

#### Evidence
- none

### APP-064 · Build the OS-owned print dialog minting per-job print capabilities
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-071, GFX-040, APP-053, APP-002
- Baseline: §9.1, §25, §63
- Threats: T-002, T-012
- Invariants: I-035

Ship an OS-owned print dialog that grants `Capability<PrintJob>` for a single job the way the file chooser grants file authority. HW owns discovery (IPP/eSCL/USB); LNX owns the CUPS socket; APP owns dialog chrome and print preview via the shared PDF renderer.

<!-- covers: GAP-0268 -->

#### Out of scope
Print discovery (HW-071). CUPS bridge (LNX-093). PDF renderer Decision (APP-053). Printers Settings list (APP-041).

#### Acceptance criteria
- [ ] The dialog runs as trusted UI; applications cannot overlay it (T-012).
- [ ] Confirming a job mints `Capability<PrintJob>` for that job only; the application cannot submit a second job on the same handle.
- [ ] Cancel allocates no PrintJob handle.
- [ ] Preview renders through the isolated PDF Component named by APP-053.
- [ ] A Component without print rights that requests the dialog still cannot talk to the printer except through the minted job.

#### Verification
- Unit: `apps:tests/print/dialog_*` on `qemu-x86_64`.
- Integration: one-job grant on a discovered printer on H-002.
- Review: HW reviewer confirms discovery stays in HW-071.

#### Evidence
- none

### APP-065 · Freeze remaining native application and shell features for 1.0
- Type: build
- Milestone: V4
- Status: todo
- Size: S
- Owner: none
- Depends on: ACC-031, APP-066, APP-058
- Baseline: §63, §66
- Risks: R-054

After RC1, every APP feature in the 1.0 definition is done; no new shell or first-party application features land after this freeze. Fixes, localization coverage and a11y holds may continue. This is a process gate over the APP workstream, not an L1 freeze.

#### Out of scope
L1 ABI freeze (ABI). UI protocol lock (UIP-055). 1.0 soak (APP-067). New applications (descoped by this freeze).

#### Acceptance criteria
- [ ] A committed freeze note lists APP deliverables in the 1.0 definition and marks each done or dropped.
- [ ] CI fails a pull request that adds a new first-party application or shell surface after freeze without an accepted superseding Decision.
- [ ] Remaining APP tasks at 1.0 are only APP-067 and APP-068 (plus LATER docs).
- [ ] Review records that no L1 surface was frozen by this task.

#### Verification
- Review: APP and GOV leads sign off on the freeze note pull request.

#### Evidence
- none

### APP-066 · Localize the shell and shipped native applications to ten languages
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: TXT-046, TXT-032, TXT-036, APP-058
- Baseline: §41, §63
- Risks: R-042, R-058

Ship translations for every first-party APP surface so the V4 localization gate can pass. TXT owns the pipeline, catalogs and coverage measurement; APP authors and lands strings for shell and native applications. This task does not restate coverage percentages; those live on TXT-046.

#### Out of scope
Coverage gate (TXT-046). Translation platform (TXT-036). Docs languages (DOC). IME engines (TXT).

#### Acceptance criteria
- [ ] Every APP first-party surface has catalogs consumed by TXT-032.
- [ ] TXT-046 is green for shell and native applications on the RC image.
- [ ] Untranslated strings in frozen APP chrome fail CI rather than shipping English-only.
- [ ] RTL layout of launcher, Settings and Editor is screenshot-reviewed for the RTL language TXT includes.

#### Verification
- Integration: TXT-046 on the V4 RC.
- Review: TXT reviewer confirms APP catalogs are complete inputs to the coverage gate.

#### Evidence
- none

### APP-067 · Hold shell and native application quality through the 1.0 soak
- Type: build
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: APP-065, ACC-031, APP-066, TXT-047, ACC-036
- Baseline: §63
- Risks: R-061, R-063

Hold V4 a11y and localization gates on the release build and keep zero open P0/P1 on Tier 1 for shell and first-party apps through the 1.0 soak. Compatibility integration (launcher, taskbar, notifications, file chooser) still holds (I-096).

#### Out of scope
Feature freeze (APP-065). Localisation re-verify (TXT-047). A11y evidence (ACC-036). Compatibility corpora (LNX, WIN).

#### Acceptance criteria
- [ ] TXT-047 and ACC-036 are green on the 1.0 release candidate.
- [ ] Zero open P0/P1 against APP chrome and first-party apps on every Tier 1 machine at soak end.
- [ ] Launcher, taskbar, notifications and file chooser integration still hold for passing Linux and Windows corpus entries.
- [ ] No new APP feature lands during the soak.

#### Verification
- Integration: V4 scripts re-run on the 1.0 RC across Tier 1.
- Review: APP lead sign-off recorded on the soak report.

#### Evidence
- none

### APP-068 · Publish 1.0 non-goals for accounts, parental controls and dictation
- Type: docs
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: APP-065, SEM-043
- Baseline: §42, §45, §63
- Invariants: I-092

Publish that online-account integration (calendar, contacts, mail sync), parental controls and speech dictation are out of 1.0, with only Semantic-interface hooks reserved so those products can appear later without GUI scraping. Enterprise directory, MDM and guest sessions remain non-goals (I-092).

<!-- covers: GAP-0321 -->

#### Out of scope
Mail Semantic interface hook (SEM-027). 1.0 release notes (DOC-042). Post-1.0 hardware non-goals (APP-069).

#### Acceptance criteria
- [ ] The published 1.0 non-promise list names online accounts, parental controls and speech dictation as out.
- [ ] Each item names the Semantic-interface hook reserved (or explicitly none) rather than a native app.
- [ ] The list does not promise a native mail, calendar or contacts client.
- [ ] DOC publishes the list through the 1.0 notes path; APP authors the APP section.

#### Verification
- Review: DOC and GOV leads sign off that the APP section matches GAP-0321 and I-092.

#### Evidence
- none

### APP-069 · Publish post-1.0 non-goals for casting, NFC, WWAN and MIDI
- Type: docs
- Milestone: LATER
- Status: todo
- Size: S
- Owner: none
- Depends on: APP-068
- Baseline: §62
- Invariants: I-093

Publish that casting (Miracast, Chromecast, AirPlay), NFC, WWAN/eSIM and MIDI/pro-audio appear in the post-1.0 non-goal list. AUD and HW own any later mechanism work; APP owns the combined declaration so the desktop-essentials list does not expand in silence.

<!-- covers: EXTRA-038 -->

#### Out of scope
MIDI/pro-audio parking (AUD-030). Hardware enablement (HW). 1.0 non-goals (APP-068).

#### Acceptance criteria
- [ ] The published post-1.0 non-goal list names casting, NFC, WWAN/eSIM and MIDI/pro-audio.
- [ ] Each item names the owning prefix for any later mechanism (AUD, HW, NET) and states APP will not ship chrome before that mechanism.
- [ ] I-093 remains the invariant this list enforces.

#### Verification
- Review: GOV reviewer confirms the list matches I-093 and EXTRA-038.

#### Evidence
- none
