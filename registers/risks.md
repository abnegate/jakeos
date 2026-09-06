# Risk register

This register holds every ladder risk, plus structural OS-project risks and risk-shaped gaps. It is owned by GOV. Likelihood and Impact are estimates, not measurements. `Mitigated by` stays `none` until tasks exist. `Retire by` is the milestone whose gates are threatened if the risk is still open.

### R-001 · Wrapper component creation misses the creation-cost band
- Likelihood: high
- Impact: high
- Status: open
- Mitigated by: BEN-001, CMP-001, CMP-005, CMP-010, CMP-015, CMP-016, CMP-034
- Retire by: V0
Wrapping `task_struct`, `mm_struct` and cgroups may make component creation far slower than the B-001 band, forcing earlier native implementation than planned. The V0 gate is publish-only; if the published p50 exceeds the register's V0 advisory band an accepted decision documenting root cause is required.

### R-002 · Rust-for-Linux toolchain and in-kernel API remain unstable
- Likelihood: high
- Impact: high
- Status: open
- Mitigated by: BLD-013, KRN-004, KRN-010, KRN-018, KRN-060
- Retire by: V0
Bindings to core subsystems may be missing. Q-051 records the upstream minimum-Rust question.

### R-003 · Capability encoding and handle-table design churn
- Likelihood: high
- Impact: critical
- Status: open
- Mitigated by: ABI-005, ABI-008, ABI-010, CAP-004, CAP-005, CAP-008, CAP-009, CAP-010, CAP-013
- Retire by: V0
The ABI must stay tiny yet leave room for hardware enforcement. Churn here delays every downstream workstream.

### R-004 · Native hooks silently regress the Linux syscall path
- Likelihood: high
- Impact: critical
- Status: open
- Mitigated by: ABI-002, KRN-014, LNX-001, LNX-002, LNX-041, SCH-005, SCH-010
- Retire by: V0
Keeping the Linux syscall path fully intact while inserting native scheduling and completion hooks can regress Linux behavior. C-001 is the detector.

### R-005 · Interface-evolution rules are under-designed
- Likelihood: medium
- Impact: critical
- Status: open
- Mitigated by: ABI-037, IPC-002, IPC-019, IPC-042, IPC-052, WASM-014
- Retire by: V1
Getting §12 wrong becomes a Layer 1 mistake. Evolution rules freeze at V1 as S-014, not at V0.

### R-006 · Scope creep toward a usable system before the execution model is proven
- Likelihood: high
- Impact: high
- Status: open
- Mitigated by: GOV-007, GOV-019
- Retire by: V0
V0 is not a desktop (I-094). Compositor, packages and installer work that lands in V0 without serving a V0 gate is this risk materialising.

### R-007 · L1 surfaces freeze before their spikes
- Likelihood: medium
- Impact: critical
- Status: open
- Mitigated by: ABI-006, ABI-008, ABI-019, ABI-020, ABI-021, ABI-027, ABI-049, CAP-031, GOV-005, HET-001, HET-002, HW-013, IPC-041, IPC-064, MEM-017, MEM-035, SCH-003, SCH-004, SCH-030, TSK-004, TSK-007, TSK-009, TSK-014, TSK-015, TSK-016, TSK-042
- Retire by: V0
I-040 forbids an L1 freeze before V4. V0 prototypes; it does not freeze.

### R-008 · Threat model missing while CAP, SEC and BOOT designs fix
- Likelihood: medium
- Impact: high
- Status: open
- Mitigated by: CAP-007, GFX-011, MEM-009, SEC-002, SEC-077
- Retire by: V0
The threat model is a V0 docs task. Every CAP, SEC and BOOT decision cites T-IDs.

### R-009 · Benchmark harness and quiet fleet missing at V0
- Likelihood: medium
- Impact: high
- Status: open
- Mitigated by: BEN-001, BEN-005, BEN-007, BEN-022, BEN-064, BLD-010, BLD-045, BLD-048, CMP-001, MEM-010, MEM-012, TSK-002, TSK-026, TSK-039
- Retire by: V0
V0 benchmark gates are publish-only and still need a harness, Q-001 methodology, and CI on H-001 and H-002.

### R-010 · Native init is mistaken for a V0 deliverable
- Likelihood: medium
- Impact: medium
- Status: open
- Mitigated by: BOOT-001, BOOT-004, BOOT-005, CMP-005, SVC-003, SVC-007
- Retire by: V0
V0 starts native components from a retained initramfs. Native init is V0.5.

