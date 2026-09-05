# HW · Hardware enablement
- Prefix: HW
- Lead: none
- Baseline: §7, §9.1, §17, §32, §33, §55, §56.1, §57, §62, §69

<!-- roadmap:generated:begin summary -->
Tasks: 90 live, 2 done, 0 in-progress, 88 todo, 0 dropped. Ready: 4. Blocked: 84. Weighted: 1%.
<!-- roadmap:generated:end -->

## Scope

HW owns enablement of the constrained x86-64 target set: SKU selection and bring-up of Reference machines, retained Linux buses under native `Object<Device>`, HID input as `Capability<InputDevice>`, user-space driver residency and the Device DMA path, Bluetooth host and profiles, USB classes, docks and Thunderbolt authorisation, printing and scanning services, firmware update as a SystemGeneration history event, in-scope sensors, and the Hardware Compatibility List schema, probes and published known-good list. Native software never opens DRM, HID, USB or V4L2 device nodes. Inherited Linux drivers remain until a classification Decision and a dual-path period move a class.

## Out of scope

Lab procurement, racks, radio peers and soak calendars (LAB). Kernel fork, rebase and DRM module tracking (KRN). Compositor, Mesa and scanout (GFX). Input routing and focus (UIP). Thermal, frequency, battery and suspend policy (PWR). Wi-Fi association and DHCP (NET). Audio mix and Bluetooth codecs (AUD). Camera service and `Capability<Camera>` (MED). Supervisor and driver hosting (SVC). MemoryObject DMA properties (MEM). Capability encoding (CAP). Object registry and Native ABI (ABI). UserSelected volumes (STO). Print and scan dialog chrome (APP). Identity matching for fingerprint and smartcard (SEC). ESP capsule staging and PCR re-seal (BOOT). History log storage (PKG). HCL website and store hosting (REL). Firmware redistribution and vendor NDA policy (GOV). Benchmark methodology (BEN). CI runners (BLD).

## Tasks

### HW-001 · Bring up the V0 AMD desktop Reference machine to a booting kernel
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-003, LAB-003, BOOT-002
- Baseline: §59, §62
- Risks: R-022
- Invariants: I-074

V0 hardware scope is QEMU plus one designated AMD desktop. LAB racks H-002 and BOOT boots the CI-built image. This task records the SKU, firmware version, IOMMU, TPM 2.0 and Secure Boot enrolment posture required by HW-003, and archives a passing boot log.

<!-- covers: INV-1154 -->

#### Out of scope
Racking and capture (LAB-003). Kernel boot image (BOOT-002). Compositor GPU path (HW-010).

#### Acceptance criteria
- [ ] `registers/hardware.md` entry H-002 names the procured SKU, firmware version, and IOMMU, TPM 2.0 and Secure Boot enrolment state.
- [ ] A boot log from H-002 of the tagged CI image is committed under Evidence and shows the kernel reaching userspace.
- [ ] The kernel log on H-002 lists the PCI root complex and USB host controllers; no inventoried bus is missing versus the QEMU image.

#### Verification
- Integration: boot-log capture on CI matrix entry `hw-h002`.
- Review: HW lead confirms H-002 fields match the accepted target-hardware Decision.
- Manual: serial console on H-002 shows the same kernel version as the QEMU CI image.

#### Evidence
- none

### HW-002 · Decide pragmatic driver residency over microkernel purity
- Type: adr
- Milestone: V0
- Status: done
- Size: S
- Owner: @agent/claude
- Depends on: none
- Baseline: §33, §55, §57
- Decision: D-0122
- Invariants: I-008, I-054, I-097
- Verified by: @jakebarnby

§33 and §55 forbid microkernel purity and flag-day driver replacement. Hardware support wins when it conflicts with native work (§57). This Decision records that residency is incremental per class and measured, so later classification and framework tasks have a written non-goal rather than prose.

<!-- covers: INV-0622, INV-1044, INV-1055, INV-1128, INV-0612 -->

#### Out of scope
Per-class keep, move or rewrite criteria (HW-016). Access mechanism (HW-006).

#### Acceptance criteria
- [x] Options evaluated include all drivers in user space, all drivers in-kernel, and pragmatic residency by measured cost per class.
- [x] The accepted option states that no driver class is replaced on a flag day and that hardware support wins until a later Decision names a class to move.
- [x] A Review line names who accepts the Decision.

#### Verification
- Review: kernel architecture and HW leads sign off on the pull request that accepts the Decision file.

#### Evidence
- decision:D-0122

### HW-003 · Decide the V1 through V2 Reference machine list and security criteria
- Type: adr
- Milestone: V0
- Status: done
- Size: M
- Owner: @agent/claude
- Depends on: none
- Baseline: §1, §62
- Decision: D-0129
- Risks: R-067, R-022
- Invariants: I-001, I-074, I-095
- Verified by: @jakebarnby

GPU, driver, energy and latency spikes need a fixed reference set before they run. This Decision names the AMD desktop, Intel laptop and AMD laptop SKUs for V1 through V2, records x86-64-only shipping, and treats Secure Boot key enrolment, TPM 2.0 and IOMMU as selection criteria. It does not promise universal PC compatibility.

<!-- covers: GAP-0550, INV-1230, INV-1231, INV-1232, GAP-0211, INV-0001, INV-1233 -->

#### Out of scope
Lab site and racking (LAB-002, LAB-003). Public vendor purchase ranking (HW-044). NVIDIA SKU (HW-018).

#### Acceptance criteria
- [x] Options evaluated include naming three SKUs now with IOMMU, TPM 2.0 and Secure Boot enrolment required, naming families without SKUs, and an unconstrained PC list.
- [x] The accepted option lists one AMD desktop, one Intel laptop and one AMD laptop and states that 1.0 ships x86-64 only.
- [x] The accepted option records IOMMU, TPM 2.0 and Secure Boot custom-key enrolment as selection criteria and that unlisted hardware is unsupported.
- [x] A Review line names who accepts the Decision.

#### Verification
- Review: HW, LAB and SEC leads sign off on the pull request that accepts the Decision file.

#### Evidence
- decision:D-0129

### HW-004 · Inventory retained Linux buses, firmware load and input in the fork
- Type: docs
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: HW-002, KRN-017
- Baseline: §2, §5.1, §55
- Invariants: I-010, I-054

Collapse PCIe, USB, ACPI, input, Bluetooth, firmware loading, power-management hooks and hardware discovery into one inventory that feeds KRN's retained-mechanism list. No driver class is replaced here. The inventory names the native object each bus will back once HW-008 lands.

<!-- covers: INV-0016, INV-0017, INV-0020, INV-0021, INV-0025, INV-0027, INV-0033, INV-0069, INV-0125, INV-0126, INV-0127, INV-0131, INV-0132, INV-0134, INV-0135, INV-0136, INV-1327 -->

#### Out of scope
Merge-blocking regression runs (HW-005). DRM inventory (GFX-001). Whole-kernel retained list (KRN-017).

#### Acceptance criteria
- [ ] The inventory lists PCIe, USB, ACPI, input, Bluetooth, firmware load, power-management hooks and hardware discovery with the native object or later rung each backs.
- [ ] The inventory is cited as the hardware row of KRN-017.
- [ ] The inventory states that native software does not open sysfs or device nodes as a native API.

#### Verification
- Review: KRN and HW leads record inventory sign-off on the pull request.
- Unit: `hw:tests/inventory/retained_buses_*` on CI matrix entry `qemu-x86_64`.

#### Evidence
- none

### HW-005 · Gate retained PCIe, USB, ACPI and input regressions on the AMD desktop
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-004, HW-001, KRN-014
- Baseline: §5.1, §55
- Risks: R-013
- Invariants: I-054, I-098

Second of the two retain tasks: kselftest and boot-time enumeration checks for the inventoried buses on H-001 and H-002. A native feature that regresses a retained driver is rejected. Bluetooth and laptop power are presence checks until later rungs.

<!-- covers: INV-0016, INV-0017, INV-0020, INV-0021, INV-0025, INV-0027, INV-0033, INV-0069, INV-0125, INV-0126, INV-0127, INV-0131, INV-0132, INV-0134, INV-0135, INV-0136, INV-1327 -->

#### Out of scope
Matrix contents for DRM, NVMe and networking (KRN-014). Bluetooth profiles (HW-037).

#### Acceptance criteria
- [ ] CI on `qemu-x86_64` and `hw-h002` runs kselftest subsets for PCI, USB, ACPI and input named by the inventory and fails the merge on a regression.
- [ ] A native feature that removes an inventoried bus without an accepted classification Decision is rejected by the same job.
- [ ] Bluetooth controller and ACPI power objects are asserted present on H-002 without requiring pairing or suspend success.

#### Verification
- Integration: `hw:tests/retain/pcie_usb_acpi_input_*` on `qemu-x86_64` and `hw-h002`.
- Review: KRN lead confirms this matrix is a row of KRN-014, not a second runner.

#### Evidence
- none

### HW-006 · Decide user-space driver access: VFIO, UIO or native Device DMA
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-013, HW-014
- Baseline: §33, §17
- Decision: D-0120
- Threats: T-020
- Invariants: I-038

INV-0623 must precede the V1 user-space driver framework. Options are reuse of VFIO or UIO-style interfaces, a native Device Capability with IOMMU-protected MemoryObject DMA, or a hybrid. The USB HID spike supplies measured cost.

<!-- covers: INV-0623 -->

#### Out of scope
IOMMU required-or-not (HW-017). Framework implementation (HW-029).

#### Acceptance criteria
- [ ] Options evaluated include VFIO/UIO-style interfaces, a native Device Capability with IOMMU-protected MemoryObject DMA, and a hybrid of the two.
- [ ] The accepted option cites the HW-014 report for interrupt latency and throughput and names which path native drivers use at V1.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: HW and kernel architecture leads sign off on the pull request that accepts the Decision file.

#### Evidence
- none

### HW-007 · Decide Capability<InputDevice> rights with no ambient device nodes
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: HW-013, CAP-007
- Baseline: §9.1, §7
- Decision: D-0123
- Threats: T-001
- Invariants: I-021

Native applications must not receive ambient HID streams. This Decision names how `Capability<InputDevice>` is minted and attenuated so the V0.5 HID service can grant keyboard and mouse without device nodes. Required by V0.5-G02 (Native application window, input and declarative UI).

#### Out of scope
HID service implementation (HW-011). Focus routing (UIP-005). Ownership split document (UIP-009).

#### Acceptance criteria
- [ ] Options evaluated include per-seat focused-surface only, per-device Capability with attenuable grab rights, and a privileged input broker.
- [ ] The accepted option states that native software is not granted a device node and that a Component without `Capability<InputDevice>` receives `Error::Rights`.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: CAP and HW leads sign off on the pull request that accepts the Decision file.

#### Evidence
- none

### HW-008 · Define Object<Device> and mint Capability<Device> from enumeration
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-013, HW-009, HW-007, ABI-005
- Baseline: §7, §9.1
- Threats: T-001
- Invariants: I-021, I-040

§7 `Object<Device>` is the hardware handle the V0.5 input path and later user-space drivers hold. The surface stays prototyped. Enumeration mints a Capability; there is no ambient device-node grant.

<!-- covers: INV-0170 -->

#### Out of scope
DeviceOperation kind (TSK-031). User-space DMA (HW-026). HAL to inherited drivers (HW-019).

#### Acceptance criteria
- [ ] Enumeration of a PCI or USB device on H-001 and H-002 mints `Capability<Device>` whose type and rights are visible to `os inspect`.
- [ ] A Component that does not hold that Capability and opens no Device handle receives `Error::Rights` and allocates no handle.
- [ ] Native tests do not open a device node. The Linux personality still can.

#### Verification
- Unit: `hw:tests/device/mint_rights_*` on `qemu-x86_64`.
- Integration: enumeration to Capability on `qemu-x86_64` and `hw-h002`.
- Review: ABI lead records the surface as prototyped, not frozen.

#### Evidence
- none

### HW-009 · Enumerate PCI, USB and ACPI devices into Object<Device> handles
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-004, HW-001
- Baseline: §2, §7, §69

