# PWR · Power management
- Prefix: PWR
- Lead: none
- Baseline: §22, §54, §61, §62
- Baseline gap: §61 and §62 require power management, suspend/resume and laptop batteries as gates but specify no Power Component, inhibit model, thermal policy, display-power policy or energy method.

<!-- roadmap:generated:begin summary -->
Tasks: 30 live, 0 done, 0 in-progress, 30 todo, 0 dropped. Ready: 0. Blocked: 30. Weighted: 0%.
<!-- roadmap:generated:end -->

## Scope

PWR owns the native power plane that makes a laptop a daily driver: the Layer 2 Power Component, suspend and resume, hibernation policy, Battery reporting and charge thresholds, thermal policy, display power (DPMS, panel self-refresh, idle dim and off), brightness, lid and dock behavior, and idle or suspend inhibition as Capabilities. Frequency and idle-state selection consume Scheduling intent and ResourceDomain energy policy; PWR enables and constrains the retained platform (ACPI, runtime PM, cpufreq, RAPL) rather than rewriting it (§2, §22, §23).

It owns the RTC-wake cycle harness used by V1 and later suspend gates, wake-source policy, inspectable power state (battery, thermal, inhibitors, suspend, wake sources, governor), and the energy-measurement method run against B-030 and B-031 with BEN. Native software talks typed Interfaces over Channels. Personality UPower and logind shims, Settings chrome, and lab fixtures live elsewhere.

## Out of scope

Scheduling intent mapping, ResourceDomain budgets and thermal-intent consumption (SCH). Thermal-zone drivers, function keys, lid and tablet switches, sensor-class scope, SKU bring-up (HW). Machine procurement, power meters, lid and RTC fixtures, soak calendar (LAB). Benchmark register, cross-OS publication policy (BEN). Battery indicator, brightness slider, lock UI, power Settings (APP). Session lock and idle, disk-key eviction, FDE (SEC). Swap and hibernation-image layout (STO). Supervisor, init, shutdown sequencing (SVC). DRM/KMS scanout and compositor lock mode (GFX). Audio restore after resume (AUD). Wi-Fi restore after resume (NET). MemoryObject pin survival (MEM). Operation deadline clock across suspend (TSK). Inspect transport and `os inspect` rendering (OBS, SDK). UPower and logind D-Bus (LNX). Capability mint and rights encoding (CAP). IDL and Layer 2 evolution rules (IPC). Surfaces register (ABI). HCL publication (REL, HW). ACPI retain inventory (KRN). Generation selection (BOOT).

## Tasks

### PWR-001 · Decide the Layer 2 power service model over retained ACPI
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: PWR-005, KRN-001
- Baseline: §2, §22, §32, §61, §66
- Decision: D-0232
- Invariants: I-009, I-010, I-054, I-055, I-040

§61 names power management as a V1 requirement without a Power Component, an inhibit model, or a statement of whether ACPI is a kernel Object. This Decision is the baseline-gap scope for PWR: where suspend, shutdown, Battery, display power and inhibit live relative to retained Linux ACPI, and which of those operations are Layer 2 Interfaces versus Layer 1. ACPI is a mature mechanism (§2); the native programming model is not logind.

#### Out of scope
Suspend state s2idle versus S3 (PWR-002). Hibernation product policy (PWR-007). UPower and logind D-Bus (LNX-069). Service supervisor (SVC-015).

#### Acceptance criteria
- [ ] Options evaluated include (A) a userspace Power Component over retained Linux ACPI, (B) a native kernel Object<Power> as Layer 1, and (C) a logind-shaped native API.
- [ ] The accepted option names ACPI as a retained mechanism (I-009, I-054) and does not schedule a rewrite of the ACPI subsystem.
- [ ] The accepted option lists which of Power, Battery, DisplayPower, InhibitIdle and InhibitSuspend are Layer 2 Interfaces, and states that no Layer 1 power surface freezes before V4 (I-040).
- [ ] Native software is recorded as never seeing logind, sysfs or POSIX power APIs, including if option C is rejected under §57 and I-006.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: architecture, ABI and kernel leads sign off on the pull request that accepts the Decision file.

#### Evidence
- none

### PWR-002 · Decide the suspend state for V1 reference machines
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: PWR-004, PWR-001
- Baseline: §61
- Decision: D-0233
- Risks: R-027

V1-G07 requires automated suspend and resume on H-004 and H-002. The platform offers s2idle and S3; mixing them without a Decision makes the cycle harness and wake-source policy untestable. Hibernation is a later Decision. This records one default suspend state for V1 reference machines and the rule for a per-machine fallback.

#### Out of scope
Hibernation product policy (PWR-007). Disk-key eviction on suspend (SEC-031). Cycle harness (PWR-014).

#### Acceptance criteria
- [ ] Options evaluated include (A) s2idle only, (B) S3 only, and (C) s2idle default with per-machine S3 fallback.
- [ ] The accepted option names the default state for H-004 and for H-002, the inspectable identifier of that state, and the condition that selects a fallback.
- [ ] The accepted option lists lid, power button and RTC as wake sources in scope for V1, and USB wake as off by default.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: PWR and HW leads sign off on the pull request that accepts the Decision file, citing `reports/spikes/PWR-004.md`.

#### Evidence
- none

### PWR-003 · Prototype the energy measurement method against Linux on the Intel laptop
- Type: spike
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: BEN-018, LAB-006, LAB-007
- Baseline: §54, §61
- Risks: R-022
- Invariants: I-061