### R-011 · Kernel developer workflow missing
- Likelihood: medium
- Impact: medium
- Status: open
- Mitigated by: BLD-008, OBS-027, SDK-038
- Retire by: V0
Without QEMU gdbstub, kgdb, early console and crash-dump analysis, V0 debugging becomes folklore.

### R-012 · Rights encoding cannot be made hardware-checkable later
- Likelihood: medium
- Impact: high
- Status: open
- Mitigated by: ABI-010, CAP-008, CAP-010, CAP-012, CAP-021
- Retire by: V1
S-003 must be a subset-checkable rights word. A software-only encoding that cannot map onto CHERI-class tags forces a Layer 1 break.

### R-013 · kselftest regressions in retained subsystems
- Likelihood: medium
- Impact: high
- Status: open
- Mitigated by: BLD-007, BLD-012, GFX-001, GFX-027, HW-005, KRN-014
- Retire by: V0
I-098 requires kselftests for every retained subsystem in the nightly matrix.

### R-014 · Windows personality has no V0 scoping decision
- Likelihood: medium
- Impact: medium
- Status: open
- Mitigated by: WIN-001
- Retire by: V0
WIN has no V0 inventory items. A scoping decision is the only V0 Windows deliverable.

### R-015 · Building a compositor and a UI toolkit at once
- Likelihood: high
- Impact: high
- Status: open
- Mitigated by: APP-002, APP-007, GFX-006, GFX-012, GFX-025, GFX-033, GFX-041, UIP-007, UIP-013, UIP-017, UIP-020, UIP-021
- Retire by: V0.5
The largest single body of new userspace code. Scope creep toward a full desktop shell is the failure mode.

### R-016 · Mesa and DRM assume Linux process and fd semantics
- Likelihood: high
- Impact: high
- Status: open
- Mitigated by: GFX-016, GFX-035, GFX-036, GFX-051, GFX-056, HET-003, HET-010, HET-015, LNX-014, LNX-031, LNX-044, MEM-019, MEM-024, MEM-025, MEM-030
- Retire by: V0.5
Adapting them to native components without forking Mesa may force a hybrid where the compositor is partly a Linux-personality process. Q-034.

### R-017 · Package manifest and capability-request schemas harden too early
- Likelihood: medium
- Impact: high
- Status: open
- Mitigated by: PKG-011, PKG-028, PKG-031, REL-001, SEC-004, SEC-007, WASM-004
- Retire by: V0.5
Layer 2 decisions that are hard to change later. Signing fields must be reserved before the first immutable install. Grant taxonomy precedes the request schema.

### R-018 · Generation switching underestimates bootloader work
- Likelihood: high
- Impact: high
- Status: open
- Mitigated by: BOOT-007, BOOT-008, BOOT-013, BOOT-014, BOOT-015, BOOT-025, BOOT-029, INS-001, INS-006, INS-013, PKG-008, PKG-015, PKG-018, PKG-020
- Retire by: V0.5
Boot-menu rollback, boot counting and ESP updates are easy to underestimate.

### R-019 · A minimal text path hardens into the permanent design
- Likelihood: medium
- Impact: high
- Status: open
- Mitigated by: TXT-003, TXT-004, TXT-008, TXT-011, UIP-023
- Retire by: V0.5
Shaping, rasterisation and IME are deep. S-016 must exist before the UI protocol freezes even if engines ship later.

### R-020 · Wayland bridge doubles the compositor test matrix
- Likelihood: high
- Impact: medium
- Status: open
- Mitigated by: GFX-020, LNX-004, LNX-006, LNX-010, LNX-040, MEM-025, UIP-029
- Retire by: V0.5
Keeping the bridge working while the native protocol evolves.

### R-021 · Accessibility tree model arrives after the UI protocol
- Likelihood: medium
- Impact: high
- Status: open
- Mitigated by: ACC-001, ACC-002, ACC-003, ACC-004, ACC-005, UIP-001
- Retire by: V0.5
The tree-model decision and metadata emission pull to V0.5. Otherwise V1 ACC work is a bolt-on.