Native device discovery over retained Linux buses so `Object<Device>` exists for the compositor input path and later drivers. Preserves §2 hardware discovery. Does not replace sysfs for the Linux personality.

<!-- covers: INV-1326, INV-0069 -->

#### Out of scope
Capability mint and rights (HW-008). Sysfs compatibility (LNX). GPU scanout (GFX).

#### Acceptance criteria
- [ ] Boot on H-001 and H-002 produces a Device list covering PCI, USB and ACPI objects named by the retained inventory.
- [ ] `os inspect device` lists bus, vendor and product identifiers without exposing a native device-node path.
- [ ] Re-enumeration after a USB hot-plug on H-002 adds a Device and unplug removes it.

#### Verification
- Integration: `hw:tests/enum/pci_usb_acpi_*` on `qemu-x86_64` and `hw-h002`.
- Unit: `hw:tests/enum/hotplug_usb_*` on `qemu-x86_64`.

#### Evidence
- none

### HW-010 · Enable amdgpu on the AMD desktop Reference machine for composition
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-001, HW-009, GFX-001
- Baseline: §39, §56.1, §57
- Invariants: I-045

V0.5 compositor drives the reference GPU via DRM/KMS. This is kernel amdgpu plus RADV/RadeonSI bring-up on H-002. GFX owns compositor abstractions. Native software never issues DRM ioctls. The laptop half of INV-1061 waits for HW-039.

<!-- covers: INV-1061 -->

#### Out of scope
Compositor Surfaces (GFX). Mesa mediation (GFX). Laptop APU (HW-039). Native GPU driver stack (I-045).

#### Acceptance criteria
- [ ] H-002 loads amdgpu and enumerates the discrete GPU as a Device.
- [ ] A compositor acceleration check on H-002 presents a frame via the inherited DRM path without a native application opening a DRM node.
- [ ] The retained DRM regression matrix stays green on H-002 after this enablement.

#### Verification
- Integration: `hw:tests/gpu/amdgpu_h002_*` on `hw-h002`.
- Review: GFX lead confirms no native DRM API was added.

#### Evidence
- none

### HW-011 · Build the HID input service minting Capability<InputDevice>
- Type: build
- Milestone: V0.5
- Status: todo
- Size: L
- Owner: none
- Depends on: HW-008, HW-007, HW-009
- Baseline: §9.1, §60
- Threats: T-001
- Invariants: I-021

V0.5 exit: a native application receives keyboard and mouse. This service enables HID devices and mints per-device Capabilities. UIP routes events to the focused Surface. Native software never sees `/dev/input`. The Linux personality keeps evdev.

<!-- covers: EXTRA-042 -->

#### Out of scope
Focus and hit-test (UIP-012, GFX-031). I2C laptop touchpad (HW-024). Layout data (HW-027).

#### Acceptance criteria
- [ ] A native Component holding `Capability<InputDevice>` for the attached keyboard and mouse on H-002 receives key and pointer events through the HID service.
- [ ] A native Component without that Capability receives `Error::Rights` and observes no HID stream.
- [ ] Native tests do not open `/dev/input`. A Linux-personality process on the same machine still can via evdev.
- [ ] `os inspect` lists the InputDevice Capabilities and their holders.

#### Verification
- Integration: `hw:tests/hid/keyboard_mouse_*` on `qemu-x86_64` and `hw-h002`.
- Unit: `hw:tests/hid/rights_denial_*` on `qemu-x86_64`.
- Demo: V0.5 native application types and clicks on H-002.

#### Evidence
- none

### HW-012 · Re-issue InputDevice Capabilities on HID hot-plug and unplug
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-011
- Baseline: §32, §60

V0.5 keyboard and mouse must survive unplug and replug without restarting the compositor. Device-management restart of INV-0598 is V1. This is the input-only hot-plug path the four demo apps need. Required by the HW scope: "HID input as `Capability<InputDevice>`".

#### Out of scope
User-space driver rebind (HW-030). Compositor crash rebind (GFX, SVC).

#### Acceptance criteria
- [ ] Unplugging the USB mouse on H-002 revokes its InputDevice Capability; holders observe a disconnect and allocate no events from it.
- [ ] Replugging mints a new Capability; a focused native application receives pointer events without a compositor restart.
- [ ] The same unplug/replug loop passes for a USB keyboard on H-002.

#### Verification
- Integration: `hw:tests/hid/hotplug_*` on `hw-h002` and USB HID unplug in `qemu-x86_64`.
- Demo: V0.5 Text Editor keeps its buffer while the USB keyboard is replugged on H-002.

#### Evidence
- none

### HW-013 · Spike Object<Device> as a Layer 1 Capability-gated hardware handle
- Type: spike
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-002, ABI-013
- Baseline: §7, §33, §65
- Risks: R-007
- Invariants: I-040

`Object<Device>` is Layer 1, so this spike lands in V0, implementation stays prototyped at V0.5, and freeze waits until V4. The report records handle shape, rights bits and IOMMU placement for the three access options later decided by HW-006.

<!-- covers: INV-0170, INV-0623 -->

#### Out of scope
Minting production handles (HW-008). Access-mechanism Decision (HW-006). L1 freeze.

#### Acceptance criteria
- [ ] The report prototypes handle layout, rights and IOMMU domain placement for VFIO-style, UIO-style and native Device Capability options.
- [ ] The report states that the surface remains prototyped through V0 and is not frozen.
- [ ] The report is committed at `reports/spikes/HW-013.md`.

#### Verification
- Report: handle shape; rights word; where the IOMMU domain sits relative to the Capability; which option HW-008 may prototype; what remains open for HW-006.
- Review: ABI and HW leads record that no L1 freeze is proposed.

#### Evidence
- none

### HW-014 · Spike a USB HID user-space driver against the in-kernel path
- Type: spike
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-002, HW-001
- Baseline: §33, §54
- Invariants: I-061

§33 requires a measured prototype of one USB HID or USB audio user-space driver versus the inherited in-kernel driver before the access-mechanism adr and V1 framework. The report publishes interrupt latency and throughput. No numeric target appears in prose.

<!-- covers: INV-0624 -->

#### Out of scope
Access-mechanism Decision (HW-006). Production HID class move (HW-031). Audio class (AUD-019).

#### Acceptance criteria
- [ ] The report measures one USB HID device on H-002 in user space and on the in-kernel path using the same method.
- [ ] The report names interrupt latency and throughput series without stating a superiority claim.
- [ ] The report is committed at `reports/spikes/HW-014.md`.

#### Verification
- Report: which USB class was prototyped; interrupt latency and throughput versus in-kernel; whether VFIO, UIO or a native Device handle was used; what HW-006 may assume.
- Review: BEN lead confirms no numeric target was written outside a report.

#### Evidence
- none

### HW-015 · Bring up the V1 Intel laptop Reference machine for daily-driving
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: HW-003, LAB-007
- Baseline: §61, §62
- Risks: R-027, R-040
- Invariants: I-074

V1 hardware scope is the designated Intel laptop (iGPU, Intel Wi-Fi, NVMe, internal plus one USB-C display). LAB procures at V0.5. This bring-up records SKU, firmware, IOMMU/TPM/Secure Boot posture and a passing boot plus HID, GPU and USB enumeration on H-004 so PWR, NET and AUD gates have a machine.

#### Out of scope
Wi-Fi association (NET-021). Suspend cycles (PWR-014). i915 acceleration check (HW-022). Touchpad (HW-024).

#### Acceptance criteria
- [ ] H-004 in `registers/hardware.md` names SKU, firmware, IOMMU, TPM 2.0 and Secure Boot enrolment.
- [ ] A CI-built image boots on H-004 and enumerates internal keyboard, USB, NVMe and the iGPU as Device objects.
- [ ] Internal panel plus one USB-C display are electrically present (KMS connectors listed) even if GFX mode-set is a sibling task.
- [ ] Boot log and `os inspect device` output are committed under Evidence.

#### Verification
- Integration: boot and enumeration on CI matrix entry `hw-h004`.
- Review: HW lead confirms H-004 matches the target-hardware Decision.
- Manual: lid, power button and internal keyboard produce events on H-004.

#### Evidence
- none

### HW-016 · Decide criteria classifying each driver as inherited, native or rewritten
- Type: adr
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-002, HW-014
- Baseline: §33, §55
- Decision: D-0121
- Invariants: I-008, I-097

Merges the two §33/§55 classification adrs: latency, DMA safety, interrupt performance, isolation value, inherited-driver compatibility and maintenance cost. Options per class are keep in-kernel, move to user-space, or rewrite in-kernel. Feeds the driver status registry.

<!-- covers: INV-0621, INV-1054 -->

#### Out of scope
Registry publication (HW-021). Bluetooth placement (HW-040). Remaining-class spike (HW-055).

#### Acceptance criteria
- [ ] Options evaluated per class include keep in-kernel, move to user-space, and rewrite in-kernel, using latency, DMA safety, interrupt performance, isolation value, inherited compatibility and maintenance cost.
- [ ] The accepted option names the criteria and the dual-path period required before an inherited class is removed.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: kernel architecture and HW leads sign off on the pull request that accepts the Decision file.

#### Evidence
- none

### HW-017 · Decide whether IOMMU is required for user-space drivers and DMA
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: HW-003, HW-006
- Baseline: §33, §17
- Decision: D-0124
- Threats: T-020, T-024
- Invariants: I-038, I-074

Without IOMMU a user-space driver can DMA over the kernel. Target SKUs already require IOMMU via GAP-0211. This Decision records whether that is a hard requirement for all hardware, required only for user-space DMA drivers, or software-only isolation with documented degradation.

<!-- covers: GAP-0536, INV-0627 -->

#### Out of scope
Degradation tests (HW-025). DMA MemoryObject wiring (HW-026). Thunderbolt authorisation (HW-057).

#### Acceptance criteria
- [ ] Options evaluated include IOMMU required on all target hardware, required only for user-space DMA drivers, and software-only isolation with documented degradation.
- [ ] The accepted option states what happens when the IOMMU is off, including whether user-space DMA is refused.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: HW and SEC leads sign off on the pull request that accepts the Decision file.

#### Evidence
- none

### HW-018 · Decide NVIDIA support and Secure Boot handling of proprietary modules
- Type: adr
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-003, GFX-047
- Baseline: §56.1
- Decision: D-0125
- Risks: R-037, R-050

Critique places the NVIDIA adr at V1 so V2 experimental and V3 Tier 1 bring-up are not research. Options: open modules plus NVK, vendor-signed proprietary module, machine-owner MOK signing, or defer past 1.0. Coordinates with KRN module signing and GFX compositor stance.

<!-- covers: INV-1063, GAP-0183 -->

#### Out of scope
Compositor 1.0 stance (GFX-064). Experimental desktop bring-up (HW-052). Module-signature enforcement (KRN-038).

#### Acceptance criteria
- [ ] Options evaluated include open modules plus NVK, vendor-signed proprietary module, machine-owner MOK signing, and deferral past 1.0.
- [ ] The accepted option states how the chosen path loads under Secure Boot and which generations are in scope at V2 experimental and V3 Tier 1.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: HW, GFX and KRN leads sign off on the pull request that accepts the Decision file.

#### Evidence
- none

### HW-019 · Bridge native Device objects to inherited Linux drivers
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: HW-008, HW-009, HW-010
- Baseline: §69, §3
- Invariants: I-005, I-045

§69 hardware abstraction: native `Object<Device>` over retained in-kernel drivers so native software never opens DRM, USB or HID nodes. Personality software keeps the inherited nodes. Required before GPU-accelerated Linux apps at V1 share hardware with native Components.

<!-- covers: INV-1326 -->

#### Out of scope
Personality node policy (Q-028, LNX). User-space driver framework (HW-029). Mesa (GFX).

#### Acceptance criteria
- [ ] A native Component performs a Device Operation on the H-002 GPU and on a USB HID device without opening a device node.
- [ ] A Linux-personality process on the same machine still opens the inherited DRM and evdev nodes.
- [ ] Concurrent native and personality holders of the same physical device are visible in `os inspect` with distinct Capabilities.
- [ ] Native crates that link a device-node path fail the ABI firewall lint.