V1-G17 publishes B-031 on H-004. BEN owns the register method; this spike places the meters, names idle and mixed workloads, and records comparison-image rules on the Intel laptop so the V1 energy gate is not the first time the path runs. Software counters are not evidence.

#### Out of scope
Meter procurement (LAB-006). Register ownership and V1 publication (BEN-024, PWR-006). Drain-cycle CI jobs (LAB-013).

#### Acceptance criteria
- [ ] `reports/spikes/PWR-003.md` exists with the spike skeleton headings.
- [ ] The report records meter placement on H-004, the idle and mixed workloads, and the comparison-image rule against a mainline Linux distribution on the same machine.
- [ ] The report compares external meter, battery gauge and RAPL or powercap samples and names which series B-031 may cite.
- [ ] The report recommends an option set for the V1 B-031 run without encoding a superiority claim (I-061).

#### Verification
- Report: Where is the external meter attached on H-004? Which idle and mixed workloads are scripted, and how is the comparison Linux image pinned? Which of external meter, battery gauge and RAPL or powercap is admissible evidence for B-031? What must LAB-013 automate before V1-G17?
- Review: BEN and LAB leads record that the method matches the B-031 register.

#### Evidence
- none

### PWR-004 · Probe suspend, battery, thermal and frequency paths on reference machines
- Type: spike
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: LAB-007, LAB-003, HW-004
- Baseline: §61
- Risks: R-022, R-027
- Invariants: I-054

V1 Intel-laptop suspend and battery gates are a cliff unless the platform paths are known at V0.5. This spike probes s2idle versus S3, cpufreq drivers, battery and thermal platform nodes, and lid or power-button events on H-004 and H-002 without making any of those a V0.5 gate. ACPI remains a retained mechanism (I-054).

#### Out of scope
SKU bring-up for daily-driving (HW-015). Suspend-state Decision (PWR-002). Cycle harness (PWR-014).

#### Acceptance criteria
- [ ] `reports/spikes/PWR-004.md` exists with the spike skeleton headings.
- [ ] The report records, for H-004 and H-002, whether s2idle and S3 complete a round trip, which cpufreq driver is bound, and whether battery, thermal, lid and power-button events are visible to the retained platform.
- [ ] The report names wake sources observed and whether USB wake was left off.
- [ ] The report recommends an option set for PWR-002 without selecting it.

#### Verification
- Report: Does s2idle complete on H-004 and on H-002? Does S3 complete on each? Which cpufreq driver and EPP interface are bound? Are battery, thermal, lid and power-button events enumerable without a native rewrite? Which wake sources fired, and was USB wake left off?
- Manual: Probe procedure on H-004 and H-002 recorded in the report.
- Review: HW lead confirms the machines used are the register SKUs.

#### Evidence
- none

### PWR-005 · Prototype Layer 2 Power, Battery and inhibit Interface shapes
- Type: spike
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: IPC-012, IPC-035, CAP-003
- Baseline: §12, §32, §65, §66
- Invariants: I-040, I-055

Freeze discipline requires a prototype before the service-model Decision. This spike builds Power, Battery, DisplayPower, InhibitIdle and InhibitSuspend as typed Interfaces over Channels so V1 implementation is not a paper API. Nothing Layer 1 is frozen (I-040). High-level power policy stays out of the kernel ABI (I-055).

#### Out of scope
The service-model Decision (PWR-001). Freeze-candidate declaration (PWR-008). Production Power Component (PWR-013).

#### Acceptance criteria
- [ ] `reports/spikes/PWR-005.md` exists with the spike skeleton headings.
- [ ] A prototype Component exposes Power, Battery, DisplayPower, InhibitIdle and InhibitSuspend over Channels generated from IDL, exercised on `qemu-x86_64`.
- [ ] The report records which operations allocate, which are Operations, and how missing rights return a typed error.
- [ ] The report recommends an option set for PWR-001 without selecting it, and states that no Layer 1 power surface exists to freeze.

#### Verification
- Report: Which of Power, Battery, DisplayPower, InhibitIdle and InhibitSuspend are Channel Interfaces versus kernel Objects? How does a holder without rights fail, and is it `Error::Rights`? Which operations are `Operation<Result>` (suspend, shutdown) versus queries? What remains Layer 2 so I-040 holds?
- Unit: `runtime:tests/pwr/spike_interfaces_*` on `qemu-x86_64`.
- Review: ABI and IPC leads record that no new Layer 1 surface was introduced.

#### Evidence
- none

### PWR-006 · Publish idle power draw and battery runtime on the Intel laptop
- Type: benchmark
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: PWR-003, PWR-009, PWR-010, LAB-013, BEN-018
- Baseline: §54, §61
- Benchmarks: B-031
- Risks: R-022, R-027
- Invariants: I-061

V1-G17 is satisfied when the B-031 target for V1 is met on H-004 beside a mainline Linux distribution on the same machine. This run uses the V0.5 energy method and LAB meters. BEN owns the register. No superiority claim is made.

#### Out of scope
Register definition (BEN). Meter hardware (LAB-006). Mixed-workload publication on H-005 (BEN-039, PWR-023).

#### Acceptance criteria
- [ ] A report exists under `reports/benchmarks/B-031/` for H-004 meeting the register target kind for V1.
- [ ] The report includes idle draw from the external meter and mixed-workload battery runtime beside the pinned Linux image on H-004.
- [ ] The report cites B-031 and states no superiority claim (I-061).

#### Verification
- Bench: B-031 on H-004; target per register.
- Review: BEN lead confirms the series names match the register method.

