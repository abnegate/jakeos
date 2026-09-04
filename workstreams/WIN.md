# WIN · Windows personality
- Prefix: WIN
- Lead: none
- Baseline: §3, §4, §9.1, §25, §48, §49, §56.2, §57, §62, §63, §69

<!-- roadmap:generated:begin summary -->
Tasks: 85 live, 0 done, 0 in-progress, 85 todo, 0 dropped. Ready: 1. Blocked: 84. Weighted: 0%.
<!-- roadmap:generated:end -->

## Scope
The Windows personality that runs existing Windows software as a product, not a shim and not a Windows clone (§3, §48, §56.2). This workstream owns the V0 scoping Decision; V1 non-gated Wine-on-Linux-personality bring-up, Wine test-suite CI, W1 Corpus definition and per-title harness; V2 PE loading, Win32 and selected NT semantics via a Wine/Proton-derived layer, per-application prefixes, registry and filesystem translation, DXVK and VKD3D-Proton, gaming proof-of-concept, and native chrome for `.exe` launch; later W2/W3 corpora, Proton-style compat configuration, installer-to-launcher registration, library adoption, and the honest anti-cheat and DRM nongoal. Native software never sees Win32, NT objects, Windows handles, DirectX, drive letters or the registry (§3, §57).

## Out of scope
Linux personality, Steam client, ia32 kernel retention and POSIX views (LNX). KVM VM manager, guest tools and the offer-the-VM triage action (VIRT). Immutable Wine prefix store layout and personality runtime multi-version Packages (PKG). DRM/KMS, RenderQueue, HDR Surfaces, VRR scheduling and immediate presentation (GFX). Clipboard policy, unified drag-and-drop types and UI protocol hooks (UIP). Taskbar, launcher, File Browser, Notify service and chooser chrome (APP). UserSelected minting and the personality view API (STO). Native AudioStream objects (AUD). HID and gamepad device enablement (HW). Secrets service and the system threat model (SEC). Wine/DXVK/VKD3D licence review and the legal entity (GOV). CI runners and the reproducible-build verifier (BLD). B-027 publication (BEN). Public compatibility database and HCL (REL). Compatibility-guide book pipeline (DOC). UIA/MSAA tree bridging (ACC). Native IME host (TXT). Crash capture format (OBS). Windows-profile import engine (INS). Component mapping of personality processes (CMP). NT object-manager research spike (ABI). Measured boot (BOOT). GPU lab machines (LAB).

## Tasks

### WIN-001 · Accept the Windows Personality scoping Decision
- Type: adr
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: none
- Baseline: §3, §48, §49, §56.2, §57
- Decision: D-0342
- Risks: R-014
- Invariants: I-007, I-025, I-096

Accept the only V0 Windows deliverable: the personality is a compatibility subsystem that starts from Wine and Proton, not a Windows clone, and native software never sees Win32 (§3, §48). The Decision forbids presenting compatibility software inside an obvious VM, on a separate desktop, or behind a wizard for ordinary titles (§49), and records Windows gaming as a major staffed objective (§56.2).

<!-- covers: EXTRA-039, INV-0012, INV-0918, INV-0919, INV-0920, INV-1068 -->

#### Out of scope
Wine bring-up (WIN-015). VM fallback product (VIRT-002). Native ABI firewall lint (ABI-003).

#### Acceptance criteria
- [ ] The Decision file evaluates at least: (A) Wine/Proton as the starting point, not a clone, with no obvious VM, no separate desktop, no ordinary-case wizard, and gaming staffed as a major objective; (B) defer the Windows personality past 1.0; (C) a clean-room Win32 implementation without Wine.
- [ ] The accepted option keeps Win32 inside the personality (I-007) and treats compatibility as a product (I-096).
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: WIN and GOV reviewers sign off on the pull request that accepts the Decision.

#### Evidence
- none

### WIN-002 · Decide the kernel-level anti-cheat policy
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: WIN-003, Q-043
- Baseline: §48, §56.2, §57
- Decision: D-0335
- Risks: R-031, R-036
- Invariants: I-071

Decide the policy for kernel-level anti-cheat drivers that cannot run under the Capability model (Q-043). Bypass is an explicit nongoal (I-071). Vendor contracts are a later Decision (WIN-058) and must not relax this policy.

<!-- covers: INV-1073, GAP-0452 -->

#### Out of scope
Per-vendor legal agreements (WIN-058). Public excluded-title page (WIN-018). VM manager (VIRT-008).

#### Acceptance criteria
- [ ] The Decision file evaluates at least: (A) refuse kernel-level anti-cheat in 1.0; (B) refuse in the personality and offer the VIRT fallback for those titles; (C) vendor engagement that would load anti-cheat kernel drivers (rejected if it breaks the Capability model).
- [ ] The accepted option forbids anti-cheat bypass (I-071) and names how excluded titles are disclosed.
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: WIN, SEC and GOV reviewers sign off on the pull request that accepts the Decision.
- Report: `reports/spikes/WIN-003.md` is cited as input.

#### Evidence
- none

### WIN-003 · Study kernel-level anti-cheat feasibility under the Capability model
- Type: spike
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: WIN-001
- Baseline: §48, §56.2
- Risks: R-031, R-035
- Invariants: I-071

Study Easy Anti-Cheat, BattlEye and other kernel-mode anti-cheat systems against Capability isolation and kernel lockdown so the policy Decision is not a V2 cliff. The report states which systems cannot work under the model and are therefore out of 1.0 scope, with no bypass research.

<!-- covers: INV-1072, GAP-0509 -->

#### Out of scope
Policy Decision (WIN-002). Measured-boot vendor signal (WIN-062). Circumvention of DRM or anti-tamper (I-071).

#### Acceptance criteria
- [ ] A report at `reports/spikes/WIN-003.md` covers EAC, BattlEye and at least one additional kernel-mode vendor, each scored as userspace-viable, vendor-cooperation-only, or incompatible with Capabilities.
- [ ] The report names the systems that cannot work under the Capability model and records bypass as out of scope.
- [ ] The report lists evidence sources (public vendor docs, Proton tracker classes) and does not include disassembly of anti-cheat binaries (I-070).

#### Verification
- Report: which vendors can operate in userspace; which require kernel drivers; which are incompatible with Capabilities and lockdown; what 1.0 may claim about gaming; which option WIN-002 must not pick.
- Review: WIN and SEC reviewers sign off on the pull request that lands the report.

#### Evidence
- none

### WIN-004 · Lint the tree against the Windows Personality clean-room policy
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: WIN-005
- Baseline: §48
- Invariants: I-070

Enforce the clean-room Decision in CI: the build fails on disassembly artefacts, leaked-source contributor markers, and bundled Microsoft fonts or unsigned redistributables so taint cannot enter the Wine/Proton packaging path.

<!-- covers: GAP-0025 -->

#### Out of scope
Policy Decision (WIN-005). Per-prefix redistributable Packages (WIN-042). Licence review text (GOV-047).

#### Acceptance criteria
- [ ] CI on `qemu-x86_64` fails the WIN packaging job when a Microsoft font, unsigned redistributable, or disassembly artefact is added under the personality tree.
- [ ] A contributor marker listed by the Decision as tainted fails the same job with a typed licence error.
- [ ] A wine-mono or wine-gecko object allowed by the Decision passes the lint.

#### Verification
- Unit: `win:tests/lint/clean_room_*` on CI matrix entries `qemu-x86_64` and `qemu-ia32`.
- Review: GOV reviewer confirms the lint matches the accepted clean-room Decision.

#### Evidence
- none

### WIN-005 · Decide a clean-room policy for the Windows Personality
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: WIN-001, GOV-003
- Baseline: §48
- Decision: D-0338
- Threats: T-007
- Invariants: I-070

Decide the clean-room rules that must precede Wine bring-up: no disassembly of Microsoft binaries, exclusion of contributors exposed to leaked Windows source, and what Microsoft bits may exist inside a prefix. Resolves the tension between a total ban on Microsoft binaries and per-prefix redistributables required by WIN-042.

<!-- covers: GAP-0025 -->

#### Out of scope
CI lint (WIN-004). Upstream-first patch policy (WIN-007). Redistributable packaging (WIN-042).

#### Acceptance criteria
- [ ] The Decision file evaluates at least: (A) Wine's rules as-is; (B) Wine's rules plus contributor affidavits; (C) a ban on all Microsoft binaries in tree and in prefixes; (D) Wine's rules plus Decision-listed per-prefix redistributables that WIN-042 may ship.
- [ ] The accepted option forbids disassembly of Microsoft binaries and leaked-source contributors (I-070).
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: WIN and GOV reviewers sign off on the pull request that accepts the Decision.

#### Evidence
- none

### WIN-006 · Build the per-title Windows scenario harness
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: WIN-009
- Baseline: §48, §49, §56.2
- Corpora: C-007
- Risks: R-031

Build the per-title scenario harness so V2 W1 runs are mechanical: each C-007 entry has a script that emits a Platinum/Gold/Silver/Bronze/Broken rating, integration-check artefacts, and the traces BEN's B-027 harness consumes.

<!-- covers: EXTRA-069 -->

#### Out of scope
Corpus membership (WIN-009). B-027 publication (BEN-044). GPU nightly corpus (WIN-030).

#### Acceptance criteria
- [ ] Every C-007 entry has a scenario script under `compat:windows-W1` that exits with a rating on the published scale.
- [ ] The harness records integration checks named by C-007 (taskbar, launcher, notifications, clipboard, audio, file chooser, gamepad) as pass, fail or not-applicable.
- [ ] Harness output on H-001 is the input schema named by B-027, with no FPS number in the WIN task prose.

#### Verification
- Integration: `compat:windows-W1` dry-run of two listed titles on H-001 and H-002.
- Compat: C-007 scenario schema review against `registers/corpora.md`.
- Review: BEN reviewer confirms the artefact schema matches harness `bench:windows-overhead`.

#### Evidence
- none

### WIN-007 · Publish an upstream-first policy for Wine, Proton, DXVK and VKD3D
- Type: docs
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: WIN-005, PKG-010
- Baseline: §48
- Invariants: I-069

Publish the contribution policy: no long-lived Wine fork, patches go upstream first, and every modified Wine build shipped is accompanied by LGPL corresponding source. Required before CI packages Wine.

<!-- covers: GAP-0028 -->

#### Out of scope
Licence review of shipped builds (GOV-047). Runtime Packages (WIN-012). Relinking mechanism (PKG-010).

#### Acceptance criteria
- [ ] A committed policy document states upstream-first for Wine, Proton, DXVK and VKD3D-Proton and forbids a long-lived Wine fork.
- [ ] The document names how corresponding source for a modified Wine Package is published beside the binary Package.
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: WIN and GOV reviewers sign off on the pull request that lands the policy.

#### Evidence
- none

### WIN-008 · Decide that Win32 emulation stays in userspace
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: WIN-001, GOV-003, Q-050
- Baseline: §3, §48, §5.1
- Decision: D-0343
- Threats: T-011
- Invariants: I-007, I-067

Decide whether NT-object or PE-loading emulation in the GPLv2 kernel conflicts with LGPL Wine, or whether all Win32, NT and PE emulation remains in userspace (Q-050). The answer constrains architecture before bring-up.

<!-- covers: GAP-0084 -->

#### Out of scope
PE loader implementation (WIN-038). NT fidelity (WIN-035). Hosting topology (WIN-013).

#### Acceptance criteria
- [ ] The Decision file evaluates at least: (A) all Win32, NT and PE emulation in userspace; (B) PE loader in-kernel; (C) NT objects in-kernel.
- [ ] The accepted option records the GPLv2/LGPL interaction and keeps native software free of Win32 types (I-007).
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: WIN, KRN and GOV reviewers sign off on the pull request that accepts the Decision.

