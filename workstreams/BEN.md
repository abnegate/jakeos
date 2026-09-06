# BEN · Benchmarks
- Prefix: BEN
- Lead: none
- Baseline: §10, §34, §53, §54, §59

<!-- roadmap:generated:begin summary -->
Tasks: 64 live, 2 done, 0 in-progress, 62 todo, 0 dropped. Ready: 3. Blocked: 59. Weighted: 3%.
<!-- roadmap:generated:end -->

## Scope

BEN owns the benchmark register, the measurement methodology, the shared runner, committed gate reports, comparison baselines, regression checks, and the rule that no performance claim ships without a harness report (I-061). It records every §54 metric as a B-ID with per-milestone target kinds `publish`, `absolute`, and `regression`, and it publishes results for gates rather than restating numbers in prose (I-088). It owns the claim-to-benchmark traceability matrix, the contributor guide for adding a metric, the time-series store with change-point detection, and the public per-commit dashboard. Energy methodology is defined here and consumed with PWR and LAB meters. Spikes in this file inform V1 absolute targets and filesystem, isolation, IPC, and startup comparisons; they do not replace per-subsystem harnesses.

V0 measures and publishes on H-001 and H-002. Later rungs add machines from the hardware register and tighten target kinds in the register. Native software is never scored against POSIX, Linux syscalls, or Win32 as if those were native APIs; Linux, Windows, macOS, containers, and language runtimes appear only as comparison baselines on the same or comparable hardware.

## Out of scope

Per-subsystem harness implementation and the code under test (CMP, TSK, IPC, MEM, SCH, OBS, GFX, AUD, PWR, PKG, STO, ENV, WASM, HET, WIN, LNX, MED, INS, SDK, SVC, BOOT, HW, APP, NET, TXT, ACC, VIRT). Quiet bare-metal performance CI fleet and merge-queue job wiring (BLD). Physical machines, photodiode rigs, power meters, and soak calendars (LAB). Dataset license for published results (GOV). Telemetry intake and HCL publication (REL, OBS). Kernel hardening configuration (KRN). Installer and updater mechanics (INS).

## Tasks

### BEN-001 · Publish V0 Component, Task and ResourceDomain cost benchmarks
- Type: benchmark
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: BEN-005, BEN-007, BEN-006, CMP-001, CMP-002, TSK-002, TSK-020, TSK-018, SCH-001, SCH-010, LAB-003
- Baseline: §10, §20, §22, §23, §54, §59
- Benchmarks: B-001, B-002, B-003, B-008, B-009, B-010, B-011, B-014
- Risks: R-001, R-009
- Invariants: I-029, I-061

V0 benchmark gates for Component creation, Task creation, idle memory, native handoff, Operation submit-to-completion, wakeup-to-run per intent class, ResourceDomain lifecycle, and concurrent Task scale are publish-only (§10, §20, §59). This task wires the CMP, TSK, and SCH harnesses through the shared runner and commits the gate reports on H-001 and H-002.

<!-- covers: INV-0228, INV-0232, INV-1019, INV-1020, INV-0378, INV-0384, INV-0359, INV-0423, INV-0445, INV-1043 -->

#### Out of scope
Harness implementation (CMP, TSK, SCH). Fast-path Component creation (CMP). V1 absolute B-001 and B-002 targets (BEN-029).

#### Acceptance criteria
- [ ] A report exists under `reports/benchmarks/` for B-001, B-002, B-003, B-008, B-009, B-010, B-011, and B-014 on H-001 and H-002 matching each register Method.
- [ ] B-014 records memory per Task and creation wall time at the live-Task scale named in the register Method, without a kernel thread per Task.
- [ ] Each cited B-ID's V0 target kind is `publish`; no absolute threshold is asserted as a V0 gate.
- [ ] If B-001's published p50 exceeds the register's V0 advisory band, the report names the Or path rather than inventing a number in prose.

#### Verification
- Bench: B-001, B-002, B-003, B-008, B-009, B-010, B-011, B-014 on H-001 and H-002; target per register.
- Review: BEN methodology sign-off recorded on the pull request.

#### Evidence
- none

### BEN-002 · Publish V0 Demo pipeline latency, copies and memory
- Type: benchmark
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: BEN-005, BEN-007, BEN-006, CMP-011, MEM-010, MEM-012, IPC-016, LAB-003
- Baseline: §17, §54, §59
- Benchmarks: B-013
- Invariants: I-061, I-063

§59 requires the V0 demo to be measured: end-to-end latency from Component A submitting on a Channel to A reading the transferred MemoryObject, copies per stage by physical-page identity, memory per Component and per Channel, creation cost of the objects in the pipeline, native handoff versus a Linux switch, and A-to-B placement on different cores and different NUMA nodes where the hardware has them.

<!-- covers: INV-0339, INV-1170, INV-1171, INV-1172, INV-1173, INV-1174 -->

#### Out of scope
Demo implementation (CMP-011). Standing IPC and MemoryObject microbenchmarks (BEN-003).

#### Acceptance criteria
- [ ] A B-013 report exists for H-001 and H-002 with end-to-end latency, copy count, and memory overhead for the payload sizes in the register Method.
- [ ] The same reports include same-core, cross-core, and cross-NUMA placements of A and B where the machine has NUMA.
- [ ] The V0 target kind is `publish`.
- [ ] No superiority claim appears outside the report tables.

#### Verification
- Bench: B-013 on H-001 and H-002; target per register.
- Demo: V0-D01 on H-002 with the published B-013 table beside the live trace.
- Review: BEN methodology sign-off recorded on the pull request.

#### Evidence
- none

### BEN-003 · Publish V0 IPC, MemoryObject and syscall-overhead benchmarks
- Type: benchmark
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: BEN-005, BEN-007, BEN-006, BEN-008, IPC-008, IPC-016, MEM-010, OBS-001, LNX-001, LAB-003
- Baseline: §15, §17, §24, §54
- Benchmarks: B-004, B-005, B-006, B-007, B-012, B-026
- Invariants: I-061

V0 gates for same-core and cross-core Channel round trip, throughput, MemoryObject transfer versus memcpy, tracing overhead on the IPC path, and fork syscall microbenchmarks versus the pinned upstream kernel are publish-only. Absolute IPC targets wait for V1. Personality productisation stays at V1; this task publishes only the fork syscall microbench rows of B-026.

<!-- covers: INV-0303, INV-0304, INV-1021, INV-1022, INV-1023, INV-1024, INV-1030, INV-0466 -->

#### Out of scope
IPC fast-path implementation (IPC). Tracing substrate (OBS). Linux personality workload overhead (BEN-027). V1 absolute B-004 and B-005 (BEN-029).

#### Acceptance criteria
- [ ] Reports exist for B-004, B-005, B-006, B-007, and B-012 on H-001 and H-002 matching each register Method, including Linux Unix-domain-socket and pipe rows on the same machine.
- [ ] B-007 covers the payload sizes named in the register and records copy count by physical-page identity.
- [ ] B-026's V0 report is the syscall microbenchmark set versus the pinned upstream kernel, not L2 corpus workloads.
- [ ] Each cited B-ID's V0 target kind is `publish`.
- [ ] No superiority claim appears outside the report tables.

#### Verification
- Bench: B-004, B-005, B-006, B-007, B-012 on H-001 and H-002; target per register. B-026 syscall microbench on H-001 and H-002; target per register.
- Review: BEN methodology sign-off recorded on the pull request.

#### Evidence
- none

### BEN-004 · Enforce claim-to-benchmark lint and traceability matrix
- Type: build
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: BEN-007
- Baseline: §53, §54, §57
- Invariants: I-009, I-050, I-052, I-061, I-088

§53, §54, and §57 collapse into one lint: no performance claim without a named B-ID and a report from its harness, no rewrite-Linux-faster framing, and benchmark priorities tied to user-perceivable outcomes. The matrix lists every architectural claim against a B-ID. CI fails a docs or task change that cites a number or an unbacked superiority claim.

<!-- covers: INV-0062, INV-0987, INV-0989, INV-1018, INV-1126 -->

#### Out of scope
Register target kinds (BEN-007). Public dashboard (BEN-031). 1.0 announcement audit (BEN-062).

#### Acceptance criteria
- [ ] A committed matrix maps every §54 claim to a B-ID in `registers/benchmarks.md`.
- [ ] CI rejects a docs, milestone, or task change that states a performance number or a superiority claim without a B-ID.
- [ ] CI rejects framing that the project is a faster Linux rewrite.
- [ ] The lint allowlist is empty except for register Method, Targets, and committed report bodies.

#### Verification
- Unit: `roadmap:tests/claim_lint_*` covering a number in prose, an unbacked superiority sentence, and a B-ID citation that passes.
- Review: GOV and BEN leads sign off on the pull request.

#### Evidence
- none

### BEN-005 · Build the shared benchmark runner and CI publication
- Type: build
- Milestone: V0
- Status: todo
- Size: L
- Owner: none
- Depends on: BEN-007
- Baseline: §54, §59
- Risks: R-009
- Invariants: I-061

V0 requires a harness and CI on QEMU plus the reference desktop; the generated results block reflects committed gate-run reports while nightly results go to the time-series export. This is the shared runner in the `bench` repository, the report skeleton under `reports/benchmarks/`, the environment record (frequency pin, mitigations, warm or cold, iteration count), and per-commit history. The public dashboard is V1.

<!-- covers: INV-1040 -->

#### Out of scope
Methodology Decision (BEN-007). CI job wiring and QEMU matrix (BLD-010, BLD-012). Public dashboard (BEN-031). Quiet fleet (BLD-048). Time-series store (BEN-032).

