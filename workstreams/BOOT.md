# BOOT · Boot and firmware
- Prefix: BOOT
- Lead: none
- Baseline: §5.1, §9.1, §26, §27, §30, §31, §51, §54, §57, §59, §63
- Baseline gap: No dedicated boot or firmware section; UEFI, bootloader, generation selection, measured boot and the boot counter are implied by §5.1 and §30 but unspecified.

<!-- roadmap:generated:begin summary -->
Tasks: 49 live, 2 done, 0 in-progress, 47 todo, 0 dropped. Ready: 2. Blocked: 45. Weighted: 1%.
<!-- roadmap:generated:end -->

## Scope
UEFI firmware path, bootloader, on-disk boot-entry contract, SystemGeneration selection and last-known-good fallback, ESP layout and atomic bootloader updates, Secure Boot and measured boot, early-boot location of the content-addressed store, boot-time integrity of the selected generation, pre-boot disk unlock UI, and coexistence with foreign bootloaders. V0 boots the forked kernel through a retained initramfs and starts native Components beside Linux init. Native init is owned by SVC and arrives at V0.5.

## Out of scope
Kernel fork, config fragments and retained-subsystem policy (KRN). Native init, supervision and boot-success reporting from init (SVC). SystemGeneration composition, store layout and `os generation` (PKG). Installer, recovery environment and dual-boot partitioning (INS). Firmware update service and LVFS (HW). Disk-key sealing, TPM unseal and FDE policy (SEC). Signing-key hierarchy, shim-review submission and SBAT numbers (REL). Lab racking and power-cycle fixtures (LAB). Shell indicator chrome (APP). Benchmark register ownership (BEN).

## Tasks

### BOOT-001 · Boot the forked kernel on QEMU/KVM under OVMF from a CI-built image
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: BOOT-003, BOOT-004, BLD-006, BLD-009, BLD-012, KRN-010, KRN-011
- Baseline: §5.1, §59
- Risks: R-010
- Invariants: I-079

Boot the forked kernel on H-001 from a CI-built UEFI image so V0 has a reproducible QEMU/OVMF path. §5.1 preserves boot; the image is the tagged-commit artifact BLD produces, not a hand-rolled firmware tree.

<!-- covers: INV-1154 -->

#### Out of scope
Physical Reference machine boot (BOOT-002). Native Component launch beside init (BOOT-005). QEMU matrix definition (BLD-012).

#### Acceptance criteria
- [ ] A tagged CI image boots to a serial login prompt on H-001 under OVMF with Secure Boot enrollment disabled for this rung.
- [ ] Repeating the boot from the same tagged commit on H-001 yields the same kernel version string and no firmware fallback to a BIOS image.
- [ ] A missing OVMF firmware blob fails the job before QEMU starts, with the failure mapped to this task id by the guest test agent.
- [ ] CI matrix entry `qemu-x86_64` records boot-complete over serial and rejects a CSM-enabled machine type.

#### Verification
- Integration: `kernel:tests/boot/qemu_ovmf_boot` on CI matrix entry `qemu-x86_64` (H-001).
- Manual: rebuild the tagged image with BLD's one-command path and confirm OVMF handoff on a clean QEMU invocation.

#### Evidence
- none

### BOOT-002 · Boot the forked kernel on the reference AMD desktop from the tagged CI image
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: BOOT-001, HW-003, LAB-003
- Baseline: §5.1, §59
- Invariants: I-074, I-079

Take the same tagged CI image that boots on H-001 and boot it on H-002 so V0 and §59 have one physical Reference machine. The task covers UEFI firmware quirks, serial console and LAB power-cycle automation, not a second firmware architecture.

<!-- covers: INV-1154 -->

#### Out of scope
QEMU/OVMF CI boot (BOOT-001). SKU selection (HW-003). Rack and console (LAB-003).

#### Acceptance criteria
- [ ] H-002 boots the tagged CI image to a serial prompt under UEFI with CSM disabled in firmware setup.
- [ ] LAB remote power-off and power-on of H-002 completes a cold boot that reaches the same serial prompt.
- [ ] The boot log on H-002 names the OVMF-equivalent vendor firmware and records TPM 2.0 presence without requiring Secure Boot on for this rung.
- [ ] A BIOS-only boot target is absent from the H-002 job definition.

#### Verification
- Integration: `kernel:tests/boot/hw_h002_uefi` on CI matrix entry `hw-h002`.
- Manual: LAB operator power-cycles H-002 from the rack PDU and captures serial through the documented console.

#### Evidence
- none

### BOOT-003 · Decide UEFI-only boot on x86-64 with no legacy BIOS/CSM support through 1.0
- Type: adr
- Milestone: V0
- Status: done
- Size: S
- Owner: @agent/claude
- Depends on: none
- Baseline: none
- Decision: D-0049
- Threats: T-008
- Invariants: I-079
- Verified by: @jakebarnby

BASELINE.md has no boot or firmware section. This workstream owns the UEFI path, bootloader, ESP, generation selection, measured boot, boot counting and early-boot integrity that §5.1 and §30 imply but do not specify. This first adr records whether x86-64 boot through 1.0 is UEFI-only, so later bootloader, installer and lab work never grow a BIOS matrix.

<!-- covers: GAP-0166, GAP-0478 -->

#### Out of scope
Bootloader family (BOOT-008). Secure Boot distribution (BOOT-031). ARM64 or RISC-V boot (I-001, I-011, I-012).

#### Acceptance criteria
- [x] The decision file evaluates at least UEFI-only through 1.0, UEFI plus CSM, and UEFI now with a BIOS revisit at V3.
- [x] The accepted option states whether a BIOS or CSM image, test job or installer path may exist before 1.0.
- [x] I-079 is cited as the standing rule the accepted option either enforces or explicitly rejects.
- [x] A Review line names the reviewer who accepts the decision on the pull request.

#### Verification
- Review: GOV maintainer sign-off recorded on the pull request, confirming at least two options and I-079 treatment.

#### Evidence
- decision:D-0049

### BOOT-004 · Decide V0 boots Linux init from a retained initramfs with native Components beside it
- Type: adr
- Milestone: V0
- Status: done
- Size: S
- Owner: @agent/claude
- Depends on: BOOT-003
- Baseline: §5.1, §59
- Decision: D-0051
- Risks: R-010
- Invariants: I-094
- Verified by: @jakebarnby

V0 exit text that demands a native init conflicts with §5.1 Phase A, which preserves boot. This adr fixes V0 as retained Linux boot and initramfs handoff, with native Components started beside Linux init, and defers native init to V0.5 (SVC).

<!-- covers: EXTRA-046, INV-0120 -->

#### Out of scope
Native init implementation (SVC-007). Early-userspace design at V0.5 (SVC-003). Component object model (CMP).

#### Acceptance criteria
- [x] The decision file evaluates at least retained initramfs plus Linux init, native init at V0, and a hybrid stub init.
- [x] The accepted option states whether V0 CI images include a Linux init in the retained initramfs and where native Components are launched from.
- [x] Native init is named as SVC work at V0.5, not as a V0 BOOT deliverable.
- [x] A Review line names the reviewer who accepts the decision on the pull request.

#### Verification
- Review: KRN and SVC lead sign-off recorded on the pull request, confirming Phase A preservation.

#### Evidence
- decision:D-0051

### BOOT-005 · Retain the Linux initramfs handoff and launch native Components beside Linux init
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: BOOT-001, BOOT-004, CMP-003, CMP-014, BLD-082
- Baseline: §5.1, §59
- Risks: R-010
- Invariants: I-094

Implement the accepted V0 boot strategy so the V0 demos start from one retained initramfs: native Components A and B, and the Linux busybox plus L0 corpus, run side by side. Linux init remains the first userspace; native Components are launched beside it from mapped initramfs objects, not from a native init.

<!-- covers: INV-0120, EXTRA-046 -->

#### Out of scope
Native init (SVC-007). Package-backed launch (CMP-027). L0 corpus pass rate (LNX).

