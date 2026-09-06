# INS · Installer, updater, recovery, migration
- Prefix: INS
- Lead: none
- Baseline: §9, §25, §30, §31, §54, §57, §60, §61, §62, §63

<!-- roadmap:generated:begin summary -->
Tasks: 56 live, 0 done, 0 in-progress, 56 todo, 0 dropped. Ready: 0. Blocked: 56. Weighted: 0%.
<!-- roadmap:generated:end -->

## Scope
INS owns the SystemGeneration image builder and the scripted install path that V0.5 demos and V1 developers use before any end-user installer exists. It owns first-boot account, encryption-confirm and privacy setup; the guided then graphical installer; the updater client that fetches and applies atomic new SystemGenerations; first-class rollback in Settings and at boot; the signed recovery SystemGeneration and recovery environment; the opt-in crash-report and bug-reporter clients with preview and redaction; dual-boot and foreign-OS coexistence; live USB evaluation; install-media creation; migration import engines; unattended install; and a supported uninstall path. Native software never sees POSIX, Linux or Win32 shapes; Personality views and foreign bootloaders stay in LNX, WIN and BOOT.

## Out of scope
SystemGeneration composition, store layout, `os generation` and `os restore` (PKG). Bootloader, boot menu, boot counter, ESP policy and chain-load (BOOT). Signing keys, repository server, crash intake, notices bundle publication and Hardware Compatibility List hosting (REL). Disk-encryption mechanism, identity, recovery-key slots and secrets (SEC). Partition layout library, NTFS/exFAT drivers, snapshots and backup targets (STO). Settings chrome, migration assistant UI and About panel (APP). Crash capture format and journal export (OBS). Screen reader (ACC). String catalogs and locale data (TXT). Bluetooth host and HCL probe (HW). Native init, safe-mode session and last-resort text console (SVC). JakeOS guest images and virtio guest agents (VIRT). Benchmark register and methodology (BEN). CI image promotion (BLD). Public docs site (DOC). Privacy policy (GOV). Lab racks and power fixtures (LAB). Wi-Fi stack (NET). `os` CLI rendering (SDK).

## Tasks

### INS-001 · Build the SystemGeneration image builder
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: BOOT-008, BOOT-011, PKG-008, PKG-016, PKG-029, REL-001, STO-014, STO-021, SVC-007
- Baseline: §27, §30, §57, §60
- Risks: R-018
- Invariants: I-009, I-020, I-022, I-080

Build the SystemGeneration image builder so V0.5 generation-switch demos and V1 developers have a bootable image from the same compose path PKG uses. The builder consumes PKG generation trees and the STO partition-layout library. It does not invent a native filesystem (§26, §57).

<!-- covers: EXTRA-070, GAP-0375 -->

#### Out of scope
Scripted install into a disk (INS-002). Bootloader binary (BOOT). Store substrate (STO). CI smoke images that are not generation images (BLD-009).

#### Acceptance criteria
- [ ] The builder emits a UEFI disk image whose selected SystemGeneration boots on H-001 to the V0.5 compositor started by SVC-007.
- [ ] Repeating the build from the same tagged commit yields the same generation identity and reserved signing fields.
- [ ] The image uses the STO partition layout and writes no objects outside the content-addressed store and ESP slots the layout names.
- [ ] A missing PKG compose input fails before any disk write, with the failure mapped to this task id.

#### Verification
- Integration: `installer:tests/image_builder_qemu` on CI matrix entry `qemu-x86_64` (H-001).
- Review: PKG and STO leads confirm the builder calls compose and layout libraries rather than a second store.

#### Evidence
- none

### INS-002 · Ship scripted install from a built SystemGeneration image
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: BOOT-007, INS-001, PKG-017, STO-021
- Baseline: §30, §31, §60
- Invariants: I-020, I-022

Ship a non-interactive install that writes a built SystemGeneration image onto a QEMU disk (and the same path on H-002) before any end-user installer exists. User data and ApplicationData stay off the immutable generation tree so later rollback cannot destroy them (§30, PKG-007).

<!-- covers: EXTRA-070 -->

#### Out of scope
Guided disk picker (INS-011). Graphical installer (INS-027). Hardware laptop bring-up (INS-005).

#### Acceptance criteria
- [ ] A CI job installs the image onto a blank GPT disk on H-001 with no TTY prompts and the guest boots the installed generation.
- [ ] After install, `os inspect` on the guest names the installed SystemGeneration and the ESP contains a BOOT boot entry for it.
- [ ] A filesystem diff of the immutable generation tree after creating a user file in ApplicationData shows no mutation of generation objects.
- [ ] Re-running the script on an already-installed disk is idempotent or fails with a typed error before overwriting the generation store.

#### Verification
- Integration: `installer:tests/scripted_install_qemu` on CI matrix entry `qemu-x86_64`.
- Manual: run the same script against a spare NVMe on H-002 and confirm serial boot of the installed generation.

#### Evidence
- none

### INS-003 · Document developer image install, update and rollback
- Type: docs
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: INS-004, INS-002, PKG-020, PKG-060, SDK-044
- Baseline: §30, §31, §61, §64

Write the committed developer procedures for installing from published QEMU and USB media, applying a repository update as a new SystemGeneration, and rolling back with `os restore` and the boot menu. Public install guides stay with DOC at V3.

#### Out of scope
Public install and recovery guides (INS-039, DOC-030). `os` CLI implementation (SDK).

#### Acceptance criteria
- [ ] A committed page lists the exact commands to install the developer image on H-001 and on USB media, apply one repository update, and roll back to the previous generation.
- [ ] A core developer following only that page on a clean checkout reaches a booted installed generation, a second generation after update, and the previous generation after rollback.
- [ ] The page cites PKG and BOOT commands rather than restating store or bootloader internals.

#### Verification
- Review: PKG and BOOT leads sign off on the pull request.
- Manual: a developer not on the INS team follows the page on H-001 and records the three generation IDs.

#### Evidence
- none

### INS-004 · Publish QEMU and USB developer install media
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: BOOT-027, INS-001, INS-002, PKG-055, REL-007
- Baseline: §30, §61
- Threats: T-008
- Invariants: I-080

Publish signed-enough QEMU and USB developer media produced from the V0.5 image builder so daily-driving and self-hosting do not wait for a graphical installer. REL owns production signing later; this rung uses the developer repository keys.

<!-- covers: GAP-0375 -->

#### Out of scope
Host-side USB writing for strangers (INS-035). Channel promotion of identical artifacts (REL-019). Guest hypervisor images (VIRT-001).

#### Acceptance criteria
- [ ] CI publishes a QEMU disk and a USB image from the same builder invocation, each with a checksum and a developer-key signature.
- [ ] Booting the QEMU artifact on H-001 and the USB artifact on H-002 reaches the scripted-install or already-installed generation without unsigned-object warnings.
- [ ] Tampering with a published artifact fails signature verification before the generation is activated.
- [ ] Nightly rebuilds of the same commit reproduce the published generation identity.

#### Verification
- Integration: `installer:tests/developer_media_sign` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Manual: flash the USB image on H-004 and confirm a signed boot of the developer generation.

#### Evidence
- none

### INS-005 · Run scripted install on the Intel laptop and AMD desktop
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-001, HW-015, INS-004, INS-002, LAB-007, LAB-003
- Baseline: §61, §62

Run the scripted image path on H-002 and H-004 so V1 daily-driving is not QEMU-only. The Intel laptop and AMD desktop are the V1 hardware scope; the AMD laptop waits for V2. Required by V1-G02 (Core team daily-drives the OS): daily-driving needs the OS installed on H-002 and H-004, not only under QEMU.

#### Out of scope
Guided disk selection for preview (INS-011). Lab fixtures (LAB). SKU selection (HW-003).

#### Acceptance criteria
- [ ] Scripted install from published USB media completes on H-002 and H-004 and each machine boots the installed SystemGeneration to a login or serial prompt.
- [ ] Repeating the install on a wiped disk on each machine yields a bootable generation whose identity matches the published media.
- [ ] Failure to unlock or find the target disk returns a typed error and leaves foreign partitions untouched.
- [ ] LAB records the two passing jobs against this task id.

#### Verification
- Integration: `installer:tests/scripted_install_hw` on CI matrix entries `hw-h002` and `hw-h004`.
- Manual: LAB operator flashes USB media, runs the script, and captures serial on both machines.

#### Evidence
- none

### INS-006 · Decide desktop-usable boot-success health criteria
- Type: adr
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: BOOT-020, SVC-007, SVC-004
- Baseline: §30, §32, §62
- Decision: D-0132
- Risks: R-018

Decide when a new SystemGeneration is marked good so automatic rollback is not a false success. Distinct from BOOT-020, which names who may clear the boot counter. This adr names the desktop-usable quorum the updater and Settings health state consume.

<!-- covers: GAP-0342 -->

#### Out of scope
Boot-counter storage and last-known-good fallback (BOOT-018). Generation health-state machine (PKG-073). Init readiness protocol (SVC).