#### Evidence
- none

### PWR-007 · Decide hibernation policy for 1.0
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: PWR-002, SEC-031, STO-014
- Baseline: §61, §62
- Decision: D-0231
- Threats: T-009, T-010

V2 lists hibernate as optional. Shipping a hibernation image without a Decision leaves T-010 (leftover image) and T-009 (suspended keys) unaddressed. This Decision chooses whether 1.0 has no hibernate, suspend-then-hibernate after idle, or full hibernate with an encrypted image. SEC owns disk-key eviction and lockdown; STO owns swap layout; this Decision is the power-product choice those workstreams constrain.

#### Out of scope
Disk-key eviction and lockdown (SEC-031). Swap and image layout (STO-014). V2 delivery (PWR-020).

#### Acceptance criteria
- [ ] Options evaluated include (A) no hibernate in 1.0, (B) suspend-then-hibernate after idle, and (C) full hibernate with an encrypted authenticated image.
- [ ] The accepted option states whether a hibernation image may exist on H-004 and H-005, and cites SEC-031 for key handling under lockdown.
- [ ] The accepted option names the inspectable unsupported status used when hibernate is not delivered.
- [ ] A Review line names who accepts the Decision.

#### Verification
- Review: PWR, SEC and STO leads sign off on the pull request that accepts the Decision file.

#### Evidence
- none

### PWR-008 · Declare Layer 2 power Interface freeze candidates
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: PWR-005, PWR-001, PWR-013, PWR-010, PWR-011, PWR-012, ABI-034, IPC-035
- Baseline: §65, §66
- Invariants: I-040

Layer 2 evolution rules freeze at V1 with SDK v1; versions lock at V4. This task names Power, Battery, DisplayPower, InhibitIdle and InhibitSuspend as freeze candidates, each citing the V0.5 spike and service-model Decision. ABI owns the surfaces register. Nothing Layer 1 is frozen (I-040).

#### Out of scope
Accepting a Layer 1 freeze (ABI-049). Locking Layer 2 versions (IPC-068). Register file edits that invent S-IDs (ABI).

#### Acceptance criteria
- [ ] A review record lists Power, Battery, DisplayPower, InhibitIdle and InhibitSuspend as Layer 2 freeze candidates and states they are not `frozen`.
- [ ] Each candidate cites PWR-005 and PWR-001 in its closure.
- [ ] The record states that no Layer 1 power surface exists to freeze (I-040).
- [ ] ABI-034 lists these Interfaces as Layer 2 candidates, not Layer 1.

#### Verification
- Review: PWR, ABI and SDK leads record freeze-candidate status on the pull request.
- Integration: CI fails if any of those Interfaces is marked `frozen` before V4.

#### Evidence
- none

### PWR-009 · Enable platform idle and frequency power management on reference machines
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: PWR-004, PWR-002, SCH-038, HW-015
- Baseline: §22, §54, §61
- Risks: R-027
- Invariants: I-009, I-032, I-054

V1 idle-power publication on B-031 needs retained Linux runtime PM, ASPM, NVMe APST and cpufreq or EPP modes that honor Scheduling intent, not a rewritten governor (§22). SCH owns hint mapping. PWR enables those platform features on H-002 and H-004 and constrains them so Background and EnergyEfficient work can sit in lower power states.

<!-- covers: INV-1206, GAP-0313 -->

#### Out of scope
Hint mapping from intent to cpufreq and EPP (SCH-038). EnergyEfficient class (SCH-037). Thermal trip policy (PWR-022). Governor rewrite.

#### Acceptance criteria
- [ ] On H-004 and H-002, runtime PM, ASPM, NVMe APST and the cpufreq driver recorded by PWR-004 are enabled, and `os inspect power` reports that set.
- [ ] A Task under EnergyEfficient intent produces a lower cpufreq or EPP ceiling than the same Task under Interactive intent, visible in inspect, with no native cpufreq sysfs API.
- [ ] CI on `hw-h004` fails if a native Component can open a Linux cpufreq or powercap node as its power Interface.
- [ ] No ACPI, cpufreq or RAPL subsystem is replaced (I-009, I-054).

#### Verification
- Integration: `runtime:tests/pwr/platform_pm_*` on `hw-h002` and `hw-h004`.
- Review: SCH lead confirms PWR consumes SCH-038 rather than duplicating the map.

#### Evidence
- none

### PWR-010 · Implement Battery reporting for the Intel laptop
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: PWR-013, PWR-004, HW-015
- Baseline: §61
- Risks: R-027

V1 daily-driving on H-004 needs charge fraction, rate, AC or DC, and a remaining-time estimate from platform data as a typed Battery Interface. APP owns the indicator. Estimate accuracy against measured runtime is a V2 gate.

<!-- covers: INV-1220 -->

#### Out of scope
Settings indicator (APP-035). Charge thresholds and profiles (PWR-019). Estimate-vs-runtime proof (PWR-023).

#### Acceptance criteria
- [ ] On H-004, the Battery Interface reports charge fraction, charge or discharge rate, AC or DC, and a remaining-time estimate, all visible through `os inspect power`.
- [ ] Unplugging AC flips the AC or DC field without restarting the Power Component.
- [ ] A Component without the Battery Capability receives `Error::Rights` and reads no charge data.
- [ ] Native software never opens a Linux power-supply sysfs node as its Battery Interface.

#### Verification
- Integration: `runtime:tests/pwr/battery_report_*` on `hw-h004`.
- Demo: V1-D02 on H-004 shows inspectable battery state during the session.
- Review: APP lead confirms the indicator can bind this Interface without a second source of truth.