#### Acceptance criteria
- [ ] The V0 CI image on H-001 reaches Linux init from the retained initramfs and then starts two native Components whose identities appear in `os inspect component`.
- [ ] A Linux busybox shell and the L0 corpus jobs run on the same booted kernel as those Components.
- [ ] No native Component is granted a POSIX namespace, file descriptor or Linux syscall as its native ABI; personality paths stay inside LNX.
- [ ] Tearing down the two native Components leaves Linux init running.

#### Verification
- Integration: `runtime:tests/boot/initramfs_native_launch` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Demo: Component A/B Channel round trip started from the retained initramfs on H-001, shown beside busybox.

#### Evidence
- none

### BOOT-006 · Build the harness measuring reboot-into-previous-Generation time and publish results
- Type: benchmark
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: BOOT-014, Q-001
- Baseline: §30, §54
- Benchmarks: B-022
- Invariants: I-061

B-022 covers generation creation, switch and rollback. PKG owns creation and switch; this harness publishes the boot-menu rollback segment: wall time from confirmed previous-generation selection in the bootloader to the previous generation accepting a serial handshake. Numbers live only in the register and reports. Required by V0.5-G16 (Compositor restart-to-rebound published).

#### Out of scope
Generation compose and switch timing (PKG-002, PKG-001). Register ownership (BEN). Methodology (Q-001).

#### Acceptance criteria
- [ ] Harness `bench:generation-switch` records a boot-menu rollback interval on H-001 and H-002.
- [ ] A V0.5 report exists under `reports/benchmarks/B-022/` for H-001 and H-002 meeting the register's publish target.
- [ ] The report names Linux ostree/nixos rollback as baselines and contains no superiority claim.

#### Verification
- Bench: B-022 on H-001 and H-002; target per register.
- Review: BEN maintainer confirms the boot-menu segment is labeled separately from PKG switch.

#### Evidence
- none

### BOOT-007 · Define the on-disk boot-entry format mapping a menu entry to a SystemGeneration
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: BOOT-008, BOOT-011, PKG-019, PKG-008, PKG-029
- Baseline: §27, §30
- Risks: R-018
- Invariants: I-022, I-080

Fix the contract between the bootloader and the generation store: one menu entry names a generation ID, kernel or UKI image, verified store root hash and signed manifest. A Rust parser/writer crate and a golden-file test keep BOOT and PKG from drifting as either side evolves.

<!-- covers: GAP-0169 -->

#### Out of scope
Store layout (PKG-014). ESP space policy (BOOT-013). Signature verification at boot (BOOT-027).

#### Acceptance criteria
- [ ] The crate round-trips a golden boot entry containing generation ID, image path or UKI, store root hash and manifest digest.
- [ ] A mutated hash or truncated record fails parse with a typed error and is not offered as a menu entry.
- [ ] PKG generation compose writes the same format the bootloader parser accepts, proven by a golden-file test in CI.
- [ ] Reserved signature fields from PKG-029 are present and ignored by the V0.5 parser rather than rejected.

#### Verification
- Unit: `runtime:tests/boot/entry_format_*` on CI matrix entry `qemu-x86_64`.
- Integration: PKG compose of generation N writes an entry that the bootloader lists on H-001.

#### Evidence
- none

### BOOT-008 · Decide the bootloader: systemd-boot, GRUB or a native Rust UEFI stub
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: BOOT-010, BOOT-003, BOOT-015
- Baseline: §2, §30, §57
- Decision: D-0042
- Risks: R-018
- Threats: T-008
- Invariants: I-009

Generation selection, boot counting and PCR policy all hang off the bootloader. §57 forbids rewriting a mature loader without demonstrated benefit; the spike report is the evidence this adr must cite. V0.5 exit requires an accepted decision for the generation switching mechanism.

<!-- covers: GAP-0167, GAP-0528 -->

#### Out of scope
UKI versus separate parts (BOOT-011). GPLv3 Installation Information (BOOT-010). ESP layout (BOOT-029).

#### Acceptance criteria
- [ ] The decision file evaluates at least systemd-boot, GRUB and a native Rust UEFI stub, citing the spike report for generation selection, Secure Boot chain, PCR measurement and ESP footprint.
- [ ] The accepted option is compatible with the GPLv3 boot-chain decision.
- [ ] A rejected rewrite, if any, names the missing benefit under §57 and I-009.
- [ ] A Review line names the reviewer who accepts the decision on the pull request.

#### Verification
- Review: BOOT and GOV lead sign-off recorded on the pull request, with the spike report attached.

#### Evidence
- none

### BOOT-009 · Decide how early boot locates the content store and the selected SystemGeneration
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: BOOT-008, BOOT-011, PKG-014, SVC-003
- Baseline: §27, §30
- Decision: D-0043
- Threats: T-008

Generation booting only works if early boot finds immutable objects before trusting mutable state. This adr picks the locator: initramfs-embedded, kernel command line, or bootloader-passed manifest, paired with SVC's early-userspace decision.

<!-- covers: GAP-0529 -->

#### Out of scope
Mount and handoff implementation (BOOT-012). Native init (SVC-007). Store GC (PKG).

#### Acceptance criteria
- [ ] The decision file evaluates at least an initramfs-embedded locator, kernel command line parameters, and a bootloader-passed manifest.
- [ ] The accepted option states what is trusted before the store is mounted and how a missing or mismatched generation ID fails closed.
- [ ] The option is consistent with the UKI and bootloader decisions.
- [ ] A Review line names the reviewer who accepts the decision on the pull request.

#### Verification
- Review: BOOT, PKG and SVC lead sign-off recorded on the pull request.

#### Evidence
- none

### BOOT-010 · Decide whether GPLv3 components may appear in the boot chain and how Installation Information is met
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: BOOT-015, GOV-003
- Baseline: none
- Decision: D-0047
- Threats: T-007, T-008

Secure Boot plus GPLv3 (GRUB, gnupg) triggers anti-tivoization duties. This adr decides whether GPLv3 may appear in the boot chain and how Installation Information is met if it does. Base-image GPLv3 (coreutils, bash) remains GOV licensing policy.

<!-- covers: GAP-0020 -->

#### Out of scope
Bootloader family choice (BOOT-008). Userspace license firewall (GOV-003). Shim review (BOOT-036).

#### Acceptance criteria
- [ ] The decision file evaluates at least no GPLv3 in the boot chain, GPLv3 permitted with user-enrollable Secure Boot keys as Installation Information, and GPLv3 only in developer-mode images.
- [ ] The accepted option states which boot-chain binaries may be GPLv3 and what Installation Information the project ships if any are.
- [ ] The option cites GOV-003 rather than restating Layer 1 versus Layer 2 license policy.
- [ ] A Review line names the reviewer who accepts the decision on the pull request.

#### Verification
- Review: GOV and BOOT lead sign-off recorded on the pull request.

#### Evidence
- none

### BOOT-011 · Decide whether each SystemGeneration boots as one signed UKI or separately verified parts
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: BOOT-003, BOOT-015
- Baseline: §30
- Decision: D-0050
- Threats: T-008, T-022

The image format must be fixed before PKG store layout and the boot-entry format harden. A single signed UKI makes Secure Boot signing units and PCR prediction tractable; separately verified parts change both.

<!-- covers: GAP-0168 -->

#### Out of scope
Bootloader family (BOOT-008). Command-line editor policy (BOOT-019). Signing keys (REL-002).

#### Acceptance criteria
- [ ] The decision file evaluates at least a single signed UKI, kernel plus separately verified initramfs and command line, and a UKI with detached add-ons.
- [ ] The accepted option names the signed unit the bootloader verifies and what is included in PCR-extend inputs.
- [ ] The option states whether the kernel command line is inside the signed unit at V0.5.
- [ ] The accepted option states how early CPU microcode is carried in the signed unit and loaded before native init starts.
- [ ] A Review line names the reviewer who accepts the decision on the pull request.

#### Verification
- Review: BOOT and REL lead sign-off recorded on the pull request, citing the spike's PCR notes.

#### Evidence
- none

