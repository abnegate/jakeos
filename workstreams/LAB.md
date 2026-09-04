# LAB · Physical hardware lab
- Prefix: LAB
- Lead: none
- Baseline: §30, §54, §55, §56.1, §59, §61, §62, §63
- Baseline gap: The baseline names Reference machines, Input-to-photon latency and energy tracking but specifies no physical lab, racks, remote power, consoles, capture, scheduler or procurement.

<!-- roadmap:generated:begin summary -->
Tasks: 25 live, 0 done, 0 in-progress, 25 todo, 0 dropped. Ready: 1. Blocked: 24. Weighted: 0%.
<!-- roadmap:generated:end -->

## Scope

LAB owns the physical hardware lab that `registers/hardware.md` lists and that later gates assume is already plugged in. It procures and racks each Reference machine as that machine's first Milestone approaches, with remote power, serial or USB-debug consoles and display capture. It builds the Input-to-photon latency photodiode or camera rig on the V0 desktop, procures external power meters and the HDR reference display plus colorimeter, equips network boot, USB switching, programmable HID, Wi-Fi and Bluetooth peers and audio loopback, and runs a job scheduler that reserves machines, flashes images, returns results to CI and recovers an unbootable board onto a known-good SystemGeneration.

It also installs Linux and Windows comparison images on the same boxes BEN publishes against, maintains a firmware version matrix, runs nightly GPU conformance on lab GPUs, automates suspend, lid, dock and RTC-wake fixtures, procures assistive-technology hardware, and executes multi-day and release-candidate soaks on the Tier 1 fleet. Lab control is operator infrastructure. Native software never sees PDU, Redfish, LAVA or capture-card POSIX APIs.

## Out of scope

SKU selection, driver bring-up, HID services, LVFS firmware updates and HCL probe schema (HW). Benchmark methodology, harness publication and cross-OS tables (BEN). CI tiers, QEMU matrix, merge queue and wiring lab jobs into nightly (BLD). Compositor, Mesa, HDR pipeline and Vulkan test content (GFX). Suspend policy, Battery reporting and energy workloads (PWR). Audio graph and B-028 harness logic (AUD). Wi-Fi association and roaming cases (NET). Screen reader and braille protocol (ACC). Graphical installer and dual-boot policy (INS). Bootloader, Secure Boot strategy and boot-counter fallback (BOOT). HCL website, channels and go/no-go (REL). Leak detectors and trace schema (OBS). Kernel fork and retained DRM (KRN). Image builder and generation compose (INS, PKG). Supervisor and native init (SVC). Signing and trust roots (SEC, REL). Docs site (DOC). Licence policy (GOV).

## Tasks

### LAB-001 · Build the Input-to-photon photodiode rig on the reference desktop
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: LAB-003, LAB-004
- Baseline: §54, §59
- Benchmarks: B-020
- Risks: R-022
- Invariants: I-061

V0.5 compositor gates publish B-020 on H-002. Software timestamps cannot measure Input-to-photon latency (§54). This task installs the sensor, HID injector and calibration path chosen by LAB-004 on the racked AMD desktop so GFX-004 and BEN-010 drive a fixture rather than a guess. Later racks reuse the same fixture design.

<!-- covers: EXTRA-049, GAP-0138, GAP-0546 -->

#### Out of scope
Sensor choice (LAB-004). Harness and B-020 publication (GFX-004, BEN-010). Toolkit-stage timestamps (UIP).

#### Acceptance criteria
- [ ] The accepted sensor from LAB-004 is mounted on H-002 and records a pixel-change event when the attached display lights a calibration patch.
- [ ] Programmable HID injection on H-002 produces a matching input event in the same capture log as the photon event.
- [ ] A calibration record in the repository names sensor model, mounting, display connector and the delay standard used; no B-020 number appears in LAB prose (I-061).
- [ ] A second lab machine can attach the same fixture class without rewriting the capture schema.

#### Verification
- Manual: operator fires the calibration patch on H-002 and files the capture log on the pull request.
- Integration: `lab:tests/photon/h002_fixture_*` records HID and photon edges on CI matrix entry `hw-h002`.
- Review: BEN lead confirms the fixture matches the B-020 method.

#### Evidence
- none

### LAB-002 · Decide lab site and the remote power, console and capture stack
- Type: adr
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: none
- Baseline: none
- Decision: D-0169
- Risks: R-081
- Invariants: I-001

BASELINE.md has no lab section. CONVENTIONS requires this first adr to define that open scope: where machines live, how power is switched, how consoles and display capture reach operators and CI, and that LAB owns racks and rigs while HW owns SKUs. The Decision must land before H-002 is procured. Job-scheduler family is LAB-005.

<!-- covers: GAP-0372 -->

#### Out of scope
Scheduler family and unbootable recovery (LAB-005). SKU list (HW-003). Quiet perf-CI fleet (BLD-048).