#### Verification
- Integration: `hw:tests/hal/native_vs_personality_*` on `hw-h002` and `qemu-x86_64`.
- Unit: `hw:tests/hal/no_devnode_native_*` on `qemu-x86_64`.
- Review: LNX lead records that inherited nodes remain for personality software.

#### Evidence
- none

### HW-020 · Lint-forbid flag-day replacement of any inherited driver class
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: HW-002, HW-016
- Baseline: §55
- Invariants: I-097

INV-1055 as a CI lint: a change that removes an inherited driver class without an accepted classification Decision and a dual-path period is rejected. Enforces HW-002 on every merge.

<!-- covers: INV-1055 -->

#### Out of scope
Classification criteria (HW-016). Per-device registry rows (HW-021).

#### Acceptance criteria
- [ ] CI rejects a tree that deletes an inventoried inherited driver class without a linked accepted Decision and a dual-path flag.
- [ ] The lint allowlist is empty at land unless a Decision names the class and the dual-path period.
- [ ] The check runs on every kernel and hardware-enablement merge.

#### Verification
- Unit: `hw:tests/lint/no_flag_day_*` on host CI with a fixture that deletes a class.
- Review: KRN lead confirms the lint is merge-blocking.

#### Evidence
- none

### HW-021 · Publish the driver status registry of class and test coverage per device
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-016
- Baseline: §55
- Invariants: I-097

INV-1058: each supported device records inherited-Linux, userspace-native or rewritten, plus test coverage. Feeds the Hardware Compatibility List. Regenerated in CI from the three-class model.

<!-- covers: INV-1058 -->

#### Out of scope
HCL schema (HW-047). Three-class narrative (HW-056).

#### Acceptance criteria
- [ ] A generated registry lists every Device on H-002 and H-004 with class (inherited, userspace-native, rewritten) and named test coverage.
- [ ] CI fails if a Device enumerated on those machines has no registry row.
- [ ] The registry is an input to later HCL rows, not a second support vocabulary.

#### Verification
- Integration: registry generation on `hw-h002` and `hw-h004`.
- Unit: `hw:tests/registry/missing_row_*` on host CI.

#### Evidence
- none

### HW-022 · Enable i915/xe and ANV/Iris on the Intel laptop Reference machine
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-015, HW-019
- Baseline: §56.1, §61
- Invariants: I-045

V1 hardware scope and GPU-accelerated Linux browser/IDE gates need Intel GPU support on H-004. GFX owns compositor and Mesa mediation. HW owns kernel driver bring-up and a passing acceleration check. Native software never opens a DRM node.

<!-- covers: INV-1062 -->

#### Out of scope
Panel mode-set and USB-C display (GFX-053). Personality GPU sharing policy (LNX).

#### Acceptance criteria
- [ ] H-004 loads i915 or xe and enumerates the iGPU as a Device.
- [ ] A native acceleration check presents a frame; a Linux-personality client also presents a frame on the same GPU.
- [ ] Native tests do not open a DRM node.

#### Verification
- Integration: `hw:tests/gpu/intel_h004_*` on `hw-h004`.
- Review: GFX lead confirms Mesa is the userspace and no native DRM API was added.

#### Evidence
- none

### HW-023 · Audit linux-firmware blobs and set ship, download-on-demand or exclude
- Type: docs
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-003, HW-015, GOV-022
- Baseline: §5.1, §62

Pulled from V2 to V1 because daily-drive images ship firmware. Per-blob redistribution terms for blobs used on H-002 and H-004: ship in the image, download-on-demand, or exclude. GOV licence scanning is the CI net. This is the HW policy for target machines.

<!-- covers: GAP-0015 -->

#### Out of scope
Official versus non-free channel (GOV-022). Vendor NDA (GOV-026). LVFS updates (HW-046).

#### Acceptance criteria
- [ ] Every linux-firmware blob referenced by H-002 and H-004 is listed with ship, download-on-demand, or exclude and a licence note.
- [ ] The V1 daily-drive image contains only blobs marked ship.
- [ ] A blob marked exclude is absent from the image and documented as a degraded Device.

#### Verification
- Review: GOV licensing and HW leads sign off on the policy table.
- Integration: image-contents check against the ship list on the V1 image build.

#### Evidence
- none

### HW-024 · Enable I2C HID touchpads on the Intel laptop Reference machine
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-015, HW-011
- Baseline: §61, §62

V1 daily-driving on H-004 fails without the internal touchpad. HID-over-I2C is the usual laptop path. Pointer events go through HW-011. Gestures wait for HW-058.

<!-- covers: INV-1219 -->

#### Out of scope
Multi-finger gestures (HW-058). Focus routing (UIP).

#### Acceptance criteria
- [ ] The internal touchpad on H-004 enumerates as `Capability<InputDevice>` and produces pointer events in a native application.
- [ ] Unloading does not require a compositor restart; `os inspect` shows the Device.
- [ ] Native tests do not open `/dev/input`.

#### Verification
- Integration: `hw:tests/hid/i2c_touchpad_h004_*` on `hw-h004`.
- Manual: pointer motion and click on H-004 internal touchpad.

#### Evidence
- none

### HW-025 · Define and test user-space driver degradation when IOMMU is absent
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-017, HW-029
- Baseline: §33, §17
- Threats: T-020, T-024
- Invariants: I-038

GAP-0536 requires explicit degradation on systems without IOMMU. Target hardware has IOMMU. This task records the refuse-or-kernel-only behaviour and a test that user-space DMA is not enabled when the IOMMU is off.

<!-- covers: GAP-0536 -->

#### Out of scope
Protected DMA path (HW-026). Thunderbolt policy (HW-057).

#### Acceptance criteria
- [ ] A QEMU or firmware configuration with IOMMU off refuses user-space DMA with a typed error and does not map a Device into an IOMMU domain.
- [ ] `os inspect` reports the Device as kernel-only or unavailable per the accepted Decision.
- [ ] The in-kernel HID path still enumerates on that configuration.

#### Verification
- Integration: `hw:tests/iommu/off_degradation_*` on a QEMU configuration with vIOMMU disabled.
- Review: SEC lead confirms user-space DMA cannot be enabled in the off case.

#### Evidence
- none

### HW-026 · Implement IOMMU-protected DMA MemoryObjects for user-space drivers
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: HW-017, HW-006, HW-008, MEM-037, MEM-028
- Baseline: §17, §33
- Threats: T-024
- Invariants: I-038

§17 DMA-suitable MemoryObjects are unsafe against malicious peripherals without IOMMU. Implements the chosen access path so a user-space driver can DMA only into MemoryObjects it holds. MEM owns MemoryObject properties. HW wires Device to the IOMMU domain.

<!-- covers: GAP-0536, INV-0627 -->

#### Out of scope
MemoryObject property bits (MEM). Thunderbolt pre-authorisation (HW-057). Degradation when off (HW-025).

#### Acceptance criteria
- [ ] A user-space driver holding `Capability<Device>` maps a DMA MemoryObject only into that Device's IOMMU domain on H-002.
- [ ] DMA to a MemoryObject the driver does not hold is rejected with `Error::Rights` and no IOMMU mapping is created.
- [ ] `os inspect` shows the Device, the IOMMU domain and the mapped MemoryObjects.
- [ ] Native software never programs the IOMMU through a device node.

#### Verification
- Integration: `hw:tests/iommu/protected_dma_*` on `hw-h002`.
- Unit: `hw:tests/iommu/rights_denial_*` on `qemu-x86_64` with vIOMMU.
- Review: MEM lead confirms properties stay in MemoryObject.

#### Evidence
- none

### HW-027 · Implement XKB-data keyboard layouts, compose, dead keys and switching
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-011, SVC-025
- Baseline: §41, §61
- Invariants: I-021

V1 daily-driving needs non-US layouts from day one. Layout data lives in HW. UIP routes keys. APP owns the shell switcher UI. Reuses xkeyboard-config. Per-window versus global policy is configured here and consumed by UIP.

<!-- covers: GAP-0254 -->

#### Out of scope
Switcher chrome (APP). IME engines (TXT). Locale data source (TXT, SVC-025).

#### Acceptance criteria
- [ ] Selecting a non-US layout from xkeyboard-config produces the corresponding keysyms for the focused native application on H-004.
- [ ] Compose and dead keys produce the documented character for that layout.
- [ ] Per-window and global policy bits are stored in the locale settings service and read by UIP; a layout change does not grant a global key-grab Capability to apps.

#### Verification
- Integration: `hw:tests/hid/layout_xkb_*` on `qemu-x86_64` and `hw-h004`.
- Review: UIP lead confirms the service emits layout events, not grabs.

#### Evidence
- none

### HW-028 · Spike a Bluetooth controller in user space versus the in-kernel host
- Type: spike
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-014, HW-006, HW-017, HW-015
- Baseline: §33, §54
- Risks: R-039
- Invariants: I-061

GAP-0537 plus §33: measure interrupt latency and throughput of one Bluetooth controller in user space (VFIO or Device Capability) against the in-kernel host before the V2 Bluetooth stack adr. USB HID is already covered by HW-014.

<!-- covers: GAP-0537 -->

#### Out of scope
Stack placement Decision (HW-040). Profiles (HW-037). Audio latency publication (B-029).

#### Acceptance criteria
- [ ] The report measures one Bluetooth controller on H-004 in user space and in-kernel using the same method.
- [ ] The report names interrupt latency and throughput series without a superiority claim.
- [ ] The report is committed at `reports/spikes/HW-028.md`.

#### Verification
- Report: controller and transport used; VFIO versus Device Capability; interrupt latency and throughput versus in-kernel; what HW-040 may assume.
- Review: BEN lead confirms numbers live only in the report.

#### Evidence
- none

### HW-029 · Build the user-space driver framework over Capability<Device>
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: HW-006, HW-017, HW-008, HW-026, HW-016
- Baseline: §33, §55
- Threats: T-020
- Invariants: I-008, I-038

§33/§55 framework: `Object<Device>` Capabilities, interrupt delivery as Operation completions, and DMA-capable MemoryObjects. Drivers live outside the kernel where HW-016 says the cost is acceptable. SVC hosts the Components.

<!-- covers: INV-0613, INV-1056, INV-0612 -->

#### Out of scope
Supervisor (SVC-022). First HID class (HW-031). Bluetooth host (HW-035).

#### Acceptance criteria
- [ ] A user-space driver Component receives interrupts as Operation completions and DMA MemoryObjects through `Capability<Device>` on H-002.
- [ ] A driver without the Capability allocates no mapping and receives `Error::Rights`.
- [ ] `os inspect` lists the driver Component, Device and outstanding Operations.
- [ ] Native software does not open VFIO or UIO device nodes unless the accepted access Decision named those nodes as a personality-only path.

#### Verification
- Integration: `hw:tests/usd/framework_*` on `hw-h002` and `qemu-x86_64`.
- Unit: `hw:tests/usd/rights_denial_*` on `qemu-x86_64`.
- Review: SVC lead confirms hosting stays in SVC.

#### Evidence
- none

### HW-030 · Restart crashed user-space drivers and re-issue Device Capabilities
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-029, HW-031
- Baseline: §32, §33
- Invariants: I-037

§32 device-management services are restartable. §33 isolates driver Component crashes under SVC supervision and re-issues Capabilities to holders. V1 daily-driving must not reboot for a USB HID driver panic.

<!-- covers: INV-0626, INV-0598 -->

#### Out of scope
Supervisor budgets (SVC). Bluetooth reconnect (HW-038).

#### Acceptance criteria
- [ ] Killing the USB HID user-space driver on H-004 restarts it under the supervisor and re-issues InputDevice Capabilities; a native editor keeps its buffer.
- [ ] `os inspect` shows the restart count increment and the new Capability identities.
- [ ] The machine does not reboot during a 20-kill loop on `hw-h004`.

#### Verification
- Integration: `hw:tests/usd/rebind_hid_*` on `hw-h004` with BLD fault injection.
- Demo: USB keyboard driver kill on H-004; typing resumes without reboot.

#### Evidence
- none

