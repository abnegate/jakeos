I have everything I need; no further reads. Here is the critique.

# JakeOS roadmap — completeness critique

Sources read: synthesis structure (36 workstreams, task grammar, ID/status/gate rules, Rust validator), milestone ladder (V0→1.0), inventory digest (1338 items: 465 V0 / 268 V0.5 / 242 V1 / 203 V2 / 55 V3 / 1 V4 / 6 "1.0" / 98 "cross-cutting"), gap digest (554 items, 7 lenses), all 554 gap sentences, and inventory samples for ABI/KRN/CAP/SEC/SVC/TXT/ACC/INS/REL/BOOT/NET/AUD/APP/LNX/BEN plus keyword sweeps over the union of inventory+gap text.

Two bookkeeping facts the orchestrator must handle before anything else:
- The inventory uses prefixes that do not exist in the final table: `CLI` (19 items → SDK), `OPS` (30 → TSK). It has no `PWR`, `DOC`, or `TSK` items. Coverage checks must run through a prefix-remap table.
- The milestone ladder uses the token `1.0`; the structure rejects `1.0` and mandates `V5` (display title "1.0 Public Stable"). The inventory also uses `cross-cutting` (98 items), which is not a milestone token. Both must be normalised before generation (see §Generation).

---

## Missing workstreams

Verdict on the eleven proposed by the lenses, given that SVC, DOC, PWR were kept:

| Proposed | Verdict | Owner / rationale |
|---|---|---|
| **LIC** Licensing & compliance engineering | Fold, but not into GOV alone. | Policy/decisions (85 GOV-lens items, licence choices, trademark, entity, DCO/CLA, CRA/EAA) → **GOV**. Mechanised compliance (SPDX header linter, cargo-deny/licence allowlist CI gate, SBOM generation per generation, corresponding-source publication, third-party-notices bundle) → **BLD** (CI) and **REL** (publication). GOV's `## Scope` must say so explicitly or these 15-odd deliverables will be orphaned as "policy". |
| **LAB** Physical hardware lab | **Needs its own prefix.** | BLD already carries 20 inventory + 49 gap items and is toolchain-shaped; the lab is capital equipment, scheduling software (LAVA-like), power/console/capture rigs, photodiode input-to-photon rig, power meters, firmware matrix, soak runs, HDR reference display + colorimeter. It has distinct leads, budget and lead-times (procurement is on the V0.5/V1 critical path — see Ordering). BEN owns methodology and results, LAB owns the machines and rigs. Without a prefix, `registers/hardware.md` has no workstream that *builds* what it lists. |
| **SVC** System services | Kept. | Make it explicitly own: native init (PID-1 replacement), boot-success reporting to BOOT's boot counter, session-scoped service lifecycle, crash-loop safe-mode session, and the "two service managers" ADR (native supervisor vs systemd expectations inside LNX). |
| **SHL** Desktop shell & session | Fold into **APP**, but split APP's scope prose. | APP currently mixes "shell" (panel, launcher, notifications service, lock screen, greeter, quick settings, media-session, global shortcuts) with "applications" (terminal, editor, viewer). Fine as one prefix while APP has 23+77 items; add an explicit "Shell" and "Applications" sub-scope and expect a GOV split adr around V2 when the file exceeds the size warning. |
| **SYS** Core system services (settings storage, time/NTP, locale daemon, hostname, machine identity, default-app registry) | Fold into **SVC**, not APP. | These are headless typed services with no UI; APP consumes them. SVC's scope must name them or they will be invisible (0 inventory hits for NTP/timezone/hostname; they exist only in gap items). |
| **MED** Media framework (codec components, HW decode via VA-API/Vulkan Video, camera/libcamera, protected content) | **Needs its own prefix.** | No existing owner: AUD is audio-only; GFX is compositor; HW is device enablement. Codec sandboxing is the flagship §11 isolation demo (ImageDecoder generalised), HW video decode is a MemoryObject zero-copy showcase, and codec patents are a LIC dependency. Four gap items today, but a 1.0 desktop needs ~40 tasks here (pipeline ADR, per-codec packages, camera service, media-session, screen recording encode, conferencing echo-cancel). |
| **PRT** Printing & scanning | Fold into **HW** (discovery, IPP/eSCL, SANE) + **APP** (print dialog, `Capability<PrintJob>` chooser) + **LNX/WIN** (CUPS socket, Wine spooler). | Too small (5 items) for a prefix; HW's scope already says "printing". Put a "Printing" paragraph in HW scope naming the APP/LNX/WIN pieces. |
| **TEL** Telemetry & crash reporting | Fold: capture → **OBS**, client/consent UI → **INS**, intake/aggregation/dashboards/compat database → **REL**, policy → **GOV**. | The synthesis already assigned this split; the risk is that the four-way split leaves the *pipeline* task (symbol server, dedup, triage) with no owner. Assign the end-to-end pipeline tasks to REL and say so in REL's scope. |
| **DOC** | Kept. | Must also own `research/` comparative studies (§58) or GOV must; today neither directory exists in the layout. |
| **MIG** Migration & foreign-OS coexistence | Fold into **INS** (installer-side detection, shrink-alongside, BitLocker, ESP policy, live USB, unattended install, uninstall) + **STO** (NTFS/exFAT/foreign-fs degradation, SMB/NFS/WebDAV clients, storage-provider interface, three-view data mapping) + **APP** (migration assistant UI). | 81 items is a lot for folding, but the work is genuinely three-owner. INS scope must add "coexistence with foreign OSes"; STO scope must add "foreign and network filesystems". |
| **VIRT** Fallback virtualisation (KVM VM manager as component, guest tools, guest GPU, JakeOS-as-guest images) | **Needs its own prefix** at V2+, with a placeholder scope now. | KRN owns "KVM retained and exposed as Capability<VirtualMachine>" (kernel side) but nobody owns the VM manager product, guest tools, JakeOS guest images (which BLD/REL need for VM-based onboarding), or the compatibility-triage "offer the VM fallback" flow. The 1.0 non-goal list says no kernel anti-cheat — the VM fallback is the project's only answer to that user expectation, so it must be tracked. |