#### Acceptance criteria
- [ ] Options evaluated include (A) colocation with PDU plus serial or USB-debug consoles and capture cards, (B) an office lab with Redfish or IPMI on each machine, and (C) a hybrid of colocated desktops and office laptops.
- [ ] The accepted option names power control, console transport, display capture and network reachability for H-002, and states that 1.0 lab machines are x86-64 only (I-001).
- [ ] The accepted option records that native software never receives PDU, Redfish or capture-card handles, and that HW-003 still owns SKU selection.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: LAB and HW leads sign off on the pull request that accepts the Decision file.

#### Evidence
- none

### LAB-003 · Procure and rack the reference AMD desktop with power and capture
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: LAB-002, HW-003
- Baseline: §59, §62
- Risks: R-022, R-081
- Invariants: I-074

V0 hardware scope is QEMU plus one AMD desktop. EXTRA-052 and the V0 boot-on-hardware gate need H-002 on the accepted power, console and capture stack before BOOT-002 can power-cycle it. The SKU is the machine HW-003 names; this task buys, racks and documents the outlet, console path and capture port.

<!-- covers: EXTRA-052 -->

#### Out of scope
SKU decision (HW-003). Kernel bring-up (HW-001). UEFI boot of the CI image (BOOT-002). Soak scheduling (LAB-010).

#### Acceptance criteria
- [ ] H-002 is in the lab on the accepted power and console stack, and `registers/hardware.md` records its SKU once procured.
- [ ] Remote power-off and power-on of H-002 is invoked from the documented PDU or Redfish path and yields a console prompt.
- [ ] Display capture and a serial or USB-debug console each produce a log for a cold boot of H-002.
- [ ] The rack record lists IOMMU, TPM 2.0 and Secure Boot enrolment posture required by I-074.

#### Verification
- Manual: operator power-cycles H-002 from the documented remote path and attaches console and capture logs to the pull request.
- Integration: `lab:tests/rack/h002_power_console_*` on CI matrix entry `hw-h002`.
- Review: HW lead confirms the SKU matches HW-003.

#### Evidence
- none

### LAB-004 · Prototype photodiode versus camera Input-to-photon measurement
- Type: spike
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: LAB-003
- Baseline: §54
- Risks: R-022
- Invariants: I-061

B-020 cannot be a software timestamp. This spike compares a photodiode against a high-speed camera on H-002, with HID injection and a calibration source, so LAB-001 installs a chosen fixture rather than both. The report is the method; BEN publishes results later.

#### Out of scope
Fixture installation (LAB-001). B-020 publication (GFX-004, BEN-010). Visible-UI boundary (BEN).

#### Acceptance criteria
- [ ] `reports/spikes/LAB-004.md` exists with the spike skeleton headings.
- [ ] The report compares photodiode and high-speed camera on H-002 against the same HID injection and calibration source.
- [ ] The report names the sensor, injection path and calibration standard LAB-001 must install, and cites B-020 rather than a latency number (I-061).
- [ ] The report states whether the same fixture class ports to later laptop racks.

#### Verification
- Report: Photodiode or high-speed camera? How is HID injected? What calibration standard is used? Does the capture schema port to H-004 without a new ABI? Which traces may B-020 cite?
- Review: BEN lead records that the recommendation matches the B-020 method.

#### Evidence
- none

### LAB-005 · Decide lab job-scheduler family and unbootable-machine recovery
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: LAB-008
- Baseline: §30
- Decision: D-0170

GAP-0141 needs an accepted choice of scheduler family and a recovery path that uses firmware boot-order fallback to a known-good SystemGeneration, so V1 nightly hardware jobs are not a manual queue. This Decision sits on the same Rung as the spike and before LAB-010. BLD consumes job results; it does not pick the scheduler.

#### Out of scope
Scheduler deployment (LAB-010). CI tiers that submit jobs (BLD-044, BLD-001). Boot-counter implementation (BOOT-018).

#### Acceptance criteria
- [ ] Options evaluated include (A) LAVA, (B) KernelCI and (C) a custom scheduler, each with firmware boot-order fallback to a known-good SystemGeneration.
- [ ] The accepted option names how machines are reserved, how images are flashed, how results return to CI, and which Component or boot path marks a generation good enough to recover onto.
- [ ] The accepted option records that BLD-044 submits jobs and that native software never holds the scheduler API as a Capability.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: LAB and BLD leads sign off on the pull request that accepts the Decision file, citing `reports/spikes/LAB-008.md`.

#### Evidence
- none

### LAB-006 · Procure external power meters for the energy-measurement path
- Type: build
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: LAB-002
- Baseline: §54
- Benchmarks: B-031
- Risks: R-022
- Invariants: I-061

V1 publishes B-031 on the Intel laptop. Software RAPL counters are not evidence. This task buys the external meters and documents their model and accuracy class so PWR-003 can place them on H-004 before the V1 energy gate. Installation into scheduler jobs is LAB-013.