#### Acceptance criteria
- [ ] The `bench` runner invokes a registered harness by B-ID, records the environment fields named by BEN-007, and writes a report matching `reports/README.md`.
- [ ] A committed gate-run report on H-001 for a V0 B-ID appears in the generated results block after `roadmap gen`, without measured values in `registers/benchmarks.md` entries.
- [ ] Report paths match `reports/benchmarks/<B-NNN>/<alias>@<sha>-<H-NNN>.md`.
- [ ] A run that omits frequency pin or warm/cold labelling is rejected by the runner.

#### Verification
- Unit: `bench:tests/runner_*` for environment record, path grammar, and rejection of an unlabelled run.
- Integration: BLD-010 invokes the runner on H-001 after this task lands.
- Review: BLD lead sign-off recorded on the pull request.

#### Evidence
- none

### BEN-006 · Pin Linux comparison baseline images on reference hardware
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: BEN-007, KRN-010, LAB-003
- Baseline: §54
- Invariants: I-061

§54 requires every claim compared against Linux on identical hardware. This task pins the upstream kernel version that was forked plus a mainstream distribution userspace on H-002 (and the matching QEMU image on H-001) so V0 tables have a replayable baseline rather than a moving distro snapshot.

<!-- covers: INV-1034 -->

#### Out of scope
Windows images (BEN-047). macOS class (BEN-046). Container runtime images (BEN-020). Lab dual-boot install at V1 (LAB-012).

#### Acceptance criteria
- [ ] A committed baseline record names kernel version, userspace image hash, and boot command line for H-001 and H-002.
- [ ] Re-running any V0 harness against that record reproduces the Linux column without an unpinned `latest` tag.
- [ ] The fork's upstream version in the record matches KRN-010.

#### Verification
- Integration: one B-004 Linux-column replay on H-001 from the pinned image.
- Review: KRN and BEN leads sign off on the pull request.

#### Evidence
- none

### BEN-007 · Decide benchmark methodology and target-kind policy
- Type: adr
- Milestone: V0
- Status: done
- Size: M
- Owner: @agent/claude
- Depends on: none
- Baseline: §54, §59
- Decision: D-0031
- Risks: R-009
- Invariants: I-061, I-088
- Verified by: @jakebarnby

V0 gates stay publish-only. Numeric targets live only in `registers/benchmarks.md` with kinds `publish`, `absolute`, and `regression` versus a prior milestone. Milestone files cite B-IDs and never restate numbers. This Decision answers Q-001 (hardware list, warm and cold definitions, percentiles, iteration counts, CPU frequency pinning, mitigation settings) and rejects a numeric V0 IPC exit.

<!-- covers: INV-1042, EXTRA-061, GAP-0482 -->

#### Out of scope
Runner implementation (BEN-005). Merge-gate policy (BEN-033). Visible-UI boundary (BEN-016). Energy method (BEN-018).

#### Acceptance criteria
- [x] Option A (register target kinds `publish`, `absolute`, and `regression` versus prior milestone; V0 publish-only; numbers only in the register), option B (milestone files restate numeric exits beside B-IDs), and option C (numeric V0 exits including a same-core IPC absolute) are evaluated.
- [x] The accepted option records hardware list, warm and cold definitions, percentiles, iteration counts, frequency pinning, and mitigation settings, and answers Q-001.
- [x] The accepted option forbids restating register numbers in milestone or task prose.
- [x] A Review line names who accepts the Decision.

#### Verification
- Review: BEN and GOV leads sign off on the pull request that accepts the Decision file.

#### Evidence
- decision:D-0031

### BEN-008 · Measure native IPC against Unix socket, pipe and D-Bus
- Type: spike
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: BEN-005, BEN-007, IPC-016, IPC-015
- Baseline: §15, §53, §54
- Invariants: I-061, I-066

§53 IPC study: typed message plus ownership transfer plus scheduler-aware handoff versus serialize-syscall-copy-deserialize on Unix sockets, pipes, and D-Bus. Standing numbers live in BEN-003; this report informs V1 absolute B-004 and B-005 targets and records which expensive steps the native path removes rather than micro-optimizes.

<!-- covers: INV-1008 -->

#### Out of scope
Standing B-004/B-005 publication (BEN-003). Fast-path technique Decision (IPC). D-Bus as a native API (forbidden, LNX).

#### Acceptance criteria
- [ ] `reports/spikes/BEN-008.md` exists with the skeleton headings from `reports/README.md`.
- [ ] The Measured section names B-004 and B-005 methods and compares native Channel, Unix-domain socket, pipe, and D-Bus on H-001 and H-002.
- [ ] The Rules out section names at least one inherited IPC shape that remains expensive after measurement.

#### Verification
- Report: which inherited steps remain on the native path; whether V1 absolute B-004 and B-005 targets are still the right kinds; whether D-Bus is comparable only as a personality baseline.
- Bench: B-004 and B-005 method runs on H-001 and H-002 as input to the report.

#### Evidence
- none

### BEN-009 · Publish native application startup and input-to-photon
- Type: benchmark
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: BEN-005, BEN-007, BEN-016, CMP-019, CMP-018, APP-001, APP-004, APP-003, APP-005, LAB-001
- Baseline: §34, §54, §60
- Benchmarks: B-016, B-017, B-020
- Invariants: I-042, I-061

V0.5 gates for warm and cold launch of Terminal, Editor, Image Viewer, and the SDK sample, plus Input-to-photon latency on the four demo apps, using the visible-UI boundary from BEN-016. V0.5 is publish-only; V1 sets the B-016 absolute.

<!-- covers: INV-0635, INV-0638, INV-1025, INV-0981, INV-1192 -->

#### Out of scope
Visible-UI Decision (BEN-016). Per-app harness internals (APP, CMP). Photodiode rig (LAB). V1 absolute warm startup (BEN-030).

#### Acceptance criteria
- [ ] B-016 and B-017 reports exist for Terminal, Editor, Image Viewer, and the SDK sample on H-002, and on H-003 where the harness runs headless.
- [ ] B-020 reports exist for those applications on H-002 using the LAB photodiode rig.
- [ ] Each cited B-ID's V0.5 target kind is `publish`.
- [ ] Public material does not treat the B-016 figure as a guarantee (I-042).

#### Verification
- Bench: B-016, B-017, B-020 on H-002; B-016 and B-017 on H-003 where applicable; target per register.
- Review: BEN methodology sign-off recorded on the pull request.

#### Evidence
- none

### BEN-010 · Publish compositor frame and Input-to-photon latency
- Type: benchmark
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: BEN-005, BEN-007, GFX-003, GFX-004, GFX-006, LAB-001, LAB-004, LAB-003
- Baseline: §40, §54
- Benchmarks: B-018, B-020
- Risks: R-022
- Invariants: I-061

V0.5 gates for commit-to-scanout latency and photodiode Input-to-photon beside a Linux Wayland desktop on the same machine. Consumes LAB's V0 rig. Deadline-miss rate as a gated absolute waits for V2.

<!-- covers: INV-0739, INV-1028, INV-1029 -->

#### Out of scope
Frame-scheduling implementation (GFX). Photodiode hardware (LAB). Deadline-miss absolute (BEN-037). Wayland as a native UI API (forbidden, UIP).

#### Acceptance criteria
- [ ] B-018 reports exist for H-002 and H-003 beside a wlroots-based compositor, KWin, or Mutter on the same machine as named in the register Baselines.
- [ ] B-020 reports exist for H-002 at the display's fixed refresh using the LAB rig.
- [ ] Each cited B-ID's V0.5 target kind is `publish`.
- [ ] No superiority claim appears outside the report tables.

#### Verification
- Bench: B-018 on H-002 and H-003; B-020 on H-002; target per register.
- Review: GFX and BEN leads sign off on the pull request.

#### Evidence
- none

### BEN-011 · Publish SystemGeneration creation time and disk overhead
- Type: benchmark
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: BEN-005, BEN-007, PKG-002, PKG-001, PKG-016, BOOT-006
- Baseline: §30, §54
- Benchmarks: B-022
- Invariants: I-061

V0.5 generation-switch gates need creation time, switch time, rollback time, and per-generation store growth as standing metrics. PKG owns switch and rollback segments; BOOT owns the boot-menu rollback segment; BEN publishes B-022.

<!-- covers: INV-0570 -->

#### Out of scope
Generation compose and boot integration (PKG, BOOT). Package install time (PKG-003). Snapshot UI restore (BEN-042).

#### Acceptance criteria
- [ ] A B-022 report exists for H-001 and H-002 covering create, switch, rollback, and store growth per the register Method.
- [ ] The V0.5 target kind is `publish`.
- [ ] PKG and BOOT segment reports are cited from the B-022 report rather than duplicated as new metrics.

#### Verification
- Bench: B-022 on H-001 and H-002; target per register.
- Review: PKG and BEN leads sign off on the pull request.

#### Evidence
- none

### BEN-012 · Track kernel image size and boot time to first frame
- Type: benchmark
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: BEN-005, BEN-007
- Baseline: §30, §54
- Benchmarks: B-032, B-044
- Invariants: I-061

Cheap isolation and fast startup degrade through bloat unless kernel image size, minimal SystemGeneration size, installer image size, and boot-to-first-frame are watched. V0.5 publishes; later rungs regress against these reports.

<!-- covers: GAP-0159 -->

#### Out of scope
Bootloader and firmware path (BOOT). Installer media wall time (INS-016). Unlock-to-desktop (BEN-035).

#### Acceptance criteria
- [ ] B-044 reports exist for every CI build of main covering kernel image, minimal generation, and installer image sizes.
- [ ] B-032 reports exist for H-001 and H-002 for firmware handoff to first presented frame and to greeter-accepting-input once a greeter exists, otherwise first frame only.
- [ ] Each cited B-ID's V0.5 target kind is `publish`.