#### Acceptance criteria
- [ ] The decision file evaluates at least greeter reached, user authenticated, and a required-service quorum including display, network and audio.
- [ ] The accepted option lists the Components whose readiness is required before the generation is marked good, and which boot modes (desktop, recovery, headless) skip which checks.
- [ ] The accepted option states that BOOT still owns when the counter may clear, and that this decision only names the desktop-usable inputs.
- [ ] A Review line names who accepts the decision on the pull request.

#### Verification
- Review: BOOT, SVC and INS leads sign off on the pull request, confirming no overlap with BOOT-020.

#### Evidence
- none

### INS-007 · Decide installer encryption default with opt-out
- Type: adr
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: SEC-005, SEC-002, STO-039
- Baseline: §9, §63
- Decision: D-0133
- Threats: T-008, T-010
- Invariants: I-073

Decide whether the installer encrypts user data by default. First-boot at V2 confirms encryption and saves the recovery key; the V3 graphical installer enforces the default. SEC owns the mechanism; STO owns layering onto the store.

<!-- covers: GAP-0196 -->

#### Out of scope
LUKS or filesystem-native mechanism (SEC-005). Volume layout (STO-039). Recovery-key slot API (SEC-018).

#### Acceptance criteria
- [ ] The decision file evaluates at least default-on with explicit opt-out, default-on with no opt-out on Tier 1, and default-off.
- [ ] The accepted option states the installer control the user sees and whether a Tier 1 image can boot unencrypted without an explicit opt-out record.
- [ ] I-073 is cited as the standing rule the accepted option either enforces or explicitly rejects.
- [ ] A Review line names who accepts the decision on the pull request.

#### Verification
- Review: SEC and INS leads sign off on the pull request, confirming T-008 and I-073 treatment.

#### Evidence
- none

### INS-008 · Decide installer disk layout, wipe and dual-boot policy
- Type: adr
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: BOOT-029, INS-007, STO-014
- Baseline: §25, §30, §63
- Decision: D-0135
- Risks: R-046
- Threats: T-008

Decide wipe-versus-alongside, ESP reuse versus a new ESP, and where the recovery SystemGeneration lives, so the V3 graphical installer is not inventing disk policy. STO already decided GPT layout and type GUIDs; BOOT already decided ESP policy. This adr is the installer offer.

#### Out of scope
GPT type GUIDs and mkfs (STO-021). ESP reuse mechanics (BOOT-029). Chain-load of foreign bootloaders (BOOT-041).

#### Acceptance criteria
- [ ] The decision file evaluates at least wipe-only, shrink-and-install-alongside as a first-class offer, and refuse-to-install when free space or ESP size cannot hold the recovery generation plus retained kernels.
- [ ] The accepted option names where the signed recovery SystemGeneration is placed (ESP versus dedicated partition) and that BOOT-029 is an input, not a second ESP decision.
- [ ] The accepted option states how an existing Windows, Linux or Intel-Mac install is detected and what the installer refuses to destroy without an explicit wipe confirmation.
- [ ] A Review line names who accepts the decision on the pull request.

#### Verification
- Review: BOOT, STO and INS leads sign off on the pull request.

#### Evidence
- none

### INS-009 · Decide client update orchestration, metered links and deferral
- Type: adr
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: PKG-070, REL-004, SEC-002
- Baseline: §30, §63
- Decision: D-0138
- Risks: R-079
- Threats: T-028, T-034
- Invariants: I-086

Decide background download, apply-on-reboot versus live userspace activation, metered-network behaviour and user deferral. Must precede the V3 updater client. PKG-070 and REL channels are inputs. I-086 keeps SystemGenerations plus reboot as the update model unless the accepted option records a userspace exception.

<!-- covers: GAP-0341 -->

#### Out of scope
Channel promotion criteria (REL-004). Whether a generation switch may avoid reboot (PKG-070). Running-app restart prompt chrome (APP-047).

#### Acceptance criteria
- [ ] The decision file evaluates at least reboot-only apply, live userspace activation with kernel reboot still required for kernel changes, and defer-until-idle with a deadline.
- [ ] The accepted option states metered-link behaviour (download, apply, or neither without consent) and how the user defers without leaving a mixed-version tree (T-034).
- [ ] The accepted option names the Settings and updater surfaces that expose the policy and which PKG/BOOT operations they call.
- [ ] A Review line names who accepts the decision on the pull request.

#### Verification
- Review: PKG, REL and INS leads sign off on the pull request.

#### Evidence
- none

### INS-010 · Build first-boot account, encryption and privacy setup
- Type: build
- Milestone: V2
- Status: todo
- Size: L
- Owner: none
- Depends on: APP-030, INS-007, INS-011, SEC-013, SEC-020, SEC-025, SVC-025
- Baseline: §9, §63
- Threats: T-008, T-023
- Invariants: I-073

Build the first-boot flow that creates the administrator account, confirms encryption and saves the recovery key, records privacy and telemetry choices, and explains Secure Boot status. It runs after scripted or guided image install so V2 preview machines are usable; the V3 installer reuses the same flow. Published privacy policy text is GOV at V3; this task stores local consent flags.

<!-- covers: GAP-0237 -->

#### Out of scope
Identity service and key-slots (SEC). Greeter chrome after first boot (APP-030). Crash-report submission (INS-020). Privacy policy publication (GOV-061).

#### Acceptance criteria
- [ ] Completing first-boot on H-002, H-004 and H-005 creates an administrator account that can log in at the greeter, and a second run of the flow is refused.
- [ ] When encryption is on, the flow displays the recovery key, requires an acknowledgement that it was saved, and records that acknowledgement; it offers no cloud escrow.
- [ ] Privacy and telemetry default to off, and the stored consent flags are readable by the later crash-report client.
- [ ] Secure Boot enrolled, custom-key, or off is shown as a status string, not as a silent pass.
- [ ] Opting out of encryption is possible only when INS-007 accepted an opt-out, and the opt-out is an explicit control rather than a missing checkbox.

#### Verification
- Integration: `installer:tests/first_boot_setup` on CI matrix entries `qemu-x86_64`, `hw-h002`, `hw-h004` and `hw-h005`.
- Manual: complete first-boot on H-004, power off, and unlock with the saved recovery key through BOOT's pre-boot unlock.

#### Evidence
- none

### INS-011 · Provide guided scripted image install for preview machines
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: INS-007, INS-008, INS-005
- Baseline: §62, §63
- Risks: R-046
- Invariants: I-073

Provide a guided script (not a public graphical installer) that walks disk selection on H-002, H-004 and H-005 using the V2 disk-policy adr, then hands off to first-boot. V2 remains image-based install.

<!-- covers: INV-1236 -->

#### Out of scope
Graphical installer (INS-027). First-boot account flow (INS-010). Dual-boot shrink of BitLocker volumes (INS-026).

#### Acceptance criteria
- [ ] On each of H-002, H-004 and H-005 the guided script lists candidate disks, applies the accepted disk policy, installs the preview image, and exits into first-boot.
- [ ] Choosing wipe requires typing a confirmation token that includes the disk identity; backing out leaves existing partitions unchanged.
- [ ] Encryption follows INS-007; a missing recovery-key acknowledgement aborts before the generation is marked current.
- [ ] The script refuses to install when free space or ESP size cannot hold the recovery generation plus retained kernels named by the disk adr.

#### Verification
- Integration: `installer:tests/guided_image_install` on `hw-h002`, `hw-h004` and `hw-h005`.
- Manual: run the wipe-abort path on a spare disk on H-002 and confirm the original partition table.

#### Evidence
- none

### INS-012 · Write the installer and updater threat model
- Type: docs
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: PKG-054, SEC-002
- Baseline: §9, §51, §63
- Threats: T-008, T-022, T-023, T-028, T-035

Extend the V0 system threat model with install-media tampering, dual-boot ESP attacks, update rollback abuse and crash-report leaks before the V3 installer and crash client ship. SEC owns the register; this document is the INS addendum. Required by V3-G04 (Crash reports are symbolicated and opted in): the documented privacy review and the no-disk-keys rule rest on this threat model.

#### Out of scope
System-wide threat register (SEC-002). Package supply-chain section (PKG-054). Crash-client implementation (INS-020).

#### Acceptance criteria
- [ ] A committed document maps T-008, T-022, T-023, T-028 and T-035 to installer, updater, recovery and crash-report controls, each with an owning task id.
- [ ] Unsigned or mutated install media is named as fail-closed before disk writes.
- [ ] Recovery that mounts user data without authentication is named as T-035 and rejected.
- [ ] A Review line names SEC review of the addendum on the pull request.

#### Verification
- Review: SEC lead sign-off recorded on the pull request, confirming each cited T-ID has a control.

#### Evidence
- none

### INS-013 · Ship a signed recovery SystemGeneration on the ESP
- Type: build
- Milestone: V2
- Status: todo
- Size: L
- Owner: none
- Depends on: BOOT-029, BOOT-027, INS-008, PKG-016, REL-003
- Baseline: §30, §63
- Risks: R-018
- Threats: T-008, T-035
- Invariants: I-022

Ship a permanently resident signed recovery SystemGeneration that boots without the main store, so V3 recovery UX and the installer have something to install. BOOT integrity-failure UX hands off to it. The recovery image is older-watermark protected (T-035) and does not mount user volumes until authenticated.