#### Evidence
- none

### PWR-011 · Implement display power management tied to idle, lock and lid
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: PWR-013, PWR-012, SEC-028, GFX-008, GFX-045, GFX-053
- Baseline: §61

V1 notes display power management in scope on H-004: DPMS, panel self-refresh, idle dim and off, tied to Session idle, lock and lid. GFX retains KMS. Full lid-close-to-suspend policy is V2. InhibitIdle held by a Component delays dim and off until the Capability is dropped.

<!-- covers: EXTRA-021 -->

#### Out of scope
Lid-close-to-suspend and dock policy (PWR-021). Brightness and ALS (PWR-018). Lock UI (APP-033). KMS drivers (GFX).

#### Acceptance criteria
- [ ] On H-004, Session idle dim then blanks the internal panel through KMS DPMS or equivalent, and activity restores scanout, recorded by `runtime:tests/pwr/display_idle_*`.
- [ ] Session lock blanks the panel; activity without unlock does not restore scanout of an unlocked session.
- [ ] Lid-close blanks the internal panel when no InhibitIdle Capability is held; an InhibitIdle holder keeps the panel on until the Capability is dropped.
- [ ] Native software never calls a POSIX DPMS or sysfs backlight node as its DisplayPower Interface.

#### Verification
- Integration: `runtime:tests/pwr/display_idle_*` and `runtime:tests/pwr/display_lock_*` on `hw-h004`.
- Manual: lid-close blanks the internal panel on H-004; opening it restores scanout without a full resume if the machine did not suspend.
- Review: GFX lead confirms DPMS and panel self-refresh go through retained KMS.

#### Evidence
- none

### PWR-012 · Implement InhibitIdle and InhibitSuspend Capabilities
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: PWR-001, PWR-005, PWR-013, CAP-036, CAP-003, SEC-028
- Baseline: §9, §9.1, §61
- Invariants: I-021

Video playback and downloads must keep the panel on or the machine awake without ambient D-Bus inhibitors (I-021). Native software holds `Capability<InhibitIdle>` or `Capability<InhibitSuspend>` granted per Component, listed by inspect, and dropped on teardown. LNX owns UPower and logind shims. APP owns shell visibility of the list.

<!-- covers: GAP-0285 -->

#### Out of scope
Shell inhibitor list UI (APP-039). UPower and logind inhibitors (LNX-069). Display-off policy (PWR-011).

#### Acceptance criteria
- [ ] A Component granted `Capability<InhibitIdle>` prevents idle dim and off for as long as it holds the Capability; dropping it resumes DisplayPower idle policy.
- [ ] A Component granted `Capability<InhibitSuspend>` causes a suspend Operation to return a typed inhibit error and leave the machine running until the Capability is dropped.
- [ ] `os inspect power` lists each inhibitor with Component identity and whether it is idle or suspend; a Component without the matching grant receives `Error::Rights` and allocates no handle.
- [ ] Native software has no D-Bus inhibitor path; a test that a native Component cannot take an inhibit grant through a Linux personality bus fails closed.

#### Verification
- Unit: `runtime:tests/pwr/inhibit_rights_*` on `qemu-x86_64`.
- Integration: `runtime:tests/pwr/inhibit_idle_*` and `runtime:tests/pwr/inhibit_suspend_*` on `hw-h004`.
- Review: CAP lead confirms rights are in the object-rights registry and are attenuable.

#### Evidence
- none

### PWR-013 · Implement the Power Component with suspend and shutdown Operations
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: PWR-001, PWR-005, SVC-015, SVC-011, SVC-007, IPC-035, CAP-003
- Baseline: §32, §61
- Invariants: I-037, I-006

§61 requires power management as a native service. This is the accepted Layer 2 Power Component: suspend, resume, shutdown and reboot as Operations, restartable under SVC, invoked by init rather than by logind. Native software never sees logind, systemd or POSIX reboot APIs (I-006).

#### Out of scope
Shutdown ordering and generation switch (SVC-030). Cycle soak (PWR-014). Hibernate image (PWR-020). UPower (LNX-069).

#### Acceptance criteria
- [ ] The Power Component is a supervised service whose manifest declares Power operations; killing it restarts it under SVC and clients rebind (I-037), visible in `os inspect service`.
- [ ] Suspend, shutdown and reboot are `Operation<Result>` held behind a Capability; a caller without rights receives `Error::Rights` and the machine does not change power state.
- [ ] A shutdown Operation is the path SVC-030 calls; native software has no `/sbin/shutdown`, logind or POSIX reboot entry point.
- [ ] On `qemu-x86_64`, unit tests complete suspend, shutdown and reboot Operations against a fake platform backend without touching host power state.

#### Verification
- Unit: `runtime:tests/pwr/power_ops_*` on `qemu-x86_64`.
- Integration: `runtime:tests/pwr/power_supervise_*` on `qemu-x86_64` and `hw-h002`.
- Review: SVC lead confirms the service manifest and restart path match S-020 usage without a second supervisor.

#### Evidence
- none

### PWR-014 · Implement suspend and resume cycle gates on laptop and desktop
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: PWR-002, PWR-013, PWR-011, PWR-015, LAB-009, LAB-011, HW-015, GFX-053, BLD-044, SEC-028
- Baseline: §61
- Risks: R-027
- Threats: T-009