### R-022 · Input-to-photon rig and laptop procurement miss their lead time
- Likelihood: high
- Impact: high
- Status: open
- Mitigated by: BEN-010, BEN-018, GFX-004, HW-001, HW-003, LAB-001, LAB-003, LAB-004, LAB-006, LAB-007, LAB-013, LAB-017, PWR-003, PWR-004, PWR-006, UIP-011
- Retire by: V0.5
The photodiode rig is scheduled at V0. H-004 is procured at V0.5. Power meters precede V1 battery gates. HDR display and colorimeter precede V2.

### R-023 · Supervision semantics missing for compositor rebind
- Likelihood: medium
- Impact: high
- Status: open
- Mitigated by: GFX-002, GFX-007, GFX-009, GFX-010, SVC-002, SVC-004, SVC-005, SVC-014, SVC-015, TXT-023
- Retire by: V0.5
The V0.5 compositor-restart gate depends on SVC restart budgets, rebind protocol and readiness reporting.

### R-024 · Filesystem choice includes a GPLv2-incompatible option
- Likelihood: low
- Impact: critical
- Status: open
- Mitigated by: BEN-019, STO-016, STO-026
- Retire by: V0.5
ZFS is rejected on CDDL grounds before measurement. I-044, I-067.

### R-025 · Browser and IDE personality weakness fails daily-driving
- Likelihood: high
- Impact: critical
- Status: open
- Mitigated by: APP-010, APP-019, APP-020, GFX-057, LNX-005, LNX-021, LNX-031, LNX-038, LNX-043, LNX-056, LNX-061, LNX-090
- Retire by: V1
If D-Bus, portals or GPU acceleration in the Linux personality are weak, daily-driving fails regardless of native quality. Seccomp, user namespaces, overlayfs, ptrace and file-watch must be in place at V1.

### R-026 · Self-hosting is blocked by a missing toolchain piece
- Likelihood: high
- Impact: critical
- Status: open
- Mitigated by: APP-016, APP-020, BLD-049, ENV-009, ENV-020
- Retire by: V1
Rust, LLVM and the kernel build must work under native environments.

### R-027 · A single Intel laptop hides power-management issues
- Likelihood: high
- Impact: high
- Status: open
- Mitigated by: BEN-024, HW-015, NET-009, NET-020, NET-021, NET-028, NET-037, PWR-002, PWR-004, PWR-006, PWR-009, PWR-010, PWR-014, PWR-026
- Retire by: V1
Suspend/resume and Wi-Fi roaming are hardware-specific. H-004 may be unrepresentative.

### R-028 · SDK stability pressure conflicts with ABI learning
- Likelihood: high
- Impact: high
- Status: open
- Mitigated by: ABI-034, ABI-037, SDK-049, SDK-054, SDK-059, TSK-042
- Retire by: V1
SDK v1 freezes S-014 and S-031 as candidates. Layer 1 stays unfrozen until V4.

### R-029 · Upstream rebases consume the whole KRN team
- Likelihood: high
- Impact: high
- Status: open
- Mitigated by: GOV-033, KRN-007, KRN-020, KRN-021, KRN-023, KRN-032, KRN-040
- Retire by: V1
Native hooks grow while the fork tracks at least one upstream stable release during V1.

### R-030 · Signed repository and trust roots are wrong early
- Likelihood: medium
- Impact: critical
- Status: open
- Mitigated by: APP-045, GOV-066, NET-005, NET-011, PKG-054, PKG-055, PKG-064, REL-002, REL-003, REL-007, REL-010, REL-041, REL-052
- Retire by: V1
Security-critical and easy to get wrong. Tamper rejection is a V1 gate.

### R-031 · Wine bring-up starts at V2
- Likelihood: high
- Impact: high
- Status: open
- Mitigated by: LNX-034, WIN-002, WIN-003, WIN-006, WIN-009, WIN-015
- Retire by: V1
Without non-gated V1 Wine-on-LNX bring-up, C-011 in nightly CI, the anti-cheat and 32-bit decisions, and W1 harness definition, V2 is a research programme with a hard gate.

### R-032 · ia32 emulation is pruned before Steam needs it
- Likelihood: medium
- Impact: high
- Status: open
- Mitigated by: LNX-015, LNX-021, LNX-035, LNX-061, LNX-086, WIN-010, WIN-055
- Retire by: V1
The 32-bit decision is V1. H-016 holds it in CI.

### R-033 · `std` port drives POSIX shapes into Layer 1
- Likelihood: medium
- Impact: critical
- Status: open
- Mitigated by: none
- Retire by: V1
I-013. `std` support lives at Layer 3.