### HW-031 · Ship selected USB HID classes as user-space drivers
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-029, HW-014, HW-011, HW-016
- Baseline: §33
- Invariants: I-097

INV-0615: selected USB classes move to user-space where the spike showed acceptable cost. First class is HID (keyboard, mouse) on H-004 using the V1 framework. Remaining USB classes wait for HW-075 at V3.

<!-- covers: INV-0615 -->

#### Out of scope
Remaining USB classes (HW-075). In-kernel HID fallback when the Decision keeps a device in-kernel.

#### Acceptance criteria
- [ ] USB keyboard and mouse on H-004 are served by a user-space driver Component when the classification Decision named HID as moved.
- [ ] If the Decision kept HID in-kernel, this task records that outcome in the driver status registry and ships no user-space HID path.
- [ ] Native applications still receive events only through `Capability<InputDevice>`.

#### Verification
- Integration: `hw:tests/usd/usb_hid_h004_*` on `hw-h004`.
- Review: classification Decision is cited in the registry row.

#### Evidence
- none

### HW-032 · Verify keyboard, touchpad and USB HID on the Intel laptop Reference machine
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: HW-015, HW-024, HW-011, HW-012, HW-027, HW-030
- Baseline: §61

V1 daily-driving gate: core team uses H-004 as primary machine. Scripted HID scenarios (internal keyboard, I2C touchpad, USB mouse) must pass on that SKU before the milestone closes.

#### Out of scope
Gestures (HW-058). Layouts (HW-027).

#### Acceptance criteria
- [ ] Scripted internal keyboard, I2C touchpad and USB mouse scenarios pass on H-004.
- [ ] USB mouse unplug/replug passes without a compositor restart.
- [ ] Results are attached as Evidence for H-004.

#### Verification
- Integration: `hw:tests/hid/verify_h004_*` on `hw-h004`.
- Manual: daily-driving checklist signed by the HW lead on H-004.

#### Evidence
- none

### HW-033 · Publish Bluetooth pairing time and audio latency on target machines
- Type: benchmark
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: HW-037, HW-036, BEN-007, LAB-011, BEN-005, AUD-017, HW-035, LAB-018
- Baseline: §54, §62
- Benchmarks: B-029

V2 benchmark gate: Bluetooth audio latency and pairing time measured and published on the three target machines. Numbers live in B-029. This harness produces pairing and reconnect series. AUD owns codec round-trip on the same B-ID.

#### Out of scope
Register ownership (BEN-007). Audio path (AUD-017). Functional 95-of-100 cycles (HW-060).

#### Acceptance criteria
- [ ] Harness `bench:bluetooth` records pairing request to first audio and power-on to reconnect for headset, mouse and keyboard on H-004 and H-005.
- [ ] Reports exist under `reports/benchmarks/B-029/` for H-004 and H-005 meeting the register target kind for V2.
- [ ] No description, criterion or report states a superiority claim.

#### Verification
- Bench: B-029 on H-004 and H-005; target per register.
- Review: BEN lead confirms pairing series names match the register method.

#### Evidence
- none

### HW-034 · Publish user-space versus in-kernel driver latency and throughput
- Type: benchmark
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: HW-035, HW-031, BEN-007
- Baseline: §54, §55
- Benchmarks: B-029
- Invariants: I-061

INV-1057 measured performance parity for a migrated class. The harness compares the Bluetooth or USB HID user-space driver to the inherited in-kernel path on the same Reference machine. BEN owns the register target. The V2 migrated class is Bluetooth when HW-040 moved it; otherwise USB HID. Numbers live only in reports.

<!-- covers: INV-1057 -->

#### Out of scope
Register methodology (BEN). Stack Decision (HW-040).

#### Acceptance criteria
- [ ] The harness records interrupt latency and throughput for the migrated class on the user-space path and the in-kernel path on the same machine.
- [ ] A report exists under `reports/benchmarks/B-029/` (or the series BEN names for this comparison) for that machine meeting the V2 target kind.
- [ ] No superiority claim appears outside the report.

#### Verification
- Bench: B-029 on the machine that hosts the migrated class; target per register.
- Review: BEN lead confirms the comparison series is labelled user-space versus in-kernel.

#### Evidence
- none

### HW-035 · Run the Bluetooth host as a restartable user-space Component
- Type: build
- Milestone: V2
- Status: todo
- Size: L
- Owner: none
- Depends on: HW-040, HW-028, HW-029
- Baseline: §33, §55, §32
- Risks: R-039
- Invariants: I-097

§33/§55: migrate the Bluetooth host to user-space where the spike showed acceptable cost, demonstrating one inherited class moved with measured parity (HW-034). V2 exit: user-space Bluetooth crash recovers without reboot. If the Decision kept the in-kernel host, the registry records that and this Component is not shipped.

<!-- covers: INV-0614, INV-1057 -->

#### Out of scope
Profiles (HW-037). Supervisor (SVC). Audio routing (AUD-021).

#### Acceptance criteria
- [ ] When the Decision moved the host, the Bluetooth host runs as a supervised user-space Component on H-004 and H-005.
- [ ] Killing that Component does not reboot the machine; paired devices reconnect per HW-038.
- [ ] When the Decision kept the in-kernel host, the driver status registry row says inherited and no user-space host is started.
- [ ] Native software does not open a Bluetooth device node.

#### Verification
- Integration: `hw:tests/bt/host_component_*` on `hw-h004` and `hw-h005`.
- Review: classification Decision is cited in the registry row.

#### Evidence
- none

### HW-036 · Grant Capability<BluetoothDevice> from a pairing agent and chooser
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-040, HW-035, CAP-007
- Baseline: §9.1, §62
- Threats: T-001
- Invariants: I-021

BLE scanning is a location-tracking vector. Apps receive per-device Capabilities, never adapter-wide scan authority. APP and UIP render the chooser. HW owns pairing, bonding and the Capability.

<!-- covers: GAP-0298, INV-1217 -->

#### Out of scope
Chooser chrome (APP-041). Profiles after pairing (HW-037).

#### Acceptance criteria
- [ ] Completing pairing mints `Capability<BluetoothDevice>` for that device only.
- [ ] A native Component without that Capability cannot scan or receive BLE advertisements and gets `Error::Rights`.
- [ ] Adapter-wide scan is not granted to application Components.

#### Verification
- Integration: `hw:tests/bt/chooser_grant_*` on `hw-h004`.
- Unit: `hw:tests/bt/no_adapter_scan_*` on `qemu-x86_64`.

#### Evidence
- none

### HW-037 · Ship A2DP, HFP, HID, GATT, BLE privacy and file-transfer profiles
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-035, HW-036
- Baseline: §62
- Risks: R-039

V2 laptop day demo: headset A2DP/HFP, mouse and keyboard HID, file transfer. LE Audio/LC3 is included where hardware supports it. BLE privacy is on by default. AUD owns audio rendering. HW owns the host profiles.

<!-- covers: GAP-0297, INV-1217 -->

#### Out of scope
Audio mix and codecs (AUD-021). Pairing cycles gate (HW-060).

#### Acceptance criteria
- [ ] A paired headset on H-004 and H-005 negotiates A2DP and HFP.
- [ ] A paired mouse and keyboard produce HID events through `Capability<InputDevice>`.
- [ ] BLE privacy is enabled by default; a GATT connection and a file-transfer session complete on one target laptop.
- [ ] LE Audio/LC3 is enabled on hardware that advertises it and is listed as unsupported on hardware that does not.

#### Verification
- Integration: `hw:tests/bt/profiles_*` on `hw-h004` and `hw-h005`.
- Demo: V2 laptop-day headset, mouse and keyboard on H-005.

#### Evidence
- none

### HW-038 · Restart the Bluetooth service with paired devices reconnecting
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-035, HW-037
- Baseline: §32
- Invariants: I-037

§32 Bluetooth service restartable. V2 exit tests stack crash recovery without reboot. Paired headsets, mice and keyboards reconnect after supervisor restart.

<!-- covers: INV-0596 -->

#### Out of scope
Supervisor (SVC-034). Pairing time (B-029).

#### Acceptance criteria
- [ ] Killing the Bluetooth service on H-004 and H-005 reconnects a paired headset, mouse and keyboard without reboot.
- [ ] `os inspect` shows the restart count and rebound Capabilities.
- [ ] An in-flight A2DP stream resumes or fails with an explicit degraded event, never a silent hang.

#### Verification
- Integration: `hw:tests/bt/service_rebind_*` on `hw-h004` and `hw-h005`.
- Demo: V2 Bluetooth crash recovery without reboot on H-005.

#### Evidence
- none

### HW-039 · Bring up the AMD laptop Reference machine including amdgpu
- Type: build
- Milestone: V2
- Status: todo
- Size: L
- Owner: none
- Depends on: HW-003, LAB-018, HW-010
- Baseline: §56.1, §62
- Risks: R-040
- Invariants: I-074

V2 hardware scope: named AMD laptop (Zen 4-class APU, Wi-Fi 6, Bluetooth). Completes INV-1061 laptop GPU support. Lid, touchpad, Bluetooth and firmware quirks are the V2 HW risk the ladder names.

<!-- covers: INV-1061 -->

#### Out of scope
Racking (LAB-018). Suspend (PWR). Wi-Fi (NET). Lid policy (PWR-021).

#### Acceptance criteria
- [ ] H-005 in `registers/hardware.md` names SKU, firmware, IOMMU, TPM 2.0 and Secure Boot enrolment.
- [ ] The machine boots a CI image, enumerates amdgpu, internal HID, Bluetooth and USB, and presents the internal panel.
- [ ] Firmware quirks that affect HID, GPU or USB are listed in the driver status registry.

#### Verification
- Integration: boot and enumeration on `hw-h005`.
- Review: HW lead confirms H-005 matches the target-hardware Decision.
- Manual: lid switch, touchpad and internal keyboard on H-005.

#### Evidence
- none

### HW-040 · Decide Bluetooth host placement and required profiles
- Type: adr
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-028, HW-016
- Baseline: §33, §62
- Decision: D-0119
- Risks: R-039

V2 Bluetooth gate depends on this adr, fed by HW-028. Options: retain BlueZ in-kernel/host, move the host to a native Component per §33, or a hybrid. Profiles: A2DP, HFP, HID, LE Audio/LC3, GATT, BLE privacy.

<!-- covers: GAP-0297 -->

#### Out of scope
Profile implementation (HW-037). Audio codecs (AUD).

#### Acceptance criteria
- [ ] Options evaluated include retain BlueZ in-kernel/host, move the host to a native Component, and a hybrid.
- [ ] The accepted option lists required profiles (A2DP, HFP, HID, LE Audio/LC3 where hardware supports it, GATT, BLE privacy) and cites the spike report.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: HW and AUD leads sign off on the pull request that accepts the Decision file.

#### Evidence
- none

### HW-041 · Decide driverless-first native printing with PDF spool and CUPS in LNX
- Type: adr
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: none
- Baseline: §33
- Decision: D-0126

Must precede the V3 print service. Options: IPP Everywhere/driverless native service with PDF spool, CUPS as the native service, or CUPS only inside the Linux personality for legacy drivers. A print job is a typed Operation. APP owns dialog chrome.

<!-- covers: GAP-0267 -->

#### Out of scope
PDF renderer (APP-053). Print service (HW-071). CUPS socket (LNX-093).

#### Acceptance criteria
- [ ] Options evaluated include IPP Everywhere/driverless native service with PDF spool, CUPS as the native service, and CUPS only inside the Linux personality for legacy drivers.
- [ ] The accepted option states that a print job is a typed Operation and that apps do not receive ambient printer enumeration.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: HW and APP leads sign off on the pull request that accepts the Decision file.

#### Evidence
- none

### HW-042 · Decide 1.0 sensor support per device class in or out of scope
- Type: adr
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: HW-003
- Baseline: §62
- Decision: D-0127

Ambient-light auto-brightness, lid and tablet-mode switches, and accelerometer rotation are declared in or out per device class. Options: all three in, lid-only, or lid plus ALS. Convertible UI remains a UIP non-goal.

<!-- covers: EXTRA-037 -->

#### Out of scope
Sensor Components (HW-054). Brightness policy (PWR-018). Convertible shell (UIP).