V1-G07 requires suspend and resume to succeed for the automated cycle counts named here on H-004 and H-002, with Wi-Fi, display and audio functional afterwards. This task owns the RTC-wake cycle harness, wake-source policy (lid, power button, RTC; USB wake off by default) and the post-resume hook table that MEM, TSK, AUD and NET register into. The accepted suspend state is the one PWR-002 recorded.

<!-- covers: INV-1207, INV-1221 -->

#### Out of scope
Wi-Fi reconnect implementation (NET-021). Audio stream restore (AUD-012). MemoryObject pin survival (MEM-044). Deadline clock (TSK-041, SVC-016). Lid and dock policy (PWR-021). Disk-key eviction (SEC-031). Lab fixture hardware (LAB-009).

#### Acceptance criteria
- [ ] 200 of 200 automated RTC-wake suspend and resume cycles succeed on H-004, and 100 of 100 succeed on H-002, each resuming to an interactive desktop.
- [ ] After every successful cycle the harness records display scanning out; Wi-Fi and audio hooks either report functional or report a missing registrant, and a missing registrant fails the cycle.
- [ ] Wake sources in use are lid, power button and RTC; USB wake is off by default and `os inspect power` shows that policy.
- [ ] The harness uses the suspend state named by PWR-002; a mismatch fails the run.
- [ ] Native software never issues a Linux `reboot(2)` or sysfs power-state write as its suspend Interface.

#### Verification
- Integration: `runtime:tests/pwr/suspend_cycles_*` on `hw-h004` and `hw-h002` via LAB-009.
- Demo: V1-D02 on H-004 includes a mid-day suspend and resume.
- Review: NET, AUD, GFX, MEM and TSK leads confirm their post-resume hooks are registered or explicitly absent with a tracking task.

#### Evidence
- none

### PWR-015 · Expose battery, thermal, inhibit and suspend state through inspect
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: PWR-013, PWR-010, PWR-012, OBS-019, SDK-007
- Baseline: §24, §61, §64
- Invariants: I-034

V1 daily-driving and the suspend and battery gates are unverifiable without inspectable power state. PWR supplies Battery, inhibitors, suspend state, wake sources, governor and platform thermal readings to the OBS provider path. SDK owns the `os inspect` command.

#### Out of scope
`os inspect` CLI (SDK-007). Inspect transport (OBS-019). Thermal trip policy (PWR-022).

#### Acceptance criteria
- [ ] `os inspect power` on H-004 prints Battery fields, inhibitor list, current suspend state, enabled wake sources and the active governor or EPP mode.
- [ ] Platform thermal readings for H-004 appear as inspectable values without granting a native sysfs path.
- [ ] A Component without inspect rights on the Power service receives `Error::Rights` and no payload.
- [ ] The Power provider is registered in the same change that ships the Power Component (I-034).

#### Verification
- Integration: `runtime:tests/pwr/inspect_power_*` on `qemu-x86_64` and `hw-h004`.
- Review: OBS lead confirms the provider uses the typed inspect Interface rather than a private log.

#### Evidence
- none

### PWR-016 · Attribute energy use to ResourceDomain for Settings
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: PWR-009, PWR-010, SCH-045, OBS-019
- Baseline: §23, §62

APP power Settings needs per-application energy attribution that only ResourceDomain accounting can give (§23). PWR maps RAPL or powercap samples onto domains so Settings can list them. SCH owns budgets; this task is the meter.

#### Out of scope
Settings UX (APP-035). Domain energy policy (SCH-045). B-031 whole-machine publication (PWR-006).

#### Acceptance criteria
- [ ] On H-004 and H-005, `os inspect resource` shows an energy series per ResourceDomain sourced from RAPL or powercap, charged to that domain.
- [ ] A domain with no runnable Tasks records a lower energy series than a Throughput domain running a compile load on the same machine and interval, both in the inspect snapshot.
- [ ] Native software never reads RAPL sysfs as its energy Interface; missing rights return `Error::Rights`.
- [ ] APP-035 can bind the per-domain series without a second meter.

#### Verification
- Integration: `runtime:tests/pwr/domain_energy_*` on `hw-h004` and `hw-h005`.
- Review: SCH and APP leads confirm the series is the Settings attribution source.

#### Evidence
- none

### PWR-017 · Publish suspend and resume latency on both laptops
- Type: benchmark
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: PWR-024, BEN-005, BEN-007, LAB-014
- Baseline: §54, §62
- Benchmarks: B-030
- Invariants: I-061

V2-G16 publishes B-030 from the cycle harness, beside mainline Linux and Windows on the same laptop where dual boot exists. This run consumes traces from PWR-024. BEN owns the register.

#### Out of scope
Cycle functional gate (PWR-024). Register definition (BEN). Windows image pin (BEN-047).

#### Acceptance criteria
- [ ] A report exists under `reports/benchmarks/B-030/` for H-004 and H-005 meeting the register target kind for V2.
- [ ] Each report includes time to platform suspend and time to an interactive unlocked desktop with Wi-Fi, display and audio functional, beside Linux and beside Windows where dual boot exists.
- [ ] The reports cite B-030 and state no superiority claim (I-061).

#### Verification
- Bench: B-030 on H-004 and H-005; target per register.
- Review: BEN lead confirms the harness is `bench:suspend-resume` as named on B-030.

#### Evidence
- none

### PWR-018 · Implement display brightness policy including ambient-light option
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: PWR-011, HW-042, HW-050
- Baseline: §62

V2 laptop scope includes brightness. HW-042 decides whether ambient-light auto-brightness is in for the device class; this task implements the brightness Interface and, when that Decision includes ALS, the optional ALS path. APP owns the slider and OSD. Function keys are HW.