### BOOT-012 · Attach the content store read-only in early boot and hand off to native init
- Type: build
- Milestone: V0.5
- Status: todo
- Size: L
- Owner: none
- Depends on: BOOT-007, BOOT-009, PKG-016, PKG-014, SVC-003
- Baseline: §27, §30
- Invariants: I-022

Implement the early-boot locator: find the store and the selected SystemGeneration, mount the store read-only, and hand off to SVC native init so the cold-boot-to-native-desktop demo starts from a generation rather than an ad-hoc root.

<!-- covers: GAP-0529 -->

#### Out of scope
Native init and supervisor (SVC-007). Store object GC (PKG-067). Unlock UI (BOOT-026).

#### Acceptance criteria
- [ ] Early boot on H-001 locates the selected generation from the boot entry and mounts its store read-only before SVC init runs.
- [ ] A writable open of a store object from early boot returns a typed error and does not mutate the store.
- [ ] A missing generation ID or mismatched store root hash fails closed and does not hand off to init.
- [ ] After handoff, `os inspect` on the session host shows Components started from the selected generation's pinned objects.

#### Verification
- Integration: `runtime:tests/boot/early_store_mount` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Demo: cold boot to the V0.5 native desktop from a SystemGeneration on H-002.

#### Evidence
- none

### BOOT-013 · Maintain ESP space so at least three previous SystemGeneration kernels stay bootable
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: BOOT-007, BOOT-008, PKG-016
- Baseline: §30
- Risks: R-018
- Invariants: I-022

V0.5 must keep the previous kernel selectable at boot; 1.0 requires at least three previous SystemGenerations bootable. Per-generation images on the ESP need retention, garbage collection of entries no longer referenced, and a typed out-of-space error instead of a silent overwrite of the last-known-good slot. Required by 1.0-G06 (Update and rollback guarantee verified on every Tier 1 machine).

#### Out of scope
Generation object GC in the store (PKG-052). ESP partition policy (BOOT-029). Atomic bootloader binary updates (BOOT-025).

#### Acceptance criteria
- [ ] Installing generation N+1 on H-001 leaves generation N's boot entry and kernel or UKI intact and selectable.
- [ ] With three retained generations configured, a fourth install either keeps the oldest still-bootable or fails with a typed out-of-space error before deleting last-known-good.
- [ ] Garbage collection removes only entries whose generation IDs are no longer in the retention set, proven by a golden ESP listing test.
- [ ] The previous generation remains bootable after an interrupted install that did not commit N+1.

#### Verification
- Integration: `runtime:tests/boot/esp_retention` on CI matrix entry `qemu-x86_64`.
- Manual: fill a constrained ESP image until the typed out-of-space path fires, then boot the retained previous generation.

#### Evidence
- none

### BOOT-014 · Show previous SystemGenerations in the boot menu with automated boot-menu selection test
- Type: build
- Milestone: V0.5
- Status: todo
- Size: L
- Owner: none
- Depends on: BOOT-007, BOOT-012, BOOT-013, BLD-022, PKG-018, PKG-020, PKG-021, SVC-007
- Baseline: §30, §60
- Risks: R-018
- Invariants: I-022

V0.5 exit: selecting the previous generation at boot restores the previous kernel, compositor and Packages. The bootloader lists boot entries from the BOOT format; firmware or serial-driven selection is automated on H-001 and repeated on H-002.

<!-- covers: INV-0561 -->

#### Out of scope
PKG compose of generation contents (PKG-016). `os restore` (PKG-060). Settings rollback UI (INS-014).

#### Acceptance criteria
- [ ] The boot menu on H-001 lists at least the current and previous SystemGenerations by generation ID.
- [ ] Serial or firmware-driven selection of the previous entry boots that generation's kernel or UKI and the compositor and Packages pinned in it.
- [ ] After rollback, a Package installed only in N+1 is absent, matching the V0.5 demo.
- [ ] The automated boot-menu test passes on H-001 in CI and on H-002 in the lab harness.

#### Verification
- Integration: `runtime:tests/boot/generation_menu_select` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Demo: install a Package, reboot, select the previous generation at the menu, Package gone and previous desktop intact on H-002.

#### Evidence
- none

### BOOT-015 · Explore systemd-boot+UKI and a Rust UEFI stub booting a SystemGeneration from the store
- Type: spike
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: BOOT-003, BOOT-004
- Baseline: §2, §30, §57
- Risks: R-018
- Invariants: I-009

§57 forbids rewriting a mature bootloader without demonstrated benefit. Prototype systemd-boot with a UKI and a Rust UEFI stub, each booting a SystemGeneration from the store on H-001, and measure generation selection, Secure Boot chain, PCR measurement and ESP footprint so BOOT-008 is not a guess.

<!-- covers: GAP-0167, GAP-0528 -->

#### Out of scope
The bootloader decision (BOOT-008). Production ESP retention (BOOT-013). Shim (BOOT-044).

#### Acceptance criteria
- [ ] Both candidates boot a store-backed generation to a serial handshake on H-001 under OVMF.
- [ ] The spike report records generation selection, Secure Boot chain shape, PCR extend inputs and ESP bytes used for each candidate on H-001.
- [ ] The report states whether a native Rust stub is justified under I-009 and §57, with the measured gap named.
- [ ] `reports/spikes/BOOT-015.md` exists with the spike skeleton headings.

#### Verification
- Report: which candidate boots a store-backed generation; comparative generation selection, Secure Boot chain, PCR measurement and ESP footprint; whether a Rust UEFI stub is justified under §57; what remains unmeasured on physical hardware.
- Integration: both prototypes run on H-001 via `runtime:tests/boot/spike_loader_*`.

#### Evidence
- none

### BOOT-016 · Build the boot-to-login timing harness and publish results beside Linux and Windows
- Type: benchmark
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: BOOT-014, HW-015, LAB-007, Q-001, SVC-007
- Baseline: §54, §61
- Benchmarks: B-032
- Invariants: I-061

Stand up harness `bench:boot-time` (B-032) from firmware handoff to the greeter accepting input, and separately to the first presented frame, on H-002 and H-004. V3 publishes per Tier 1 machine; the harness and method exist from V1 so later gates do not invent a metric. Required by V3-G16 (Install, first-boot and update-apply time published).

#### Out of scope
Fleet publication (BEN-048). Unlock-to-desktop after greeter (B-033, APP). Pre-boot disk-unlock interval (BOOT-017).

#### Acceptance criteria
- [ ] The harness records B-032 p50 and p99 over the register's cold-boot count on H-002 and H-004.
- [ ] V1 reports exist under `reports/benchmarks/B-032/` for H-002 and H-004 meeting the register target kind for V1.
- [ ] Linux and, where dual-boot exists, Windows baselines are named in the report with no superiority claim.

#### Verification
- Bench: B-032 on H-002 and H-004; target per register.
- Review: BEN maintainer confirms timestamps come from the bootloader and compositor as the register method requires.

#### Evidence
- none

### BOOT-017 · Build the pre-boot unlock latency harness and publish results
- Type: benchmark
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: BOOT-026, Q-001
- Baseline: §54
- Benchmarks: B-032
- Invariants: I-061

Publish the unlock-submission-to-root-pivot interval as a labeled split of B-032 on H-002 and H-004. The figure is a harness output, not a milestone claim.

#### Out of scope
Greeter-to-desktop (B-033). Disk-encryption throughput (B-038). Unlock UI (BOOT-026).

#### Acceptance criteria
- [ ] The B-032 harness records an unlock-submission-to-root-pivot interval on H-002 and H-004 when a passphrase slot is enrolled.
- [ ] V1 reports under `reports/benchmarks/B-032/` include that labeled interval for both machines.
- [ ] The report contains no absolute latency promise in prose.

#### Verification
- Bench: B-032 unlock-to-pivot split on H-002 and H-004; target per register.
- Integration: `runtime:tests/boot/unlock_pivot_stamp` on CI matrix entries `hw-h002` and `hw-h004`.

#### Evidence
- none

### BOOT-018 · Implement boot counting, bad-Generation marking and unattended fallback to last known-good
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: BOOT-020, BOOT-025, BOOT-014, SVC-007
- Baseline: §30
- Threats: T-022, T-028
- Invariants: I-022