### R-034 · eBPF native role is undecided while tracing ships
- Likelihood: medium
- Impact: medium
- Status: open
- Mitigated by: KRN-024, OBS-003, OBS-010, OBS-011, SCH-061
- Retire by: V1
Tracing substrate, sched_ext, network policy, and whether the Linux personality exposes `bpf()` beyond a CAP_BPF-equivalent.

### R-035 · Windows gaming is a research-scale problem
- Likelihood: high
- Impact: critical
- Status: open
- Mitigated by: BEN-041, BEN-044, VIRT-007, WIN-003, WIN-013, WIN-014, WIN-015, WIN-029, WIN-036, WIN-051, WIN-054
- Retire by: V2
Wine and Proton assume glibc and Linux process semantics, so the personality may initially be Wine inside the Linux personality and must still feel native.

### R-036 · Users judge V2 by anti-cheat and DRM titles
- Likelihood: high
- Impact: high
- Status: open
- Mitigated by: VIRT-002, VIRT-013, WIN-002, WIN-018, WIN-029
- Retire by: V2
Those titles are structurally excluded (I-071, Q-043). Communication and the VM fallback are the mitigation.

### R-037 · NVIDIA becomes a hard user expectation before it is gated
- Likelihood: high
- Impact: high
- Status: open
- Mitigated by: BLD-069, GFX-035, GFX-064, GFX-087, HET-023, HW-018, HW-044, HW-052, HW-070, VIRT-007
- Retire by: V2
H-006 is experimental at V2 and gated at V3. Messaging must match.

### R-038 · HDR color management is immature everywhere
- Likelihood: high
- Impact: high
- Status: open
- Mitigated by: GFX-062, GFX-063, GFX-068, GFX-070, GFX-073, LNX-075
- Retire by: V2
Correctness requires the LAB HDR reference display and colorimeter on H-002.

### R-039 · User-space Bluetooth is a full stack to build or port
- Likelihood: high
- Impact: high
- Status: open
- Mitigated by: AUD-017, AUD-021, HW-028, HW-035, HW-037, HW-040
- Retire by: V2

### R-040 · Laptop firmware quirks consume the HW team
- Likelihood: high
- Impact: high
- Status: open
- Mitigated by: HW-015, HW-039, HW-061, LAB-016
- Retire by: V2
Two specific machines can dominate the milestone.

### R-041 · Permissions prompts become a deny-list UX
- Likelihood: medium
- Impact: high
- Status: open
- Mitigated by: APP-025, APP-029, SEC-007, SEC-043
- Retire by: V2
Contradicts I-060.

### R-042 · Accessibility and localization are treated as bolt-ons
- Likelihood: medium
- Impact: high
- Status: open
- Mitigated by: ACC-007, ACC-016, ACC-017, ACC-019, ACC-020, ACC-021, ACC-023, ACC-025, ACC-027, ACC-028, ACC-029, ACC-031, ACC-034, APP-066, TXT-027, TXT-032, TXT-033
- Retire by: V2
They arrive here. The V0.5 tree and IME surfaces exist so this is integration, not invention.

### R-043 · Semantic registry, automation and AI broker are built out of order
- Likelihood: medium
- Impact: high
- Status: open
- Mitigated by: APP-059, GOV-042, OBS-042, SEC-034, SEM-001, SEM-004, SEM-007, SEM-009, SEM-010, SEM-013, SEM-029, SEM-031, SEM-033, WASM-015
- Retire by: V2
Dependency is registry, then automation rules, then AI broker. The AI demo depends on a done semantic-registry task.

### R-044 · Hybrid graphics and DisplayPort MST are untested
- Likelihood: medium
- Impact: medium
- Status: open
- Mitigated by: GFX-071, GFX-072, GFX-075, GFX-076, HW-048, HW-059
- Retire by: V2
Needed for the multi-monitor hot-plug and V4 laptop generations.

### R-045 · Safe-mode session is missing when the shell exhausts its budget
- Likelihood: medium
- Impact: high
- Status: open
- Mitigated by: APP-037
- Retire by: V2

### R-046 · Installer edge cases dominate support load
- Likelihood: high
- Impact: high
- Status: open
- Mitigated by: INS-008, INS-011, INS-017, INS-026, INS-027, INS-031, INS-032, INS-053
- Retire by: V3
Firmware quirks, existing partitions, dual boot, RAID and Optane.