#### Evidence
- none

### WIN-009 · Define the W1 Corpus and rating scale
- Type: docs
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: WIN-001, WIN-002, WIN-010
- Baseline: §48, §56.2, §62
- Corpora: C-007
- Risks: R-031
- Invariants: I-071

Define C-007 before the V2 gate: 50 titles (35 games without kernel-level anti-cheat, 15 productivity applications) with Platinum/Gold/Silver/Bronze/Broken scenarios in `registers/corpora.md`. Membership follows WIN-002 and WIN-010 so 32-bit and anti-cheat questions are not rediscovered at V2.

<!-- covers: EXTRA-069 -->

#### Out of scope
Per-title harness (WIN-006). W1 gate run (WIN-051). Public database publication (REL-015).

#### Acceptance criteria
- [ ] `registers/corpora.md` C-007 lists 50 named titles matching the register Size clause, with scenario identifiers.
- [ ] No listed game requires kernel-level anti-cheat (I-071).
- [ ] If WIN-010 accepts 64-bit-only, every listed title is 64-bit or has a 64-bit build named in the entry.
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: WIN and BEN reviewers sign off that C-007 matches the register Size, Scale and V2 threshold clauses.
- Compat: C-007 membership diff against the anti-cheat exclusion list from WIN-002.

#### Evidence
- none

### WIN-010 · Decide 32-bit Win32 support and WoW64 requirements
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: WIN-001, LNX-015, Q-040
- Baseline: §48, §56.2
- Decision: D-0344
- Risks: R-032

Decide whether 32-bit Win32 binaries (WoW64) are supported in 1.0 (Q-040). Kernel ia32 retention is LNX-015; this Decision is the Win32/WoW64 half so C-007 membership is honest.

<!-- covers: GAP-0507, INV-0914 -->

#### Out of scope
ia32 syscall retention (LNX-015). WoW64 runtime (WIN-055). Steam 32-bit multilib (LNX-086).

#### Acceptance criteria
- [ ] The Decision file evaluates at least: (A) 32-bit Win32 in 1.0; (B) 64-bit-only; (C) 32-bit only via the VIRT fallback.
- [ ] The accepted option states the kernel support it assumes from LNX-015.
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: WIN and LNX reviewers sign off on the pull request that accepts the Decision.

#### Evidence
- none

### WIN-011 · Survey Win32 APIs required by the W1 productivity set
- Type: spike
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: WIN-009
- Baseline: §48

Survey the 15 C-007 productivity applications and list the Win32 APIs, COM/OLE surfaces, installers and runtimes that WIN-053, WIN-022 and WIN-042 must enable.

Required by V2-G19 (W1 corpus meets its threshold): the survey is the input WIN-053, WIN-022 and WIN-042 enable the C-007 productivity titles from.

#### Out of scope
Win32 enablement (WIN-053). Redistributable Packages (WIN-042). COM runtime (WIN-022).

#### Acceptance criteria
- [ ] A report at `reports/spikes/WIN-011.md` names each of the 15 C-007 productivity titles and the APIs, COM classes, installers and runtimes it requires.
- [ ] The report marks each dependency as Wine-already, redistributable, or missing, with no Microsoft-binary disassembly (I-070).
- [ ] The report is an input cited by WIN-053 and WIN-042.

#### Verification
- Report: which APIs the V2 Win32 surface must enable; which titles need COM/OLE; which runtimes become prefix Packages; which installer families appear.
- Review: WIN reviewer sign-off on the pull request that lands the report.

#### Evidence
- none

### WIN-012 · Package Wine and Proton runtimes as immutable Packages for CI
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: WIN-007, WIN-004, PKG-047, PKG-038
- Baseline: §28, §48
- Invariants: I-036, I-069

Produce content-addressed Wine and Proton runtime Packages that CI installs without mutating a shared prefix. PKG owns store layout and later wine-prefix packaging; this task ships the runtime objects V1 bring-up installs.

<!-- covers: INV-0908 -->

#### Out of scope
Per-application prefix overlays (PKG-080). Wine process host (LNX-034). Bring-up of a PE binary (WIN-015).

#### Acceptance criteria
- [ ] A Wine runtime Package and a Proton runtime Package install from the store on H-001 with no writes outside the content-addressed store and ApplicationData.
- [ ] Two builds of identical Wine sources yield the same Package identity on the PKG identity check.
- [ ] Corresponding source for any WIN patch is published beside the binary Package (I-069).
- [ ] The clean-room lint job is green on the packaged tree.

#### Verification
- Integration: `win:tests/packages/wine_runtime_install_*` on `qemu-x86_64`.
- Review: PKG reviewer confirms the objects are immutable Packages, not a mutable prefix.

#### Evidence
- none

### WIN-013 · Decide whether Wine hosts on the Linux Personality or the Native ABI
- Type: adr
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: WIN-014, WIN-008
- Baseline: §3, §4, §48, §69
- Decision: D-0345
- Risks: R-035

Decide the V1 non-gated architecture for hosting Wine: over the Linux personality, ported onto the Native ABI, or a hybrid with graphics and input native and the remainder on Linux. The spike report is an input; V2 object mapping follows this Decision.

<!-- covers: GAP-0508, INV-0907 -->

#### Out of scope
Object-mapping architecture at V2 (WIN-036). Bring-up on Linux personality (WIN-015). Overhead measurement (WIN-014).

#### Acceptance criteria
- [ ] The Decision file evaluates at least: (A) Wine over the Linux personality; (B) Wine ported onto the Native ABI; (C) hybrid with graphics and input native and the remainder on Linux, each citing `reports/spikes/WIN-014.md`.
- [ ] The accepted option states what native software still must not see (I-007) and which V2 tasks change if the host is Linux syscalls versus Native ABI bindings.
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: WIN, LNX and ABI reviewers sign off on the pull request that accepts the Decision.

#### Evidence
- none

### WIN-014 · Measure Wine-on-Linux-Personality translation overhead
- Type: spike
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: WIN-015
- Baseline: §48, §54, §56.2
- Explores: S-030
- Risks: R-035

Measure translation overhead of Wine hosted by the Linux personality on a fixed game set, and estimate Native ABI port cost, so WIN-013 is not guesswork. Publish-only; B-027 remains BEN's W1 Gold harness and this spike makes no superiority claim.

<!-- covers: GAP-0508 -->

#### Out of scope
Hosting Decision (WIN-013). W1 Gold FPS tables (BEN-044). Native ABI unixlib port (WIN-036).

#### Acceptance criteria
- [ ] A report at `reports/spikes/WIN-014.md` runs the named game set under Wine on the Linux personality on H-002 and records CPU and frame-time method against B-027, with numbers only in the report.
- [ ] The report estimates Native ABI port cost as engineering scope, not as a promised speedup.
- [ ] The report scores how each hosting option attaches to S-030 (clipboard, GPU, input, chooser).

#### Verification
- Report: measured overhead of Wine-on-Linux-personality versus the Linux+Proton baseline method; estimated port cost of unixlib-to-Native-ABI; which S-030 attachments stay native in a hybrid.
- Bench: B-027 method on H-002 for the spike set; publish-only, no target claimed.
- Review: WIN and BEN reviewers sign off on the pull request that lands the report.

#### Evidence
- none

### WIN-015 · Bring Wine up on the Linux Personality
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: WIN-008, WIN-012, LNX-034, CMP-036
- Baseline: §3, §48, §56.2
- Risks: R-031, R-035
- Invariants: I-007

Non-gated V1 bring-up: a PE binary starts under Wine hosted by the Linux personality. There is no V1 pass-rate gate. Native software still sees no Win32.

<!-- covers: EXTRA-069, INV-0894 -->

#### Out of scope
Wine test suite CI (WIN-017). V2 personality core (WIN-054). Hosting architecture Decision (WIN-013).

#### Acceptance criteria
- [ ] A named 64-bit PE binary packaged for CI starts under Wine on H-001 via LNX-034 and exits with a recorded status.
- [ ] Native crates in the same image still fail ABI-003 if they link Win32 or Wine headers (I-007).
- [ ] `os inspect` on the hosting Component shows a compatibility-default Capability set, not ambient home authority.
- [ ] No V1 C-007 pass-rate is asserted.

#### Verification
- Integration: `win:tests/bringup/pe_start_*` on `qemu-x86_64` and H-002.
- Review: LNX reviewer confirms the host is the Linux personality, not a second desktop.

#### Evidence
- none

### WIN-016 · Publish the V1 Wine test-suite feasibility report
- Type: docs
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: WIN-017
- Baseline: §48
- Corpora: C-011

Publish the V1 compatibility-gate feasibility report on running C-011 under the Linux personality, with no pass-rate gate.

#### Out of scope
Nightly runner (WIN-017). GPU corpus (WIN-030). W1 ratings (WIN-051).

#### Acceptance criteria
- [ ] A committed report records C-011 pass, fail and skip counts on H-001 against the V1 publish threshold, with no pass-rate target claimed.
- [ ] The report names harness gaps that WIN-030 must close.
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Compat: C-011 V1 publish report path under `reports/compat/C-011/`.
- Review: WIN reviewer sign-off on the pull request that lands the report.

#### Evidence
- none

### WIN-017 · Run the Wine test suite in CI as a non-gated V1 bring-up signal
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: WIN-012, WIN-015
- Baseline: §48
- Corpora: C-011

Package C-011 as a WIN-owned CI artefact (test binaries, skip taxonomy, result schema) and wire a non-gating BLD job so Wine packaging regressions are visible before the nightly run. WIN owns the suite; BLD hosts runners.

<!-- covers: INV-0912, EXTRA-069 -->

#### Out of scope
Nightly GPU lab run (WIN-030). BLD runner hardware (BLD). W1 title harness (WIN-006).

#### Acceptance criteria
- [ ] The Wine test suite is a content-addressed artefact CI can fetch, with a published skip taxonomy matching C-011 Scale.
- [ ] A non-gating BLD job consumes that artefact and posts a bring-up signal (pass/fail/skip totals) on WIN packaging changes.
- [ ] The job is not a merge blocker on Wine test fails.

#### Verification
- Integration: `win:tests/wine_suite/artefact_*` on `qemu-x86_64`.
- Review: BLD reviewer confirms the job is non-gating and hosted on BLD runners.

#### Evidence
- none

### WIN-018 · Publish titles blocked by kernel-level anti-cheat as a nongoal
- Type: docs
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: WIN-002, WIN-009
- Baseline: §48, §56.2, §62
- Risks: R-036
- Invariants: I-071

Publish the honest page of games excluded by kernel-level anti-cheat so V2 users judging the milestone by those titles see the nongoal. Bypass remains out of scope.

<!-- covers: GAP-0452 -->

#### Out of scope
Policy Decision (WIN-002). 1.0 unsupported matrix (WIN-083). VM offer UX (VIRT-013).

#### Acceptance criteria
- [ ] A committed page lists excluded titles by vendor class (EAC, BattlEye, other kernel-mode) and states bypass is a nongoal (I-071).
- [ ] No listed excluded title appears as a C-007 member.
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: WIN and DOC reviewers sign off on the pull request that lands the page.

#### Evidence
- none

### WIN-019 · Decide the case-insensitive view for Windows Personality storage
- Type: adr
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: STO-047, WIN-001
- Baseline: §25, §48
- Decision: D-0337
- Invariants: I-007

Decide how case-insensitive, case-preserving semantics exist for the personality view of user data without leaking into native or Linux views.

<!-- covers: GAP-0432 -->

#### Out of scope
Filesystem translation implementation (WIN-028). Native and Linux views (STO, LNX-019).

#### Acceptance criteria
- [ ] The Decision file evaluates at least: (A) Wine path lookup; (B) a per-prefix casefold overlay; (C) filesystem casefold confined to the personality view API from STO-047.
- [ ] The accepted option forbids case-insensitive lookup on native and Linux views (I-007).
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: WIN and STO reviewers sign off on the pull request that accepts the Decision.