§30 says if boot fails, boot the previous generation, but does not define detection. Store a boot attempt counter on the ESP or in an EFI variable, mark a generation bad after the configured consecutive failure count, and boot last-known-good without a user at the menu. SVC init reports success through a Capability-gated primitive this task defines.

<!-- covers: GAP-0174, INV-0562, GAP-0175 -->

#### Out of scope
What "boot succeeded" means (BOOT-020). Fault-injection matrix on three machines (BOOT-028). Generation health state machine (PKG-073).

#### Acceptance criteria
- [ ] A generation that fails to clear the counter within the configured consecutive attempts is marked bad and is not selected on the next unattended boot.
- [ ] Last-known-good boots without menu interaction on H-001 after that marking.
- [ ] Clearing the counter requires the Capability named by BOOT-020; a Component without it receives `Error::Rights` and the counter is unchanged.
- [ ] Counter storage survives a power cut during increment, proven by a QEMU power-cut test.

#### Verification
- Integration: `runtime:tests/boot/counter_fallback` on CI matrix entry `qemu-x86_64`.
- Unit: `runtime:tests/boot/boot_success_cap_deny` on `qemu-x86_64`.
- Fuzz: `runtime:fuzz/boot_counter` one hour nightly without panic.

#### Evidence
- none

### BOOT-019 · Include the kernel command line in the signed SystemGeneration and test that edits fail Verification
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: BOOT-011, BOOT-022, BOOT-027
- Baseline: §30
- Threats: T-008, T-022
- Invariants: I-079

An editable command line defeats Secure Boot and measured boot. The command line is part of the signed generation. The boot-time editor is disabled outside developer mode, and CI proves a modified command line is rejected.

<!-- covers: GAP-0177 -->

#### Out of scope
Developer-mode enrollment (BOOT-022). UKI format decision (BOOT-011).

#### Acceptance criteria
- [ ] A boot entry whose command line bytes do not match the signed generation fails verification and is not booted on H-001.
- [ ] Outside developer mode the bootloader offers no command-line editor, proven by a firmware-automated menu dump.
- [ ] In developer mode a modified command line boots only after the developer-mode indicator path is active.
- [ ] I-079's command-line rule is enforced by this test, not by documentation alone.

#### Verification
- Integration: `runtime:tests/boot/cmdline_signature_reject` on CI matrix entry `qemu-x86_64`.
- Review: SEC reviewer confirms the reject path is fail-closed.

#### Evidence
- none

### BOOT-020 · Decide what 'boot succeeded' means and which Component may clear the boot counter
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: BOOT-014, SVC-007
- Baseline: §30, §32
- Decision: D-0041
- Threats: T-028

Too early a success mark lets a broken desktop count as good; too late a mark makes headless and recovery boots impossible. SVC init is the reporter. This adr picks the event and the Component authorized to clear the counter. Desktop-usable health (display, network, audio) remains INS-006.

<!-- covers: GAP-0175 -->

#### Out of scope
Counter storage and fallback (BOOT-018). Desktop-usable health quorum (INS-006). Safe-mode session (SVC).

#### Acceptance criteria
- [ ] The decision file evaluates at least session manager reached greeter, user authenticated, and a per-boot-mode profile (headless, recovery, desktop).
- [ ] The accepted option names the Component that holds the clear-counter Capability and the event that authorizes the clear.
- [ ] Headless and recovery boots have an explicit success rule, not an implied desktop greeter.
- [ ] A Review line names the reviewer who accepts the decision on the pull request.

#### Verification
- Review: BOOT, SVC and INS lead sign-off recorded on the pull request.

#### Evidence
- none

### BOOT-021 · Decide the trusted time source policy before network time is available
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: SEC-002
- Baseline: none
- Decision: D-0044
- Threats: T-007, T-019

Certificate and signature checks at early boot need a clock-trust policy before SVC time sync exists. Options are a bootloader-persisted monotonic floor, the booted generation's build-timestamp floor, or an RTC trusted with TPM-backed clock attestation.

<!-- covers: GAP-0209 -->

#### Out of scope
Entropy seeding (BOOT-023). NTP/NTS client (SVC-018, SVC-032).

#### Acceptance criteria
- [ ] The decision file evaluates at least a bootloader-persisted monotonic floor, a build-timestamp floor of the booted generation, and an RTC trusted with TPM-backed clock attestation.
- [ ] The accepted option states how a clock behind the floor is treated during signature validity checks.
- [ ] The option names what is stored on the ESP, if anything, and who refreshes it after a successful boot.
- [ ] A Review line names the reviewer who accepts the decision on the pull request.

#### Verification
- Review: BOOT and SEC lead sign-off recorded on the pull request.

#### Evidence
- none

### BOOT-022 · Ship documented developer mode: own Secure Boot keys, self-built kernels, visible indicator
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: BOOT-003, BOOT-027
- Baseline: §61
- Threats: T-008, T-022
- Invariants: I-074

V1 self-hosting requires booting self-built kernels. Developer mode enrolls owner keys, disables lockdown and exposes a non-suppressible state that APP's shell renders as a persistent indicator. This task ships enrollment, boot-chain acceptance of owner-signed kernels, and the enrollment guide.

<!-- covers: GAP-0178 -->

#### Out of scope
Shell indicator chrome (APP). Microsoft shim (BOOT-044). Secure Boot strategy for retail hardware (BOOT-031).

#### Acceptance criteria
- [ ] Enrolling an owner key on H-002 allows a locally built kernel or UKI signed with that key to boot under Secure Boot.
- [ ] While developer mode is active, a firmware or bootloader state bit is visible to the session host; clearing it requires leaving developer mode.
- [ ] A machine not in developer mode rejects the same locally built image at verification.
- [ ] The enrollment guide exists as a committed document and a lab operator can follow it on H-002 without undocumented steps.

#### Verification
- Integration: `runtime:tests/boot/developer_mode_owner_key` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Review: SEC reviewer signs off the enrollment guide on the pull request.
- Manual: enroll, boot a self-built image, confirm the indicator bit, unenroll, confirm reject.

#### Evidence
- none

### BOOT-023 · Initialize the kernel RNG at boot from TPM, CPU RNG and a bootloader-persisted seed
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: BOOT-021, BOOT-025
- Baseline: none
- Threats: T-010

First-boot disk unlock, TLS and signature checks depend on early entropy. The bootloader persists and refreshes a seed on the ESP; the kernel credits TPM and CPU RNG sources before unlock and before native Components start.

<!-- covers: GAP-0209 -->

#### Out of scope
Time-trust policy (BOOT-021). Unlock UI (BOOT-026). Kernel hardening config (KRN).

#### Acceptance criteria
- [ ] Dmesg or `os inspect` on H-001 and H-002 shows credited TPM, CPU RNG and bootloader-seed sources before the first userspace signature check.
- [ ] After a successful boot the ESP seed bytes change, proven by a before/after hash in the integration test.
- [ ] A missing TPM still credits CPU RNG and the persisted seed and does not hang unlock.
- [ ] The seed file is not world-readable from a native Component without a firmware-variable Capability.

#### Verification
- Integration: `kernel:tests/boot/early_entropy_sources` on CI matrix entries `qemu-x86_64` (H-001 with emulated TPM) and `hw-h002`.
- Review: SEC reviewer confirms seed refresh and credit order.

#### Evidence
- none

### BOOT-024 · Guard UEFI variable and firmware-settings access behind a Capability with no ambient efivarfs
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: BOOT-003, CAP-007, SEC-002
- Baseline: §9.1
- Threats: T-001, T-008, T-021
- Invariants: I-021

Writable firmware variables can brick firmware or alter Secure Boot state. Native Components start with no ambient firmware-variable access. A typed firmware-variable service, granted as a Capability, replaces ambient efivarfs for native code. Linux-personality views of firmware remain inside LNX.

<!-- covers: GAP-0204 -->

#### Out of scope
Linux efivarfs inside the personality (LNX). Boot-counter EFI variable owned by the bootloader (BOOT-018). Device Capability shape (HW-008).

