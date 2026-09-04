# BLD · Build, toolchain, CI
- Prefix: BLD
- Lead: none
- Baseline: §50, §51, §54, §55

<!-- roadmap:generated:begin summary -->
Tasks: 80 live, 0 done, 0 in-progress, 80 todo, 0 dropped. Ready: 1. Blocked: 79. Weighted: 0%.
<!-- roadmap:generated:end -->

## Scope

BLD owns the build graph and the CI that proves it. That includes the pinned Rust-in-kernel and userspace rustc toolchains, LLVM as the sole C compiler, the top-level build orchestrator, hermetic one-command Linux-host builds and later `os env` self-host, the content-addressed remote cache, determinism flags, double-build identity checks, independent rebuilders and a public verifier. It owns the QEMU boot matrix and harness, the guest test agent, kselftest runners, sanitizer and coverage profiles, CI tiers and the merge queue, infrastructure as code, lab-job submission, the quiet performance fleet, and the jobs that execute BEN harnesses and compatibility corpora. It owns Native ABI syzkaller adaptation, continuous fuzz infrastructure, userspace parser fuzzing, the semantic GUI harness, license and SBOM and SLSA provenance gates, milestone test images, promote-without-rebuild, the embargoed compile path, dashboards, and flake quarantine.

Native software is built and tested against the Native ABI. Personality corpora run inside guests; they are not native APIs (§3, §57).

## Out of scope

Kernel fork, retained-mechanism inventory and RISC-V build-only CI (KRN). Bootloader, firmware and generation menu (BOOT). ABI suite content, snapshot and POSIX-shape lints (ABI). Capability, Channel and Component fuzz targets (CAP, IPC, CMP). Benchmark methodology, result TSDB and publication (BEN). Lab machines, scheduler and soak calendar (LAB). Signing, channels, advisories and debuginfod hosting (REL, OBS). License policy, DCO and forge hosting (GOV). Image builder and installer (INS). SDK `forbid(unsafe_code)` templates, `os` CLI and SDK suite content (SDK). DevelopmentEnvironment object (ENV). Content-addressed store substrate (STO). Package identity and SystemGeneration compose (PKG). Compositor golden images (GFX). Accessibility tree schema (ACC). Desktop UX scenario content (APP). Linux and Windows corpus scenarios (LNX, WIN). Threat model document (SEC). Docs site (DOC).

## Tasks

### BLD-001 · Structure CI into tiers and enforce a merge queue
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: BLD-003, BLD-012, BLD-011
- Baseline: §59, §55
- Invariants: I-099

V0 gates and bisection assume main is always green and every landed commit was tested against the tree it landed on (§59). This task structures CI into pre-merge (lint, unit, QEMU smoke boot), post-merge (full QEMU matrix), and nightly (sanitizers, kselftests, publish-only benches, hardware regression), and enforces a merge queue with no direct pushes. Release qualification is a later tier (BLD-067).

<!-- covers: GAP-0109, GAP-0110 -->

#### Out of scope
Runner platform choice (BLD-003). Harness implementation (BLD-012). Release-qualification checklist (BLD-067).

#### Acceptance criteria
- [ ] Pre-merge required checks are lint, unit tests and a QEMU smoke boot on H-001; a failing check rejects the merge-queue entry.
- [ ] Post-merge CI runs the declared QEMU matrix from BLD-012 on every landed commit.
- [ ] Nightly CI runs sanitizer boots, retained-subsystem kselftests and BLD-010 publication jobs.
- [ ] Direct pushes to the default branch are rejected; every merge is a merge-queue commit with a recorded passing run.

#### Verification
- Integration: merge-queue fixture on H-001 where a red smoke boot prevents land and a green run produces a merge commit.
- Review: GOV process reviewer confirms no direct-push path remains on the default branch.

#### Evidence
- none

### BLD-002 · Decide the top-level build orchestrator for kernel and userspace
- Type: adr
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: BLD-005, BLD-004
- Baseline: §27, §50
- Decision: D-0033

The orchestrator for a mixed C plus Rust kernel and a Rust-first userspace decides whether hermeticity, remote caching and bit-for-bit identity are properties of the graph or bolt-ons (§27, §50). This Decision is required before the one-command Linux-host build. It does not choose the CI platform or the repository topology.

<!-- covers: GAP-0087 -->

#### Out of scope
Repository topology (BLD-005). CI platform (BLD-003). Remote cache deployment (BLD-025).

#### Acceptance criteria
- [ ] Options evaluated include Kbuild driven by Cargo workspaces, Bazel or Buck2, and a derivation-based builder aligned with the content-addressed store.
- [ ] The accepted option states how hermetic inputs, remote cache keys and bit-for-bit artifact identity are properties of the graph.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: BLD toolchain owner and kernel architecture lead sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- none

### BLD-003 · Decide the CI platform with self-hosted KVM runners
- Type: adr
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: BLD-005, GOV-001
- Baseline: §55, §59
- Decision: D-0034

Boot, graphics, performance and hardware jobs need nested virtualization or lab devices that hosted CI cannot provide (§55). This Decision selects the platform and requires self-hosted KVM runners for those jobs, with hosted runners allowed only for lint and unit tests. It precedes CI tiers and the QEMU harness.

<!-- covers: GAP-0107 -->

#### Out of scope
CI tier layout and merge queue (BLD-001). Lab scheduler (LAB-005). Forge hosting (GOV-001).

#### Acceptance criteria
- [ ] Options evaluated include self-hosted KVM runners with hosted lint and unit only, all jobs self-hosted, and hosted runners with nested virtualization for boot jobs.
- [ ] The accepted option names where boot, graphics, performance and hardware jobs run, and forbids hosted runners for those classes.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: BLD CI owner and LAB lead sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- none

### BLD-004 · Decide LLVM/Clang as the sole C compiler and reject a custom compiler
- Type: adr
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: none
- Baseline: §50, §51
- Decision: D-0036
- Invariants: I-082, I-089

Rust-in-kernel needs bindgen against libclang, and mixing GCC-built C with rustc-built Rust doubles the sanitizer and ABI matrix (§50, §51). This Decision standardises on LLVM/Clang as the sole C compiler, records GCC kernel builds as unsupported, and rejects a custom compiler or forked LLVM in favour of upstream-bound patches.

<!-- covers: GAP-0088, GAP-0160 -->

#### Out of scope
Kernel rustc pin (BLD-013, KRN-004). Linker, LTO and PGO (BLD-039).

#### Acceptance criteria
- [ ] Options evaluated include LLVM/Clang as the sole C compiler, a dual GCC plus Clang matrix, and a project-maintained compiler or forked LLVM.
- [ ] The accepted option states that GCC kernel builds are unsupported and that rustc and LLVM carry only minimal upstream-bound patches (I-089).
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: BLD toolchain owner and kernel architecture lead sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- none

### BLD-005 · Decide repository topology before a second repository exists
- Type: adr
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: GOV-001
- Baseline: §50, §56.4
- Decision: D-0037

Every CI, bisection and promotion design depends on whether one commit identifies the whole system (§56.4). This Decision records monorepo versus pinned-manifest multi-repo versus a separate kernel tree before any second repository exists. GOV-001 chooses the forge; this Decision chooses the topology on that forge.

<!-- covers: GAP-0086 -->

#### Out of scope
Forge hosting (GOV-001). Kernel fork bootstrap (KRN-010). Build orchestrator (BLD-002).

#### Acceptance criteria
- [ ] Options evaluated include a single monorepo containing kernel fork, native userspace, SDK and tooling; a pinned-manifest multi-repo; and a separate kernel repository with a userspace monorepo.
- [ ] The accepted option states whether one commit identifies the whole system and how bisection and promotion refer to that identity.
- [ ] The Decision is accepted before a second project repository is created.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: BLD CI owner and GOV maintainer sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- none

### BLD-006 · Build a guest-side test agent that reports structured results
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: BLD-012, BLD-009
- Baseline: §10, §12, §16, §59

V0 Component, Channel, Capability and MemoryObject tests must run inside the booted image, not only on the Linux host (§59). The agent starts as a Component in the guest, runs the nominated suite, and streams structured results including task IDs over virtio-serial or vsock to the harness.

<!-- covers: GAP-0114 -->

#### Out of scope
Boot-complete detection and panic handling (BLD-012). Compatibility-corpus job plumbing (BLD-017). Test content (CMP, IPC, CAP, MEM).

#### Acceptance criteria
- [ ] The agent runs inside the H-001 smoke image and reports pass, fail and skip records as JSON over virtio-serial or vsock.
- [ ] Each failing record includes the roadmap task ID named by the test.
- [ ] A harness-side timeout with no agent handshake fails the job and captures serial plus QEMU logs.
- [ ] Host-side unit tests of the Native ABI are not a substitute for a guest-agent run of the V0 primitive suite.

#### Verification
- Integration: V0 primitive suite on H-001 via the agent, with a deliberate failing fixture mapping to a nominated task ID.
- Unit: `bld:tests/guest_agent_protocol_*` on `qemu-x86_64`.

#### Evidence
- none

### BLD-007 · Run the hardware regression matrix on QEMU and the Reference machine
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: BLD-012, KRN-014, LAB-003
- Baseline: §55
- Risks: R-013
- Invariants: I-054, I-098

Native features must not regress hardware support while the fork lands (§55). KRN owns the retained-mechanism inventory and matrix contents; BLD hosts the DRM, PCI, USB, NVMe, networking and ACPI regression jobs on H-001 and H-002 and fails the merge when a retained kselftest or hardware probe regresses.

<!-- covers: INV-1052 -->

#### Out of scope
Matrix contents and GPL-only-symbol tests (KRN-014). Nightly kselftest runner axes (BLD-012). Lab racking (LAB-003).

#### Acceptance criteria
- [ ] Post-merge CI runs the KRN retained-subsystem matrix on H-001 for DRM, PCI, USB, NVMe, networking and ACPI.
- [ ] The same matrix runs on H-002 for every kernel change that lands on the default branch.
- [ ] A regression in a retained kselftest or probe fails the job and is visible as a required check.
- [ ] `os inspect` on a passing H-002 run lists the matrix job identifier and the KRN inventory revision it executed.

#### Verification
- Integration: matrix jobs on H-001 and H-002; a fixture that breaks a retained NVMe kselftest fails the check.
- Review: KRN lead confirms BLD executes the KRN-014 contents without a second inventory.

#### Evidence
- none

### BLD-008 · Document and CI-exercise the kernel debug workflow
- Type: docs
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: BLD-012
- Baseline: §50, §64
- Risks: R-011

Without QEMU gdbstub, kgdb, early console and crash-dump analysis, V0 debugging becomes folklore (R-011). This page records those four paths for the fork and CI exercises each one so the first boot is debuggable.

<!-- covers: EXTRA-010 -->

#### Out of scope
Crash-capture format (OBS). Symbol upload (BLD-038). Watchdog wiring (KRN).

#### Acceptance criteria
- [ ] A committed page documents QEMU gdbstub attach, kgdb, early console and drgn analysis of a crash dump on H-001.
- [ ] CI breaks in the gdbstub, hits a documented kgdb breakpoint, captures early-console output, and symbolises a deliberate panic with drgn.
- [ ] The page names the QEMU profile from BLD-012 and does not invent a second machine type.

#### Verification
- Integration: debug-workflow job on H-001 covering gdbstub, kgdb, early console and drgn.
- Review: kernel architecture lead accepts the page on the pull request.

#### Evidence
- none

### BLD-009 · Provide a hermetic one-command Linux-host build and source mirror
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: BLD-002, BLD-004, BLD-005, BLD-013
- Baseline: §27, §50, §59
- Threats: T-007
- Invariants: I-090

V0 boots from a CI-built image reproducible from a tagged commit; this command is the contributor entry point (§59). Builds run in a hermetic sandbox with pinned content-addressed inputs, no network after fetch, committed lockfiles and a project-controlled source mirror keyed by content hash. Non-Linux hosts get a containerised Linux environment; native Windows and macOS host builds are out of 1.0 (I-090). The native `os env` sandbox waits for V1.