<!-- covers: EXTRA-050 -->

#### Out of scope
Energy methodology and workloads (BEN-018, PWR-003). Meter wiring and drain-cycle jobs (LAB-013). B-031 publication (BEN-024, PWR-006).

#### Acceptance criteria
- [ ] External power meters sufficient for H-004 idle-draw and battery-drain runs named by B-031 are in the lab inventory with model and serial recorded.
- [ ] A repository note names connector type, sample rate class and that RAPL or battery-gauge series are not substitutes for the meter (I-061).
- [ ] The meters are on site before LAB-013 is claimed.

#### Verification
- Manual: operator photographs the meters on the documented shelf or bench and records serials on the pull request.
- Review: BEN lead confirms the models can execute the B-031 method.

#### Evidence
- none

### LAB-007 · Procure and rack the Intel laptop with remote power and console
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: LAB-002, LAB-003, HW-003
- Baseline: §61, §62
- Risks: R-022, R-081
- Invariants: I-074

V1 Wi-Fi roaming, suspend-cycle and B-031 gates name an Intel laptop. H-004 is procured at V0.5 so those gates are not waiting on shipping. V0.5 hardware scope allows exploratory laptop use with no V0.5 laptop gate. The SKU is HW-003; this task racks power, console and a capture path that survives lid close.

#### Out of scope
SKU decision (HW-003). Laptop bring-up (HW-015). Suspend policy (PWR-014). Energy meter attachment (LAB-013).

#### Acceptance criteria
- [ ] H-004 is in the lab on the accepted power and console stack, and `registers/hardware.md` records its SKU once procured.
- [ ] Remote power-off and power-on of H-004 yields a console log, including after a lid-close cycle.
- [ ] The rack record lists IOMMU, TPM 2.0 and Secure Boot enrolment posture required by I-074.
- [ ] Display capture of the internal panel or a USB-C external display is documented for later B-020 reuse.

#### Verification
- Manual: operator power-cycles H-004 from the remote path and attaches console logs, including one lid-close cycle, to the pull request.
- Integration: `lab:tests/rack/h004_power_console_*` on CI matrix entry `hw-h004`.
- Review: HW lead confirms the SKU matches HW-003.

#### Evidence
- none

### LAB-008 · Prototype lab reservation, image flash and unbootable recovery
- Type: spike
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: LAB-007, LAB-003
- Baseline: §30

Without automation the lab is a manual queue CI cannot depend on. This spike runs reservation, image flash and firmware boot-order fallback on H-002 and H-004 under LAVA, KernelCI and a custom controller so LAB-005 is evidence-based. Recovery lands on a known-good SystemGeneration, not a hand-imaged rescue disk.

#### Out of scope
The scheduler Decision (LAB-005). Production deploy (LAB-010). Boot-counter semantics (BOOT-018). CI job submission (BLD-044).

#### Acceptance criteria
- [ ] `reports/spikes/LAB-008.md` exists with the spike skeleton headings.
- [ ] The report compares LAVA, KernelCI and a custom scheduler on H-002 and H-004 for reserve, flash, run and result return.
- [ ] The report records at least one unbootable-kernel recovery via firmware boot-order fallback to a known-good SystemGeneration on each of those machines.
- [ ] The report names which option LAB-005 must evaluate as the default and what BLD-044 would consume.

#### Verification
- Report: Which scheduler family reserved and flashed H-002 and H-004? How did firmware boot-order fallback recover an unbootable image? What job result schema can CI ingest? What operator steps remain after automation?
- Review: BLD lead records that a CI consumer could submit the prototype job schema.

#### Evidence
- none

### LAB-009 · Automate suspend and resume cycle testing on V1 Reference machines
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: LAB-010, LAB-011, LAB-007
- Baseline: §54, §61
- Benchmarks: B-030

V1 exit requires automated suspend and resume on H-004 and H-002 with Wi-Fi, display and audio probed afterwards. PWR-014 owns policy and the RTC-wake harness content; this task is the fixture, power control and scheduler job that repeats those cycles and stores traces B-030 consumes. Lid, dock, hibernate and the V2 cycle-count gate are LAB-014.

#### Out of scope
Suspend state and wake-source policy (PWR-014, PWR-002). Wi-Fi restore (NET-021). Audio restore (AUD-012). Lid and dock fixtures (LAB-014).

#### Acceptance criteria
- [ ] A scheduler job on H-004 and H-002 runs the V1 suspend cycle count named by PWR-014 with RTC wake, without an operator at the chassis.
- [ ] Each cycle captures console, display presence and a post-resume probe slot for Wi-Fi, display and audio.
- [ ] A failed cycle leaves the machine recovered to a booting SystemGeneration via the scheduler recovery path.
- [ ] Cycle traces are stored in a path the B-030 harness can read.