#### Acceptance criteria
- [ ] A native Component with no firmware-variable Capability cannot read or write UEFI variables; the call returns `Error::Rights` and allocates no handle.
- [ ] The firmware-variable service, holding the Capability, can read a named variable used by tests and cannot write Secure Boot PK without an additional right.
- [ ] No native Component in the V1 session host's default grant set has the write right, proven by an `os inspect` dump test.
- [ ] Ambient efivarfs is not mounted into native Component namespaces.

#### Verification
- Unit: `runtime:tests/boot/efivar_cap_deny` on CI matrix entry `qemu-x86_64`.
- Integration: `runtime:tests/boot/efivar_service_rights` on `qemu-x86_64` and `hw-h002`.
- Review: CAP reviewer confirms the rights word is a subset of S-003 style attenuation.

#### Evidence
- none

### BOOT-025 · Make bootloader and ESP updates atomic with tested rollback of the bootloader itself
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: BOOT-007, BOOT-008, BOOT-013
- Baseline: §30
- Risks: R-018
- Threats: T-028

An interrupted bootloader update is the failure SystemGenerations cannot roll back from. Use dual-slot or write-then-rename on the ESP and prove a power cut during the bootloader write still boots last-known-good on H-001 before the V1 signed-repository update path ships.

<!-- covers: GAP-0176 -->

#### Out of scope
Generation image atomicity (PKG-082). Capsule firmware staging (BOOT-037). Signing (REL).

#### Acceptance criteria
- [ ] A QEMU power-cut injected after the new bootloader bytes start landing still boots the previous bootloader and last-known-good generation.
- [ ] A completed bootloader update is the one used on the next boot, proven by a version string in the firmware log.
- [ ] Updating the bootloader does not delete retained generation entries.
- [ ] The scheme is the one named dual-slot or write-then-rename in the implementation README and matches the test.

#### Verification
- Integration: `runtime:tests/boot/esp_powercut_bootloader` on CI matrix entry `qemu-x86_64`.
- Manual: repeat the power-cut test on H-002 once the QEMU test is green.

#### Evidence
- none

### BOOT-026 · Build the text pre-boot unlock UI with passphrase, TPM+PIN, recovery key and layouts
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: BOOT-012, SEC-005, SEC-017, SEC-018
- Baseline: none
- Threats: T-008, T-010

V1 allows manually configured encryption. Unlock must work before the compositor exists: passphrase, TPM-plus-PIN when SEC exposes the slot, recovery key, keyboard layout selection and multiple enrolled slots, consuming SEC-018.

<!-- covers: GAP-0197 -->

#### Out of scope
Graphical DPI-aware unlock (BOOT-035). Installer FDE UX (INS, SEC-055). Early-boot verity setup (SVC-023). Key-slot crypto (SEC).

#### Acceptance criteria
- [ ] On H-001 and H-002, a passphrase slot unlocks the volume and early boot continues to store mount.
- [ ] A recovery-key slot unlocks after three failed passphrase attempts, with the remaining-attempt count shown in the text UI.
- [ ] Keyboard layout can be selected before the passphrase is entered; a named layout test types a non-US character into the secret.
- [ ] A wrong passphrase does not pivot root and does not print the secret to serial.
- [ ] Multiple enrolled slots are listed; selecting slot 2 unlocks when slot 1 is empty.

#### Verification
- Integration: `runtime:tests/boot/preboot_unlock_text` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Manual: enroll passphrase and recovery key on H-004 and complete unlock at the text UI.

#### Evidence
- none

### BOOT-027 · Verify the release-key signature of every SystemGeneration manifest before offering it
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: BOOT-007, PKG-029, REL-002
- Baseline: §27, §30
- Threats: T-006, T-007, T-028
- Invariants: I-080

The immutable generation model needs a root of trust. The bootloader verifies the release-key signature on each SystemGeneration manifest before listing or booting it. Tampered manifests are not offered. Key hierarchy is REL's decision.

<!-- covers: GAP-0172 -->

#### Out of scope
Key hierarchy and custody (REL-002). Package repository client (PKG-064). Measured PCR policy (BOOT-034).

#### Acceptance criteria
- [ ] A generation whose manifest signature does not verify is absent from the boot menu on H-001.
- [ ] A generation with a valid release-key signature is listed and boots.
- [ ] Truncating or swapping the signature field fails closed with a typed verify error in the bootloader log.
- [ ] Developer-mode owner keys are accepted only when developer mode is active.

#### Verification
- Integration: `runtime:tests/boot/manifest_signature_verify` on CI matrix entry `qemu-x86_64`.
- Review: REL reviewer confirms the verified key is the channel key named by REL-002.

#### Evidence
- none

### BOOT-028 · Simulate a broken SystemGeneration and verify unattended fallback on the three target machines
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: BOOT-018, LAB-018, PKG-073, SVC-033
- Baseline: §30, §62
- Benchmarks: B-035
- Invariants: I-022

V2 exit: a fault-injected broken update boots the previous generation without a user, on H-002, H-004 and H-005. Reuse the LAB power-cycle harness for V3 and 1.0 rollback gates. B-035 rollback time is recorded from failure detection to the previous generation's greeter.

<!-- covers: INV-0562, GAP-0174 -->

#### Out of scope
Counter implementation (BOOT-018). 1.0 fleet guarantee (BOOT-049). Delta size (BEN).

#### Acceptance criteria
- [ ] Injecting a broken generation N+1 on H-002, H-004 and H-005 results in an unattended boot of N on the subsequent automatic retry.
- [ ] The boot menu still lists N as selectable after fallback.
- [ ] The LAB power-cycle harness used here is the same job definition later rungs cite.
- [ ] A B-035 rollback-time sample is produced on each of the three machines.

#### Verification
- Integration: `runtime:tests/boot/fault_fallback` on CI matrix entries `hw-h002`, `hw-h004` and `hw-h005`.
- Bench: B-035 rollback interval on H-002, H-004 and H-005; target per register.
- Demo: fault-injected broken update on H-002 returns to the previous desktop without a keypress.

#### Evidence
- none

### BOOT-029 · Decide ESP policy: reuse the existing OEM ESP or create a dedicated ESP for Generation entries
- Type: adr
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: BOOT-008, BOOT-013
- Baseline: none
- Decision: D-0045
- Risks: R-018
- Threats: T-008

V2 laptops ship Windows-created undersized OEM ESPs. V3 dual-boot (INS) depends on the layout. Decide reuse with compact entries, a dedicated ESP, or reuse plus an XBOOTLDR-style extended partition.

<!-- covers: GAP-0400 -->

#### Out of scope
Installer partitioning (INS-026). Foreign bootloader chain-load (BOOT-041). Retention counts (BOOT-013).

#### Acceptance criteria
- [ ] The decision file evaluates at least reuse of the existing ESP with compact entries, a dedicated ESP, and reuse with an XBOOTLDR-style extended partition.
- [ ] The accepted option states how an undersized OEM ESP is handled so at least three generations remain bootable or install refuses with a typed error.
- [ ] Dual-boot with Windows Boot Manager is named as a constraint, not a non-goal.
- [ ] A Review line names the reviewer who accepts the decision on the pull request.

#### Verification
- Review: BOOT and INS lead sign-off recorded on the pull request.

#### Evidence
- none

### BOOT-030 · Decide atomicity of kernel, driver and firmware updates within a SystemGeneration
- Type: adr
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: BOOT-020, PKG-019, SEC-002
- Baseline: §30, §31
- Decision: D-0046
- Threats: T-021
- Invariants: I-022

§30 rollback conflicts with firmware that cannot be rolled back. This adr defines what a generation guarantees and what §31 history records as irreversible. HW owns the firmware update service; BOOT owns the boot-chain consequences. Answers Q-024.

<!-- covers: INV-0567 -->

#### Out of scope
LVFS service (HW-046). Capsule staging (BOOT-037). PCR re-seal (SEC-051).