<!-- covers: GAP-0458, GAP-0092, GAP-0157, GAP-0161, GAP-0096 -->

#### Out of scope
Native `os env` hermetic builds (BLD-043). Remote cache (BLD-025). Image builder shared with release (INS-001, BLD-024).

#### Acceptance criteria
- [ ] From a clean Linux checkout, one documented command produces a bootable H-001 image whose inputs are content-hashed and whose build has no network after fetch.
- [ ] All crates, C dependencies, firmware, Linux sources and toolchain artifacts used by that command exist in the project mirror keyed by content hash, with committed lockfiles.
- [ ] A new dependency without a lockfile and mirror entry fails the build.
- [ ] The contributor path for macOS and Windows hosts is the containerised Linux environment; a native Windows or macOS toolchain is not documented or tested.
- [ ] A tagged commit rebuilds the same command on a second Linux machine and boots H-001.

#### Verification
- Integration: one-command build and H-001 boot from a tagged commit on two Linux hosts; a network-off after fetch rebuild succeeds.
- Manual: follow the committed contributor page on a Linux host with no ambient rustup toolchain.

#### Evidence
- none

### BLD-010 · Run the core benchmark suite and V0 demos in CI
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: BLD-012, BLD-006, BEN-005, BEN-007, BLD-001
- Baseline: §54, §59
- Benchmarks: B-001, B-004, B-013
- Risks: R-009
- Invariants: I-061, I-088

V0 benchmark gates are publish-only (§54, §59). BLD executes BEN harnesses on pinned H-001 and H-002 and records V0 demo runs. Numeric fail-on-regression is BLD-033; BEN owns methodology and the quiet fleet consumption.

<!-- covers: INV-1041, INV-1175 -->

#### Out of scope
Harness implementation and report skeleton (BEN-005). Quiet fleet (BLD-045, BLD-048). Blocking merge policy (BEN-033, BLD-033).

#### Acceptance criteria
- [ ] Nightly CI runs the BEN V0 harness set including B-001, B-004 and B-013 on H-001 and H-002 and commits reports under `reports/benchmarks/` for those B-IDs.
- [ ] The V0 demo pipeline runs under the guest agent on H-001 with traces retained as Evidence artifacts.
- [ ] A missing or failed publish job fails nightly; a numeric regression does not fail merge at V0.
- [ ] Published reports cite B-IDs and contain no performance claim without a harness name (I-061).

#### Verification
- Bench: B-001, B-004 and B-013 on H-001 and H-002; target per register (V0 publish).
- Demo: V0 Component-to-Channel-to-MemoryObject pipeline on H-001 via BLD-006.
- Review: BEN lead confirms BLD executes BEN harnesses and does not define targets.

#### Evidence
- none

### BLD-011 · Enforce license, provenance, unsafe and static-analysis merge gates
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: BLD-003, BLD-004, GOV-003, GOV-002, ABI-003, ABI-018, BEN-004
- Baseline: §50, §51, §57
- Threats: T-007
- Invariants: I-013, I-049, I-067

V0 merge is the cheapest place to settle license, provenance and style (§51, §57). Required pre-merge checks enforce the kernel license allowlist (no Apache-2.0-only or CDDL), SPDX headers, Signed-off-by and verified signatures, rustfmt and clippy with warnings denied, sparse, smatch, adapted checkpatch, cargo-deny, Miri where feasible, and a CI-generated unsafe inventory. ABI and BEN own their lint content; this task makes those jobs required. SDK `forbid(unsafe_code)` templates stay in SDK. Userspace allowlist scanning beyond cargo-deny is BLD-023.

<!-- covers: GAP-0002, GAP-0009, GAP-0037, GAP-0123, INV-0957, GAP-0124, GAP-0011, GAP-0155 -->

#### Out of scope
DCO versus CLA policy (GOV-002). Userspace allowlist publication (GOV-016). SDK templates (SDK). Full firmware and corpus license scan (BLD-023).

#### Acceptance criteria
- [ ] A kernel crate or source file whose license is Apache-2.0-only or CDDL fails pre-merge CI (I-067).
- [ ] A source file without `SPDX-License-Identifier` fails pre-merge CI.
- [ ] A merge-queue commit lacking Signed-off-by or a verified signature fails pre-merge CI.
- [ ] rustfmt drift, clippy warnings, sparse/smatch/checkpatch findings and cargo-deny hits fail pre-merge CI.
- [ ] CI publishes an unsafe-block inventory per native crate; a new unsafe block without a `SAFETY` comment fails the job.
- [ ] ABI-003, ABI-018 and BEN-004 are required checks of this gate.

#### Verification
- Unit: fixtures for missing SPDX, Apache-2.0-only crate, missing Signed-off-by, clippy warning, and undocumented unsafe on `qemu-x86_64`.
- Integration: each fixture is a rejected merge-queue entry.
- Review: GOV licensing reviewer confirms the kernel allowlist matches GOV-003.

#### Evidence
- none

### BLD-012 · Define the QEMU boot matrix, boot harness and kselftests
- Type: build
- Milestone: V0
- Status: todo
- Size: L
- Owner: none
- Depends on: BLD-003, BLD-009, KRN-017
- Baseline: §55, §59
- Risks: R-013
- Invariants: I-098

V0 primary CI is QEMU/KVM on H-001 (§59). This task declares the q35, OVMF with and without Secure Boot, TPM, virtio block/net/input/sound, CPU-count, memory and NUMA axes, drives boot-complete over serial, detects kernel and Rust panics, enforces timeouts, and exercises reboot and kexec. Nightly kselftests run for every retained subsystem named by KRN. Generation-selection cycles wait for V0.5. virtio-gpu compositor axes wait for V0.5.

<!-- covers: GAP-0111, GAP-0112, EXTRA-011 -->

#### Out of scope
Guest test agent (BLD-006). Generation-selection cycles (BLD-022). virtio-gpu compositor matrix (BLD-028). Matrix contents ownership (KRN-014).

#### Acceptance criteria
- [ ] A committed matrix document lists q35, OVMF with and without Secure Boot, TPM emulation, virtio block/net/input/sound, CPU counts 1, 2, 8 and 64, memory from 512 MB to 64 GB, and multi-node NUMA topologies.
- [ ] The harness reaches a boot-complete marker over serial on every matrix cell, fails on kernel panic or Rust panic, and fails on timeout.
- [ ] Reboot and kexec cycles complete on the default H-001 cell and leave a boot-complete marker after each.
- [ ] Nightly CI runs kselftests for every retained subsystem named by KRN-017 (I-098).
- [ ] virtio-gpu and generation-selection cells are absent from this matrix.

#### Verification
- Integration: full declared matrix on H-001; panic and timeout fixtures fail the job; reboot and kexec cells pass.
- Review: KRN lead confirms kselftest set matches the retained-mechanism inventory.

#### Evidence
- none

### BLD-013 · Establish pinned Rust-in-kernel toolchain and Kbuild integration
- Type: build
- Milestone: V0
- Status: todo
- Size: L
- Owner: none
- Depends on: BLD-004, KRN-004, KRN-010
- Baseline: §50, §51
- Risks: R-002
- Invariants: I-082, I-089

V0 native kernel subsystems need Rust-in-kernel so they can land (§50). This task pins rustc, the allowed unstable features, bindgen and Kbuild integration, ties upgrades to the Rust-for-Linux cadence chosen by KRN-004, and runs a next-candidate toolchain canary that does not gate the primary kernel build. Userspace rustc is a separate pin.

<!-- covers: INV-0007, INV-0936, GAP-0089 -->

#### Out of scope
Pinning policy Decision (KRN-004). Userspace rustc (BLD-046). Rewrite-versus-retain (KRN).

#### Acceptance criteria
- [ ] The fork builds with a committed rustc version, allowed-unstable-feature set and bindgen/Kbuild integration on H-001.
- [ ] A next-candidate toolchain job builds the kernel and is non-blocking on the primary merge queue.
- [ ] Toolchain artifacts come from the content-hash mirror used by BLD-009; no ambient rustup is required.
- [ ] rustc and LLVM carry only upstream-bound patches (I-089); a forked LLVM is not in the graph.

#### Verification
- Integration: pinned toolchain kernel build and next-candidate canary on H-001.
- Review: KRN lead confirms the pin matches KRN-004.

#### Evidence
- none

### BLD-014 · Run accessibility-tree dump tests for the four V0.5 apps in CI
- Type: build
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: BLD-027, ACC-003, UIP-001
- Baseline: §41, §42, §60

V0.5 requires every UI widget to emit accessibility metadata inspectable via `os inspect`, verified by a tree dump on Terminal, File Browser, Text Editor and Image Viewer (§60). UIP and ACC own metadata and the dump tool; BLD runs the dump in the GUI harness on H-003 and H-002.

#### Out of scope
Tree schema and dump command (ACC-003, ACC-002). Widget emission (UIP-001). Harness implementation (BLD-027).

#### Acceptance criteria
- [ ] Post-merge CI dumps the accessibility tree of Terminal, File Browser, Text Editor and Image Viewer on H-003.
- [ ] The same dumps run on H-002 in nightly CI.
- [ ] A widget missing role, name or state fails the job with the app and widget identifier in the log.
- [ ] Dumps are taken through BLD-027, not through pixel capture.

#### Verification
- Integration: four-app dump on H-003 and H-002; a fixture unlabeled button fails the job.
- Review: ACC lead confirms the dump is ACC-003 output.

#### Evidence
- none

### BLD-015 · Run the ABI conformance and cross-version suite in CI
- Type: build
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: ABI-024, BLD-001, BLD-006
- Baseline: §65, §66
- Invariants: I-040, I-041

No Layer 1 freeze happens before V4 (I-040). ABI owns suite content at V0.5; BLD wires it into post-merge CI via the guest agent so every prototyped entry point has a running conformance case and the V0 negotiation test stays green.

<!-- covers: INV-1282 -->

#### Out of scope
Suite content (ABI-024). V1 cross-version SDK jobs (BLD-037). Layer 1 freeze (ABI-049).

#### Acceptance criteria
- [ ] Post-merge CI runs ABI-024 inside the guest agent on H-001.
- [ ] A new prototyped Layer 1 entry point without a suite case fails the job.
- [ ] The Layer 1 version-negotiation case from ABI remains a required check.

#### Verification
- Integration: conformance job on H-001; a fixture entry point without a case fails.
- Review: ABI lead confirms BLD runs the ABI suite without forking its content.

#### Evidence
- none

### BLD-016 · Adapt syzkaller to the Native ABI with a Capability-aware executor
- Type: build
- Milestone: V0.5
- Status: todo
- Size: L
- Owner: none
- Depends on: BLD-006, IPC-012, IPC-029, ABI-023
- Baseline: §7, §12, §18, §51, §65

Stock syzkaller cannot express typed, capability-scoped calls; the Native ABI is where novel kernel bugs will live (§7, §65). This port adds syzlang for handles, Channels, Operations and MemoryObjects, a capability-aware executor that obtains and derives Capabilities, and IDL-generated descriptions. Continuous triage infrastructure waits for V1. Per-interface mutators stay in IPC.

<!-- covers: GAP-0127, GAP-0548 -->

#### Out of scope
Crash dedup, bisection and task filing (BLD-035). IDL mutator emission (IPC-029). Userspace parser fuzz (BLD-042).

#### Acceptance criteria
- [ ] A syzkaller executor on H-001 obtains a Capability, derives an attenuated Capability, and submits Operations against handles, Channels and MemoryObjects.
- [ ] IDL-generated syzlang descriptions build from ABI-023 and IPC-029.
- [ ] A forged handle or non-subset derive is rejected with `Error::Rights` and does not crash the executor.
- [ ] Nightly CI runs the port for a committed duration and stores corpus plus crashers as artifacts, without requiring triage automation.

#### Verification
- Fuzz: Native ABI syzkaller on H-001 nightly; no executor panic on the capability-aware path.
- Integration: IDL-generated descriptions compile and execute one Channel and one MemoryObject program via BLD-006.
- Review: IPC lead confirms per-interface mutators remain IPC-owned.

#### Evidence
- none