### R-047 · Secure Boot shim review has uncertain timelines
- Likelihood: high
- Impact: high
- Status: open
- Mitigated by: BOOT-031, BOOT-036, BOOT-041, BOOT-042, BOOT-044, REL-040
- Retire by: V3
External review process. Dual path (project key enrolment) is the hedge.

### R-048 · Crash reporting creates privacy and legal obligations
- Likelihood: high
- Impact: critical
- Status: open
- Mitigated by: GOV-055, GOV-061, INS-020, OBS-029, OBS-050, REL-023, REL-038, REL-042
- Retire by: V3
Governance before code. T-023. Q-049.

### R-049 · Repository review at scale needs people, not just tooling
- Likelihood: high
- Impact: high
- Status: open
- Mitigated by: REL-021, REL-028
- Retire by: V3

### R-050 · NVIDIA driver strategy forces kernel-tree or licensing decisions
- Likelihood: high
- Impact: critical
- Status: open
- Mitigated by: GFX-035, GFX-047, GFX-064, GFX-087, HET-003, HET-023, HW-018, HW-052, HW-070, KRN-027
- Retire by: V3
Secure Boot module signing and out-of-tree modules. H-006's first gate.

### R-051 · Public exposure surfaces capability-system issues faster than they can be fixed
- Likelihood: high
- Impact: critical
- Status: open
- Mitigated by: CAP-042, SCH-054, TSK-051, WASM-010, WASM-020
- Retire by: V3
Continuous syscall and IPC fuzzing with no known open crasher older than the register's V3 window.

### R-052 · Documentation debt from V0 through V2 becomes a wall
- Likelihood: high
- Impact: high
- Status: open
- Mitigated by: DOC-002, DOC-009, DOC-010, DOC-012, DOC-023, WASM-021
- Retire by: V3
IDL-to-docs generation must exist at V1 so V3's 100% Layer 1 reference is mechanical.

### R-053 · Funding for CDN, lab, build farm and signing hardware is missing
- Likelihood: medium
- Impact: critical
- Status: open
- Mitigated by: BLD-034, BLD-053, GOV-041, REL-024, REL-031, REL-041
- Retire by: V3
Q-053.

### R-054 · ABI freeze pressure versus late beta discoveries
- Likelihood: high
- Impact: critical
- Status: open
- Mitigated by: ABI-048, ABI-049, APP-065, CAP-052, HET-026, HET-027, MEM-053, MEM-054, SCH-030
- Retire by: V4
The freeze must hold even when a cleaner design appears. A Layer 1 change after freeze is a new major version.

### R-055 · External audit findings require deep late changes
- Likelihood: medium
- Impact: critical
- Status: open
- Mitigated by: CAP-050, CMP-053, GFX-093, KRN-054, LNX-103, SEC-067, SEC-068, SEC-069, SEC-070
- Retire by: V4
Capability enforcement or personalities. All High and Critical findings fixed and re-verified.

### R-056 · Long-tail hardware bugs across ten machines multiply the matrix
- Likelihood: high
- Impact: high
- Status: open
- Mitigated by: BLD-078, HW-062, HW-080, HW-081, HW-086
- Retire by: V4

### R-057 · Fleet size and crash-free targets depend on community adoption
- Likelihood: high
- Impact: high
- Status: open
- Mitigated by: BEN-049, BEN-051, OBS-051, OBS-053, REL-042, REL-055, REL-056, REL-062, REL-066
- Retire by: V4
Cannot be scheduled. B-041 and B-042.

### R-058 · Documentation and localization are deferred until they block
- Likelihood: high
- Impact: high
- Status: open
- Mitigated by: APP-066, TXT-032, TXT-036, TXT-041, TXT-046
- Retire by: V4

### R-059 · Ecosystem thresholds depend on external interest
- Likelihood: high
- Impact: medium
- Status: open
- Mitigated by: GOV-073
- Retire by: V4
External native packages and merged contributors.

### R-060 · Reproducible builds across the inherited Linux toolchain fail
- Likelihood: high
- Impact: high
- Status: open
- Mitigated by: BLD-029, BLD-039, BLD-041, BLD-051, BLD-074, BLD-077, BLD-080, WASM-023
- Retire by: V4
Harder than for new code.

### R-061 · Support commitments convert engineering into a permanent obligation
- Likelihood: high
- Impact: critical
- Status: open
- Mitigated by: APP-067, GOV-075, GOV-081, GOV-083, KRN-055, REL-053, REL-060, REL-064
- Retire by: 1.0
The team must be staffed for the published support window before declaring. §56.4.

