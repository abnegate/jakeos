# KRN · Kernel fork and upstream tracking
- Prefix: KRN
- Lead: none
- Baseline: §2, §5, §6, §55, §56.4

<!-- roadmap:generated:begin summary -->
Tasks: 60 live, 8 done, 0 in-progress, 52 todo, 0 dropped. Ready: 3. Blocked: 49. Weighted: 8%.
<!-- roadmap:generated:end -->

## Scope

The Linux-derived kernel fork and everything required to keep that fork honest: the strategy Decision, the upstream tree and tracking cadence, divergence phases A through E, the retained-mechanism inventory, the native in-tree subsystem layout, config fragments, hardening, identity, CVE applicability, driver-adaptation tooling, KVM as a capability-mediated object, watchdogs, module and firmware signing, and the 1.x maintenance branch. KRN owns kernel-resident policy for Rust-first new code, rewrite-versus-retain, eBPF's native role, and live-patching as a non-goal. Hardware support is preserved by inventory plus a merge-blocking regression matrix, not by one task per inherited subsystem.

## Out of scope

Boot and firmware (BOOT). Native ABI, handles and Object registry (ABI). Capability rights and tables (CAP). Component creation (CMP). ResourceDomain accounting (SCH). Channels and IDL (IPC). Tracing substrate and crash-report format (OBS). Service supervisor and native init (SVC). Linux personality syscall retain and translation (LNX). VM manager and guest tools (VIRT). NVIDIA driver stance (HW, GFX). SBOM, QEMU harness, kselftest runners and Rust-in-kernel toolchain pinning mechanics (BLD). Advisory publication and CVE SLA register (REL). Userspace license firewall (GOV).

## Tasks

### KRN-001 · Decide kernel-core vs user-space service boundary and the criteria for moving one
- Type: adr
- Milestone: V0
- Status: done
- Size: S
- Owner: @agent/claude
- Depends on: KRN-002
- Baseline: §4, §33, §57
- Decision: D-0157
- Invariants: I-008
- Verified by: @jakebarnby

§4 places Memory, IPC, Capabilities and the scheduler in the kernel core and UI, storage and network in user space. This Decision records that split for V0 and the measured-cost criteria for moving a service across the boundary, so residency is decided by cost rather than microkernel ideology (§1, §57).

<!-- covers: INV-0117, INV-0111, INV-0013, INV-1118, INV-0061 -->

#### Out of scope
ResourceDomain substrate (SCH-003). User-space driver hosting (SVC). Native ABI object residency (ABI-013).

#### Acceptance criteria
- [x] Options evaluated include the §4 split as written, moving storage or network into the kernel core, and moving IPC or the scheduler into user space, each with measured-cost criteria for a later move.
- [x] The accepted option lists kernel-core objects for V0 and the metric that must be published before a later Decision may move one.
- [x] A Review line names who accepts the Decision.

#### Verification
- Review: kernel architecture lead sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- decision:D-0157

### KRN-002 · Decide kernel strategy: Linux fork vs new microkernel vs Linux-as-hypervisor
- Type: adr
- Milestone: V0
- Status: done
- Size: S
- Owner: @agent/claude
- Depends on: none
- Baseline: §1, §5, §57
- Decision: D-0158
- Invariants: I-008, I-010
- Verified by: @jakebarnby

V0 requires an accepted Decision for the kernel strategy. This records the hardware-maturity versus native-model rationale of §1 and §5: inherit Linux's mature hardware foundation without inheriting Linux as the native programming model. Microkernel-for-purity and Linux-as-hypervisor are named options so those non-goals are enforced by a Decision rather than by prose.

<!-- covers: INV-0002, INV-0013, INV-1118 -->

#### Out of scope
Upstream tree and LTS series (KRN-005). Service-boundary criteria (KRN-001). Personality translation phase (LNX-003).

#### Acceptance criteria
- [x] Options evaluated include a radical fork of Linux, a new microkernel with Linux drivers in a virtual machine, and Linux-as-hypervisor with the native model in a guest.
- [x] The accepted option states what hardware foundation is inherited and that native software does not see POSIX, Linux syscalls or Win32.
- [x] A Review line names who accepts the Decision.

#### Verification
- Review: kernel architecture lead sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- decision:D-0158

### KRN-003 · Decide the licence for new native kernel code
- Type: adr
- Milestone: V0
- Status: done
- Size: S
- Owner: @agent/claude
- Depends on: KRN-002
- Baseline: §5.1, §50
- Decision: D-0162
- Invariants: I-067
- Verified by: @jakebarnby

New code linked into a GPLv2 kernel must be GPLv2-compatible, and the choice decides whether kernel-side abstractions can ever be shared with permissive user space. This Decision must precede the first native commit. Outbound userspace licenses remain GOV-003; the two Decisions coordinate and neither depends on the other.

<!-- covers: GAP-0001 -->

#### Out of scope
Userspace license firewall and Layer 2 through 4 outbound terms (GOV-003). MODULE_LICENSE for inherited modules (KRN-028). IDL generated-code exception (IPC-005).

#### Acceptance criteria
- [x] Options evaluated include GPLv2-only, GPLv2-or-later, and dual GPLv2/MIT for reusable abstractions.
- [x] The accepted option states whether a kernel-side abstraction may be copied into a permissive userspace crate.
- [x] Apache-2.0-only and CDDL kernel dependencies remain forbidden (I-067).
- [x] A Review line names who accepts the Decision.

#### Verification
- Review: GOV licensing reviewer and kernel architecture lead sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- decision:D-0162

### KRN-004 · Decide kernel Rust toolchain pinning relative to the Rust-for-Linux minimum
- Type: adr
- Milestone: V0
- Status: done
- Size: S
- Owner: @agent/claude
- Depends on: KRN-002, KRN-005, Q-051
- Baseline: §50
- Decision: D-0165
- Risks: R-002
- Verified by: @jakebarnby

The fork repository and CI need a Rust toolchain on day one. This Decision answers Q-051 early: whether the fork tracks upstream Linux's minimum Rust version, pins independently, or pins with a bounded lag, and what happens when upstream stabilizes features the fork's Rust code depends on.

<!-- covers: GAP-0052, GAP-0162 -->

#### Out of scope
Kbuild integration and bindgen (BLD-013). Userspace rustc pin (BLD-046). Rewrite-versus-retain criteria (KRN-015).

#### Acceptance criteria
- [x] Options evaluated include tracking upstream's minimum, pinning independently, and pinning with a bounded lag behind upstream.
- [x] The accepted option states the action when upstream raises its minimum or stabilizes a feature the fork already uses.
- [x] Q-051 is marked answered by this task.
- [x] A Review line names who accepts the Decision.

#### Verification
- Review: kernel architecture lead and BLD toolchain owner sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- decision:D-0165

### KRN-005 · Decide the upstream Linux tree and LTS series the fork is cut from
- Type: adr
- Milestone: V0
- Status: done
- Size: S
- Owner: @agent/claude
- Depends on: KRN-002
- Baseline: §5.1, §6
- Decision: D-0166
- Verified by: @jakebarnby

Nothing in KRN can start before the base is chosen. This Decision selects mainline versus LTS and the specific series the fork is cut from, with a review hook fed by KRN-021. Tracking cadence is a separate Decision.

<!-- covers: INV-0118, GAP-0534, GAP-0050, GAP-0348 -->

#### Out of scope
Rebase versus merge and per-phase cadence (KRN-007). Phase D entry (KRN-042). 1.x LTS window (KRN-055).

#### Acceptance criteria
- [x] Options evaluated include a named current mainline tag, a named LTS series, and a named stable branch that is not LTS.
- [x] The accepted option names the tree, the series, and the condition that reopens this Decision after KRN-021.
- [x] A Review line names who accepts the Decision.

#### Verification
- Review: kernel architecture lead sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- decision:D-0166

### KRN-006 · Decide the upstream-first policy for the hardware layer and Rust abstractions
- Type: adr
- Milestone: V0
- Status: done
- Size: S
- Owner: @agent/claude
- Depends on: KRN-002
- Baseline: §5.1, §6, §55
- Decision: D-0167
- Verified by: @jakebarnby

Driver and subsystem fixes, and reusable Rust-for-Linux abstractions, either go to upstream Linux before or alongside the fork, or they accumulate as fork-only patches. This Decision bounds the divergence surface from the first patch and is the policy the divergence ledger classifies against.

<!-- covers: GAP-0048, GAP-0052 -->

#### Out of scope
Per-patch classification tooling (KRN-008). Toolchain pin (KRN-004). GPU tracking cadence (KRN-033).

#### Acceptance criteria
- [x] Options evaluated include upstream-first (fixes land upstream before or with the fork), alongside (same-week dual posting), and fork-only with later contribution.
- [x] The accepted option applies to both C hardware-layer patches and reusable Rust abstractions.
- [x] A Review line names who accepts the Decision.

#### Verification
- Review: kernel architecture lead sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- decision:D-0167

### KRN-007 · Decide upstream tracking: rebase vs merge and cadence per divergence phase
- Type: adr
- Milestone: V0
- Status: done
- Size: S
- Owner: @agent/claude
- Depends on: KRN-005
- Baseline: §6, §56.4
- Decision: D-0168
- Risks: R-029
- Verified by: @jakebarnby

Separate from the base-version Decision: this fixes periodic rebase versus merge, the cadence for phases A through C, and how stable-branch backports are automated. The merge bot, delta report and V1 rebase gate implement this policy.

<!-- covers: GAP-0348, GAP-0050, INV-1100, GAP-0534 -->

#### Out of scope
Which tree is cut (KRN-005). When full merges stop (KRN-042). Divergence policy document (KRN-009).

#### Acceptance criteria
- [x] Options evaluated include periodic rebase onto the chosen series, merge of each upstream tag, and rebase for LTS plus merge for mainline.
- [x] The accepted option states cadence for phases A through C and how stable-branch backports are automated.
- [x] A Review line names who accepts the Decision.

#### Verification
- Review: kernel architecture lead sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- decision:D-0168

### KRN-008 · Build the divergence ledger classifying every fork patch with a CI Gate
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: KRN-010, KRN-009, KRN-006
- Baseline: §6, §56.4
- Risks: R-069

Controlled divergence (§6) is only controlled if every fork commit is classified. Each commit on the kernel `main` that is not an upstream commit carries a trailer `Jakeos-Divergence: upstream-candidate | fork-only | temporary` and, for `upstream-candidate`, a `Jakeos-Upstream-Series: <lore link or "pending">` trailer; `temporary` commits carry `Jakeos-Remove-When: <condition>` (a condition, never a date). The ledger tool `tools/jakeos/divergence/` in the kernel tree (a Rust binary, `jakeos-divergence`) walks `git log upstream/master..main`, reads the trailers, and generates `Documentation/jakeos/divergence-ledger.md` and `divergence-ledger.json` grouping series by classification with their upstream status.

The `pre-merge` job `divergence` runs `jakeos-divergence check <base>..HEAD` and fails when any non-upstream commit in the range lacks a valid classification trailer; the `post-merge` job regenerates the ledger and commits it through the queue. The classification vocabulary is the one KRN-009's divergence policy defines; SBOM coverage of the same commits is BLD's (BLD-055, BLD-068).

<!-- covers: GAP-0049, GAP-0055 -->

#### Out of scope
SBOM generation and the SBOM CI gate (BLD-055, BLD-068). Upstream trial-merge bot (KRN-023). The policy text itself (KRN-009).