### BLD-017 · Provide CI plumbing for compatibility-Corpus jobs
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: BLD-006, BLD-001
- Baseline: §3, §46, §49
- Corpora: C-001, C-002

V0.5 L1 corpus gates and later L2 through L5 and W1 through W3 jobs need a guest-agent path that runs corpus scenarios and records C-ID results (§3, §46). LNX and WIN own scenario content; BLD owns the job shape, artifact layout and report path under `reports/compat/`.

<!-- covers: INV-0865 -->

#### Out of scope
L0 and L1 scenario content (LNX-002, LNX-007). W1 definition (WIN-009). Corpus legal review (GOV-031).

#### Acceptance criteria
- [ ] A guest-agent job kind runs a named C-ID scenario set and writes `reports/compat/<C-ID>/` artifacts with pass or fail per entry.
- [ ] C-001 and C-002 jobs are wired on H-001 using this plumbing; scenario binaries come from LNX.
- [ ] A scenario timeout or guest panic fails the job and preserves serial logs plus the partial result file.
- [ ] Native software is not linked against personality corpus binaries in these jobs.

#### Verification
- Compat: C-001 and C-002 plumbing on H-001 with an LNX-provided fixture scenario.
- Integration: panic and timeout fixtures fail the job and retain logs.

#### Evidence
- none

### BLD-018 · Decide a semantic-Interface GUI test harness over pixel scripting
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: ACC-002
- Baseline: §41, §42, §60
- Decision: D-0039

V0.5 four apps and compositor-restart need an end-to-end GUI harness (§42, §60). The project's own rule is that Semantic interfaces beat GUI scraping; this Decision records that the harness drives applications through Semantic interfaces and the accessibility tree. Pixel goldens stay reserved for compositor rendering (GFX).

<!-- covers: GAP-0118 -->

#### Out of scope
Harness implementation (BLD-027). Pixel goldens (GFX-025). Tree schema (ACC-002).

#### Acceptance criteria
- [ ] Options evaluated include Semantic interfaces plus the accessibility tree, coordinate scripting, and pixel goldens as the primary driver.
- [ ] The accepted option states that pixel goldens are reserved for compositor rendering correctness and are not the app-scenario driver.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: BLD test-infra owner, ACC lead and GFX lead sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- none

### BLD-019 · Turn on deterministic compile and link flags for Packages
- Type: build
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: BLD-009, PKG-035
- Baseline: §27, §28

V0.5 immutable Packages are content-addressed (§27, §28). Path, timestamp and codegen-unit flags must be pinned so two builds of identical content yield the same Package identity before the V1 double-build gate.

<!-- covers: GAP-0094 -->

#### Out of scope
Double-build merge gate (BLD-041). Package builder (PKG-035). Store addressing (STO).

#### Acceptance criteria
- [ ] Kernel and userspace compile and link flags pin paths, timestamps, codegen units, hash seeds and embedded absolute paths to hermetic values.
- [ ] Two `os package build` invocations of identical content on one machine produce the same Package hash.
- [ ] A flag regression that reintroduces a timestamp or absolute path fails a determinism unit test.

#### Verification
- Unit: `bld:tests/determinism_flags_*` on `qemu-x86_64`.
- Integration: two Package builds of the V0.5 Terminal sources compare equal hashes.

#### Evidence
- none

### BLD-020 · Build a fault-injection framework for service kill and kernel failures
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: BLD-006, SVC-015
- Baseline: §32
- Benchmarks: B-023, B-024

V0.5 compositor-restart and SVC supervision need CI that kills services, injects allocation and I/O failures, and checks rebind (§32). BLD owns the injection and observation plumbing; SVC, GFX and others own per-service expectations.

<!-- covers: GAP-0132, INV-0608 -->

#### Out of scope
Supervisor policy (SVC-015). Compositor rebind protocol (GFX-009). Per-service kill matrices (SVC, AUD, NET, HW, GFX).

#### Acceptance criteria
- [ ] CI can kill a named supervised Component in the guest and observe restart plus client rebind through the guest agent.
- [ ] CI can inject kernel allocation failure and I/O failure during a nominated test and record whether the service rebinds or reports degraded recovery.
- [ ] The compositor kill-and-rebind loop from GFX-007 runs through this framework on H-003.
- [ ] Injection is Capability-gated inside the guest; unauthorised Components cannot trigger it.

#### Verification
- Integration: compositor kill loop on H-003 and a second supervised service kill on H-001.
- Bench: B-023 and B-024 publication jobs consume traces from this framework; target per register.

#### Evidence
- none

### BLD-021 · Publish a flaky-test policy and automate quarantine in CI
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: BLD-001
- Baseline: §59

Desktop and hardware jobs will flake. Unmanaged flakes erode trust until people merge on red. Policy: automatic quarantine after a defined flake rate, at most one retry, a time-boxed deflake obligation on the owning workstream, and quarantine expiry that deletes or fixes the test. V4 elimination depends on this existing.

<!-- covers: GAP-0120 -->

#### Out of scope
Per-commit dashboard (BLD-052). V4 empty-quarantine program (BLD-072).

#### Acceptance criteria
- [ ] A committed policy states the flake-rate trigger, the single-retry cap, the owning-workstream deflake obligation, and that expiry deletes or fixes the test.
- [ ] CI auto-quarantines a test that exceeds the trigger and records the owner prefix.
- [ ] A quarantined test is not a required merge check; a non-quarantined failure is.
- [ ] Quarantine entries carry an expiry tied to a Milestone, not a calendar date; expired entries fail CI until deleted or fixed.

#### Verification
- Integration: fixture test that fails intermittently is quarantined after the trigger and no longer blocks merge; an expired quarantine fails CI.
- Review: GOV process reviewer accepts the policy page.

#### Evidence
- none

### BLD-022 · Extend the boot harness to SystemGeneration selection cycles
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: BLD-012, PKG-015
- Baseline: §30, §31, §60

V0.5 exit requires that selecting the previous generation at boot restores kernel, compositor and Packages (§30, §60). The V0 harness covered reboot and kexec only. This extends it with firmware-driven boot-menu selection on H-001 and H-002.

<!-- covers: GAP-0112 -->

#### Out of scope
Boot menu presentation (BOOT-014). Generation compose and rollback test content (PKG-016, PKG-015).

#### Acceptance criteria
- [ ] The harness selects the previous SystemGeneration at the boot menu on H-001 over serial or firmware and reaches boot-complete.
- [ ] After selection, the running kernel, compositor Package and application Packages match the previous generation's pins.
- [ ] The same cycle runs on H-002 in nightly CI.
- [ ] A broken new generation that the boot counter rejects leaves the previous generation bootable.

#### Verification
- Integration: automated boot-menu cycle on H-001 and H-002 using PKG-015 content.
- Review: BOOT lead confirms the harness drives BOOT-014 rather than a second menu.

#### Evidence
- none

### BLD-023 · Run license compliance scanning in CI across kernel, crates and firmware
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: BLD-011, GOV-016, GOV-003
- Baseline: §50, §51
- Threats: T-007
- Invariants: I-067, I-068

V0.5 ships the first immutable image. SPDX and cargo-deny run with a zero-unknown gate across kernel, crates and firmware. GOV owns the allowlist Decision and the userspace allowlist document; BLD enforces both. Corpus redistribution review stays GOV.

<!-- covers: GAP-0155, GAP-0010, GAP-0011 -->

#### Out of scope
Allowlist policy (GOV-003, GOV-016). SBOM generation (BLD-055). Firmware ship-versus-download policy (HW, GOV-022).

#### Acceptance criteria
- [ ] CI scans kernel, crates and firmware for SPDX identities and fails on unknown licenses.
- [ ] A userspace dependency whose license is AGPL, SSPL or BUSL fails the default-image scan (I-068).
- [ ] A kernel dependency whose license is Apache-2.0-only or CDDL fails the scan (I-067).
- [ ] The scanner reads GOV-016 without a duplicated table.

#### Verification
- Integration: fixtures for unknown SPDX, AGPL crate and CDDL kernel dependency fail the job.
- Review: GOV licensing reviewer confirms the scanner consumes the published allowlist path.

#### Evidence
- none

### BLD-024 · Produce Milestone test images from the shared image builder
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: INS-001, BLD-009, PKG-016
- Baseline: §30, §60
- Invariants: I-094

Test images that diverge from shipped images test the wrong thing. INS owns the image builder; BLD produces the V0.5 desktop test image and later milestone images from that builder. V0 continues to use the hermetic QEMU smoke image from BLD-009.

<!-- covers: GAP-0115 -->

#### Out of scope
Image builder implementation (INS-001). Developer install media (INS-004). V0 smoke image (BLD-009).

#### Acceptance criteria
- [ ] The V0.5 desktop test image is produced by INS-001 and boots to the four demo apps on H-001 and H-003.
- [ ] Package hashes in the test image match the content-addressed store identities from PKG-016.
- [ ] V0 H-001 smoke jobs continue to use the hermetic one-command image, not the desktop image.
- [ ] A documented image identifier maps each CI job to the builder revision that produced it.

#### Verification
- Integration: builder-produced desktop image boots H-001 and H-003; store hashes match PKG compose output.
- Review: INS lead confirms BLD does not fork the builder.

#### Evidence
- none

### BLD-025 · Deploy a shared content-addressed remote build cache
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: BLD-002, BLD-009, BLD-003
- Baseline: §27

A kernel plus userspace plus personality tree is too large to rebuild from scratch on every pre-merge run (§27 cached build outputs). Compiler and artifact caches are shared by CI and developers; a cache hit is accepted only when the output hash matches a reproducible rebuild of the same inputs.

<!-- covers: GAP-0101 -->

#### Out of scope
Orchestrator choice (BLD-002). Double-build identity gate (BLD-041). Store substrate (STO).

#### Acceptance criteria
- [ ] CI and a documented developer path share a content-addressed compiler and artifact cache.
- [ ] A cache hit is rejected when the artifact hash does not match a rebuild from the same input hashes.
- [ ] A cold cache still produces a bootable H-001 image via BLD-009.
- [ ] Cache credentials are scoped to fetch and put of hashed objects and do not grant repository push.

#### Verification
- Integration: warm-cache pre-merge on H-001 host runners; a tampered cache object is rejected; cold-cache rebuild boots H-001.
- Review: BLD CI owner confirms hits are hash-checked against reproducible outputs.

#### Evidence
- none

### BLD-026 · Define sanitizer debug and release build profiles and boot them nightly
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: BLD-013, BLD-012, BLD-004
- Baseline: §51

Memory-safety bugs in inherited C and unsafe Rust are caught only if sanitizer kernels actually boot (§51). Debug profile enables lockdep, KASAN, KCSAN, UBSAN, KFENCE and Rust debug assertions. Release is LTO plus stripped. A symbols-preserved release variant feeds later symbol upload. Nightly CI boots the debug profile on H-001.

<!-- covers: GAP-0103 -->

#### Out of scope
Linker, LTO and PGO policy Decision (BLD-039). Symbol upload (BLD-038). Userspace ASAN/LSAN/TSAN (BLD-059).

#### Acceptance criteria
- [ ] Committed debug, release-stripped and symbols-preserved profiles exist in the build graph.
- [ ] Nightly CI boots the debug profile on H-001 to the boot-complete marker with lockdep, KASAN, KCSAN, UBSAN, KFENCE and Rust debug assertions enabled.
- [ ] The release-stripped profile boots on H-001; its debuginfo is absent from the image and present in the symbols-preserved variant.
- [ ] LTO in the release profile follows the default until BLD-029 lands, and is documented as provisional.

#### Verification
- Integration: nightly debug-profile boot and release-stripped boot on H-001; a deliberate KASAN trip fails the debug job.
- Review: KRN lead confirms sanitizer flags match the fork's supported Kconfig.

#### Evidence
- none

### BLD-027 · Build the semantic-Interface and accessibility-tree GUI test harness
- Type: build
- Milestone: V0.5
- Status: todo
- Size: L
- Owner: none
- Depends on: BLD-018, BLD-006, BLD-028
- Baseline: §41, §42, §60