#### Verification
- Bench: B-044 on CI; B-032 on H-001 and H-002; target per register.
- Review: BEN methodology sign-off recorded on the pull request.

#### Evidence
- none

### BEN-013 · Publish Task baselines against Tokio, Go and BEAM
- Type: benchmark
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: BEN-005, BEN-007, TSK-021, TSK-019
- Baseline: §20, §54
- Benchmarks: B-002, B-003, B-009, B-014
- Invariants: I-061

§54 requires language-runtime comparison for Task creation, Operation cost, and message passing. This task pins Tokio tasks, Go goroutines, and Erlang/BEAM processes on H-001 and H-002 and publishes those rows beside the native harnesses so later B-002, B-003, B-009, and B-014 reports have a replayable runtime baseline rather than a citation of someone else's blog.

<!-- covers: INV-1038 -->

#### Out of scope
Native Task multiplexer (TSK). IPC round trip (BEN-003). Isolation versus OCI (BEN-020).

#### Acceptance criteria
- [ ] Reports under `reports/benchmarks/` for B-002, B-003, B-009, and B-014 on H-001 and H-002 include Tokio, Go, and BEAM columns measured in the same session as the native path.
- [ ] The V0 target kind for each cited B-ID is `publish`.
- [ ] No superiority claim appears outside the report tables.

#### Verification
- Bench: B-002, B-003, B-009, B-014 on H-001 and H-002; target per register.
- Review: BEN methodology sign-off recorded on the pull request.

#### Evidence
- none

### BEN-014 · Publish storage-Object access against Linux file I/O
- Type: benchmark
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: BEN-005, BEN-007, STO-020, STO-029
- Baseline: §25, §26, §54
- Benchmarks: B-037
- Invariants: I-016, I-061

§54 filesystem and object access through Capability-scoped storage objects versus direct Linux file I/O on NVMe. STO owns the service under test; BEN publishes B-037. Native software sees `Capability<File>` and never a path as authority.

<!-- covers: INV-1027 -->

#### Out of scope
Storage service (STO). Filesystem substrate Decision (STO-016). Snapshot create/restore (BEN-042). Candidate-filesystem spike (BEN-019).

#### Acceptance criteria
- [ ] A B-037 report exists for H-001 and H-002 covering File read/write, content-store open and map, and directory listing at the sizes in the register Method.
- [ ] Linux `read`/`pread`/`pwrite`, `mmap`, and io_uring rows are measured on the same volume.
- [ ] The V0.5 target kind is `publish`.

#### Verification
- Bench: B-037 on H-001 and H-002; target per register.
- Review: STO and BEN leads sign off on the pull request.

#### Evidence
- none

### BEN-015 · Verify no V0 benchmark regression at V0.5
- Type: benchmark
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: BEN-005, BEN-007, BEN-001, BEN-003, BEN-002, BEN-013
- Baseline: §54
- Benchmarks: B-051
- Invariants: I-061, I-088

V0.5 benchmark gate: every V0 B-ID with a committed report is re-run on H-001 and H-002. A regression beyond the register band versus the V0 report requires an accepted Decision; the band lives only in the register.

#### Out of scope
New V0.5 metrics (sibling BEN tasks). Merge-gate policy (BEN-033). Quiet fleet (BLD).

#### Acceptance criteria
- [ ] A B-051 report exists for H-001 and H-002 listing every V0 B-ID compared against the committed V0 report on the same hardware entry.
- [ ] Any exceedance of the V0.5 `regression` clause names an accepted Decision, or no exceedance is present.
- [ ] The runner uses the same Method and environment record as V0.

#### Verification
- Bench: B-051 on H-001 and H-002; target per register.
- Review: BEN methodology sign-off recorded on the pull request.

#### Evidence
- none

### BEN-016 · Decide the visible-UI measurement boundary
- Type: adr
- Milestone: V0.5
- Status: done
- Size: S
- Owner: @agent/claude
- Depends on: BEN-007
- Baseline: §34, §54
- Decision: D-0032
- Invariants: I-042
- Verified by: @jakebarnby

V0.5 startup gates are incomparable unless visible UI is fixed. B-016 cites Q-029; this Decision answers Q-029 so warm and cold startup numbers are comparable across applications and systems.

<!-- covers: INV-0640 -->

#### Out of scope
Startup harnesses (CMP, APP, BEN-009). Photodiode rig (LAB). Launch-path profile (BEN-021).

#### Acceptance criteria
- [x] Option A (first compositor presentation of a non-blank frame), option B (first client commit), and option C (first photodiode edge) are evaluated against comparability across apps and against B-020.
- [x] The accepted option is the measurement boundary recorded on B-016 and answers Q-029.
- [x] A Review line names who accepts the Decision.

#### Verification
- Review: GFX, APP, and BEN leads sign off on the pull request that accepts the Decision file.

#### Evidence
- decision:D-0032

### BEN-017 · Measure native startup against traditional ELF load
- Type: spike
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: BEN-005, BEN-007, BEN-016, CMP-027
- Baseline: §34, §53, §54
- Invariants: I-039, I-061, I-066

§53 startup comparison: map immutable objects and reuse verified pages versus lookup, shared-library load, reloc, and constructors. Informs B-016 and B-017 and the Package mapping path. Native launch does not grow a POSIX loader.

<!-- covers: INV-1016 -->

#### Out of scope
Launch-path stage profile (BEN-021). Standing B-016/B-017 publication (BEN-009). Linking model (SDK).

#### Acceptance criteria
- [ ] `reports/spikes/BEN-017.md` exists with the skeleton headings.
- [ ] The Measured section compares native map-and-launch against ELF load of an equivalent binary on H-002 using the B-016 boundary.
- [ ] The Rules out section names inherited startup steps the native path still pays, if any.

#### Verification
- Report: which ELF steps remain; whether Package mapping is on the critical path; what must change before V1 B-016 absolute checks.
- Bench: B-016 method runs on H-002 as input to the report.

#### Evidence
- none

### BEN-018 · Define energy measurement methodology with external meters
- Type: spike
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: BEN-007, LAB-006, LAB-007, PWR-004
- Baseline: §22, §54
- Risks: R-022
- Invariants: I-061

Energy is uniquely easy to mismeasure. This protocol covers external meter versus battery gauge versus RAPL, drain workloads, and comparison OS images on the same laptop. B-031 stays unpublished until this report exists. LAB owns meters; PWR owns the power service under test.

<!-- covers: GAP-0547 -->

#### Out of scope
Meter procurement (LAB-006). Drain-cycle wiring (LAB-013). V1 B-031 publication (BEN-024, PWR-006).

#### Acceptance criteria
- [ ] `reports/spikes/BEN-018.md` exists with the skeleton headings.
- [ ] The report chooses the primary instrument (external meter, battery gauge, or RAPL) and names when the others may be used as secondary.
- [ ] Idle, video, compile, and browsing workloads are defined so B-031 can run without inventing a method later.
- [ ] No B-031 report is committed before this report exists.

#### Verification
- Report: which instrument is primary; how comparison OS images are imaged on the same laptop; what LAB-013 must wire; confirmation that software RAPL-only evidence is insufficient for the V1 gate.
- Review: PWR and LAB leads sign off on the spike pull request.

#### Evidence
- none

### BEN-019 · Measure filesystem candidates for snapshot and clone cost
- Type: spike
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: BEN-005, BEN-007, STO-023, GOV-003
- Baseline: §26, §54, §57
- Risks: R-024
- Invariants: I-044, I-067

Informs the V0.5 STO filesystem Decision with snapshot creation, clone time, checksum overhead, and dedup on a representative system image. ZFS is rejected on license before measurement and is not a candidate. STO-026 adds crash-consistency runs on this harness.

<!-- covers: INV-0507 -->

#### Out of scope
Filesystem Decision (STO-016). Crash-consistency extension (STO-026). Native filesystem before 1.0 (forbidden, I-044).

#### Acceptance criteria
- [ ] `reports/spikes/BEN-019.md` exists with the skeleton headings.
- [ ] btrfs, bcachefs, and XFS-reflink are measured for snapshot create, clone, checksum overhead, and dedup on H-001 and H-002.
- [ ] ZFS appears only as a CDDL-excluded reference row with no benchmark run.

#### Verification
- Report: which candidate is cheapest on snapshot and clone; whether checksum overhead is visible on the representative image; confirmation that ZFS was not measured.
- Bench: candidate runs using the B-037 method family on H-001 and H-002 as input to the report.

#### Evidence
- none

### BEN-020 · Measure native isolation against OCI container start
- Type: spike
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: BEN-005, BEN-007, CMP-005, SCH-007
- Baseline: §36, §53, §54
- Invariants: I-019, I-061, I-066

§53 isolation comparison of the five-step native path (ResourceDomain, Component, capabilities, map immutable objects, schedule) against runtime-daemon, namespaces, overlay, cgroups, and process start. Pins Docker/Podman runc and crun images on the same hardware. Publish-only at V0; V1 publishes the cheaper-than-OCI claim as B-015.

<!-- covers: INV-1001, INV-0677, INV-1037 -->

#### Out of scope
Standing B-015 publication (BEN-026). OCI as a native dependency (ENV-001). Personality containers (LNX).

#### Acceptance criteria
- [ ] `reports/spikes/BEN-020.md` exists with the skeleton headings.
- [ ] The Measured section compares native ResourceDomain plus Component plus capability attachment against podman/runc, podman/crun, and systemd-nspawn on H-001 and H-002 using the B-015 method.
- [ ] Pinned OCI baseline image hashes are recorded for replay by BEN-026.