#### Deliverables
- kernel:tools/jakeos/divergence/ · Crate `jakeos-divergence`: `check` (fails unclassified commits in a range) and `render` (writes the ledger).
- kernel:Documentation/jakeos/divergence-ledger.md · Generated ledger: series by classification, upstream link or `pending`, removal condition for temporary patches.
- kernel:Documentation/jakeos/divergence-ledger.json · Machine-readable form of the ledger for the roadmap dashboard and KRN-023.
- kernel:Documentation/jakeos/commit-trailers.md · The trailer grammar: `Jakeos-Divergence`, `Jakeos-Upstream-Series`, `Jakeos-Remove-When`.
- kernel:.github/workflows/pre-merge.yml · The `divergence` job running `jakeos-divergence check`.
- kernel:.github/workflows/post-merge.yml · The ledger regeneration step.
- kernel:tools/jakeos/divergence/tests/ledger_*.rs · Tests: unclassified commit fails, each classification renders, temporary without a removal condition fails.

#### Acceptance criteria
- [ ] Every non-upstream commit on `main` carries a `Jakeos-Divergence` trailer with one of `upstream-candidate`, `fork-only`, `temporary`, and `Documentation/jakeos/divergence-ledger.md` lists every series under its classification with its upstream link or `pending`.
- [ ] The `pre-merge` `divergence` job on `qemu-x86_64` fails when any commit in `<base>..HEAD` that is not an upstream commit lacks a valid trailer, or when a `temporary` commit lacks `Jakeos-Remove-When`.
- [ ] The ledger records rebase status per classified series: `clean`, `conflicted` or `dropped` against the latest merged upstream tag.

#### Verification
- Unit: `kernel:tests/divergence/ledger_*` on `qemu-x86_64`.
- Integration: a deliberately unclassified commit is rejected by the `pre-merge` check.
- Review: kernel architecture lead confirms the three classification labels match the KRN-009 policy.

#### Evidence
- none

### KRN-009 · Write the kernel divergence policy defining phases A-E and their gates
- Type: docs
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: KRN-002, KRN-007, KRN-006
- Baseline: §6, §56.4, §57
- Invariants: I-053

§6 describes the fork's evolution as phases: A (native subsystem behind a config symbol, upstream merged every tag), B (native primitives take over paths the Linux personality still uses through the retained syscall table), C (wrappers over Linux internals replaced by native implementations where measured), D (selective adaptation of upstream instead of whole-tag merges, entered only on measured triggers), E (the native model is the kernel's primary model). This document, `Documentation/jakeos/divergence-policy.md`, defines each phase's upstream-tracking obligation (which follows D-0168: merge every tag through phase C), the divergence classifications KRN-008 enforces, the review rule for a `fork-only` patch that touches a shared file, and the measurable triggers for entering phase D: the merge-conflict rate per tag from the KRN-008 ledger, the count of `fork-only` series touching shared files, and the retained-mechanism inventory review outcome, each as a threshold recorded in the document and evaluated by KRN-042 against live bot data.

It states I-053 plainly: upstream mergeability never outranks the architecture (§57), and a native-model change is not softened to ease a merge. It also states what the personality keeps at each phase (the Linux syscall table stays through phase C at least) so LNX-003 and KRN-050 decide against a written baseline.

<!-- covers: INV-0156, INV-1100, GAP-0050, INV-1127 -->

#### Out of scope
Applying the phase D triggers to live bot data (KRN-042). 1.0 phase requirement (KRN-050). Trailer grammar and ledger tool (KRN-008).

#### Deliverables
- kernel:Documentation/jakeos/divergence-policy.md · Phases A to E, tracking obligation per phase, classification vocabulary, shared-file review rule, phase D entry triggers with their thresholds, and I-053.
- kernel:README.md · A section pointing at the policy and the current phase.

#### Acceptance criteria
- [ ] `Documentation/jakeos/divergence-policy.md` names phases A through E, the upstream-tracking obligation of each, and the measurable phase D entry triggers (conflict rate per tag, `fork-only` series touching shared files, inventory review outcome) with the threshold for each.
- [ ] The document states I-053: upstream mergeability does not outrank the architecture, and names the review rule for a `fork-only` patch to a shared file.
- [ ] The document defines the `Jakeos-Divergence` vocabulary KRN-008 enforces and states which phase the fork is in, updated by the KRN lead when a phase gate is passed.
- [ ] The document is committed under `Documentation/jakeos/` in the kernel repository and linked from `kernel:README.md`.

#### Verification
- Review: kernel architecture lead sign-off recorded on the pull request.

#### Evidence
- none

### KRN-010 · Create the kernel fork repository with upstream remote and rebase workflow
- Type: build
- Milestone: V0
- Status: done
- Size: M
- Owner: @agent/claude
- Depends on: KRN-002, KRN-005, KRN-007, KRN-006, KRN-003, KRN-004, BLD-005
- Baseline: §5.1
- Risks: R-002
- Verified by: @jakebarnby

Establishes the kernel tree (its own repository or the path decided by BLD-005), the upstream remote for the chosen series, patch-series tracking, and a documented rebase workflow that every later KRN task builds on. Boot from a tagged commit is a V0 exit criterion owned with BOOT and BLD.

<!-- covers: INV-0119 -->

#### Out of scope
QEMU boot harness (BLD-012). OVMF boot of the image (BOOT-001). Native subsystem layout (KRN-013).

#### Acceptance criteria
- [x] The kernel alias in `registers/repos.md` resolves to a tree with an upstream remote on the series named by KRN-005.
- [x] A documented rebase workflow exists in the tree and is exercised once on a no-op range without rewriting published tags.
- [x] A tagged commit builds with the BLD-013 pin.

#### Verification
- Integration: `kernel:.github/workflows` clones the tree, fetches upstream and builds the tag on a GitHub-hosted runner; the QEMU boot smoke on H-001 belongs to BLD-012.
- Review: kernel architecture lead sign-off on the bootstrap README.

#### Evidence
- kernel@1e549edb57f3
- https://github.com/abnegate/jakeos-kernel/actions/runs/33958979664

### KRN-011 · Maintain a base defconfig plus per-target fragments with a build-and-boot check
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: KRN-010, KRN-017
- Baseline: §5.1, §55, §62

Uncontrolled `CONFIG` drift is the usual way a fork silently drops hardware it once supported (§55). The fork keeps one base configuration, `arch/x86/configs/jakeos_defconfig`, and Kconfig fragments under `kernel/configs/jakeos/`: `qemu-minimal.config` (the BLD-012 default cell), `qemu-desktop.config` (virtio-gpu, sound, TPM, for later V0.5 cells) and `h002.config` (the reference AMD desktop: its NVMe controller, Ethernet, RDNA 3-class GPU, IOMMU, TPM 2.0 and USB controllers). `scripts/jakeos/build-config.sh <fragment>` merges base plus fragment with `scripts/kconfig/merge_config.sh` and the CI jobs build and boot each result.

The fragments keep every facility the retained-mechanism inventory names enabled: `scripts/jakeos/check-retained-kconfig.py` reads the `kconfig` field of each `Documentation/jakeos/retained.toml` entry (KRN-017) and fails when a merged configuration disables one of those symbols. The QEMU fragments build and boot on `qemu-x86_64` in `pre-merge`; the H-002 fragment builds in `pre-merge` and boots on `hw-h002` in `nightly`.

<!-- covers: GAP-0097 -->

#### Out of scope
Intel and AMD laptop fragments (KRN-049). Hardening overlay (KRN-034). QEMU matrix axes (BLD-012). Inventory contents (KRN-017).

#### Deliverables
- kernel:arch/x86/configs/jakeos_defconfig · The base configuration.
- kernel:kernel/configs/jakeos/qemu-minimal.config · Fragment for the BLD-012 default cell.
- kernel:kernel/configs/jakeos/qemu-desktop.config · Fragment for QEMU cells with virtio-gpu, sound and TPM.
- kernel:kernel/configs/jakeos/h002.config · Fragment for the reference AMD desktop.
- kernel:scripts/jakeos/build-config.sh · Merges base plus fragment and writes `.config`.
- kernel:scripts/jakeos/check-retained-kconfig.py · Fails when a merged configuration disables a `kconfig` symbol named in `retained.toml`.
- kernel:.github/workflows/pre-merge.yml · `config-qemu-minimal`, `config-qemu-desktop` (build and boot) and `config-h002` (build) jobs.
- kernel:.github/workflows/nightly.yml · `config-h002-boot` on `hw-h002`.
- kernel:Documentation/jakeos/configs.md · Which fragment serves which H-ID, how to add one, and the retained-symbol rule.

#### Acceptance criteria
- [ ] `kernel/configs/jakeos/` contains `qemu-minimal.config`, `qemu-desktop.config` and `h002.config`, and `scripts/jakeos/build-config.sh <fragment>` produces a `.config` from `jakeos_defconfig` plus the fragment.
- [ ] `pre-merge` on `qemu-x86_64` builds and boots the two QEMU fragments and builds the H-002 fragment; `nightly` boots the H-002 fragment on `hw-h002` to the BLD-012 boot-complete marker.
- [ ] `check-retained-kconfig.py` fails a merged configuration that disables any symbol listed under `kconfig` in `Documentation/jakeos/retained.toml`, and runs in `pre-merge` for every fragment.

#### Verification
- Integration: build-and-boot job per fragment on the named matrix entries.
- Unit: `kernel:tests/config/retained_symbols_*`: a fragment that disables a retained symbol fails the check; the shipped fragments pass.

#### Evidence
- none

### KRN-012 · Wire KUnit and Rust kernel unit tests for native primitives into pre-merge CI
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: KRN-013, BLD-013, BLD-012
- Baseline: §50, §51

Capability derivation, Channel semantics and Operation lifecycles need in-kernel unit coverage on every pre-merge build, before integration tests through the guest agent are practical. This task wires KUnit and Rust kernel unit tests for the native subsystem into the `pre-merge` tier: the fragment `kernel/configs/jakeos/kunit.config` enables `CONFIG_KUNIT` and every `CONFIG_JAKEOS_*_KUNIT_TEST` symbol, `tools/testing/kunit/kunit.py run --kunitconfig kernel/configs/jakeos/kunit.config --arch x86_64` runs them under the BLD-012 default cell, and Rust doctests and `#[kunit_tests]` modules in `jakeos/` are included through the Rust-for-Linux KUnit integration. The job `native-unit` in `pre-merge` fails the change on any failing case.

The harness path is documented so CAP, IPC and TSK add tests without touching the runner: a test module lives at `kernel:jakeos/<area>/tests/<name>.rs`, is registered by `obj-$(CONFIG_JAKEOS_<AREA>_KUNIT_TEST) += tests/<name>.o` in the area's Makefile, and its Kconfig symbol defaults to `y` under `CONFIG_JAKEOS_KUNIT_ALL`. Tests exercise native entry points only; a test that calls the Linux syscall path as if it were a native API fails review.

<!-- covers: GAP-0113 -->

#### Out of scope
Capability test content (CAP). Channel test content (IPC). Operation lifecycle test content (TSK). Sanitizer profiles (BLD-026). Guest-agent integration tests (BLD-006).

#### Deliverables
- kernel:kernel/configs/jakeos/kunit.config · Fragment enabling KUnit and every native-subsystem test symbol.
- kernel:jakeos/Kconfig.tests · `CONFIG_JAKEOS_KUNIT_ALL` and the per-area `CONFIG_JAKEOS_<AREA>_KUNIT_TEST` symbols.
- kernel:scripts/jakeos/run-kunit.sh · Wrapper over `kunit.py run` with the fragment, the QEMU default cell and JUnit output for CI.
- kernel:.github/workflows/pre-merge.yml · The `native-unit` job.
- kernel:Documentation/jakeos/testing.md · How to add a KUnit or Rust unit test for a native area, the file and Kconfig conventions, and the rule against testing through the Linux syscall path.
- kernel:jakeos/tests/smoke.rs · One always-present test proving the harness runs.