Also unowned in the table today (no lens proposed a prefix, but no scope line claims them): **input stack** (libinput/evdev/HID routing, keyboard layouts, gestures — split UIP for routing vs HW for devices; say so), **filesystem repair/disk health/TRIM/SMART** (STO), **firmware update service** (HW, with PKG for history events), **desktop search/indexing** (APP with STO), **eBPF's native role** (KRN/OBS decision).

Recommendation: 39 prefixes — the 36 kept plus **LAB**, **MED**, **VIRT**. Keep the alphabetical table ≤ 40; anything else folds with an explicit scope sentence.

---

## Missing requirements

Neither inventory nor gap items cover these (keyword-verified zero or off-topic hits). Format: requirement — PREFIX, earliest milestone.

Kernel / execution model
1. Define Channel<T> backpressure: bounded queue depth per channel, sender behaviour when the receiver is slow (block-as-Operation, fail, drop-by-policy), and expose depth in `os inspect` — IPC, V0.
2. Bound kernel-object consumption per ResourceDomain (handles, tasks, channels, memory objects, outstanding Operations) as a pids-controller equivalent, with typed exhaustion errors — SCH, V0.
3. Define the memory-charging rule for a MemoryObject whose ownership moves between ResourceDomains (charge follows owner; borrow charges nobody twice) and test budget enforcement across a transfer — MEM/SCH, V0.5.
4. Specify component panic/abort semantics: Rust panic in a component aborts only that component, stack-overflow and OOM-abort are reported as typed exit causes to the supervisor, no unwinding across the ABI — SDK/CMP, V0.
5. Define a typed file/directory change-notification Operation (inotify/fanotify replacement) and its personality bridge; IDEs at V1 require it — STO/TSK, V1.
6. Define mmap-style mapping of Capability<File> into a MemoryObject: coherence with Write Operations, msync equivalent, and interaction with CoW snapshots — MEM/STO, V0.5.
7. Define the durability contract of Write and StorageTransaction (when data is power-loss safe) and prove it with a fault-injected power-cut test on NVMe — STO, V1.
8. Retain overlayfs, FUSE, user namespaces and seccomp-bpf in the fork and verify them from the Linux personality at V1 (Chromium sandbox, Flatpak/bubblewrap, podman all need them) — LNX, V1 (inventory has seccomp at V2 — too late).
9. Provide ptrace-equivalent for the personality (gdb, strace, Wine debugging, Crashpad) gated by an explicit debug capability — LNX/SEC, V1.
10. Kernel developer workflow for the fork: QEMU gdbstub, kgdb, early console, crash-dump analysis with drgn, documented in one page and tested in CI — BLD, V0.
11. Retain and run kselftests for every retained subsystem in the nightly matrix — BLD, V0.
12. Hardware watchdog, softlockup/hung-task/NMI detection retained and wired to the supervisor and crash-capture path — KRN, V1.
13. Decide eBPF's native role (tracing substrate, sched_ext, network policy) and whether the Linux personality exposes bpf() beyond CAP_BPF-equivalent capability — KRN/OBS, V1.
14. Interrupt/IRQ affinity steering as a ResourceDomain CPU-policy input for LowLatency domains, measured on the audio benchmark — SCH, V2.
15. Publish a microarchitectural side-channel position statement: what isolation the capability model claims, which shared resources (glyph atlas, shared MemoryObjects, SMT siblings) are cross-domain channels, and which mitigations are on by default — SEC, V1.