#### Verification
- Report: which native steps dominate isolation cost; whether the OCI baseline images are replayable; what remains before B-015 may be published at V1.
- Bench: B-015 method runs on H-001 and H-002 as input to the report.

#### Evidence
- none

### BEN-021 · Profile the native application launch path
- Type: spike
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: BEN-007, BEN-016, BEN-017, CMP-027, OBS-011
- Baseline: §34, §54
- Invariants: I-039, I-042

Breaks launch into Component creation, capability attachment, mapping, runtime init, and first frame so the B-016 budget is spent on a named stage rather than a single wall number. Precedes V1 absolute warm-startup checks.

<!-- covers: INV-0643 -->

#### Out of scope
Standing B-016 publication (BEN-009). ELF comparison (BEN-017). Tracing substrate (OBS).

#### Acceptance criteria
- [ ] `reports/spikes/BEN-021.md` exists with the skeleton headings.
- [ ] The Measured section attributes wall time on H-002 to named stages from launch request to the visible-UI boundary.
- [ ] The Recommends section names the stage that must move before BEN-030 can meet the V1 B-016 target kind.

#### Verification
- Report: stage breakdown for Terminal and Editor; which stage dominates; whether Component creation (B-001) is on the critical path.
- Review: CMP and APP leads sign off on the spike pull request.

#### Evidence
- none

### BEN-022 · Measure harness noise floor on QEMU versus hardware
- Type: spike
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: BEN-005, BEN-007, BEN-003, BEN-001, LAB-003
- Baseline: §54
- Risks: R-009
- Invariants: I-061

Microsecond IPC and Component-creation numbers cannot be taken from shared or virtualised runners. This report records QEMU versus H-002 noise so V1 blocking gates have a calibrated band and V0 publish-only reports stay labelled by environment.

Required by V1-G14 (Prior benchmarks show no unexplained regression): the B-051 regression band consumes the noise band this report names.

#### Out of scope
Quiet fleet (BLD-048). Merge-gate policy (BEN-033). Time-series thresholds (BEN-032).

#### Acceptance criteria
- [ ] `reports/spikes/BEN-022.md` exists with the skeleton headings.
- [ ] Repeatability of B-001 and B-004 is reported on H-001 and H-002 with the methodology's frequency pin.
- [ ] The report names a noise band that BEN-033 and BEN-032 consume, without stating that band as a milestone number in prose.

#### Verification
- Report: QEMU versus H-002 noise on B-001 and B-004; whether H-001 results may feed a blocking gate; what isolation BLD-048 must provide.
- Bench: B-001 and B-004 repeatability series on H-001 and H-002 as input to the report.

#### Evidence
- none

### BEN-023 · Publish full-image build time on the OS versus Linux
- Type: benchmark
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: BEN-005, BEN-007, BLD-032, ENV-020
- Baseline: §50, §54, §61
- Benchmarks: B-039
- Invariants: I-061

V1 benchmark gate and self-hosting exit: clean and incremental full-image build time inside `os env` versus Linux on the same hardware. BLD owns runner-side budgets; BEN publishes B-039.

#### Out of scope
Toolchain and image build (BLD). environment.yaml (ENV). Onboarding time (BEN-028).

#### Acceptance criteria
- [ ] A B-039 report exists for H-002 covering clean full-image, incremental kernel, and incremental userspace builds inside `os env` and on the Linux baseline.
- [ ] The V1 target kind is `publish`.
- [ ] No superiority claim appears outside the report tables.

#### Verification
- Bench: B-039 on H-002; target per register.
- Review: BLD and BEN leads sign off on the pull request.

#### Evidence
- none

### BEN-024 · Publish energy use on the Intel laptop
- Type: benchmark
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: BEN-018, BEN-005, PWR-006, LAB-013, LAB-012
- Baseline: §22, §54, §61
- Benchmarks: B-031
- Risks: R-027
- Invariants: I-061

V1 battery gate: idle, video, compile, and browsing watt-hours on H-004 beside a mainline Linux distribution on the same machine, using the energy-method spike and LAB meters. No superiority claim. PWR owns the laptop-side harness; BEN publishes B-031.

<!-- covers: INV-1033 -->

#### Out of scope
Energy methodology (BEN-018). Meter hardware (LAB). V2 second laptop (BEN-039). Frequency hints (SCH).

#### Acceptance criteria
- [ ] A B-031 report exists for H-004 covering idle desktop and the scripted mixed workload, measured with the instrument chosen by BEN-018.
- [ ] A Linux baseline column exists on the same laptop from LAB-012.
- [ ] The V1 target kind is `publish`.
- [ ] No superiority claim appears in the report prose or in release notes.

#### Verification
- Bench: B-031 on H-004; target per register.
- Review: PWR and BEN leads sign off on the pull request.

#### Evidence
- none

### BEN-025 · Publish os env startup against Docker Compose
- Type: benchmark
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: BEN-005, BEN-007, ENV-014, ENV-018, SDK-041, BEN-020
- Baseline: §35, §36, §54
- Benchmarks: B-025
- Invariants: I-043, I-061

V1 gate and §35: cached `os env enter` versus `docker compose up` for the reference stack, with cold published separately. ENV owns the cache; BEN publishes B-025. Native environments do not require Docker, a Linux VM, or overlayfs.

<!-- covers: INV-0661, INV-1026 -->

#### Out of scope
Environment cache (ENV). CLI enter (SDK). OCI lint (ENV-001). Isolation cost (BEN-026).

#### Acceptance criteria
- [ ] A B-025 report exists for H-002 and H-004 with cached and cold `os env enter` for the reference Postgres plus Redis stack.
- [ ] `docker compose up` cached-images and cold-pull columns exist on the same machines.
- [ ] The V1 cached target kind is `absolute` per register; cold is `publish`.

#### Verification
- Bench: B-025 on H-002 and H-004; target per register.
- Review: ENV and BEN leads sign off on the pull request.

#### Evidence
- none

### BEN-026 · Publish native isolation cost against OCI containers
- Type: benchmark
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: BEN-020, BEN-005, CMP-035
- Baseline: §36, §53, §54
- Benchmarks: B-015
- Invariants: I-019, I-061

§36 and §53: native ResourceDomain plus Component plus capabilities must be published beside OCI creation latency and memory on the same hardware. Uses the V0 spike's baseline images. Native isolation does not require containers.

<!-- covers: INV-0673 -->

#### Out of scope
Spike and image pinning (BEN-020). Component creation microbenchmark (BEN-001). Personality OCI (LNX).

#### Acceptance criteria
- [ ] A B-015 report exists for H-002 and H-004 using the images pinned by BEN-020.
- [ ] The V1 target kind is `publish`.
- [ ] No superiority claim appears outside the report tables.

#### Verification
- Bench: B-015 on H-002 and H-004; target per register.
- Review: CMP and BEN leads sign off on the pull request.

#### Evidence
- none

### BEN-027 · Publish Linux Personality overhead against upstream Linux
- Type: benchmark
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: BEN-005, BEN-007, BEN-006, LNX-056, LNX-001
- Baseline: §46, §54
- Benchmarks: B-026
- Invariants: I-025, I-061

V1 gate: syscall latency and L2 corpus workload throughput versus upstream Linux on the same hardware. Personality productisation is V1; V0 measured only fork syscall overhead. Native software still does not see Linux syscalls.

<!-- covers: INV-0866, INV-1032 -->

#### Out of scope
L2 corpus scenarios (LNX). Fork syscall microbench (BEN-003). L3 and later republish (LNX-063). Windows overhead (BEN-044).

#### Acceptance criteria
- [ ] A B-026 report exists for H-002 and H-004 on L2 non-graphics workloads plus the syscall microbenchmark set versus the pinned upstream kernel.
- [ ] The V1 target kind is `publish` on L2 workloads.
- [ ] No superiority claim and no native-API claim for Linux syscalls appear in the report prose.

#### Verification
- Bench: B-026 on H-002 and H-004; target per register.
- Review: LNX and BEN leads sign off on the pull request.

#### Evidence
- none

### BEN-028 · Publish developer onboarding time
- Type: benchmark
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: BEN-005, BEN-007, SDK-032, SDK-043, SDK-039
- Baseline: §50, §52, §54
- Benchmarks: B-050
- Invariants: I-061

V1 daily-driving and SDK v1 make time-to-hello-Component a tracked claim. SDK owns the timed sessions; BEN publishes B-050: SDK download to running hello, and fresh clone to booted QEMU CI image.

#### Out of scope
SDK generator and host SDK (SDK). Full-image build time (BEN-023). Docs site (DOC).

#### Acceptance criteria
- [ ] A B-050 report exists covering SDK download to running hello Component and fresh clone to booted QEMU image, with at least the participant count in the register Method.
- [ ] The V1 target kind is `publish`.
- [ ] No onboarding duration is restated in milestone prose.

#### Verification
- Bench: B-050 on the host SDK path and H-001; target per register.
- Review: SDK and BEN leads sign off on the pull request.

#### Evidence
- none

### BEN-029 · Verify V1 absolute targets and prior-Rung regression
- Type: benchmark
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: BEN-015, BEN-007, BEN-033, BEN-022, BEN-030, BLD-048, TSK-039
- Baseline: §54
- Benchmarks: B-001, B-004, B-005, B-016, B-051
- Invariants: I-061, I-088

V1 gates: all V0 and V0.5 benchmarks re-run on V1 hardware with no regression beyond the register band, plus first absolute B-001, B-004, and B-005 targets on the reference desktop. Warm-startup absolute is BEN-030; this task includes it in the B-051 roll-up.