Implements the accepted harness so V0.5 scripted app scenarios and compositor-rebind run in QEMU CI without pixel scraping (§42). Drivers speak Semantic interfaces and the accessibility tree; compositor rendering goldens remain GFX.

<!-- covers: GAP-0118, INV-0796 -->

#### Out of scope
Decision (BLD-018). Pixel goldens (GFX-025). Tree test driver content (ACC-014). App scenario scripts (APP).

#### Acceptance criteria
- [ ] The harness launches a guest on H-003, dumps the accessibility tree, and invokes a Semantic action or tree action on each of the four V0.5 apps.
- [ ] Compositor kill-and-rebind from BLD-020 is observable through the harness as rebound Surfaces, not as pixel diffs.
- [ ] A scenario that uses coordinate clicks or screenshot diffs as its primary assertion is rejected by harness lint.
- [ ] Harness logs include Component IDs and Interface names, inspectable via `os inspect`.

#### Verification
- Integration: four-app scripted scenarios on H-003; compositor-rebind scenario on H-003.
- Review: ACC and UIP leads confirm the driver uses the tree and Semantic interfaces, not coordinates.

#### Evidence
- none

### BLD-028 · Add virtio-gpu QEMU configurations to the compositor CI matrix
- Type: build
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: BLD-012
- Baseline: §39, §60

V0.5 hardware scope includes QEMU/KVM with virtio-gpu for compositor CI (H-003). This extends the V0 q35 matrix so GFX jobs have a GPU-less path and a virtio-gpu path without requiring H-002 for every UI change.

Required by V0.5-G01 (Native compositor presents on the reference GPU): H-003 covers the same path in CI.

#### Out of scope
Compositor virtio-gpu implementation (GFX-041). Golden-image comparison (GFX-025). Physical GPU jobs (BLD-007).

#### Acceptance criteria
- [ ] H-003 is a declared CI cell: q35 plus virtio-gpu, booting to compositor-ready via the boot harness.
- [ ] GFX compositor jobs can select GPU-less H-001 or virtio-gpu H-003 as named matrix axes.
- [ ] A virtio-gpu boot failure fails the compositor CI cell and captures serial plus QEMU logs.

#### Verification
- Integration: H-003 boot-complete and a GFX compositor smoke job on H-003.
- Review: GFX lead confirms the cell matches GFX-041.

#### Evidence
- none

### BLD-029 · Apply the chosen linker, LTO and PGO policy in CI profiles
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: BLD-039, BLD-041, BLD-026
- Baseline: §50, §27
- Risks: R-060

Implements BLD-039 in debug and release profiles and in the double-build job so the V1 image is the policy, not a local override. Where the Decision refuses PGO, CI profiles do not enable it.

<!-- covers: GAP-0104 -->

#### Out of scope
Decision and spike (BLD-039, BLD-051). Independent rebuilders (BLD-074).

#### Acceptance criteria
- [ ] Debug and release CI profiles use the accepted linker and LTO scope on H-001 and H-002 builds.
- [ ] PGO is enabled only where the Decision allows it; a profile that enables refused PGO fails CI.
- [ ] The double-build job from BLD-041 still diffs equal under the applied policy.
- [ ] Sanitizer debug boots from BLD-026 still boot on H-001 with the new linker.

#### Verification
- Integration: profile builds on H-001; double-build diff under the policy; sanitizer boot on H-001.
- Review: BLD toolchain owner confirms profiles match the accepted Decision.

#### Evidence
- none

### BLD-030 · Keep an ARM64 kernel and userspace cross-build compiling in CI
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: BLD-013, BLD-009, BLD-046
- Baseline: §38, §50
- Risks: R-083
- Invariants: I-001, I-011, I-100

§38 forbids fossilizing on x86-64. This standing compile-only ARM64 job builds the kernel, Native ABI headers and a smoke Component without shipping or booting ARM64 (I-001, I-011). KRN owns RISC-V build-only CI.

<!-- covers: GAP-0105, INV-0718 -->

#### Out of scope
RISC-V build-only CI and the port plan (KRN-043). Shipping ARM64 (LATER). ABI header content (ABI).

#### Acceptance criteria
- [ ] Post-merge CI cross-compiles the kernel for ARM64 and fails on breakage.
- [ ] The same job compiles Native ABI headers and a smoke Component for ARM64.
- [ ] The job does not boot ARM64 and does not add ARM64 to hardware scope.
- [ ] x86-64 remains the only shipped architecture in image-builder output (I-001).

#### Verification
- Integration: ARM64 kernel plus smoke Component compile job on every merge; a deliberate ARM64 break fails the check.
- Review: ABI lead confirms the smoke Component uses the Native ABI headers, not a POSIX shim.

#### Evidence
- none

### BLD-031 · Automate CI-validated dependency update proposals
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: BLD-034, BLD-009, BLD-023
- Baseline: §50, §56.4
- Threats: T-007

Manual tracking across a kernel fork and a Rust ecosystem falls behind during daily-driving (§56.4). Crates, C libraries, firmware, QEMU and upstream Linux updates arrive as CI-validated changes with a documented review cadence. Landing remains a human merge-queue decision.

<!-- covers: GAP-0154 -->

#### Out of scope
Upstream rebase strategy (KRN). Firmware ship policy (HW). License allowlist (GOV).

#### Acceptance criteria
- [ ] A documented job proposes updates for crates, C libraries, firmware, QEMU and the pinned upstream Linux tag as change sets with lockfile and mirror diffs.
- [ ] Each proposal runs BLD-009, BLD-023 and H-001 smoke boot before it is offered for review.
- [ ] A proposal that fails license scan or smoke boot is not offered as ready.
- [ ] The review cadence is recorded as a Milestone-relative obligation, not a calendar date.

#### Verification
- Integration: fixture crate bump that builds and boots is offered; a license-failing bump is not.
- Review: KRN and BLD owners accept the cadence page.

#### Evidence
- none

### BLD-032 · Track build-time budgets on the reference runner
- Type: benchmark
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: BLD-009, BLD-043, BEN-007
- Baseline: §54, §61
- Benchmarks: B-039
- Invariants: I-061, I-088

Incremental kernel, incremental userspace and clean CI image builds are measured and gated by B-039, not by prose (§54). V1 records the register targets; this task is the reference-runner harness BLD owns. BEN publishes methodology and comparisons.

<!-- covers: GAP-0102 -->

#### Out of scope
Methodology Decision (BEN-007). Publication versus Linux (BEN-023). Self-host graph (BLD-049).

#### Acceptance criteria
- [ ] CI measures clean full-image, incremental kernel and incremental userspace build times on the nominated reference runner via harness `bench:build-time`.
- [ ] Reports land under `reports/benchmarks/B-039/` for that runner and cite B-039 only.
- [ ] A result that misses the B-039 target kind for V1 fails the benchmark job.
- [ ] No task, gate or dashboard restates a build-time number except by citing B-039 (I-088).

#### Verification
- Bench: B-039 on the reference runner; target per register.
- Review: BEN lead confirms the harness matches `bench:build-time` and that numbers live only in the register and reports.

#### Evidence
- none

### BLD-033 · Fail merges that regress beyond the calibrated performance band
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: BEN-033, BLD-048, BLD-010, BEN-022
- Baseline: §54
- Benchmarks: B-001, B-004, B-016
- Invariants: I-061

V0 executed the suite publish-only. V1 applies BEN-033 on the quiet fleet: regressions outside the calibrated noise band block merge unless an adr-linked exception exists. BLD owns the job; BEN owns the policy and noise calibration.

<!-- covers: INV-1041 -->

#### Out of scope
Merge-gate policy Decision (BEN-033). Quiet fleet configuration (BLD-048). Harness content (BEN).

#### Acceptance criteria
- [ ] Quiet-fleet jobs for the metrics named by BEN-033 run as required checks on the merge queue.
- [ ] A regression outside the calibrated noise band fails the check unless the change links an accepted Decision that names the B-ID.
- [ ] Publish-only V0 behavior is retained on H-001; H-001 numbers do not fail merge.
- [ ] Exception records cite a Decision file, not a comment in the job log.

#### Verification
- Bench: B-001, B-004 and B-016 on the quiet fleet; target per register and BEN-033.
- Integration: fixture regression without a Decision fails merge; the same change with an accepted Decision exception lands.

#### Evidence
- none

### BLD-034 · Manage CI and lab infrastructure as code with disaster recovery
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: BLD-003, BLD-001
- Baseline: §61
- Risks: R-053

V1 daily-driving depends on a recreatable pipeline from the repository plus backups. BLD owns runner, cache, queue and secret layout as code. Lab machine definitions stay in LAB. The drill that proves recovery is BLD-053.

<!-- covers: GAP-0108 -->

#### Out of scope
Lab machine and scheduler definitions (LAB-010, LAB-011). Recovery drill (BLD-053). Funding (Q-053, GOV).

#### Acceptance criteria
- [ ] Runner, cache, merge-queue and secret-layout definitions live in the repository and apply without undocumented console steps.
- [ ] A committed recovery document lists the backup set required to recreate the pipeline.
- [ ] Secrets are referenced by name from code and are not committed.
- [ ] Lab hostnames and SKUs are not duplicated; BLD consumes LAB machine identifiers.

#### Verification
- Review: BLD CI owner accepts the infrastructure tree and recovery document.
- Integration: applying the repository definitions recreates a lint-and-smoke runner that boots H-001.

#### Evidence
- none

### BLD-035 · Run continuous Native ABI fuzzing with dedup, bisect and Task filing
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: BLD-016, BLD-034
- Baseline: §51, §65
- Threats: T-003

Fuzzing without triage produces a crash backlog nobody works (§51). This syzbot-like layer on the V0.5 syzkaller port does crash dedup, reproducer minimisation, automatic bisection and filing into the Markdown repository, including IDL deserialisers. V3 and V4 crasher-age gates consume its inventory. Userspace parser fuzz is a separate fleet.

<!-- covers: GAP-0128, INV-0961, GAP-0130 -->

#### Out of scope
syzkaller port (BLD-016). Userspace parser corpora (BLD-042). Per-interface mutators (IPC). Crasher-age gates (BLD-063, BLD-073).

#### Acceptance criteria
- [ ] Continuous Native ABI fuzzing runs on the V0.5 port with crash deduplication and minimized reproducers stored as artifacts.
- [ ] A new unique crasher is bisected to a commit and a draft task is filed in the Markdown repository with the reproducer path.
- [ ] IDL deserialiser harnesses from IPC-029 are registered in the inventory.
- [ ] Open crashers and their ages are queryable for BLD-063.

#### Verification
- Fuzz: continuous Native ABI fleet; a planted crasher is deduped, minimized, bisected and filed.
- Review: GOV tooling owner confirms filed tasks parse as roadmap tasks.

#### Evidence
- none

### BLD-036 · Measure kernel and userspace coverage and Gate security-critical paths
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: BLD-026, BLD-001
- Baseline: §51, §65

Security-critical paths need evidence of exercise before SDK v1 freeze candidates (§51, §65). kcov/gcov and Rust coverage run in nightly CI with minimum gates on capability enforcement, ABI entry points and IDL-generated paths. Thresholds live in committed coverage policy, not in prose performance claims.

<!-- covers: GAP-0122 -->

#### Out of scope
Sanitizer boots (BLD-026). Fuzz coverage (BLD-035). Freeze candidates (ABI).

#### Acceptance criteria
- [ ] Nightly CI publishes kernel (kcov/gcov plus Rust) and userspace coverage reports for the default image.
- [ ] Capability enforcement, Native ABI entry points and IDL-generated paths have committed minimum coverage gates that fail nightly when missed.
- [ ] A new ABI entry point or IDL method without coverage in those gates fails the job.
- [ ] Coverage reports do not contain performance numbers.

#### Verification
- Integration: nightly coverage job; a fixture ABI entry with no test fails the gate.
- Review: CAP and ABI leads accept the path list that is gated.

#### Evidence
- none

### BLD-037 · Run SDK cross-version compatibility jobs in CI
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: SDK-036, BLD-024, BLD-015
- Baseline: §66, §61