#### Verification
- Integration: `lab:tests/suspend/v1_cycles_*` on `hw-h004` and `hw-h002` through LAB-010.
- Manual: operator confirms one unattended RTC-wake run on H-004 from the scheduler UI or CLI.
- Review: PWR lead confirms the fixture matches PWR-014.

#### Evidence
- none

### LAB-010 · Deploy lab scheduling that reserves, flashes and recovers machines
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: LAB-005, LAB-011, LAB-008, BOOT-018
- Baseline: §30
- Risks: R-081

GAP-0141: the accepted scheduler reserves machines, flashes images, runs jobs and returns results to CI, with automatic recovery of an unbootable machine onto a known-good SystemGeneration. V1 nightly hardware lab (BLD-044) and the hardware regression matrix depend on this deploy. Machine definitions live here; runner cache and secrets stay BLD.

<!-- covers: GAP-0141 -->

#### Out of scope
CI submission and nightly wiring (BLD-044, BLD-034). Boot-success reporting (BOOT-018, SVC). Quiet perf fleet (BLD-048).

#### Acceptance criteria
- [ ] H-002 and H-004 can be reserved, flashed with a CI image, run a smoke job and released through the accepted scheduler, with results visible to CI.
- [ ] An image that fails to reach a console is recovered by firmware boot-order fallback to a known-good SystemGeneration without an operator at the chassis.
- [ ] Job and machine definitions are committed as code in the repository and recreate the V1 lab inventory from that code.
- [ ] A job that panics the kernel is marked failed, the machine is returned to a booting generation, and the failure maps to a task id.
- [ ] Native Components running on the DUT do not hold scheduler, PDU or Redfish Capabilities.

#### Verification
- Integration: `lab:tests/scheduler/reserve_flash_recover_*` on `hw-h002` and `hw-h004`.
- Manual: operator injects an unbootable kernel on H-002 and records automatic recovery on the pull request.
- Review: BLD lead confirms CI can submit the job schema without a second queue.

#### Evidence
- none

### LAB-011 · Equip the lab with netboot, capture, USB switching and radio peers
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: LAB-002, LAB-007, LAB-003
- Baseline: §55, §61, §62
- Benchmarks: B-028
- Invariants: I-054

GAP-0140: the physical lab holding §62 Reference machines needs network boot, consoles, HDMI or DisplayPort capture, USB device switching, programmable HID, two Wi-Fi access points for the V1 roaming gate, Bluetooth peers, and audio loopback for B-028. QEMU cannot validate GPU, suspend, wireless or peripherals, and §55 forbids destabilizing hardware support to fake those tests (I-054).

<!-- covers: GAP-0140, GAP-0372 -->

#### Out of scope
Scheduler software (LAB-010). Wi-Fi association (NET-021). Bluetooth host (HW-035). B-028 harness (AUD-004). Photodiode sensor (LAB-001).

#### Acceptance criteria
- [ ] H-002 and H-004 network-boot a CI image from the lab boot service and reach a console without a local installer key.
- [ ] HDMI or DisplayPort capture on H-002 stores a frame that CI can fetch with the job id.
- [ ] USB switching attaches and detaches a HID device on H-002 and H-004 without recabling, and programmable HID injection is available on both.
- [ ] Two Wi-Fi access points with distinct BSSIDs are reachable from H-004 for the V1 roaming job, and a Bluetooth peer device is present for HW pairing jobs.
- [ ] An analog or digital loopback path exists on H-002 for the B-028 harness.

#### Verification
- Integration: `lab:tests/equip/netboot_capture_usb_radio_*` on `hw-h002` and `hw-h004`.
- Manual: operator runs one netboot, one USB switch cycle, one roam between the two APs, and one loopback tone, and files the artifacts.
- Review: NET and AUD leads confirm the radio and loopback fixtures match their V1 jobs.

#### Evidence
- none

### LAB-012 · Install Linux comparison baselines on lab Reference machines
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: LAB-007, LAB-003, BEN-006
- Baseline: §54
- Invariants: I-061

B-020, B-031 and V1 energy and Input-to-photon gates require a mainline Linux baseline on the same Reference machine. LAB images dual-boot or swap disks; BEN publishes the comparison. Windows dual-boot waits for LAB-015.

#### Out of scope
Pinning the upstream kernel version and distro userspace (BEN-006). B-ID publication (BEN). Windows images (LAB-015). End-user dual-boot policy (INS).

#### Acceptance criteria
- [ ] H-002 and H-004 boot a pinned mainline Linux userspace from the image BEN-006 names, via dual-boot or a swap disk, without replacing the JakeOS SystemGeneration slot.
- [ ] Firmware boot-order documents how an operator or scheduler job selects Linux versus JakeOS.
- [ ] A lab job can boot the Linux image, run a BEN comparison hook, and return to the previous JakeOS generation.
- [ ] No LAB report states a superiority claim against Linux (I-061).