#### Out of scope
Function keys and keyboard backlight (HW-050). Slider and OSD (APP-042, APP-035). Sensor-class Decision (HW-042).

#### Acceptance criteria
- [ ] On H-004 and H-005, the brightness Interface sets internal-panel brightness, and `os inspect power` reports the current value.
- [ ] If HW-042 includes ALS, auto-brightness follows the ALS Device signal on those laptops; if it excludes ALS, inspect reports ALS unsupported and manual brightness still sets the panel.
- [ ] A Component without the brightness Capability receives `Error::Rights` and the panel level does not change.
- [ ] Native software never opens a Linux backlight sysfs node as its brightness Interface.

#### Verification
- Integration: `runtime:tests/pwr/brightness_*` on `hw-h004` and `hw-h005`.
- Review: HW lead confirms the ALS path matches HW-042.

#### Evidence
- none

### PWR-019 · Implement charge thresholds and power profiles
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: PWR-010, PWR-009, PWR-011, SCH-045
- Baseline: §22, §23, §62

V2 batteries and power-profile policy: charge start and stop thresholds, low-battery actions (notify, dim, suspend), and performance, balanced and power-saver profiles mapped onto ResourceDomain energy policy that PWR reads as frequency and idle ceilings. APP owns Settings UX.

<!-- covers: INV-1220 -->

#### Out of scope
Settings UX (APP-035). Domain energy policy schema (SCH-045). Notifications chrome (APP).

#### Acceptance criteria
- [ ] Charge start and stop thresholds set through the Battery Interface persist across reboot on H-004 and H-005 and are visible in `os inspect power`.
- [ ] Crossing the low-battery threshold emits a typed event, dims via DisplayPower, and at the suspend threshold submits a suspend Operation unless InhibitSuspend is held.
- [ ] Selecting performance, balanced or power-saver writes the corresponding ResourceDomain energy policy that PWR-009 already honors as a frequency and idle ceiling.
- [ ] A Component without the profile Capability receives `Error::Rights` and the active profile does not change.

#### Verification
- Integration: `runtime:tests/pwr/charge_threshold_*` and `runtime:tests/pwr/power_profile_*` on `hw-h004` and `hw-h005`.
- Review: SCH lead confirms profiles reuse SCH-045 rather than a second governor table.

#### Evidence
- none

### PWR-020 · Deliver the accepted hibernation policy on V2 laptops
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: PWR-007, PWR-014, SEC-048, LAB-014, HW-039, STO-014
- Baseline: §62
- Threats: T-010

V2 lists hibernate as optional. This task implements PWR-007: either a tested suspend-then-hibernate or full-hibernate path on H-004 and H-005, or inspectable unsupported status with no hibernation image in use. Unsigned leftover images are a T-010 asset; none are left on disk when hibernate is unsupported.

#### Out of scope
Key eviction and authenticated images (SEC-048). Swap layout (STO). Lab RTC and lid fixture (LAB-014).

#### Acceptance criteria
- [ ] If the accepted option is no hibernate, `os inspect power` reports hibernate unsupported on H-004 and H-005, and a disk scan after a suspend cycle finds no hibernation image.
- [ ] If the accepted option includes hibernate, both H-004 and H-005 complete a hibernate or suspend-then-hibernate cycle under LAB-014 and resume to an interactive desktop with Wi-Fi, display and audio hooks functional.
- [ ] A hibernate Operation without rights returns `Error::Rights` and writes no image.
- [ ] Native software never opens a Linux resume device or swsusp interface as its hibernate API.

#### Verification
- Integration: `runtime:tests/pwr/hibernate_policy_*` on `hw-h004` and `hw-h005`.
- Review: SEC lead confirms the delivered path matches SEC-048, including the unsupported case.

#### Evidence
- none

### PWR-021 · Implement lid, dock and external-display power behavior
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: PWR-011, PWR-014, HW-051, LAB-014, HW-039, LAB-018
- Baseline: §62

V2 laptop-day use includes open lid, close lid, resume, and an external display at a different scale. Policy: blank versus suspend on lid with and without external displays, plus dock undock. HW delivers lid Device signals. LAB automates the events. GFX owns arrangement and scale.

#### Out of scope
Lid Device signals (HW-051). Multi-monitor arrangement (GFX). Settings lid-close UI (APP-035). Lab fixture (LAB-014).

#### Acceptance criteria
- [ ] On H-004 and H-005, lid-close with no external display suspends using the accepted suspend state; lid-open resumes to an interactive desktop.
- [ ] Lid-close with an external display connected blanks the internal panel and does not suspend; the external display keeps scanning out.
- [ ] Undocking the only remaining display while the lid is closed submits suspend; docking with lid closed restores the external display without requiring a full resume if the machine stayed awake.
- [ ] Policy in force is visible in `os inspect power` and is the same source APP Settings binds.

#### Verification
- Integration: `runtime:tests/pwr/lid_dock_*` on `hw-h004` and `hw-h005` via LAB-014.
- Demo: V2 laptop session on H-004 or H-005 shows close lid, open lid, and an external display at a different scale.
- Review: GFX and HW leads confirm blank versus suspend is PWR policy over their signals.

#### Evidence
- none

### PWR-022 · Implement thermal policy with capacity-loss signal to ResourceDomain
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: PWR-004, PWR-009, PWR-015, HW-039
- Baseline: §22, §23, §62
- Invariants: I-032