V1 exit requires that applications built against SDK v1 run on later v1.x generations and the reverse where promised (§66). SDK owns the suite; BLD runs it on builder-produced images via the guest agent.

#### Out of scope
Suite content (SDK-036). ABI conformance cases (ABI, BLD-015). Host SDK packages (SDK-039).

#### Acceptance criteria
- [ ] CI builds a sample app against SDK v1.0.0, runs it on the current v1.x image, and records the SDK suite result.
- [ ] CI runs the reverse job where the suite marks it promised.
- [ ] Jobs use images from BLD-024, not ad-hoc developer images.
- [ ] A suite failure fails the job and is a required post-merge check.

#### Verification
- Integration: SDK-036 on builder images via the guest agent.
- Review: SDK lead confirms BLD does not fork suite content.

#### Evidence
- none

### BLD-038 · Upload content-hashed debug symbols for every stripped Generation
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: BLD-026, OBS-028
- Baseline: §24, §61

V1 daily-driving and crash reports need symbols for immutable stripped binaries. OBS packages debuginfo; BLD publishes the symbols-preserved profile by content hash so a stripped SystemGeneration can be symbolicated. Debuginfod hosting is REL/OBS operation.

<!-- covers: GAP-0362 -->

#### Out of scope
Debuginfod server operation (OBS, REL). Capture format (OBS). Crash-report client (INS).

#### Acceptance criteria
- [ ] Every stripped SystemGeneration CI produces has a content-hashed symbols-preserved artifact uploaded beside it.
- [ ] The hash of the stripped binary maps to the debuginfo object; a mismatch fails the upload job.
- [ ] OBS-028 can retrieve those objects by build-id or content hash in a CI smoke lookup.

#### Verification
- Integration: stripped image plus symbol upload; OBS symbolicator resolves a deliberate panic from the stripped generation.
- Review: OBS lead confirms hosting remains OBS/REL and BLD only publishes hashed artifacts.

#### Evidence
- none

### BLD-039 · Decide linker, LTO scope and PGO policy for kernel and userspace
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: BLD-051
- Baseline: §27, §50, §54
- Decision: D-0035
- Risks: R-060

Link-time choices affect binary size, startup and reproducibility, and PGO in particular threatens bit-for-bit SystemGeneration identity (§27, §50). Options must include refusing PGO where it breaks identity. The spike report is required evidence.

<!-- covers: GAP-0104 -->

#### Out of scope
Spike measurements (BLD-051). Profile application (BLD-029). Image-size publication (BEN, B-044).

#### Acceptance criteria
- [ ] Options evaluated include lld, mold, and the status-quo linker, each with LTO off, thin LTO and full LTO, and PGO on versus PGO refused where identity breaks.
- [ ] The accepted option cites BLD-051 and states whether PGO is refused for any profile that must be bit-for-bit identical.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: BLD toolchain owner and BEN lead sign-off recorded on the pull request that accepts the Decision file, citing the spike report.

#### Evidence
- none

### BLD-040 · Demonstrate clone, edit, build, install, reboot and rollback on the OS
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: BLD-049, BLD-043, PKG-060, BOOT-014, ENV-020
- Baseline: §35, §50, §59

V1 demo: on H-002, clone the OS repository on the OS, edit a kernel file, build the image inside `os env`, install as a new SystemGeneration, reboot into it, and roll back through the boot menu (§59). This task is the scripted demonstration, not the build orchestrator.

#### Out of scope
Hermetic env builds (BLD-043). Generation restore CLI (PKG-060). Boot menu (BOOT-014). Reproducible two-rebuilder proof (BLD-080).

#### Acceptance criteria
- [ ] A scripted run on H-002 clones the repository, edits a kernel file, builds inside `os env`, installs a new SystemGeneration and reboots into it.
- [ ] Selecting the previous SystemGeneration at the boot menu restores the previous kernel and userspace.
- [ ] The demo is the Verified-by task of V1-D01.

#### Verification
- Demo: V1-D01 on H-002.
- Manual: the scripted run is recorded as Evidence on the pull request.

#### Evidence
- none

### BLD-041 · Eliminate artifact non-determinism and double-build on every merge
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: BLD-019, BLD-047, STO-009
- Baseline: §27, §61
- Risks: R-060
- Invariants: I-036, I-062

V1 self-host requires bit-for-bit image identity across two machines (§27, §61). CI builds twice, diffs artifacts, and verifies Package hashes against the content-addressed store. STO owns addressing; BLD owns the rebuild check.

<!-- covers: GAP-0094, INV-0517 -->

#### Out of scope
Determinism flags (BLD-019). Store substrate (STO-009). Independent rebuilders (BLD-074).

#### Acceptance criteria
- [ ] Every merge-queue land builds the default image twice from identical inputs and diffs artifacts byte-for-byte.
- [ ] Package hashes match objects in the content-addressed store; a mismatch fails the job.
- [ ] The two builds may run on two machines; host path differences do not appear in artifacts.
- [ ] A planted timestamp or path leak fails the double-build job.

#### Verification
- Integration: double-build on merge to the default branch; leak fixture fails.
- Review: STO lead confirms hash verification uses the store, not a parallel checksum table.

#### Evidence
- none

### BLD-042 · Fuzz userspace parsers of untrusted input with in-tree corpora
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: BLD-009, BLD-001
- Baseline: §28, §51
- Threats: T-006

Userspace parser fuzzing is distinct from Native ABI syzkaller. cargo-fuzz corpora live in the repository for Package manifests, environment.yaml, fonts, images, PE/ELF and Capability serialisation. Owning workstreams supply harnesses; BLD runs them nightly and files crashers. Size L because parsers span PKG, ENV, TXT, MED, LNX, WIN and CAP.

<!-- covers: GAP-0130 -->

#### Out of scope
Native ABI syzkaller (BLD-016). Continuous kernel triage (BLD-035). Per-parser semantics (PKG, ENV, TXT, MED, LNX, WIN, CAP).

#### Acceptance criteria
- [ ] Nightly CI runs in-tree cargo-fuzz harnesses for Package manifests, environment.yaml, fonts, images, PE/ELF and Capability serialisation.
- [ ] Corpora are committed in the repository; a harness without a corpus entry fails inventory CI.
- [ ] Unique userspace crashers are filed as roadmap tasks with the corpus input attached.
- [ ] Native parser harnesses do not load POSIX path APIs as the untrusted-input surface.

#### Verification
- Fuzz: listed harnesses nightly; a planted manifest crasher is filed.
- Review: PKG, TXT, MED, CAP, LNX and ENV leads confirm harness ownership versus BLD execution.

#### Evidence
- none

### BLD-043 · Run hermetic builds inside native development environments
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: ENV-013, BLD-009, BLD-047, SDK-041
- Baseline: §35, §36, §61
- Invariants: I-019, I-043

GAP-0092 required the native environment primitive (§35) once it exists. V1 `os env` becomes the sandbox for self-hosted builds, replacing the Linux-host hermetic equivalent for work that runs on the OS. Builds still have pinned inputs and no network after fetch.

<!-- covers: GAP-0092 -->

#### Out of scope
DevelopmentEnvironment object (ENV-013). `os env` CLI (SDK-041). Full self-host graph (BLD-049).

#### Acceptance criteria
- [ ] A documented `os env` definition builds the kernel and native userspace with no network after fetch and no ambient host toolchain.
- [ ] The environment does not depend on an OCI runtime, Docker daemon or Linux VM (I-043).
- [ ] Input hashes match the content-addressed sysroot from BLD-047.
- [ ] The Linux-host one-command path remains documented for bootstrap.

#### Verification
- Integration: `os env` hermetic build on H-002; network-off after fetch succeeds.
- Review: ENV lead confirms this sandbox is ENV-013, not a second container runtime.

#### Evidence
- none

### BLD-044 · Wire the hardware lab scheduler into nightly and post-merge CI
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: LAB-010, BLD-001, BLD-007
- Baseline: §55, §61, §62

V1 Intel-laptop gates (Wi-Fi, suspend cycles, GPU-accelerated Linux apps) put the lab on the critical path. LAB owns LAVA-like scheduling; BLD submits jobs and consumes results so nightly and selected post-merge checks include H-002 and H-004.

<!-- covers: GAP-0372 -->

#### Out of scope
Scheduler implementation (LAB-010). Machine racking (LAB). Suspend fixtures (LAB-009). Quiet perf fleet (BLD-045).

#### Acceptance criteria
- [ ] Nightly CI submits lab jobs through LAB-010 and fails when H-002 or H-004 jobs fail or expire.
- [ ] Selected post-merge kernel changes run BLD-007 on H-002 via the scheduler.
- [ ] An unbootable machine is reported as a lab failure, not as a passing skip.
- [ ] BLD does not embed SKU or PDU details; it references LAB machine IDs.

#### Verification
- Integration: nightly H-002 and H-004 jobs; a forced lab failure fails the CI aggregator.
- Review: LAB lead confirms BLD is a scheduler client.

#### Evidence
- none

### BLD-045 · Stand up the bare-metal performance CI fleet
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: BLD-034, BLD-003
- Baseline: §54
- Risks: R-009
- Invariants: I-061

The performance CI fleet is BLD capacity that BEN consumes for V1 blocking jobs, distinct from H-001 publish-only and from the general lab matrix. This task provisions those bare-metal runners. Quiet frequency and isolation policy is BLD-048. LAB hosts functional machines; this fleet is not the soak rack.

<!-- covers: GAP-0135 -->

#### Out of scope
Quiet configuration (BLD-048). Blocking hook (BLD-033). BEN harnesses (BEN). Lab scheduler for functional jobs (LAB).

#### Acceptance criteria
- [ ] A named set of bare-metal runners exists as code under BLD-034 and accepts BEN harness jobs.
- [ ] Fleet machines are not shared with functional QEMU or lab soak load.
- [ ] `os inspect` or the CI inventory lists each fleet machine separately from H-001 and from LAB functional hosts.
- [ ] V0 publish-only jobs on H-001 continue unchanged.

#### Verification
- Integration: a BEN harness job schedules on a fleet machine and not on an H-001 QEMU runner.
- Review: BEN lead confirms the fleet is the execution home for V1 blocking metrics.

#### Evidence
- none

### BLD-046 · Pin the userspace rustc and rustup profile for self-host builds
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: BLD-013, BLD-004
- Baseline: §50, §61
- Invariants: I-089

V1 self-host and SDK v1 need a pinned userspace rustc distinct from the kernel Rust-for-Linux pin, with a next-candidate canary that does not block kernel builds (§50, §61).

Required by V1-G01 (Self-hosted image builds bit-for-bit): a bit-for-bit image needs a pinned userspace toolchain.

#### Out of scope
Kernel pin (BLD-013, KRN-004). Sysroot contents (BLD-047). Layer 3 std crate (SDK).

#### Acceptance criteria
- [ ] Userspace builds use a committed rustc version and profile separate from the kernel pin.
- [ ] A userspace next-candidate canary builds native crates and is non-blocking on kernel merge.
- [ ] A userspace rustc bump does not change the kernel toolchain pin in the same change unless KRN agrees.
- [ ] The pin is an upstream rustc, not a forked compiler (I-089).

#### Verification
- Integration: userspace pin build and canary job; kernel pin remains the BLD-013 version.
- Review: SDK lead confirms the pin is the userspace SDK compiler.

#### Evidence
- none

### BLD-047 · Pin a reproducible userspace sysroot and toolchain artifacts
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: BLD-046, BLD-002, BLD-009
- Baseline: §27, §50, §61
- Invariants: I-089

V1 bit-for-bit self-host and later independent rebuilders need a content-addressed sysroot, not an ambient distro toolchain, while still carrying only upstream rustc and LLVM (§27, I-089).

<!-- covers: GAP-0092 -->

#### Out of scope
Userspace rustc version (BLD-046). Store substrate (STO). Independent rebuilders (BLD-074).

#### Acceptance criteria
- [ ] Userspace builds resolve libc, binutils, headers and rustc artifacts by content hash from the project mirror.
- [ ] A build with an ambient distro compiler on `PATH` still uses the pinned sysroot or fails closed.
- [ ] Sysroot objects are recorded in the build graph consumed by BLD-041.
- [ ] No forked LLVM is part of the sysroot (I-089).