#### Verification
- Integration: `lab:tests/baseline/linux_boot_*` on `hw-h002` and `hw-h004`.
- Manual: operator boots Linux and JakeOS on H-002 from the documented boot-order and files both console banners.
- Review: BEN lead confirms the image identity matches BEN-006.

#### Evidence
- none

### LAB-013 · Instrument lab machines with external meters and battery-drain cycles
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: BEN-018, LAB-010, LAB-006, LAB-007, PWR-003
- Baseline: §54, §61
- Benchmarks: B-031
- Risks: R-022
- Invariants: I-061

V1 publishes B-031 idle draw and battery runtime on H-004. Software counters are not evidence. BEN and PWR own methodology; this task wires the procured meters and drain cycles into scheduler jobs so each release can reproduce the B-031 method on the Intel laptop.

<!-- covers: GAP-0139 -->

#### Out of scope
Meter purchase (LAB-006). Workload definitions (BEN-018, PWR-003). B-031 publication (BEN-024, PWR-006). Platform idle enablement (PWR-009).

#### Acceptance criteria
- [ ] A scheduler job on H-004 records external-meter idle draw using the placement named by PWR-003.
- [ ] A scheduler job runs the B-031 battery-drain cycle on H-004 from full charge to shutdown and stores the meter series with the job id.
- [ ] The job refuses to publish when the external meter is disconnected; RAPL-only runs are marked non-evidence (I-061).
- [ ] The same job schema accepts H-005 once that machine is racked, without a new meter protocol.

#### Verification
- Integration: `lab:tests/energy/h004_meter_drain_*` on `hw-h004` through LAB-010.
- Bench: B-031 traces on H-004 are produced for the harness; target per register.
- Review: BEN lead confirms the job matches the B-031 method.

#### Evidence
- none

### LAB-014 · Automate lid, dock, hibernate and RTC-wake cycles on laptops
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: LAB-009, LAB-018
- Baseline: §62
- Benchmarks: B-030

GAP-0143 and the V2 laptop gate need suspend, resume, hibernate and lid or dock events with RTC wake across the milestone cycle count on each laptop Reference machine. This extends LAB-009 onto H-004 and H-005 with a dock and a second display. PWR and HW own policy daemons; LAB owns the fixture and repetition.

<!-- covers: GAP-0143 -->

#### Out of scope
Lid and dock power policy (PWR-021). Lid Device signals (HW-051). Hibernate product policy (PWR-020). V2 cycle publication (PWR-024, PWR-017).

#### Acceptance criteria
- [ ] Scheduler jobs on H-004 and H-005 actuate lid close and open, dock attach and detach, RTC wake and a hibernate or suspend-then-hibernate path named by PWR, without an operator at the chassis.
- [ ] Each event class captures console and display presence, including the second display on the dock path.
- [ ] The V2 cycle count named by PWR-024 runs unattended on both laptops and stores traces for B-030.
- [ ] A stuck lid or dock actuator fails the job and recovers the laptop to a booting SystemGeneration.

#### Verification
- Integration: `lab:tests/laptop/lid_dock_hibernate_*` on `hw-h004` and `hw-h005`.
- Manual: operator watches one unattended lid and dock sequence on H-005 and files the capture.
- Review: PWR lead confirms event classes match PWR-021.

#### Evidence
- none

### LAB-015 · Install Windows dual-boot baselines on the AMD desktop
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: LAB-012, LAB-003
- Baseline: §54, §56.2
- Invariants: I-061

V2 B-027, HDR and VRR gaming, and battery comparisons need Windows on the same AMD desktop where dual-boot exists. LAB owns the disk image and boot-order; WIN owns W1 scenarios; INS owns end-user dual-boot policy. BEN-047 pins the comparison identity after this image boots.

#### Out of scope
W1 corpus scenarios (WIN). End-user dual-boot installer policy (INS-008, INS-026). Comparison publication (BEN-047, BEN-044). Guest VM Windows (VIRT).

#### Acceptance criteria
- [ ] H-002 dual-boots a pinned Windows image beside the JakeOS SystemGeneration without deleting the Linux comparison slot.
- [ ] Firmware boot-order documents Windows, Linux and JakeOS selection for scheduler jobs and operators.
- [ ] A lab job can boot Windows, run a BEN comparison hook, and return to JakeOS.
- [ ] No LAB report states a superiority claim against Windows (I-061).

#### Verification
- Integration: `lab:tests/baseline/windows_boot_h002_*` on `hw-h002`.
- Manual: operator boots Windows and JakeOS on H-002 from the documented boot-order and files both banners.
- Review: BEN and WIN leads confirm the image is the V2 comparison baseline.

#### Evidence
- none

### LAB-016 · Maintain a firmware version matrix and test vendor updates
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: LAB-010, HW-023
- Baseline: §55, §56.4
- Risks: R-040, R-063
- Invariants: I-054