#### Evidence
- none

### WIN-020 · Route Windows open and save dialogs through the native file chooser
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: WIN-054, APP-002, STO-034, WIN-040
- Baseline: §9.1, §25, §49
- Threats: T-002, T-012, T-025
- Invariants: I-021, I-035

Route `GetOpenFileName` and `IFileDialog` through the OS-owned chooser so the personality receives `UserSelected` Capabilities rather than ambient paths. V2 exit names the native file chooser.

<!-- covers: INV-0927 -->

#### Out of scope
Chooser UI and minting (APP-002, STO-034). OLE drops (WIN-037). Linux portals (LNX-036).

#### Acceptance criteria
- [ ] A W1 productivity title that opens a file via the common dialog receives a `UserSelected` Capability and cannot enumerate the parent directory.
- [ ] A prefix with no chooser grant that calls the common dialog receives a typed denial, allocates no path handle, and appears in the capability audit log.
- [ ] The chooser renders on a trusted Surface the personality cannot overlay (T-012).

#### Verification
- Integration: `win:tests/chooser/userselected_*` on H-002 and H-003.
- Compat: C-007 file-chooser integration check on H-002.
- Unit: `win:tests/chooser/deny_*` on `qemu-x86_64`.

#### Evidence
- none

### WIN-021 · Bridge the Windows clipboard onto the native clipboard Capability
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: WIN-054, UIP-003, UIP-029
- Baseline: §9.1, §41, §49
- Threats: T-001, T-025, T-032
- Invariants: I-021

Implement the Wine/OLE side of the native clipboard so Windows applications copy and paste through `Capability` on S-032. UIP owns policy and unified types.

<!-- covers: INV-0924 -->

#### Out of scope
Clipboard policy (UIP-004). Unified type registry (UIP-050). X11 primary selection (T-032, LNX).

#### Acceptance criteria
- [ ] Copy from a W1 Windows title and paste into a native demo application transfers the negotiated type; the reverse path succeeds on H-002.
- [ ] A Component launched without the clipboard Capability that reads the Windows clipboard via the bridge receives `Error::Rights` and allocates no handle.
- [ ] An X11 primary-selection offer injected into the bridge is rejected and creates no native clipboard entry (T-032).

#### Verification
- Integration: native-to-Windows and Windows-to-native copy-paste on H-002.
- Compat: C-007 clipboard integration check on H-002 and H-004.
- Unit: `win:tests/clipboard/deny_*` on `qemu-x86_64`.

#### Evidence
- none

### WIN-022 · Provide COM and OLE for W1 productivity applications
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: WIN-053, WIN-011, WIN-040
- Baseline: §48
- Corpora: C-007
- Invariants: I-007

Enable Wine COM and OLE inside the per-application prefix for the 15 C-007 productivity titles. COM is not a native API.

<!-- covers: INV-0900 -->

#### Out of scope
Native COM (forbidden by I-007). OLE drag-and-drop bridge (WIN-037). Redistributables (.NET, VC++) (WIN-042).

#### Acceptance criteria
- [ ] Each C-007 productivity title that the survey marked COM-required creates and invokes a COM object inside its prefix on H-002.
- [ ] Native crates cannot link COM headers; ABI-003 remains green.
- [ ] COM activation in prefix A cannot instantiate a class registered only in prefix B (I-076).

#### Verification
- Compat: C-007 productivity COM smoke on H-002.
- Unit: `win:tests/com/prefix_isolation_*` on `qemu-x86_64`.
- Review: WIN reviewer confirms COM stays inside the personality.

#### Evidence
- none

### WIN-023 · Deliver native keyboard, mouse and pointer input to Windows applications
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: WIN-054, HW-011, UIP-012
- Baseline: §41, §49
- Threats: T-012

Deliver HID keyboard, mouse and pointer events through the personality to the focused Windows Surface. Gamepad/XInput is WIN-056.

<!-- covers: INV-0928 -->

#### Out of scope
Gamepad mapping (WIN-056). HID service (HW-011). Focus policy (UIP-005).

#### Acceptance criteria
- [ ] A W1 title on H-002 receives key and pointer events only while its Surface is focused.
- [ ] An unfocused Windows Surface observes no pointer or key events.
- [ ] Synthetic input into a trusted chooser Surface from the personality is rejected (T-012).

#### Verification
- Integration: `win:tests/input/focus_*` on H-002 and H-003.
- Compat: C-007 input integration check for keyboard and mouse on H-002.

#### Evidence
- none

### WIN-024 · Integrate Windows applications into the native desktop chrome
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: WIN-054, APP-043, APP-032, WIN-040
- Baseline: §49, §62
- Invariants: I-007

Supply window identity, PE icons and no-separate-desktop chrome so Windows applications appear on the native desktop. APP owns taskbar and launcher widgets.

<!-- covers: INV-1224 -->

#### Out of scope
Panel and launcher chrome (APP-043, APP-032). Double-click handler (WIN-027). Separate-desktop nongoal already in WIN-001.

#### Acceptance criteria
- [ ] A running W1 title appears in the native taskbar and launcher with its PE icon and window title on H-002.
- [ ] The title's Surface is a native compositor window, not a nested desktop or VM window.
- [ ] Closing the last Windows window does not leave a personality desktop session visible.

#### Verification
- Integration: `win:tests/desktop/chrome_*` on H-002 and H-004.
- Compat: C-007 taskbar and launcher integration checks on H-002.
- Demo: V2 double-click path consumed by WIN-048.

#### Evidence
- none

### WIN-025 · Map Credential Manager and DPAPI onto the native secrets service
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: WIN-040, SEC-027, SEC-026
- Baseline: §9.1, §48
- Threats: T-025, T-026
- Invariants: I-076

Implement the Wine Credential Manager and DPAPI path with per-application scoping onto the native secrets service so W1 titles store credentials without a shared user-wide vault.

#### Out of scope
Native secrets service (SEC-027). Linux libsecret shim (LNX-085). Prefix isolation layout (WIN-040).

#### Acceptance criteria
- [ ] A W1 title storing a credential via DPAPI reads it back after restart of its Component.
- [ ] Prefix B that enumerates or reads prefix A's DPAPI blobs receives `Error::Rights` and allocates no handle.
- [ ] Secrets do not appear in crash dumps captured from the prefix (I-077).

#### Verification
- Integration: `win:tests/dpapi/roundtrip_*` on H-002.
- Unit: `win:tests/dpapi/cross_prefix_deny_*` on `qemu-x86_64`.
- Review: SEC reviewer confirms scoping matches SEC-026.

#### Evidence
- none

### WIN-026 · Integrate DXVK for Direct3D 9, 10 and 11 translation
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: WIN-054, GFX-057, WIN-007
- Baseline: §48, §56.2
- Invariants: I-007, I-045

Integrate DXVK so D3D9/10/11 calls inside the personality become Vulkan submitted through native graphics abstractions (GFX RenderQueue). Not a native DirectX API and not a native GPU driver stack.

<!-- covers: INV-0098, INV-0897, INV-0904, INV-1070 -->

#### Out of scope
D3D12 (WIN-049). Present/scanout (WIN-031). HDR metadata (WIN-032). Native GPU stack (I-045).

#### Acceptance criteria
- [ ] A named D3D11 C-007 title renders via DXVK to a Vulkan RenderQueue on H-002, confirmed by `os inspect` on the queue.
- [ ] Native crates cannot link d3d11 headers; ABI-003 remains green.
- [ ] DXVK is an immutable Package pinned by the prefix, not a runtime download.

#### Verification
- Integration: `win:tests/dxvk/d3d11_present_*` on H-002 and H-017.
- Compat: C-007 GPU smoke subset on H-002.
- Review: GFX reviewer confirms submission is via RenderQueue Capabilities.

#### Evidence
- none

### WIN-027 · Launch a .exe by double-click as a normal application window
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: WIN-038, WIN-024, APP-052, WIN-020, WIN-040
- Baseline: §49, §62
- Invariants: I-007

Implement the V2 exit and demo path: double-click a `.exe` in the File Browser and it appears as a normal application window with native chrome, no intermediate UI, and no wizard. APP decides which foreign formats are openable.

<!-- covers: INV-0917, INV-0916 -->

#### Out of scope
Format mapping Decision (APP-052). Taskbar widgets (APP-043). Installer-to-launcher model (WIN-033).

#### Acceptance criteria
- [ ] Double-clicking a C-007 `.exe` from the File Browser on H-002 starts a prefix Component and shows a native Surface with no wizard and no extra desktop.
- [ ] The new window appears in the native task switcher.
- [ ] A non-PE file named `.exe` does not start Wine; the failure is a typed error visible to WIN-064.

#### Verification
- Integration: `win:tests/launch/double_click_*` on H-002.
- Demo: consumed by WIN-048 on H-002.
- Compat: C-007 launch path for at least one productivity and one game entry.

#### Evidence
- none

### WIN-028 · Translate Windows filesystem semantics onto native storage
- Type: build
- Milestone: V2
- Status: todo
- Size: L
- Owner: none
- Depends on: WIN-019, STO-047, WIN-040
- Baseline: §25, §48
- Threats: T-011, T-025
- Invariants: I-007, I-016, I-076

Translate drive letters, share/lock semantics, path-length rules and the accepted casefold view onto native storage, confined to the personality view. Native and Linux views remain case-sensitive unless their own Decisions say otherwise.

<!-- covers: INV-0099, INV-0896, INV-0902, GAP-0432 -->

#### Out of scope
Casefold Decision (WIN-019). Personality view API (STO-047). Registry (WIN-043). Native filesystem rewrite (I-044).