#### Out of scope
Merge-gate CI hook (BLD-033). Quiet fleet bring-up (BLD-048). Per-metric publication (sibling BEN tasks).

#### Acceptance criteria
- [ ] A B-051 report exists for H-001, H-002, and H-004 covering every prior B-ID with a committed report.
- [ ] B-001, B-004, and B-005 reports on H-002 meet the V1 `absolute` clauses in the register, or an accepted Decision documents the miss.
- [ ] Any regression exceedance names an accepted Decision.

#### Verification
- Bench: B-051 on H-001, H-002, and H-004; B-001, B-004, B-005 on H-002; target per register.
- Review: BEN methodology sign-off recorded on the pull request.

#### Evidence
- none

### BEN-030 · Publish native warm startup against the V1 target
- Type: benchmark
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: BEN-009, BEN-021, BEN-016, CMP-019
- Baseline: §34, §54
- Benchmarks: B-016
- Invariants: I-042, I-061

V1 benchmark gate: Terminal and Editor warm startup against B-016 on V1 hardware. First absolute application-startup target; V0.5 was publish-only. The figure is a measurement target, never a public guarantee.

<!-- covers: INV-0636 -->

#### Out of scope
Cold startup (CMP-018). Launch-path profile (BEN-021). Toolkit text path (TXT).

#### Acceptance criteria
- [ ] B-016 reports exist for Terminal and Editor on H-002 and H-004 using the visible-UI boundary.
- [ ] The V1 `absolute` clause in the register is met, or an accepted Decision documents the miss.
- [ ] Public SDK material does not treat the B-016 figure as a guarantee (I-042).

#### Verification
- Bench: B-016 on H-002 and H-004; target per register.
- Review: APP and BEN leads sign off on the pull request.

#### Evidence
- none

### BEN-031 · Publish the public per-commit benchmark dashboard
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: BEN-005, BEN-032, BEN-004
- Baseline: §54
- Invariants: I-050, I-061

Performance claims must be continuously measured and publicly visible. The dashboard is fed by CI with per-commit history and regression alerting for every register metric. V3 and V4 gates require this dashboard to stay public. Measured values never appear in the register.

<!-- covers: GAP-0391 -->

#### Out of scope
Time-series store (BEN-032). CI pass/fail dashboards (BLD-066). Dataset license (GOV-040). HCL site (REL).

#### Acceptance criteria
- [ ] A public dashboard shows the latest committed report per B-ID per in-scope H-ID without requiring a login for read.
- [ ] Per-commit history for B-001, B-004, B-016, and B-020 is visible for V1 hardware.
- [ ] A regression beyond the calibrated band produces an alert that names the B-ID and H-ID.
- [ ] Dashboard copy cites B-IDs and does not restate register thresholds as slogans.

#### Verification
- Manual: open the dashboard URL and confirm history plus one synthetic regression alert in staging.
- Review: REL and BEN leads sign off on the pull request.

#### Evidence
- none

### BEN-032 · Store benchmark results with change-point detection
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: BEN-005, BEN-007, BEN-022, BLD-048
- Baseline: §54
- Invariants: I-061

§54 demands watched benchmarks. A time-series store holds nightly and per-merge results with change-point detection, noise-calibrated thresholds from BEN-022, and automatic bisection to the offending commit. Committed reports stay per-milestone; the store holds the rest.

<!-- covers: GAP-0136 -->

#### Out of scope
Committed gate reports (BEN-005). Public dashboard UI (BEN-031). Functional test dashboard (BLD-052). Quiet fleet (BLD-048).

#### Acceptance criteria
- [ ] Every runner publication from main is appended to the store keyed by B-ID, H-ID, and commit.
- [ ] A planted change-point on B-004 is detected and a bisection identifies the planted commit.
- [ ] Thresholds are derived from the noise-floor spike, not from a number in a task file.
- [ ] `registers/benchmarks.md` entries still contain no measured values.

#### Verification
- Integration: planted B-004 change-point on H-001 produces a bisection pointing at the planted commit.
- Review: BLD and BEN leads sign off on the pull request.

#### Evidence
- none

### BEN-033 · Decide the blocking performance merge-Gate policy
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: BEN-007, BEN-022
- Baseline: §54
- Decision: D-0030
- Invariants: I-061, I-088

V1 is the first rung with absolute targets. BLD owns the CI job; this Decision is the policy those jobs enforce. The noise band comes from BEN-022 and never from a number restated in this task.

<!-- covers: GAP-0137 -->

#### Out of scope
CI hook (BLD-033). Quiet fleet (BLD-048). Time-series detection (BEN-032).

#### Acceptance criteria
- [ ] Option A (block merge on a regression beyond the calibrated noise band on B-001, B-004, B-016, and B-020), option B (nightly-only fail), and option C (warn with a required Decision) are evaluated.
- [ ] The accepted option names the B-IDs in the blocking set, the exception path (an accepted Decision), and that numbers live only in the register and the noise-floor report.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: BLD and BEN leads sign off on the pull request that accepts the Decision file.

#### Evidence
- none

### BEN-034 · Write the contributor guide for adding a benchmark
- Type: docs
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: BEN-005, BEN-007, BEN-004
- Baseline: §54

V1 self-hosting and SDK v1 mean other prefixes add B-IDs. The guide states how to register a metric, write a harness against the runner, and land a report, including the ban on numbers in prose and the claim lint.

Required by the BEN scope: "the contributor guide for adding a metric".

#### Out of scope
Register schema (GOV tooling). Runner implementation (BEN-005). Docs site chrome (DOC).

#### Acceptance criteria
- [ ] A committed guide in the `bench` or `docs` repository names register fields, harness alias grammar, report path grammar, and the claim lint.
- [ ] The guide shows how a new B-ID is cited from a task without restating Targets numbers.
- [ ] A Review line names who accepts the guide.

#### Verification
- Review: BEN and DOC leads sign off on the pull request.

#### Evidence
- none

### BEN-035 · Publish boot-to-login and unlock-to-desktop latency
- Type: benchmark
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: BEN-012, BOOT-016, APP-033, APP-030, SEC-017
- Baseline: §30, §54, §63
- Benchmarks: B-032, B-033
- Invariants: I-061

Verity, measured boot, and encryption add boot cost; §54 requires the claim be measured. Numbers live only in the register. Beside Linux and Windows where dual-boot exists on V2 machines.

<!-- covers: GAP-0238 -->

#### Out of scope
Bootloader (BOOT). Greeter and lock UI (APP). FDE NVMe throughput (BEN-040). Fleet-wide boot (BEN-048).

#### Acceptance criteria
- [ ] B-032 reports exist for H-002, H-004, and H-005 from firmware handoff to greeter-accepting-input.
- [ ] B-033 reports exist for those machines from successful unlock to interactive desktop.
- [ ] Each cited B-ID's V2 target kind matches the register (`regression` for B-032, `publish` for B-033).

#### Verification
- Bench: B-032 and B-033 on H-002, H-004, and H-005; target per register.
- Review: BOOT, APP, and BEN leads sign off on the pull request.

#### Evidence
- none

### BEN-036 · Publish ComputeDevice dispatch overhead
- Type: benchmark
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: BEN-005, BEN-007, HET-018, HET-016
- Baseline: §37, §54
- Benchmarks: B-048
- Invariants: I-061, I-064

V2 ComputeDevice demo must not ship an unmeasured dispatch path. HET owns the harness versus direct Vulkan compute and a native thread pool; BEN publishes B-048.

#### Out of scope
ComputeDevice implementation (HET). GPU profiler UI (SDK). Zero-copy media path (BEN-045).

#### Acceptance criteria
- [ ] A B-048 report exists for H-002 covering GPU and CPU dispatch versus the register Baselines.
- [ ] The V2 target kind is `publish`.
- [ ] No superiority claim appears outside the report tables.

#### Verification
- Bench: B-048 on H-002; target per register.
- Review: HET and BEN leads sign off on the pull request.

#### Evidence
- none

### BEN-037 · Publish compositor frame-deadline miss rate
- Type: benchmark
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: BEN-010, GFX-060, LAB-018, OBS-046
- Baseline: §40, §54
- Benchmarks: B-019
- Invariants: I-061

V2 benchmark gate: deadline misses under a scripted mixed desktop workload at 60 Hz. GFX owns the harness; BEN publishes B-019. The target lives only in the register.

#### Out of scope
Frame scheduling (GFX). Input-to-photon (B-020). V4 maximum-refresh absolute (BEN-056).

#### Acceptance criteria
- [ ] A B-019 report exists for H-002, H-004, and H-005 at 60 Hz matching the register Method.
- [ ] The V2 `absolute` clause in the register is met, or an accepted Decision documents the miss.
- [ ] Linux compositor baseline columns exist on the same machines.

#### Verification
- Bench: B-019 on H-002, H-004, and H-005; target per register.
- Review: GFX and BEN leads sign off on the pull request.

#### Evidence
- none

### BEN-038 · Publish desktop-essentials latency suite
- Type: benchmark
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: BEN-005, BEN-007, APP-022, TXT-029, APP-014, APP-033, NET-021, MED-013, LAB-018
- Baseline: §54, §62
- Benchmarks: B-045
- Invariants: I-052, I-061

V2 desktop preview: keystroke-to-glyph through the IME path, notification post-to-display, lock-to-unlock, Wi-Fi reconnect after resume, Bluetooth audio connect, and camera cold start. Owning prefixes instrument the paths; BEN publishes B-045.

<!-- covers: GAP-0319 -->

#### Out of scope
Shell implementation (APP). IME engines (TXT). Camera service (MED). Wi-Fi (NET). Unlock metric B-033 (BEN-035).