GAP-0144: firmware silently changes ACPI, TPM and boot behavior. The lab matrix records versions per machine and runs before-and-after vendor updates as scheduler jobs so HW and BOOT stacks see the delta. The firmware update service and LVFS policy remain HW.

<!-- covers: GAP-0144 -->

#### Out of scope
LVFS service and capsule staging (HW-046, BOOT-037). Blob redistribution policy (HW-023). PCR re-seal (SEC, BOOT).

#### Acceptance criteria
- [ ] A committed matrix lists firmware version, vendor, and date-free generation id for every in-lab Reference machine.
- [ ] A scheduler job applies a vendor firmware update on a nominated machine, records before and after versions, and runs the smoke suite both sides.
- [ ] A firmware update that leaves the machine unbootable is recovered by the scheduler fallback path.
- [ ] The matrix is an input to HCL rows; LAB does not publish the HCL.

#### Verification
- Integration: `lab:tests/firmware/matrix_before_after_*` on at least H-002 and H-004.
- Manual: operator applies one vendor update on a spare slot or H-002 and files before/after versions.
- Review: HW lead confirms the matrix fields match HW-046 history events.

#### Evidence
- none

### LAB-017 · Procure an HDR reference display and colorimeter
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: LAB-003
- Baseline: §62
- Risks: R-022

V2 HDR10 and VRR gates need a reference HDR display and colorimeter on H-002. GFX-068 and ICC profiles consume the fixture; LAB owns the hardware. The HDR pipeline Decision stays GFX.

<!-- covers: EXTRA-051 -->

#### Out of scope
HDR output pipeline and tone mapping (GFX-068, GFX-063). ICC application (GFX-073). VRR verification logic (GFX-088).

#### Acceptance criteria
- [ ] An HDR display capable of HDR10 and VRR is attached to H-002 and named in the lab inventory.
- [ ] A colorimeter that GFX-073 can drive is in the lab inventory with model and serial recorded.
- [ ] Capture and photon fixtures on H-002 still function with the HDR display connected.
- [ ] The rack note records connector, peak-brightness class and that LAB does not publish HDR latency numbers.

#### Verification
- Manual: operator photographs the HDR display and colorimeter on H-002 and files serials on the pull request.
- Integration: `lab:tests/hdr/h002_display_present_*` on `hw-h002`.
- Review: GFX lead confirms the display and colorimeter can execute the V2 HDR and ICC jobs.

#### Evidence
- none

### LAB-018 · Procure and rack V2 target machines, dock and extra displays
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-018, HW-003, LAB-011, LAB-007
- Baseline: §62
- Risks: R-081
- Invariants: I-074

V2 hardware scope is the AMD desktop, Intel laptop and AMD laptop, plus an experimental NVIDIA desktop with no gate. This task racks H-005, a dock, a second display, and H-006 as experimental, and attaches Input-to-photon and energy fixtures so B-020 at the refresh rates named by the register, B-031 per laptop, and multi-monitor or lid-dock jobs can run. SKUs follow HW target and NVIDIA adrs.

#### Out of scope
AMD laptop bring-up (HW-039). Experimental NVIDIA driver path (HW-052). MST compositor support (GFX-075). Lid policy (PWR).

#### Acceptance criteria
- [ ] H-005 is in the lab on the accepted power and console stack, and `registers/hardware.md` records its SKU.
- [ ] A dock and a second display are attached so H-004 and H-005 can run a two-display job without recabling.
- [ ] H-006 is racked as experimental with power and console, and no V2 gate job requires it to pass.
- [ ] Photon and energy fixtures from earlier rungs are documented as attachable on the V2 laptops.

#### Verification
- Manual: operator power-cycles H-005 and H-006 from the remote path and files console logs.
- Integration: `lab:tests/rack/v2_targets_*` on `hw-h005` (H-006 logged as non-gating).
- Review: HW lead confirms SKUs match HW-003 and HW-018.

#### Evidence
- none

### LAB-019 · Run Vulkan CTS and compositor tests nightly on lab GPUs
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: LAB-010, LAB-018, GFX-027
- Baseline: §56.1
- Invariants: I-054

GAP-0142 and §56.1: nightly Vulkan CTS, dEQP and compositor tests on AMD, Intel and NVIDIA lab machines, with pass rates tracked per driver generation. NVIDIA remains non-gating at V2. GFX owns compositor test content; LAB runs the jobs on hardware via the scheduler and does not rewrite DRM (I-054).

<!-- covers: GAP-0142 -->

#### Out of scope
Compositor test content (GFX). Kernel DRM regression matrix content (GFX-027). NVIDIA driver policy (HW-018, GFX-064). HCL publication (REL, HW).

#### Acceptance criteria
- [ ] Nightly scheduler jobs run Vulkan CTS, dEQP and the GFX compositor suite on H-002 (AMD) and H-004 (Intel) and publish pass rates per driver generation id.
- [ ] A nightly job definition exists for H-006 and is marked non-gating at V2; a red NVIDIA run does not fail V2 qualification.
- [ ] Job logs map failures to a task id and retain the driver package version.
- [ ] A DUT kernel panic during CTS is recovered by the scheduler fallback path.