#### Acceptance criteria
- [ ] `pre-merge` on `qemu-x86_64` runs `scripts/jakeos/run-kunit.sh`, executes every `CONFIG_JAKEOS_*_KUNIT_TEST` module and Rust doctest under `jakeos/`, and fails the change on any failing case.
- [ ] `Documentation/jakeos/testing.md` documents the `jakeos/<area>/tests/<name>.rs` path, the Makefile and Kconfig lines, and a new test added by following it runs without any change to `run-kunit.sh` or the workflow.
- [ ] The documentation states, and review enforces, that native unit tests call native entry points and never the Linux syscall table as a native API.

#### Verification
- Integration: a deliberately failing native KUnit case is rejected by `pre-merge` on `qemu-x86_64`.
- Unit: `kernel:tests/kunit/harness_*`: the smoke test appears in the JUnit output of a `run-kunit.sh` run.
- Review: BLD CI owner confirms the `native-unit` job is in the `pre-merge` tier.

#### Evidence
- none

### KRN-013 · Add the native platform as an in-tree kernel subsystem beside Linux facilities
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: KRN-010, KRN-001, KRN-017, KRN-015, KRN-016, KRN-014
- Baseline: §5.1, §6
- Invariants: I-010

Phase A mechanics (§6): the native platform is one self-contained subsystem in the kernel tree, enabled by a single Kconfig symbol, with minimal hooks into core files, such that a build with the symbol disabled is functionally the chosen upstream tag. This task creates the layout every kernel-side task lands into and documents it in `Documentation/jakeos/layout.md`: the subsystem directory `jakeos/` with `Kconfig`, `Makefile`, one directory per area (`abi/`, `cap/`, `cmp/`, `ipc/`, `mem/`, `tsk/`, `sch/`, `obs/`), each holding a Rust crate root `<area>.rs` and a `tests/` directory (KRN-012); Rust abstractions over Linux internals that the areas share in `rust/kernel/jakeos/`; user-visible ABI headers in `include/uapi/linux/jakeos/` (generated by ABI-017's source of truth once it exists); kselftests in `tools/testing/selftests/jakeos/<area>/`; documentation in `Documentation/jakeos/`. The roadmap's `kernel:tests/<area>/<name>_*` paths denote `tools/testing/selftests/jakeos/<area>/<name>_*`, and this document says so.

Hooks into core files are the minimum: an `#ifdef CONFIG_JAKEOS` call-out in the syscall entry path for the ABI-002 entry layer, a hook in `exit`/`fork` for Component lifetime (CMP-005), and a hook in the scheduler for ResourceDomain accounting (SCH); each hook is a one-line call to a function in `jakeos/hooks.c` so upstream merges touch as little as possible. New files in the subsystem are Rust unless an accepted decision exempts a named file (KRN-015, KRN-016). The Linux syscall path remains for the Linux personality.

<!-- covers: INV-0140, INV-0138, INV-0015 -->

#### Out of scope
Entry layer dispatch (ABI-002). Capability table (CAP-005). Component wrapper (CMP-005). Kernel unit-test wiring (KRN-012). Platform monorepo layout (BLD-081).

#### Deliverables
- kernel:jakeos/Kconfig · `CONFIG_JAKEOS` (default `n`) and the per-area symbols beneath it.
- kernel:jakeos/Makefile · `obj-$(CONFIG_JAKEOS)` entries for every area crate root.
- kernel:jakeos/hooks.c · The single C file holding the core-file hook targets, each a one-line call site in core Linux files.
- kernel:jakeos/<area>/<area>.rs · Empty crate roots for `abi`, `cap`, `cmp`, `ipc`, `mem`, `tsk`, `sch`, `obs`, each compiling under `CONFIG_JAKEOS`.
- kernel:rust/kernel/jakeos/mod.rs · Shared Rust abstractions module in the `kernel` crate, behind `CONFIG_JAKEOS`.
- kernel:include/uapi/linux/jakeos/README · Placeholder stating that headers here are generated from the ABI-017 specification.
- kernel:tools/testing/selftests/jakeos/Makefile · Selftest scaffolding with one directory per area.
- kernel:Documentation/jakeos/layout.md · The layout above, the `kernel:tests/<area>/…` path rule, the hook rule, and the Rust-first rule pointer.

#### Acceptance criteria
- [ ] `jakeos/` exists with `Kconfig`, `Makefile`, `hooks.c`, the eight area directories with compiling crate roots, and `CONFIG_JAKEOS` is the single symbol that enables all of it, default `n`.
- [ ] A build with `CONFIG_JAKEOS=n` boots on `qemu-x86_64` and passes the KRN-014 retained-subsystem kselftest subset and the C-001 L0 subset that KRN-014 runs, matching the upstream tag's behaviour.
- [ ] Every hook into a core Linux file is a one-line call to a function in `jakeos/hooks.c` guarded by `CONFIG_JAKEOS`, and `Documentation/jakeos/layout.md` lists each hook with the core file it touches.
- [ ] New files under `jakeos/` are Rust; the KRN-016 lint passes on the initial tree, and `layout.md` states the `kernel:tests/<area>/<name>_*` to `tools/testing/selftests/jakeos/<area>/` rule.

#### Verification
- Integration: `CONFIG_JAKEOS=n` build boots on `qemu-x86_64` and the retained-subsystem kselftest subset is green; `CONFIG_JAKEOS=y` build boots and the KRN-012 smoke test runs.
- Unit: `kernel:tests/layout/hooks_*`: every `CONFIG_JAKEOS` reference outside `jakeos/` and `rust/kernel/jakeos/` is a hook listed in `layout.md`.

#### Evidence
- none

### KRN-014 · Define the retained-subsystem regression matrix as a merge-blocking Gate
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: KRN-017, KRN-011, BLD-012
- Baseline: §2, §5.1, §55
- Corpora: C-001
- Risks: R-004, R-013
- Invariants: I-054, I-098

The fork preserves Linux's hardware layer (§2, §55), so a native change that breaks a retained subsystem must be caught before it lands. This task generates the regression matrix from the inventory: `tools/jakeos/retained-matrix/` (crate `jakeos-retained-matrix`) reads every `Documentation/jakeos/retained.toml` entry whose `kselftest` field names a subset (DRM, PCI, USB, NVMe/block, networking, ACPI at minimum) and writes `Documentation/jakeos/retained-matrix.toml`: one row per subset with the H-IDs it runs on (`H-001` for everything, `H-002` for everything with real hardware behind it) and the `retained.toml` entry it guards. It also adds `tools/testing/selftests/jakeos/licence/module_licence_test.sh`, which walks every built module, asserts `MODULE_LICENSE` is present and GPL-compatible, and asserts that no `EXPORT_SYMBOL_GPL` symbol is used by a module whose licence is not GPL-compatible.

BLD hosts the runners (BLD-007, BLD-012); this task owns the contents and the gate rule: the `pre-merge` job `retained-matrix` runs the matrix on `qemu-x86_64` and fails the change on any red subset, and the `nightly` job runs it on `hw-h002`; while any `hw-h002` row is red, the merge queue for changes under `jakeos/` is paused by the `retained-red` required status that BLD-007 publishes.

<!-- covers: INV-1045, INV-1046, INV-1047, INV-1048, INV-1049, INV-1050, INV-1051, GAP-0018, INV-0030 -->

#### Out of scope
Runner and QEMU axes (BLD-012, BLD-007). L0 corpus scenarios (LNX-002). Audio driver inventory (AUD, through the retained-mechanism list). Inventory contents (KRN-017).

#### Deliverables
- kernel:tools/jakeos/retained-matrix/ · Crate `jakeos-retained-matrix`: generates the matrix from `retained.toml` and fails when an entry names a `kselftest` subset that does not exist in the tree.
- kernel:Documentation/jakeos/retained-matrix.toml · Generated matrix: subset, H-IDs, guarded entry.
- kernel:tools/testing/selftests/jakeos/licence/module_licence_test.sh · `MODULE_LICENSE` presence and GPL-only-symbol check over every built module.
- kernel:.github/workflows/pre-merge.yml · The `retained-matrix` job on `qemu-x86_64`.
- kernel:.github/workflows/nightly.yml · The `retained-matrix` job on `hw-h002`.
- kernel:Documentation/jakeos/retained-matrix.md · The gate rule, how a subset is added (through `retained.toml`, never here), and what a red row blocks.

#### Acceptance criteria
- [ ] `Documentation/jakeos/retained-matrix.toml` is generated from the `kselftest` entries of `Documentation/jakeos/retained.toml` and maps each retained DRM, PCI, USB, NVMe/block, networking and ACPI subset to `H-001` and `H-002`; a hand edit to the generated file fails the `retained-matrix` job.
- [ ] `module_licence_test.sh` is a matrix row and fails when a built module lacks `MODULE_LICENSE`, carries a non-GPL-compatible licence, or uses an `EXPORT_SYMBOL_GPL` symbol without a GPL-compatible licence.
- [ ] `pre-merge` on `qemu-x86_64` and `nightly` on `hw-h002` run the matrix; a red subset in `pre-merge` blocks the change that introduced it.
- [ ] While any `hw-h002` row is red, no change under `jakeos/` lands: the `retained-red` status published by BLD-007 is required for those paths.

#### Verification
- Integration: matrix jobs on `qemu-x86_64` and `hw-h002`; a fixture that breaks a retained NVMe kselftest fails `pre-merge`.
- Unit: `kernel:tests/retained/matrix_*`: generation from a fixture inventory, missing-subset failure, hand-edit detection.
- Compat: C-001 on H-001 and H-002 remains the LNX-owned detector for syscall-path regressions; this matrix is the hardware-layer detector.
- Review: kernel architecture lead accepts the inventory-to-kselftest mapping.

#### Evidence
- none

### KRN-015 · Write the rewrite-versus-retain policy for replacing inherited C with Rust
- Type: docs
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: KRN-002, KRN-018, KRN-003
- Baseline: §1, §2, §50, §57
- Invariants: I-009

The no-heroic-rewrite rules (§2, §57, Principle 15 of §67) become one document, `Documentation/jakeos/rewrite-policy.md`: new kernel code under `jakeos/` and `rust/kernel/jakeos/` is Rust unless an accepted decision exempts a named file; inherited C is replaced only by an accepted decision that cites a semantic benefit (a native semantic the C cannot express) or a measured benefit (a B-ID report), and mature C stays where a rewrite would add insufficient value. The document defines the exemption-list format the KRN-016 lint reads: `Documentation/jakeos/rust-exemptions.toml`, one entry per exempted file with `path`, `decision` (a D-ID) and `reason`, and one entry per approved rewrite with `replaces` (the C path), `decision` and `benefit` (`semantic` or a B-ID).

It also records the boundary KRN-018's spike found (which subsystems have usable Rust abstractions, which need new ones) so a developer knows before starting whether a piece of work is Rust-ready or needs an abstraction first, and it states that the personalities' C code is not covered by this rule because it is not kernel code.

<!-- covers: INV-0947, INV-0014, INV-0061, INV-1115, INV-0935, INV-0138, INV-0006 -->

#### Out of scope
The CI lint (KRN-016). Per-subsystem rewrite decisions owned by MEM, SCH, NET, STO. The Rust abstraction spike itself (KRN-018).

#### Deliverables
- kernel:Documentation/jakeos/rewrite-policy.md · The rules, the exemption-list format, the KRN-018 boundary summary and the scope statement.
- kernel:Documentation/jakeos/rust-exemptions.toml · The initial, empty exemption and rewrite list with the documented schema in comments.

#### Acceptance criteria
- [ ] `Documentation/jakeos/rewrite-policy.md` states that new code under `jakeos/` and `rust/kernel/jakeos/` is Rust unless `rust-exemptions.toml` names the file with an accepted decision.
- [ ] The document states that replacing inherited C requires an accepted decision citing a semantic benefit or a B-ID report, and defines the `rust-exemptions.toml` schema (`path`, `decision`, `reason`; `replaces`, `decision`, `benefit`) that KRN-016 reads.
- [ ] The document summarises the KRN-018 boundary per subsystem (mm, scheduler, DRM, VFS) as Rust-ready, needs-abstraction or C-only-for-now, and is linked from `Documentation/jakeos/layout.md`.

#### Verification
- Review: kernel architecture lead sign-off recorded on the pull request.

#### Evidence
- none

### KRN-016 · Enforce Rust-first and rewrite-requires-ADR rules with a kernel-tree CI lint
- Type: build
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: KRN-015, KRN-010, BLD-013, BLD-011
- Baseline: §50, §51, §57
- Invariants: I-082, I-009

V0 exit requires that all new kernel code is Rust unless an accepted decision exempts a specific file, and that clippy and rustfmt are clean (§50). The lint `scripts/jakeos/rust-first-lint.py` runs in `pre-merge` over `<base>..HEAD`: a new `.c` or `.h` file under `jakeos/` or `rust/kernel/jakeos/` fails unless `Documentation/jakeos/rust-exemptions.toml` lists its path with a `decision` whose file in the roadmap repository is `accepted` (the lint fetches `decisions/<D-ID>-*.md` from the roadmap at the pinned commit in `Documentation/jakeos/roadmap-pin`); a diff that deletes inherited C outside `jakeos/` and adds Rust that replaces it fails unless the exemption list has a matching `replaces` entry with an accepted decision. `cargo clippy -D warnings` and `rustfmt --check` run over the native subsystem through the BLD-011 style jobs; this task adds the Rust-first checks as the `rust-first` job.

Exemptions are never comments in code; the only mechanism is the TOML entry with a decision. The lint prints the exact entry a developer must add, so a rejected change knows what to do next.

<!-- covers: INV-0934, INV-0935, INV-1115, INV-0014 -->

#### Out of scope
Unsafe-code inventory publication (KRN-056). Userspace `forbid(unsafe_code)` templates (SDK). Licence allowlist scanning (BLD-011). Policy text (KRN-015).

#### Deliverables
- kernel:scripts/jakeos/rust-first-lint.py · New-C detection, rewrite-without-decision detection, exemption lookup against the roadmap's decision status.
- kernel:Documentation/jakeos/roadmap-pin · The roadmap commit the lint reads decision status from, bumped by the queue.
- kernel:.github/workflows/pre-merge.yml · The `rust-first` job.
- kernel:tools/jakeos/lint-fixtures/rust-first/ · Fixtures: exempted C file, unexempted new C file, rewrite without a decision, rewrite with an accepted decision.

#### Acceptance criteria
- [ ] `pre-merge` on `qemu-x86_64` fails a new `.c` or `.h` file under `jakeos/` or `rust/kernel/jakeos/` unless `rust-exemptions.toml` names it with a decision that is `accepted` in the roadmap at `roadmap-pin`.
- [ ] `pre-merge` fails a diff that deletes inherited C and adds a Rust replacement without a matching `replaces` entry backed by an accepted decision, and passes the same diff when the entry exists.
- [ ] `cargo clippy -D warnings` and `rustfmt --check` are clean for the native subsystem on every `pre-merge` run.
- [ ] A rejected change's log names the exact `rust-exemptions.toml` entry that would make it pass.

#### Verification
- Unit: `kernel:tests/lint/rust_first_*` over the four fixtures.
- Integration: `pre-merge` job on `qemu-x86_64`.

#### Evidence
- none

### KRN-017 · Produce the retained-mechanism inventory from a study of Linux subsystems
- Type: spike
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: KRN-002
- Baseline: §2, §5.1, §55, §58
- Invariants: I-010, I-054

The forty-six preserve-or-retain items of the baseline collapse into one inventory (§2, §5.1, §55). The spike studies the Linux tree at the merged tag and writes two things: the report `reports/spikes/KRN-017.md` in the roadmap repository, and the machine-readable inventory `Documentation/jakeos/retained.toml` in the kernel tree with one `[[mechanism]]` entry per retained mechanism carrying `subsystem` (for example `mm`, `sched`, `drm`, `block`, `net`, `kvm`, `arch/x86`), `path` (the tree path), `mechanism` (what is kept, for example `page tables`, `SLUB`, `CFS/EEVDF`, `blk-mq`, `netfilter`), `replaces` (the native semantic it does not provide, for example `process identity`, `ambient authority`), `kselftest` (the selftest subset that guards it, or `none`), `kconfig` (the symbols a fragment must keep enabled) and `arch` (`retained`, `compile-kept` or `dropped`). ARM64 and RISC-V arch code are `compile-kept`, not 1.0 platforms (I-011, I-012). KVM, page tables, allocators, interrupts and x86-64 are `retained`.

KRN-011 (fragments), KRN-014 (regression matrix), BLD-012 (nightly kselftests) and NET-001 consume the TOML without a second copy; later milestone reviews annotate the report rather than re-litigating each subsystem.

<!-- covers: INV-1131, INV-0015, INV-1337, INV-0063, INV-0064, INV-0068, INV-0121, INV-0122, INV-0123, INV-0133, INV-0030, INV-0031, INV-0032, INV-0115, INV-0029, INV-0026 -->

#### Out of scope
Regression matrix contents (KRN-014). Per-workstream inventories that consume this list (NET-001). Redox and XNU studies (GOV research programme). Kconfig fragments (KRN-011).

#### Deliverables
- roadmap:reports/spikes/KRN-017.md · The report: method, per-subsystem findings, what is retained, replaced, compile-kept or dropped, and which later gate reviews the list.
- kernel:Documentation/jakeos/retained.toml · The inventory: one `[[mechanism]]` per retained mechanism with the seven fields.
- kernel:Documentation/jakeos/retained.md · The schema of `retained.toml`, who consumes each field, and the rule that consumers never keep a second copy.
- kernel:scripts/jakeos/check-retained-toml.py · Schema check: required fields present, `arch` in the allowed set, every `path` exists in the tree, every `kselftest` subset exists.

#### Acceptance criteria
- [ ] `reports/spikes/KRN-017.md` lists each retained mechanism, the Linux subsystem path, and the native semantic it does not provide, and names the later gate that reviews the list.
- [ ] `retained.toml` marks ARM64 and RISC-V arch code `compile-kept` (I-011, I-012) and marks KVM, page tables, allocators, interrupts and x86-64 `retained`.
- [ ] Every `[[mechanism]]` entry carries `subsystem`, `path`, `mechanism`, `replaces`, `kselftest`, `kconfig` and `arch`, and `check-retained-toml.py` passes on the committed file and fails a fixture missing a field or naming a nonexistent path.
- [ ] KRN-011 and KRN-014 read `retained.toml` directly; the report cites it and no consumer keeps a second list.

#### Verification
- Report: which mechanisms are retained, which semantics are replaced, which arch code is compile-kept, and which later gate reviews the list.
- Unit: `kernel:tests/retained/schema_*` over the committed inventory and the failing fixtures.
- Review: kernel architecture lead sign-off on the spike report.

#### Evidence
- none

### KRN-018 · Spike Rust-in-kernel viability and decide the C-versus-Rust boundary
- Type: spike
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: KRN-004, KRN-010, BLD-013
- Baseline: §50, §51
- Risks: R-002

The Rust-for-Linux API surface at the merged tag may lack bindings to memory management, the scheduler, DRM and VFS that the native subsystem needs. This spike writes, for each of the four, a minimal Rust probe under `jakeos/spikes/rust-boundary/` (not shipped) that attempts the operations the V0 tasks need (mm: allocate and map pages into a process, walk a VMA; scheduler: create a kernel thread, attach to a cgroup, read the runqueue clock; DRM: obtain a dma-buf and export it; VFS: open a file by path from kernel context and read it) using only upstream abstractions, and records whether an abstraction exists, is missing, or exists but is insufficient (and why) in `reports/spikes/KRN-018.md`. The report names the C-versus-Rust boundary the KRN-015 policy and the KRN-016 lint enforce, and lists the missing bindings that are V0 blockers versus later work, so the areas know whether to write an abstraction in `rust/kernel/jakeos/` first.

No subsystem is rewritten here and no performance number is claimed; any cost comparison cites a B-ID or is qualitative.

<!-- covers: GAP-0535, INV-0934 -->

#### Out of scope
The rewrite policy document (KRN-015). The lint (KRN-016). DRM stability while building graphics (GFX). Writing the missing abstractions (the owning areas, via `rust/kernel/jakeos/`).

#### Deliverables
- roadmap:reports/spikes/KRN-018.md · Findings per subsystem, the boundary, the V0-blocking bindings list.
- kernel:jakeos/spikes/rust-boundary/ · The four probes as `#[cfg(CONFIG_JAKEOS_SPIKES)]` modules, kept for reference and never built by default.

#### Acceptance criteria
- [ ] `reports/spikes/KRN-018.md` records, for mm, scheduler, DRM and VFS, whether a Rust abstraction exists upstream, is missing, or is insufficient for the native platform, with the probe that showed it.
- [ ] The report names the C-versus-Rust boundary the KRN-016 lint enforces and lists the missing bindings that block V0 tasks versus those that can wait.
- [ ] The report claims no performance number; any cost comparison cites a B-ID or is qualitative.
- [ ] The probes exist under `jakeos/spikes/rust-boundary/` behind `CONFIG_JAKEOS_SPIKES` and are excluded from every shipped configuration.

#### Verification
- Report: which subsystems need new Rust abstractions, what the C-versus-Rust boundary is, and which missing bindings are V0 blockers versus later work.
- Review: kernel architecture lead sign-off on the spike report.

#### Evidence
- none

### KRN-019 · Verify V0 kernel gates: reproducible boot from a tagged commit and L0 parity
- Type: docs
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: KRN-013, KRN-014, KRN-011, KRN-016, KRN-008, KRN-012, BOOT-001, LNX-002, HW-001
- Baseline: §5.1, §6, §59
- Corpora: C-001

V0 exit for the kernel (§59): the fork boots on `qemu-x86_64` and `hw-h002` from a CI-built image reproducible from a tagged commit, native Components start from the retained initramfs (D-0051), and the L0 corpus (C-001) matches the unforked kernel of the same upstream tag. This task is the roll-up: `Documentation/jakeos/reviews/v0-kernel.md` records the tag, the BLD-009 input manifest hash, the boot logs from both matrix entries, the C-001 report paths for both machines and the baseline kernel they were compared with, and the first milestone review note on the retained-mechanism inventory (which entries were touched by V0 work, which gate reviews it next).

Nothing new is built; the task exists so the V0 gate has one document to point at and so the review of the inventory happens at a fixed point rather than never.

<!-- covers: INV-1337, INV-0139 -->

#### Out of scope
Native init (SVC-007). Image builder (INS). OVMF boot implementation (BOOT-001). L0 scenario content (LNX-002). Hardware bring-up (HW-001).

#### Deliverables
- kernel:Documentation/jakeos/reviews/v0-kernel.md · The roll-up: tag, manifest hash, boot logs, C-001 comparison, inventory review note.
- roadmap:reports/compat/C-001/ · The C-001 reports for H-001 and H-002 referenced by the roll-up (produced by LNX-002's runs, cited here).

#### Acceptance criteria
- [ ] The roll-up names a tagged commit whose CI image boots on `qemu-x86_64` and `hw-h002` from the retained initramfs and starts a native Component, with both boot logs attached.
- [ ] C-001 on H-001 and H-002 matches the unforked baseline kernel of the same upstream tag, and the roll-up cites the `reports/compat/C-001/` reports for both.
- [ ] The retained-mechanism inventory has a V0 review note in the roll-up and on `reports/spikes/KRN-017.md` naming the entries V0 touched and the gate that reviews the list next.

#### Verification
- Integration: boot logs from `qemu-x86_64` and `hw-h002` attached as Evidence when the task is done.
- Compat: C-001 reports for H-001 and H-002.
- Review: kernel architecture lead sign-off on the V0 kernel roll-up.

#### Evidence
- none

### KRN-020 · Merge the first upstream stable release into the fork with boot matrix green
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: KRN-023, KRN-014, KRN-007, LNX-002
- Baseline: §6
- Corpora: C-001
- Risks: R-029

Rehearses the tracking strategy on a real upstream stable release before the V1 gate requires it, keeping Phase A close to upstream and producing the measured rebase cost KRN-005 asked for.

<!-- covers: GAP-0534, INV-0140 -->

#### Out of scope
The cost report write-up (KRN-021). V1 rebase gate (KRN-040).

#### Acceptance criteria
- [ ] One upstream stable tag of the chosen series is merged or rebased per KRN-007.
- [ ] The retained-subsystem matrix is green on H-001 and H-002 after the merge.
- [ ] C-001 on H-001 and H-002 has zero regressions versus the pre-merge fork.

#### Verification
- Integration: boot matrix on `qemu-x86_64` and `hw-h002` after the merge.
- Compat: C-001 on H-001 and H-002.
- Review: kernel architecture lead accepts the merge commit.

#### Evidence
- none

### KRN-021 · Publish the six-month rebase cost report and revisit the tracking ADRs
- Type: docs
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: KRN-020, KRN-022, KRN-005, KRN-007
- Baseline: §6, §56.4
- Risks: R-029

GAP-0534 requires a measured rebase cost after the first tracking interval. The report records engineer-days, conflict counts by subsystem and regressions, and reopens the base and tracking Decisions if the thresholds named in those Decisions are exceeded.

<!-- covers: GAP-0534 -->

#### Out of scope
Changing the base series (a superseding Decision owned by a later adr task). Merge-bot implementation (KRN-023).

#### Acceptance criteria
- [ ] A committed report lists engineer-days, conflict counts by subsystem, and retained-matrix regressions for the first stable merge.
- [ ] The report states whether the reopen conditions on KRN-005 and KRN-007 fired.
- [ ] No performance number appears; conflict counts and engineer-days are process metrics, not B-IDs.

#### Verification
- Review: kernel architecture lead sign-off on the report pull request.

#### Evidence
- none

### KRN-022 · Generate the upstream delta report: patch delta, conflict hot-spots, unmerged fixes
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: KRN-008, KRN-023
- Baseline: §6, §56.4

Consumes ledger and bot output to publish patch delta, conflict hot-spots and unmerged upstream security fixes per subsystem. Feeds KRN-042 and the CVE pipeline.

<!-- covers: INV-0157, INV-1101 -->

#### Out of scope
CVE ingestion (KRN-030). Phase D Decision (KRN-042).

#### Acceptance criteria
- [ ] A generated report per trial-merged upstream tag lists patch delta, conflict hot-spots by subsystem, and unmerged upstream security fixes.
- [ ] The report is produced from ledger classifications plus bot output with no hand-edited counts.
- [ ] CI stores the latest report as a build artifact on `qemu-x86_64`.

#### Verification
- Unit: `kernel:tools/divergence/delta_*` against a fixture tag.
- Integration: bot plus ledger produce a report in CI.

#### Evidence
- none

### KRN-023 · Build the upstream-tracking bot that trial-merges stable and mainline tags
- Type: build
- Milestone: V0.5
- Status: todo
- Size: L
- Owner: none
- Depends on: KRN-010, KRN-007, KRN-011, BLD-012
- Baseline: §6, §56.4
- Risks: R-029

Attempts merges of each upstream stable and mainline tag, reports conflicts by subsystem, and runs the boot matrix on the result, giving continuous visibility of divergence cost before Phase D. The bot does not publish to the fork's main branch without a human merge.

<!-- covers: GAP-0125 -->

#### Out of scope
Delta-report formatting (KRN-022). Human merge of the first stable (KRN-020).

#### Acceptance criteria
- [ ] The bot trial-merges each new upstream stable and mainline tag of the chosen series into a throwaway branch.
- [ ] Conflicts are reported grouped by subsystem.
- [ ] The boot matrix from BLD-012 runs on a successful trial merge on `qemu-x86_64`.
- [ ] A failed trial merge never updates the fork's default branch.

#### Verification
- Integration: a fixture upstream tag with a planted conflict produces a per-subsystem conflict report and does not update default.
- Integration: a clean tag boots on `qemu-x86_64`.
- Review: kernel architecture lead accepts the bot's branch and permission model.

#### Evidence
- none

### KRN-024 · Decide eBPF's native role and the Linux Personality's bpf() exposure
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: KRN-017, OBS-003, LNX-003
- Baseline: §24, §46, §58
- Decision: D-0156
- Risks: R-034

Decides whether eBPF is the tracing substrate, a sched_ext host, a network-policy engine natively, some combination, or retained only for the Linux personality, and whether that personality exposes `bpf()` beyond a CAP_BPF-equivalent Capability. OBS and NET tasks at V1 and V2 depend on it. Native software never sees `bpf()` as a native API.

<!-- covers: EXTRA-013 -->

#### Out of scope
Tracing substrate implementation (OBS-011). Network capability broker (NET-012). Per-domain sched_ext policies (SCH-061).

#### Acceptance criteria
- [ ] Options evaluated include eBPF as native tracing substrate, as sched_ext host, as network-policy engine, as personality-only, and combinations of those roles.
- [ ] The accepted option states whether the Linux personality exposes `bpf()` and which Capability gates it.
- [ ] Native crates have no `bpf()` entry in Layer 1.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: kernel architecture lead, OBS lead and LNX lead sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- none

### KRN-025 · Decide how KVM is exposed natively as Capability<VirtualMachine>
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: KRN-017, KRN-001
- Baseline: §36, §69
- Decision: D-0159
- Invariants: I-081

Answers Q-048: whether virtualization is `Capability<VirtualMachine>`, and whether compatibility personalities may fall back to VMs for unsupported software. The VIRT VM manager and the kernel object task depend on this Decision. Each VM remains a capability-scoped Component.

<!-- covers: INV-1329 -->

#### Out of scope
Kernel object implementation (KRN-037). VM manager (VIRT-008). Nested-virt CI (KRN-036). Personality fallback product (VIRT-002).

#### Acceptance criteria
- [ ] Options evaluated include `Capability<VirtualMachine>` as a kernel object, KVM retained only for the Linux personality, and no host VM product.
- [ ] The accepted option states whether personalities may fall back to VMs and that host access is granted through Capabilities visible in `os inspect`.
- [ ] Q-048 is marked answered by this task.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: kernel architecture lead and VIRT lead sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- none

### KRN-026 · Decide that kernel live-patching is a non-goal in favour of generations plus reboot
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: KRN-009, PKG-009
- Baseline: §30, §56.4
- Decision: D-0160
- Invariants: I-086

Records SystemGenerations plus reboot as the update model, with kernel live-patching as a named option, before the update channel ships at V1. Live-patching is a non-goal (I-086); this Decision is how that non-goal is enforced. Reboot-free generation switch is a separate PKG Decision.

<!-- covers: EXTRA-041 -->

#### Out of scope
Reboot-free SystemGeneration switch (PKG-070). Updater UX (INS). kexec policy under lockdown (BOOT, SEC-031).

#### Acceptance criteria
- [ ] Options evaluated include kernel live-patching as a supported update path, SystemGenerations plus reboot only, and kexec into a new kernel without a livepatch series.
- [ ] The accepted option is recorded against I-086 and names the rejected live-patching path.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: kernel architecture lead and PKG lead sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- none

### KRN-027 · Decide module signing under Secure Boot for out-of-tree, GPU and local modules
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: KRN-028
- Baseline: §51, §56.1
- Decision: D-0161
- Risks: R-050

Pulled from V2 to V1 because HW-018 depends on it. Options include project key only, MOK enrolment, and local developer keys with taint. Secure Boot distribution (shim versus enrolled keys) remains BOOT.

<!-- covers: GAP-0327 -->

#### Out of scope
NVIDIA driver stance (HW-018). Secure Boot shim versus project keys (BOOT-031). Enforcement implementation (KRN-038).

#### Acceptance criteria
- [ ] Options evaluated include project key only, MOK enrolment of third-party keys, and local developer keys that taint the kernel.
- [ ] The accepted option states how out-of-tree, proprietary GPU and locally built modules load under Secure Boot, or that they do not.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: kernel architecture lead, HW lead and SEC lead sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- none

### KRN-028 · Decide out-of-tree module policy: GPL-only native exports and taint semantics
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: KRN-003, KRN-017, KRN-014
- Baseline: §5.1, §51
- Decision: D-0163

Settles whether native kernel symbols are exported GPL-only and how taint behaves before the native ABI stabilizes at V1. MODULE_LICENSE semantics for inherited modules are preserved as a named option so hardware support does not silently change legal status.

<!-- covers: GAP-0017, GAP-0018 -->

#### Out of scope
Signing keys under Secure Boot (KRN-027). Proprietary GPU policy (HW-018, GFX-047).

#### Acceptance criteria
- [ ] Options evaluated include GPL-only native exports with inherited MODULE_LICENSE unchanged, permissive native exports, and no loadable modules for native objects.
- [ ] The accepted option states taint behavior for proprietary and out-of-tree modules.
- [ ] Inherited MODULE_LICENSE and GPL-only-symbol tests from KRN-014 remain required.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: kernel architecture lead and GOV licensing reviewer sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- none

### KRN-029 · Benchmark security mitigation overhead with mitigations on versus off
- Type: benchmark
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: KRN-034, Q-001, BEN-007
- Baseline: §51, §54
- Benchmarks: B-040

The V4 benchmark gate cites mitigation overhead measured and published. The harness lands with the hardening baseline so every later baseline change is costed on the IPC, Component-creation and L2 workloads named by B-040. This task publishes; it does not set a numeric target in prose.

<!-- covers: GAP-0185 -->

#### Out of scope
V4 gate publication of B-040 (BEN-055). Hardening config contents (KRN-034). BEN methodology (Q-001).

#### Acceptance criteria
- [ ] `bench:mitigation-overhead` runs mitigations-on versus mitigations-off on the B-001, B-004, B-016 and B-026 workloads named by B-040.
- [ ] Reports exist for H-001, H-002 and H-004.
- [ ] No superiority claim appears outside the report files.

#### Verification
- Bench: B-040 on H-001, H-002 and H-004; target per register (publish at V4; this task lands the harness and V1 measurements).
- Review: BEN owner accepts the harness path named in the register.

#### Evidence
- none

### KRN-030 · Build the CVE ingestion pipeline tagging applicability against the fork
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: KRN-031, KRN-008, KRN-022
- Baseline: §56.4
- Risks: R-062

Pulls kernel CNA and upstream advisories, auto-tags applicability against forked or removed subsystems using the ledger, and opens kernel-side triage tasks. Required before the fork carries daily-driving users. REL-006 opens tracked tasks with policy deadlines; this pipeline supplies applicability.

<!-- covers: GAP-0349 -->

#### Out of scope
REL task filing and deadlines (REL-006). Advisory publication (REL-044). Backports into diverged code (KRN-045).

#### Acceptance criteria
- [ ] New kernel CNA and upstream advisories are ingested and tagged applicable, not-applicable or unknown against ledger classifications.
- [ ] An applicable item opens a kernel triage task that names the subsystem owner from KRN-039.
- [ ] Unknown tags are listed and cannot remain unknown through a milestone roll-up.

#### Verification
- Unit: `kernel:tools/cve/tag_*` against fixture advisories and ledger snapshots.
- Integration: one historical advisory is tagged and filed in CI.
- Review: REL security-response owner accepts the handoff fields.

#### Evidence
- none

### KRN-031 · Write the kernel CVE response process: triage, backport, test, disclosure
- Type: docs
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: KRN-008, KRN-039
- Baseline: §56.4

Defines triage, upstream patch identification, backport, testing and disclosure timeline for the forked kernel. REL owns publication and advisories; KRN owns the kernel-side steps this document names.

<!-- covers: INV-1095 -->

#### Out of scope
Advisory feed and disclosure policy (REL-047). Ingest automation (KRN-030). V3 drill (KRN-052).

#### Acceptance criteria
- [ ] The document names triage, upstream-patch identification, backport, retained-matrix testing and the handoff to REL disclosure.
- [ ] The document names the subsystem-owner map as the routing table.
- [ ] The document is committed in the kernel tree and cited by REL-047.

#### Verification
- Review: kernel architecture lead and REL security-response owner sign-off recorded on the pull request.

#### Evidence
- none

### KRN-032 · Build tooling to pull upstream driver changes into the fork per subsystem
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: KRN-014, KRN-017, KRN-023
- Baseline: §55, §56.4
- Risks: R-029, R-069

Per-subsystem cherry-pick and adapt tooling with conflict reports and regression-matrix runs, so driver fixes keep flowing while native hooks grow. GPU tracking is a specialised consumer of this tooling.

<!-- covers: INV-1096 -->

#### Out of scope
GPU-specific cadence (KRN-033). Phase D playbook (KRN-047). Hardware enablement after divergence (KRN-046).

#### Acceptance criteria
- [ ] A maintainer can select an upstream driver commit, apply it onto the fork, and receive a per-file conflict report grouped by subsystem.
- [ ] A successful adapt run executes the retained-subsystem matrix entries for that subsystem on `qemu-x86_64`.
- [ ] The tool refuses to drop a classified ledger patch without an explicit override.

#### Verification
- Integration: fixture upstream driver commit adapted onto a fixture fork with conflict and matrix output.
- Review: kernel architecture lead accepts the override policy.

#### Evidence
- none

### KRN-033 · Run the GPU driver update process tracking amdgpu, i915, xe and nouveau fixes
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: KRN-032, KRN-014
- Baseline: §55, §56.1
- Invariants: I-054

V1 requires GPU acceleration on the AMD desktop and Intel laptop. A tracking job and documented cadence bring upstream amdgpu, i915, xe and nouveau fixes into the fork without destabilizing DRM. NVIDIA policy is HW.

<!-- covers: INV-1067 -->

#### Out of scope
NVIDIA modules (HW-018). Compositor and Mesa (GFX). DRM kselftest contents (KRN-014).

#### Acceptance criteria
- [ ] A documented cadence names how often amdgpu, i915, xe and nouveau upstream fixes are adapted.
- [ ] A tracking job lists unmerged upstream GPU commits against the fork.
- [ ] DRM kselftest subsets stay green on H-002 and H-004 after each adapted batch.

#### Verification
- Integration: tracking job output on CI; DRM matrix on `hw-h002` and the Intel laptop matrix entry.
- Review: GFX lead confirms DRM is not destabilized by the cadence.

#### Evidence
- none

### KRN-034 · Define and CI-enforce the kernel hardening configuration baseline
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: KRN-011, KRN-028
- Baseline: §51
- Benchmarks: B-040

KASLR, stack protector, kCFI, SMEP/SMAP, IBT, shadow stacks, init_on_alloc/free, hardened usercopy, W^X and randomized freelists as a checked-in config, with a CI check that no fragment weakens it. Lockdown mode when Secure Boot is active is included so GAP-0184 is not a silent default.

<!-- covers: GAP-0185, INV-0959, GAP-0184 -->

#### Out of scope
Mitigation-overhead harness (KRN-029). Module signature enforcement (KRN-038). Side-channel statement (SEC-029).

#### Acceptance criteria
- [ ] A checked-in hardening fragment enables KASLR, stack protector, kCFI, SMEP/SMAP, IBT, shadow stacks, init_on_alloc/free, hardened usercopy, W^X and randomized freelists.
- [ ] CI fails if any per-target fragment disables a hardening symbol the baseline requires.
- [ ] Lockdown is enabled in the H-002 and H-004 fragments when Secure Boot is on.

#### Verification
- Unit: kconfig diff of each fragment against the hardening baseline.
- Integration: boot with the baseline on `qemu-x86_64` and `hw-h002`.

#### Evidence
- none

### KRN-035 · Set kernel identity: uname strings, version scheme and bug-report routing
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: KRN-010, KRN-002
- Baseline: §1, §56.4

Once developers daily-drive the OS at V1, crash and bug reports must never reach upstream Linux as Linux. This task fixes uname strings, the version scheme, taint flag text and the routing text in oops output.

<!-- covers: GAP-0051 -->

#### Out of scope
Crash-capture format (OBS-029). Product naming and the Linux mark (GOV-051).

#### Acceptance criteria
- [ ] `uname` on a booted image does not identify the kernel as Linux in the sysname field used by bug-report tools.
- [ ] Oops and panic text names the JakeOS report path and does not instruct reporters to file at upstream Linux.
- [ ] A regression test on `qemu-x86_64` fails if the sysname or oops footer regresses to upstream Linux identity.

#### Verification
- Unit: identity-string tests in `kernel:tests/identity_*`.
- Integration: boot on `qemu-x86_64` and capture `uname` plus a triggered oops footer.

#### Evidence
- none

### KRN-036 · Test KVM and nested virtualisation in CI including the harness inside a guest
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: KRN-017, KRN-011, BLD-012, BLD-003
- Baseline: §5.1, §36
- Invariants: I-081

Preservation claims for KVM need tests: the retained path and nested virtualization run in CI, and JakeOS runs its own test harness inside a guest. The native VirtualMachine object is a separate task.

<!-- covers: GAP-0158, INV-0029, INV-0133 -->

#### Out of scope
Capability-mediated VirtualMachine (KRN-037). VM manager (VIRT). Guest images as a product (VIRT-001).

#### Acceptance criteria
- [ ] CI on `qemu-x86_64` with nested virtualization enabled boots a KVM guest of the fork.
- [ ] The guest runs the kernel's own KUnit or kselftest harness and reports results to the host.
- [ ] A failure in the guest harness fails the host CI job.

#### Verification
- Integration: nested-virt job on `qemu-x86_64` with host-passthrough as named by BLD-003.
- Review: BLD CI owner confirms the job is in the post-merge or nightly tier.

#### Evidence
- none

### KRN-037 · Expose retained KVM to native software as a Capability-mediated VirtualMachine
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: KRN-025, KRN-013, KRN-036, CAP-005, ABI-005, MEM-003
- Baseline: §7, §36, §69
- Invariants: I-081

Kernel-side object wrapping KVM VM and vCPU handles behind Capabilities, with MemoryObject-backed guest memory. Native software never sees `/dev/kvm` or POSIX fds. The VIRT workstream builds the VM manager on this object at V2. Size L because it adds a kernel object type, Capability rights and MemoryObject guest memory; a later split would separate the object, vCPU Operations and guest-memory backing.

<!-- covers: INV-1328 -->

#### Out of scope
VM manager Component (VIRT-008). Host-access grants UI (VIRT-005). virtio-gpu (VIRT-011). Linux `/dev/kvm` for the personality (LNX).

#### Acceptance criteria
- [ ] A native Component holding `Capability<VirtualMachine, …>` can create, start and destroy a VM without opening a device node.
- [ ] Guest memory is a MemoryObject; transferring it is ownership transfer, not a copy of payload pages.
- [ ] A Component without the Capability receives `Error::Rights` and allocates no handle.
- [ ] `os inspect` shows the VirtualMachine, its Capabilities and its ResourceDomain.
- [ ] No `unsafe` outside the KVM-wrapper files listed in the native subsystem.

#### Verification
- Unit: `kernel:tests/kvm/object_*` on `qemu-x86_64`.
- Integration: create/start/destroy plus MemoryObject guest memory on `qemu-x86_64` nested virt.
- Review: ABI lead sign-off that no POSIX-shaped entry was added to Layer 1.

#### Evidence
- none

### KRN-038 · Enforce signature Verification for modules and driver firmware under Secure Boot
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: KRN-027, KRN-034
- Baseline: §51, §63
- Threats: T-007, T-008, T-037

Kernel lockdown, module signature enforcement and verified firmware loading so the chain of trust does not end at the kernel image. V3 Secure Boot on all Tier 1 machines depends on this enforcement existing.

<!-- covers: GAP-0182, GAP-0184 -->

#### Out of scope
Shim and MOK enrolment UX (BOOT-044). Developer-mode key enrolment (BOOT-022). NVIDIA module policy (HW-018).

#### Acceptance criteria
- [ ] With Secure Boot on, an unsigned module is rejected and the load allocates no handle.
- [ ] With Secure Boot on, unsigned firmware requested by a driver is rejected.
- [ ] Lockdown refuses unsigned kexec and raw physical-memory access in the configuration named by KRN-034.
- [ ] Tests cover the accepted signing option from KRN-027 (project key, MOK, or taint-on-local).

#### Verification
- Integration: Secure Boot on in OVMF on `qemu-x86_64`; unsigned module and firmware rejected.
- Manual: repeat the unsigned-module rejection on H-002 with enrolled keys.
- Review: SEC lead accepts the lockdown configuration.

#### Evidence
- none

### KRN-039 · Publish the subsystem maintenance ownership map for retained Linux code
- Type: docs
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: KRN-017
- Baseline: §56.4

Assigns an owner to every inherited subsystem in the retained-mechanism inventory so CVE triage tasks and merge conflicts route to a person. Owners are handles, not `@agent/` identities.

<!-- covers: INV-1097 -->

#### Out of scope
CVE ingest routing implementation (KRN-030). Workstream Lead fields in this repository.

#### Acceptance criteria
- [ ] Every subsystem in the retained-mechanism inventory has a named owner handle.
- [ ] The map is committed in the kernel tree and consumed by the CVE ingest pipeline.
- [ ] Unmapped subsystems fail a CI check on inventory updates.

#### Verification
- Review: kernel architecture lead sign-off recorded on the pull request.
- Unit: CI check that inventory entries have owners.

#### Evidence
- none

### KRN-040 · Rebase or adapt one upstream stable release during V1 per the divergence policy
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: KRN-020, KRN-009, KRN-014, LNX-041
- Baseline: §6, §56.4
- Corpora: C-001
- Risks: R-029

V1 exit: at least one upstream stable release is rebased or adapted during V1 following the divergence policy, with C-001 still green. Includes the milestone review of the retained-mechanism list.

<!-- covers: INV-1337, INV-0140 -->

#### Out of scope
Phase D entry Decision (KRN-042). Merge bot (KRN-023).

#### Acceptance criteria
- [ ] One upstream stable tag is rebased or adapted onto the fork during V1 per the tracking Decision.
- [ ] C-001 on H-001 and H-002 has zero regressions versus the pre-rebase fork.
- [ ] The retained-mechanism inventory has a V1 review note.

#### Verification
- Integration: boot matrix on `qemu-x86_64` and `hw-h002`.
- Compat: C-001 on H-001 and H-002.
- Review: kernel architecture lead accepts the rebase and the inventory review.

#### Evidence
- none

### KRN-041 · Retain watchdog, softlockup, hung-Task and NMI detection and wire them to supervision
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: KRN-011, SVC-015
- Baseline: §32, §55

Daily-driving needs lockups surfaced. Hardware watchdog, softlockup, hung-task and NMI detectors stay enabled and emit typed events to the SVC supervisor and the OBS crash-capture path. SVC-033 pets the hardware watchdog and escalates; this task is the kernel side.

<!-- covers: EXTRA-012 -->

#### Out of scope
Supervisor escalation and reboot into the previous SystemGeneration (SVC-033). Crash artifact persistence (OBS-027). pstore/kdump (KRN-048).

#### Acceptance criteria
- [ ] Hardware watchdog, softlockup, hung-task and NMI detection are enabled on the H-001, H-002 and H-004 fragments.
- [ ] Each detector emits a typed event consumed by the supervisor interface; a unit test fires a synthetic hung-task and observes the event.
- [ ] Native software has no `/proc/sys/kernel/hung_task*` ambient interface.

#### Verification
- Unit: `kernel:tests/watchdog/events_*` on `qemu-x86_64`.
- Integration: synthetic hung-task reaches SVC-015 in a QEMU guest.
- Review: SVC lead confirms the event schema.

#### Evidence
- none

### KRN-042 · Decide Phase D entry: when full upstream merges stop being mandatory
- Type: adr
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: KRN-009, KRN-022, KRN-021, KRN-040
- Baseline: §6, §56.4
- Decision: D-0164
- Risks: R-069

Applies the measurable triggers from the divergence policy to merge-bot and delta-report data and decides whether controlled divergence begins. The Phase D playbooks depend on it. Entering Phase D does not mean dropping security-fix adaptation.

<!-- covers: INV-0148, GAP-0050 -->

#### Out of scope
Driver-adaptation playbook (KRN-047). CVE backports into diverged code (KRN-045). 1.0 phase requirement (KRN-050).

#### Acceptance criteria
- [ ] Options evaluated include entering Phase D at V2, remaining in Phase C with mandatory merges, and deferring Phase D past V2 with named triggers.
- [ ] The accepted option cites merge-bot conflict data and the rebase-cost report against the policy triggers.
- [ ] The accepted option states that security-fix adaptation continues regardless of merge policy.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: kernel architecture lead sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- none

### KRN-043 · Publish the architecture maintenance plan with build-only ARM64 and RISC-V CI
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: KRN-011, BLD-030
- Baseline: §1, §38, §56.4
- Risks: R-083
- Invariants: I-001, I-011, I-012

x86-64 is the only shipped 1.0 target, but the fork must not break ARM64 or RISC-V. Build-only per-architecture CI enforces that, and the plan documents what a future port needs. BLD already compiles ARM64; this task adds RISC-V build-only CI and the written plan.

<!-- covers: INV-1098, INV-0031, INV-0032 -->

#### Out of scope
ARM64 userspace cross-build (BLD-030). Shipping ARM64 or RISC-V as platforms (LATER).

#### Acceptance criteria
- [ ] CI compiles the kernel for ARM64 and RISC-V on every merge to the fork's default branch and fails on breakage.
- [ ] Neither architecture is required to boot for 1.0.
- [ ] A committed plan lists what a future port would still need (firmware, bring-up, LAB) without making those 1.0 work.

#### Verification
- Integration: ARM64 and RISC-V build-only jobs in post-merge CI.
- Review: kernel architecture lead accepts the port plan.

#### Evidence
- none

### KRN-044 · Prune the kernel attack Surface for the three target machines
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: KRN-011, KRN-017, KRN-034, KRN-049, LNX-015, LNX-061, LNX-049, LNX-055
- Baseline: §51, §62

§62 constrains hardware to three named machines, so unused legacy drivers, obsolete filesystems and unneeded protocols can be disabled. The pruned config must keep every facility the retained inventory and LNX's V1 Decisions (seccomp, user namespaces, overlayfs, ptrace, inotify, ia32) require. Native software still does not see those facilities as native APIs.

<!-- covers: GAP-0186 -->

#### Out of scope
Personality sandbox implementation (LNX-061). ia32 Decision (LNX-015). Hardening symbols (KRN-034).

#### Acceptance criteria
- [ ] Fragments for H-002, H-004 and H-005 disable legacy drivers, filesystems and protocols not named by the retained-mechanism inventory or by the LNX V1 Decision set.
- [ ] CI fails if pruning disables a symbol required by the inventory or by LNX-061.
- [ ] Retained-subsystem matrix and C-001 stay green on H-001 and H-002 after pruning.

#### Verification
- Unit: kconfig required-symbol check including LNX sandbox symbols.
- Integration: matrix and boot on `qemu-x86_64`, `hw-h002`, and the V2 laptop fragments.
- Compat: C-001 on H-001 and H-002.

#### Evidence
- none

### KRN-045 · Extend the CVE pipeline to backport fixes into diverged subsystems
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: KRN-030, KRN-042, KRN-032
- Baseline: §6, §56.4
- Risks: R-062, R-069

Phase D security fixes need patch-porting automation and applicability checks against diverged or removed code so the V3 and 1.0 CVE-response gates remain reachable. Upstream patches are adapted, not blindly merged.

<!-- covers: INV-0150 -->

#### Out of scope
V3 response-time publication (KRN-052). REL advisory feed (REL-044).

#### Acceptance criteria
- [ ] An applicable CVE against a diverged subsystem produces a backport branch with conflict markers grouped by file.
- [ ] Removed subsystems are tagged not-applicable and do not open a backport.
- [ ] A successful backport runs the retained-matrix subset for the affected subsystem on `qemu-x86_64`.

#### Verification
- Integration: fixture diverged tree plus fixture CVE produce a backport branch and a not-applicable tag.
- Review: kernel architecture lead accepts the not-applicable rules.

#### Evidence
- none

### KRN-046 · Write the hardware enablement process for new devices after divergence
- Type: docs
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: KRN-042, KRN-032
- Baseline: §55, §56.4, §62

V3 Tier 2 machines and the compatibility database require a process for enabling new devices when upstream merges are no longer mandatory. HW owns SKU bring-up; this document is the kernel-side adaptation path.

<!-- covers: INV-1099 -->

#### Out of scope
HCL publication (REL). SKU selection (HW-003). Driver-adaptation tooling (KRN-032).

#### Acceptance criteria
- [ ] The document names how a new device is enabled after Phase D: inventory update, fragment, kselftest, HCL probe handoff.
- [ ] The document states that hardware support is not broken without an accepted Decision (I-054).
- [ ] The document is committed in the kernel tree and cited by HW bring-up tasks.

#### Verification
- Review: kernel architecture lead and HW lead sign-off recorded on the pull request.

#### Evidence
- none

### KRN-047 · Write the Phase D playbook for adapting upstream driver improvements
- Type: docs
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: KRN-042, KRN-032
- Baseline: §6, §55, §56.4

Once merges are no longer mandatory, driver improvements are adapted rather than merged. The playbook codifies selection, adaptation and regression-matrix steps on the V1 tooling.

<!-- covers: INV-0149 -->

#### Out of scope
Tooling implementation (KRN-032). Architecture-improvement adoption (KRN-051). Selective mm/scheduler adoption (KRN-053).

#### Acceptance criteria
- [ ] The playbook names selection criteria, adaptation steps and which matrix entries must be green before a driver adapt lands.
- [ ] The playbook applies only if KRN-042 accepted Phase D; otherwise it records that mandatory merges continue.
- [ ] The playbook is committed in the kernel tree.

#### Verification
- Review: kernel architecture lead sign-off recorded on the pull request.

#### Evidence
- none

### KRN-048 · Capture kernel panics to pstore/kdump and hand them to the crash reporter
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: KRN-041, OBS-027, OBS-029
- Baseline: §24, §56.4
- Invariants: I-077

Kernel panics are the highest-severity regressions. Persistent capture via pstore/kdump and hand-off after the next successful boot feed the V3 crash-reporting pipeline and panic-rate telemetry. OBS owns format and symbolication; KRN owns kernel persistence.

<!-- covers: GAP-0363 -->

#### Out of scope
Crash-report client and consent (INS). Dedup dashboard (REL-023). Panic-rate fleet dashboard (OBS-053).

#### Acceptance criteria
- [ ] A panic on `qemu-x86_64` is present in pstore or the kdump capture after reboot.
- [ ] After the next successful boot the capture is handed to the OBS crash-artifact path.
- [ ] Captures do not contain disk keys or unlocked secrets (I-077).

#### Verification
- Integration: induced panic, reboot, capture present, hand-off observed on `qemu-x86_64`.
- Manual: repeat on H-002.
- Review: OBS lead confirms the hand-off schema.

#### Evidence
- none

### KRN-049 · Add kernel config fragments for the Intel and AMD target laptops
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: KRN-011, KRN-034
- Baseline: §55, §62

V2 hardware scope adds two laptops. Each gets a fragment in the build-and-boot check so laptop-specific drivers cannot drift out of the config. Hardening symbols stay required.

<!-- covers: GAP-0097 -->

#### Out of scope
Laptop bring-up (HW). Suspend/resume policy (PWR). Wi-Fi (NET).

#### Acceptance criteria
- [ ] Checked-in fragments exist for H-004 and H-005.
- [ ] CI builds and boots each fragment on the corresponding lab or QEMU stand-in named by BLD.
- [ ] Hardening required-symbols remain enabled.

#### Verification
- Integration: build-and-boot jobs for the H-004 and H-005 fragments.
- Unit: hardening required-symbol check.

#### Evidence
- none

### KRN-050 · Decide which kernel evolution phase is required at 1.0
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: KRN-009, LNX-003
- Baseline: §6
- Decision: D-0155

Which of phases C, D or E is a hard 1.0 requirement is KRN divergence policy, not GOV legal. It is decided at V1, after the personality depth Decision and before the V2 syscall-translation work, so that work is built against the phase 1.0 requires rather than judged afterwards. The divergence policy defines the phases; KRN-059 is the 1.0 measurement. This Decision picks the gate and answers Q-003.

<!-- covers: INV-0155 -->

#### Out of scope
Phase E readiness report (KRN-059). GOV 1.0 governance bundle (GOV-080).

#### Acceptance criteria
- [ ] Options evaluated include Phase C wrappers still allowed at 1.0, Phase D controlled divergence required, and Phase E independent native ABI required.
- [ ] The accepted option is the 1.0 kernel-evolution gate cited by KRN-059.
- [ ] Q-003 is marked answered by this task.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: kernel architecture lead and GOV maintainer sign-off recorded on the pull request that accepts the Decision file.

#### Evidence
- none

### KRN-051 · Establish the process for adopting upstream architecture improvements
- Type: docs
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: KRN-042, KRN-029
- Baseline: §6, §56.4

Phase D source of architecture improvements (x86 mitigations, new CPU features) needs an adoption process with the mitigation benchmark as its acceptance check. Blind merges of arch code are out.

<!-- covers: INV-0151 -->

#### Out of scope
B-040 harness (KRN-029). Selective mm/scheduler adoption (KRN-053).

#### Acceptance criteria
- [ ] The process names how an upstream architecture improvement is selected, adapted and validated.
- [ ] Landing an improvement requires a B-040 report on H-002 comparing before and after.
- [ ] The process is committed in the kernel tree.

#### Verification
- Review: kernel architecture lead sign-off recorded on the pull request.
- Bench: B-040 cited as the acceptance check; no number in this document.

#### Evidence
- none

### KRN-052 · Exercise kernel CVE response and publish the V3 response-time distribution
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: KRN-030, KRN-031, KRN-045, REL-034
- Baseline: §56.4, §63
- Risks: R-062

V3 exit: inherited High and Critical CVEs are fixed in a shipped generation with the distribution published against the register's V3 window, and the process is exercised via at least two advisories. REL publishes; KRN produces the kernel fixes and the kernel-side timestamps.

<!-- covers: INV-1095, GAP-0349 -->

#### Out of scope
Public postmortems and advisory feed (REL-034, REL-044). 1.0 SLA report (KRN-057).

#### Acceptance criteria
- [ ] At least two advisories (real or simulated) complete the kernel triage-backport-test handoff.
- [ ] Inherited High and Critical kernel CVEs in the V3 window have kernel-side timestamps consumed by REL-018.
- [ ] The retained-matrix subset for each fix is green on H-001 before the generation ships.

#### Verification
- Integration: two drill advisories through ingest, backport and matrix on `qemu-x86_64`.
- Bench: REL-018 reads this task's timestamps; target per register.
- Review: REL security-response owner accepts the handoff.

#### Evidence
- none

### KRN-053 · Establish selective adoption of upstream mm, scheduler and subsystem patches
- Type: docs
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: KRN-042, KRN-047
- Baseline: §6, §56.4

Defines how algorithms, scheduler ideas, memory-management improvements and subsystem patches are evaluated and adapted rather than blindly merged, with benchmark evidence required per adoption. Driver adaptation is a separate playbook.

<!-- covers: INV-0152, INV-0153 -->

#### Out of scope
Driver playbook (KRN-047). Architecture improvements (KRN-051). Native mm rewrite (MEM). Native scheduler rewrite (SCH).

#### Acceptance criteria
- [ ] The document names evaluation criteria for mm, scheduler and other subsystem patches, including a required B-ID report per adoption.
- [ ] Blind merge of those classes is forbidden once Phase D has been entered.
- [ ] The document is committed in the kernel tree.

#### Verification
- Review: kernel architecture lead, MEM lead and SCH lead sign-off recorded on the pull request.

#### Evidence
- none

### KRN-054 · Remediate external audit findings in kernel-resident native code
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: KRN-013, SEC-070
- Baseline: §51
- Risks: R-055

V4 exit: external security audit of kernel Capability enforcement and IPC with all High and Critical findings fixed and re-verified by the auditor. KRN owns the kernel-side fixes. SEC owns commissioning and the evidence pack; CAP and IPC own findings in their code.

#### Out of scope
Audit commissioning (SEC-070). Auditor re-verification letter (SEC-069). Capability-enforcement findings in CAP (CAP-050). Personality findings (LNX, WIN).

#### Acceptance criteria
- [ ] Every High and Critical finding whose root is kernel-resident native code in the KRN tree has a fix merged.
- [ ] Each such fix has a regression test that fails without the fix.
- [ ] Medium findings in the KRN tree are triaged with public tracking or fixed.

#### Verification
- Unit: regression tests for each High and Critical KRN finding on `qemu-x86_64`.
- Review: auditor re-verification recorded by SEC-069; kernel architecture lead accepts the KRN subset.

#### Evidence
- none

### KRN-055 · Draft the 1.x kernel base and support-window policy
- Type: docs
- Milestone: V4
- Status: todo
- Size: S
- Owner: none
- Depends on: KRN-005, KRN-050, REL-053, GOV-075
- Baseline: §56.4
- Risks: R-061

V4 support-policy drafts include LTS branch policy with KRN as co-owner. This document names which upstream LTS underpins 1.x and how stable backports flow for the published support window. Cadence and EOL communication remain REL-053.

#### Out of scope
Published support contract (GOV-083). 1.x branch creation (KRN-058). CVE SLA numbers (registers, REL-060).

#### Acceptance criteria
- [ ] The draft names the upstream LTS series that underpins 1.x and the backport path for stable fixes.
- [ ] The draft is RFC-reviewed with GOV-075 and REL-053.
- [ ] No calendar date appears; the window is the support duration named in the REL Decision.

#### Verification
- Review: kernel architecture lead, REL and GOV sign-off recorded on the RFC.

#### Evidence
- none

### KRN-056 · Publish the kernel unsafe-code inventory with a justification per block
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: KRN-013, KRN-016
- Baseline: §51
- Invariants: I-082

V4 exit requires the unsafe-code inventory published with justification per block and unsafe authority minimized per §51. Generated from the native subsystem tree and gated in CI. SEC-071 publishes the security evidence; this task produces the kernel inventory.

<!-- covers: INV-0934, INV-0935 -->

#### Out of scope
Userspace unsafe inventory (SDK, BLD). Evidence pack (SEC-071).

#### Acceptance criteria
- [ ] A generated inventory lists every `unsafe` block in the native subsystem with a justification comment that CI requires.
- [ ] CI fails a new `unsafe` block without a justification.
- [ ] The inventory is an input to SEC-071.

#### Verification
- Unit: generator and CI lint fixtures for justified and unjustified `unsafe`.
- Review: kernel architecture lead accepts remaining `unsafe` blocks.

#### Evidence
- none

### KRN-057 · Publish the 90-day kernel CVE SLA compliance report for 1.0
- Type: docs
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: KRN-052, KRN-008, REL-060
- Baseline: §56.4
- Risks: R-062

1.0 exit: trailing CVE response meets the published SLA for the required fraction of applicable CVEs, with the distribution published, and the ledger shows zero unclassified patches at the gate. REL-064 is the release-side publication; this report is the kernel-side evidence.

<!-- covers: INV-1095, GAP-0055 -->

#### Out of scope
SLA target definition (REL-060). Advisory feed (REL).

#### Acceptance criteria
- [ ] A committed report covers the trailing window named by 1.0-G04 and lists applicable kernel CVEs, response times and the SLA verdict.
- [ ] The divergence ledger shows zero unclassified patches at the 1.0 gate.
- [ ] Percentages and windows are taken from the register and the gate; they are not restated as new targets.

#### Verification
- Review: kernel architecture lead and REL security-response owner sign-off recorded on the report pull request.

#### Evidence
- none

### KRN-058 · Create the 1.x kernel maintenance branch with backport rules
- Type: build
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: KRN-055, KRN-030
- Baseline: §56.4

1.0 scope: the 1.x maintenance branch and backport policy. The kernel branch pins the LTS base from the V4 policy and wires the CVE pipeline to it. GOV owns process; KRN owns the branch.

#### Out of scope
Support-window contract (GOV-083). Phase E RFC (KRN-059).

#### Acceptance criteria
- [ ] A 1.x kernel maintenance branch exists and is pinned to the LTS series named by KRN-055.
- [ ] Backport rules are committed and the CVE ingest pipeline files applicable items against this branch.
- [ ] The branch is not rewritten; tags on it are append-only.

#### Verification
- Integration: ingest files a fixture CVE against the 1.x branch.
- Review: kernel architecture lead accepts the backport rules.

#### Evidence
- none

### KRN-059 · Assess Phase E readiness and open the 2.0 kernel divergence RFC
- Type: docs
- Milestone: 1.0
- Status: todo
- Size: M
- Owner: none
- Depends on: KRN-050, KRN-013, KRN-040, KRN-042
- Baseline: §6

Measures the kernel against the §6 Phase E definition (independent native ABI, execution, security, IPC and system semantics) and lists deferred divergence items in the 2.0 planning RFC required by the 1.0 exit. Whether Phase E is required at 1.0 is already decided by KRN-050; this task measures against that Decision.

<!-- covers: INV-0154, INV-0155 -->

#### Out of scope
1.0 phase requirement Decision (KRN-050). ABI fossilization review (ABI-054).

#### Acceptance criteria
- [ ] A committed assessment maps the running kernel to phases C, D and E against the §6 definitions.
- [ ] Deferred divergence items are listed in a 2.0 kernel RFC opened in the project RFC process.
- [ ] The assessment states whether the 1.0 phase requirement Decision is met.

#### Verification
- Review: kernel architecture lead and GOV RFC editor sign-off recorded on the assessment and RFC.

#### Evidence
- none

### KRN-060 · Enforce fallible allocation and no-panic rules in native kernel Rust code
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: KRN-013, KRN-016, BLD-013
- Baseline: §50, §51
- Risks: R-002
- Invariants: I-082, I-101

A Rust panic inside the kernel is a kernel crash, and an infallible allocation in kernel context is a panic waiting for memory pressure. This task makes I-101 mechanical for the native subsystem: `scripts/jakeos/no-panic-lint.py` runs in `pre-merge` over `jakeos/` and `rust/kernel/jakeos/` and fails on `unwrap()`, `expect()`, `panic!`, `unreachable!`, `todo!`, slice or array indexing with `[]` (use `get`), arithmetic on integers without `checked_`, `wrapping_` or `saturating_` (or an explicit overflow comment), and any allocation that is not the fallible kernel form (`KBox::new(..., GFP_KERNEL)?`, `KVec::push(..., GFP_KERNEL)?`), outside modules listed in `Documentation/jakeos/no-panic-allowlist.toml`. Each allowlist entry names a module path, a `reason` and a `// SAFETY:`-style justification of why the panic or infallible allocation cannot be reached from user space; boot-time initialisation modules are the expected entries.

Every allocation on a user-reachable path returns a typed error (`Error::Exhausted` from the ABI-009 vocabulary) on failure. The `nightly` fault-injection run `alloc-fault` uses the kernel's `failslab` and `fail_page_alloc` under the guest agent to fail allocations during Channel, Capability and MemoryObject creation and asserts typed errors reach user space with no panic. The rule is recorded beside the rewrite policy and cited by the ABI-006 review-gate checklist.

#### Out of scope
Rust-first and rewrite-requires-ADR lint (KRN-016). Unsafe inventory publication (KRN-056). Userspace runtime mitigations (SDK-048). The fault-injection harness plumbing (BLD-026 for sanitizers; this task adds only the `alloc-fault` job).

#### Deliverables
- kernel:scripts/jakeos/no-panic-lint.py · The lint over the native subsystem with the pattern list above and allowlist lookup.
- kernel:Documentation/jakeos/no-panic-allowlist.toml · Allowlisted modules with `path`, `reason`, `justification`.
- kernel:Documentation/jakeos/no-panic.md · The rule, the pattern list, the allowlist format, and the fallible allocation idioms to use.
- kernel:.github/workflows/pre-merge.yml · The `no-panic` job.
- kernel:.github/workflows/nightly.yml · The `alloc-fault` job running fault injection under the BLD-006 agent.
- kernel:tools/testing/selftests/jakeos/mem/alloc_fault_*.sh · Fault-injection scenarios for Channel, Capability and MemoryObject creation.
- kernel:tools/jakeos/lint-fixtures/no-panic/ · Fixtures: `unwrap`, panicking index, unchecked arithmetic, infallible allocation, allowlisted boot module.

#### Acceptance criteria
- [ ] `pre-merge` on `qemu-x86_64` fails a native-subsystem change that uses `unwrap`, `expect`, `panic!`, a panicking index, unchecked arithmetic or an infallible allocation outside a module listed in `no-panic-allowlist.toml`.
- [ ] The `alloc-fault` nightly job fails allocations during Channel, Capability and MemoryObject creation and every failure surfaces to user space as a typed exhaustion error with no kernel panic in the serial log.
- [ ] Every `no-panic-allowlist.toml` entry carries `path`, `reason` and a `justification` stating why the panic or infallible allocation is unreachable from user space; an entry without one fails the lint.
- [ ] `Documentation/jakeos/no-panic.md` is linked from `rewrite-policy.md` and named in the ABI-006 review-gate checklist.

#### Verification
- Unit: `kernel:tests/lint/no_panic_*` over the five fixtures.
- Integration: the `alloc-fault` job on `qemu-x86_64` with no panic across the three creation paths.
- Review: kernel architecture lead and SEC lead sign off on the allowlist format on the pull request.

#### Evidence
- none