#### Acceptance criteria
- [ ] A B-045 report exists for H-002, H-004, and H-005 covering every item in the register Metric.
- [ ] The V2 target kind is `publish`.
- [ ] Linux and Windows columns exist on the same machines where dual-boot is present.

#### Verification
- Bench: B-045 on H-002, H-004, and H-005; target per register.
- Review: APP and BEN leads sign off on the pull request.

#### Evidence
- none

### BEN-039 · Publish energy use on both V2 laptops
- Type: benchmark
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: BEN-024, PWR-006, LAB-018, LAB-015, GFX-081
- Baseline: §22, §54, §62
- Benchmarks: B-031
- Invariants: I-061

V2 gates: idle desktop energy and mixed-workload battery runtime on H-004 and H-005 beside mainline Linux and Windows where dual-boot exists. Extends B-031 from the V1 Intel laptop.

#### Out of scope
Energy methodology (BEN-018). Suspend latency (PWR-017). V3 Tier 1 laptops (PWR-025).

#### Acceptance criteria
- [ ] B-031 reports exist for H-004 and H-005 covering idle and mixed-workload runtime.
- [ ] Linux columns exist on both laptops; Windows columns exist where dual-boot is present.
- [ ] The V2 target kind is `publish` per laptop.
- [ ] No superiority claim appears outside the report tables.

#### Verification
- Bench: B-031 on H-004 and H-005; target per register.
- Review: PWR and BEN leads sign off on the pull request.

#### Evidence
- none

### BEN-040 · Publish disk-encryption overhead on NVMe
- Type: benchmark
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: BEN-005, BEN-007, SEC-017, SEC-005
- Baseline: §51, §54, §63
- Benchmarks: B-038
- Invariants: I-061, I-073

Encryption by default is only defensible if sequential and random NVMe cost is measured. Versus LUKS2 and BitLocker on the same drive. V2 publishes; later rungs extend per Tier 1.

<!-- covers: GAP-0202 -->

#### Out of scope
Encryption layer Decision (SEC). Installer FDE contract (SEC-055). Boot-to-login (BEN-035).

#### Acceptance criteria
- [ ] A B-038 report exists for H-002 with encryption enabled versus disabled on the same volume using the register fio profile.
- [ ] LUKS2 and BitLocker columns exist on that drive where the comparison OS is installed.
- [ ] The V2 target kind is `publish`.

#### Verification
- Bench: B-038 on H-002; target per register.
- Review: SEC and BEN leads sign off on the pull request.

#### Evidence
- none

### BEN-041 · Publish the gaming benchmark suite on the AMD desktop
- Type: benchmark
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: BEN-044, WIN-006, WIN-009, LAB-015, LAB-018
- Baseline: §48, §54, §56.2
- Benchmarks: B-027
- Risks: R-035
- Invariants: I-061, I-071

§56.2 V2 gaming proof: frame-time distribution, input latency, load time, and stutter for W1 Gold titles on H-002 versus Windows and Proton-on-Linux. Complements B-027 overhead with the stutter and load suite. Anti-cheat bypass is a non-goal.

<!-- covers: INV-1079 -->

#### Out of scope
W1 corpus and title harness (WIN). Personality overhead tables (BEN-044). Blocked titles (WIN). HDR display (LAB, GFX).

#### Acceptance criteria
- [ ] Reports exist for each W1 Gold title on H-002 covering frame-time distribution, input latency, load time, and stutter at the register's resolution and settings.
- [ ] Windows and Proton-on-Linux columns exist on H-002.
- [ ] Titles excluded by I-071 are listed as out of corpus, not as failed runs.
- [ ] The V2 target kind on W1 Gold is `publish`.

#### Verification
- Bench: B-027 on H-002; target per register.
- Review: WIN and BEN leads sign off on the pull request.

#### Evidence
- none

### BEN-042 · Publish snapshot creation and restore time
- Type: benchmark
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: BEN-005, BEN-007, STO-025, PKG-068
- Baseline: §26, §31, §54
- Benchmarks: B-036
- Invariants: I-061

V2 benchmark gate: snapshot creation and restore from the settings UI on a fixed user-data set. STO and PKG own the mechanism; BEN publishes B-036.

#### Out of scope
Snapshot Operations (STO). Package-set restore (PKG). Generation rollback (B-022). Backup product (STO, APP).

#### Acceptance criteria
- [ ] A B-036 report exists for H-002 on the user-data set size in the register Method.
- [ ] The V2 target kind is `publish`.
- [ ] Linux btrfs or bcachefs and Windows restore-point columns exist on the same machine.

#### Verification
- Bench: B-036 on H-002; target per register.
- Review: STO and BEN leads sign off on the pull request.

#### Evidence
- none

### BEN-043 · Verify no V1 benchmark regression at V2
- Type: benchmark
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: BEN-029, BEN-005
- Baseline: §54
- Benchmarks: B-051
- Invariants: I-061, I-088

V2 benchmark gate: all prior benchmarks re-run on H-002, H-004, and H-005 with no regression beyond the register band versus V1 without an accepted Decision.

#### Out of scope
New V2 metrics (sibling BEN tasks). Merge exceptions (BEN-033).

#### Acceptance criteria
- [ ] A B-051 report exists for H-002, H-004, and H-005 covering every prior B-ID with a committed V1 report.
- [ ] Any exceedance of the V2 `regression` clause names an accepted Decision, or no exceedance is present.

#### Verification
- Bench: B-051 on H-002, H-004, and H-005; target per register.
- Review: BEN methodology sign-off recorded on the pull request.

#### Evidence
- none

### BEN-044 · Publish Windows Personality overhead against Proton and Windows
- Type: benchmark
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: BEN-005, BEN-047, WIN-006, WIN-009, LAB-015
- Baseline: §48, §54, §56.2
- Benchmarks: B-027
- Risks: R-035
- Invariants: I-007, I-061, I-071

V2 W1 gate: frame time and CPU overhead of Gold titles versus Linux plus Proton and versus native Windows on the same hardware. No superiority claim. Wine bring-up stays non-gated in WIN at V1. Native software still does not see Win32.

<!-- covers: INV-0911, INV-1031 -->

#### Out of scope
W1 corpus (WIN). Gaming stutter suite extras (BEN-041). Proton stack (WIN). Anti-cheat titles (I-071).

#### Acceptance criteria
- [ ] B-027 reports exist for each W1 Gold title on H-002 with personality, Proton-on-Linux, and Windows columns.
- [ ] The V2 target kind is `publish` on W1 Gold titles.
- [ ] No superiority claim appears outside the report tables.

#### Verification
- Bench: B-027 on H-002; target per register.
- Review: WIN and BEN leads sign off on the pull request.

#### Evidence
- none

### BEN-045 · Publish Zero-copy NIC-to-GPU copies per stage
- Type: benchmark
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: BEN-005, BEN-007, MED-028, MED-017, MEM-010
- Baseline: §17, §54
- Benchmarks: B-046
- Invariants: I-061, I-063

§17 path from NIC or storage through a decoder Component to GPU and scanout, with copies counted by physical-page identity where hardware permits. MED owns the harness; BEN publishes B-046 when the media path exists.

<!-- covers: INV-0326 -->

#### Out of scope
Hardware decode (MED). MemoryObject transfer microbenchmark (B-007). ComputeDevice dispatch (B-048). Native GPU driver stack (forbidden, I-045).

#### Acceptance criteria
- [ ] A B-046 report exists for H-002 with copy count per stage and end-to-end latency for the register Method.
- [ ] GStreamer dma-buf and software-copy columns exist on the same machine.
- [ ] The V2 target kind is `publish`.

#### Verification
- Bench: B-046 on H-002; target per register.
- Review: MED, MEM, and BEN leads sign off on the pull request.

#### Evidence
- none

### BEN-046 · Pin macOS comparison baseline on comparable hardware
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: BEN-007, BEN-006
- Baseline: §54
- Invariants: I-061

§54 macOS comparison where a hardware-comparable class exists (Input-to-photon, app startup, energy). Pins machine class, OS version, and method so V4 and 1.0 cross-OS tables are replayable. This is a class comparison, not a claim that macOS hardware is in Tier 1.

<!-- covers: INV-1036 -->

#### Out of scope
Tier 1 membership for Apple hardware (HW, LAB). Windows images (BEN-047). 1.0 publication pack (BEN-060).

#### Acceptance criteria
- [ ] A committed baseline record names machine class, OS version, and which B-IDs are comparable (B-016, B-020, B-031 at minimum).
- [ ] B-IDs that have no comparable class are recorded as `class: none` rather than omitted silently.
- [ ] The method matches BEN-007 percentiles and warm/cold definitions.

#### Verification
- Review: BEN lead sign-off recorded on the pull request.
- Manual: replay B-016 on the pinned class and attach the environment record.

#### Evidence
- none

### BEN-047 · Pin Windows comparison baseline on reference hardware
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: BEN-007, LAB-015, LAB-018
- Baseline: §54
- Invariants: I-061

§54 Windows comparison for startup, input latency, gaming, and energy needs a pinned Windows image on dual-boot reference hardware before W1 overhead is published.

<!-- covers: INV-1035 -->

#### Out of scope
Dual-boot install mechanics (LAB). W1 overhead publication (BEN-044). End-user dual-boot policy (INS).

#### Acceptance criteria
- [ ] A committed baseline record names Windows version, image hash, and boot path for H-002, and for H-004/H-005 where dual-boot exists.
- [ ] Re-running B-016, B-020, B-027, and B-031 against that record reproduces the Windows column without an unpinned feature-update train.

#### Verification
- Integration: one B-027 Windows-column replay on H-002 from the pinned image.
- Review: WIN and BEN leads sign off on the pull request.

#### Evidence
- none