#### Acceptance criteria
- [ ] Options evaluated include all three classes in for 1.0, lid-only, and lid plus ALS.
- [ ] The accepted option names each class in or out per desktop and laptop device class.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: HW and PWR leads sign off on the pull request that accepts the Decision file.

#### Evidence
- none

### HW-043 · Decide Hardware Compatibility List tiers and promotion criteria
- Type: adr
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: HW-003
- Baseline: §62
- Decision: D-0128
- Invariants: I-095

Public vocabulary for tested-in-lab, community-reported and unsupported, plus promotion rules into Tier 1. Options: two tiers, three tiers, or lab-only until 1.0. Feeds HW-053 and the V3 installer verdict.

<!-- covers: GAP-0371 -->

#### Out of scope
REL published unit (REL-011). Schema fields (HW-047). Promotion automation (HW-083).

#### Acceptance criteria
- [ ] Options evaluated include two tiers, three tiers, and lab-only until 1.0.
- [ ] The accepted option defines promotion into Tier 1 and states that unlisted hardware is unsupported.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: HW and REL leads sign off on the pull request that accepts the Decision file.

#### Evidence
- none

### HW-044 · Decide public vendor priorities for purchase guidance
- Type: adr
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: HW-003, HW-018
- Baseline: §62
- Decision: D-0131
- Risks: R-037
- Invariants: I-095

Honest guidance (AMD graphics first, Intel/MediaTek Wi-Fi, NVIDIA/Broadcom best-effort, or an alternative ranking) before public alpha. Prevents a wave of unsupported-hardware reports. Distinct from SKU selection at V0.

<!-- covers: GAP-0469 -->

#### Out of scope
SKU list (HW-003). Known-good publication (HW-053).

#### Acceptance criteria
- [ ] Options evaluated include AMD-graphics-first with Intel/MediaTek Wi-Fi and NVIDIA/Broadcom best-effort, an inverted vendor ranking, and no public ranking.
- [ ] The accepted option is the text later copied into the known-good list.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: HW and REL leads sign off on the pull request that accepts the Decision file.

#### Evidence
- none

### HW-045 · Enable basic touchscreen and pen digitizer as InputDevice Capabilities
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-011, HW-015, HW-039
- Baseline: §41, §62

UIP requires pointer, touch and pen at V0.5 protocol level and ships basic touchscreen at 1.0 while on-screen keyboard stays out. This is USB/I2C touch and Wacom-class pen enablement on target laptops that include them.

<!-- covers: GAP-0287 -->

#### Out of scope
On-screen keyboard (UIP). Gesture routing (UIP). Machines without the hardware.

#### Acceptance criteria
- [ ] On a target laptop that includes a touchscreen, touch contacts mint `Capability<InputDevice>` and reach a focused native Surface.
- [ ] On a target laptop that includes a Wacom-class pen, pen events including pressure bits are delivered the same way.
- [ ] Laptops without those devices record "absent" in the driver status registry rather than a failed test.

#### Verification
- Integration: `hw:tests/hid/touch_pen_*` on the SKU that has the hardware.
- Review: registry rows distinguish absent versus failed.

#### Evidence
- none

### HW-046 · Ship an LVFS firmware update service recorded in SystemGeneration history
- Type: build
- Milestone: V2
- Status: todo
- Size: L
- Owner: none
- Depends on: HW-023, BOOT-030, BOOT-037, PKG-022, LAB-016
- Baseline: §31, §32
- Threats: T-021
- Invariants: I-022

Incoming from BOOT plus HW firmware items: fwupd/LVFS-class updates for UEFI, microcode, SSDs, docks and peripherals, each recorded as a history event with explicit rollback limits. BOOT keeps ESP capsule staging and PCR re-seal. PKG records the history event.

<!-- covers: GAP-0203, EXTRA-043, GAP-0442, GAP-0343 -->

#### Out of scope
ESP staging (BOOT-037). History storage (PKG-022). Lab version matrix (LAB-016).

#### Acceptance criteria
- [ ] An LVFS-class update for UEFI or device firmware on H-004 is applied through a Capability-gated service and appears as a typed history event.
- [ ] The event records whether rollback is possible; irreversible firmware is labelled as such in `os history`.
- [ ] Unsigned firmware is rejected (T-021).
- [ ] Native software does not write SPI or EFI variables except through this service and BOOT-024.

#### Verification
- Integration: `hw:tests/fwupd/history_event_*` on `hw-h004`.
- Review: BOOT and PKG leads confirm capsule staging and history types.
- Manual: one dock or SSD firmware payload on lab hardware listed in the firmware matrix.

#### Evidence
- none

### HW-047 · Define the Hardware Compatibility List schema and probe record format
- Type: docs
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-043, HW-021
- Baseline: §62
- Invariants: I-095

V3 public HCL needs a schema before community ingest. Records PCI/USB/DMI identifiers, per-generation status, known issues, workarounds and provenance. REL publishes. HW owns the schema the probe tools emit.

<!-- covers: GAP-0368 -->

#### Out of scope
Publication hosting (REL-048). Probe implementation (HW-066). Privacy policy (HW-068).

#### Acceptance criteria
- [ ] The schema specifies PCI, USB and DMI identifiers, per-SystemGeneration status, known issues, workarounds and provenance.
- [ ] The schema has a version field and a golden example record for H-002.
- [ ] Probe tools are specified to emit this schema and no other.

#### Verification
- Review: HW and REL leads sign off on the schema document.
- Unit: `hw:tests/hcl/schema_golden_*` on host CI.

#### Evidence
- none

### HW-048 · Enable iGPU plus dGPU mux and render-offload on hybrid laptops
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-009, HW-039
- Baseline: §40, §56.1
- Risks: R-044

Hybrid-graphics laptops across compositor and personalities. GFX owns scanout and offload. HW owns ACPI mux, power-well and Device enumeration of both GPUs on machines that have them. H-011 and H-012 are V4; this task lands the mechanism so V2 machines that include a mux are not skipped.

#### Out of scope
Render-to-scanout (GFX-076). V4 hybrid SKUs (HW-081).

#### Acceptance criteria
- [ ] On a hybrid SKU in the lab, both GPUs enumerate as Device objects and the ACPI mux state is visible to `os inspect`.
- [ ] Switching mux power-well does not drop HID or USB Device objects.
- [ ] Machines without a mux record "absent" rather than fail.

#### Verification
- Integration: `hw:tests/gpu/hybrid_mux_*` on the hybrid SKU if present, otherwise a documented skip on H-004 and H-005.
- Review: GFX lead confirms scanout stays in GFX.

#### Evidence
- none

### HW-049 · Enumerate gamepads as Capability<InputDevice> for native and personalities
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-011, HW-031
- Baseline: §9.1, §49
- Invariants: I-021

V2 gaming proof-of-concept and W1 integration require gamepad input. HW mints HID gamepad Capabilities. WIN and LNX consume them. No ambient `/dev/input` for native software. Required by V2-G07 (Gaming proof of concept on W1 Gold titles).

#### Out of scope
Wine XInput mapping (WIN). evdev for personality (LNX).

#### Acceptance criteria
- [ ] A USB HID gamepad on H-002 enumerates as `Capability<InputDevice>` and delivers axes and buttons to a native Component that holds it.
- [ ] A native Component without the Capability receives `Error::Rights`.
- [ ] Personality software can still consume the device through the personality path.

#### Verification
- Integration: `hw:tests/hid/gamepad_*` on `hw-h002`.
- Unit: `hw:tests/hid/gamepad_rights_*` on `qemu-x86_64`.

#### Evidence
- none

### HW-050 · Expose laptop function keys, keyboard backlight and brightness
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-015, HW-039, HW-011
- Baseline: §62
- Invariants: I-021

§62 laptop support: function keys, keyboard backlight and display brightness as typed settings. Thermal and frequency stay in PWR. First hardware check reviewers make on the two V2 laptops.

<!-- covers: INV-1219 -->

#### Out of scope
Thermal policy (PWR-022). Brightness policy and ALS (PWR-018). OSD chrome (APP-042).

#### Acceptance criteria
- [ ] Function keys on H-004 and H-005 emit typed key events through `Capability<InputDevice>`.
- [ ] Keyboard backlight and panel brightness are writable through a Capability-gated settings Interface and readable by `os inspect`.
- [ ] Apps without that Capability cannot change backlight or brightness.

#### Verification
- Integration: `hw:tests/laptop/hotkeys_backlight_*` on `hw-h004` and `hw-h005`.
- Manual: brightness and backlight keys on both laptops.

#### Evidence
- none

### HW-051 · Deliver lid and tablet-mode switch events as typed Device signals
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-042, HW-015, HW-039, LAB-014
- Baseline: §62
- Invariants: I-021

§62 lid events on reference laptops. GFX lock-on-lid and PWR suspend consume these signals. HW owns ACPI/HID switch enablement. Tablet-mode follows HW-042. Convertible UI stays out.

<!-- covers: INV-1219, EXTRA-037 -->

#### Out of scope
Lid-close power policy (PWR-021). Lock Surface (GFX-045). Convertible shell (UIP).

#### Acceptance criteria
- [ ] Closing and opening the lid on H-004 and H-005 emits typed Device signals consumed by PWR and GFX.
- [ ] If tablet-mode is in scope, the switch enumerates and emits; if out of scope, the registry says out and no native API is stubbed.
- [ ] LAB automated lid cycles observe the same signals.

#### Verification
- Integration: `hw:tests/laptop/lid_switch_*` on `hw-h004` and `hw-h005`.
- Review: PWR and GFX leads confirm they consume HW signals rather than raw ACPI from apps.

#### Evidence
- none

### HW-052 · Bring up an experimental NVIDIA desktop on the decided driver path
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-018, LAB-018, KRN-038
- Baseline: §56.1
- Risks: R-037, R-050

V2 tracks one NVIDIA desktop as experimental with no gate. Exercises HW-018 so V3 Tier 1 is not the first boot. Documented supported generations start here.

<!-- covers: INV-1064 -->

#### Out of scope
Tier 1 gating (HW-070). Compositor stance (GFX-064).

#### Acceptance criteria
- [ ] H-006 boots the CI image on the path named by the NVIDIA Decision and enumerates the GPU as a Device.
- [ ] Secure Boot module load follows the Decision (open modules, vendor-signed, or MOK).
- [ ] Supported generations are listed in the driver status registry as experimental, not Tier 1.

#### Verification
- Integration: boot and GPU enumeration on `hw-h006`.
- Review: HW lead records experimental, not gated.

#### Evidence
- none

### HW-053 · Publish the tiered known-good hardware list from the compatibility database
- Type: docs
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-043, HW-044, HW-047, HW-061
- Baseline: §62, §56.5
- Invariants: I-095

§62 constrained target hardware communicated as reference, supported and community-reported, with GPU, Wi-Fi, fingerprint and dock guidance. Regenerated from the database each release. Does not promise universal PC compatibility.

<!-- covers: GAP-0468, INV-1113, INV-1233 -->

#### Out of scope
REL hosting (REL-048). Community ingest (HW-065).

#### Acceptance criteria
- [ ] A generated list names H-002, H-004 and H-005 with tier, GPU, Wi-Fi, fingerprint and dock guidance.
- [ ] Unlisted hardware is labelled unsupported.
- [ ] Regeneration from the schema does not require hand-edited SKU rows.

#### Verification
- Review: HW and REL leads sign off on the generated list.
- Unit: `hw:tests/hcl/known_good_generate_*` on host CI.

#### Evidence
- none

### HW-054 · Implement in-scope sensor classes as user-space Components
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-042, HW-029, HW-051
- Baseline: §33, §62

§33 sensor drivers as user-space Components for the classes HW-042 includes. Typed settings for ALS and orientation. PWR consumes ALS for brightness. Out-of-scope classes are documented, not stubbed as native APIs.

<!-- covers: INV-0617, EXTRA-037 -->

#### Out of scope
Brightness policy (PWR-018). Out-of-scope classes.