Platform / desktop
16. Safe-mode session: when the shell or compositor exhausts its restart budget, SVC starts a minimal recovery shell with settings, log export and `os restore` — SVC/APP, V2.
17. Behaviour of running applications when their package is replaced by a new generation (old objects stay mapped; restart prompt; deferred activation) and test that no running component observes a mixed-version tree — PKG, V1.
18. Grant continuity across application updates: persistent capability grants key on package identity plus publisher, not content hash; revoke on publisher change — CAP/SEC, V2.
19. System CA trust store as a typed object with per-application pinning and enterprise-CA enrolment, consumed by native TLS and mirrored into the personalities' cert stores — NET/SEC, V1.
20. Remote shell access for developers at V1 (sshd via personality, later a native typed remote shell decision) with network-listen capability explicit — NET/LNX, V1.
21. Display power management (DPMS/panel self-refresh, idle dim/off) tied to idle, lock and lid state — PWR/GFX, V1.
22. Audio device hot-plug and default-device switching (headphone jack, HDMI/DP audio, USB), per-application volume mixer, Bluetooth codec selection — AUD, V1/V2.
23. Echo cancellation and noise suppression pipeline for conferencing (L3 corpus includes video conferencing) — AUD/MED, V2.
24. Hardware cursor and overlay planes, direct scanout and tearing/immediate presentation for fullscreen games and HDR video (needed for the V2 gaming and HDR gates) — GFX, V2.
25. Hybrid-graphics laptops (iGPU + dGPU, render offload, mux switching) across compositor and personalities — GFX/HW, V2 (Tier 1 V4 laptops "two generations each" will include them).
26. DisplayPort MST / daisy-chained docks in the multi-monitor hot-plug test — GFX/HW, V2.
27. Steam runtime / pressure-vessel / gamescope requirements (user namespaces, 32-bit multilib, /dev/input via udev, `SDL_GAMECONTROLLER` evdev) as a named LNX sub-corpus with its own scenario — LNX, V2 (the 32-bit decision must move to V1: syscall-pruning gap item could remove ia32 emulation).
28. Windows personality runtime redistributables (VC++ runtimes, .NET, WebView2, DirectX legacy) as per-prefix packaged dependencies rather than downloads — WIN, V2.
29. Windows background services, scheduled tasks and autostart entries inside a prefix bounded by the owning component's lifecycle — WIN, V3.
30. Personality version pinning per application (multiple Wine/Proton builds coexisting via PKG multi-version) surfaced in native UI, distinct from the per-game compat database already listed — WIN/PKG, V2.
31. Native profiler: sampling CPU/GPU profiles attributed to Task and Component (not threads), flame graphs by TaskGroup, exportable with traces — SDK/OBS, V1 (table names "profiler integration" but no item exists).
32. Interface design guidelines for IDL authors (naming, error taxonomy, async/stream patterns, capability-passing idioms) reviewed before the first L2 interface freezes — IPC/SDK, V0.5.
33. Storage quotas per user and per ResourceDomain with UI, distinct from the store GC already listed — STO, V2.
34. Disk health: TRIM/discard policy, SMART monitoring, checksum-scrub error reporting UX, and filesystem repair in the recovery environment — STO/INS, V2/V3.
35. User-data backup to external or network targets (encrypted, snapshot-based, scheduled) as the other half of "machine-to-machine restore" — STO/APP, V3.
36. Sensors scope: ambient-light auto-brightness, lid/tablet-mode switches, accelerometer rotation — declare in or out per device class — HW/PWR, V2.
37. Casting (Miracast/Chromecast/AirPlay), NFC, WWAN/eSIM, MIDI/pro-audio — declare LATER explicitly so they appear in the non-goal list — APP/AUD/HW, LATER.
38. Windows scoping adr for V0 (the V0 ladder's only WIN deliverable) — WIN has zero V0 inventory items — WIN, V0.
39. Offline on-device documentation (`os help`, man-page equivalent for the personality) — DOC/SDK, V2.
40. Kernel live-patching declared a non-goal (generations + reboot are the model) — KRN, V1 adr.

---

## Contradictions with the baseline

Against §57 (do-nots), §65 (ABI rules), §67 (principles) as reflected in the inventory:

1. **Rust `std` port over the native ABI** (BLD gap #5 "x86_64-unknown-jakeos with a std/alloc port"). `std`'s `fs`/`net`/`process` are POSIX-shaped; a port either drives POSIX shapes into Layer 1 (violates ABI rule 9 / §57 "no POSIX-shaped native API") or is a lossy façade. Require the adr to place `std` support at Layer 3 (a compatibility crate over the native SDK) and forbid any Layer 1 entry point justified by std.
2. **V0 exit "boots to a native init"** vs inventory BOOT/KRN Phase A "preserve Linux boot, initramfs handoff initially" and the BOOT-lens adr on early-userspace strategy. Either V0 boots Linux init and starts native components beside it (Phase A) or a native init is a V0 deliverable; the ladder and the divergence policy disagree. Resolve in the V0 milestone file; recommended: V0 = native components started from a retained initramfs; native init is V0.5.
3. **Semantic interfaces and AI in the same rung (V2).** §57/§67 order AI *after* the semantic object model. SEM has 27 V2 items covering registry, discovery, automation and the AI broker at once; the V2 exit demands an AI-assistant demo. Not a milestone violation, but the dependency graph must force registry → automation rules → AI broker, and the AI demo must depend on a done semantic-registry task; otherwise agents will build the broker on ad-hoc interfaces.
4. **ZFS in the filesystem benchmark** (research gap #45) vs LIC gap #23 (CDDL excluded). Keep the benchmark but the STO adr must list ZFS as rejected on licence grounds before measurement, or drop it.
5. **Unmeasured or inconsistent performance numbers.** Research gap #3 sets IPC same-core "< 1 µs" as a V0 exit criterion; the ladder sets p50 ≤ 2 µs at V1 and only "measured and published" at V0. Boot gap #74 sets "< 10 s to greeter, < 500 ms unlock". BLD gap #17 sets build-time budgets. Under BEN's anti-fake-claim policy every number must live in `registers/benchmarks.md` with a method and baseline; the V0 gate must stay "publish", V1 sets the target. Delete the "< 1 µs" V0 target.
6. **"Compositor restart under 500 ms"** and "1 GiB MemoryObject transfer under 1 ms" are targets in a milestone file; the structure says targets live only in the register. Milestone gates must cite B-IDs, not restate numbers.
7. **Shared GPU glyph atlas as a cross-component MemoryObject** (desktop gap #4) contradicts §67 principle 3 / no-ambient-authority in spirit: a shared writable atlas is a cross-domain channel and a font-parsing attack surface. Make it a read-only object minted by a text service, with the side-channel statement (missing req. 15) as a dependency.
8. **"Migrate CI runners to JakeOS by V2"** (BLD gap #68) predates the V3 installer/updater and would make the release pipeline depend on an alpha OS. Not a baseline violation but a §57-style "usable system before the model is proven" scope creep; move to V3/V4 with a QEMU-hosted JakeOS runner as the V2 step.
9. **Desktop gap #7 exposes the native font store via fontconfig and Wine font dirs** — fine — but desktop gap #33 keeps X11 primary selection "only inside the personality" while LNX inventory says Linux GUI apps use the *native* clipboard. Decide once: primary selection is emulated inside the Wayland bridge and never crosses to native.
10. **`Verified by` may never be an `@agent/` identity, yet agents generate ~2000 tasks and will be Owners.** Consistent, but V0 with `require_independent_verification = false` lets agents self-close tasks. Recommend switching the flag on for any task with `Freezes:` or Type adr even in V0 — those are the §65 "most permanent decisions".
11. **Milestone files reference paths that do not exist in the layout**: `hardware/reference.md`, `hardware/targets.md`, `compat/corpus/linux-L0.md`…`windows-W3.md`, `BENCHMARKS.md`, `decisions/` ADRs "listing two rejected options" (structure requires ≥ 2 *options*, not two *rejected*). Reconcile: `registers/hardware.md`, a new `registers/corpora.md` (C-NNN), `generated/benchmarks.md`.
12. **Gap BLD #14 "ABI snapshot check fails unless the change is linked to an approved ADR"** and structure §7 — consistent, but the ladder freezes L1 at V4 while ABI rule 6 requires "version negotiation from V0". The V0 gate must include the negotiation test (it does: unknown-field forward/backward compat) — ensure it is a *Layer 1* handshake test, not only an IDL message test.

---

## Ordering issues

**V0 is overloaded and mis-shaped.** 465 inventory items in V0, of which only 232 are deliverables; 84 are constraints, 46 "preserve X in the fork initially", 35 research studies, 35 decisions, 11 non-goals, 15 targets. Recommended normalisation before generation:
- The 46 "preserve/retain X" items collapse into two KRN tasks (retained-mechanism inventory; hardware regression matrix in BLD) plus one acceptance criterion each. Do not emit 46 tasks.
- Constraints (84) become acceptance criteria or lint tasks (ABI review gate, syscall-filter test, `#![forbid(unsafe)]` policy), not standalone tasks; ~10 lint/gate tasks total.
- Research studies (35) become GOV/DOC docs tasks under the research programme; only the six that inform V0 surfaces (Zircon, seL4 caps, io_uring lineage, Fuchsia FIDL, UMCG, CHERI escape hatches) stay in V0; the rest move to V0.5/V1 as non-gating.
- Deliverables to move out of V0 (they are not in the V0 exit criteria): OBS "every primitive observable" beyond the six `os inspect` kinds and per-domain dynamic enable (~20 items → V0.5); SCH seven intent classes (V0 needs memory budget + CPU share; Interactive/Background only → V0.5 for the rest, Realtime V1); MEM CoW/NUMA/DMA/GPU properties (→ V0.5/V1); SDK bindings beyond Rust and the C header (→ V1); IPC transport abstraction for distribution (→ LATER); CMP component graphs, warm-start latency, Inputs/Outputs manifest (→ V0.5); LNX 18 V0 items collapse to "retained syscall ABI intact + L0 corpus" (2 tasks); STO to a single "File object with Capability<File, Rights> sufficient for the derive test".
- Expected V0 after normalisation: ~180–220 tasks. V0.5 grows to ~320, V1 to ~330.

**Freeze-before-spike hazards.**
- The ABI-shape adr (entry mechanism, handle table, dispatch, error model, negotiation) is V0 and the filled example freezes `S-003` (rights encoding) in V0. Research gaps #1, 4, 5, 10, 13, 15, 18, 21 are the spikes that inform it and are unscheduled. Make them explicit V0 spike tasks with `Explores:` and put the adr's `Depends on` on all of them; freeze nothing L1 in V0 — set S-state `prototyped` and declare "Surfaces to freeze" empty for V0, freeze candidates at V1 (SDK v1), freeze at V4 per the ladder.
- Interface-versioning scheme (§12): V0 exit tests forward/backward compat, research gap #42 says evolve one real interface through three incompatible revisions first; the first real exercise is the V0.5 UI-protocol bump. Freeze L2 evolution rules at V1, not V0.
- Package manifest and capability-request schema (L2, V0.5) depend on the SEC authority-source adr (V0.5) and the grant taxonomy (Boot gap #63, unscheduled) — add the dependency chain; taxonomy adr must be V0.5.
- Accessibility tree model adr (desktop gap #17) must precede the UI protocol's L2 freeze; V0.5 exit already requires a11y metadata emission, but ACC's items are all V1. Pull the adr and the emission task to V0.5.
- IME protocol shape (preedit/commit/surrounding text) must be an S-entry before the UI protocol freezes at V1, even though IME engines ship at V2.
- Threat model: inventory has it at V0.5; BOOT-lens gap and research gap #61 say before CAP/SEC/BOOT designs are fixed. Move to V0 (it is a docs task, cheap).
- REL gap #2 "reserve signature/signer/trust-policy fields in manifests before the first immutable install" must be V0.5, not V1.

**Benchmark gates without harness.** V0 has eight benchmark gates; the BEN methodology question (§54) is V0 but the perf-CI quiet fleet (BLD gap #50), results TSDB (#51), and reference desktop procurement are unscheduled. V0.5 requires input-to-photon photodiode rig (BLD gap #53, research gap #67) — a hardware build with lead time; schedule the rig at V0. V1 battery gate requires the energy methodology (research gap #68) and power meters at V0.5. Regression gates ("no regression > 10% vs prior") need a benchmark *kind* that compares against the previous milestone's report, not a register target — add to the register schema.

**Hardware before the milestone that assumes it.** Intel laptop gates V1 (Wi-Fi roaming, 200 suspend cycles) but research gap #71 (target-hardware adr) is unscheduled and PWR has zero inventory items; laptop procurement + PWR bring-up must be V0.5. V2 needs an HDR/VRR display and a colorimeter (LAB). V3 Tier 1 adds NVIDIA and two laptops; V4 needs ten machines — lab build-out (BLD gap #55) spans V1–V3 and is on the critical path of every later gate. NVIDIA adr is HW V1 (fine) but the LIC/Secure-Boot module-signing adrs it depends on are unscheduled.

**Linux personality depth vs V1 daily-driving.** V1 exit requires 100% for browser, IDE, container runtime and Git, plus Flatpak, XWayland, Electron, portals, notifications, scaling. LNX has 36 V1 items but: seccomp/LSM/uid semantics are V2 (Chromium's sandbox needs seccomp-bpf + user namespaces at V1); overlayfs is nowhere (podman at V1); 32-bit decision is V2 (Steam at V2/L3, and a V1 syscall-pruning task could delete ia32); ptrace nowhere (VS Code debugging, Crashpad); file-watch nowhere (VS Code). Pull these to V1. Conversely, "signal delivery over native primitives" (V3) and "fork over native primitives" (V3) are correctly late.

**Windows personality cliff.** WIN: 1 item at V1, 44 at V2, yet V2 gates 50% Gold on 50 titles with HDR gaming and Bluetooth. Add V1 non-gated tasks: Wine-on-LNX bring-up, Wine test suite in nightly CI (BLD gap #32), the anti-cheat and 32-bit adrs, W1 corpus definition, per-title scenario harness. Otherwise V2 is a 44-task research programme with a hard gate.

**Other.** DOC: V3 requires 100% L1 reference pages; IDL→docs generation must exist at V1 with SDK v1. SVC: only 2 inventory items yet the V0.5 compositor-restart gate depends on supervision semantics — SVC needs ~15 V0.5 tasks. TXT: 5 items total; the V0.5 apps need shaping/rasterisation and the V4 CJK gate needs IME at V2 — plan ~40 tasks. INS: 7 items, all V2/V3, but developers "install from a scripted image" at V1 — the image builder and scripted install are V0.5/V1 INS tasks.

---

## Structure weaknesses

Not already in the synthesis's risk table:

1. **No register for compatibility corpora.** L0–L5 and W1–W3 are cited by twelve gates with thresholds, rating scales and integration checklists; nothing in the layout holds them. Add `registers/corpora.md` (C-NNN: definition, size, scenario-script alias, per-milestone threshold, rating scale) and `reports/compat/<C-NNN>/<alias>@<sha>-<H-NNN>.md`, plus Kind `compatibility` on gates. Without it the compatibility gates are un-verifiable prose.
2. **Benchmark register cannot express three gate shapes the ladder uses**: regression-vs-previous-milestone ("no regression > 10%/5%"), conditional gates ("if p50 > 500 µs an adr must be accepted"), and publish-only gates ("measured and published, no target"). Add `Target kind: absolute | regression(<milestone>, <pct>) | publish` per B-ID/milestone, and let a gate of Kind benchmark carry `Or: <adr task ID>`.
3. **No threat register.** BOOT gap #1 requires every CAP/SEC/BOOT adr to cite the threats it addresses; there is no T-NNN family, so the citation is prose and unvalidated. Add `registers/threats.md` and a `Threats:` optional field on adr tasks.
4. **`cross-cutting` and constraints have no home.** 98 inventory items and ~30 gap items are invariants, not work. The grammar has no place for "standing rule enforced by lint X" except a done docs task. Either add a `registers/invariants.md` (I-NNN with `Enforced by:` task IDs) or accept that they become acceptance criteria scattered across tasks with no index. Recommend the register; the validator can then check every I-ID has ≥ 1 enforcing task.
5. **Hub-task fan-in.** The ABI-shape adr, the IDL adr, the threat model and the target-hardware adr will each be depended on by 100–400 tasks. `generated/blocked.md` aggregated by blocker is fine, but every re-touch of a hub task's block is a semantic edit requiring Lead approval, and `impact ID` output becomes a wall of text. Add a `--depth` and `--summary` mode to `impact`, and a validator warning when fan-in exceeds N so hubs are split into "decide" and "implement" tasks.
6. **Milestone monotonicity blocks legitimate early work.** Rule "dependency rank ≤ dependent rank" forbids a V0.5 task depending on a V1 decision even when the V0.5 task is deliberately started ahead (e.g. laptop procurement V0.5 for a V1 gate — the adr is V1). In practice authors will lie about milestones to satisfy the validator. Allow `Depends on` across ranks when the dependency is Type adr/spike and flag it as a steering signal rather than an error, or force such adrs to the earlier rung (preferred, but say so in CONVENTIONS).
7. **No dropped-reason taxonomy.** With 2500 tasks over years, `Dropped because:` free text cannot answer "how many were dropped as duplicates vs descoped vs superseded". Add an enum prefix (`duplicate | descoped | superseded | infeasible | merged`).
8. **Report files scale poorly.** `reports/benchmarks/<B>/<alias>@<sha>-<H>.md` at ~40 B-IDs × 10 machines × every merge to main is tens of thousands of files in git. Commit per-milestone gate runs and nightly summaries only; point the register's `results` block at a TSDB export for the rest (BLD gap #51 already proposes the TSDB).
9. **Determinism vs `--base` checks.** The tool is "std-only, no git metadata", but `check --base origin/main` needs the base tree. Spell out that the tool shells out to `git show <ref>:<path>` (or accepts a pre-extracted directory) so CI and pre-commit behave identically; otherwise the diff-aware rules cannot run offline.
10. **Draft-ID resolution across parallel authors.** `PREFIX-@slug` is unique only within a branch. Ten agents in ten worktrees will collide on slugs like `IPC-@fast-path` when cross-referencing another agent's not-yet-existing task. The generation process (below) needs a *shared* slug registry before agents start; the tool needs `assign-ids --index <file>` that resolves cross-file drafts in one pass.
11. **No `Milestone` for LATER items' return.** `LATER` = rank ∞ means no LATER task can be depended on by anything; when a LATER item is promoted it needs a rank change on a possibly-done dependency chain. Fine, but the "2.0 planning RFC" gate at 1.0 needs a way to *cite* LATER tasks without depending on them — allow `See also:` in gate/demo blocks.
12. **Glossary casing warnings across 36 files with agent authors will be thousands of warnings** on day one; `fmt` should auto-correct casing for exact glossary matches rather than warn.
13. **`Owner: @agent/<name>`** plus "one owner per task" means after generation every task is owned by a generator agent; `roadmap unclaim --all --owner @agent/*` must exist, and the generation process must leave `Owner: none`.
14. **Size XL "must be split before leaving todo"** but generated tasks will be sized by agents who cannot estimate; expect ~30% XL. Provide a size heuristic in AGENTS.md (acceptance-criterion count, subsystem count) and a validator warning when an XL task has no `Depends on` from a split adr.
15. **Workstream `Baseline gap:` header only allowed for BOOT, NET, AUD, TXT** — but PWR, SVC, DOC, LAB, MED, VIRT also have thin or no baseline sections. Extend the list or those files cannot use `Baseline: none`.

---

## Generation process recommendations

Target: ~2000 tasks, 39 workstream files, ~10 parallel agents, validator green at the end. Every pass is a separate agent wave; the orchestrator (conductor) runs the validator between waves and never lets a wave start on red.

**Pass 0 — Normalise inputs (orchestrator, single agent, ~1 hour).**
- Build `inputs/inventory.jsonl` and `inputs/gaps.jsonl` with a stable item ID each (`INV-0001…`, `GAP-0001…`), the remapped prefix (`CLI→SDK`, `OPS→TSK`, lens-proposed `LIC/LAB/SVC/SHL/SYS/MED/PRT/TEL/DOC/MIG/VIRT` → the owners in §Missing workstreams), the normalised milestone (`1.0→V5`, `cross-cutting→` the earliest milestone where an enforcing task can exist, default V0), and a `shape` tag (`task | criterion | invariant | nongoal | question`). Apply the V0 collapse rules from §Ordering here, recording `merged-into` so coverage still resolves.
- Emit the seed for every register from these files: B-IDs from every `target`/benchmark item and every ladder benchmark gate; R-IDs from every ladder risk bullet; Q-IDs from every `question`; H-IDs from the ladder hardware paragraphs; S-IDs from every "ADR:"/"decide the shape of" item touching L1/L2; C-IDs from the corpora; T-IDs from the BOOT-lens threat list; I-IDs from constraints. Registers are written *before any task exists* so tasks can cite them.
- Write `inputs/slugs.tsv`: one line per planned task, `PREFIX-@slug  milestone  title  covers=[INV/GAP ids]`. This is the shared draft-ID namespace. Produce it by having one planning agent per workstream (in parallel, read-only) propose its slugs from its assigned items, then a merge step that rejects duplicate slugs and checks every INV/GAP id appears in ≥ 1 `covers`. Nobody writes a task until every input item maps to a slug.

**Pass 1 — Skeleton and registers (orchestrator).** Commit README, AGENTS, CONVENTIONS, GLOSSARY, BASELINE (§-numbered), roadmap.toml, fields.json, all seven milestone files with gates/demos but empty `Verified by`, all registers filled from Pass 0, D-0001, and the Rust tool. Run `roadmap check`; expect only "gate has no Verified by" and "B-ID referenced by no task" warnings. Enable `--strict` later, not now.

**Pass 2 — Task authoring (10 agents, worktrees, 3 waves of 13 files each, or 4 files per agent).** One agent owns one workstream file at a time; it may not touch any other file. Each agent writes its file in milestone chunks (V0 → V0.5 → V1 → V2+ → V3–V5/LATER) as separate appends so no single response exceeds output limits (~40 tasks × 35 lines ≈ 1400 lines per chunk). Rules given to every agent:
- Use only draft IDs from `slugs.tsv`, both for its own tasks and for `Depends on` into other files (`IPC-@ring-fast-path`), Q/B/R/S/H/C/T IDs from the registers; never invent IDs.
- Every task carries a hidden HTML comment `<!-- covers: INV-0123, GAP-0456 -->` so coverage is machine-checkable; `fmt` preserves comments.
- Type discipline: an item with "decide/ADR" → `adr` with `Decision: D-@slug` (D drafts allowed on branch); "prototype/measure/benchmark X vs Y" → `spike` with `Explores:`; a B-ID target → `benchmark`; everything else `build`/`docs`.
- §57 checklist in the prompt as hard rejection rules: no task whose acceptance criterion is "POSIX/Linux syscall X available natively"; no native filesystem/object-store build task before V5; no AI-broker task without `Depends on` a semantic-registry task; no performance number in prose — cite a B-ID; no calendar dates; no "should".
- Per-chunk self-check before returning: run `roadmap fmt && roadmap check --allow-drafts` on the single file (a tool flag the validator needs: resolve unknown draft refs against `slugs.tsv` rather than erroring).

**Pass 3 — Consolidate and assign IDs (orchestrator).** Merge the 39 branches (disjoint files; conflicts impossible except registers, which agents did not touch). Run `roadmap assign-ids --index slugs.tsv` to convert every draft to `PREFIX-NNN` across all files, decisions and milestone files in one deterministic pass (sort by milestone rank then slug). Commit. Run `roadmap check`; fix only *mechanical* errors here (dangling refs, cycles, monotonicity). Cycles and monotonicity violations are returned to the owning agent with the exact edge.

**Pass 4 — Coverage and constraint audit (3 agents, read-only, parallel).**
- Agent A: `covers` audit — every INV/GAP id appears in ≥ 1 non-dropped task; every task covers ≥ 1 item or is justified by a gate; list orphans both ways.
- Agent B: §57/§65/§67 audit — grep acceptance criteria and titles for the banned shapes above; check every `Freezes:` has a spike in closure; check every adr has ≥ 2 options; check no L1 S-ID is `frozen` before V4.
- Agent C: schedule audit — V0 task count ≤ 220; every ladder exit criterion maps to ≥ 1 task in that rung; every benchmark gate's B-ID has a harness task at or before its rung; every H-ID has a procurement/bring-up task before its first gating rung; Wave-1 missing-requirement list (above) is present.
Findings go back to owning agents as targeted edits (Pass 2 agents re-spawned with their file and the finding list).

**Pass 5 — Gates and demos (orchestrator + 1 agent per milestone, parallel).** Now that IDs exist, fill each gate's `Verified by:` and each demo's `Verified by:` by matching exit-criterion text to tasks (the coverage comments make this mechanical). Run `roadmap gate <M>` for every milestone: every gate must have ≥ 1 task; every task must be reachable from a gate (unanchored count → 0 or justified). Fill milestone `Surfaces to freeze`, `Risks to retire`, `Hardware scope` from the registers.

**Pass 6 — Register back-fill.** Fill `Mitigated by` on R-IDs, `Answered by` on Q-IDs, `Explored by/Decided by/Frozen by` on S-IDs, `Enforced by` on I-IDs from the task graph (`roadmap gen` can derive these; commit them as generated blocks, not hand-typed).

**Pass 7 — Review (2 reviewer agents, independent).** Sample 10% of tasks per workstream against the template rubric (imperative title, observable criteria, Verification kinds, correct Type conditionals, Size plausibility, `Out of scope` naming the right prefix). Reject vacuous criteria ("works correctly"). Return per-file fix lists.

**Pass 8 — Final gate.** `roadmap fmt --check && roadmap check --strict && roadmap gen --check` green; `Owner: none` everywhere (`unclaim --all`); no draft IDs; STATUS.md unanchored = 0; ROADMAP.md shows V0 active with 0% and every gate n/m populated; fixture tests for the validator pass; property tests (fmt idempotence, gen determinism across two clones) pass. Only then merge to main.

Checks to run at every wave boundary (fail-fast list): duplicate IDs; prefix ≠ file; non-ascending IDs; dangling task/D/R/B/Q/H/S/C/T refs; cycles (print SCC); monotonicity; XL without split note; adr without D file or with < 2 options; spike without `Report:` verification line; benchmark without `Bench:` line; `Freezes:` without spike in closure; "should" in criteria; dates; hand-typed percentages; task > 100 lines; file > size limit; gate `Verified by` rank > gate rank; B-ID with no task; Q open with no bound task; V0 task count; covers-orphans in both directions.

Tooling the Rust validator needs *before* Pass 2 that the synthesis did not list: `--allow-drafts --index slugs.tsv`, `assign-ids --index`, `covers` comment parsing and an orphan report, `unclaim --all`, `impact --summary`, corpora/threat/invariant registers and the three benchmark target kinds.