#### Verification
- Integration: build with a conflicting ambient compiler still produces the hashed sysroot output; double-build remains equal.
- Review: BLD toolchain owner confirms sysroot contents are upstream artifacts.

#### Evidence
- none

### BLD-048 · Configure the performance CI fleet as a quiet measurement environment
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: BLD-045, BEN-022, BEN-007
- Baseline: §54
- Risks: R-009
- Invariants: I-061

V1 blocking gates need a quiet environment: fixed frequency, disabled turbo and SMT, isolated from other CI load. BLD-045 provides the machines; this task applies that policy so BEN-022 calibration remains valid. LAB does not rack this fleet as functional soak hardware.

<!-- covers: GAP-0135 -->

#### Out of scope
Machine provisioning (BLD-045). Merge-gate policy (BEN-033). Functional lab (LAB).

#### Acceptance criteria
- [ ] Fleet machines run BEN jobs at a documented fixed frequency with turbo and SMT disabled.
- [ ] Functional CI and soak jobs cannot schedule on fleet machines.
- [ ] A configuration drift that re-enables turbo or SMT fails a pre-job inventory check.
- [ ] Noise-floor records from BEN-022 name these machines as the V1 blocking environment.

#### Verification
- Integration: inventory check fails when turbo is enabled; a functional job cannot land on a fleet machine.
- Bench: a B-004 run on the quiet fleet records the frequency-pin and isolation flags in the report environment block.
- Review: BEN lead confirms the environment matches BEN-007.

#### Evidence
- none

### BLD-049 · Deliver Linux-host bootstrap and self-hosted full-system builds
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: BLD-043, BLD-009, BLD-041, ENV-013, SDK-041
- Baseline: §35, §61
- Risks: R-026
- Invariants: I-043

V1 exit: a clean checkout builds the full image on the OS inside `os env` and that image boots the V0 and V0.5 suites (§61). The Linux-host one-command path remains the bootstrap. ENV owns `os env`; BLD owns the build graph. Contributor steps stay a small fixed sequence on Linux and on JakeOS.

<!-- covers: GAP-0091, GAP-0390, INV-1211 -->

#### Out of scope
Environment object (ENV). `os env` CLI (SDK). Install media (INS-004). B-039 publication (BEN-023).

#### Acceptance criteria
- [ ] On H-002, `os env enter` plus the documented build command produces a full SystemGeneration image with no network after fetch.
- [ ] That image boots H-001 and H-002 and passes the V0 primitive suite and the V0.5 four-app smoke.
- [ ] The Linux-host one-command path still produces a bootable image used to reach the self-host environment.
- [ ] Contributor documentation on Linux and on JakeOS is a small fixed sequence of committed steps, exercised in CI.

#### Verification
- Integration: self-host image build on H-002; boot and V0/V0.5 suites on H-001 and H-002.
- Manual: follow the committed contributor pages on Linux and inside `os env`.
- Review: ENV lead confirms the sandbox is BLD-043.

#### Evidence
- none

### BLD-050 · Spike Component and ResourceDomain isolation for parallel tests
- Type: spike
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: BLD-006, CMP-005
- Baseline: §10, §22, §53
- Benchmarks: B-001

Once V0 primitives are stable, evaluate one Component per test as the isolation unit (§10, §53). Using the OS's own cheap isolation is a continuous proof that Component creation cost remains usable. The report measures creation cost against B-001 rather than claiming speedups.

<!-- covers: GAP-0163 -->

#### Out of scope
Guest agent protocol (BLD-006). Component creation implementation (CMP). OCI comparison publication (BEN, B-015).

#### Acceptance criteria
- [ ] The spike runs a parallel test fixture as one Component per case and as a co-located fixture, and records isolation failures (handle leak, ResourceDomain leak) if any.
- [ ] The report publishes B-001 for the per-test Component path on H-001 and H-002 without a superiority claim.
- [ ] The report recommends keep, defer or reject for default CI isolation, with the ResourceDomain budget required per test.

#### Verification
- Report: answers whether per-test Components isolate failures without leaked handles, what B-001 records for that path versus co-located fixtures, and what ResourceDomain budget each test needs.
- Bench: B-001 on H-001 and H-002 for the per-test Component fixture; target per register (publish at this spike).

#### Evidence
- none

### BLD-051 · Spike linker, LTO and PGO effects on binaries and benchmarks
- Type: spike
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: BLD-026, BLD-041, BEN-005
- Baseline: §27, §50, §54
- Benchmarks: B-039, B-016, B-044
- Risks: R-060

Must precede BLD-039. Measures lld versus mold, LTO scope and PGO against BEN harnesses and bit-for-bit rebuilds so PGO cannot silently break SystemGeneration identity (§27, §54).

<!-- covers: GAP-0104 -->

#### Out of scope
The Decision (BLD-039). Applying the policy (BLD-029).

#### Acceptance criteria
- [ ] The report records B-039, B-016 and B-044 for lld versus mold and for LTO off, thin and full on H-002.
- [ ] The report records whether PGO preserves bit-for-bit identity under BLD-041.
- [ ] Each option lists consequences for sanitizer boots and for independent rebuilders.

#### Verification
- Report: answers linker choice, LTO scope, whether PGO breaks identity, and which BEN reports the Decision must cite.
- Bench: B-039, B-016 and B-044 on H-002 for each linker and LTO option; target per register.

#### Evidence
- none

### BLD-052 · Deploy a per-commit test-results database and dashboard
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: BLD-001, BLD-034, BLD-021
- Baseline: §61
- Invariants: I-088

Years of results across a hardware matrix cannot be understood from individual CI logs. This KernelCI-style store records pass, fail, flake and duration per commit, configuration and machine. BEN owns the performance TSDB; this dashboard is functional jobs only and does not restate B-ID numbers.

<!-- covers: GAP-0121 -->

#### Out of scope
Benchmark TSDB (BEN-032). Public alpha dashboards (BLD-066). Flake policy (BLD-021).

#### Acceptance criteria
- [ ] Every functional CI job on H-001, H-002, H-003 and H-004 writes pass, fail, flake and duration keyed by commit, configuration and machine.
- [ ] The dashboard shows flake rate and duration trends and lists quarantined tests from BLD-021.
- [ ] Performance numbers are not displayed; links to BEN reports are allowed (I-088).
- [ ] Embargoed jobs are absent from any view that is not access-controlled.

#### Verification
- Integration: seeded jobs appear per commit and machine; a quarantined test is labelled.
- Review: BEN lead confirms no B-ID values are duplicated here.

#### Evidence
- none

### BLD-053 · Rehearse disaster recovery of the CI and lab pipeline
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: BLD-034
- Baseline: §61
- Risks: R-053

GAP-0108 is unproven until the pipeline is recreated from the repository plus backups in a drill, before public alpha depends on it. BLD runs the drill; LAB participates for machine definitions it owns.

<!-- covers: GAP-0108 -->

#### Out of scope
Infrastructure definitions (BLD-034). Lab scheduler internals (LAB). Funding (GOV).

#### Acceptance criteria
- [ ] A drill recreates runner, cache, queue and secret layout from the repository plus the documented backup set and boots H-001 smoke.
- [ ] The drill record lists gaps found and the tasks that close them; it contains no calendar date.
- [ ] Lab machine recovery, if in scope of the backup set, uses LAB identifiers rather than a BLD-side SKU list.

#### Verification
- Manual: execute the committed drill procedure and attach the record as Evidence.
- Review: BLD CI owner and LAB lead accept the drill record.

#### Evidence
- none

### BLD-054 · Decide SBOM format for Packages and SystemGenerations
- Type: adr
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: BLD-023
- Baseline: §27, §28
- Decision: D-0038
- Threats: T-007

The V2 generator needs SPDX versus CycloneDX versus both decided before store and repository SBOMs and before V3 attestations. REL publishes; this Decision only picks the format BLD emits from the content-addressed graph.

<!-- covers: GAP-0352 -->

#### Out of scope
Generator (BLD-055). Publication and signing (REL-051). License allowlist (GOV).

#### Acceptance criteria
- [ ] Options evaluated include SPDX only, CycloneDX only, and both emitted from the same graph.
- [ ] The accepted option states the unit of SBOM (Package and SystemGeneration) and that content hashes are the identity keys.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: BLD CI owner, REL lead and GOV licensing reviewer sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- none

### BLD-055 · Generate an SBOM from the content-addressed dependency graph
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: BLD-054, BLD-041, STO-009, PKG-016
- Baseline: §27, §28
- Threats: T-007
- Invariants: I-036

Mechanised compliance is BLD CI; REL publishes. V2 native repository needs per-Package and per-SystemGeneration SBOMs from the build graph, keyed by content hash (§27).

<!-- covers: GAP-0352, GAP-0010 -->

#### Out of scope
Format Decision (BLD-054). Release publication (REL-051). Release-qualification failure (BLD-068).

#### Acceptance criteria
- [ ] CI emits an SBOM in the accepted format for every Package and every SystemGeneration built on the default branch.
- [ ] Each SBOM entry is keyed by content hash and matches store objects from STO-009.
- [ ] Unknown or disallowed licenses already failing BLD-023 also fail SBOM generation.
- [ ] REL can consume the artifacts without rebuilding them.

#### Verification
- Integration: SBOM emission on a V2 test generation; hash keys match store objects.
- Review: REL lead confirms the artifact layout is the input to REL-051.

#### Evidence
- none

### BLD-056 · Run the 40-scenario desktop UX script in the GUI harness
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: BLD-027, APP-048, BLD-058
- Baseline: §62

V2 exit: the desktop shell passes the APP UX script on all three target machines (§62). APP owns scenarios; BLD runs them through the semantic harness on H-001, H-002, H-004 and H-005.

#### Out of scope
Scenario content (APP-048). Harness implementation (BLD-027). Pixel goldens (GFX).

#### Acceptance criteria
- [ ] CI runs APP-048 through BLD-027 on H-002, H-004 and H-005.
- [ ] A QEMU H-003 or H-001 run exists for the subset of scenarios that do not need physical displays.
- [ ] A failed scenario names the APP scenario ID and the machine; it does not assert on pixels.
- [ ] P0 and P1 scenario failures fail the job.

#### Verification
- Integration: full script on H-002, H-004 and H-005; subset on H-003.
- Review: APP lead confirms scenarios are APP-owned.

#### Evidence
- none

### BLD-057 · Run a QEMU-hosted JakeOS CI runner as the self-host step
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: BLD-049, INS-004, BLD-034
- Baseline: §61
- Risks: R-070
- Invariants: I-043

Migrating CI to JakeOS at V2 predates the installer that public alpha needs. The V2 step is a QEMU-hosted JakeOS runner executing a real CI job; bare-metal migration waits for V3. This dogfoods unattended operation without requiring LAB JakeOS hosts yet.

<!-- covers: GAP-0153 -->

#### Out of scope
Bare-metal JakeOS runners (BLD-064). Installer product (INS). Nested-virt KVM product (VIRT).

#### Acceptance criteria
- [ ] A QEMU-hosted JakeOS guest registered as a CI runner executes the H-001 smoke job from a self-hosted image.
- [ ] The guest has no network after fetching job inputs, matching hermetic policy.
- [ ] Failure of the guest runner fails the parent job and captures serial logs.
- [ ] Production merge-queue jobs still have a Linux-host runner path; this guest is an additional self-host proof.

#### Verification
- Integration: smoke job on the QEMU JakeOS runner; parent job fails when the guest panics.
- Review: BLD CI owner confirms this is not the sole merge-queue capacity at V2.

#### Evidence
- none

### BLD-058 · Run CI on the three V2 target machines
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: BLD-044, LAB-018
- Baseline: §62

V2 hardware scope is exactly three named machines: H-002, H-004 and H-005 (§62). This extends lab CI so nightly and qualification jobs cover the AMD desktop, Intel laptop and AMD laptop. NVIDIA remains experimental and non-gating.

Required by V2-G01 (Desktop shell passes the UX script on all three machines) and every V2 gate scoped to H-002, H-004 and H-005.