#### Acceptance criteria
- [ ] Each in-scope class runs as a user-space Component on H-004 and H-005 and exports typed settings.
- [ ] Each out-of-scope class is listed as out in the driver status registry with no native Interface.
- [ ] A Component without the sensor Capability receives `Error::Rights`.

#### Verification
- Integration: `hw:tests/sensors/userspace_*` on `hw-h004` and `hw-h005`.
- Review: PWR lead confirms ALS consumption if ALS is in scope.

#### Evidence
- none

### HW-055 · Evaluate remaining peripheral classes for user-space driver residency
- Type: spike
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-016, HW-031
- Baseline: §33

§33 remaining classes: input already moved, cameras/UVC, storage bridges. Report per class against HW-016 so V3 USB mass storage, webcams and MTP are scheduled as inherited or user-space, not by default.

<!-- covers: INV-0619 -->

#### Out of scope
UVC enablement (HW-078). Mass storage (HW-076). MTP (HW-069).

#### Acceptance criteria
- [ ] The report classifies cameras/UVC, USB mass storage and MTP/PTP as inherited, user-space, or rewrite using the accepted criteria.
- [ ] The report cites measured cost where a prototype ran, and states "unmeasured, keep inherited" where it did not.
- [ ] The report is committed at `reports/spikes/HW-055.md`.

#### Verification
- Report: per-class residency; evidence or explicit unmeasured keep-inherited; which V3 tasks may assume user-space.
- Review: HW lead records the classifications in the driver status registry.

#### Evidence
- none

### HW-056 · Document the three coexisting driver classes and per-class residency
- Type: docs
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-016, HW-021, HW-061
- Baseline: §55
- Invariants: I-097

§55 long-term model: inherited Linux drivers, userspace native drivers and selected rewritten in-kernel drivers coexist. Writes the residency map from HW-016 into the driver status registry for V2 target machines.

<!-- covers: INV-1053 -->

#### Out of scope
Registry generator (HW-021). Criteria Decision (HW-016).

#### Acceptance criteria
- [ ] A committed map lists every Device on H-002, H-004 and H-005 under inherited, userspace-native or rewritten.
- [ ] The map matches the generated registry and names the dual-path period for any moved class.
- [ ] No class is described as fully replaced.

#### Verification
- Review: HW lead signs off on the map versus the registry dump from the three machines.

#### Evidence
- none

### HW-057 · Require IOMMU and authorize Thunderbolt, USB4 and USB devices before DMA
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-017, HW-026, HW-039
- Baseline: §17, §62
- Threats: T-009, T-024
- Invariants: I-074

GAP-0210: IOMMU on, all DMA devices behind it, bolt-style Thunderbolt/USB4 authorisation before PCIe tunnelling. Extends the same authorisation to USB DMA devices so docks at V2 cannot DMA until the user allows them.

<!-- covers: GAP-0210 -->

#### Out of scope
Dock feature enablement (HW-059). IOMMU off case (HW-025).

#### Acceptance criteria
- [ ] A Thunderbolt or USB4 device on H-004 or H-005 does not tunnel PCIe until the user authorises it through a Capability-gated prompt.
- [ ] A USB DMA device is likewise unauthorised until allow; `os inspect` shows pending versus authorised.
- [ ] IOMMU is enabled on H-002, H-004 and H-005 during this test.

#### Verification
- Integration: `hw:tests/tb/authorize_before_dma_*` on `hw-h004` and `hw-h005`.
- Review: SEC lead confirms no DMA window exists before authorisation.

#### Evidence
- none

### HW-058 · Enable multi-finger touchpad gestures on the reference laptops
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-024, HW-039
- Baseline: §62

§62 touchpad gestures. UIP ships basic gestures while on-screen keyboard stays out. HW produces gesture events from the I2C HID device. UIP routes them to focused Surfaces.

<!-- covers: INV-1219 -->

#### Out of scope
Gesture routing (UIP). On-screen keyboard (UIP).

#### Acceptance criteria
- [ ] Two- and three-finger gestures on H-004 and H-005 are emitted as typed InputDevice events.
- [ ] A native Component without the InputDevice Capability observes none of those events.
- [ ] Gesture configuration is a settings Interface, not an app-held global grab.

#### Verification
- Integration: `hw:tests/hid/touchpad_gestures_*` on `hw-h004` and `hw-h005`.
- Review: UIP lead confirms routing consumes HW events.

#### Evidence
- none

### HW-059 · Enable USB-C docks: USB, Ethernet, HID and DisplayPort tunnels
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-057, HW-015, HW-039, LAB-018
- Baseline: §62
- Risks: R-044

V2 laptop day demo plugs an external display via USB-C. GFX owns MST and hot-plug. HW enables dock USB hubs, NIC, HID and DP-alt-mode Device objects, gated on HW-057.

<!-- covers: INV-1222 -->

#### Out of scope
MST mode-set (GFX). NIC DHCP (NET). Authorisation prompt (HW-057).

#### Acceptance criteria
- [ ] After authorisation, a USB-C dock on H-004 and H-005 enumerates hub, NIC, HID and DP-alt-mode Devices.
- [ ] Unplug removes those Devices; replug requires authorisation again before DMA.
- [ ] HID from the dock reaches native apps only through InputDevice Capabilities.

#### Verification
- Integration: `hw:tests/dock/usbc_*` on `hw-h004` and `hw-h005`.
- Demo: V2 laptop-day external display via USB-C on H-005.

#### Evidence
- none

### HW-060 · Verify headset, mouse and keyboard pairing cycles on target laptops
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-037, HW-038, LAB-011
- Baseline: §62

V2 exit: pairing and reconnect of a headset (A2DP and HFP), a mouse and a keyboard succeed in 95 of 100 automated cycles. LAB supplies peer devices. HW owns the scenario scripts.

#### Out of scope
Latency publication (HW-033). Peer fixtures (LAB-011).

#### Acceptance criteria
- [ ] 95 of 100 automated pairing and reconnect cycles succeed for headset A2DP, headset HFP, mouse and keyboard on H-004 and on H-005.
- [ ] Failures are logged per cycle with Device identity redacted of serials.
- [ ] The scripts run against LAB radio peers, not against ad-hoc phones.

#### Verification
- Integration: `hw:tests/bt/pairing_cycles_*` on `hw-h004` and `hw-h005`.
- Review: LAB lead confirms peer devices are the lab fixtures.

#### Evidence
- none

### HW-061 · Verify HID, GPU, USB and firmware on the three V2 target machines
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-039, HW-022, HW-024, HW-051, HW-050, HW-046
- Baseline: §62
- Risks: R-040

V2 hardware scope is exactly three named machines. Gate-verifying matrix: internal/external HID, GPU enumeration, USB hot-plug, lid/brightness, firmware probe. PWR, NET, AUD and GFX own their slices. HW owns the combined per-machine report.

<!-- covers: INV-1113 -->

#### Out of scope
Suspend (PWR). Wi-Fi (NET). Audio (AUD). Scanout (GFX).

#### Acceptance criteria
- [ ] Combined reports exist for H-002, H-004 and H-005 covering HID, GPU enumeration, USB hot-plug, lid/brightness and firmware probe.
- [ ] Each report cites the sibling prefix tasks that own power, network, audio and display slices.
- [ ] A failing HID, GPU, USB or firmware row fails this task.

#### Verification
- Integration: `hw:tests/matrix/v2_three_targets_*` on `hw-h002`, `hw-h004` and `hw-h005`.
- Review: HW lead signs the combined per-machine report.

#### Evidence
- none

### HW-062 · Bring up one additional Intel laptop and one additional AMD laptop
- Type: build
- Milestone: V3
- Status: todo
- Size: L
- Owner: none
- Depends on: HW-003, LAB-021, HW-061
- Baseline: §63, §62
- Risks: R-056
- Invariants: I-074

V3 Tier 1 is six machines: three V2 targets plus NVIDIA desktop plus one extra laptop per vendor. HW bring-up of H-007 and H-008 onto the same HID/GPU/USB/firmware matrix. Required by V3-G01 (Installer completes on Tier 1 with full-disk encryption).

#### Out of scope
Racking (LAB-021). NVIDIA desktop (HW-070).

#### Acceptance criteria
- [ ] H-007 and H-008 are named in `registers/hardware.md` with SKU, firmware, IOMMU, TPM 2.0 and Secure Boot enrolment.
- [ ] Both boot a CI image and pass the HID, GPU, USB and firmware rows used on H-004 and H-005.
- [ ] Driver status registry rows exist for both SKUs.

#### Verification
- Integration: bring-up suite on `hw-h007` and `hw-h008`.
- Review: HW lead confirms Tier 1 six-machine floor except NVIDIA.

#### Evidence
- none

### HW-063 · Enable fingerprint readers as Device objects for the identity service
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-008, SEC-014
- Baseline: §9.1, §63
- Invariants: I-021

SEC greeter adds fingerprint login at V3. GAP-0468 lists fingerprint readers in purchase guidance. HW enables USB/SPI fingerprint devices behind `Capability<Device>`. SEC owns matching. Out of scope if the SKU has no reader.

#### Out of scope
Matcher and login (SEC-057). Greeter chrome (APP).

#### Acceptance criteria
- [ ] On a Tier 1 SKU that includes a reader, the device enumerates as `Capability<Device>` and is visible to the authenticator Interface.
- [ ] A native app without that Capability cannot capture fingerprints and receives `Error::Rights`.
- [ ] SKUs without a reader record "absent" in the registry.

#### Verification
- Integration: `hw:tests/fp/device_*` on a SKU that has a reader.
- Review: SEC lead confirms matching stays in SEC.

#### Evidence
- none

### HW-064 · Ship a user-runnable hardware conformance tool with consented upload
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-047, HW-068, HW-066
- Baseline: §62, §63
- Threats: T-042
- Invariants: I-078

Public alpha meets hardware the lab never will. Executes the compatibility suite on an arbitrary machine and submits anonymised, consented results. Distinct from the pre-install checker: this is post-install conformance.

<!-- covers: GAP-0146, GAP-0471 -->

#### Out of scope
Live-image checker (HW-066). REL store (REL-048).

#### Acceptance criteria
- [ ] The tool runs the HW compatibility suite on an installed system and produces a schema-valid record.
- [ ] Upload is opt-in, shows the redacted payload, and refuses to proceed if serials or network identifiers remain.
- [ ] A run with consent declined stores the record only locally.

#### Verification
- Integration: `hw:tests/hcl/conformance_tool_*` on `hw-h002`.
- Review: GOV privacy reviewer confirms the payload matches HW-068.

#### Evidence
- none

### HW-065 · Ingest community Hardware Compatibility List probes for 100 machines
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-047, HW-068, HW-064
- Baseline: §63
- Risks: R-085
- Invariants: I-095

V3 exit: database has all Tier 1 machines and at least 100 community-submitted machines with probe data. HW owns validation against schema and privacy. REL hosts the store.

#### Out of scope
Hosting (REL-048). Host-side probe (HW-067).

#### Acceptance criteria
- [ ] The ingest path validates schema and privacy rules and rejects records with serials or network identifiers.
- [ ] The database contains every V3 Tier 1 machine plus at least 100 community records.
- [ ] Invalid records never appear in the published list.

#### Verification
- Integration: `hw:tests/hcl/ingest_validate_*` on host CI with fixture records.
- Review: REL lead confirms the store receives only validated records.

#### Evidence
- none

### HW-066 · Ship a live-image Hardware Compatibility List checker for PCI USB ACPI
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-047, HW-009
- Baseline: §63
- Invariants: I-095

V3 installer compatibility verdict and probe-from-live-image. Inventories PCI, USB, ACPI and firmware identifiers and reports against the database. INS embeds it. HW owns the probe and verdict logic.

<!-- covers: GAP-0470 -->

#### Out of scope
Installer warning chrome (INS-028). Host-side probe (HW-067). REL publication (REL-035).

#### Acceptance criteria
- [ ] From live media, the checker emits a schema-valid inventory of PCI, USB, ACPI and firmware identifiers.
- [ ] The verdict is Tier 1 match, community match, or unsupported, using HW-043 vocabulary.
- [ ] The checker does not write disks.