V2 laptop thermal: trip points, cooling, and a capacity-loss signal that SCH-051 consumes so Background and Throughput shrink first. HW owns thermal-zone drivers and function keys. PWR does not rewrite the thermal subsystem.

<!-- covers: GAP-0313 -->

#### Out of scope
Thermal-zone drivers and hotkeys (HW). Scheduler quota response (SCH-051). Frequency hint map (SCH-038).

#### Acceptance criteria
- [ ] On H-004 and H-005, platform trip points and cooling state are visible in `os inspect power`.
- [ ] Crossing a capacity-loss trip emits a typed signal naming the affected ResourceDomain set; a test subscriber records Background and Throughput as the first class SCH is expected to shrink (I-032).
- [ ] Cooling engages through the retained thermal path; no native Component talks thermal-zone sysfs as its thermal Interface.
- [ ] A Component without thermal inspect rights receives `Error::Rights`.

#### Verification
- Integration: `runtime:tests/pwr/thermal_signal_*` on `hw-h004` and `hw-h005`.
- Review: SCH lead confirms the signal is the input SCH-051 consumes.

#### Evidence
- none

### PWR-023 · Prove battery estimate accuracy against measured runtime
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: PWR-010, PWR-019, PWR-006, LAB-013, BEN-039
- Baseline: §54, §62
- Benchmarks: B-031

V2-G05 requires the remaining-time estimate to sit within the tolerance named by this task of measured runtime on both laptops. Calibration uses B-031 discharge runs. APP displays the estimate; this task is the proof.

#### Out of scope
Settings indicator (APP-035). B-031 publication (BEN-039). Charge thresholds (PWR-019).

#### Acceptance criteria
- [ ] On H-004 and H-005, a B-031 mixed-workload discharge records remaining-time estimate at start and measured runtime at shutdown in the B-031 report.
- [ ] `runtime:tests/pwr/battery_estimate_*` fails the V2-G05 run when those two values disagree by more than the tolerance constant in that test.
- [ ] The same assertion runs on both laptops; a skip on an in-scope H-ID fails CI.

#### Verification
- Integration: `runtime:tests/pwr/battery_estimate_*` on `hw-h004` and `hw-h005`.
- Bench: B-031 discharge series on H-004 and H-005; estimate compared in the same report.
- Review: BEN lead confirms the measured runtime series is the B-031 runtime, not a second drain.

#### Evidence
- none

### PWR-024 · Run the V2 suspend cycle Gate on both laptops
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: PWR-014, PWR-021, LAB-014, HW-039, LAB-018
- Baseline: §62

V2-G05 requires automated suspend and resume cycles per laptop, each resuming to an interactive desktop with display scanning out. This extends the V1 harness onto H-005 and onto Intel and AMD lid and dock paths. LAB owns the physical rig. Cycle count and success rate are named here.

<!-- covers: INV-1221 -->

#### Out of scope
Lab fixture (LAB-014). B-030 publication (PWR-017). Hibernate path (PWR-020).

#### Acceptance criteria
- [ ] 500 of 500 automated suspend and resume cycles succeed on H-004, and 500 of 500 succeed on H-005, each resuming to an interactive desktop with display scanning out.
- [ ] The run includes lid-close and lid-open cycles and a dock path on each laptop, using LAB-014.
- [ ] Post-resume hooks for Wi-Fi, display and audio are functional after every cycle; a missing hook fails the cycle.
- [ ] `os inspect power` after the soak shows no stuck inhibitor and USB wake still off by default.

#### Verification
- Integration: `runtime:tests/pwr/suspend_cycles_v2_*` on `hw-h004` and `hw-h005`.
- Review: LAB lead confirms the fixture executed lid, dock and RTC-wake as named.

#### Evidence
- none

### PWR-025 · Publish idle and mixed-workload energy on V3 laptops
- Type: benchmark
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: PWR-006, PWR-026, LAB-013, LAB-021
- Baseline: §54, §63
- Benchmarks: B-031
- Invariants: I-061

V3-G15 and V3-G17 require every §54 metric to have a public number on at least one Tier 1 machine and B-031 to be re-measured on the laptops. This publishes idle and mixed-workload energy on H-004, H-005, H-007 and H-008. No superiority claim.

#### Out of scope
Suspend functional re-run (PWR-026). Register ownership (BEN).

#### Acceptance criteria
- [ ] A report exists under `reports/benchmarks/B-031/` for H-004, H-005, H-007 and H-008 meeting the register target kind for V3.
- [ ] Each report includes idle draw and mixed-workload battery runtime beside the pinned Linux image.
- [ ] The reports cite B-031 and state no superiority claim (I-061).

#### Verification
- Bench: B-031 on H-004, H-005, H-007 and H-008; target per register.
- Review: BEN lead confirms V3 target kind is publish per Tier 1 laptop.

#### Evidence
- none

### PWR-026 · Verify suspend, battery and thermal on V3 Tier 1 machines
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: PWR-024, PWR-010, PWR-022, PWR-021, HW-062, LAB-021
- Baseline: §62, §63
- Risks: R-027

V3 hardware scope adds H-006, H-007 and H-008. Energy republish is not evidence if suspend, battery and thermal were never proven on the new machines. This re-runs the suspend harness, Battery reporting, thermal policy and lid or dock paths on every V3 Tier 1 machine.

#### Out of scope
HCL publication (REL-048). Energy numbers (PWR-025). NVIDIA driver (HW).