#### Acceptance criteria
- [ ] The decision file evaluates at least firmware outside generations, firmware staged with the generation and flagged irreversible, and firmware applied only after boot-success.
- [ ] The accepted option names what `os restore` of generation N may undo and what history events are irreversible.
- [ ] Q-024 is answerable from the accepted option without a second decision.
- [ ] A Review line names the reviewer who accepts the decision on the pull request.

#### Verification
- Review: BOOT, HW and PKG lead sign-off recorded on the pull request.

#### Evidence
- none

### BOOT-031 · Decide Secure Boot distribution: Microsoft-signed shim, enrolled project keys, or both
- Type: adr
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: BOOT-008, BOOT-010, BOOT-011, BOOT-022
- Baseline: none
- Decision: D-0048
- Risks: R-047
- Threats: T-008, T-021
- Invariants: I-074

Most retail hardware ships only Microsoft keys. Dual-boot machines must keep Windows Secure Boot and BitLocker functional. V3 requires Secure Boot on every Tier 1 machine, so the distribution strategy is fixed at V2: shim plus MOK, user-enrolled PK/KEK/db, or both.

<!-- covers: GAP-0179, GAP-0326, GAP-0403 -->

#### Out of scope
Shim-review plan (BOOT-036). Shim integration (BOOT-044). NVIDIA module signing (HW, R-050).

#### Acceptance criteria
- [ ] The decision file evaluates at least Microsoft-signed shim plus MOK, user-enrolled project PK/KEK/db, and both, including dual-boot BitLocker impact.
- [ ] The accepted option names the documented path V3 install docs will teach.
- [ ] Developer mode remains possible under the accepted option.
- [ ] A Review line names the reviewer who accepts the decision on the pull request.

#### Verification
- Review: BOOT, SEC, INS and GOV lead sign-off recorded on the pull request.

#### Evidence
- none

### BOOT-032 · Verify SystemGeneration images at boot against their content hashes via the boot-entry root
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: BOOT-007, BOOT-027, STO-052
- Baseline: §26, §27, §30
- Threats: T-006, T-008
- Invariants: I-022

§26 integrity verification and §27 content addressing: the verified store root hash in the boot entry drives generation-image verification at boot. A tampered image fails closed. The mechanism is the mature Linux verity path; native software does not receive a block device as its ABI.

<!-- covers: INV-0569 -->

#### Out of scope
Integrity-failure UX (BOOT-033). Store verify command (STO-045). PCR extends (BOOT-034).

#### Acceptance criteria
- [ ] A generation whose image bytes do not match the boot-entry root hash does not pivot root on H-001.
- [ ] An unmodified signed generation boots and reaches init.
- [ ] The bootloader log names the root hash that was checked.
- [ ] No native Component is granted a raw verity device node; personality access stays in LNX.

#### Verification
- Integration: `runtime:tests/boot/generation_verity` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Review: STO reviewer confirms the hash is the store root PKG and STO publish.

#### Evidence
- none

### BOOT-033 · Design and build the integrity-failure experience with a guaranteed recovery path
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: BOOT-032, INS-013
- Baseline: §32, §63
- Threats: T-008, T-021

Verity, PCR or signature failures after firmware changes are inevitable. The user sees a diagnosis, can enter the signed recovery boot entry, and no path silently bypasses verification. INS owns the recovery environment; this task owns the boot-chain failure screen and the handoff.

<!-- covers: GAP-0206 -->

#### Out of scope
Recovery environment contents (INS-041, INS-013). Safe-mode session (SVC-036). Audit of findings (BOOT-046).

#### Acceptance criteria
- [ ] A verity mismatch on H-002 shows a text diagnosis that names verity and offers the recovery entry, with no continue-anyway control.
- [ ] Selecting recovery boots the signed recovery SystemGeneration from the ESP.
- [ ] A signature mismatch and a PCR policy mismatch each have a distinct diagnosis string in the test catalog.
- [ ] Automated firmware selection of the recovery entry succeeds on H-001.

#### Verification
- Integration: `runtime:tests/boot/integrity_failure_paths` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Manual: trigger a PCR mismatch on H-004 after a firmware setting change and enter recovery.

#### Evidence
- none

### BOOT-034 · Extend kernel, command line, Generation ID and verity root into TPM PCRs with signed PCR policies
- Type: build
- Milestone: V2
- Status: todo
- Size: L
- Owner: none
- Depends on: BOOT-031, BOOT-011, BOOT-032, SEC-050, SEC-053
- Baseline: §51
- Threats: T-008, T-010, T-021, T-022
- Invariants: I-074

Without a predictable PCR policy every update strands sealed disk keys. The bootloader extends kernel, command line, generation ID and verity root into TPM PCRs and ships signed expected PCR values per generation so SEC re-seal and the TPM service keep unlock working across updates.

<!-- covers: GAP-0191 -->

#### Out of scope
Re-seal implementation (SEC-051). TPM Capability service (SEC-053). Event-log fleet verify (BOOT-043).

#### Acceptance criteria
- [ ] After a successful boot of generation N on H-002, the TPM event log contains extends for kernel or UKI, command line, generation ID and verity root.
- [ ] A signed PCR policy for N is present in the generation and matches the event log, proven by a policy-verify test.
- [ ] Booting N+1 with its own signed policy does not require the N policy to remain valid.
- [ ] A Component without the TPM Capability cannot read PCR values through a native API.

#### Verification
- Integration: `runtime:tests/boot/pcr_policy_match` on CI matrix entries `qemu-x86_64` (emulated TPM) and `hw-h002`.
- Review: SEC reviewer confirms the extend order matches the seal policy SEC-052 consumes.

#### Evidence
- none

### BOOT-035 · Build the graphical DPI-aware pre-boot unlock UI
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: BOOT-026, SEC-052
- Baseline: §62
- Threats: T-010

V2 polished desktop on three target machines needs graphical, DPI-aware unlock including HiDPI laptop panels. Same slots as the text UI: passphrase, TPM-plus-PIN, recovery key, layouts. Runs before the compositor; it is not the APP lock screen.

<!-- covers: GAP-0197 -->

#### Out of scope
Text unlock (BOOT-026). Session lock (APP-033). Compositor (GFX).

#### Acceptance criteria
- [ ] Graphical unlock presents on H-002, H-004 and H-005 at the panel's native pixel size without compositor startup.
- [ ] Passphrase, TPM-plus-PIN and recovery-key slots each unlock on at least one of those machines.
- [ ] The HiDPI laptop panel (H-004 or H-005 as equipped) shows glyphs at a readable scale in a captured framebuffer test.
- [ ] Failure paths match the text UI: no continue-anyway, no secret on serial.

#### Verification
- Integration: `runtime:tests/boot/preboot_unlock_graphical` on CI matrix entries `hw-h002`, `hw-h004` and `hw-h005`.
- Manual: photograph or capture the HiDPI unlock screen on the Intel laptop and attach to the pull request.

#### Evidence
- none

### BOOT-036 · Plan the shim-review submission, legal applicant and re-signing process
- Type: docs
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: BOOT-031, GOV-024
- Baseline: none
- Risks: R-047
- Threats: T-007

If the Secure Boot strategy includes a Microsoft-signed shim, V3 retail hardware needs an external review. This plan names the applicant entity, review-board requirements, SBAT and re-signing per shim change so REL can run the submission (REL-040) on the V3 rung. If shim is rejected, the plan records the no-op path.

<!-- covers: GAP-0021 -->

#### Out of scope
Actual shim-review filing (REL-040). Legal entity form (GOV-024). Boot-chain wiring (BOOT-044).

#### Acceptance criteria
- [ ] A committed plan names the applicant, the review-board artifacts required, and the re-signing trigger per shim change, or records that shim is not used.
- [ ] SBAT ownership is assigned to REL-040 in the plan.
- [ ] Dual-boot BitLocker impact of MOK enrollment is described.
- [ ] A Review line names GOV and REL reviewers.

#### Verification
- Review: GOV and REL sign-off recorded on the pull request.

#### Evidence
- none

### BOOT-037 · Stage UEFI capsule updates on the ESP within a Generation transition with PCR re-seal hooks
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: BOOT-029, BOOT-030, BOOT-034
- Baseline: §30, §31
- Threats: T-021