#### Verification
- Integration: `hw:tests/hcl/live_checker_*` on `hw-h002` booted from live media.
- Review: INS lead confirms the verdict is consumed without HW owning installer chrome.

#### Evidence
- none

### HW-067 · Ship a host-side Hardware Compatibility List probe for foreign OSes
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-047, HW-066
- Baseline: §63
- Invariants: I-095

GAP-0470 also requires a probe that runs on a Windows or Linux host before wiping a disk. Same schema as HW-066. Output is a file the installer and website can score.

<!-- covers: GAP-0470 -->

#### Out of scope
Live-image checker (HW-066). Website scoring (REL).

#### Acceptance criteria
- [ ] The probe runs on a Linux host and on a Windows host in the lab and writes a schema-valid file.
- [ ] The file contains no serials or network identifiers.
- [ ] The installer can score the file with the same verdict logic as the live checker.

#### Verification
- Integration: `hw:tests/hcl/host_probe_*` on lab Linux and Windows baseline images.
- Compat: file scores identically to the live checker on H-002.

#### Evidence
- none

### HW-068 · Make Hardware Compatibility List submissions opt-in, redacted and reviewable
- Type: docs
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: HW-047, GOV-061
- Baseline: §9.1, §63
- Risks: R-085
- Threats: T-042
- Invariants: I-078

Submissions are opt-in, pseudonymous, user-reviewable, and contain no serials or network identifiers. Binding on HW-066, HW-064 and REL ingest.

<!-- covers: GAP-0471 -->

#### Out of scope
Privacy policy text (GOV-061). Store hosting (REL).

#### Acceptance criteria
- [ ] The policy lists forbidden fields (serials, MAC, IP, user identity) and the user-review step before upload.
- [ ] Probe and conformance tools fail closed if a forbidden field is present.
- [ ] Upload requires an explicit opt-in; a declined run keeps the record local.

#### Verification
- Review: GOV privacy and HW leads sign off on the policy document.
- Unit: `hw:tests/hcl/privacy_redaction_*` on host CI with fixture payloads containing serials.

#### Evidence
- none

### HW-069 · Support MTP and PTP phone-to-desktop transfer for Android and iOS
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-055, HW-008, STO-062
- Baseline: §9.1, §25
- Invariants: I-021

Photo import is a top desktop task at public alpha. USB MTP/PTP runs as a user-space Component granting UserSelected files, not ambient phone filesystem access.

<!-- covers: GAP-0439 -->

#### Out of scope
Chooser chrome (APP). Collection mount (STO-062). Ambient `/media` (LNX).

#### Acceptance criteria
- [ ] Connecting an Android MTP device and an iOS PTP device enumerates a Device and offers UserSelected files through the OS chooser.
- [ ] A native app without that grant cannot list the phone store and receives `Error::Rights`.
- [ ] Residency follows HW-055 (inherited or user-space) and is recorded in the driver status registry.

#### Verification
- Integration: `hw:tests/usb/mtp_ptp_*` on `hw-h002` with lab phones or fixtures.
- Review: STO lead confirms files arrive as UserSelected, not a global volume.

#### Evidence
- none

### HW-070 · Bring up the Tier 1 NVIDIA desktop on the decided driver and Secure Boot path
- Type: build
- Milestone: V3
- Status: todo
- Size: L
- Owner: none
- Depends on: HW-052, HW-018, LAB-021, KRN-038, GFX-064, GFX-027
- Baseline: §56.1, §63
- Risks: R-050, R-037
- Invariants: I-074

V3 hardware scope adds one NVIDIA desktop, proprietary or open path decided by HW-018, fully tested each release. Completes INV-1064 with documented generations and a passing Secure Boot module load.

<!-- covers: INV-1064 -->

#### Out of scope
Compositor NVIDIA stance (HW-052). Experimental-only boot (HW-052).

#### Acceptance criteria
- [ ] H-006 is Status in-lab in `registers/hardware.md` with SKU, firmware, IOMMU, TPM 2.0 and Secure Boot enrolment.
- [ ] The GPU loads on the Decision path under Secure Boot and enumerates as a Device.
- [ ] Supported generations are listed in the driver status registry as Tier 1, not experimental.
- [ ] HID, USB and firmware probe rows used on H-002 pass on H-006.

#### Verification
- Integration: bring-up and Secure Boot module load on `hw-h006`.
- Review: GFX and KRN leads confirm the module path matches the NVIDIA Decision.

#### Evidence
- none

### HW-071 · Ship driverless print discovery and Capability<PrintJob> for a single job
- Type: build
- Milestone: V3
- Status: todo
- Size: L
- Owner: none
- Depends on: HW-041, NET-024, HW-029
- Baseline: §33, §9.1
- Threats: T-002
- Invariants: I-021

V3 printing scope. IPP Everywhere/AirPrint, USB, WSD and SMB-shared printers via mDNS. Printer drivers as user-space Components. OS-owned discovery grants `Capability<PrintJob>` the way the file chooser grants file authority. APP owns dialog chrome. LNX owns the CUPS socket.

<!-- covers: GAP-0268, GAP-0438, INV-0618 -->

#### Out of scope
Dialog chrome (APP-064). CUPS personality socket (LNX-093). PDF renderer (APP-053).

#### Acceptance criteria
- [ ] mDNS, USB, WSD and SMB-shared IPP Everywhere printers are discovered without granting apps adapter-wide scan.
- [ ] Completing the system dialog mints `Capability<PrintJob>` for one job; a native app without it receives `Error::Rights`.
- [ ] Printer drivers run as user-space Components when the Decision named that path.
- [ ] Native software does not open a CUPS socket; that path stays in LNX.

#### Verification
- Integration: `hw:tests/print/discovery_job_*` on `hw-h002` against a lab IPP Everywhere printer and a USB printer.
- Review: APP lead confirms chrome stays in APP.
- Demo: V3 native app prints one page on H-002.

#### Evidence
- none

### HW-072 · Restart the printing service with queued jobs preserved
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-071
- Baseline: §32
- Invariants: I-037

§32 printing service restartable with queued jobs preserved. Supervisor restart must not drop an in-flight IPP job on Tier 1 machines.

<!-- covers: INV-0597 -->

#### Out of scope
Supervisor (SVC). Personality CUPS (LNX-093).

#### Acceptance criteria
- [ ] Killing the print service on a Tier 1 machine preserves queued IPP jobs and completes them after rebind.
- [ ] `os inspect` shows the restart count and rebound PrintJob Capabilities.
- [ ] An in-flight job is not silently deleted.

#### Verification
- Integration: `hw:tests/print/service_rebind_*` on `hw-h002` and one V3 laptop.
- Review: SVC lead confirms restart budgets apply.

#### Evidence
- none

### HW-073 · Enable SD and MMC readers as Capability-gated removable storage
- Type: build
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: HW-009, STO-062
- Baseline: §9.1, §25
- Invariants: I-021

Common laptop hardware at public alpha. Same authority model as USB mass storage: Device enumeration in HW, UserSelected volume in STO.

#### Out of scope
Volume mount and Collection (STO-062). exFAT (STO-058).

#### Acceptance criteria
- [ ] Inserting an SD card on a laptop that has a reader enumerates a Device and offers a UserSelected volume.
- [ ] A native app without that grant cannot read the card and receives `Error::Rights`.
- [ ] SKUs without a reader record "absent" in the registry.

#### Verification
- Integration: `hw:tests/storage/sd_mmc_*` on a laptop SKU with a reader.
- Review: STO lead confirms UserSelected, not ambient `/media`.

#### Evidence
- none

### HW-074 · Enable CCID smartcard readers as Device objects for authentication
- Type: build
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: HW-008, SEC-014
- Baseline: §9.1
- Invariants: I-021

APP/SEC lock screen lists smartcard authentication. HW enables USB CCID readers as `Capability<Device>`. SEC owns PIN and certificate use. No ambient PC/SC for native apps.

#### Out of scope
PIN and certificates (SEC). Lock chrome (APP).

#### Acceptance criteria
- [ ] A USB CCID reader enumerates as `Capability<Device>` on a Tier 1 machine.
- [ ] A native app without that Capability cannot talk to the reader and receives `Error::Rights`.
- [ ] Native tests do not open a PC/SC device node.

#### Verification
- Integration: `hw:tests/ccid/device_*` on `hw-h002` with a lab reader.
- Review: SEC lead confirms PIN handling stays in SEC.

#### Evidence
- none

### HW-075 · Enable common USB classes on Tier 1 and record them in the driver registry
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-055, HW-021, HW-031
- Baseline: §33, §63
- Invariants: I-097

V3 scope: common USB classes beyond HID and mass storage (CDC, hub, serial, audio pass-through to AUD). Per-class residency follows HW-055. Each class is an entry in the driver status registry.

#### Out of scope
Audio graph (AUD). Mass storage (HW-076). HID (HW-031).

#### Acceptance criteria
- [ ] CDC, hub, serial and USB-audio (pass-through) enumerate on every V3 Tier 1 machine that has the hardware.
- [ ] Each class has a driver status registry row naming inherited or user-space per the spike.
- [ ] Native software does not open USB device nodes for these classes.

#### Verification
- Integration: `hw:tests/usb/class_matrix_*` on V3 Tier 1 machines.
- Review: AUD lead confirms USB-audio is pass-through, not a native mixer.

#### Evidence
- none

### HW-076 · Expose USB mass-storage volumes through Capability-gated storage
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-055, HW-009, STO-062
- Baseline: §9.1, §25, §33
- Invariants: I-021

V3 scope: USB mass storage and common USB classes. HW enumerates the device. STO mounts with UserSelected authority so apps do not receive ambient removable-volume access.

#### Out of scope
Mount and Collection (STO-062). MTP (HW-069).

#### Acceptance criteria
- [ ] A USB mass-storage device on H-002 enumerates as a Device and offers a UserSelected volume.
- [ ] A native app without that grant cannot list the volume and receives `Error::Rights`.
- [ ] Residency follows the remaining-peripherals spike and is recorded in the registry.

#### Verification
- Integration: `hw:tests/usb/mass_storage_*` on `hw-h002`.
- Review: STO lead confirms UserSelected grant.

#### Evidence
- none

### HW-077 · Verify driverless printing on every V3 Tier 1 machine
- Type: build
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: HW-071, HW-062, HW-070
- Baseline: §63

V3 scope includes printing on six Tier 1 machines. Scripted IPP Everywhere job from a native app plus discovery of a USB printer. LNX CUPS path is a separate LNX task.

<!-- covers: GAP-0438 -->

#### Out of scope
CUPS personality (LNX-093). Dialog chrome (APP).

#### Acceptance criteria
- [ ] A scripted IPP Everywhere job from a native app completes on each of H-002, H-004, H-005, H-006, H-007 and H-008.
- [ ] USB printer discovery succeeds on each of those machines.
- [ ] Per-machine results are committed under Evidence.

#### Verification
- Integration: `hw:tests/print/tier1_verify_*` on the six V3 Tier 1 machines.
- Review: HW lead signs the combined print report.

#### Evidence
- none

### HW-078 · Enable UVC webcams as Device objects consumed by the camera service
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-008, HW-055, MED-013
- Baseline: §7, §9.1, §33
- Threats: T-001
- Invariants: I-021

V3 cameras and webcams. HW enables UVC devices as `Object<Device>`. MED mints `Capability<Camera>` and the in-use indicator. Native software never receives ambient V4L2 nodes.

<!-- covers: GAP-0303 -->

#### Out of scope
Camera service and grants (MED-013). In-use chrome (APP). Object<Camera> (MED).

#### Acceptance criteria
- [ ] A UVC webcam on each V3 laptop enumerates as `Capability<Device>` for the camera service.
- [ ] A native app cannot open a V4L2 node and receives `Error::Rights` without a Camera grant from MED.
- [ ] Hot-plug adds and removes the Device without restarting MED.

#### Verification
- Integration: `hw:tests/uvc/device_*` on `hw-h004` and `hw-h005`.
- Review: MED lead confirms Device consumption and Camera minting stay in MED.

#### Evidence
- none

### HW-079 · Record Wi-Fi and Bluetooth inherited-driver breadth in the compatibility database
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-021, HW-047, HW-065
- Baseline: §62, §63
- Invariants: I-095