#### Out of scope
Racking (LAB-018). NVIDIA bring-up (HW, GFX). Six-machine nightly (BLD-069).

#### Acceptance criteria
- [ ] Nightly CI runs the hardware regression and desktop smoke jobs on H-002, H-004 and H-005.
- [ ] A failure on any of the three fails nightly.
- [ ] H-006 may appear as an experimental job and is not required to pass.
- [ ] Job inventory names the three machines by H-ID.

#### Verification
- Integration: nightly aggregator requires H-002, H-004 and H-005; an H-005 failure fails the aggregator.
- Review: LAB lead confirms the three IDs match LAB-018.

#### Evidence
- none

### BLD-059 · Run userspace sanitizer profiles for native Components in nightly CI
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: BLD-026, BLD-049
- Baseline: §51

V0 sanitizer profiles boot the kernel. V2 desktop code needs ASAN, LSAN and TSAN (or equivalent) on native Components in nightly CI without exploding the pre-merge budget. Pre-merge remains the V0 debug kernel profile plus unit tests.

#### Out of scope
Kernel sanitizer boots (BLD-026). Personality sanitizers (LNX, WIN). Pre-merge lint (BLD-011).

#### Acceptance criteria
- [ ] Nightly CI builds native Components with ASAN and LSAN and runs the V0.5 four-app plus V2 shell smokes on H-001 or H-003.
- [ ] A TSAN (or equivalent) job runs a nominated subset of Channel and Task tests nightly.
- [ ] These jobs are nightly, not required pre-merge checks.
- [ ] A sanitizer trip fails the nightly job and files a crasher or bug task.

#### Verification
- Integration: ASAN/LSAN four-app smoke; TSAN subset; a planted use-after-free fails ASAN.
- Review: SDK lead confirms flags apply to native Components, not personality binaries.

#### Evidence
- none

### BLD-060 · Provision GPU-equipped nightly capacity for the W1 Corpus
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: BLD-058, WIN-009, BLD-017
- Baseline: §48, §56.2, §62
- Corpora: C-007

V2 W1 gates need GPU lab jobs. WIN owns corpus and Wine suite; BLD sizes nightly GPU runners so those jobs are not starved by kernel QEMU. Capacity is scheduled against H-002 and other GPU lab machines LAB hosts.

#### Out of scope
W1 scenarios and ratings (WIN-051, WIN-009). GPU CTS nightly (LAB-019). Quiet perf fleet (BLD-048).

#### Acceptance criteria
- [ ] Nightly GPU job slots exist for C-007 that cannot be consumed by kernel QEMU queues.
- [ ] WIN-051 jobs receive those slots on H-002 at minimum.
- [ ] Starvation is visible: a skipped W1 job for capacity reasons fails the nightly aggregator rather than reporting pass.

#### Verification
- Compat: C-007 plumbing on H-002 using BLD-017.
- Integration: a synthetic kernel-QEMU flood leaves a W1 slot available or fails nightly for skip.

#### Evidence
- none

### BLD-061 · Produce nightly images for the alpha Channel
- Type: build
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: BLD-024, REL-005
- Baseline: §30, §63

V3 exit: the updater delivers consecutive alpha releases (§63). BLD produces the nightly SystemGeneration images REL promotes without rebuilding.

Required by V3-G03 (Updater, automatic rollback and recovery): the updater delivers consecutive alpha releases built here.

#### Out of scope
Channel operation and signing (REL-005, REL-019). Updater client (INS). Promote-without-rebuild mechanics (BLD-065).

#### Acceptance criteria
- [ ] Nightly CI produces a SystemGeneration image from INS-001 suitable for the alpha channel.
- [ ] The image identifier is a content hash; REL promotion refers to that hash without a rebuild.
- [ ] A failed nightly image build does not publish a partial generation.

#### Verification
- Integration: nightly image hash is the object REL-005 would promote; a broken build publishes nothing.
- Review: REL lead confirms no rebuild occurs at promote time.

#### Evidence
- none

### BLD-062 · Build an embargoed private path for security-release compilation
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: BLD-034, BLD-065
- Baseline: §51, §63
- Threats: T-007

Public CI leaks embargoed fixes before coordinated disclosure. A private compile, sign-staging and hold path is BLD infrastructure REL uses for advisories. Public dashboards must not list these jobs.

<!-- covers: GAP-0356 -->

#### Out of scope
Advisory text and disclosure process (REL). Signing keys (REL-041). Public dashboards (BLD-066).

#### Acceptance criteria
- [ ] An access-controlled pipeline compiles a SystemGeneration from a private branch with no logs on public dashboards.
- [ ] Artifacts are staged for REL signing without being published to alpha or stable channels.
- [ ] The public double-build and SBOM jobs do not receive embargoed sources.
- [ ] After public disclosure, the same artifact hash can be promoted without rebuild.

#### Verification
- Integration: fixture private branch builds and stages an artifact that is absent from public dashboards and from the public store until promote.
- Review: REL security-response owner accepts the path.

#### Evidence
- none

### BLD-063 · Gate V3 on no known open Native ABI fuzzer crasher
- Type: build
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: BLD-035, BLD-042, NET-023
- Baseline: §51, §63

V3 exit: kernel and IPC fuzzing run continuously in CI with no known open crasher older than the window named by the V3 gate. This task turns BLD-035 inventory into a release-qualification failure. Workstreams own closing their crashers.

#### Out of scope
Fuzz fleets (BLD-035, BLD-042). V4 clean window (BLD-073). Per-surface targets (IPC, CAP, CMP, GFX, NET).

#### Acceptance criteria
- [ ] Release-qualification queries the fuzz inventory and fails when an open Native ABI or IDL deserialiser crasher is older than the V3 gate window.
- [ ] Userspace parser crashers are listed; those older than the window fail the same job unless an accepted Decision defers a named harness.
- [ ] The job prints crasher IDs and owning prefixes; it does not close crashers itself.

#### Verification
- Integration: planted aged crasher fails qualification; a fresh crasher within the window does not.
- Review: IPC and CAP leads confirm inventory coverage includes their registered harnesses.

#### Evidence
- none

### BLD-064 · Migrate CI runners to bare-metal JakeOS
- Type: build
- Milestone: V3
- Status: todo
- Size: L
- Owner: none
- Depends on: BLD-057, INS-004, LAB-021, BLD-034
- Baseline: §61, §63
- Risks: R-070

Full CI-on-JakeOS after installer and updater exist dogfoods unattended operation and remote management that desktop daily-driving never hits. QEMU-hosted runners remain for nested cases; production merge-queue capacity moves to bare-metal JakeOS hosts.

<!-- covers: GAP-0153 -->

#### Out of scope
QEMU self-host step (BLD-057). Installer and updater (INS). Lab racking (LAB).

#### Acceptance criteria
- [ ] Bare-metal JakeOS machines registered as CI runners execute merge-queue smoke and post-merge QEMU-hosting jobs.
- [ ] Unattended reboot and remote management recover a hung runner without desktop session login.
- [ ] Linux-host runners remain only where the job is building JakeOS from Linux bootstrap, and are named as such.
- [ ] Embargoed path can target a JakeOS runner without public logs.

#### Verification
- Integration: merge-queue smoke on a bare-metal JakeOS runner; induced hang recovers via remote management.
- Review: LAB and BLD owners confirm host identity versus functional Tier 1 test machines.

#### Evidence
- none

### BLD-065 · Build release artifacts once and promote them across channels
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: BLD-041, BLD-024
- Baseline: §27, §30, §63
- Invariants: I-036

Rebuilding per channel voids the testing that justified promotion. BLD produces each SystemGeneration, installer image and SDK artifact once; REL signs and publishes the same bytes through nightly, testing and stable.

<!-- covers: GAP-0147 -->

#### Out of scope
Signing and channel policy (REL-019, REL-005). Nightly image production (BLD-061).

#### Acceptance criteria
- [ ] A generation promoted from nightly to testing has the same content hashes as the nightly artifact.
- [ ] CI refuses a promote job that would rebuild kernel, Packages or installer images.
- [ ] SBOM and provenance artifacts share those hashes.
- [ ] Qualification records cite the hash, not a channel-specific rebuild.

#### Verification
- Integration: promote fixture compares hashes across two channel names; a rebuild attempt fails the job.
- Review: REL lead confirms REL-019 consumes these hashes.

#### Evidence
- none

### BLD-066 · Publish public CI and benchmark dashboards
- Type: build
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: BLD-052, BEN-031
- Baseline: §54, §63
- Invariants: I-061, I-088

V3 benchmark gates require public dashboards. BEN owns numbers; BLD publishes pass, fail, flake and links to BEN exports without leaking embargoed jobs.

#### Out of scope
Benchmark TSDB and public metric plots (BEN-031). Embargoed path (BLD-062). Internal functional dashboard (BLD-052).

#### Acceptance criteria
- [ ] A public dashboard shows functional pass, fail and flake for non-embargoed jobs per commit and H-ID.
- [ ] Benchmark panels are BEN-031 exports or links; BLD does not copy B-ID values into its own charts (I-088).
- [ ] Embargoed jobs and private branches are absent from the public view.
- [ ] The public view is updated on each V3 qualification run.

#### Verification
- Integration: public fetch sees a green smoke job and does not see an embargoed fixture job.
- Review: BEN and REL leads confirm metric plots remain BEN-owned and embargoed jobs are excluded.

#### Evidence
- none

### BLD-067 · Implement the release-qualification CI tier
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: BLD-001, BLD-069, BLD-041, BLD-033, BLD-063
- Baseline: §63
- Invariants: I-088

GAP-0109 named release qualification at V0, but public alpha is the first rung that ships channels. The mechanical checklist: gates green, reproducibility, Tier 1 pass, no unresolved BEN regressions, no aged open crasher. REL consumes the result.

<!-- covers: GAP-0109 -->

#### Out of scope
Channel launch (REL). Soak calendar (LAB, BLD-076). SBOM completeness failure (BLD-068).

#### Acceptance criteria
- [ ] A qualification job fails unless merge-queue required checks, double-build, BLD-069, BLD-033 and BLD-063 are green for the candidate hash.
- [ ] Unresolved BEN regressions beyond the register band fail qualification unless an accepted Decision names the B-ID.
- [ ] The qualification record cites B-IDs and C-IDs and does not restate numbers (I-088).
- [ ] REL-019 can consume a passed record without rebuilding.

#### Verification
- Integration: fixture candidate missing a Tier 1 machine or carrying an aged crasher fails qualification.
- Review: REL lead confirms the checklist is the input to promotion.

#### Evidence
- none

### BLD-068 · Gate releases on complete SBOMs for every SystemGeneration
- Type: build
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: BLD-055, BLD-067
- Baseline: §27, §63
- Threats: T-007

1.0 security posture and REL provenance publication need every shipped generation to have an SBOM. This turns BLD-055 into a release-qualification failure when any Package or SystemGeneration lacks one.

#### Out of scope
SBOM generation (BLD-055). Signed publication (REL-051). Format Decision (BLD-054).

#### Acceptance criteria
- [ ] Qualification fails if any Package in the candidate SystemGeneration lacks an SBOM artifact keyed by content hash.
- [ ] Qualification fails if the SystemGeneration itself lacks an SBOM.
- [ ] A generation that passed BLD-055 on the default branch still fails here if the promoted hash has no SBOM attached.

#### Verification
- Integration: candidate with a missing Package SBOM fails qualification; a complete candidate passes this check.
- Review: REL lead confirms the gate matches REL-051 inputs.

#### Evidence
- none

### BLD-069 · Run nightly CI on all six Tier 1 machines
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: BLD-058, LAB-021
- Baseline: §63
- Risks: R-037

V3 hardware scope: six Tier 1 machines fully tested each release, including NVIDIA (H-002, H-004, H-005, H-006, H-007, H-008). BLD schedules the nightly matrix LAB hosts.

Required by V3-G01 (Installer completes on Tier 1 with full-disk encryption): automated runs on every Tier 1 machine need this nightly matrix.

#### Out of scope
Racking (LAB-021). NVIDIA driver stance (HW, GFX). Ten-machine nightly (BLD-078).