<!-- covers: GAP-0205 -->

#### Out of scope
Recovery UX for unlock, restore, reinstall and logs (INS-041). Filesystem repair tools inside the image (STO-061). Integrity-failure handoff (BOOT-033). Last-resort text console (SVC-040).

#### Acceptance criteria
- [ ] Guided or scripted install on H-002, H-004 and H-005 leaves a signed recovery generation on the ESP (or the partition the disk adr named) that boots after the main store is wiped in the test harness.
- [ ] Booting recovery does not mount encrypted user volumes until a passphrase, TPM-PIN or recovery key succeeds.
- [ ] A recovery image older than the anti-rollback watermark is refused (or the BOOT anti-rollback adr's accepted branch is followed) and the refusal is logged.
- [ ] BOOT integrity-failure UX on a tampered generation offers this recovery entry and no unsigned bypass.

#### Verification
- Integration: `installer:tests/recovery_generation_esp` on `qemu-x86_64`, `hw-h002`, `hw-h004` and `hw-h005`.
- Manual: wipe the store partition on H-002 and boot recovery from firmware boot order.

#### Evidence
- none

### INS-014 · Expose SystemGeneration restore in Settings and boot UX
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: APP-040, BOOT-028, BOOT-014, INS-006, PKG-073, PKG-074, PKG-060, SDK-069, PKG-017
- Baseline: §30, §31, §62
- Invariants: I-022

Expose SystemGeneration restore as a first-class Settings action and at boot, not emergency tooling. Consumes PKG's generation-management interface and BOOT's generation menu; PKG keeps `os restore`. Surfaces automatic fallback so a fault-injected bad update is visible in Settings for the V2 demo.

<!-- covers: INV-0563, INV-0590, INV-1226 -->

#### Out of scope
`os restore` implementation (PKG, SDK). Boot-menu selection mechanics (BOOT-014). Safe-mode recovery shell (APP-037, SVC-036).

#### Acceptance criteria
- [ ] Settings on H-002, H-004 and H-005 lists bootable SystemGenerations and restoring one sets the next-boot default through PKG's interface.
- [ ] After BOOT-028 rolls back unattended, Settings shows the failed generation as bad and the previous generation as current.
- [ ] Restoring from Settings preserves ApplicationData that PKG-017 left off the generation tree.
- [ ] Native software in this UI holds no POSIX path authority; it talks to the typed generation interface only.

#### Verification
- Integration: `installer:tests/rollback_settings_ui` on `hw-h002`, `hw-h004` and `hw-h005`.
- Demo: fault-inject a bad update on H-002; automatic fallback; Settings shows the bad generation and restore remains available.

#### Evidence
- none

### INS-015 · Publish delta update size and automatic rollback time
- Type: benchmark
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: BOOT-028, INS-025, INS-045, Q-001, BEN-005, PKG-083
- Baseline: §30, §54, §63
- Benchmarks: B-035
- Invariants: I-061, I-088

Run harness `bench:delta-rollback` so V3, V4 and 1.0 gates can publish B-035: client download size for a typical release-to-release update relative to the full generation, and wall time from boot-failure detection to the previous generation's greeter. PKG benches store-level delta size; this task is the client and rollback-time half. Publish-only; no superiority claim.

#### Out of scope
Register ownership and methodology (BEN-007, Q-001). Store-level delta encoding (REL-009, PKG-083).

#### Acceptance criteria
- [ ] A committed B-035 report exists for every V3 Tier 1 H-ID with client bytes downloaded and rollback time to greeter.
- [ ] The report names the full-generation size used as the denominator and the ostree and Windows baselines from the register.
- [ ] No announcement or task prose states a byte count or duration except by citing B-035 (I-061).

#### Verification
- Bench: B-035 on every V3 Tier 1 H-ID; target per register (V3 publish).
- Review: BEN lead confirms the harness matches registers/benchmarks.md.

#### Evidence
- none

### INS-016 · Publish install, first-boot and update-apply times
- Type: benchmark
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: INS-010, INS-027, INS-045, Q-001, BEN-005, INS-032, LAB-021
- Baseline: §54, §63
- Benchmarks: B-034
- Invariants: I-061, I-088

Run harness `bench:install-update-time` so V3, V4 and 1.0 gates can publish B-034 per Tier 1 machine: media boot to first login, first-boot setup to desktop, and download-plus-apply of a typical update as a new SystemGeneration. INS owns the harness; BEN owns the register. Publish-only; no superiority claim.

<!-- covers: GAP-0411 -->

#### Out of scope
Methodology (Q-001, BEN-007). Boot-to-login excluding installer (B-032, BOOT, SVC).

#### Acceptance criteria
- [ ] A committed B-034 report exists for every V3 Tier 1 H-ID with the three stages named in the register.
- [ ] USB-media and update-channel methods in the report match registers/benchmarks.md.
- [ ] No superiority claim versus Linux or Windows appears without the published table (I-061).

#### Verification
- Bench: B-034 on every V3 Tier 1 H-ID; target per register (V3 publish).
- Review: BEN lead confirms stage boundaries are not double-counted against B-032.

#### Evidence
- none

### INS-017 · Share Bluetooth pairing keys across dual-boot
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-035, HW-036, INS-026, SEC-027
- Baseline: §33, §63
- Risks: R-046

Import and export Bluetooth pairing keys during install and recovery so a co-installed Windows or Linux keeps mice and headsets paired. HW owns the Bluetooth stack; INS owns the dual-boot transfer. Keys land in the SEC secrets service, not a world-readable file.

<!-- covers: GAP-0420 -->

#### Out of scope
Bluetooth host, profiles and pairing UI (HW). Secrets storage (SEC-027). Dual-boot partitioning (INS-026).

#### Acceptance criteria
- [ ] After a dual-boot install beside a fixture Windows or Linux image that has a paired HID device, JakeOS connects that device without a new pairing dance on H-004 or H-005.
- [ ] Exported keys are written only into Capability-scoped secret objects; a native Component without the Bluetooth-device Capability cannot read them.
- [ ] Recovery can re-import the same keys after a reinstall onto the same disk layout.
- [ ] When the foreign OS store cannot be decoded, the installer records a typed skip and first-boot pairing proceeds normally.

#### Verification
- Integration: `installer:tests/bluetooth_key_share` on `hw-h004` and `hw-h005` with LAB Bluetooth peers.
- Manual: pair a headset under the foreign OS, install alongside, confirm A2DP reconnect on JakeOS without pairing UI.

#### Evidence
- none

### INS-018 · Import browser bookmarks, history, passwords and extensions
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: APP-019, INS-036, INS-049, SEC-027
- Baseline: §25, §31, §63
- Threats: T-023
- Invariants: I-021

Import bookmarks, history, saved passwords and extensions from Chrome, Edge, Firefox and Safari profiles into the browser APP-019 selected. Passwords land in the SEC secrets service, not a world-readable file.

<!-- covers: GAP-0418 -->

#### Out of scope
Browser strategy (APP-019). Secrets service (SEC). Migration assistant chrome (APP-062). Undo snapshot (INS-036).

#### Acceptance criteria
- [ ] Import from each of the four profile kinds on a fixture disk copies bookmarks and history into the selected browser's Collection and reports a per-item count.
- [ ] Saved passwords are written only as secret objects; `os inspect` on the importer Component shows no Capability to a world-readable password file after completion.
- [ ] Extensions the selected browser cannot load are listed as skipped, not silently dropped.
- [ ] Abort mid-import leaves the pre-import snapshot restorable through INS-036.

#### Verification
- Integration: `installer:tests/browser_profile_import` on `qemu-x86_64` with fixture Chrome, Edge, Firefox and Safari profile trees.
- Review: SEC lead confirms passwords never hit ApplicationData as plaintext.

#### Evidence
- none

### INS-019 · Ship a built-in bug reporter bundling traces, inspect output and Generation ID
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: INS-021, OBS-006, OBS-034, SDK-007, HW-068
- Baseline: §24, §63, §64

Ship a built-in bug reporter that bundles SystemGeneration ID, hardware fingerprint, relevant traces and `os inspect` output with user-controlled redaction. Crash-report consent and intake stay on the crash-client and REL path; this is the user-initiated bundle.

<!-- covers: GAP-0387 -->

#### Out of scope
Crash capture format (OBS). Intake and triage dashboard (REL). Exact-contents preview widget (INS-021).

#### Acceptance criteria
- [ ] From Settings a user can generate a bundle that contains generation ID, inspect snapshot and a trace window, and can open the redaction preview before save or send.
- [ ] The reporter Component holds only the inspect and journal Capabilities it needs; it has no ambient filesystem or network grant.
- [ ] Saving the bundle without sending stores it in user-selected storage; sending uses the same opt-in path as the crash-report client.
- [ ] Hardware fingerprint in the bundle matches HW-068 (no serials or network identifiers).

#### Verification
- Integration: `installer:tests/bug_reporter_bundle` on `qemu-x86_64` and `hw-h002`.
- Review: OBS lead confirms the bundle schema matches journal-redaction-export.

#### Evidence
- none

### INS-020 · Ship opt-in crash-report client with secret scrubbing
- Type: build
- Milestone: V3
- Status: todo
- Size: L
- Owner: none
- Depends on: GOV-061, INS-021, INS-010, INS-012, KRN-048, OBS-026, OBS-027, OBS-029
- Baseline: §51, §63
- Risks: R-048
- Threats: T-023
- Invariants: I-077

Ship the opt-in crash-report client and consent UI. OBS owns capture format; REL owns intake; kernel dumps under lockdown stay with SEC. The client never stores disk keys or unlocked secrets (I-077) and submits only after show-and-redact.

<!-- covers: INV-1240, GAP-0235, GAP-0387, GAP-0365 -->

#### Out of scope
Capture format and symbolication (OBS). Intake, dedup and dashboard (REL). Privacy policy text (GOV-061). Kernel lockdown dump policy (SEC). Redaction UI widget (INS-021).

#### Acceptance criteria
- [ ] A Component crash, a service crash and a kernel panic each produce a client prompt only when first-boot or Settings consent is on; with consent off, nothing is queued for the network.
- [ ] Before send, the user sees the exact payload via INS-021 and can drop fields; send without that preview is impossible.
- [ ] A synthetic dump containing a disk-key or unlocked-secret marker is refused for storage and for send, and the refusal is tested (I-077).
- [ ] Submitted reports that REL-038 accepts include generation ID and the OBS signature, with no serials.
- [ ] The client Component has no ambient filesystem, home or network Capability beyond the intake Channel granted at consent.

#### Verification
- Integration: `installer:tests/crash_report_client` on `qemu-x86_64` and `hw-h002`.
- Review: SEC and GOV leads confirm I-077 and the published privacy policy are honoured.
- Demo: opt in, crash a test Component on H-002, redact one field, observe the report in REL intake.

#### Evidence
- none

### INS-021 · Show crash and telemetry contents and allow redaction before send
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: GOV-061, OBS-050, OBS-034
- Baseline: §9, §63
- Threats: T-023
- Invariants: I-077

Provide the exact-contents preview and field-level redaction used by the crash-report client, bug reporter and any opt-in telemetry upload. Consent without preview is not consent (T-023).

<!-- covers: GAP-0365 -->

#### Out of scope
Intake (REL). Capture (OBS). Policy text (GOV). Client orchestration (INS-020).

#### Acceptance criteria
- [ ] Preview renders every field OBS-050 lists, including payloads the user did not author.
- [ ] Redacting a field removes it from the bytes that would be sent, proven by a hash of the payload before and after.
- [ ] Fields classified as disk keys or unlocked secrets are pre-removed and cannot be re-added in the UI (I-077).
- [ ] Cancel leaves nothing queued for the network.

#### Verification
- Unit: `installer:tests/redaction_preview_*` on `qemu-x86_64`.
- Integration: crash-client and bug-reporter paths both open this preview on `qemu-x86_64`.

#### Evidence
- none

### INS-022 · Decide hardware clock UTC versus localtime for dual-boot
- Type: adr
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: INS-008, SVC-018, SVC-032
- Baseline: §63
- Decision: D-0134

Decide RTC UTC versus local time and time-sync behaviour on machines shared with Windows, so clocks do not drift by the timezone offset on every reboot. Applied by the V3 installer. SVC owns the time service.

<!-- covers: GAP-0405 -->

#### Out of scope
NTP/NTS client (SVC-032). Timezone auto-detection (SVC-037). Dual-boot partitioning (INS-026).

#### Acceptance criteria
- [ ] The decision file evaluates at least UTC with a documented Windows registry fix, localtime to match Windows default, and UTC-only with a warning when Windows is detected.
- [ ] The accepted option states what the installer writes to the RTC and what it displays when a Windows Boot Manager entry is present.
- [ ] The accepted option names the SVC time-service interface used after first boot so the policy is not a one-shot write.
- [ ] A Review line names who accepts the decision on the pull request.

#### Verification
- Review: SVC and INS leads sign off on the pull request.

#### Evidence
- none

### INS-023 · Decide Linux home adopt-in-place versus copy
- Type: adr
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: STO-042, STO-074
- Baseline: §25, §46, §63
- Decision: D-0136
- Invariants: I-021

Decide whether an existing Linux `/home` can be adopted in place for the Linux personality or must be copied into native Collections. STO three-view mapping is an input. Native software still sees Collections, not a POSIX home.

<!-- covers: GAP-0413 -->

#### Out of scope
Import engine (INS-033). Foreign volume unlock (STO-074). Personality path synthesis (STO-047, LNX).

#### Acceptance criteria
- [ ] The decision file evaluates at least copy-only, adopt-in-place read-write for the Linux personality, and adopt read-only with copy into Collections.
- [ ] The accepted option states what the user sees for each outcome and that native apps still receive Collection Capabilities, not a home path.
- [ ] The accepted option names the failure when the volume is dirty, encrypted, or smaller than the copy requires.
- [ ] A Review line names who accepts the decision on the pull request.

#### Verification
- Review: STO, LNX and INS leads sign off on the pull request.

#### Evidence
- none

### INS-024 · Decide dual-boot shared data partition format
- Type: adr
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: INS-008, STO-055, STO-058, STO-060
- Baseline: §25, §63
- Decision: D-0137

Decide the recommended dual-boot shared data partition format and whether the installer offers to create one. STO owns the drivers; this adr is the installer offer and default.

<!-- covers: GAP-0430 -->

#### Out of scope
NTFS dirty-volume policy (STO-060). exFAT driver (STO-058). Creating the volume (INS-042).

#### Acceptance criteria
- [ ] The decision file evaluates at least exFAT, NTFS, and no shared partition by default.
- [ ] The accepted option states Fast Startup and hibernation limits the installer must explain when NTFS is involved.
- [ ] The accepted option names the STO degradation contract used when the chosen format lacks native metadata.
- [ ] A Review line names who accepts the decision on the pull request.

#### Verification
- Review: STO and INS leads sign off on the pull request.

#### Evidence
- none

### INS-025 · Fetch chunk-level deltas between SystemGenerations
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: INS-045, PKG-083, REL-009
- Baseline: §27, §30, §63
- Threats: T-028

Fetch only new content-addressed objects when applying generation N+1 so the updater does not download a full image. Uses PKG delta-object-transfer and REL's delta-size spike. Client-side; the store reconstructs objects before activation.

<!-- covers: GAP-0333 -->

#### Out of scope
Store-level encode/decode (PKG-083). Server delta publication (REL). Orchestration policy (INS-009). Benchmark publication (INS-015).

#### Acceptance criteria
- [ ] Applying N+1 when N is present transfers only objects not in N, proven by a trace of fetched identities.
- [ ] A truncated or unauthenticated delta does not activate N+1 and leaves N selected.
- [ ] Reconstructed objects match PKG content identity before the generation is offered to BOOT.
- [ ] On a metered link the fetch follows INS-009 (consent or skip).

#### Verification
- Integration: `installer:tests/delta_fetch_client` on `qemu-x86_64` and `hw-h002`.
- Bench: B-035 samples recorded from this client on one Tier 1 machine before the dedicated bench task publishes the set.

#### Evidence
- none

### INS-026 · Preserve foreign OS, ESP and BitLocker during install
- Type: build
- Milestone: V3
- Status: todo
- Size: L
- Owner: none
- Depends on: BOOT-029, INS-022, INS-008, STO-060
- Baseline: §63
- Risks: R-046
- Threats: T-008

Preserve existing ESP entries, offer shrink-and-install-alongside Windows, Linux and Intel-Mac installs, warn on BitLocker/TPM with recovery-key confirmation, and refuse read-write on dirty or Fast Startup hibernated NTFS. BOOT chain-loads; STO owns NTFS policy; this is the installer-side coexistence.

<!-- covers: GAP-0208, GAP-0399, GAP-0404, GAP-0406 -->

#### Out of scope
Chain-load and BootOrder repair (BOOT-041). NTFS driver (STO-060). Shared-volume creation (INS-042). Hardware clock policy (INS-022).

#### Acceptance criteria
- [ ] Installing alongside a fixture Windows image on H-001 preserves every pre-existing ESP entry and the Windows Boot Manager path.
- [ ] Detecting BitLocker or a TPM-sealed Windows disk shows an explanation that boot-chain changes may trigger BitLocker recovery and requires the user to confirm a recovery key was recorded before any resize.
- [ ] A dirty or hibernated NTFS volume is never mounted read-write; the installer explains Fast Startup and offers abort.
- [ ] Shrink-and-alongside of an unprotected volume leaves every foreign partition and boot entry intact, proven by a pre/post partition table and EFI BootOrder diff.
- [ ] Intel-Mac and Linux fixture disks are detected and offered the same non-destructive alongside path, or refused with a typed reason.

#### Verification
- Integration: `installer:tests/dual_boot_coexistence` on `qemu-x86_64` with Windows, Linux and dirty-NTFS fixtures, Secure Boot on and off.
- Manual: alongside-install on a LAB Windows baseline disk (LAB-015) on H-002.

#### Evidence
- none

### INS-027 · Build the guided graphical installer with encryption
- Type: build
- Milestone: V3
- Status: todo
- Size: L
- Owner: none
- Depends on: INS-007, INS-008, INS-026, INS-010, INS-034, INS-037, INS-013, UIP-013
- Baseline: §41, §63
- Risks: R-046
- Threats: T-008
- Invariants: I-073, I-095

Build the guided native-UI installer with disk selection, encryption default-on with opt-out as decided, and dual-boot awareness. Not a POSIX installer clone. First-boot is the same flow V2 already shipped. Pre-commit Hardware Compatibility List warning is a sibling task.

<!-- covers: INV-1236, GAP-0196 -->

#### Out of scope
HCL verdict (INS-028). Live session that hosts this UI (INS-034). Screen-reader engine (ACC). Partition mkfs library (STO).

#### Acceptance criteria
- [ ] From live media on H-001 the installer completes a fresh UEFI encrypted install and hands off to first-boot without a POSIX shell as the user path.
- [ ] Encryption is on unless the user takes the explicit opt-out named by INS-007 (I-073).
- [ ] Disk selection shows wipe and alongside offers from INS-026 and will not write until the user confirms.
- [ ] The installer is a Component graph holding only installer Capabilities; it has no ambient filesystem namespace.
- [ ] Cancel or crash before commit leaves the disk layout unchanged, proven by a partition-table snapshot.
- [ ] On H-004 the path from live-media boot to a working desktop with Wi-Fi completes within 30 minutes wall-clock (the V3-D01 bound).

#### Verification
- Integration: `installer:tests/graphical_installer_fresh` on `qemu-x86_64` and `hw-h002`.
- Demo: live media on H-002, encrypted install, first-boot, desktop.

#### Evidence
- none

### INS-028 · Warn from the Hardware Compatibility List before disk writes
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-066, HW-047, INS-027, INS-034, REL-048
- Baseline: §62, §63
- Invariants: I-095

Query the compatibility database from live media and show an accurate verdict before any disk write. REL and HW own the database and probe; SDK may wrap the CLI; INS owns the pre-commit warning. Unlisted hardware is unsupported (I-095).

<!-- covers: GAP-0370 -->

#### Out of scope
Probe implementation (HW-066). Database publication (REL-048). Host-side probe on a foreign OS (HW-067).

#### Acceptance criteria
- [ ] On live media the installer shows a verdict of supported, degraded or unsupported for the running machine before the commit step is enabled.
- [ ] A fixture machine whose Wi-Fi or GPU is listed degraded shows that component named in the warning text.
- [ ] An unlisted machine shows unsupported and requires an explicit continue control before writes; abort leaves disks untouched.
- [ ] The verdict bytes match the probe output for the same boot (no second, weaker probe).
- [ ] The verdict is correct for 20 community-reported Tier 2 machines drawn from the compatibility database (the V3-G01 sample size).

#### Verification
- Integration: `installer:tests/hcl_preinstall_warn` on `qemu-x86_64` with fixture HCL rows and on `hw-h002`.
- Demo: live media on an unsupported fixture shows the warning before disk selection commit.

#### Evidence
- none

### INS-029 · Run the unaided public-install usability study
- Type: docs
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: DOC-030, INS-027, INS-035, INS-039
- Baseline: §58, §63

Run the moderated unaided public-install study the V3 exit names: participants without project involvement install from public media on a Tier 1 machine following only public documentation. DOC verifies the docs are the sole materials; INS owns the installer behaviour under study.

#### Out of scope
Docs site and install guide authorship for publication (DOC). Media creation (INS-035). Accessibility audit (ACC-034).

#### Acceptance criteria
- [ ] The study has eight participants with no project involvement and at least seven complete the install unaided (the V3-G02 participant and success counts); the report records each session against that bar.
- [ ] Failures are filed as INS or DOC tasks with the screen and docs paragraph that failed, not as private notes.
- [ ] The report states that only public media and public docs were available to participants.

#### Verification
- Review: DOC and INS leads accept the study report on the pull request.
- Manual: a moderator who is not an INS author runs the gate's participant protocol on a Tier 1 machine using only public media and public docs.

#### Evidence
- none

### INS-030 · Make installer and recovery screen-reader accessible and localised
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: ACC-021, ACC-023, INS-027, INS-041, SVC-025, TXT-041, TXT-032
- Baseline: §41, §42, §63

Make installer, migration assistant surfaces INS owns, and recovery match desktop languages and be screen-reader accessible from the first screen. Inherit keyboard layout, locale, timezone and hostname from an existing OS when present. ACC owns the reader; TXT owns strings; APP hosts migration chrome.

<!-- covers: GAP-0474, GAP-0475 -->

#### Out of scope
Screen-reader engine (ACC). Catalog format (TXT). Greeter and lock (APP, ACC-029 remainder).

#### Acceptance criteria
- [ ] The first installer screen on live media is spoken by the screen reader after the documented keystroke, before any disk write control is focused.
- [ ] Recovery's unlock, restore and reinstall screens are in the same spoken tree and in each language TXT-041 ships.
- [ ] When an existing OS is detected, keyboard layout, locale, timezone and hostname are offered as pre-filled values the user can accept or edit.
- [ ] A missing translation key fails CI for installer and recovery catalogs rather than showing an English fallback in a shipped language.

#### Verification
- Integration: `installer:tests/a11y_l10n_first_screen` on `qemu-x86_64` with the ACC tree driver.
- Manual: ACC task-script first-install tasks on H-002 with speech capture.

#### Evidence
- none

### INS-031 · Automate installer QEMU runs across disk layouts
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: BLD-012, INS-026, INS-027, INS-028
- Baseline: §63
- Risks: R-046

Run the installer automatically in QEMU for fresh UEFI, encrypted, dual-boot alongside Windows, existing-data preservation, with Secure Boot enabled and disabled. This is the permanent CI matrix for the V3 installer gate.

<!-- covers: GAP-0150 -->

#### Out of scope
Tier 1 hardware success (INS-032). QEMU matrix axes for kernel boot (BLD-012). Guest agent (BLD-006).

#### Acceptance criteria
- [ ] CI matrix entries exist for fresh UEFI, encrypted, dual-boot-alongside-Windows, existing-data-preservation, Secure Boot on, and Secure Boot off, each installing and booting the new generation on H-001.
- [ ] A failure maps to this task id and to the layout name; a pass records the generation identity.
- [ ] Existing-data-preservation leaves the fixture user partition hashes unchanged.
- [ ] The matrix is merge-blocking for installer changes.

#### Verification
- Integration: `installer:tests/qemu_layout_matrix` on CI matrix entry `qemu-x86_64`.
- Review: BLD lead confirms the jobs are in the release-qualification tier.

#### Evidence
- none

### INS-032 · Prove installer success rate on every Tier 1 machine
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-062, HW-070, INS-027, INS-028, INS-031, LAB-021, LAB-022, SEC-055
- Baseline: §62, §63
- Risks: R-046
- Invariants: I-073

Prove the installer completes on every V3 Tier 1 machine in automated runs with full-disk encryption, including the live-installer HCL verdict path. The success threshold is the V3 installer gate, not a number in this task.

#### Out of scope
Lab scheduling (LAB-022). FDE contract check (SEC-055). QEMU layouts (INS-031).

#### Acceptance criteria
- [ ] Automated encrypted install plus first-boot succeeds on H-002, H-004, H-005, H-006, H-007 and H-008 in the LAB qualification job.
- [ ] Each run records an HCL verdict before commit and a recovery generation present after commit.
- [ ] Failures are filed per H-ID with serial and installer logs attached to this task id.
- [ ] At least 99 of 100 automated encrypted installs succeed on each V3 Tier 1 machine in the qualification job, and every failure is classified as lab infrastructure with the log attached; a failure attributable to the installer fails the gate regardless of count (the V3-G01 success rate and run count).

#### Verification
- Integration: LAB qualification suite `installer:tests/tier1_install` on every V3 Tier 1 H-ID.
- Review: LAB and INS leads accept the per-machine report on the pull request.

#### Evidence
- none

### INS-033 · Import Linux home directories with per-item destination choice
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: INS-023, INS-036, STO-042, STO-074
- Baseline: §25, §46, §63
- Invariants: I-021

Import XDG user directories, dotfiles, SSH and GPG keys, git config and shell history from an existing Linux home, each choosable as Linux personality home versus native application data, according to INS-023. Native apps receive Collection Capabilities, not a home path.

<!-- covers: GAP-0413, GAP-0415 -->

#### Out of scope
Adopt-versus-copy decision (INS-023). Volume unlock (STO-074). Migration UI chrome (APP-062). SSH agent (SEC-030).

#### Acceptance criteria
- [ ] A fixture Linux home offers per-item destinations for Documents, Pictures, Desktop, Downloads, Music, Videos, dotfiles, SSH keys, GPG keys, git config and shell history.
- [ ] Items sent to native application data appear as Collection objects; items sent to the Linux personality home are visible only through the personality view API.
- [ ] SSH and GPG keys land in the secrets or Capability-scoped store the SEC tasks name, not in a world-readable Collection.
- [ ] Import refuses to start without a pre-import snapshot (INS-036).

#### Verification
- Integration: `installer:tests/linux_home_import` on `qemu-x86_64` with a fixture home and a LUKS+ext4 volume via STO-074.
- Review: STO and LNX leads confirm native Components still hold no POSIX home Capability.

#### Evidence
- none

### INS-034 · Ship a live-boot USB desktop that does not touch disks
- Type: build
- Milestone: V3
- Status: todo
- Size: L
- Owner: none
- Depends on: APP-040, APP-043, GFX-008, GFX-010, HW-066, INS-004, INS-001, HW-068
- Baseline: §60, §63
- Invariants: I-095

Ship a live-boot USB image that runs a full native desktop from removable media without touching internal disks, and that produces a hardware compatibility report before the user commits to installing. Primary HCL submission path at public alpha.

<!-- covers: GAP-0407 -->

#### Out of scope
Graphical installer commit (INS-027). HCL probe (HW-066). Media writing on foreign hosts (INS-035).

#### Acceptance criteria
- [ ] Booting the live image on H-001 with a fixture internal disk whose hashes are recorded leaves those hashes unchanged after a desktop session and shutdown.
- [ ] The live session starts Terminal, File Browser, Settings and the HCL checker without requiring an install.
- [ ] The HCL report can be saved to the USB or submitted only after opt-in consent that matches HW-068.
- [ ] Internal disks are not mounted read-write; an attempt to open them without the installer commit path returns `Error::Rights`.

#### Verification
- Integration: `installer:tests/live_usb_no_touch` on `qemu-x86_64` and `hw-h002`.
- Demo: live USB on H-002, run HCL checker, shutdown, verify internal disk hashes.

#### Evidence
- none

### INS-035 · Ship install-media creation with checksum and signature checks
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: REL-020, REL-019, INS-027
- Baseline: §63
- Threats: T-008

Ship install-media creation for Windows, macOS and Linux hosts (or vetted third-party tool instructions) with mandatory checksum and signature verification. The first adoption step happens on a foreign OS.

<!-- covers: GAP-0408 -->

#### Out of scope
Download site and signatures (REL-020). Image builder (INS-001). Live desktop contents (INS-034).

#### Acceptance criteria
- [ ] Official tooling or documented third-party steps verify checksum and signature before any USB write, and refuse to write on mismatch.
- [ ] A mutated image fails verification on a Linux host in CI and on documented Windows and macOS steps in a manual matrix.
- [ ] Successful write produces media that boots the graphical installer on H-001.
- [ ] Instructions never ask the user to disable signature checks.

#### Verification
- Integration: `installer:tests/media_create_verify` on a Linux host in CI writing a USB image consumed by H-001.
- Manual: follow the Windows and macOS host instructions once per V3 media cut and attach checksum logs.

#### Evidence
- none

### INS-036 · Record each migration import as a restorable history event
- Type: build
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: PKG-022, PKG-060, STO-025, STO-077
- Baseline: §31, §63
- Invariants: I-022

Record every migration import as a system history event with a pre-import snapshot undoable via standard restore. PKG owns the log; INS emits the events and refuses import without a snapshot.

<!-- covers: GAP-0422 -->

#### Out of scope
History log (PKG-022). User-data snapshot restore (STO-077). Import engines (INS-049, INS-033).

#### Acceptance criteria
- [ ] Starting any import engine without a successful pre-import snapshot returns a typed error and copies no files.
- [ ] A completed import appears in `os history` as a migration event naming the source and snapshot id.
- [ ] Restoring that history event via the standard restore operation returns Collections to the pre-import snapshot, proven by content hashes.
- [ ] Partial failure still leaves the snapshot restorable.

#### Verification
- Integration: `installer:tests/migration_undo_snapshot` on `qemu-x86_64`.
- Unit: `installer:tests/refuse_import_without_snapshot`.

#### Evidence
- none

### INS-037 · Show third-party notices and GPL offer in installer and Settings
- Type: build
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: REL-049
- Baseline: §63, §67

Show an About/licenses view in the installer exposing all license texts and the GPL written offer. REL publishes the notices bundle; APP-057 renders the Settings copy.

<!-- covers: GAP-0014 -->

#### Out of scope
Notices bundle generation (REL-049). Settings About panel (APP-057). License firewall policy (GOV).

#### Acceptance criteria
- [ ] The installer has a licenses screen that lists every text in the REL bundle for the generation being installed, including the GPL written offer.
- [ ] Missing bundle fails the image build rather than shipping an installer without notices.
- [ ] The screen is reachable before disk commit so a user can read licenses without installing.

#### Verification
- Integration: `installer:tests/notices_view` on `qemu-x86_64`.
- Review: GOV licensing lead confirms the GPL written offer text is present.

#### Evidence
- none

### INS-038 · Support offline install media and local repository mirrors
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: INS-027, INS-035, INS-045, REL-024, REL-037, REL-050
- Baseline: §27, §63

Support air-gapped and lab machines that install and update without internet via offline media and local mirrors. REL owns mirror protocol; INS ships media that embeds or points at a verified local repository.

<!-- covers: GAP-0337 -->

#### Out of scope
Mirror topology and CDN (REL-024). Public repository (REL-050). Unattended answer files (INS-052).

#### Acceptance criteria
- [ ] An offline USB image installs on H-001 with the NIC unplugged and reaches first-boot.
- [ ] A local repository mirror specified at install is signature-verified before any object is activated.
- [ ] An update from that mirror creates SystemGeneration N+1 with no WAN traffic, proven by a packet capture in the test.
- [ ] A mirror whose TUF or signature metadata is stale is refused (REL-037).

#### Verification
- Integration: `installer:tests/offline_media_and_mirror` on `qemu-x86_64` with network disabled after mirror attach.
- Manual: air-gapped install on one LAB machine using a local mirror.

#### Evidence
- none

### INS-039 · Write public install and recovery procedures for strangers
- Type: docs
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: BOOT-042, INS-026, INS-027, INS-041
- Baseline: §63

Author the install guide, recovery guide and dual-boot/BitLocker warnings from actual INS behaviour so DOC can publish them. The V3 documentation gate and usability study depend on this source. Required by V3-G02 (A stranger installs from public documentation).

#### Out of scope
Docs site (DOC-030, DOC-026). Secure Boot enrolment steps owned by BOOT (BOOT-042).

#### Acceptance criteria
- [ ] Committed procedures cover fresh install, alongside dual-boot, BitLocker recovery-key warning, encrypted first-boot, and recovery unlock/restore/reinstall.
- [ ] Every step is something the graphical installer or recovery environment actually presents, checked against screenshots or inspect dumps in the pull request.
- [ ] DOC-030 can consume these procedures without inventing UI that INS does not ship.

#### Verification
- Review: DOC and INS leads sign off on the pull request.
- Manual: a reviewer follows the recovery procedure on H-002 after a deliberate unbootable generation.

#### Evidence
- none

### INS-040 · Restore a system whose current SystemGeneration is corrupted
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: INS-041, PKG-088
- Baseline: §30, §63
- Invariants: I-022

Use the recovery environment plus PKG store verify/repair to restore a system whose current generation is corrupted, tested on every V3 Tier 1 machine.

<!-- covers: INV-1239 -->

#### Out of scope
Recovery UX (INS-041). Store verify implementation (PKG-088). Boot integrity-failure handoff (BOOT-033).

#### Acceptance criteria
- [ ] After corrupting the current generation's store objects on each V3 Tier 1 H-ID, recovery boots, verify reports the corruption, and restore to the previous generation yields a login-capable desktop.
- [ ] User data volumes remain sealed until unlock and are unchanged after restore, proven by hashes.
- [ ] If no previous generation is bootable, recovery offers reinstall from media or network without claiming a false restore.

#### Verification
- Integration: `installer:tests/recovery_corrupt_restore` on every V3 Tier 1 H-ID.
- Demo: corrupt generation on H-002, recover to previous, desktop returns.

#### Evidence
- none

### INS-041 · Build recovery unlock, restore, reinstall and log collection
- Type: build
- Milestone: V3
- Status: todo
- Size: L
- Owner: none
- Depends on: INS-013, PKG-088, STO-061
- Baseline: §30, §31, §63
- Threats: T-035
- Invariants: I-022

Build recovery UX on the V2 signed recovery generation: disk unlock, restore any generation or snapshot, reinstall from network or media, and collect logs. STO ships filesystem repair inside it; SVC owns the last-resort text console when this UI cannot start.

<!-- covers: INV-1239, GAP-0205 -->

#### Out of scope
Resident signed image (INS-013). Text console fallback (SVC-040). Filesystem repair (STO-061). Corrupt-generation proof (INS-040).

#### Acceptance criteria
- [ ] Recovery on H-002 offers unlock, list-and-restore of generations and snapshots, reinstall from USB, reinstall from a verified network repository, and log bundle export.
- [ ] Unlock failure does not mount user volumes (T-035).
- [ ] Restore of generation N boots N on the next restart and leaves user data hashes unchanged.
- [ ] Log collection uses the same redaction inventory as crash reports for secret classes, and does not include disk keys.

#### Verification
- Integration: `installer:tests/recovery_environment_ux` on `qemu-x86_64` and `hw-h002`.
- Manual: each of unlock, restore, USB reinstall and log export on H-004.

#### Evidence
- none

### INS-042 · Offer a dual-boot shared data partition at install
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: INS-024, INS-026, STO-058, STO-060
- Baseline: §25, §63

Implement INS-024: the installer can create the recommended shared volume and explain Fast Startup and hibernation limits using the NTFS guard.

<!-- covers: GAP-0430 -->

#### Out of scope
Format decision (INS-024). NTFS dirty-volume refusal (INS-026). Driver (STO).

#### Acceptance criteria
- [ ] When the accepted option offers a shared volume, the installer can create it in the format named by the adr and both the foreign OS fixture and JakeOS can read a test file written from each side (or the adr's documented limitation is displayed).
- [ ] When the accepted option is no shared partition by default, the installer does not create one unless the user opts in, and opt-in still explains the limitation.
- [ ] NTFS shared volumes are never mounted read-write while dirty or hibernated.

#### Verification
- Integration: `installer:tests/shared_data_partition` on `qemu-x86_64` with a Windows fixture.
- Review: STO lead confirms the created volume uses the decided format only.

#### Evidence
- none

### INS-043 · Prove six consecutive Channel updates with automatic rollback
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: BLD-061, BOOT-028, INS-045, INS-046, REL-043
- Baseline: §30, §63
- Benchmarks: B-043
- Threats: T-028

Prove the updater delivers the consecutive-alpha-release count the V3 updater gate names through the channel, and that automatic rollback on a fault-injected failing generation works on every V3 Tier 1 machine.

#### Out of scope
Promotion without rebuild (REL-043, BLD-065). Boot fallback mechanism (BOOT). Update-success publication (BEN-052).

#### Acceptance criteria
- [ ] Each V3 Tier 1 H-ID applies six consecutive alpha-channel updates (the V3-G03 count), each as a new SystemGeneration, ending on a bootable desktop.
- [ ] A fault-injected failing generation in that sequence boots the previous generation unattended on each of those machines.
- [ ] B-043 samples for the sequence are recorded; this task does not restate the success threshold.

#### Verification
- Integration: `installer:tests/six_release_soak` on every V3 Tier 1 H-ID.
- Bench: B-043 on those H-IDs; target per register (V3 publish).

#### Evidence
- none

### INS-044 · Spike Wi-Fi credential import from NetworkManager and Windows
- Type: spike
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: NET-021, SEC-027
- Baseline: §9, §63
- Threats: T-023

Determine whether Wi-Fi credentials can be imported from NetworkManager keyfiles and DPAPI-encrypted Windows WLAN profiles, or whether the user must re-enter them. Answers Q-054 before INS-048. Native software never reads ambient network state; the spike runs as an installer Component with an explicit grant.

<!-- covers: GAP-0419 -->

#### Out of scope
Wi-Fi station stack (NET). Production import (INS-048). DPAPI inside the Windows personality runtime (WIN).

#### Acceptance criteria
- [ ] The report states for NetworkManager keyfiles and for Windows WLAN profiles whether 1.0 import is in, out, or requires re-entry, with evidence from fixture profiles.
- [ ] The report names the secret store the importer would use and the Capabilities it would hold.
- [ ] `reports/spikes/INS-044.md` exists with the spike skeleton headings.
- [ ] Q-054 is listed as answered by this task in the report's follow-up table (register update is GOV/register process, not this file).

#### Verification
- Report: which sources are in 1.0, which require re-entry, what DPAPI or keyfile constraints remain, and whether first-boot can be online without re-entry.
- Integration: fixture reads on `qemu-x86_64` recorded in the report.

#### Evidence
- none

### INS-045 · Apply updates as atomic new SystemGenerations
- Type: build
- Milestone: V3
- Status: todo
- Size: L
- Owner: none
- Depends on: BOOT-018, INS-006, INS-009, PKG-082, PKG-064, REL-005, REL-007
- Baseline: §30, §63
- Risks: R-079
- Threats: T-028, T-034
- Invariants: I-022, I-086

Apply updates as atomic new SystemGenerations, roll back from the updater and from the boot menu, and honour INS-009 (background fetch, deferral, metered links). PKG commits the generation; BOOT falls back; INS is the client.

<!-- covers: INV-1237, INV-1238 -->

#### Out of scope
Delta fetch (INS-025). Channel picker UI (INS-046). Atomic commit unit test (PKG-082). Running-app prompt (APP-047).

#### Acceptance criteria
- [ ] Fetching and applying N+1 from the developer then alpha channel leaves N bootable and never a mixed-version tree visible to running Components (T-034).
- [ ] An interrupted apply (killed client or power cut in QEMU) leaves either N or a complete N+1, matching PKG-082.
- [ ] The updater UI can roll back to N; the boot menu can roll back to N; both land on a desktop that INS-006 would mark good.
- [ ] Metered-link and deferral behaviour match the accepted orchestration adr, proven by tests for skip, defer, and deadline-apply.
- [ ] Tampered repository metadata is refused before activation (T-028).

#### Verification
- Integration: `installer:tests/updater_atomic_apply` on `qemu-x86_64` and `hw-h002`.
- Demo: apply an alpha generation on H-002, roll back from Settings, roll back from boot menu.

#### Evidence
- none

### INS-046 · Let the user pick and switch update channels
- Type: build
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: APP-040, INS-045, REL-004, REL-005
- Baseline: §63

Let the user pick and switch update channels (alpha, testing). REL owns promotion criteria; INS owns the Settings control that selects the channel and shows soak state. Stable waits for V4 REL infrastructure. Required by V3-G03 (Updater, automatic rollback and recovery): the alpha releases it counts are delivered through the channel selected here.

#### Out of scope
Promotion criteria (REL-004). Staged rollouts (REL-056). Updater apply path (INS-045).

#### Acceptance criteria
- [ ] Settings lists the channels REL-004 named for this rung and switching channel changes the next fetched generation's channel field.
- [ ] Soak or promotion state REL exposes is shown as text from repository metadata, not as an INS-invented health claim.
- [ ] Switching channel does not activate a generation until the user applies an update through the updater client.

#### Verification
- Integration: `installer:tests/channel_picker` on `qemu-x86_64`.
- Review: REL lead confirms channel names match the adr.

#### Evidence
- none

### INS-047 · Test N-to-N+1, rollback and oldest-to-current upgrade chains
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: INS-043, INS-045, LAB-022
- Baseline: §30, §63
- Invariants: I-022

Test generation N to N+1 and rollback, plus a long chain from the oldest supported release to current, on QEMU and lab hardware, so the §30 rollback guarantee stays true across public alphas.

<!-- covers: GAP-0149 -->

#### Out of scope
Six-release soak gate (INS-043). Format migration V3 to V4 (INS-054). Repository retention (REL-029).

#### Acceptance criteria
- [ ] CI on H-001 applies N to N+1, rolls back to N, then re-applies N+1, with user-data hashes preserved across the three boots.
- [ ] A chain from the oldest supported alpha generation retained in the test corpus to current succeeds on H-001 and on one lab Tier 1 machine.
- [ ] A failing generation in the chain triggers automatic rollback and the job fails if rollback does not boot.

#### Verification
- Integration: `installer:tests/upgrade_chain` on `qemu-x86_64` and one `hw-h00*` LAB entry.
- Review: REL lead confirms the oldest-supported fixture matches REL-029.

#### Evidence
- none

### INS-048 · Import Wi-Fi credentials during migration when feasible
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: INS-044, NET-021, SEC-027
- Baseline: §63
- Invariants: I-021

Implement the spike result so first-boot is not stuck offline. If the spike forbids a source, the installer requires re-entry for that source and records it. Answers the production half of Q-054.

<!-- covers: GAP-0419 -->

#### Out of scope
Spike (INS-044). Supplicant (NET). Secrets (SEC).

#### Acceptance criteria
- [ ] Sources the spike accepted import into NET profiles via the secrets service and associate on H-004 without re-entry.
- [ ] Sources the spike forbade show a re-entry field and write no partial secrets.
- [ ] Imported credentials are not visible to Components without a network or secrets Capability.
- [ ] The installer records which sources were imported and which were re-entry in the history event.

#### Verification
- Integration: `installer:tests/wifi_credential_import` on `hw-h004`.
- Review: the spike report's in/out table matches the implemented sources.

#### Evidence
- none

### INS-049 · Import Windows user-profile data into native Collections
- Type: build
- Milestone: V3
- Status: todo
- Size: L
- Owner: none
- Depends on: INS-026, INS-036, STO-042, STO-060
- Baseline: §25, §31, §48, §63
- Invariants: I-021

Import from on-disk or attached Windows profiles (Documents, Pictures, Desktop, Downloads, Music, Videos, and browser profiles handed to INS-018) into native Collections with progress, resumability and post-copy verification. APP may host the UI; INS owns the import engine. Native software never sees Win32 paths as authority.

<!-- covers: GAP-0414 -->

#### Out of scope
Migration assistant chrome (APP-062). Browser secret import (INS-018). Three-view mapping (STO-042). BitLocker unlock (SEC, STO-072).

#### Acceptance criteria
- [ ] Import from a fixture Windows profile copies the named folders into native Collections with a progress record that survives process kill and resumes.
- [ ] Post-copy verification hashes match source files that were readable; unreadable files are listed, not skipped silently.
- [ ] Hibernated or dirty NTFS sources are refused read-write; the engine explains Fast Startup and aborts.
- [ ] The engine Component holds only the UserSelected or Collection Capabilities for the source and destination; it has no ambient C:\Users namespace.

#### Verification
- Integration: `installer:tests/windows_profile_import` on `qemu-x86_64` with an NTFS fixture and on `hw-h002` with the LAB Windows baseline disk attached read-only.
- Fuzz: `installer:fuzz/windows_profile_tree` nightly without panic.

#### Evidence
- none

### INS-050 · Restore a system history point onto different hardware
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: INS-045, PKG-060, STO-071, STO-077
- Baseline: §31, §63
- Invariants: I-022

Restore a system history point (generations, packages, configuration, user data) onto different hardware from a backup target. STO EXTRA-036 is the backup counterpart; this task is the machine-to-machine apply path.

<!-- covers: GAP-0425 -->

#### Out of scope
Backup service (STO-071). HCL warning on the target machine (INS-028). Unattended disk layout (INS-052).

#### Acceptance criteria
- [ ] A backup of H-002 restored onto H-009 or another V4 desktop in lab boots a SystemGeneration whose package set and user Collections match the history point, after first-boot hardware-specific steps.
- [ ] Disk keys and TPM seals are not copied; the target re-enrols encryption and records a new recovery key.
- [ ] The HCL probe runs before commit on the target and can abort if the backup's required devices are unsupported.
- [ ] Rollback of the restore on the target returns it to its pre-restore generation.

#### Verification
- Integration: `installer:tests/machine_to_machine_restore` on a LAB pair of Tier 1 desktops.
- Manual: restore H-002 backup onto H-009, confirm Collections and a new recovery key.

#### Evidence
- none

### INS-051 · Spike macOS Time Machine and APFS migration for 1.0
- Type: spike
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: INS-033, STO-016
- Baseline: §25, §57, §63
- Invariants: I-009

Investigate importing from Time Machine backups on external drives and from a macOS host over the local network, and whether read-only APFS access is in scope for 1.0. The report accepts or parks APFS; no native filesystem rewrite (§57, I-009).

<!-- covers: GAP-0417 -->

#### Out of scope
Production macOS import engine (no 1.0 task until this report). Native APFS implementation. Windows and Linux import engines (already shipped at V3).

#### Acceptance criteria
- [ ] The report measures Time Machine on an external drive and over the local network against a fixture backup, naming what user data classes are reachable.
- [ ] The report states whether read-only APFS is in 1.0, parked to LATER, or infeasible, citing I-009 and §57.
- [ ] `reports/spikes/INS-051.md` exists with the spike skeleton headings.
- [ ] Follow-up task ids in the report already exist or are explicitly "none until `roadmap new task`".

#### Verification
- Report: which Time Machine paths work, whether APFS read-only is in 1.0, what remains network-only, and what is parked.
- Manual: attach a fixture Time Machine volume in LAB and record the reachable catalog.

#### Evidence
- none

### INS-052 · Support unattended install from a declarative answer file
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: INS-010, INS-027, INS-008, LAB-010
- Baseline: §63
- Invariants: I-073

Support unattended install from a declarative answer file covering disk layout, user, locale, network and generation channel. Required by the V4 lab matrix and fleet; the same answer file drives QEMU and hardware.

<!-- covers: GAP-0409 -->

#### Out of scope
Lab scheduler (LAB-010). Graphical UI (INS-027). Channel semantics (INS-046).

#### Acceptance criteria
- [ ] An answer file specifying disk, administrator, locale, network and channel installs on H-001 with no interactive prompts and boots to a completed first-boot state.
- [ ] The same file installs on one V4 Tier 1 machine via the LAB scheduler.
- [ ] Encryption defaults follow INS-007 even when the file omits the encryption key; omitting a required recovery-key acknowledgement field fails closed.
- [ ] A malformed file fails before disk writes with a typed parse error naming the field.

#### Verification
- Integration: `installer:tests/unattended_answer_file` on `qemu-x86_64` and one LAB Tier 1 H-ID.
- Review: LAB lead confirms the scheduler job uses this file, not a second ad-hoc script.

#### Evidence
- none

### INS-053 · Provide a supported uninstall that restores the previous bootloader
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: BOOT-041, INS-026, STO-077
- Baseline: §63
- Risks: R-046

Provide a supported uninstall that removes JakeOS boot entries, restores the previous default bootloader, reclaims partitions and exports user data in standard formats. Dual-boot coexistence must be reversible.

<!-- covers: GAP-0410 -->

#### Out of scope
Chain-load implementation (BOOT). Export format libraries (STO). Foreign OS repair beyond restoring BootOrder and ESP files this install created.

#### Acceptance criteria
- [ ] Uninstall on a dual-boot fixture restores Windows Boot Manager (or the recorded previous loader) as default and removes JakeOS ESP entries, proven by EFI BootOrder and directory listing.
- [ ] User Collections export to a user-chosen volume in the documented standard formats before partitions are reclaimed.
- [ ] Abort before the destructive step leaves boot entries and partitions unchanged.
- [ ] A wipe-only (no dual-boot) machine's uninstall offers export then reclaim without claiming a foreign OS was restored.

#### Verification
- Integration: `installer:tests/uninstall_restore_bootloader` on `qemu-x86_64` with a Windows fixture.
- Manual: uninstall on the LAB Windows dual-boot disk on H-002 and boot Windows from firmware.

#### Evidence
- none

### INS-054 · Upgrade V3 installs to V4 with data preserved and rollback
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: INS-045, PKG-089, STO-079
- Baseline: §30, §31, §63
- Benchmarks: B-043
- Invariants: I-022

Upgrade V3 alpha installs through the update channel with user data preserved and rollback to V3 possible. PKG owns format migration; STO owns layout migration; REL qualifies artifacts; INS owns the client path. The sample size and success threshold live on the V4 gate and B-043, not in this prose.

#### Out of scope
Format migration (PKG-089). Partition layout version (STO-079). Qualification (REL-059).

#### Acceptance criteria
- [ ] A V3 fixture generation on H-001 and on one V4 Tier 1 machine applies the V4 channel update, boots V4, and retains Collection hashes from before the update.
- [ ] Rolling back to the V3 generation boots V3 with those Collections intact.
- [ ] B-043 V4 samples for this path are recorded; this task does not restate the rate.
- [ ] At least 100 community V3 installations have upgraded through the channel with user data preserved (the V4-G08 count).

#### Verification
- Integration: `installer:tests/v3_to_v4_upgrade` on `qemu-x86_64` and one V4 Tier 1 H-ID.
- Bench: B-043 on those H-IDs; target per register (V4 absolute).

#### Evidence
- none

### INS-055 · Prove full-disk encryption install with Secure Boot on Tier 1
- Type: build
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: BOOT-043, INS-027, INS-032, SEC-055
- Baseline: §51, §63
- Threats: T-008
- Invariants: I-073, I-074

Prove the installer with full-disk encryption and Secure Boot succeeds on every Tier 1 machine as the 1.0 exit and release-ceremony demo of a signed 1.0 image installed that way.

#### Out of scope
Secure Boot strategy (BOOT). FDE mechanism (SEC). Installer implementation (INS-027).

#### Acceptance criteria
- [ ] Installing the signed 1.0 image with FDE and Secure Boot on every 1.0 Tier 1 H-ID reaches a login-capable desktop.
- [ ] Each machine records Secure Boot on, measured-boot generation identity, and a saved recovery key with no cloud escrow.
- [ ] The release-ceremony demo on H-002 is this path.

#### Verification
- Integration: `installer:tests/fde_secureboot_tier1` on every 1.0 Tier 1 H-ID.
- Demo: signed 1.0 image installed with FDE and Secure Boot on H-002.

#### Evidence
- none

### INS-056 · Prove power-cut mid-update recovers the previous SystemGeneration
- Type: build
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: BOOT-025, INS-045, LAB-003, PKG-082
- Baseline: §30, §63
- Invariants: I-022

Prove on a Tier 1 desktop that pulling power during an update boots the previous SystemGeneration and that retrying the update completes. PKG atomic-commit is the unit test; this is the hardware demo.

#### Out of scope
QEMU power-cut unit (PKG-082). Bootloader dual-slot (BOOT-025). Fleet soak (LAB-025).

#### Acceptance criteria
- [ ] LAB PDU power-cut during apply on H-002 boots the previous generation without a keypress.
- [ ] Retrying the update on that machine activates N+1 and boots a desktop INS-006 would mark good.
- [ ] User-data hashes from before the cut match after recovery.

#### Verification
- Integration: `installer:tests/power_cut_update_hw` on `hw-h002`.
- Demo: release-ceremony power pull on H-002.

#### Evidence
- none