#### Verification
- Integration: `lab:tests/gpu/cts_nightly_*` on `hw-h002` and `hw-h004`, with `hw-h006` optional and non-gating.
- Review: GFX lead confirms the suites are the compositor and CTS content GFX owns.
- Manual: operator inspects one nightly result bundle and files the pass-rate artifact path.

#### Evidence
- none

### LAB-020 · Procure assistive-technology hardware including a braille display
- Type: build
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: LAB-011
- Baseline: §41

ACC-027 lands braille at V3 so the V4 assistive-technology script is not the first braille run. LAB procures a USB braille display (and a Bluetooth display if the SKU exists) and attaches it through the USB switch from LAB-011. Screen-reader software is ACC.

#### Out of scope
Braille protocol and screen reader (ACC-027). Bluetooth host (HW-035). HID service (HW-011). V4 50-task script (ACC-031).

#### Acceptance criteria
- [ ] A USB braille display is in the lab inventory with model and serial recorded and is attached through the USB switch to a named Reference machine.
- [ ] A scheduler or operator procedure selects that machine and presents the display to ACC-027 without recabling.
- [ ] If a Bluetooth braille display is procured, it is listed as a radio peer beside the V2 Bluetooth peers.
- [ ] Native software on the DUT does not open a vendor POSIX node to talk to the display; ACC consumes Device Capabilities.

#### Verification
- Manual: operator attaches the USB display through the switch and files a photo and serial on the pull request.
- Integration: `lab:tests/at/braille_present_*` on the machine the inventory names.
- Review: ACC lead confirms the model is one ACC-027 can drive.

#### Evidence
- none

### LAB-021 · Procure and rack the six-machine V3 Tier 1 lab fleet
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: HW-018, LAB-018
- Baseline: §63
- Risks: R-081
- Invariants: I-074

V3 hardware scope is the three V2 targets plus one NVIDIA desktop (gated) and one extra laptop per vendor: H-002, H-004, H-005, H-006, H-007 and H-008, six machines fully tested each release. Lab build-out spans V1 through V3 and sits on the critical path of later gates. NVIDIA SKU follows HW-018; extra laptop SKUs follow HW bring-up tasks after this rack.

#### Out of scope
Extra laptop bring-up (HW-062). NVIDIA Tier 1 driver bring-up (HW-070). Installer qualification jobs (LAB-022). HCL publication (REL-048).

#### Acceptance criteria
- [ ] H-006, H-007 and H-008 are in the lab on the accepted power, console and capture stack, and `registers/hardware.md` records their SKUs.
- [ ] All six V3 hardware-scope machines accept scheduler reserve, flash and recover jobs.
- [ ] Each new machine record lists IOMMU, TPM 2.0 and Secure Boot enrolment posture required by I-074.
- [ ] Photon, energy, dock and radio fixtures are documented per machine or marked not-applicable with a reason.

#### Verification
- Manual: operator power-cycles H-006, H-007 and H-008 from the remote path and files console logs.
- Integration: `lab:tests/rack/tier1_six_*` on the six V3 H-IDs.
- Review: HW lead confirms SKUs match the V3 hardware scope.

#### Evidence
- none

### LAB-022 · Run installer and Secure Boot qualification across the Tier 1 fleet
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: BOOT-031, BOOT-044, INS-027, LAB-010, LAB-021
- Baseline: §63

The V3 installer gate (automated runs across the fleet with FDE) and Secure Boot on every Tier 1 machine need LAB scheduler jobs, key enrolment and capture. INS owns the installer; BOOT owns Secure Boot strategy; LAB drives the fleet and recovery when an install leaves a machine unbootable.

#### Out of scope
Installer implementation (INS-027, INS-032). Secure Boot strategy and shim (BOOT-031, BOOT-044). HCL verdict content (INS-028, HW-066). Signing keys (REL).

#### Acceptance criteria
- [ ] Scheduler jobs run the graphical installer with FDE on every V3 Tier 1 machine and store console plus capture for each run.
- [ ] Scheduler jobs boot each of those machines with Secure Boot enabled on the path BOOT documents, and record enrolment state.
- [ ] An installer or enrolment failure that leaves a machine unbootable is recovered to a known-good SystemGeneration.
- [ ] Per-machine result files name H-ID, generation id and pass or fail; LAB does not set the INS success-rate gate.

#### Verification
- Integration: `lab:tests/qualify/installer_secureboot_*` on every V3 Tier 1 H-ID.
- Manual: operator enrols keys on one machine through the documented path and files capture.
- Review: INS and BOOT leads confirm the jobs invoke their install and Secure Boot artifacts.

#### Evidence
- none