#### Acceptance criteria
- [ ] Nightly CI requires passing hardware jobs on H-002, H-004, H-005, H-006, H-007 and H-008.
- [ ] A skip on H-006 fails nightly; NVIDIA is gating at V3.
- [ ] Per-machine results are visible on BLD-052 by H-ID.

#### Verification
- Integration: aggregator requires the six H-IDs; an H-006 failure fails nightly.
- Review: LAB lead confirms IDs match LAB-021.

#### Evidence
- none

### BLD-070 · Emit SLSA-style build provenance from hermetic CI
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: BLD-055, BLD-009, BLD-065
- Baseline: §27, §51
- Threats: T-007

REL publishes SBOM plus signed provenance; BLD emits provenance from the hermetic, no-network-after-fetch builder so attestations match actual CI. Provenance names input hashes, builder identity and the output SystemGeneration hash.

#### Out of scope
Signing and publication (REL-051). Embargoed path (BLD-062). Format of SBOM (BLD-054).

#### Acceptance criteria
- [ ] Every qualified SystemGeneration has a provenance document listing content-hashed inputs, builder ID and output hash.
- [ ] Provenance generation runs in the hermetic builder with no network after fetch.
- [ ] A mismatch between provenance output hash and the promoted artifact fails qualification.
- [ ] REL-051 can sign the document without regenerating it.

#### Verification
- Integration: provenance for a qualified generation matches store hashes; tampered output hash fails qualification.
- Review: REL lead confirms the document is the attestation payload.

#### Evidence
- none

### BLD-071 · Lock CI against new Layer 1 entry points after feature freeze
- Type: build
- Milestone: V4
- Status: todo
- Size: S
- Owner: none
- Depends on: ABI-049, ABI-027, BLD-067
- Baseline: §65, §66
- Invariants: I-040

V4 feature freeze after RC1 allows no new Layer 1 ABI entry points. ABI owns the snapshot and freeze Decision; BLD fails qualification if the snapshot grows without that Decision.

#### Out of scope
Freeze Decision and snapshot content (ABI-049, ABI-027). Feature-freeze governance text (GOV).

#### Acceptance criteria
- [ ] After the freeze Decision is accepted, qualification fails if the Layer 1 snapshot gains an entry point.
- [ ] A snapshot change that only documents a frozen entry does not fail this lock.
- [ ] The job cites ABI-027 as the snapshot source.

#### Verification
- Integration: planted new L1 entry after freeze fails qualification; a docs-only snapshot comment does not.
- Review: ABI lead confirms the lock uses the ABI snapshot.

#### Evidence
- none

### BLD-072 · Eliminate flaky tests or expire their quarantine
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: BLD-021, BLD-052
- Baseline: §63

V4 stability includes flaky-test elimination. Quarantine from BLD-021 must be empty or expired at RC; expired entries fail until deleted or fixed. Owning workstreams deflake; BLD enforces emptiness at qualification.

#### Out of scope
Quarantine policy (BLD-021). Observability of flakes (OBS, BLD-052).

#### Acceptance criteria
- [ ] Release-candidate qualification fails if any non-expired quarantine entry remains.
- [ ] Expired quarantine entries already fail CI per BLD-021.
- [ ] The dashboard lists remaining flakes with owner prefixes at RC start; BLD does not reassign owners.

#### Verification
- Integration: RC candidate with a live quarantine entry fails; a candidate with empty quarantine passes this check.
- Review: GOV process reviewer confirms expiry rules were not replaced by a calendar date.

#### Evidence
- none

### BLD-073 · Gate V4 on a clean Native ABI fuzz window
- Type: build
- Milestone: V4
- Status: todo
- Size: S
- Owner: none
- Depends on: BLD-063, BLD-035
- Baseline: §51, §65

V4 exit: kernel and IPC fuzzing have zero known open crashers for the consecutive window named by the V4 gate before freeze. This tightens BLD-063 from aged-crasher to a clean window.

#### Out of scope
Fuzz fleet (BLD-035). V3 aged-crasher gate (BLD-063). Closing crashers (owning prefixes).

#### Acceptance criteria
- [ ] Qualification fails unless the Native ABI and IDL deserialiser inventory shows zero open crashers throughout the V4 gate window.
- [ ] A crasher opened and closed inside the window still fails if it was open at qualification time.
- [ ] The job prints the window definition from the V4 gate and the inventory query, with no restated duration in other documents.

#### Verification
- Integration: inventory with an open crasher fails; a clean inventory for the window passes.
- Review: ABI and IPC leads confirm the inventory is the freeze-time fuzz evidence.

#### Evidence
- none

### BLD-074 · Verify reproducible builds with independent rebuilders
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: BLD-077, BLD-047
- Baseline: §27
- Risks: R-060, R-065
- Invariants: I-036

V4 exit: SystemGeneration Packages reproduce on two independent builders and a public verifier exists. This task stands up the second builder and publishes per-generation status. The public tool is BLD-075. 1.0 third-party proof is BLD-080.

<!-- covers: GAP-0377 -->

#### Out of scope
Making every Package reproduce (BLD-077). Public verifier tool (BLD-075). Third-party 1.0 proof (BLD-080).

#### Acceptance criteria
- [ ] A second builder, not the primary CI cluster, rebuilds the candidate SystemGeneration from mirrored inputs and matches hashes.
- [ ] Per-generation status (match or mismatch per Package) is published beside the generation hash.
- [ ] A mismatch fails V4 qualification.
- [ ] The second builder uses BLD-047, not an ambient distro toolchain.

#### Verification
- Integration: second-builder rebuild of a V4 candidate; planted mismatch fails qualification.
- Review: REL lead confirms published status is the reproducibility evidence REL cites.

#### Evidence
- none

### BLD-075 · Ship a public verifier for independent rebuilds
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: BLD-074
- Baseline: §27
- Threats: T-007

V4 exit: a public verifier tool is available so reproducibility is not only the project's own pipeline. The tool consumes mirrored inputs, the SBOM and provenance, and reports hash match or mismatch.

#### Out of scope
Second builder (BLD-074). 1.0 third-party run (BLD-080). Signing (REL).

#### Acceptance criteria
- [ ] A documented verifier command, runnable outside project CI, rebuilds a nominated Package set from the public mirror and compares hashes.
- [ ] The verifier consumes SBOM and provenance artifacts and fails closed on a hash mismatch.
- [ ] The tool is itself a content-addressed Package or a signed binary with a published hash.
- [ ] Embargoed generations are not required to be verifiable until they are public.

#### Verification
- Integration: verifier run against a public V4 candidate matches BLD-074 status.
- Manual: follow the committed verifier page on a Linux host that is not a CI runner.

#### Evidence
- none

### BLD-076 · Run release-candidate soak matrices on the Tier 1 fleet
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: BLD-078, LAB-024, BLD-067
- Baseline: §63
- Risks: R-063

V4 exit: RC cycles each soak on the full Tier 1 fleet with no P0. BLD owns the matrix jobs; LAB owns machine uptime and the soak calendar. Firmware refreshes during soak are R-063.

#### Out of scope
Soak execution calendar (LAB-024). Ten-machine nightly (BLD-078). 1.0 soak (BLD-079).

#### Acceptance criteria
- [ ] Each V4 RC hash runs the soak matrix on every Tier 1 H-ID required by BLD-078.
- [ ] An open P0 on any machine fails the RC; P0 is defined by GOV triage policy, not by BLD.
- [ ] Soak results appear per H-ID on the qualification record.
- [ ] A firmware change mid-soak is recorded and reruns the affected machine's matrix.

#### Verification
- Integration: RC soak aggregator requires all ten-machine IDs; a planted P0 fails the RC.
- Review: LAB lead confirms BLD submits jobs to LAB-024.

#### Evidence
- none

### BLD-077 · Make every SystemGeneration Package reproduce bit-for-bit
- Type: build
- Milestone: V4
- Status: todo
- Size: L
- Owner: none
- Depends on: BLD-041, BLD-029, BLD-055
- Baseline: §27, §50
- Risks: R-060, R-065
- Invariants: I-036

V4 reproducible-builds gate covers the inherited Linux toolchain plus native Packages (§27). Double-build on primary CI is not enough: every Package in the generation must rebuild identically, including inherited toolchain outputs. 1.0 two-rebuilder-release reuses this as the remaining independent-party proof.

<!-- covers: GAP-0093 -->

#### Out of scope
Second builder (BLD-074). Public verifier (BLD-075). SDK binding packages (SDK-091).

#### Acceptance criteria
- [ ] Every Package in the V4 candidate SystemGeneration rebuilds to the same content hash on primary CI.
- [ ] Inherited Linux toolchain outputs in the generation are included in that set.
- [ ] A non-reproducing Package fails qualification and is named in the report.
- [ ] SBOM hashes match the reproducing set.

#### Verification
- Integration: full-generation rebuild on primary CI; planted non-reproducing Package fails.
- Review: PKG lead confirms Package identity is store hash, not a parallel name.

#### Evidence
- none

### BLD-078 · Run nightly CI on at least ten Tier 1 machines
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: BLD-069, LAB-023
- Baseline: §62, §63
- Risks: R-056

V4 hardware: at least ten named Tier 1 machines fully tested each RC (H-002 through H-014 as racked). Extends BLD-069. LAB owns racking; BLD owns nightly and RC job submission.

#### Out of scope
Racking (LAB-023). HCL publication (HW, REL). Soak matrix (BLD-076).

#### Acceptance criteria
- [ ] Nightly CI requires hardware jobs on at least ten named Tier 1 H-IDs including the V3 six plus the V4 additions LAB racked.
- [ ] A missing machine fails nightly rather than silently dropping below ten.
- [ ] Per-machine results feed BLD-076.

#### Verification
- Integration: aggregator counts H-IDs and fails below ten; a new H-014 failure fails nightly.
- Review: LAB lead confirms the named set matches LAB-023.

#### Evidence
- none

### BLD-079 · Run the 1.0 soak matrix on the Tier 1 fleet
- Type: build
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: BLD-076, LAB-025
- Baseline: §63
- Risks: R-063

1.0 exit: the final RC is soaked on the full Tier 1 fleet with zero open P0 and P1. BLD keeps the qualification matrix green for the soak window; REL owns channel launch. LAB executes the fleet soak.

#### Out of scope
Channel launch (REL). Fleet soak calendar (LAB-025). Two-rebuilder proof (BLD-080).

#### Acceptance criteria
- [ ] The 1.0 candidate hash remains green on BLD-067 for the soak window named by the 1.0 gate.
- [ ] Open P0 or P1 on any Tier 1 H-ID fails the soak aggregator.
- [ ] Soak results per H-ID are attached to the 1.0 qualification record.

#### Verification
- Integration: soak aggregator on the 1.0 candidate; planted P1 fails.
- Review: REL lead confirms this record is the soak evidence for channel launch.

#### Evidence
- none

### BLD-080 · Reproduce the 1.0 image with two independent rebuilders
- Type: build
- Milestone: 1.0
- Status: todo
- Size: M
- Owner: none
- Depends on: BLD-074, BLD-075, BLD-077
- Baseline: §27
- Risks: R-060, R-065
- Invariants: I-036

1.0 exit: every 1.0 SystemGeneration Package reproduces bit-for-bit on two independent builders, and a third party has reproduced the release image with published matching checksums (§27). This is the independent-party proof on top of V4 rebuilders.

<!-- covers: GAP-0093 -->

#### Out of scope
V4 second builder (BLD-074). Verifier tool (BLD-075). Signing (REL).

#### Acceptance criteria
- [ ] Two independent builders reproduce every Package of the 1.0 image to matching content hashes.
- [ ] A third-party run of BLD-075 publishes matching checksums for the 1.0 image.
- [ ] A mismatch fails 1.0 qualification.
- [ ] Published checksums cite generation hashes, not channel names.

#### Verification
- Integration: two-builder rebuild of the 1.0 candidate; verifier output matches.
- Review: REL and GOV leads confirm the third-party publication is the 1.0 reproducibility evidence.

#### Evidence
- none