Boot-chain side of HW's firmware update service: capsules are staged on the ESP, applied by firmware at reboot, and the boot chain triggers SEC re-seal before and after, implementing BOOT-030.

<!-- covers: INV-0567 -->

#### Out of scope
LVFS client and device firmware (HW-046). History event schema (PKG). Re-seal crypto (SEC-051).

#### Acceptance criteria
- [ ] A staged capsule on H-002 is applied on reboot and the new firmware version appears in the boot log.
- [ ] SEC re-seal hooks run before the capsule is applied and after the subsequent successful boot, proven by hook trace events.
- [ ] An interrupted capsule apply leaves the previous generation bootable and records an irreversible-firmware history flag when the accepted decision requires it.
- [ ] Capsule files live in the ESP layout named by BOOT-029.

#### Verification
- Integration: `runtime:tests/boot/capsule_stage_reseal` on CI matrix entry `hw-h002`.
- Review: HW and SEC reviewers confirm hook order against the atomicity decision.

#### Evidence
- none

### BOOT-038 · Enforce the security-fix watermark in the bootloader per the anti-rollback Decision
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: BOOT-040, BOOT-027
- Baseline: §30
- Threats: T-022
- Invariants: I-022

The signed manifest carries a watermark. The bootloader compares it against a monotonic counter. Behavior follows BOOT-040: block below watermark, allow with warning, or allow only in developer mode, with a test for each branch the decision keeps.

<!-- covers: GAP-0207 -->

#### Out of scope
Policy decision (BOOT-040). Key rotation (BOOT-047). PKG advisory matching (PKG-081).

#### Acceptance criteria
- [ ] A generation whose watermark is below the stored counter is treated exactly as the accepted decision specifies, proven by a test per live branch.
- [ ] A generation at or above the counter boots when otherwise valid.
- [ ] The counter never decreases on a successful boot of a higher watermark, proven by a monotonicity test.
- [ ] Developer mode, if the decision uses it as an escape, is the only escape and is visible.

#### Verification
- Integration: `runtime:tests/boot/anti_rollback_watermark` on CI matrix entry `qemu-x86_64`.
- Unit: `runtime:tests/boot/watermark_monotonic` on `qemu-x86_64`.

#### Evidence
- none

### BOOT-039 · Expose SystemGenerations to foreign bootloaders via BLS entries and an os-release descriptor
- Type: build
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: BOOT-007, BOOT-008, BOOT-029
- Baseline: none
- Invariants: I-025

Coexistence is bidirectional. os-prober and systemd-boot on a co-installed distro must list JakeOS generations. Native software does not use BLS or os-release as its ABI; these files exist for foreign loaders.

<!-- covers: GAP-0402 -->

#### Out of scope
Chain-load and BootOrder repair (BOOT-041). Installer dual-boot (INS-026). Native generation menu (BOOT-014).

#### Acceptance criteria
- [ ] Each retained SystemGeneration has a BLS snippet a systemd-boot on a sibling ESP volume can parse in the golden-file test.
- [ ] An os-release-compatible descriptor names JakeOS and the running generation ID.
- [ ] Native Components are not granted a path Capability to those files by default.
- [ ] Updating generations refreshes BLS snippets without breaking the native boot-entry format.

#### Verification
- Unit: `runtime:tests/boot/bls_os_release_golden` on CI matrix entry `qemu-x86_64`.
- Compat: dual-boot fixture with a foreign systemd-boot that lists JakeOS generations on H-002.

#### Evidence
- none

### BOOT-040 · Decide the anti-rollback policy for SystemGenerations older than a security watermark
- Type: adr
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: BOOT-018, BOOT-027, SEC-002
- Baseline: §30
- Decision: D-0040
- Threats: T-022

§30 first-class rollback and downgrade-attack protection are in tension. V3 public alpha exposes rollback to strangers. Decide whether booting generations older than a security-fix watermark is blocked, allowed with warning, or allowed only in developer mode.

<!-- covers: GAP-0207 -->

#### Out of scope
Watermark implementation (BOOT-038). Unsigned kexec (KRN lockdown). Advisory publication (REL).

#### Acceptance criteria
- [ ] The decision file evaluates at least block below watermark, allow with warning, and allow only in developer mode.
- [ ] The accepted option states how a user still reaches last-known-good when last-known-good is older than the watermark, or that they cannot.
- [ ] T-022 is cited and either mitigated or accepted in the Consequences section.
- [ ] A Review line names the reviewer who accepts the decision on the pull request.

#### Verification
- Review: BOOT, SEC and GOV lead sign-off recorded on the pull request.

#### Evidence
- none

### BOOT-041 · Load Windows Boot Manager and foreign Linux loaders and self-heal BootOrder
- Type: build
- Milestone: V3
- Status: todo
- Size: L
- Owner: none
- Depends on: BOOT-039, BOOT-029, BOOT-031, INS-026
- Baseline: none
- Risks: R-047
- Threats: T-008

V3 installer supports dual boot. Windows updates rewrite BootOrder and fallback bootx64.efi. The boot chain must chain-load Windows Boot Manager and foreign Linux loaders and repair its own entries on the next JakeOS boot.

<!-- covers: GAP-0401 -->

#### Out of scope
Installer preserve-ESP and BitLocker warning (INS-026). BLS publication (BOOT-039). NTFS policy (STO).

#### Acceptance criteria
- [ ] On a dual-boot H-002 image, the JakeOS bootloader chain-loads Windows Boot Manager and returns after a Windows boot.
- [ ] Overwriting BootOrder and bootx64.efi to Windows-only is repaired on the next JakeOS boot from the remaining ESP entry or fallback.
- [ ] A foreign Linux loader listed at install remains chain-loadable.
- [ ] Repair does not disable Windows Secure Boot or require BitLocker recovery when the Secure Boot strategy forbids that.

#### Verification
- Integration: `runtime:tests/boot/bootorder_self_heal` on a dual-boot QEMU image and on H-002.
- Manual: apply a Windows update in the dual-boot fixture, confirm JakeOS still boots on the subsequent restart.

#### Evidence
- none

### BOOT-042 · Document the Secure Boot key or shim path, developer mode and recovery for public alpha users
- Type: docs
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: BOOT-031, BOOT-022, BOOT-033, BOOT-036
- Baseline: §63
- Risks: R-047

V3 exit requires Secure Boot using the documented key or shim path, and public install documentation good enough for strangers. Cover key enrollment, MOK prompts, developer mode and integrity-failure recovery.

<!-- covers: GAP-0178, GAP-0206 -->

#### Out of scope
Installer copy (INS). Administrator guide chapters owned by DOC. Shim wiring (BOOT-044).

#### Acceptance criteria
- [ ] A committed guide describes the documented Secure Boot path matching BOOT-031, including MOK or project-key enrollment.
- [ ] Developer mode enrollment and the visible indicator are documented with a warning that lockdown is off.
- [ ] Integrity-failure recovery points at the recovery generation and states that bypass is not offered.
- [ ] A Review line names DOC and SEC reviewers.

#### Verification
- Review: DOC and SEC sign-off recorded on the pull request.
- Manual: a lab operator who did not write the guide follows it on one Tier 1 machine and files the result on the pull request.

#### Evidence
- none

### BOOT-043 · Verify Secure Boot and measured-boot event-log recording on every Tier 1 machine
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: BOOT-034, BOOT-044, LAB-022, BOOT-019
- Baseline: §63
- Threats: T-008
- Invariants: I-074

V3 exit: every V3 Tier 1 machine boots with Secure Boot on and measured boot records generation identity in the TPM event log. The matrix extends to ten machines at V4 and is re-run each RC; this task covers the V3 six-machine set.

<!-- covers: GAP-0191, GAP-0326 -->

#### Out of scope
Shim integration (BOOT-044). HCL publication (HW, REL). V4 ten-machine soak (LAB-024).