#### Acceptance criteria
- [ ] A W1 title that opens `C:\` paths inside its prefix reads and writes ApplicationData through the STO view API, not an ambient POSIX tree.
- [ ] Two files whose names differ only by case are distinct on the native view and behave as the accepted casefold Decision requires inside the prefix.
- [ ] Prefix A cannot open a path that resolves to prefix B's files (I-076, T-011).
- [ ] Share/lock requests from two Handles in one prefix match the documented Wine lock behaviour on a test file.

#### Verification
- Unit: `win:tests/fs/casefold_*`, `win:tests/fs/drives_*`, `win:tests/fs/share_lock_*` on `qemu-x86_64`.
- Integration: C-007 title file I/O on H-002.
- Fuzz: `win:fuzz/win_paths` one hour nightly without panic.

#### Evidence
- none

### WIN-029 · Run ten Gold W1 titles with HDR or VRR and gamepad input
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: WIN-009, WIN-032, WIN-050, WIN-056, WIN-031, WIN-046, WIN-006
- Baseline: §48, §56.2, §62
- Benchmarks: B-027
- Corpora: C-007
- Risks: R-035, R-036

Deliver the V2 gaming proof-of-concept and demo: named C-007 titles at Gold with HDR or VRR and gamepad input on H-002. FPS is published by BEN against Linux+Proton on the same hardware; WIN records ratings and makes no superiority claim.

<!-- covers: INV-1225, INV-1111, INV-0906 -->

#### Out of scope
B-027 publication (BEN-044, BEN-041). W1 threshold run (WIN-051). Anti-cheat titles (WIN-018).

#### Acceptance criteria
- [ ] C-007's V2 HDR-or-VRR Gold clause is met on H-002 with gamepad input scored pass.
- [ ] Each proof title is named in the Evidence-bound rating sheet produced by WIN-006.
- [ ] BEN-044 has a V2 publish report for those Gold titles; WIN prose cites B-027 only.

#### Verification
- Compat: C-007 V2 HDR/VRR Gold clause on H-002.
- Bench: B-027 on H-002; target per register (publish).
- Demo: a named W1 title with VRR and HDR on H-002 using the B-027 scene, consumed by WIN-048.

#### Evidence
- none

### WIN-030 · Run Wine tests and the W1 smoke Corpus on GPU lab machines
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: WIN-017, WIN-006, WIN-026
- Baseline: §48, §56.2
- Corpora: C-007, C-011

Run C-011 plus a curated W1 smoke subset nightly on GPU-equipped lab machines so graphics regressions are visible before the V2 gate. V1 wine-test-ci remains CPU/QEMU.

<!-- covers: GAP-0117, INV-0912 -->

#### Out of scope
QEMU C-011 (WIN-017). Full W1 gate (WIN-051). Lab procurement (LAB).

#### Acceptance criteria
- [ ] A nightly job on H-002 and H-017 runs C-011 and the named W1 smoke subset with GPU acceleration confirmed via RenderQueue inspect.
- [ ] A QEMU-only pass with a GPU-job fail is reported as a GPU regression, not a silent skip.
- [ ] Results land under `reports/compat/C-011/` and `reports/compat/C-007/` as bring-up signals, not as the V2 threshold.

#### Verification
- Compat: C-011 and C-007 smoke on H-002 and H-017 nightly.
- Integration: `win:tests/ci/gpu_smoke_launch_*` on H-002.

#### Evidence
- none

### WIN-031 · Wire Vulkan present, explicit sync and fullscreen scanout
- Type: build
- Milestone: V2
- Status: todo
- Size: L
- Owner: none
- Depends on: WIN-026, GFX-074, GFX-057
- Baseline: §39, §40, §56.2
- Threats: T-031
- Invariants: I-084

Wire Vulkan device access, explicit synchronisation, present timing and fullscreen/direct scanout for the personality. Consumes GFX immediate-presentation and the personality GPU path.

<!-- covers: INV-1078, INV-0926 -->

#### Out of scope
Immediate-presentation mode (GFX-074). HDR metadata (WIN-032). VRR scheduling (WIN-050). Native GPU drivers (I-045).

#### Acceptance criteria
- [ ] A fullscreen W1 title on H-002 presents through the GFX immediate-presentation path with explicit sync; `os inspect` shows the Frame and RenderQueue.
- [ ] A personality client cannot sample another client's scanout buffer (T-031).
- [ ] Windowed and fullscreen present both complete without falling back to a second compositor desktop.

#### Verification
- Integration: `win:tests/gpu/present_fullscreen_*` on H-002 and H-017.
- Review: GFX reviewer confirms explicit sync and scanout rights.
- Compat: C-007 GPU present smoke on H-002.

#### Evidence
- none

### WIN-032 · Pass HDR metadata from DXVK and VKD3D to the native Surface
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: WIN-026, WIN-049, GFX-069
- Baseline: §40, §48, §56.2

Pass HDR metadata from DXVK and VKD3D-Proton onto native Surfaces so C-007's V2 HDR-or-VRR clause can be exercised. GFX owns HDR output.

<!-- covers: INV-1076, INV-0913 -->

#### Out of scope
HDR output pipeline (GFX-068). Personality HDR Surfaces (GFX-069). Linux HDR (LNX-075).

#### Acceptance criteria
- [ ] A named C-007 Gold title on H-002 delivers HDR metadata through DXVK or VKD3D onto a native Surface consumed by GFX-069.
- [ ] An SDR title on the same display does not force the HDR path; mixed SDR/HDR follows GFX tone mapping.
- [ ] Native software never sees DXGI HDR types (I-007).

#### Verification
- Integration: `win:tests/hdr/metadata_*` on H-002 with the LAB HDR display.
- Compat: C-007 V2 HDR-or-VRR clause contribution on H-002.
- Review: GFX reviewer confirms metadata matches the HDR Surface contract.

#### Evidence
- none

### WIN-033 · Decide how prefix installers become installed applications
- Type: adr
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: WIN-040, APP-032, Q-041
- Baseline: §28, §49
- Decision: D-0339

Decide how a `setup.exe` that writes a prefix becomes a launcher-visible installed application and a system-history event (Q-041). Needed before WIN-061.

<!-- covers: INV-0931 -->

#### Out of scope
Launcher implementation (WIN-061). APP launcher chrome (APP-032). History log (PKG-022).

#### Acceptance criteria
- [ ] The Decision file evaluates at least: (A) treat the prefix as the application; (B) scan Start Menu links after install; (C) require a native manifest wrapper.
- [ ] The accepted option names the history event type and the launcher identity key.
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: WIN, APP and PKG reviewers sign off on the pull request that accepts the Decision.

#### Evidence
- none

### WIN-034 · Map NT objects and handles onto native Objects and Capabilities
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: WIN-035, WIN-054, ABI-005
- Baseline: §3, §7, §48
- Threats: T-003, T-011
- Invariants: I-007, I-015

Map NT objects and Windows handles inside the personality onto native Objects and Capabilities per WIN-035. Native software never sees NT handles.

<!-- covers: INV-0095, INV-0097, INV-0898 -->

#### Out of scope
Fidelity Decision (WIN-035). Process semantics (WIN-041). In-kernel NT (WIN-008).

#### Acceptance criteria
- [ ] A W1 title that opens an NT handle holds a personality-side mapping to a native Object; `os inspect` shows the native terminus, not an NT type, on the native side.
- [ ] Forging a Windows handle from another prefix returns a typed denial and allocates no native handle (T-003).
- [ ] Native crates cannot include NT handle types; ABI-003 remains green.

#### Verification
- Unit: `win:tests/nt/handle_map_*`, `win:tests/nt/forge_deny_*` on `qemu-x86_64`.
- Integration: `os inspect` on a running W1 prefix Component on H-002.
- Review: ABI reviewer confirms native Object types are unchanged.

#### Evidence
- none

### WIN-035 · Decide NT Object-manager, async I/O, descriptor and section fidelity
- Type: adr
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: WIN-008, ABI-031
- Baseline: §7, §48
- Decision: D-0340

Decide which NT semantics (object manager, async I/O, security descriptors, sections) are emulated and to what fidelity. ABI-031 is input.

<!-- covers: INV-0901 -->

#### Out of scope
Handle mapping implementation (WIN-034). Hosting topology (WIN-013). Kernel NT objects (rejected unless WIN-008 accepted them).

#### Acceptance criteria
- [ ] The Decision file evaluates at least: (A) Wine's current NT layer; (B) a documented subset; (C) a native Object mapping per NT type, each citing `reports/spikes/ABI-031.md`.
- [ ] The accepted option lists object-manager, async I/O, descriptor and section fidelity as emulated, stubbed or out of scope.
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: WIN and ABI reviewers sign off on the pull request that accepts the Decision.

#### Evidence
- none

### WIN-036 · Decide how Wine and Proton map onto native Objects
- Type: adr
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: WIN-013, WIN-035
- Baseline: §4, §48, §69
- Decision: D-0341
- Risks: R-035

Decide the V2 architecture for how Wine and Proton map onto native Objects: remain on Linux-personality syscalls with native UX chrome, replace unixlib with Native ABI bindings, or hybrid native graphics and input. Required by the V2 exit.

<!-- covers: INV-0895, INV-1331 -->

#### Out of scope
Core integration (WIN-054). NT fidelity (WIN-035). Hosting Decision (WIN-013).

#### Acceptance criteria
- [ ] The Decision file evaluates at least: (A) Wine stays on Linux-personality syscalls with native UX chrome; (B) unixlib replaced by Native ABI bindings; (C) hybrid with graphics and input native, remainder on Linux, each citing WIN-013.
- [ ] The accepted option names the native Object terminus for files, GPU, input, audio and clipboard (I-027).
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: WIN, LNX and ABI reviewers sign off on the pull request that accepts the Decision.

#### Evidence
- none

### WIN-037 · Bridge OLE drag-and-drop onto native Capability-granting drops
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: WIN-021, UIP-050, UIP-032, WIN-040
- Baseline: §9.1, §41, §49
- Threats: T-002, T-025
- Invariants: I-021, I-035

Implement the Wine OLE drag-and-drop side so a native drop grants a Capability to the dropped object, not a path. UIP owns unified types.

#### Out of scope
Unified type registry (UIP-050). Native drag-and-drop (UIP-032). File chooser (WIN-020).

#### Acceptance criteria
- [ ] Dragging a file from the native File Browser onto a W1 window grants that prefix a Capability to the object and not to the parent directory.
- [ ] Dragging from a W1 window onto a native application transfers a Capability, not a DOS path.
- [ ] A drop into a prefix that lacks drop rights receives `Error::Rights` and allocates no handle.

#### Verification
- Integration: `win:tests/dnd/capability_drop_*` on H-002.
- Compat: C-007 drag-and-drop scoring where the scenario names it.
- Unit: `win:tests/dnd/deny_*` on `qemu-x86_64`.

#### Evidence
- none

### WIN-038 · Load PE executables and DLLs inside the Windows Personality
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: WIN-008, WIN-054
- Baseline: §48
- Threats: T-011
- Invariants: I-007

Load PE executables and DLLs for C-007 inside the personality, in userspace per WIN-008. Fuzzed by WIN-039.

<!-- covers: INV-0899 -->

#### Out of scope
Fuzz harness (WIN-039). In-kernel PE (WIN-008). Double-click UX (WIN-027).

#### Acceptance criteria
- [ ] A C-007 EXE and its dependent DLLs load and reach entry inside a prefix Component on H-001 and H-002.
- [ ] A malformed DOS header is rejected with a typed error and allocates no native executable mapping.
- [ ] Native software cannot invoke the PE loader; the symbol is not on Layer 1.

#### Verification
- Integration: `win:tests/pe/load_w1_*` on `qemu-x86_64` and H-002.
- Unit: `win:tests/pe/reject_malformed_*` on `qemu-x86_64`.
- Review: KRN reviewer confirms no in-kernel PE path unless WIN-008 accepted one.

#### Evidence
- none

### WIN-039 · Fuzz the PE loader without panic on malformed images
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: WIN-038
- Baseline: §48, §51
- Threats: T-011

Season the userspace PE-parse fuzz target before V3 continuous fuzzing and the V4 personality audit. Malformed images must not panic the loader Component.

<!-- covers: GAP-0130 -->

#### Out of scope
Loader behaviour for valid W1 binaries (WIN-038). Kernel fuzzing (BLD). Audit close-out (WIN-073).

#### Acceptance criteria
- [ ] Harness `win:fuzz/pe_loader` runs on H-001 without panic or host abort on malformed images for the nightly duration.
- [ ] Each unique crash is recorded as a typed reject, not an uncaught panic, before the harness is marked green.
- [ ] The corpus of malformed images lives in-tree and is not a Microsoft binary dump (I-070).

#### Verification
- Fuzz: `win:fuzz/pe_loader` one hour nightly on `qemu-x86_64` without panic.
- Unit: `win:tests/pe/fuzz_regressions_*` replay saved crashes on `qemu-x86_64`.

#### Evidence
- none

### WIN-040 · Isolate each Windows application in its own Component and prefix
- Type: build
- Milestone: V2
- Status: todo
- Size: L
- Owner: none
- Depends on: WIN-054, PKG-080, CMP-036
- Baseline: §9.1, §10, §48, §49
- Threats: T-011, T-025, T-026
- Invariants: I-021, I-076

Run each Windows application as a Component with its own Wine prefix: immutable Wine Packages plus mutable per-app prefix state, and the compatibility-default Capability set, so one compromised title cannot read another application's files or credentials. PKG owns store layout.

<!-- covers: INV-0908, INV-0909, GAP-0232 -->

#### Out of scope
Store layout (PKG-080). Process/job semantics (WIN-041). Services/autostart lifecycle (WIN-066).

#### Acceptance criteria
- [ ] Two W1 titles launched together have distinct prefix ApplicationData and distinct Components visible in `os inspect`.
- [ ] A process in prefix A that opens a path or registry key owned by prefix B receives a typed denial and allocates no handle (I-076, T-011).
- [ ] The compatibility-default Capability set does not include ambient home, clipboard-read, or other-prefix rights (I-021).
- [ ] Killing the Component tears down its prefix processes; no user-wide Wine server remains.

#### Verification
- Integration: `win:tests/prefix/isolation_*` on H-002.
- Unit: `win:tests/prefix/cross_deny_*` on `qemu-x86_64`.
- Review: SEC reviewer confirms the default set matches the personality threat addendum inputs.

#### Evidence
- none

### WIN-041 · Provide Windows process, thread, inheritance and job semantics
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: WIN-034, CMP-036, WIN-040
- Baseline: §3, §10, §20, §48
- Invariants: I-007, I-014

Provide CreateProcess, threads, handle inheritance and job objects inside the personality, mapped onto Components and Tasks per CMP-036. TSK owns native Task mapping.

<!-- covers: INV-0096, INV-0905 -->

#### Out of scope
Component mapping Decision (CMP-036). Native TaskGroup (TSK). Prefix lifecycle of services (WIN-066).

#### Acceptance criteria
- [ ] A W1 title that spawns a child process with inherited handles observes those handles in the child and not in another prefix.
- [ ] Job-object kill on the parent ends children in that prefix and does not cancel a second prefix's Component.
- [ ] Native software never sees Windows process IDs as a Layer 1 type (I-007).

#### Verification
- Unit: `win:tests/process/create_inherit_*`, `win:tests/process/job_kill_*` on `qemu-x86_64`.
- Integration: C-007 installer or launcher that spawns a child, on H-002.
- Review: CMP reviewer confirms the mapping matches the accepted Decision.

#### Evidence
- none

### WIN-042 · Package Windows runtime redistributables as prefix dependencies
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: WIN-005, WIN-040, PKG-080, WIN-011
- Baseline: §28, §48
- Threats: T-007
- Invariants: I-070

Package VC++ runtimes, .NET, WebView2 and legacy DirectX as per-prefix Packages, not runtime downloads. Contents are limited by WIN-005 (wine-mono/gecko versus Microsoft bits).

<!-- covers: EXTRA-029 -->

#### Out of scope
Clean-room Decision (WIN-005). Prefix store layout (PKG-080). Win32 surface enablement (WIN-053).

#### Acceptance criteria
- [ ] Each redistributable allowed by the clean-room Decision installs as a Package dependency of a prefix and is not fetched from the network at launch.
- [ ] A redistributable forbidden by the Decision fails `win:tests/lint/clean_room_*`.
- [ ] Two prefixes may pin different redistributable versions without a user-wide install.

#### Verification
- Integration: `win:tests/redist/prefix_pin_*` on H-001 and H-002.
- Unit: `win:tests/redist/no_runtime_download_*` on `qemu-x86_64`.
- Review: GOV reviewer confirms contents match the clean-room Decision.

#### Evidence
- none

### WIN-043 · Emulate the registry in per-application prefix storage
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: WIN-040
- Baseline: §3, §48
- Threats: T-011, T-026
- Invariants: I-007, I-076

Emulate registry semantics in per-application mutable prefix storage. A shared user-wide registry is forbidden.

<!-- covers: INV-0100, INV-0903 -->

#### Out of scope
Prefix isolation (WIN-040). Filesystem view (WIN-028). Native settings storage (SVC).

#### Acceptance criteria
- [ ] A W1 title that writes `HKCU` reads the value back after Component restart from its prefix ApplicationData.
- [ ] Prefix A cannot read `HKCU` or `HKLM` software keys written by prefix B (I-076).
- [ ] Native software has no registry API (I-007).

#### Verification
- Unit: `win:tests/registry/hkcu_roundtrip_*`, `win:tests/registry/cross_prefix_deny_*` on `qemu-x86_64`.
- Integration: C-007 productivity title settings survive relaunch on H-002.

#### Evidence
- none

### WIN-044 · Surface Wine and Proton version pinning in native UI
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: PKG-076, WIN-040
- Baseline: §29, §48

Surface per-application Wine/Proton pins in native UI for C-007 titles. PKG owns multi-version Packages.

<!-- covers: EXTRA-031 -->

#### Out of scope
Multi-version Package store (PKG-076). Proton compat database (WIN-067). Settings chrome (APP).

#### Acceptance criteria
- [ ] A C-007 title can be pinned to a named Wine or Proton Package version from native UI; relaunch uses that Package identity.
- [ ] Two titles may run different pinned versions concurrently.
- [ ] Clearing the pin restores the generation default runtime.

#### Verification
- Integration: `win:tests/pinning/ui_pin_*` on H-002.
- Review: PKG reviewer confirms pins reference Package identities, not paths.

#### Evidence
- none

### WIN-045 · Apply native per-display scaling to Windows Personality Surfaces
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: WIN-054, GFX-049, UIP-029
- Baseline: §40, §49
- Invariants: I-007

Map Win32 DPI awareness onto native per-display and fractional scale so C-007 integration scoring includes scaling. GFX owns the Display; WIN owns the personality.

<!-- covers: INV-0926 -->

#### Out of scope
Fractional scaling (GFX-049). Protocol hooks (UIP-029). Linux scaling (LNX-076).

#### Acceptance criteria
- [ ] A W1 window on a HiDPI display follows the native per-display scale factor on H-004.
- [ ] Hot-plugging a second display at a different scale re-scales the Windows Surface without a nested desktop.
- [ ] Native software never sees Win32 DPI APIs (I-007).

#### Verification
- Integration: `win:tests/dpi/per_display_*` on H-002 and H-004.
- Compat: C-007 scaling integration check on H-004.

#### Evidence
- none

### WIN-046 · Launch Steam Proton titles through the Windows Personality
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: WIN-040, WIN-054, LNX-066
- Baseline: §48, §49, §56.2
- Corpora: C-007

Launch Proton prefixes for C-007 Steam titles as Windows personality Components. The Steam client stays in LNX; titles must not appear as an extra desktop.

Required by V2-G19 (W1 corpus meets its threshold): C-007 Steam titles are scored through this launch path.

#### Out of scope
Steam client and pressure-vessel (LNX-066, LNX-071). Library adoption from attached volumes (WIN-057). Compat database (WIN-067).

#### Acceptance criteria
- [ ] A named C-007 Steam title starts as a WIN prefix Component from the Steam client hosted by LNX, with native chrome.
- [ ] The title does not open a nested desktop or a second session.
- [ ] The prefix Capability set is the compatibility default, not the Steam client's authority (T-025).

#### Verification
- Integration: `win:tests/steam/proton_launch_*` on H-002.
- Compat: C-007 Steam-distributed subset on H-002.
- Review: LNX reviewer confirms the client remains a Linux-personality application.

#### Evidence
- none

### WIN-047 · Map Windows toast and balloon notifications onto the native service
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: WIN-054, APP-014, WIN-040
- Baseline: §9.1, §49
- Threats: T-025
- Invariants: I-021

Implement the Wine toast and balloon path onto the native Notify service with per-app grants. V2 C-007 integration scoring includes notifications. LNX owns the Linux half.

<!-- covers: INV-0923 -->

#### Out of scope
Notify service (APP-014). Linux notifications (LNX-065). Notification history UI (APP).

#### Acceptance criteria
- [ ] A W1 title that posts a toast appears in the native notification service on H-002.
- [ ] A prefix without `Capability<Notifications>` that posts a toast receives a typed denial and the shell shows no notification.
- [ ] Notifications survive compositor restart with the same per-app grant.

#### Verification
- Integration: `win:tests/notify/toast_*` on H-002.
- Compat: C-007 notifications integration check on H-002.
- Unit: `win:tests/notify/deny_*` on `qemu-x86_64`.

#### Evidence
- none

### WIN-048 · Verify the V2 double-click .exe Demo on a target machine
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: WIN-027, WIN-020, WIN-024, WIN-051
- Baseline: §49, §62

Verify the V2 demo: double-click a Windows `.exe` from the File Browser; native chrome, native file chooser, task switcher; no separate desktop or wizard.

#### Out of scope
Launch implementation (WIN-027). W1 thresholds (WIN-051). Gaming demo (WIN-029).

#### Acceptance criteria
- [ ] On H-002 the scripted demo double-clicks a named C-007 `.exe`, shows native chrome, uses the native chooser, and appears in the task switcher.
- [ ] The demo recording shows no wizard and no separate desktop.
- [ ] The same script is retained as a regression on H-004 and H-005.

#### Verification
- Demo: double-click `.exe` on H-002 as named by the V2 demo list.
- Integration: `win:tests/demo/v2_double_click_*` on H-002, H-004 and H-005.

#### Evidence
- none

### WIN-049 · Integrate VKD3D-Proton for Direct3D 12 translation
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: WIN-026, GFX-057
- Baseline: §48, §56.2
- Invariants: I-007, I-045

Integrate VKD3D-Proton for D3D12, split from DXVK so D3D12 regressions do not block D3D11 C-007 titles.

<!-- covers: INV-1071, INV-0904 -->

#### Out of scope
D3D9/10/11 (WIN-026). Present path (WIN-031). Native DirectX (I-007).

#### Acceptance criteria
- [ ] A named D3D12 C-007 title renders via VKD3D-Proton to a Vulkan RenderQueue on H-002.
- [ ] A D3D12 failure does not fail the D3D11 DXVK job.
- [ ] VKD3D-Proton is an immutable Package, not a runtime download.

#### Verification
- Integration: `win:tests/vkd3d/d3d12_present_*` on H-002 and H-017.
- Compat: C-007 D3D12 subset on H-002.
- Review: GFX reviewer confirms RenderQueue submission.

#### Evidence
- none

### WIN-050 · Schedule Windows Personality frames onto compositor VRR
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: WIN-031, GFX-088
- Baseline: §40, §56.2, §62

Present personality frames on the compositor VRR path so C-007's V2 HDR-or-VRR clause can be exercised. Frame scheduling stays in GFX.

<!-- covers: INV-1077, INV-0913 -->

#### Out of scope
VRR output (GFX-088). HDR metadata (WIN-032). Immediate presentation (GFX-074).

#### Acceptance criteria
- [ ] A named C-007 Gold title on H-002 presents on the VRR path; display-reported variable refresh is visible via GFX inspect.
- [ ] Windowed present on a VRR display does not disable native desktop VRR for other Surfaces.
- [ ] WIN records the present path; GFX records the schedule.

#### Verification
- Integration: `win:tests/vrr/present_*` on H-002.
- Compat: C-007 V2 HDR-or-VRR clause contribution on H-002.
- Review: GFX reviewer confirms scheduling remains in the compositor.

#### Evidence
- none

### WIN-051 · Run the W1 Corpus to the V2 Gold and Silver thresholds
- Type: build
- Milestone: V2
- Status: todo
- Size: L
- Owner: none
- Depends on: WIN-006, WIN-053, WIN-027, WIN-021, WIN-020, WIN-047, WIN-052, WIN-056, WIN-024, WIN-030, WIN-055, WIN-022, WIN-028, WIN-043, WIN-029, WIN-023, WIN-034, WIN-041, WIN-045, BLD-060
- Baseline: §48, §56.2, §62
- Benchmarks: B-027
- Corpora: C-007
- Risks: R-035

Run C-007 to its V2 Gold and Silver thresholds with integration checks. BEN publishes B-027; WIN records ratings.

<!-- covers: INV-0906, INV-1111 -->

#### Out of scope
Harness (WIN-006). B-027 tables (BEN-044). Public database (REL-015).

#### Acceptance criteria
- [ ] C-007 meets its V2 Gold and Silver thresholds on H-002, H-004 and H-005.
- [ ] Integration checks named by C-007 are scored for every entry.
- [ ] A rating sheet is committed under `reports/compat/C-007/` for the V2 run.
- [ ] B-027 V2 publish exists for Gold titles; WIN cites the B-ID only.

#### Verification
- Compat: C-007 V2 thresholds on H-002, H-004 and H-005.
- Bench: B-027 on H-002; target per register.
- Review: WIN reviewer sign-off on the committed rating sheet.

#### Evidence
- none

### WIN-052 · Map WASAPI and XAudio2 onto native low-latency audio Objects
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: WIN-054, AUD-022
- Baseline: §49, §56.2
- Invariants: I-007

Map WASAPI and XAudio2 inside the personality onto native AudioStream objects. AUD owns native objects and B-028; WIN does not state a latency number.

<!-- covers: INV-1075, INV-0913 -->

#### Out of scope
AudioStream service (AUD-022, AUD-006). B-028 publication (AUD-004, AUD-004). Linux PipeWire (LNX-033).

#### Acceptance criteria
- [ ] A W1 title playing via WASAPI or XAudio2 holds an AudioStream Capability inspectable with `os inspect`.
- [ ] A prefix without playback rights receives `Error::Rights` and allocates no stream.
- [ ] Native software never sees WASAPI types (I-007).

#### Verification
- Integration: `win:tests/audio/wasapi_play_*` on H-002.
- Compat: C-007 audio integration check on H-002.
- Bench: B-028 remains AUD/BEN; WIN cites the B-ID only.

#### Evidence
- none

### WIN-053 · Expose the Win32 API Surface required by the W1 Corpus
- Type: build
- Milestone: V2
- Status: todo
- Size: L
- Owner: none
- Depends on: WIN-054, WIN-011, WIN-038
- Baseline: §3, §48
- Corpora: C-007
- Invariants: I-007

Enable Wine's Win32 for C-007. This is not a clean-room reimplementation.

<!-- covers: INV-0094, INV-0900 -->

#### Out of scope
API survey (WIN-011). COM (WIN-022). NT objects (WIN-034). Native Win32 (I-007).

#### Acceptance criteria
- [ ] Every API the survey marked Wine-already or missing-required is enabled for C-007 titles, with missing-required items tracked as prefix bugs rather than silent stubs that return success.
- [ ] A native crate that includes `windows.h` fails ABI-003.
- [ ] C-007 productivity titles that the survey listed reach first window on H-002.

#### Verification
- Compat: C-007 productivity subset first-window on H-002.
- Unit: `win:tests/win32/stub_policy_*` on `qemu-x86_64`.
- Review: WIN reviewer confirms the enabled set matches the survey report.

#### Evidence
- none

### WIN-054 · Integrate Wine and Proton as the Windows Personality core
- Type: build
- Milestone: V2
- Status: todo
- Size: L
- Owner: none
- Depends on: WIN-036, WIN-015, WIN-004, GOV-047
- Baseline: §3, §4, §48, §56.2, §69
- Risks: R-035
- Invariants: I-007, I-025, I-096

Turn V1 bring-up into the V2 product: Win32/NT via a Wine/Proton-derived layer that feels native, still behind the §3 firewall.

<!-- covers: INV-0005, INV-0106, INV-0894, INV-1069, INV-1331 -->

#### Out of scope
Hosting Decision (WIN-013). Object-mapping Decision (WIN-036). DXVK (WIN-026). Prefix isolation (WIN-040).

#### Acceptance criteria
- [ ] A C-007 title starts through the personality core on H-002 as a Component whose native terminus objects match WIN-036.
- [ ] Native crates still cannot link Win32 or Wine (I-007).
- [ ] The shipped Wine/Proton builds match the upstream-first and clean-room policies.
- [ ] `os inspect` shows the title as a Windows-personality Component, not a generic Linux process without a personality tag.

#### Verification
- Integration: `win:tests/core/launch_component_*` on H-002.
- Review: GOV reviewer confirms licence review GOV-047 is cited.
- Compat: C-007 smoke launch on H-002.

#### Evidence
- none

### WIN-055 · Enable 32-bit Win32 binaries under the Windows Personality
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: WIN-010, WIN-054, LNX-035, LNX-015
- Baseline: §48, §56.2
- Corpora: C-007
- Risks: R-032

Implement WIN-010 when that Decision accepts 32-bit Win32 in 1.0. Many C-007 games are 32-bit. If the Decision is 64-bit-only or VIRT-only, this task is dropped with that reason in the same change.

<!-- covers: GAP-0507, INV-0914 -->

#### Out of scope
ia32 kernel Decision (LNX-015). 32-bit Decision (WIN-010). Steam multilib (LNX-086).

#### Acceptance criteria
- [ ] If WIN-010 accepts 32-bit Win32 in 1.0, a 32-bit C-007 title starts on H-016 and on H-002 and is rated by WIN-006.
- [ ] If that Decision accepts 64-bit-only or VIRT-only, Status is dropped with `Dropped because` naming the Decision in the same change.
- [ ] 32-bit execution stays inside the personality; native crates remain 64-bit-only at Layer 1.

#### Verification
- Integration: `win:tests/wow64/launch_32_*` on H-016 and H-002 when 32-bit is accepted.
- Compat: C-007 32-bit subset on H-002 when 32-bit is accepted.
- Review: LNX reviewer confirms ia32 userland is present when this task is not dropped.

#### Evidence
- none

### WIN-056 · Map XInput, DirectInput, raw input and force feedback
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: WIN-054, HW-049
- Baseline: §49, §56.2, §62
- Invariants: I-007

Map XInput, DirectInput, raw input and force feedback onto native input objects so the V2 gaming proof-of-concept has gamepad input. HW owns devices.

<!-- covers: INV-1074, INV-0913 -->

#### Out of scope
Gamepad HID enumeration (HW-049). Keyboard/mouse (WIN-023). Linux evdev (LNX-074).

#### Acceptance criteria
- [ ] A C-007 Gold title on H-002 receives XInput from a HID gamepad Capability minted by HW.
- [ ] Unplugging the gamepad removes the Capability; the title does not retain ambient `/dev/input` access.
- [ ] Native software never sees XInput types (I-007).

#### Verification
- Integration: `win:tests/xinput/gamepad_*` on H-002.
- Compat: C-007 gamepad integration check on H-002.
- Manual: force-feedback smoke on one named title on H-002 where the hardware supports it.

#### Evidence
- none

### WIN-057 · Adopt existing Steam, Wine, Proton and Bottles libraries in place
- Type: build
- Milestone: V3
- Status: todo
- Size: L
- Owner: none
- Depends on: WIN-040, WIN-046, INS-049
- Baseline: §48, §49, §63
- Threats: T-025
- Invariants: I-076

Detect Steam libraries, Wine prefixes, Proton compatdata and Bottles/Lutris configurations on attached volumes and adopt them into per-application prefixes without re-download.

<!-- covers: GAP-0421 -->

#### Out of scope
Windows user-profile import (INS-049). Steam client (LNX). Prefix layout (PKG-080).

#### Acceptance criteria
- [ ] An attached volume containing a Steam library plus Proton compatdata is detected and each adopted title becomes a prefix Component that launches without fetching the game content again.
- [ ] Adopted Bottles/Lutris prefixes become isolated prefixes; they do not share a user-wide Wine prefix (I-076).
- [ ] Adoption records a history event and is reversible without deleting the source library.

#### Verification
- Integration: `win:tests/adopt/steam_library_*` and `win:tests/adopt/bottles_*` on H-002 with a fixture volume.
- Manual: adopt a real attached Steam library on H-005 without re-download.
- Review: INS reviewer confirms coexistence with profile import.

#### Evidence
- none

### WIN-058 · Decide anti-cheat vendor engagement and required legal agreements
- Type: adr
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: WIN-002, GOV-024, WIN-062
- Baseline: §48, §56.2
- Decision: D-0336
- Invariants: I-071

Decide EAC/BattlEye agreements and whether the project entity can sign them. Options must not relax WIN-002. GOV-024 is the counterparty.

<!-- covers: GAP-0027 -->

#### Out of scope
Kernel-level policy (WIN-002). Legal entity form (GOV-024). Measured-boot research (WIN-062).

#### Acceptance criteria
- [ ] The Decision file evaluates at least: (A) no vendor deals in 1.0; (B) userspace-only titles; (C) pursue contracts, each citing WIN-002 and `reports/spikes/WIN-062.md`.
- [ ] The accepted option does not permit kernel anti-cheat drivers or bypass (I-071).
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: WIN, GOV and SEC reviewers sign off on the pull request that accepts the Decision.

#### Evidence
- none

### WIN-059 · Publish a no-circumvention DRM and anti-tamper policy
- Type: docs
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: WIN-002
- Baseline: §48, §56.2
- Invariants: I-071

Publish that compatibility work never circumvents DRM or anti-tamper, and document DMCA 1201 and EU-equivalent exposure, before public-alpha gaming claims.

<!-- covers: GAP-0026 -->

#### Out of scope
Anti-cheat policy (WIN-002). Unsupported-title matrix (WIN-083). Protected media path (MED).

#### Acceptance criteria
- [ ] A committed policy states that WIN will not circumvent DRM or anti-tamper (I-071).
- [ ] The document records DMCA 1201 and EU-equivalent exposure for gaming compatibility claims.
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: WIN and GOV reviewers sign off on the pull request that lands the policy.

#### Evidence
- none

### WIN-060 · Bridge Wine IMM32 and TSF onto the native input-method host
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: WIN-054, TXT-029, LNX-064
- Baseline: §41, §49

Bridge Wine IMM32 and TSF onto the native IME host so one IME types into every window, including Windows-personality Surfaces. Required for the V4 CJK gate. LNX names the Linux half.

#### Out of scope
IME host Component (TXT-029). Linux text-input/IBus bridge (LNX-064). CJK engines (TXT-028).

#### Acceptance criteria
- [ ] The native IME host commits text into a focused W2 Windows Surface on H-002.
- [ ] An IME Component without `Capability<TextInputFocus>` receives no key events from a Windows field.
- [ ] Switching IME engines switches input for native, Linux and Windows focused fields in one session.

#### Verification
- Integration: `win:tests/ime/imm32_commit_*` on H-002.
- Review: TXT reviewer confirms the host protocol is unchanged.
- Manual: CJK input into a W2 title on H-005.

#### Evidence
- none

### WIN-061 · Register prefix installers as launcher-visible installed applications
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: WIN-033, APP-032, PKG-023, WIN-040
- Baseline: §28, §31, §49

Implement WIN-033: a `setup.exe` that writes a prefix becomes a launcher entry and a system-history event. APP consumes the entries.

<!-- covers: INV-0931 -->

#### Out of scope
Model Decision (WIN-033). Launcher chrome (APP-032). History transport (PKG-022).

#### Acceptance criteria
- [ ] Installing a named C-008 productivity title via `setup.exe` creates a launcher entry that starts the prefix without a wizard.
- [ ] The install records a typed history event listing prefix identity and Package pins.
- [ ] Uninstall removes the launcher entry and does not delete other prefixes.

#### Verification
- Integration: `win:tests/installers/setup_launcher_*` on H-002.
- Compat: C-008 application-install subset on H-002.
- Review: APP reviewer confirms launcher identity matches the Decision.

#### Evidence
- none

### WIN-062 · Research measured-boot integrity as an anti-cheat vendor signal
- Type: spike
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: WIN-002, BOOT-034
- Baseline: §51, §56.2
- Invariants: I-071

Research whether measured boot and kernel lockdown can provide a platform-integrity signal anti-cheat vendors would accept in lieu of kernel drivers. Lockdown makes those drivers impossible; the report is honest input to gaming claims.

<!-- covers: GAP-0242 -->

#### Out of scope
Vendor contracts (WIN-058). Measured-boot implementation (BOOT-034). Anti-cheat bypass (I-071).

#### Acceptance criteria
- [ ] A report at `reports/spikes/WIN-062.md` states whether EAC and BattlEye have a documented userspace attestation path that could consume PCR quotes.
- [ ] The report records the likely outcome for 1.0 gaming claims if vendors require kernel drivers.
- [ ] The report does not prototype a driver or a bypass.

#### Verification
- Report: which vendors could accept measured-boot quotes; which still require kernel drivers; what WIN-058 may promise; what remains a nongoal.
- Review: WIN, BOOT and SEC reviewers sign off on the pull request that lands the report.

#### Evidence
- none

### WIN-063 · Capture Windows application crashes as native crash records
- Type: build
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: WIN-040, OBS-029, OBS-026
- Baseline: §24, §63
- Threats: T-023
- Invariants: I-077

Map Wine/Windows crashes, including minidump-shaped payloads, onto the native crash format with prefix identity. OBS owns the format; this is the V3 crash-reporting personality half.

Required by V3-G04 (Crash reports are symbolicated and opted in): alpha-fleet crash reports include Windows-personality crashes.

#### Out of scope
Capture format Decision (OBS-029). Intake pipeline (REL-038). Consent UI (INS-020).

#### Acceptance criteria
- [ ] A crashing W2 title produces a native crash record that names the prefix Component and does not contain disk keys or unlocked secrets (I-077).
- [ ] Minidump-shaped payloads are converted or attached per OBS-029 without a second dump format in REL intake.
- [ ] A crash in prefix A cannot attach prefix B's memory.

#### Verification
- Integration: `win:tests/crash/minidump_map_*` on H-002.
- Review: OBS reviewer confirms the record matches the accepted format.
- Unit: `win:tests/crash/no_secrets_*` on `qemu-x86_64`.

#### Evidence
- none

### WIN-064 · Emit typed failure reasons when a PE binary cannot run
- Type: build
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: WIN-038, WIN-002, WIN-018
- Baseline: §49, §63

Emit a personality-authored reason (anti-cheat, architecture, missing runtime) when a PE binary cannot run, so a failed double-click is not silent. WIN owns the reason; REL owns triage UX; VIRT owns the VM offer.

<!-- covers: EXTRA-056 -->

#### Out of scope
Triage UX and public database (REL-015, REL-022). VM offer (VIRT-013). Loader (WIN-038).

#### Acceptance criteria
- [ ] Double-clicking a kernel-anti-cheat-excluded title returns a typed reason `anti-cheat` matching WIN-018.
- [ ] A 32-bit binary on a 64-bit-only Decision returns typed reason `architecture`.
- [ ] A missing redistributable returns typed reason `runtime` naming the Package identity if known.

#### Verification
- Unit: `win:tests/launch/failure_reason_*` on `qemu-x86_64`.
- Integration: failed double-click on H-002 produces the reason consumed by VIRT-013 tests.
- Review: REL reviewer confirms the reason schema matches triage intake.

#### Evidence
- none

### WIN-065 · Write the Windows Personality threat addendum for the external audit
- Type: docs
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: SEC-002, WIN-040, WIN-038
- Baseline: §3, §9.1, §48, §51
- Threats: T-011, T-015, T-025, T-026
- Invariants: I-007, I-076

Document prefix isolation, PE parsing, GPU/CPU side channels and the §3 firewall for the V4 external audit of personalities. SEC owns the V0 threat model; this is the Windows addendum.

Required by V4-G04 (External security audit High and Critical closed): the audit covers personalities.

#### Out of scope
System threat model (SEC-002). Audit fix-out (WIN-073). Linux personality addendum (LNX).

#### Acceptance criteria
- [ ] A committed addendum cites T-011, T-015, T-025 and T-026 against prefix isolation, PE parsing and GPU/CPU side channels.
- [ ] The addendum restates that native software never sees Win32 (I-007) and that prefixes do not share credentials (I-076).
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: WIN and SEC reviewers sign off on the pull request that lands the addendum.

#### Evidence
- none

### WIN-066 · Bound Windows services, tasks and autostart to Component lifecycle
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: WIN-040, WIN-041
- Baseline: §10, §21, §48
- Threats: T-025
- Invariants: I-031, I-076

Keep Windows background services, scheduled tasks and autostart entries inside a prefix, dying with the owning Component. No user-wide svchost.

<!-- covers: EXTRA-030 -->

#### Out of scope
Prefix isolation (WIN-040). Native supervision (SVC). Installer registration (WIN-061).

#### Acceptance criteria
- [ ] An autostart entry in prefix A starts only when that Component starts and exits when the Component is cancelled.
- [ ] A Windows service created by prefix A is not visible to prefix B and does not survive A (I-031, I-076).
- [ ] `os inspect` lists prefix services as children of the Component TaskGroup.

#### Verification
- Unit: `win:tests/lifecycle/autostart_*`, `win:tests/lifecycle/service_die_*` on `qemu-x86_64`.
- Integration: C-008 title with a helper service on H-002.
- Review: CMP reviewer confirms TaskGroup cancellation tears helpers down.

#### Evidence
- none

### WIN-067 · Ship a Proton-style per-application compatibility database
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: WIN-070, WIN-044
- Baseline: §48
- Corpora: C-008

Ship a per-game/per-app compatibility configuration database consumed by launch and surfaced in UI, feeding REL's public corpus.

<!-- covers: INV-0910 -->

#### Out of scope
Runtime pin UI (WIN-044). Public corpus publication (REL-015). Title reports (WIN-068).

#### Acceptance criteria
- [ ] Each C-008 entry can carry pin, environment and workaround keys consumed at launch.
- [ ] UI surfaces the active configuration for a title without requiring a wizard.
- [ ] Database contents are content-addressed and versioned with the generation.

#### Verification
- Integration: `win:tests/compatdb/launch_pin_*` on H-002.
- Compat: C-008 entries with non-default pins on H-002.
- Review: REL reviewer confirms export schema for the public database.

#### Evidence
- none

### WIN-068 · Publish per-title W2 reports with the public rating scale
- Type: docs
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: WIN-071, REL-015
- Baseline: §63
- Corpora: C-008

Publish a public per-title report with the Platinum/Gold/Silver/Bronze/Broken scale for the V3 gate. REL owns the community database; WIN authors the W2 results.

<!-- covers: INV-1246 -->

#### Out of scope
W2 gate run (WIN-071). Community submissions (REL-022). Corpus publication plumbing (REL-015).

#### Acceptance criteria
- [ ] Every C-008 entry has a public report naming rating, integration checks and scenario script identity.
- [ ] Reports are reproducible from the committed scenario scripts.
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Compat: C-008 public report set matches the V3 gate sheet.
- Review: WIN and REL reviewers sign off on the pull request that publishes the reports.

#### Evidence
- none

### WIN-069 · Verify V3 W2 thresholds and the Windows game-on-laptop Demo
- Type: build
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: WIN-071, WIN-072
- Baseline: §63
- Corpora: C-008

Verify the V3 demo: a Windows game and a Linux IDE on the AMD laptop, on battery, with a Bluetooth headset. Wire W2-gate evidence to the milestone demo.

#### Out of scope
W2 thresholds (WIN-071). Passthrough completeness (WIN-072). Linux IDE (LNX). Headset routing (AUD-021).

#### Acceptance criteria
- [ ] On H-005 a named C-008 Gold game runs with headset audio while a Linux IDE runs in the same session on battery.
- [ ] C-008 V3 threshold evidence is linked from the demo notes.
- [ ] The demo shows native chrome, not a nested desktop.

#### Verification
- Demo: Windows game plus Linux IDE on H-005 with Bluetooth headset.
- Compat: C-008 V3 threshold citation from WIN-071.

#### Evidence
- none

### WIN-070 · Define the W2 Corpus of 150 titles and scenario scripts
- Type: docs
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: WIN-051
- Baseline: §48, §63
- Corpora: C-008
- Invariants: I-071

Define C-008: 150 titles (110 games without kernel anti-cheat, 40 applications) with scenario scripts.

Required by V3-G19 (W2 corpus meets its threshold): WIN-071 runs the corpus this task defines.

#### Out of scope
W2 gate run (WIN-071). W1 definition (WIN-009). Public reports (WIN-068).

#### Acceptance criteria
- [ ] `registers/corpora.md` C-008 lists 150 named titles matching the register Size clause, each with a scenario script.
- [ ] No listed game requires kernel-level anti-cheat (I-071).
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: WIN and BEN reviewers sign off that C-008 matches Size, Scale and V3 threshold clauses.
- Compat: C-008 membership includes C-007 holdovers plus new titles.

#### Evidence
- none

### WIN-071 · Run the W2 Corpus to the V3 Gold and Silver thresholds
- Type: build
- Milestone: V3
- Status: todo
- Size: L
- Owner: none
- Depends on: WIN-070, WIN-072, WIN-006, WIN-030, WIN-061
- Baseline: §48, §63
- Benchmarks: B-027
- Corpora: C-008

Run C-008 to its V3 Gold and Silver thresholds. This is meaningful Windows compatibility. BEN re-measures B-027 on W2 Gold titles.

<!-- covers: INV-1246 -->

#### Out of scope
Corpus definition (WIN-070). Per-title public reports (WIN-068). B-027 publication (BEN-044).

#### Acceptance criteria
- [ ] C-008 meets its V3 Gold and Silver thresholds on the V3 Tier 1 set.
- [ ] A rating sheet is committed under `reports/compat/C-008/`.
- [ ] B-027 V3 publish exists for W2 Gold titles; WIN cites the B-ID only.

#### Verification
- Compat: C-008 V3 thresholds on H-002, H-004, H-005 and H-006.
- Bench: B-027 on W2 Gold titles; target per register.
- Review: WIN reviewer sign-off on the committed rating sheet.

#### Evidence
- none

### WIN-072 · Extend input, audio, HDR and VRR passthrough across the W2 Corpus
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: WIN-032, WIN-050, WIN-056, WIN-052, WIN-070
- Baseline: §48, §56.2, §63
- Corpora: C-008

Extend the V2 proof-of-concept paths across C-008 Gold titles at public-alpha quality.

<!-- covers: INV-0913 -->

#### Out of scope
V2 path builds (WIN-032, WIN-050, WIN-056, WIN-052). W2 gate (WIN-071).

#### Acceptance criteria
- [ ] Every C-008 Gold title that declares HDR, VRR, gamepad or low-latency audio in its scenario exercises the corresponding path on H-002 or H-005 as named.
- [ ] Integration scoring for those titles records the path as pass, not not-applicable, when the scenario requires it.
- [ ] Failures are prefix bugs, not silent fallback to a nested desktop.

#### Verification
- Compat: C-008 Gold passthrough subset on H-002 and H-005.
- Integration: `win:tests/passthrough/w2_*` on H-002.

#### Evidence
- none

### WIN-073 · Close High and Critical Windows Personality findings from the audit
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: WIN-065, WIN-040, WIN-039, SEC-067
- Baseline: §9.1, §48, §51
- Threats: T-011, T-025

Close High and Critical Windows-personality findings from the V4 external security audit of personalities, then re-verify. SEC owns the audit programme; WIN owns Windows findings.

#### Out of scope
Auditor re-verify process (SEC-069). Medium triage (SEC-068). Linux findings (LNX-103).

#### Acceptance criteria
- [ ] Every High and Critical finding tagged Windows personality has a fix landed and a regression test.
- [ ] SEC-069 records auditor agreement for those findings.
- [ ] PE fuzz and prefix isolation tests remain green on H-001 and H-002.

#### Verification
- Review: SEC and WIN reviewers sign off against the auditor list.
- Unit: new `win:tests/audit/*` regressions on `qemu-x86_64`.
- Fuzz: `win:fuzz/pe_loader` nightly remains panic-free.

#### Evidence
- none

### WIN-074 · Export Corpus ratings in machine-readable form for third parties
- Type: build
- Milestone: V4
- Status: todo
- Size: S
- Owner: none
- Depends on: WIN-080, REL-015
- Baseline: §63
- Corpora: C-007, C-008, C-009

Export C-007/C-008/C-009 ratings and scenario scripts in machine-readable form so third parties can reproduce V4 pass rates.

#### Out of scope
Public database hosting (REL-015). W3 gate run (WIN-080). Linux export (LNX-106).

#### Acceptance criteria
- [ ] A versioned export lists every C-009 entry, rating, integration checks and scenario script path.
- [ ] A third-party reproduction using the published scripts is documented as a procedure, not as a promised service.
- [ ] The export contains no performance numbers; it cites B-027 reports by path.

#### Verification
- Compat: C-009 export schema review against the V4 gate sheet.
- Review: REL reviewer confirms the export is the Windows half of the published corpus.

#### Evidence
- none

### WIN-075 · Reproduce Wine, DXVK and VKD3D-Proton builds bit-for-bit
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: WIN-012, BLD-077, WIN-026, WIN-049
- Baseline: §28, §51
- Invariants: I-036

Make Wine, DXVK and VKD3D-Proton Packages reproduce on two builders. BLD owns the verifier; WIN makes these personality runtimes reproduce.

#### Out of scope
Generation-wide repro (BLD-077). Clean-room lint (WIN-004). Corresponding source (WIN-007).

#### Acceptance criteria
- [ ] Wine, DXVK and VKD3D-Proton Packages built on two independent builders have identical content identities.
- [ ] A WIN patch that breaks reproducibility fails the PKG identity CI check.
- [ ] Reproduced Packages still pass WIN-004.

#### Verification
- Integration: two-builder identity check on BLD runners for the three runtimes.
- Review: BLD reviewer confirms the verifier consumes these Packages.

#### Evidence
- none

### WIN-076 · Record the Windows Personality feature freeze for 1.x
- Type: docs
- Milestone: V4
- Status: todo
- Size: S
- Owner: none
- Depends on: WIN-080, WIN-036, WIN-035, WIN-013
- Baseline: §48, §66
- Invariants: I-040

Record the V4 feature freeze after RC1: the Win32/NT subset, Wine/Proton versions and nongoals that 1.0 will support. This is not a Layer 1 freeze.

#### Out of scope
Layer 1 freeze (ABI-049). Linux personality contracts (LNX-105). S-030 freeze owned with LNX.

#### Acceptance criteria
- [ ] A committed freeze document lists the Win32/NT subset, Wine/Proton/DXVK/VKD3D Package identities, and nongoals (kernel anti-cheat, vendor DRM, clone).
- [ ] The document states no new WIN features after RC1 except audit fixes.
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: WIN and GOV reviewers sign off on the pull request that lands the freeze record.

#### Evidence
- none

### WIN-077 · Verify V4 W3 thresholds and Gold-hold on the beta fleet
- Type: build
- Milestone: V4
- Status: todo
- Size: S
- Owner: none
- Depends on: WIN-080, WIN-078, WIN-074
- Baseline: §63
- Corpora: C-008, C-009

Wire V4 compatibility gates to verifying tasks: W3 thresholds, W2 Gold-hold, machine-readable export.

#### Out of scope
W3 run (WIN-080). Gold-hold (WIN-078). Export (WIN-074).

#### Acceptance criteria
- [ ] C-009 V4 thresholds, C-008 Gold-hold versus V3, and the machine-readable export are all present as Evidence on the verifying tasks.
- [ ] The beta-fleet note cites those reports without restating pass-rate numbers.
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Compat: C-009 V4 and C-008 Gold-hold citations.
- Review: WIN reviewer sign-off that the three V4 compatibility gates are wired.

#### Evidence
- none

### WIN-078 · Hold zero Gold-to-lower regressions on W2 entries versus V3
- Type: build
- Milestone: V4
- Status: todo
- Size: S
- Owner: none
- Depends on: WIN-080, WIN-071
- Baseline: §63
- Corpora: C-008

Hold the V4 compatibility gate: zero Gold-to-lower regressions on C-008 entries versus V3.

#### Out of scope
W3 thresholds (WIN-080). W2 V3 run (WIN-071).

#### Acceptance criteria
- [ ] C-008 meets its V4 clause of zero Gold-to-lower regressions versus the committed V3 sheet on the V4 Tier 1 set.
- [ ] Any Gold-to-lower change is either fixed or recorded as an accepted Decision exception in the release notes.
- [ ] The hold sheet is committed under `reports/compat/C-008/`.

#### Verification
- Compat: C-008 V4 Gold-hold on V4 Tier 1 machines.
- Review: WIN reviewer sign-off on the hold sheet.

#### Evidence
- none

### WIN-079 · Define the W3 Corpus of 300 titles and scenario scripts
- Type: docs
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: WIN-071
- Baseline: §48
- Corpora: C-009
- Invariants: I-071

Define C-009: 300 titles (220 games without kernel anti-cheat, 80 applications) with scenario scripts.

Required by V4-G19 (W3 corpus meets its threshold): WIN-080 runs the corpus this task defines.

#### Out of scope
W3 gate run (WIN-080). W2 definition (WIN-070). Ratings export (WIN-074).

#### Acceptance criteria
- [ ] `registers/corpora.md` C-009 lists 300 named titles matching the register Size clause, each with a scenario script.
- [ ] No listed game requires kernel-level anti-cheat (I-071).
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: WIN and BEN reviewers sign off that C-009 matches Size, Scale and V4 threshold clauses.

#### Evidence
- none

### WIN-080 · Run the W3 Corpus to the V4 Gold and Silver thresholds
- Type: build
- Milestone: V4
- Status: todo
- Size: L
- Owner: none
- Depends on: WIN-079, WIN-071, WIN-006
- Baseline: §48
- Benchmarks: B-027
- Corpora: C-009

Run C-009 to its V4 Gold and Silver thresholds with per-title public reports and third-party-reproducible scenario scripts.

#### Out of scope
Corpus definition (WIN-079). Export (WIN-074). B-027 publication (BEN-044).

#### Acceptance criteria
- [ ] C-009 meets its V4 Gold and Silver thresholds on the V4 Tier 1 set.
- [ ] Per-title public reports exist for every entry.
- [ ] B-027 V4 publish exists for W3 Gold titles; WIN cites the B-ID only.

#### Verification
- Compat: C-009 V4 thresholds on V4 Tier 1 machines.
- Bench: B-027 on W3 Gold titles; target per register.
- Review: WIN reviewer sign-off on the committed rating sheet.

#### Evidence
- none

### WIN-081 · Write the Windows Personality chapters of the compatibility guide
- Type: docs
- Milestone: 1.0
- Status: todo
- Size: M
- Owner: none
- Depends on: WIN-085, WIN-083, DOC-028
- Baseline: §48, §49, §56.5, §63
- Corpora: C-009

Author the Windows personality chapters of the 1.0 compatibility guide, including W3 how-to and the unsupported matrix. DOC owns the book.

#### Out of scope
Book pipeline (DOC-028). Unsupported statement (WIN-083). Linux chapters (LNX-099).

#### Acceptance criteria
- [ ] The 1.0 compatibility guide contains Windows personality chapters covering launch, prefixes, pinning, W3 how-to and a pointer to the unsupported matrix.
- [ ] Chapters cite C-009 and B-027 by ID and contain no unbacked superiority claim (I-061).
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: WIN and DOC reviewers sign off on the pull request that lands the chapters.

#### Evidence
- none

### WIN-082 · Hold launcher, clipboard, audio, chooser and input integration
- Type: build
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: WIN-085, WIN-024, WIN-021, WIN-020, WIN-052, WIN-047, ACC-033
- Baseline: §49
- Corpora: C-009

Hold 1.0 integration requirements for every passing Windows entry: launcher, taskbar, notifications, clipboard, audio, file chooser, input, scaling, and accessibility where the toolkit exposes it.

#### Out of scope
W3 rating hold (WIN-085). Accessibility scoring implementation (ACC-033). Shell chrome (APP).

#### Acceptance criteria
- [ ] Every C-009 entry that meets the 1.0 rating hold also meets the integration checks named by C-009, including accessibility where the toolkit exposes it.
- [ ] A regression that keeps Gold but fails chooser or clipboard is a gate failure.
- [ ] The hold sheet is committed under `reports/compat/C-009/`.

#### Verification
- Compat: C-009 1.0 integration scoring on Tier 1.
- Review: WIN and ACC reviewers sign off on accessibility-where-exposed scoring.

#### Evidence
- none

### WIN-083 · Publish the unsupported-title statement for anti-cheat and DRM
- Type: docs
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: WIN-018, WIN-059, WIN-002, WIN-080, WIN-058
- Baseline: §48, §56.2
- Invariants: I-071

Publish the 1.0 statement of what is not supported: kernel-level anti-cheat, vendor DRM, and Broken titles with reasons.

#### Out of scope
V2 excluded-title page (WIN-018). DRM policy (WIN-059). VM fallback chapter (VIRT-015).

#### Acceptance criteria
- [ ] A committed 1.0 statement lists kernel-level anti-cheat, vendor DRM, and Broken C-009 titles with reasons.
- [ ] Bypass remains a nongoal (I-071).
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: WIN, GOV and DOC reviewers sign off on the pull request that lands the statement.

#### Evidence
- none

### WIN-084 · Verify 1.0 W3 thresholds and the working-day Windows game Demo
- Type: build
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: WIN-085, WIN-082, WIN-081
- Baseline: §62, §63
- Benchmarks: B-027
- Corpora: C-009

Verify 1.0 demos: W3 coverage proof and a full working day including a Windows game with HDR and VRR.

#### Out of scope
W3 hold (WIN-085). Guide chapters (WIN-081). B-027 republication (BEN-044).

#### Acceptance criteria
- [ ] The W3 dashboard cited by WIN-085 is current on the 1.0 candidate.
- [ ] A working-day demo on a Tier 1 machine includes a Windows game with HDR and VRR and native chrome.
- [ ] The demo notes cite B-027 and C-009 by ID only.

#### Verification
- Demo: working day including a Windows game with HDR and VRR on H-002.
- Compat: C-009 1.0 threshold citation from WIN-085.
- Bench: B-027 1.0 publish cited, not restated.

#### Evidence
- none

### WIN-085 · Hold W3 Gold and Silver thresholds with zero V4 Gold regressions
- Type: build
- Milestone: 1.0
- Status: todo
- Size: M
- Owner: none
- Depends on: WIN-080, WIN-078, BEN-044
- Baseline: §48
- Benchmarks: B-027
- Corpora: C-009

Hold the 1.0 compatibility gate: C-009 meets its 1.0 thresholds on Tier 1 with zero Gold-to-lower versus V4. BEN republishes B-027.

#### Out of scope
V4 W3 run (WIN-080). Integration hold (WIN-082). B-027 harness (BEN-044).

#### Acceptance criteria
- [ ] C-009 meets its 1.0 Gold and Silver thresholds on every Tier 1 machine in hardware scope.
- [ ] Zero Gold-to-lower versus the V4 sheet, or an accepted Decision exception in the release notes.
- [ ] B-027 1.0 publish exists for W3 Gold titles; WIN cites the B-ID only.

#### Verification
- Compat: C-009 1.0 thresholds and Gold-hold versus V4 on Tier 1.
- Bench: B-027 on W3 Gold titles; target per register.
- Review: WIN reviewer sign-off on the committed 1.0 sheet.

#### Evidence
- none