### R-062 · Upstream CVE cadence exceeds the SLA
- Likelihood: high
- Impact: high
- Status: open
- Mitigated by: GOV-075, KRN-030, KRN-045, KRN-052, KRN-057, MEM-055, REL-006, REL-018, REL-034, REL-039, REL-047, REL-053, REL-060, REL-064
- Retire by: 1.0
Worse if the fork has diverged in affected subsystems.

### R-063 · Last-minute driver or firmware refreshes regress hardware tests during soak
- Likelihood: medium
- Impact: high
- Status: open
- Mitigated by: APP-067, BLD-076, BLD-079, HW-087, LAB-016, LAB-024, LAB-025, REL-066
- Retire by: 1.0

### R-064 · Compatibility thresholds depend on external software that changes
- Likelihood: high
- Impact: high
- Status: open
- Mitigated by: LNX-073, LNX-084, LNX-107, LNX-110
- Retire by: 1.0
Steam, browsers, Wine upstream.

### R-065 · Reproducibility of the inherited toolchain breaks on a late dependency update
- Likelihood: medium
- Impact: high
- Status: open
- Mitigated by: BLD-074, BLD-077, BLD-080
- Retire by: 1.0

### R-066 · Governance and legal items block the declaration independently of engineering
- Likelihood: medium
- Impact: critical
- Status: open
- Mitigated by: GOV-062, GOV-080, REL-032, REL-041
- Retire by: 1.0
Trademark, licensing of inherited code, signing keys, Q-049, Q-050.

### R-067 · Hub-task fan-in makes ABI and threat-model edits semantic events
- Likelihood: high
- Impact: medium
- Status: open
- Mitigated by: ABI-011, ABI-017, ABI-027, GOV-010, GOV-011, HW-003, SEC-002
- Retire by: V0
The ABI-shape decision, IDL decision, threat model and target-hardware decision will each be depended on by hundreds of tasks. Splitting decide from implement is the mitigation.

### R-068 · Codec patents block the media framework
- Likelihood: medium
- Impact: high
- Status: open
- Mitigated by: GOV-020, MED-002, MED-004, MED-007, MED-024, MED-033
- Retire by: V2
MED packaging carries licensing metadata. GOV owns the patent position.

### R-069 · Fork divergence makes Phase D driver adaptation intractable
- Likelihood: medium
- Impact: critical
- Status: open
- Mitigated by: KRN-008, KRN-032, KRN-042, KRN-045
- Retire by: V1
Q-003, Q-051. Staffing and process for fork maintenance are documented before Phase D.

### R-070 · CI runners migrate to JakeOS before the updater exists
- Likelihood: medium
- Impact: high
- Status: open
- Mitigated by: BLD-057, BLD-064, REL-031
- Retire by: V2
A QEMU-hosted JakeOS runner is the V2 step. Hosted production CI on JakeOS waits for V3 or V4.

### R-071 · Shared writable glyph atlas becomes a cross-domain channel
- Likelihood: medium
- Impact: high
- Status: open
- Mitigated by: TXT-004, TXT-008, TXT-015, TXT-020
- Retire by: V0.5
I-083, T-030.

### R-072 · Two service managers confuse native and Linux-personality supervision
- Likelihood: medium
- Impact: high
- Status: open
- Mitigated by: LNX-029, SVC-003, SVC-026
- Retire by: V0.5
Native supervisor versus systemd expectations inside LNX. The SVC decision names both.

### R-073 · Channel backpressure is unspecified and the fast path livelocks
- Likelihood: medium
- Impact: high
- Status: open
- Mitigated by: IPC-009, OBS-007
- Retire by: V0
Bounded queue depth, sender behavior and `os inspect` exposure.

### R-074 · Kernel-object consumption per ResourceDomain is unbounded
- Likelihood: medium
- Impact: high
- Status: open
- Mitigated by: IPC-027, OBS-020, OBS-036, SCH-009
- Retire by: V0
A pids-controller equivalent with typed exhaustion errors. T-016.

### R-075 · Component panic unwinds across the ABI
- Likelihood: medium
- Impact: high
- Status: open
- Mitigated by: CMP-004, CMP-008, OBS-026, WASM-012
- Retire by: V0
A Rust panic aborts only that component. Stack overflow and OOM are typed exit causes.