### BEN-048 · Publish boot time to login on every Tier 1 machine
- Type: benchmark
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: BEN-035, BOOT-016, LAB-021
- Baseline: §54, §63
- Benchmarks: B-032
- Invariants: I-061

V3 gate: boot time to login on each of the six Tier 1 machines beside a mainline Linux distribution and Windows where dual-boot exists. Extends B-032 from V0.5/V2 hardware.

#### Out of scope
Bootloader (BOOT). Installer wall time (INS-016). Unlock-to-desktop (B-033).

#### Acceptance criteria
- [ ] B-032 reports exist for H-002, H-004, H-005, H-006, H-007, and H-008.
- [ ] Linux columns exist on every machine; Windows columns exist where dual-boot exists.
- [ ] The V3 target kind is `publish` per Tier 1 machine.

#### Verification
- Bench: B-032 on H-002, H-004, H-005, H-006, H-007, and H-008; target per register.
- Review: BOOT and BEN leads sign off on the pull request.

#### Evidence
- none

### BEN-049 · Publish crash-free session rate from opt-in telemetry
- Type: benchmark
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: BEN-005, REL-042, OBS-049
- Baseline: §54, §63
- Benchmarks: B-041
- Risks: R-057
- Invariants: I-061

V3 gate: crash-free session rate from opt-in telemetry over the stated window and fleet size. OBS and REL collect; BEN publishes B-041 against the register target. Fleet size is a register clause, not a promise in this description.

#### Out of scope
Telemetry intake (REL). Crash capture format (OBS). Consent UI (INS). Panic rate (BEN-051).

#### Acceptance criteria
- [ ] A B-041 report exists for the V3 window naming session count, machine count, and the crash-free fraction.
- [ ] The V3 `absolute` clause in the register is met, or an accepted Decision documents the miss.
- [ ] The report includes only consented schema from REL-042.

#### Verification
- Bench: B-041 over the V3 telemetry window; target per register.
- Review: REL, OBS, and BEN leads sign off on the pull request.

#### Evidence
- none

### BEN-050 · Publish interoperability path throughput and import time
- Type: benchmark
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: BEN-005, BEN-007, VIRT-011, LAB-021
- Baseline: §54
- Benchmarks: B-049
- Invariants: I-061

V3 migration and interop: NTFS and exFAT throughput, SMB client throughput, VM-guest launch time and GPU fraction, and migration import time at the corpus size named in the B-049 Method, versus Linux and Windows on the same hardware.

<!-- covers: GAP-0476 -->

#### Out of scope
Foreign filesystems (STO). VM manager (VIRT). Migration assistant UI (APP). Guest GPU implementation (VIRT-011).

#### Acceptance criteria
- [ ] A B-049 report exists on at least one Tier 1 machine covering every path in the register Metric.
- [ ] Linux and Windows columns exist for each path on that machine.
- [ ] The V3 target kind is `publish`.

#### Verification
- Bench: B-049 on H-002; target per register.
- Review: STO, VIRT, INS, and BEN leads sign off on the pull request.

#### Evidence
- none

### BEN-051 · Publish kernel panic rate from opt-in telemetry
- Type: benchmark
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: BEN-049, REL-042, OBS-027
- Baseline: §54, §63
- Benchmarks: B-042
- Risks: R-057
- Invariants: I-061

V3 gate: kernel panics per machine-day from opt-in telemetry, published monthly as the register Method requires. V4 and 1.0 add the absolute ceiling in the register. OBS captures; REL intakes; BEN publishes.

#### Out of scope
Crash-free sessions (B-041). Capture format (OBS). CVE response (REL, KRN).

#### Acceptance criteria
- [ ] A B-042 report exists for the V3 window naming panics, machine-days, and the rate.
- [ ] The V3 target kind is `publish` monthly per the register.
- [ ] The report includes only consented telemetry.

#### Verification
- Bench: B-042 over the V3 telemetry window; target per register.
- Review: REL, OBS, and BEN leads sign off on the pull request.

#### Evidence
- none

### BEN-052 · Publish update success rate
- Type: benchmark
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: BEN-005, INS-045, REL-042
- Baseline: §30, §54, §63
- Benchmarks: B-043
- Invariants: I-061

V3 updater gate and later 1.0 update-success guarantee: fraction of update attempts that boot the new SystemGeneration without automatic rollback. INS and REL collect attempts; BEN publishes B-043.

Required by V4-G08 (In-place upgrade from V3 preserves user data) and 1.0-G06 (Update and rollback guarantee verified on every Tier 1 machine): both are satisfied when the B-043 target is met.

#### Out of scope
Updater client (INS). Rollback time (B-035). Channel publication (REL).

#### Acceptance criteria
- [ ] A B-043 report exists naming attempt count and success fraction on Tier 1 and across the opt-in fleet.
- [ ] The V3 target kind is `publish`.
- [ ] Automatic rollbacks are counted as failures of this metric, not omitted.

#### Verification
- Bench: B-043 over the V3 window; target per register.
- Review: INS, REL, and BEN leads sign off on the pull request.

#### Evidence
- none

### BEN-053 · Verify no V2 benchmark regression at V3
- Type: benchmark
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: BEN-043, BEN-031, LAB-021
- Baseline: §54
- Benchmarks: B-051
- Invariants: I-061, I-088

V3 benchmark gate: all prior benchmarks re-run across Tier 1 with no regression beyond the register band versus V2, with dashboards public.

#### Out of scope
Dashboard implementation (BEN-031). Completeness matrix (BEN-054).

#### Acceptance criteria
- [ ] A B-051 report exists for every V3 Tier 1 machine covering every prior B-ID with a committed V2 report.
- [ ] Any exceedance names an accepted Decision, or no exceedance is present.
- [ ] The public dashboard shows the V3 B-051 run.

#### Verification
- Bench: B-051 on H-002, H-004, H-005, H-006, H-007, and H-008; target per register.
- Review: BEN methodology sign-off recorded on the pull request.

#### Evidence
- none

### BEN-054 · Prove every tracked metric has a public Tier 1 number
- Type: docs
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: BEN-053, BEN-031, GOV-040
- Baseline: §54
- Invariants: I-061, I-088

V3 benchmark gate: the full §54 list has a public number for every item on at least one Tier 1 machine; any missing item blocks the gate. Produces the completeness matrix against the register.

#### Out of scope
Collecting new measurements (sibling BEN tasks). Dataset license (GOV-040). HCL (REL).

#### Acceptance criteria
- [ ] A committed matrix lists every B-ID in `registers/benchmarks.md` with a public report URL and at least one Tier 1 H-ID.
- [ ] Any B-ID without a public report is named as a failing gate reason, not skipped.
- [ ] A Review line names who accepts the matrix.

#### Verification
- Review: BEN and REL leads sign off on the pull request.
- Manual: spot-check five B-IDs from the matrix against the public dashboard.

#### Evidence
- none

### BEN-055 · Publish security-mitigation overhead
- Type: benchmark
- Milestone: V4
- Status: todo
- Size: S
- Owner: none
- Depends on: BEN-005, KRN-029, KRN-034
- Baseline: §51, §54
- Benchmarks: B-040
- Invariants: I-061

V4 benchmark gate: relative cost of shipped kernel and runtime mitigations on versus off for B-001, B-004, B-016, and B-026. KRN owns the harness; BEN publishes B-040.

#### Out of scope
Hardening config (KRN). Side-channel position statement (SEC). Runtime mitigations (SDK).

#### Acceptance criteria
- [ ] A B-040 report exists for H-002 covering mitigations on versus off on B-001, B-004, B-016, and B-026.
- [ ] The V4 target kind is `publish`.
- [ ] Upstream Linux mitigation-toggle columns exist on the same machine.

#### Verification
- Bench: B-040 on H-002; target per register.
- Review: KRN and BEN leads sign off on the pull request.

#### Evidence
- none

### BEN-056 · Publish V4 absolute targets for creation, IPC and startup
- Type: benchmark
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: BEN-029, BEN-030, BEN-037, LAB-023
- Baseline: §10, §15, §34, §54
- Benchmarks: B-001, B-002, B-004, B-005, B-016, B-019, B-020, B-025
- Invariants: I-042, I-061, I-088

V4 gates restated only by B-ID: Component creation, Task creation, same-core and cross-core IPC, Terminal and Editor warm startup, cached `os env`, Input-to-photon, and compositor deadline misses on the reference desktop and every Tier 1 machine.

#### Out of scope
Fleet publication of every metric (BEN-058). Regression band versus V3 (BEN-059). Tuning work (CMP, IPC, GFX).

#### Acceptance criteria
- [ ] Reports exist for B-001, B-002, B-004, B-005, B-016, B-019, B-020, and B-025 on H-002 and on every V4 Tier 1 machine in hardware scope.
- [ ] Each cited B-ID meets its V4 `absolute` clause, or an accepted Decision documents the miss.
- [ ] Public material does not treat B-016 as a guarantee (I-042).

#### Verification
- Bench: B-001, B-002, B-004, B-005, B-016, B-019, B-020, B-025 on H-002 and every V4 Tier 1 machine; target per register.
- Review: BEN methodology sign-off recorded on the pull request.

#### Evidence
- none

### BEN-057 · Update public dashboards on every V4 release candidate
- Type: build
- Milestone: V4
- Status: todo
- Size: S
- Owner: none
- Depends on: BEN-031, BEN-054
- Baseline: §54
- Invariants: I-061

V4 benchmark gate: public dashboards updated on every RC. Extends the V1 dashboard with RC snapshots and fleet-wide Tier 1 views.

#### Out of scope
Dashboard first publish (BEN-031). RC soak matrix (BLD, LAB).