### LAB-023 · Procure and rack the ten-machine V4 Tier 1 lab fleet
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: LAB-021
- Baseline: §62, §63
- Risks: R-081
- Invariants: I-074

V4 hardware scope and demo require at least ten named Tier 1 machines (second GPU generations, Intel desktop, NVIDIA, laptops per vendor) and a ten-machine run of the full suite with per-machine results. This task racks H-009 through H-014 beside the V3 six so the V4 hardware-scope list in `registers/hardware.md` is physically present. HCL publication stays HW and REL.

#### Out of scope
Second-generation bring-up (HW-081, HW-080). Combined hardware suite content (HW-086). HCL publication (HW-088, REL). Nightly CI wiring (BLD-078).

#### Acceptance criteria
- [ ] H-009, H-010, H-011, H-012, H-013 and H-014 are in the lab on the accepted power, console and capture stack, with SKUs recorded.
- [ ] At least the ten named V4 Tier 1 machines accept scheduler reserve, flash and recover jobs.
- [ ] Each new machine record lists IOMMU, TPM 2.0 and Secure Boot enrolment posture required by I-074.
- [ ] A fleet inventory view lists every V4 hardware-scope H-ID with fixture attachments or an explicit not-applicable reason.

#### Verification
- Manual: operator power-cycles each newly racked H-ID and files console logs.
- Integration: `lab:tests/rack/tier1_ten_*` on the V4 hardware-scope H-IDs.
- Review: HW lead confirms the fleet matches V4 hardware scope.

#### Evidence
- none

### LAB-024 · Run multi-day soak and release-candidate fleet soaks on Tier 1
- Type: build
- Milestone: V4
- Status: todo
- Size: L
- Owner: none
- Depends on: LAB-014, LAB-010, LAB-023, OBS-036
- Baseline: §54
- Risks: R-063
- Invariants: I-061

GAP-0134: multi-day soaks covering suspend and resume, SystemGeneration accumulation, repeated application launches and long compositor uptime, with memory and latency trend alerts. This also runs the V4 release-candidate soak on Tier 1. OBS leak detection and BEN dashboards consume the traces; LAB owns calendar and execution. Firmware refreshes during soak are tracked against LAB-016 (R-063).

<!-- covers: GAP-0134 -->

#### Out of scope
Leak detector implementation (OBS-036). Public dashboards (BEN-057). CI matrix ownership (BLD-076). Channel go/no-go (REL).

#### Acceptance criteria
- [ ] Scheduler soak jobs covering suspend and resume, generation accumulation, repeated launches and compositor uptime exist for every V4 hardware-scope H-ID.
- [ ] Memory and latency trend series from those jobs are exported to the OBS and BEN consumers, with no superiority claim in LAB prose (I-061).
- [ ] An unbootable machine during soak is recovered by the scheduler fallback path and the incident is recorded against the generation id and H-ID.
- [ ] Each V4 release candidate named by BLD or REL has a fleet soak result bundle with per-machine pass or fail.
- [ ] Firmware versions in the soak bundle match LAB-016 or the mismatch is recorded as a defect.

#### Verification
- Integration: `lab:tests/soak/multiday_rc_*` on the V4 hardware-scope H-IDs.
- Manual: operator starts one soak window on the fleet and files the result-bundle path.
- Review: OBS and BEN leads confirm traces match leak and dashboard inputs.

#### Evidence
- none

### LAB-025 · Run the public-stable soak on the full Tier 1 fleet
- Type: build
- Milestone: 1.0
- Status: todo
- Size: M
- Owner: none
- Depends on: LAB-023, LAB-024
- Baseline: §54
- Risks: R-063
- Invariants: I-061

1.0 exit soaks the final release candidate on the full Tier 1 fleet. REL owns channel launch and go/no-go; LAB executes the fleet soak and publishes per-machine hardware results that HW and REL fold into the HCL. Zero open P0 or P1 is a process verdict REL records, not a LAB checkbox.

#### Out of scope
Stable channel launch (REL-063, REL-066). HCL publication (HW-088, REL-048). Combined feature sign-off (HW-089). Crash-free rate publication (BEN, REL).

#### Acceptance criteria
- [ ] Every H-ID in the 1.0 hardware scope completes the soak job schema used by LAB-024 on the candidate generation REL names.
- [ ] Per-machine result files list soak outcome, firmware version, fixture set and generation id, and are the lab input to HCL rows.
- [ ] Unbootable machines during the soak are recovered and recorded; they do not disappear from the result set.
- [ ] LAB reports cite B-IDs and H-IDs and contain no superiority claim (I-061).

#### Verification
- Integration: `lab:tests/soak/stable_fleet_*` on every 1.0 hardware-scope H-ID.
- Demo: per-machine hardware result bundle produced for the candidate generation on the Tier 1 fleet.
- Review: REL and HW leads confirm the bundle is the lab evidence their HCL and launch tasks consume.

#### Evidence
- none