### R-076 · Memory charging double-counts across ResourceDomain transfer
- Likelihood: medium
- Impact: medium
- Status: open
- Mitigated by: MEM-015
- Retire by: V0.5
Charge follows owner; a borrow never double-charges.

### R-077 · Write durability is unspecified and power-cut loses data
- Likelihood: medium
- Impact: high
- Status: open
- Mitigated by: none
- Retire by: V1
Fault-injected power-cut test on NVMe.

### R-078 · Grant continuity breaks across application updates
- Likelihood: medium
- Impact: high
- Status: open
- Mitigated by: APP-029, APP-045, CAP-043
- Retire by: V2
Grants key on package identity plus publisher, not content hash. T-033.

### R-079 · Running components observe a mixed-version tree after update
- Likelihood: medium
- Impact: high
- Status: open
- Mitigated by: APP-047, INS-009, INS-045
- Retire by: V1
T-034.

### R-080 · Side-channel position statement is missing at V1
- Likelihood: medium
- Impact: high
- Status: open
- Mitigated by: MEM-009, SEC-029, TXT-015
- Retire by: V1
T-015. Shared glyph atlas, shared MemoryObjects, SMT siblings.

### R-081 · Lab capital equipment is on every later gate's critical path
- Likelihood: high
- Impact: critical
- Status: open
- Mitigated by: LAB-002, LAB-003, LAB-007, LAB-010, LAB-018, LAB-021, LAB-023
- Retire by: V0.5
Without a LAB prefix and procurement tasks, `registers/hardware.md` has no workstream that builds what it lists. Soak scheduling, power, console and capture rigs span V1 through V3.

### R-082 · Formal entity, trademark and DCO are missing at public alpha
- Likelihood: medium
- Impact: critical
- Status: open
- Mitigated by: GOV-002, GOV-023, GOV-024, GOV-030, GOV-034, GOV-036, GOV-056, GOV-060
- Retire by: V3
Public RFC, license and trademark policy are V3 exit criteria.

### R-083 · ARM64 silently stops compiling
- Likelihood: medium
- Impact: medium
- Status: open
- Mitigated by: BLD-030, KRN-043
- Retire by: V1
I-011. A build that has become x86-only is far harder to un-break.

### R-084 · Application-state restore is promised and then found infeasible
- Likelihood: medium
- Impact: medium
- Status: open
- Mitigated by: APP-039, APP-056, ENV-028, ENV-035
- Retire by: V2
Q-056. Scope it as a 1.0 non-goal if evidence is negative rather than slipping the V2 snapshots gate.

### R-085 · Community HCL submissions leak identifiers
- Likelihood: low
- Impact: high
- Status: open
- Mitigated by: GOV-061, HW-065, HW-068, REL-048
- Retire by: V3
T-042.