#### Acceptance criteria
- [ ] Each V4 RC has a frozen dashboard snapshot naming the RC identifier without a calendar date.
- [ ] Fleet-wide Tier 1 views show every B-ID that BEN-054 listed as public.
- [ ] A missing RC snapshot fails the V4 dashboard gate.

#### Verification
- Manual: compare two RC snapshots and confirm Tier 1 views.
- Review: REL and BEN leads sign off on the pull request.

#### Evidence
- none

### BEN-058 · Publish every tracked metric on every Tier 1 machine
- Type: benchmark
- Milestone: V4
- Status: todo
- Size: L
- Owner: none
- Depends on: BEN-054, BEN-056, BEN-046, BEN-047, LAB-023
- Baseline: §54
- Benchmarks: B-001, B-002, B-003, B-004, B-005, B-006, B-007, B-008, B-009, B-010, B-011, B-012, B-013, B-014, B-015, B-016, B-017, B-018, B-019, B-020, B-021, B-022, B-023, B-024, B-025, B-026, B-027, B-028, B-029, B-030, B-031, B-032, B-033, B-034, B-035, B-036, B-037, B-038, B-039, B-040, B-041, B-042, B-043, B-044, B-045, B-046, B-047, B-048, B-049, B-050
- Invariants: I-061

V4 gate: every §54 metric published for every Tier 1 machine with comparison to Linux and, where dual-boot exists, Windows; macOS comparison where a comparable class exists. Extends the V3 completeness proof across the ten-machine fleet.

#### Out of scope
Absolute-target checks (BEN-056). Regression check (BEN-059). 1.0 L5/W3 overhead (BEN-060).

#### Acceptance criteria
- [ ] Every B-ID from B-001 through B-050 has a public report on every V4 Tier 1 machine to which the metric applies (laptops for B-030/B-031, dual-boot for Windows columns).
- [ ] Linux columns exist on every such machine; Windows columns exist where dual-boot exists; macOS columns exist where BEN-046 recorded a class.
- [ ] Inapplicable pairs are recorded as `class: none` or `hardware: none`, not omitted.

#### Verification
- Bench: every B-ID in the Benchmarks field on the V4 hardware scope as applicable; target per register.
- Review: BEN lead sign-off recorded on the completeness matrix update.

#### Evidence
- none

### BEN-059 · Verify no V3 benchmark regression at V4
- Type: benchmark
- Milestone: V4
- Status: todo
- Size: S
- Owner: none
- Depends on: BEN-053, LAB-023
- Baseline: §54
- Benchmarks: B-051
- Invariants: I-061, I-088

V4 benchmark gate: all prior benchmarks re-run on all Tier 1 machines; regression beyond the tighter V4 band versus V3 blocks unless an accepted Decision explains it.

#### Out of scope
Absolute targets (BEN-056). Mitigation overhead (BEN-055).

#### Acceptance criteria
- [ ] A B-051 report exists for every V4 Tier 1 machine covering every prior B-ID with a committed V3 report.
- [ ] Any exceedance of the V4 `regression` clause names an accepted Decision, or no exceedance is present.

#### Verification
- Bench: B-051 on every V4 Tier 1 machine; target per register.
- Review: BEN methodology sign-off recorded on the pull request.

#### Evidence
- none

### BEN-060 · Publish 1.0 cross-OS comparison on every Tier 1 machine
- Type: benchmark
- Milestone: 1.0
- Status: todo
- Size: M
- Owner: none
- Depends on: BEN-058, BEN-046, BEN-047, LNX-108
- Baseline: §54
- Benchmarks: B-001, B-002, B-003, B-004, B-005, B-006, B-007, B-008, B-009, B-010, B-011, B-012, B-013, B-014, B-015, B-016, B-017, B-018, B-019, B-020, B-021, B-022, B-023, B-024, B-025, B-026, B-027, B-028, B-029, B-030, B-031, B-032, B-033, B-034, B-035, B-036, B-037, B-038, B-039, B-040, B-041, B-042, B-043, B-044, B-045, B-046, B-047, B-048, B-049, B-050
- Invariants: I-050, I-061

1.0 exit and definition: every §54 metric published for every Tier 1 machine against Linux, Windows where dual-boot exists, and macOS where a comparable class exists, including L5 and W3 overhead and energy. No unmeasured superiority claim.

#### Out of scope
Repro pack (BEN-063). Claim audit of the announcement (BEN-062). Regression versus V4 (BEN-061).

#### Acceptance criteria
- [ ] Every B-ID from B-001 through B-050 has a 1.0-milestone report on every in-scope Tier 1 machine, including B-026 on L5 and B-027 on W3.
- [ ] Linux, Windows-where-dual-boot, and macOS-where-class-exists columns are present.
- [ ] Release notes cite B-IDs only; they do not restate register numbers.

#### Verification
- Bench: every B-ID in the Benchmarks field on the 1.0 hardware scope as applicable; target per register.
- Review: BEN and REL leads sign off on the publication set.

#### Evidence
- none

### BEN-061 · Verify no V4 benchmark regression at 1.0
- Type: benchmark
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: BEN-059, BEN-060
- Baseline: §54
- Benchmarks: B-051
- Invariants: I-061, I-088

1.0 benchmark gate: no regression beyond the register band versus V4 final on any tracked benchmark on any Tier 1 machine; any exception is an accepted Decision referenced in the release notes.

#### Out of scope
Cross-OS publication (BEN-060). Release notes (REL).

#### Acceptance criteria
- [ ] A B-051 report exists for every 1.0 Tier 1 machine covering every B-ID with a committed V4 report.
- [ ] Any exceedance of the 1.0 `regression` clause names an accepted Decision referenced from the release notes, or no exceedance is present.

#### Verification
- Bench: B-051 on every 1.0 Tier 1 machine; target per register.
- Review: BEN methodology sign-off recorded on the pull request.

#### Evidence
- none

### BEN-062 · Audit 1.0 claims against published benchmark reports
- Type: docs
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: BEN-004, BEN-060
- Baseline: §53, §54, §57
- Invariants: I-050, I-061

1.0 exit: the release announcement contains no performance claim without a link to the measurement. Checklist against BEN-004 and the public reports; I-061 enforcement at ship time.

#### Out of scope
Lint implementation (BEN-004). Release-note drafting (REL). Repro pack (BEN-063).

#### Acceptance criteria
- [ ] A committed checklist maps every performance sentence in the 1.0 announcement and release notes to a B-ID and a report URL.
- [ ] Any sentence without a report is removed or rewritten before ship.
- [ ] A Review line names who accepts the checklist.

#### Verification
- Review: BEN, REL, and GOV leads sign off on the pull request.
- Manual: grep the announcement and notes for numbers and superiority adjectives and match each hit to the checklist.

#### Evidence
- none

### BEN-063 · Publish methodology, raw data and scripts for third parties
- Type: docs
- Milestone: 1.0
- Status: todo
- Size: M
- Owner: none
- Depends on: BEN-007, BEN-034, BEN-060, GOV-040, BLD-041
- Baseline: §54
- Invariants: I-061

1.0 benchmark gate: methodology, raw data, and scripts published so third parties can reproduce every number. Pairs with the reproducible-build story; BEN owns the measurement pack. Dataset terms come from GOV-040.

#### Out of scope
Dataset license Decision (GOV). Bit-for-bit image reproducibility (BLD). Claim audit (BEN-062).

#### Acceptance criteria
- [ ] A published pack contains methodology text, harness scripts, pinned baseline image hashes, and raw data for every B-ID in the 1.0 publication set.
- [ ] The pack's license matches GOV-040.
- [ ] A third-party reproduction of B-004 on H-001 from the pack is documented as a worked example.

#### Verification
- Review: BEN, GOV, and BLD leads sign off on the pull request.
- Manual: follow the worked example for B-004 on H-001 and attach the resulting report.

#### Evidence
- none

### BEN-064 · Decide the benchmark methodology standard: hardware list, warm and cold runs, percentiles, iterations, pinning and mitigations
- Type: adr
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: BEN-007, HW-003
- Baseline: §54, §59
- Decision: D-0348
- Risks: R-009
- Invariants: I-061

BEN-007 fixed the target-kind policy and left the methodology standard that Q-001 asks for unwritten: the reference hardware list per rung, warm and cold definitions, the percentiles every report states, iteration and warm-up counts, CPU frequency pinning, SMT and mitigation settings, and how a QEMU profile is labelled so it is never read as a hardware result. Every V0 benchmark task depends on Q-001; this Decision answers it so the runner (BEN-005) has fields to enforce. It does not restate any number in prose.

#### Out of scope
Target kinds and register-only numbers (BEN-007, D-0031). Runner implementation (BEN-005). Energy measurement method (BEN-018). Visible-UI boundary (BEN-016).

#### Acceptance criteria
- [ ] Option A (one standard for every B-ID: p50 and p99 with fixed iteration and warm-up counts, frequency pinned, mitigations at the shipped default, SMT as shipped), option B (per-B-ID methodology recorded on each register entry), and option C (adopt an existing published benchmarking standard verbatim) are evaluated.
- [ ] The accepted option records the hardware list per rung by H-ID, the warm and cold definitions, the statistics reported, iteration and warm-up counts, frequency pinning, SMT and mitigation settings, and the rule that QEMU-profile results are labelled as functional coverage rather than performance results.
- [ ] The accepted option names the environment fields the BEN-005 runner rejects a run for omitting.
- [ ] Q-001 is marked answered by this task in `registers/questions.md` when the Decision is accepted.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: BEN and GOV leads sign off on the pull request that accepts the Decision file.
- Manual: `registers/questions.md` shows Q-001 answered by BEN-064 in the same change.

#### Evidence
- none