V3 scope: Wi-Fi and Bluetooth hardware beyond target machines via inherited Linux drivers and the database. NET owns association. HW owns chipset enablement notes and HCL rows for community adapters.

<!-- covers: GAP-0468 -->

#### Out of scope
Association and DHCP (NET-033). Bluetooth profiles on targets (HW-037).

#### Acceptance criteria
- [ ] Community HCL rows for Wi-Fi and Bluetooth adapters name the inherited driver and test coverage.
- [ ] Target-machine chipsets on H-004 and H-005 have registry rows that the HCL cites.
- [ ] Unlisted adapters are unsupported, not silently claimed.

#### Verification
- Review: NET and HW leads sign the chipset notes against the HCL schema.
- Unit: `hw:tests/hcl/wifi_bt_rows_*` on host CI.

#### Evidence
- none

### HW-080 · Bring up the Tier 1 Intel desktop Reference machine
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-003, LAB-023
- Baseline: §62
- Risks: R-056
- Invariants: I-074

V4 hardware scope: at least ten named machines including one Intel desktop. SKU recorded in the hardware register. HID, GPU, USB, firmware and HCL row required before RC soak.

#### Out of scope
Racking (LAB-023). Full suite (HW-086).

#### Acceptance criteria
- [ ] H-010 is named in `registers/hardware.md` with SKU, firmware, IOMMU, TPM 2.0 and Secure Boot enrolment.
- [ ] The machine boots a CI image and passes HID, GPU, USB and firmware probe rows used on H-002.
- [ ] An HCL row exists for H-010.

#### Verification
- Integration: bring-up suite on `hw-h010`.
- Review: HW lead confirms H-010 counts toward the ten-machine floor.

#### Evidence
- none

### HW-081 · Bring up second-Generation AMD and Intel laptops and a second AMD GPU
- Type: build
- Milestone: V4
- Status: todo
- Size: L
- Owner: none
- Depends on: HW-003, LAB-023, HW-062
- Baseline: §62
- Risks: R-056
- Invariants: I-074

V4: AMD desktops at least two GPU generations, Intel laptops two generations, AMD laptops two. Completes the ten-machine Tier 1 floor with SKUs, firmware matrix and suite green. Covers H-009, H-011, H-012, H-013 and H-014.

#### Out of scope
Intel desktop (HW-080). Racking (LAB-023). Combined RC suite (HW-086).

#### Acceptance criteria
- [ ] H-009, H-011, H-012, H-013 and H-014 are named in `registers/hardware.md` with SKU, firmware, IOMMU, TPM 2.0 and Secure Boot enrolment.
- [ ] Each boots a CI image and passes HID, GPU, USB and firmware probe rows.
- [ ] Hybrid SKUs enumerate both GPUs as Device objects.
- [ ] Driver status registry and HCL rows exist for each.

#### Verification
- Integration: bring-up suite on `hw-h009`, `hw-h011`, `hw-h012`, `hw-h013` and `hw-h014`.
- Review: LAB and HW leads confirm the ten-machine floor with H-010.

#### Evidence
- none

### HW-082 · Decide policy for third-party user-space drivers and firmware packages
- Type: adr
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-056, HW-047
- Baseline: §55, §62
- Decision: D-0130
- Invariants: I-097

Vendors and community porters need a channel without forking the OS. Options: signed Packages with review and revocation, in-tree only, or unrestricted user-space loaders. How they appear in the Hardware Compatibility List is part of the Decision.

<!-- covers: GAP-0479 -->

#### Out of scope
Implementation (HW-085). Repository hosting (PKG, REL).

#### Acceptance criteria
- [ ] Options evaluated include signed Packages with review and revocation, in-tree only, and unrestricted user-space loaders.
- [ ] The accepted option states review, signing, revocation and how out-of-tree drivers appear in the HCL.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: HW, PKG and REL leads sign off on the pull request that accepts the Decision file.

#### Evidence
- none

### HW-083 · Automate Tier 2 to Tier 1 promotion from probe data and suite results
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-043, HW-065, HW-086
- Baseline: §62
- Invariants: I-095

V4 scope: Tier 2 process mature with a documented promotion path. Implements HW-043 criteria against community probes plus a lab suite run, without silently claiming Tier 1.

<!-- covers: GAP-0371 -->

#### Out of scope
Community ingest (HW-065). Suite content (HW-086).

#### Acceptance criteria
- [ ] A promotion job consumes a community probe plus a lab suite run and emits a proposed Tier 1 row.
- [ ] The job refuses promotion when suite rows are missing or the Decision criteria are unmet.
- [ ] No HCL row changes from Tier 2 to Tier 1 without an explicit accept step.

#### Verification
- Integration: `hw:tests/hcl/promote_tier2_*` on host CI with fixture probes.
- Review: REL lead confirms publication stays gated on the accept step.

#### Evidence
- none

### HW-084 · Ship eSCL/WSD and SANE scanning with Capability<Scanner> via a system dialog
- Type: build
- Milestone: V4
- Status: todo
- Size: L
- Owner: none
- Depends on: HW-041, HW-071, NET-024
- Baseline: §33, §9.1
- Invariants: I-021

V4 hardware test suite lists scanning. Driverless eSCL/WSD plus SANE backends. `Capability<Scanner>` from an OS dialog. First-party scanner app is APP. This task is the service and dialog grant.

<!-- covers: GAP-0269 -->

#### Out of scope
First-party scanner app (APP). Print jobs (HW-071).

#### Acceptance criteria
- [ ] eSCL/WSD discovery plus a SANE backend enumerate scanners without granting apps adapter-wide scan.
- [ ] Completing the system dialog mints `Capability<Scanner>` for one scan; a native app without it receives `Error::Rights`.
- [ ] A scan produces a UserSelected image object, not an ambient directory.

#### Verification
- Integration: `hw:tests/scan/escl_sane_*` on `hw-h002` against a lab eSCL device.
- Review: APP lead confirms chrome stays in APP.
- Demo: scan-to-image on H-002.

#### Evidence
- none

### HW-085 · Implement signed third-party driver and firmware Packages with revocation
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-082, HW-029, HW-047
- Baseline: §55, §28
- Threats: T-020
- Invariants: I-097

Implements HW-082: review, signing, revocation and HCL rows for out-of-tree user-space drivers. PKG and REL own the repository. HW owns Device matching and the compatibility-database fields.

<!-- covers: GAP-0479 -->

#### Out of scope
Repository (PKG, REL). In-tree classification (HW-016).

#### Acceptance criteria
- [ ] A signed third-party user-space driver Package matches a Device and loads only with the rights the Decision named.
- [ ] Revoking the Package unloads the driver and revokes its Device Capabilities; `os inspect` shows the revocation.
- [ ] An HCL field records provenance as third-party and the signer identity.
- [ ] If the Decision forbade out-of-tree loaders, CI rejects such a Package and this task records that outcome.

#### Verification
- Integration: `hw:tests/usd/third_party_sign_revoke_*` on `hw-h002`.
- Review: PKG and REL leads confirm signing and repository fields.

#### Evidence
- none

### HW-086 · Run the full hardware suite on every Tier 1 machine each release candidate
- Type: build
- Milestone: V4
- Status: todo
- Size: L
- Owner: none
- Depends on: HW-081, HW-080, HW-070, HW-084, HW-077, HW-075, HW-076
- Baseline: §62
- Risks: R-056

V4 exit: every Tier 1 machine passes display hot-plug, suspend/resume cycles, Wi-Fi, Bluetooth, audio, camera, USB classes and printing each RC. HW owns the combined suite and per-machine report. PWR, NET, AUD, GFX and MED supply their cases.

#### Out of scope
Case authorship in PWR, NET, AUD, GFX, MED. Soak calendar (LAB-024).

#### Acceptance criteria
- [ ] The combined suite runs on every Tier 1 machine listed in `registers/hardware.md` for V4.
- [ ] A per-machine report lists display hot-plug, suspend/resume, Wi-Fi, Bluetooth, audio, camera, USB classes, printing and scanning with pass or inapplicable.
- [ ] A failing required row fails the RC job.

#### Verification
- Integration: `hw:tests/suite/tier1_rc_*` on every V4 Tier 1 machine.
- Review: prefix leads for PWR, NET, AUD, GFX and MED confirm their cases are invoked, not duplicated.

#### Evidence
- none

### HW-087 · Sign off hardware soak: no P0 driver or firmware regression on Tier 1
- Type: docs
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: HW-089, LAB-025
- Baseline: §62, §63
- Risks: R-063

1.0 risk: last-minute driver or firmware refreshes regress hardware tests during soak. Written sign-off that the soak fleet matches published HCL firmware versions and has zero open P0 or P1 hardware defects.

#### Out of scope
Soak execution (LAB-025). HCL publication (HW-088).

#### Acceptance criteria
- [ ] A signed report lists every Tier 1 machine, the HCL firmware version, and zero open P0 or P1 hardware defects.
- [ ] A driver or firmware change during soak that fails HW-086 is recorded as a rejected refresh.
- [ ] The sign-off is attached as Evidence on the 1.0 candidate.

#### Verification
- Review: HW lead and release engineer sign the soak hardware report.
- Manual: firmware versions on the soak fleet match the published HCL rows.

#### Evidence
- none

### HW-088 · Publish the 1.0 Hardware Compatibility List and per-machine lab results
- Type: docs
- Milestone: 1.0
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-089, HW-083, HW-053
- Baseline: §62, §63
- Invariants: I-095

1.0 scope and exit: final HCL publication with Tier 1 at least ten named x86-64 machines and Tier 2 community, plus test-lab results per Tier 1 machine. REL hosts. HW authors the content from the database.

#### Out of scope
Channel launch (REL-063). Hosting (REL-048).

#### Acceptance criteria
- [ ] The 1.0 HCL lists at least ten named Tier 1 machines and the community Tier 2 set.
- [ ] Each Tier 1 row links the lab suite report from HW-089.
- [ ] Unlisted hardware is labelled unsupported.

#### Verification
- Review: HW and REL leads sign the 1.0 HCL content.
- Unit: `hw:tests/hcl/final_ten_machines_*` on host CI against the register.

#### Evidence
- none

### HW-089 · Verify every listed Tier 1 feature on the 1.0 release candidate
- Type: build
- Milestone: 1.0
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-086
- Baseline: §62, §63

1.0 hardware coverage: every listed feature works on each Tier 1 machine, including suspend/resume, Wi-Fi, Bluetooth, external displays and HDR/VRR where the hardware supports it. Runs HW-086 as a release gate.

#### Out of scope
Feature authorship (PWR, NET, AUD, GFX, MED). Sign-off letter (HW-087).

#### Acceptance criteria
- [ ] The V4 combined suite passes on the 1.0 candidate for every Tier 1 machine.
- [ ] HCL-listed features (suspend/resume, Wi-Fi, Bluetooth, external displays, HDR/VRR where supported) are pass or inapplicable, never silent fail.
- [ ] Results are committed per machine under Evidence.

#### Verification
- Integration: `hw:tests/suite/1_0_candidate_*` on every Tier 1 machine.
- Review: HW lead compares results to published HCL rows.

#### Evidence
- none

### HW-090 · Park NFC, WWAN and eSIM hardware enablement after 1.0
- Type: docs
- Milestone: LATER
- Status: todo
- Size: S
- Owner: none
- Depends on: none
- Baseline: §62
- Invariants: I-093

NFC, WWAN and eSIM Device classes are declared LATER so they appear in the 1.0 non-goal list. Casting and MIDI are APP/AUD. This task parks the radio and NFC Device classes only.

<!-- covers: EXTRA-038 -->

#### Out of scope
Combined non-goal publication (APP-069). MIDI (AUD-030). Casting (APP).

#### Acceptance criteria
- [ ] NFC, WWAN and eSIM are listed as LATER Device classes with no native Interface in 1.0.
- [ ] The 1.0 HCL does not claim those classes.
- [ ] No stub native API is shipped for them.

#### Verification
- Review: HW and APP leads confirm the parked classes match the published 1.0 non-goal list.

#### Evidence
- none