#### Acceptance criteria
- [ ] H-002, H-004, H-005, H-006, H-007 and H-008 each boot a signed generation with Secure Boot enabled.
- [ ] Each machine's TPM event log contains the generation ID extend from BOOT-034.
- [ ] A machine with Secure Boot off fails this task's job, not the unsigned-boot developer path.
- [ ] Results are attached per H-ID in the pull request.

#### Verification
- Integration: `runtime:tests/boot/secure_boot_eventlog` on CI matrix entries `hw-h002`, `hw-h004`, `hw-h005`, `hw-h006`, `hw-h007` and `hw-h008`.
- Review: LAB operator confirms firmware Secure Boot state matches the job record.

#### Evidence
- none

### BOOT-044 · Integrate shim and MOK enrollment into the signed boot chain per the Secure Boot strategy
- Type: build
- Milestone: V3
- Status: todo
- Size: L
- Owner: none
- Depends on: BOOT-031, BOOT-022, BOOT-036, KRN-038, REL-040
- Baseline: none
- Risks: R-047
- Threats: T-007, T-008
- Invariants: I-074

V3 exit: Tier 1 machines boot with Secure Boot enabled using the documented shim or key path. Wire shim, MOK enrollment of the project certificate and SBAT metadata into the bootloader and UKI build. If the strategy rejected shim, this task ships the enrolled-key path only and records the unused shim branch as unbuilt.

<!-- covers: GAP-0179, GAP-0326, GAP-0021 -->

#### Out of scope
Shim-review filing (REL-040). Module signing policy (KRN-038). Docs (BOOT-042).

#### Acceptance criteria
- [ ] A factory-key QEMU OVMF image with only Microsoft keys boots the signed chain using the documented shim or key path, or the enrolled-key path if shim was rejected.
- [ ] MOK enrollment of the project certificate, when the strategy includes MOK, is a scripted step that then boots without further prompts.
- [ ] SBAT metadata is present in the bootloader or UKI artifact REL-040 versions.
- [ ] Developer mode still boots owner-signed images without the Microsoft path.

#### Verification
- Integration: `runtime:tests/boot/shim_mok_chain` on CI matrix entry `qemu-x86_64` and on H-002.
- Review: REL reviewer confirms SBAT numbers match the REL register of generations.

#### Evidence
- none

### BOOT-045 · Record SystemGeneration identity in the TPM event log
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: BOOT-034, BOOT-031
- Baseline: §9.1, §30, §51

V3 measured boot records SystemGeneration identity in the TPM event log so attestation can name the booted generation (§9.1, §51). PCR extension of kernel, command line and verity root is BOOT-034; this task adds the generation identity event and the inspectable log.

#### Out of scope
PCR coverage (BOOT-034). Inspect CLI for the log (SEC-065). Attestation protocol (SEC-058). Firmware update reseal (SEC-051).

#### Acceptance criteria
- [ ] Booting a SystemGeneration on H-002 appends an event that names that generation's identity in the TPM event log.
- [ ] Rolling back to the previous generation appends a distinct event naming the previous identity.
- [ ] `os inspect` on the attestation service shows the event for the running generation.

#### Verification
- Integration: `boot:tests/tpm/generation_event_log_*` on CI matrix entry `hw-h002`.
- Review: BOOT and SEC leads sign off on the pull request.

#### Evidence
- none

### BOOT-046 · Close all High and Critical external-audit findings against the bootloader chain
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: BOOT-034, BOOT-044, SEC-070
- Baseline: §51, §63
- Threats: T-008, T-021

V4 exit: the external security audit GOV commissions names the bootloader chain. Every High and Critical finding against that chain is fixed and re-verified by the auditor. This task is the BOOT fix queue, not the contract.

#### Out of scope
Commissioning the auditor (SEC-070). Capability-kernel and FDE findings (SEC, KRN).

#### Acceptance criteria
- [ ] Every High and Critical bootloader-chain finding from the V4 audit has a linked fix commit and a re-test note in Evidence.
- [ ] The auditor's re-verification statement is attached as an Evidence URL or report path.
- [ ] Open Medium findings are listed with owners; they do not include High or Critical items.
- [ ] Regression tests added for each High/Critical fix run on H-001.

#### Verification
- Review: auditor re-verification recorded on the pull request.
- Integration: new regression tests under `runtime:tests/boot/audit_*` on `qemu-x86_64`.

#### Evidence
- none

### BOOT-047 · Execute signing-key rotation and SBAT revocation through the boot chain on the testing Channel
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: BOOT-038, BOOT-044, REL-041
- Baseline: §63
- Threats: T-007, T-028

V4 exit: a signing-key rotation drill on the testing channel must not break updates. The bootloader accepts rotated db/MOK certificates and revoked SBAT generations while keeping previous generations bootable. REL operates keys; this task is the boot-chain half of the drill.

#### Out of scope
HSM ceremony (REL-041, REL-054). Watermark policy (BOOT-040).

#### Acceptance criteria
- [ ] After rotating the channel certificate, a new generation signed with the rotated key boots on H-002 under Secure Boot.
- [ ] A generation whose SBAT generation is revoked does not boot; the previous unrevoked generation does.
- [ ] At least three previous unrevoked generations remain selectable after the drill.
- [ ] The testing-channel drill does not change stable-channel trust anchors.

#### Verification
- Integration: `runtime:tests/boot/key_rotation_sbat` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Review: REL reviewer confirms the drill used testing-channel keys only.

#### Evidence
- none

### BOOT-048 · Make UKI and bootloader images reproduce bit-for-bit with deterministic signing layout
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: BOOT-008, BOOT-011, BOOT-044
- Baseline: §27, §51
- Threats: T-007
- Invariants: I-082

V4 requires SystemGeneration packages to reproduce on two independent builders. PE assembly and embedded Authenticode signatures need a deterministic layout or detached verification so BLD's verifier can pass on bootloader and UKI artifacts.

#### Out of scope
Full package reproducibility (BLD-077). Independent rebuilders (BLD-074). Key custody (REL).

#### Acceptance criteria
- [ ] Two builds of the same revision produce byte-identical unsigned UKI and bootloader PE images, or a documented detached-signature layout whose payload hashes match.
- [ ] Embedded Authenticode, if used, is applied in a deterministic order recorded in the build graph.
- [ ] A third-party verifier using BLD's instructions reports a match on the boot-chain artifacts.
- [ ] New boot-chain code remains Rust except files the KRN rewrite policy exempts, listed in the pull request.

#### Verification
- Integration: `runtime:tests/boot/repro_uki_bootloader` in CI comparing two builder outputs.
- Review: BLD reviewer confirms the artifacts are in the generation SBOM.

#### Evidence
- none

### BOOT-049 · Verify the update and rollback guarantee by fault injection on every Tier 1 machine
- Type: build
- Milestone: 1.0
- Status: todo
- Size: M
- Owner: none
- Depends on: BOOT-038, BOOT-028, BOOT-013, BOOT-047, INS-056, INS-043, PKG-091, STO-083
- Baseline: §30, §63
- Benchmarks: B-035, B-043
- Invariants: I-022

1.0 update and rollback guarantee: on every Tier 1 machine a fault-injected failing generation boots back automatically, at least three generations remain bootable, and a power pull mid-update recovers. PKG and INS run sibling verifications; this task is the boot-chain half the release-ceremony demo depends on.

<!-- covers: INV-0562, GAP-0174 -->

#### Out of scope
User-data preservation (PKG, STO). Installer recovery UX (INS-041). Fleet success-rate publication (BEN, B-043 register ownership).

#### Acceptance criteria
- [ ] Fault injection of a failing generation on each Tier 1 H-ID in hardware scope boots the previous generation unattended.
- [ ] At least three previous generations remain selectable on each of those machines after the injection.
- [ ] A power pull during ESP update on each of those machines still boots a complete generation.
- [ ] B-035 and B-043 samples for the 1.0 target kind exist per in-scope H-ID.

#### Verification
- Integration: `runtime:tests/boot/rollback_guarantee` on every Tier 1 H-ID in the 1.0 hardware scope.
- Bench: B-035 and B-043 on those H-IDs; target per register.
- Demo: release-ceremony fault injection on H-002 returns to the previous generation without a keypress.

#### Evidence
- none