#### Acceptance criteria
- [ ] H-002 and H-006 complete 100 of 100 automated suspend and resume cycles with display functional afterwards.
- [ ] H-004, H-005, H-007 and H-008 complete 500 of 500 automated suspend and resume cycles with display functional afterwards; laptops also complete a lid path.
- [ ] Battery reporting is present on every V3 Tier 1 laptop; thermal inspect data is present on every V3 Tier 1 machine.
- [ ] `os inspect power` names the verified suspend state per machine.

#### Verification
- Integration: `runtime:tests/pwr/tier1_v3_*` on `hw-h002`, `hw-h004`, `hw-h005`, `hw-h006`, `hw-h007` and `hw-h008`.
- Review: HW lead confirms the SKUs match the hardware register.

#### Evidence
- none

### PWR-027 · Publish idle and mixed-workload energy on every Tier 1 laptop
- Type: benchmark
- Milestone: V4
- Status: todo
- Size: S
- Owner: none
- Depends on: PWR-025, PWR-028, LAB-013, LAB-023
- Baseline: §54, §66
- Benchmarks: B-031
- Invariants: I-061

V4-G17 publishes idle and mixed-workload battery runtime for every Tier 1 laptop beside mainline Linux on the same machine, with regression versus V3 per the register. Laptop set is H-004, H-005, H-007, H-008, H-011, H-012 and H-013.

#### Out of scope
Suspend RC suite (PWR-028). Cross-OS Windows table beyond the register baselines (BEN).

#### Acceptance criteria
- [ ] A report exists under `reports/benchmarks/B-031/` for H-004, H-005, H-007, H-008, H-011, H-012 and H-013 meeting the register target kind for V4.
- [ ] Each report includes idle draw and mixed-workload battery runtime beside Linux on that machine.
- [ ] The reports cite B-031, include the V4 regression-versus-V3 clause, and state no superiority claim (I-061).

#### Verification
- Bench: B-031 on H-004, H-005, H-007, H-008, H-011, H-012 and H-013; target per register.
- Review: BEN lead confirms the V4 regression clause is applied per H-ID.

#### Evidence
- none

### PWR-028 · Verify the suspend cycle Gate on every V4 Tier 1 machine
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: PWR-026, HW-086, LAB-023, LAB-024
- Baseline: §62, §66

V4-G09 requires every named Tier 1 machine to pass the hardware test suite, including suspend and resume, each RC. PWR supplies the suspend cases at the cycle counts named for desktops in PWR-014 and for laptops in PWR-024. HW owns the combined suite.

#### Out of scope
Combined hardware suite orchestration (HW-086). Soak calendar (LAB-024). Energy publication (PWR-027).

#### Acceptance criteria
- [ ] Each RC, every machine in V4 Hardware scope completes the desktop cycle count (100 of 100) or laptop cycle count (500 of 500) with display functional afterwards.
- [ ] Hybrid-graphics laptops H-011 and H-012 complete the laptop cycle count with the internal panel scanning out after resume.
- [ ] `os inspect power` on each machine names the verified suspend state and that USB wake is off by default.
- [ ] A per-machine report lists pass or fail for suspend; a fail fails the RC.

#### Verification
- Integration: `runtime:tests/pwr/tier1_v4_*` on every V4 Hardware-scope H-ID.
- Review: HW lead confirms PWR cases are the suspend slice of HW-086.

#### Evidence
- none

### PWR-029 · Publish energy numbers versus Linux and Windows on 1.0
- Type: benchmark
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: PWR-027, BEN-060, BEN-047
- Baseline: §54
- Benchmarks: B-031
- Invariants: I-061

1.0-G13 and 1.0-G16 require every §54 metric published for every Tier 1 machine against Linux and Windows where dual boot exists. The energy gate is idle and mixed-workload battery runtime (B-031) with no superiority claim without the table.

#### Out of scope
Full §54 table (BEN-060). HCL suspend column (PWR-030).

#### Acceptance criteria
- [ ] A report exists under `reports/benchmarks/B-031/` for every Tier 1 laptop in 1.0 Hardware scope meeting the register target kind for 1.0.
- [ ] Each report includes Linux and, where dual boot exists, Windows on the same machine.
- [ ] The reports cite B-031 and state no superiority claim (I-061).

#### Verification
- Bench: B-031 on every 1.0 Tier 1 laptop; target per register.
- Review: BEN lead confirms the 1.0 table links these reports and contains no unmeasured claim.

#### Evidence
- none

### PWR-030 · Prove Tier 1 suspend on the 1.0 soak and HCL
- Type: build
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: PWR-028, LAB-025, REL-048, HW-089
- Baseline: §62, §54

1.0 hardware coverage: every listed Tier 1 feature works, including suspend and resume. This confirms the soak fleet has no open P0 or P1 suspend defects and that each HCL entry names the verified suspend path (state, wake sources, hibernate supported or not).

#### Out of scope
Soak calendar and fleet execution (LAB-025). HCL store (REL-048). Combined feature matrix (HW-089).

#### Acceptance criteria
- [ ] The 1.0 soak on every Tier 1 machine records zero open P0 or P1 defects in suspend, resume, Battery or display power.
- [ ] Each Tier 1 HCL row names the verified suspend state, whether hibernate is supported, and the wake sources tested.
- [ ] A machine whose HCL row claims suspend without a passing PWR-028 (or this soak) result fails the 1.0-G07 hardware coverage check.

#### Verification
- Integration: soak suspend slice `runtime:tests/pwr/soak_1_0_*` on every 1.0 Hardware-scope H-ID.
- Review: REL and HW leads confirm HCL suspend columns match these results.

#### Evidence
- none