<!-- roadmap:generated:begin status -->
| ID | Title | Status |
| --- | --- | --- |
| R-001 | Wrapper component creation misses the creation-cost band | open |
| R-002 | Rust-for-Linux toolchain and in-kernel API remain unstable | open |
| R-003 | Capability encoding and handle-table design churn | open |
| R-004 | Native hooks silently regress the Linux syscall path | open |
| R-005 | Interface-evolution rules are under-designed | open |
| R-006 | Scope creep toward a usable system before the execution model is proven | open |
| R-007 | L1 surfaces freeze before their spikes | open |
| R-008 | Threat model missing while CAP, SEC and BOOT designs fix | open |
| R-009 | Benchmark harness and quiet fleet missing at V0 | open |
| R-010 | Native init is mistaken for a V0 deliverable | open |
| R-011 | Kernel developer workflow missing | open |
| R-012 | Rights encoding cannot be made hardware-checkable later | open |
| R-013 | kselftest regressions in retained subsystems | open |
| R-014 | Windows personality has no V0 scoping decision | open |
| R-015 | Building a compositor and a UI toolkit at once | open |
| R-016 | Mesa and DRM assume Linux process and fd semantics | open |
| R-017 | Package manifest and capability-request schemas harden too early | open |
| R-018 | Generation switching underestimates bootloader work | open |
| R-019 | A minimal text path hardens into the permanent design | open |
| R-020 | Wayland bridge doubles the compositor test matrix | open |
| R-021 | Accessibility tree model arrives after the UI protocol | open |
| R-022 | Input-to-photon rig and laptop procurement miss their lead time | open |
| R-023 | Supervision semantics missing for compositor rebind | open |
| R-024 | Filesystem choice includes a GPLv2-incompatible option | open |
| R-025 | Browser and IDE personality weakness fails daily-driving | open |
| R-026 | Self-hosting is blocked by a missing toolchain piece | open |
| R-027 | A single Intel laptop hides power-management issues | open |
| R-028 | SDK stability pressure conflicts with ABI learning | open |
| R-029 | Upstream rebases consume the whole KRN team | open |
| R-030 | Signed repository and trust roots are wrong early | open |
| R-031 | Wine bring-up starts at V2 | open |
| R-032 | ia32 emulation is pruned before Steam needs it | open |
| R-033 | `std` port drives POSIX shapes into Layer 1 | open |
| R-034 | eBPF native role is undecided while tracing ships | open |
| R-035 | Windows gaming is a research-scale problem | open |
| R-036 | Users judge V2 by anti-cheat and DRM titles | open |
| R-037 | NVIDIA becomes a hard user expectation before it is gated | open |
| R-038 | HDR color management is immature everywhere | open |
| R-039 | User-space Bluetooth is a full stack to build or port | open |
| R-040 | Laptop firmware quirks consume the HW team | open |
| R-041 | Permissions prompts become a deny-list UX | open |
| R-042 | Accessibility and localization are treated as bolt-ons | open |
| R-043 | Semantic registry, automation and AI broker are built out of order | open |
| R-044 | Hybrid graphics and DisplayPort MST are untested | open |
| R-045 | Safe-mode session is missing when the shell exhausts its budget | open |
| R-046 | Installer edge cases dominate support load | open |
| R-047 | Secure Boot shim review has uncertain timelines | open |
| R-048 | Crash reporting creates privacy and legal obligations | open |
| R-049 | Repository review at scale needs people, not just tooling | open |
| R-050 | NVIDIA driver strategy forces kernel-tree or licensing decisions | open |
| R-051 | Public exposure surfaces capability-system issues faster than they can be fixed | open |
| R-052 | Documentation debt from V0 through V2 becomes a wall | open |
| R-053 | Funding for CDN, lab, build farm and signing hardware is missing | open |
| R-054 | ABI freeze pressure versus late beta discoveries | open |
| R-055 | External audit findings require deep late changes | open |
| R-056 | Long-tail hardware bugs across ten machines multiply the matrix | open |
| R-057 | Fleet size and crash-free targets depend on community adoption | open |
| R-058 | Documentation and localization are deferred until they block | open |
| R-059 | Ecosystem thresholds depend on external interest | open |
| R-060 | Reproducible builds across the inherited Linux toolchain fail | open |
| R-061 | Support commitments convert engineering into a permanent obligation | open |
| R-062 | Upstream CVE cadence exceeds the SLA | open |
| R-063 | Last-minute driver or firmware refreshes regress hardware tests during soak | open |
| R-064 | Compatibility thresholds depend on external software that changes | open |
| R-065 | Reproducibility of the inherited toolchain breaks on a late dependency update | open |
| R-066 | Governance and legal items block the declaration independently of engineering | open |
| R-067 | Hub-task fan-in makes ABI and threat-model edits semantic events | open |
| R-068 | Codec patents block the media framework | open |
| R-069 | Fork divergence makes Phase D driver adaptation intractable | open |
| R-070 | CI runners migrate to JakeOS before the updater exists | open |
| R-071 | Shared writable glyph atlas becomes a cross-domain channel | open |
| R-072 | Two service managers confuse native and Linux-personality supervision | open |
| R-073 | Channel backpressure is unspecified and the fast path livelocks | open |
| R-074 | Kernel-object consumption per ResourceDomain is unbounded | open |
| R-075 | Component panic unwinds across the ABI | open |
| R-076 | Memory charging double-counts across ResourceDomain transfer | open |
| R-077 | Write durability is unspecified and power-cut loses data | open |
| R-078 | Grant continuity breaks across application updates | open |
| R-079 | Running components observe a mixed-version tree after update | open |
| R-080 | Side-channel position statement is missing at V1 | open |
| R-081 | Lab capital equipment is on every later gate's critical path | open |
| R-082 | Formal entity, trademark and DCO are missing at public alpha | open |
| R-083 | ARM64 silently stops compiling | open |
| R-084 | Application-state restore is promised and then found infeasible | open |
| R-085 | Community HCL submissions leak identifiers | open |
<!-- roadmap:generated:end -->
