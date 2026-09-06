# SCH · Scheduling intent and resource domains
- Prefix: SCH
- Lead: none
- Baseline: §5.1, §9.1, §10, §11, §15, §18, §19, §22, §23, §53, §54, §59, §65, §66, §69

<!-- roadmap:generated:begin summary -->
Tasks: 61 live, 0 done, 0 in-progress, 61 todo, 0 dropped. Ready: 0. Blocked: 61. Weighted: 0%.
<!-- roadmap:generated:end -->

## Scope

SCH owns scheduling intent and ResourceDomain: the native unit of CPU policy, memory budget, GPU budget, I/O budget, network policy, storage quota, energy policy, latency policy and kernel-object limits. A Component belongs to a ResourceDomain; Tasks and Operations are accounted there. Intent classes are declared on the domain and overridable per Task, and they influence placement, frequency, I/O and GPU priority without exposing numeric nice, cgroupfs or Linux scheduler syscalls to native software. V0 ships Interactive and Background on a Capability-referenced domain with CPU share, memory budget and object limits. V0.5 adds Throughput, LowLatency, Deadline, latency policy, profiles, delegation and exhaustion handling. V1 adds Realtime, EnergyEfficient, GPU/I/O/network/storage budgets and frequency/core hints. V2 adds energy policy, IRQ affinity, thermal coordination and elevated-intent Capabilities. Later rungs validate, fuzz, freeze and sign off.

## Out of scope

Kernel fork, retained Linux scheduler internals and the eBPF/sched_ext role (KRN). Component creation and address-space membership (CMP). Task multiplexing and Operation rings (TSK). Channel transport and IDL (IPC). MemoryObject charging and NUMA attributes (MEM). `os inspect`/`os trace` rendering (OBS). Governors, suspend, thermal sensors and InhibitIdle/InhibitSuspend (PWR). Compositor frame scheduling and DRM (GFX). ComputeDevice dispatch (HET). Audio streams (AUD). Network Capabilities (NET). Store-side quota objects (STO). Linux personality syscalls (LNX). Wine/Proton threads (WIN). Grant taxonomy, Session and permissions UI (SEC). Capability mint/derive (CAP). SDK crates and `os` CLI (SDK). Benchmark methodology (BEN). CI and fuzz infrastructure (BLD). Shell resource UI (APP). Supervision (SVC). Package manifests (PKG). Layer 1 freeze process (ABI).

## Tasks

### SCH-001 · Benchmark ResourceDomain enforcement accuracy and per-domain overhead
- Type: benchmark
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: SCH-006, SCH-008, SCH-009, SCH-007, BEN-007, BEN-005
- Baseline: §10, §23, §53, §54, §59
- Benchmarks: B-011

Create and tear down a ResourceDomain with CPU share and memory budget, attach a Component, and publish share accuracy under contention, budget-hit behaviour, creation cost and per-domain overhead on H-001 and H-002. V0 is publish-only (B-011). The report is the overhead input for SCH-003 and the measured half of V0-G09.

<!-- covers: INV-1163 -->

#### Out of scope
Component creation latency (CMP, B-001). Isolation versus OCI (B-015 at V1). OBS inspect rendering.

#### Acceptance criteria
- [ ] Harness `bench:resourcedomain-lifecycle` exists and is invoked from the BEN runner on H-001 and H-002.
- [ ] Each run publishes p50 and p99 for domain create, attach and teardown, plus CPU-share error and memory-budget hit/fail counts, into a B-011 report path.
- [ ] The same session records a Linux cgroup v2 create/attach/remove baseline on the same machine.
- [ ] Nightly CI on H-001 publishes B-011 results to the benchmark time-series export.

#### Verification
- Bench: B-011 on H-001 and H-002; target per register (V0 publish).
- Integration: `kernel:tests/sch/domain_enforcement_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.
- Review: BEN methodology sign-off recorded on the pull request.

#### Evidence
- none

### SCH-002 · Decide hierarchical versus flat ResourceDomains and budget delegation via Capability
- Type: adr
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: CAP-010
- Baseline: §8, §9.1, §23
- Decision: D-0250

Decide whether ResourceDomains nest with parent enforcement or form a flat set, and how a holder of `Capability<ResourceDomain>` delegates and attenuates budgets. The choice shapes the V0 kernel object and the later launcher and `os env` paths. Native software never configures cgroups to express nesting.

<!-- covers: INV-0435 -->

#### Out of scope
Implementation of derive (SCH-019). Capability rights encoding (CAP).

#### Acceptance criteria
- [ ] The decision file lists at least hierarchical nested budgets with parent enforcement, and a flat set whose only structure is Capability attenuation.
- [ ] Each option states how a child budget that exceeds its parent is rejected, and whether parent counters include children.
- [ ] The accepted option names the V0 kernel operations for create, attach and (if any) derive, without freezing S-009.
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: ABI lead and capability reviewers record sign-off on the pull request.

#### Evidence
- none

### SCH-003 · Decide ResourceDomain over cgroup v2 controllers versus native accounting
- Type: adr
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: SCH-001, SCH-012
- Baseline: §6, §23, §53, §57
- Decision: D-0251
- Risks: R-007
- Invariants: I-033, I-057

Decide whether ResourceDomain is implemented over cgroup v2 controllers as a Phase C internal detail or as native kernel accounting, using the B-011 overhead report. Accounting remains a native kernel concept applied to every Component and Task (I-033). cgroup configuration is never a semantic step for native software (I-057).

<!-- covers: GAP-0531, INV-0424, INV-0994, INV-0436 -->

#### Out of scope
Making the native ABI hide cgroupfs (SCH-023). Personality cgroups (LNX).

#### Acceptance criteria
- [ ] The decision file lists at least cgroup v2 as an internal Phase C implementation, and native accounting with no cgroup controllers on the native path.
- [ ] Each option cites the B-011 report for per-domain create/teardown cost and a migration path that keeps S-009 stable.
- [ ] The accepted option records that native software never opens cgroupfs and that every Component and Task is in a domain at creation.
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: kernel and ABI leads record sign-off on the pull request citing the B-011 report.

#### Evidence
- none

### SCH-004 · Decide how intents map onto Linux scheduler mechanisms versus a native class
- Type: adr
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: SCH-012
- Baseline: §5.1, §6, §22, §65
- Decision: D-0253
- Risks: R-007
- Invariants: I-032

Decide how the native intent interface maps onto the retained Linux scheduler: EEVDF nice and latency-nice, SCHED_DEADLINE, SCHED_FIFO/RR, uclamp and cgroup cpu controllers, a sched_ext BPF scheduler, or a new native class. The native interface expresses intent, not a numeric priority (I-032). Phase A retains the Linux scheduler with intent layered on top. S-009 stays prototyped; nothing in Layer 1 freezes in V0.

<!-- covers: INV-0419, INV-0398, INV-0124 -->

#### Out of scope
Implementing Interactive and Background (SCH-010). eBPF's native role (KRN-024).

#### Acceptance criteria
- [ ] The decision file lists at least mapping onto retained Linux scheduler knobs, a sched_ext BPF scheduler, and a new native scheduling class.
- [ ] Each option is scored against the SCH-012 wakeup measurements for audio-like and compositor-like workloads on H-001 and H-002.
- [ ] The accepted option states that native crates expose intent classes, not nice, and that S-009 remains prototyped.
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: scheduler and ABI leads record sign-off on the pull request citing `reports/spikes/SCH-012.md`.

#### Evidence
- none

### SCH-005 · Make Operation completion wake the awaiting Task and schedule TaskGroups directly
- Type: build
- Milestone: V0
- Status: todo
- Size: L
- Owner: none
- Depends on: TSK-020, TSK-018, CMP-005, SCH-011
- Baseline: §18, §20, §53, §59
- Risks: R-004

Completion of an Operation makes the awaiting Task runnable with no polling and no extra wakeup syscall (§18). A new Component's TaskGroup is enqueued directly, with no process bootstrap (§53 isolation step 5). Both sit on the V0 demo path and feed B-001, B-003 and B-009. The Linux syscall path stays intact (R-004).

<!-- covers: INV-0358, INV-1000 -->

#### Out of scope
Operation ring layout (TSK). Component object creation (CMP). Channel wakeup (IPC).

#### Acceptance criteria
- [ ] Completing a Wait Operation on H-001 makes the awaiting Task runnable without a second submit or poll syscall, shown by `os trace` on the V0 demo.
- [ ] Creating a Component enqueues its TaskGroup on a runqueue without `fork` or `exec` on the native path.
- [ ] L0 corpus pass rate on the fork equals the unforked kernel of the same version on H-001 and H-002.
- [ ] No `unsafe` outside the scheduler wake and enqueue files named in the pull request.

#### Verification
- Unit: `kernel:tests/sch/wake_on_complete_*` and `kernel:tests/sch/direct_enqueue_*` on `qemu-x86_64` and `hw-h002`.
- Integration: V0-D01 pipeline on H-002 with `os trace` showing completion-to-run.
- Bench: B-003 and B-009 on H-001 and H-002; target per register (V0 publish).
- Compat: C-001 on H-001 and H-002.

#### Evidence
- none

### SCH-006 · Enforce ResourceDomain CPU policy: share, quota, allowed cores
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: SCH-007
- Baseline: §11, §22, §23, §59

Implement share, quota and allowed-core set on the Phase C wrapper. The intent field is filled by SCH-010. Internals may later change under SCH-003 without changing `Capability<ResourceDomain>`. This is the CPU half of V0-G09.

<!-- covers: INV-0238, INV-0426 -->

#### Out of scope
Intent classes (SCH-010). Frequency hints (SCH-038). Hybrid core selection (SCH-036).

#### Acceptance criteria
- [ ] A ResourceDomain with a 25 percent CPU share, under a busy-loop contention load for 10 seconds on H-001 and H-002, has measured share within 5 percent of the requested share.
- [ ] A quota of zero runnable time leaves member Tasks unscheduled until the quota is raised.
- [ ] A Task whose domain's allowed-core set is `{0}` never runs on another CPU, verified by `os inspect` and `/proc` traces on the personality side of the same kernel.
- [ ] Setting share, quota or allowed cores through the native ABI does not require opening cgroupfs from the test Component.

#### Verification
- Unit: `kernel:tests/sch/cpu_share_*`, `cpu_quota_*`, `cpuset_*` on `qemu-x86_64` and `hw-h002`.
- Integration: V0-G09 contention scenario on H-001 and H-002.
- Bench: B-011 share-error column on H-001 and H-002.

#### Evidence
- none

### SCH-007 · Implement ResourceDomain as a Capability-referenced kernel Object
- Type: build
- Milestone: V0
- Status: todo
- Size: L
- Owner: none
- Depends on: SCH-002, CAP-005, ABI-005, KRN-013, CMP-007
- Baseline: §7, §10, §19, §23, §53, §59, §69
- Invariants: I-033

Create ResourceDomain in one kernel operation (§53 isolation step 1), referenced only by `Capability<ResourceDomain>`. Every Component is a member. Every Operation is attributed to its domain for CPU, I/O and memory accounting. Per-domain counters and scheduling-delay tracepoints feed V0 `os inspect resource` and `os trace`. Native callers never hold a cgroup path.

<!-- covers: INV-0053, INV-0425, INV-0996, INV-0227, INV-0368, INV-1319 -->

#### Out of scope
CPU/memory/object enforcement (sibling SCH tasks). OBS CLI rendering. CMP spawn.

#### Acceptance criteria
- [ ] `create_resource_domain` returns `Capability<ResourceDomain>` and is a single kernel operation, with no cgroupfs open in the calling Component.
- [ ] A Component created into that domain appears as a member in `os inspect resource` on H-001 and H-002.
- [ ] An Operation submitted by a member Task increments that domain's CPU and I/O counters, visible through the inspect provider.
- [ ] Destroying the last member and the domain reclaims the kernel object, verified by a create/destroy loop with no unbounded growth in domain handles.
- [ ] Wrong-type use of the handle returns `Error::Rights` and allocates no handle.

#### Verification
- Unit: `kernel:tests/sch/domain_create_*`, `domain_attach_*`, `domain_account_*` on `qemu-x86_64` and `hw-h002`.
- Integration: V0-G10 `os inspect resource` on H-001 and H-002.
- Fuzz: `kernel:fuzz/sch_domain_create` one hour nightly without panic.

#### Evidence
- none

### SCH-008 · Enforce the ResourceDomain memory budget for member Components
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: SCH-007
- Baseline: §11, §16, §23, §59

Enforce the domain memory budget. Charging covers anonymous memory and MemoryObjects owned by the domain. Over-budget allocation returns a typed exhaustion error until SCH-016 refines reclaim versus kill. MEM owns the page-charging hook; SCH owns the cap. This is the memory half of V0-G09.

<!-- covers: INV-0237, INV-0427 -->

#### Out of scope
MemoryObject charging implementation (MEM-004). Exhaustion policy (SCH-016). GPU memory (SCH-031).

#### Acceptance criteria
- [ ] A Component in a ResourceDomain with a 64 MiB budget that maps anonymous memory plus MemoryObjects past that cap receives a typed exhaustion error on H-001 and H-002.
- [ ] After the failed map, `os inspect resource` shows charged bytes at or below 64 MiB.
- [ ] Transferring a MemoryObject out of the domain drops the charge on the sender and raises it on the receiver, with no double count.
- [ ] The test Component never opens a memory cgroup file.

#### Verification
- Unit: `kernel:tests/sch/memory_budget_*` on `qemu-x86_64` and `hw-h002`.
- Integration: V0-G09 memory scenario on H-001 and H-002.
- Bench: B-011 budget-hit column on H-001 and H-002.

#### Evidence
- none

### SCH-009 · Bound kernel-Object consumption per ResourceDomain with typed exhaustion errors
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: SCH-007
- Baseline: §23, §51, §59
- Risks: R-074
- Threats: T-016

Bound handles, Tasks, Channels, MemoryObjects and outstanding Operations per ResourceDomain so a runaway Component cannot exhaust kernel memory (R-074, T-016). Limits are a pids-controller equivalent behind the native object. Personality forks count against the enclosing domain. Needed for the V0 100,000-Component leak test and the V0 fault demo to stay meaningful.

<!-- covers: EXTRA-002 -->

#### Out of scope
Budget exhaustion policy beyond fail-closed (SCH-016). Capability table internals (CAP).

#### Acceptance criteria
- [ ] Creating one more handle, Task, Channel, MemoryObject or outstanding Operation than the domain's limit returns a typed exhaustion error and allocates no object.
- [ ] `os inspect resource` reports live counts and limits for each of those five kinds.
- [ ] A Linux-personality `fork` bomb inside a domain hits the same Task/handle limit as a native spawn loop.
- [ ] The V0 Component create/destroy leak test (CMP) still reclaims kernel memory with these limits enabled.

#### Verification
- Unit: `kernel:tests/sch/object_limits_*` on `qemu-x86_64` and `hw-h002`.
- Integration: V0-G09 object-limit scenario and V0-G02 leak test on H-001 and H-002.
- Fuzz: `kernel:fuzz/sch_object_limits` one hour nightly without panic.

#### Evidence
- none

### SCH-010 · Implement intent declaration on ResourceDomain and Task with Interactive and Background
- Type: build
- Milestone: V0
- Status: todo
- Size: L
- Owner: none
- Depends on: SCH-004, SCH-006
- Baseline: §4, §22, §69
- Risks: R-004
- Invariants: I-032

Add a typed intent field to domain CPU policy and a per-Task override. V0 carries Interactive and Background only. Wire both through the mapping chosen by SCH-004 onto the retained Linux scheduler so intent influences scheduling class and placement (§22). Kernel core owns scheduling as a native responsibility. Native crates expose the class names, not nice.

<!-- covers: INV-0399, INV-0400, INV-0401, INV-0413, INV-0114, INV-1322 -->

#### Out of scope
Throughput, LowLatency (SCH-026). Deadline (SCH-025). Realtime (SCH-042).

#### Acceptance criteria
- [ ] A ResourceDomain and a member Task can each declare Interactive or Background; the Task override wins when set.
- [ ] Under mixed load on H-001 and H-002, an Interactive Task is selected to run while a Background busy-loop is runnable, shown by `os trace` wakeup-to-run.
- [ ] The native SDK and ABI headers contain no nice, `SCHED_FIFO` or cpuset entry points.
- [ ] C-001 pass rate is unchanged on H-001 and H-002 with intent enabled.

#### Verification
- Unit: `kernel:tests/sch/intent_interactive_*`, `intent_background_*` on `qemu-x86_64` and `hw-h002`.
- Integration: V0-G10 `os trace` scheduling-delay column on H-002.
- Bench: B-010 on H-001 and H-002; target per register (V0 publish).
- Compat: C-001 on H-001 and H-002.

#### Evidence
- none

### SCH-011 · Prototype scheduler-aware Channel handoff and measure latency and fairness
- Type: spike
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: IPC-010, TSK-018, IPC-017
- Baseline: §15, §53, §54
- Explores: S-012

Prototype a direct switch to the receiver on Channel send (the §53 scheduler-aware handoff) on the small-message fast path. Publish round-trip latency and fairness against plain wakeup so the V0 IPC table (B-003, B-004, publish-only) has a native-handoff column. Freeze nothing.

<!-- covers: INV-0296 -->

#### Out of scope
Selecting the fast path (IPC-003). Intent inheritance (SCH-017).

#### Acceptance criteria
- [ ] A prototype on H-001 and H-002 runs Channel ping-pong with direct switch and with plain wakeup, same core and cross core.
- [ ] The report publishes p50 and p99 for both modes beside Linux futex and pipe ping-pong, citing B-003 and B-004, with no superiority claim.
- [ ] The report records fairness: a Background receiver still runs when an Interactive sender floods the Channel.
- [ ] `reports/spikes/SCH-011.md` exists with the spike skeleton headings.

#### Verification
- Report: Does direct switch beat plain wakeup on B-003/B-004 same-core and cross-core on H-001 and H-002? What is the fairness cost under a Background flood? Which wake path does SCH-005 take?
- Bench: B-003 and B-004 on H-001 and H-002; target per register (V0 publish).

#### Evidence
- none

### SCH-012 · Prototype scheduling-intent routes and measure audio and compositor latency
- Type: spike
- Milestone: V0
- Status: todo
- Size: L
- Owner: none
- Depends on: KRN-013, BEN-007
- Baseline: §5.1, §22, §54
- Explores: S-009

Measure whether EEVDF plus uclamp plus cgroup cpu controllers can express the seven intent classes, then prototype intent as (a) an extension of the existing class, (b) a sched_ext BPF scheduler, and (c) a new scheduler class. Publish wakeup latency for audio-like and compositor-like workloads on H-001 and H-002. This spike feeds SCH-004. Freeze nothing.

<!-- covers: GAP-0530, INV-0420 -->

#### Out of scope
The mapping decision (SCH-004). Audio server (AUD). Compositor (GFX).

#### Acceptance criteria
- [ ] The report contains wakeup p50/p99/p99.9 for Interactive, Background, Throughput, LowLatency, Realtime, EnergyEfficient and Deadline-shaped loads on each of the three routes, on H-001 and H-002.
- [ ] Audio-like (periodic wakeup) and compositor-like (vblank-period wakeup) traces are included for each route.
- [ ] The report states whether EEVDF plus uclamp plus cgroup controllers can distinguish the seven classes without a new class, with evidence from those traces.
- [ ] `reports/spikes/SCH-012.md` exists with the spike skeleton headings.

#### Verification
- Report: Can EEVDF+uclamp+cgroup express all seven classes? What wakeup latency does each of (a) class extension, (b) sched_ext, (c) new class deliver for audio-like and compositor-like loads on H-001 and H-002? Which route does SCH-004 accept?
- Bench: B-010 on H-001 and H-002; target per register (V0 publish).

#### Evidence
- none

### SCH-013 · Benchmark wakeup latency per intent class under contention
- Type: benchmark
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: SCH-010, SCH-026, SCH-025, SCH-001, BEN-005
- Baseline: §22, §54
- Benchmarks: B-010

Stand up `bench:sched-wakeup`: wakeup-to-run p50/p99/p99.9 per intent class against a Background and Throughput flood, beside Linux nice and SCHED_FIFO on the same machine. V0.5 frame-time gates depend on Interactive and Deadline behaviour. Later rungs re-run this harness under B-051. Required by V0.5-G17 (Prior benchmarks show no unexplained regression): B-010 is a tracked benchmark with a V0.5 regression target, and this harness produces its V0.5 report.

#### Out of scope
Audio callback jitter (SCH-028). Compositor frame latency (GFX, B-018).

#### Acceptance criteria
- [ ] Harness `bench:sched-wakeup` publishes p50, p99 and p99.9 for Interactive, Background, Throughput, LowLatency and Deadline on H-001, H-002 and H-003.
- [ ] The same session records Linux nice and SCHED_FIFO under the same flood.
- [ ] A B-010 report lands for each of those H-IDs with the V0.5 target kind (regression versus V0).
- [ ] The harness is wired into the BEN runner so later B-051 jobs can re-run it.

#### Verification
- Bench: B-010 on H-001, H-002 and H-003; target per register.
- Integration: contention flood scenario in `kernel:tests/sch/wakeup_contention_*` on `qemu-x86_64` and `hw-h002`.

#### Evidence
- none

### SCH-014 · Implement budget exhaustion handling and owner notification per ResourceDomain
- Type: build
- Milestone: V0.5
- Status: todo
- Size: L
- Owner: none
- Depends on: SCH-016, SVC-015, SCH-008, SCH-009, SCH-006
- Baseline: §23, §32
- Threats: T-016

Implement the chosen exhaustion policy for memory, CPU and object limits, with typed errors, throttling and an owner-visible event Channel so SVC can restart or shrink a misbehaving service. Fault-injection tests cover each budget kind (T-016).

<!-- covers: INV-0442 -->

#### Out of scope
The policy decision (SCH-016). Safe-mode session (SVC).

#### Acceptance criteria
- [ ] Exhausting memory, CPU quota or object limits produces the result named by SCH-016 for that kind, with no untyped kill.
- [ ] The domain owner receives an event on the notification Channel naming the budget kind and the member Component.
- [ ] Fault injection of each kind on H-001 and H-003 leaves other domains runnable.
- [ ] SVC supervision tests can subscribe to the Channel and restart the member without a kernel panic.

#### Verification
- Unit: `kernel:tests/sch/exhaustion_memory_*`, `exhaustion_cpu_*`, `exhaustion_objects_*` on `qemu-x86_64`.
- Integration: fault-injection with SVC restart on H-003.
- Fuzz: `kernel:fuzz/sch_exhaustion` one hour nightly without panic.

#### Evidence
- none

### SCH-015 · Run the compositor under Deadline intent aligned to display vblank
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: SCH-025, GFX-015, GFX-029
- Baseline: §22, §40, §60

The compositor frame Task declares a per-vblank Deadline and Interactive default so composition is deadline-sensitive (§22). GFX owns frame scheduling; SCH supplies the class and admission. Verified against the V0.5 idle capture and B-018.

<!-- covers: INV-0408 -->

#### Out of scope
Frame object and explicit sync (GFX). Input-to-photon rig (LAB, BEN).

#### Acceptance criteria
- [ ] The compositor's frame Task is visible in `os inspect` as Deadline with period equal to the display refresh on H-002.
- [ ] The idle-desktop capture named by GFX-029 records zero dropped frames on H-002 with this intent set.
- [ ] A Background flood in another domain does not change the compositor Task's declared deadline.
- [ ] Admission rejects a Deadline that exceeds the compositor domain's CPU quota.

#### Verification
- Integration: idle capture on H-002 and H-003.
- Bench: B-018 on H-002; target per register (V0.5 publish).
- Demo: V0.5 compositor presents at fixed refresh on H-002.

#### Evidence
- none

### SCH-016 · Decide behaviour on ResourceDomain budget exhaustion and owner reporting
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: SCH-008, SCH-009, SCH-006, Q-015
- Baseline: §23, §32
- Decision: D-0249
- Threats: T-016
- Invariants: I-033

Decide what happens when a ResourceDomain exhausts a budget: memory reclaim, throttling, typed Operation failure, or Component termination, per budget kind, and how the owner is notified. Packages and system services run under real budgets in V0.5; the choice must be typed and observable (T-016).

<!-- covers: INV-0442 -->

#### Out of scope
Implementation (SCH-014). Safe-mode UX (SVC, APP).

#### Acceptance criteria
- [ ] The decision file lists at least fail-closed typed Operation errors with an owner event, Component termination after reclaim, and a per-kind mix of those.
- [ ] Each option states the result for memory, CPU and object limits, and the Channel or inspect signal the owner sees.
- [ ] The accepted option names the error type and forbids untyped SIGKILL-style death on the native path.
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: kernel, SVC and ABI leads record sign-off on the pull request.

#### Evidence
- none

### SCH-017 · Decide intent and priority inheritance across Channel handoff
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: SCH-010, IPC-010, SCH-011, Q-014
- Baseline: §15, §22, §32
- Decision: D-0252

Decide how a LowLatency or Interactive client calling a shared service avoids queueing behind Background work in that service: per-request intent on the message, receiver-side boosting, or dedicated service Tasks per class. The compositor and file-chooser paths at V0.5 need the rule before rebind ships.

<!-- covers: INV-0422 -->

#### Out of scope
Implementation (SCH-024). Operation priority model (TSK-027).

#### Acceptance criteria
- [ ] The decision file lists at least per-request intent propagation, receiver-side boosting, and dedicated per-class service Tasks.
- [ ] Each option states what a Background flood in the service does to an Interactive client's round trip, citing SCH-011.
- [ ] The accepted option names the Channel or Task fields that carry intent and whether attenuation is allowed.
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: IPC and scheduler leads record sign-off on the pull request.

#### Evidence
- none

### SCH-018 · Export per-ResourceDomain usage and scheduling-delay data for inspection
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: SCH-007, OBS-007, SCH-010
- Baseline: §23, §24, §64

Export typed per-domain CPU, memory, object-count and wakeup-delay data over a Channel for OBS `os inspect`/`os trace` and the later shell resource view. V0 shipped counters and tracepoints; V0.5 is the structured export.

<!-- covers: INV-0444, INV-0465 -->

#### Out of scope
OBS CLI and trace UI (OBS). Shell resource panel (APP).

#### Acceptance criteria
- [ ] A holder of `Capability<ResourceDomain>` can read current CPU, memory, object counts and wakeup-delay histograms over the export Channel.
- [ ] `os inspect resource` on H-003 prints those fields for every live domain.
- [ ] A Component without the Capability receives `Error::Rights` and no histogram bytes.
- [ ] Export is disabled-tracepoint cheap when no subscriber is attached, measured on the B-004 path as part of B-012.

#### Verification
- Unit: `kernel:tests/sch/domain_export_*` on `qemu-x86_64`.
- Integration: `os inspect resource` on H-003 with two domains.
- Bench: B-012 on H-001; target per register.

#### Evidence
- none

### SCH-019 · Implement ResourceDomain delegation and attenuation through Capability
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: SCH-002, SCH-007, CAP-003
- Baseline: §8, §23, §35

Implement SCH-002: a holder of `Capability<ResourceDomain>` derives a child whose budgets are attenuated and enforced against the parent. The V0.5 launcher places each app in its own domain; `os env` reuses the same derive.

<!-- covers: INV-0435 -->

#### Out of scope
Hierarchy decision (SCH-002). Environment object (ENV).

#### Acceptance criteria
- [ ] `derive(domain, mask)` with a budget that is not a subset returns `Error::Rights` and allocates no domain.
- [ ] A derived domain's CPU share plus siblings cannot exceed the parent, enforced on H-001 and H-003.
- [ ] Destroying or revoking the parent makes child domain operations fail within one Operation.
- [ ] The launcher test creates four app domains from one session root without opening cgroupfs.

#### Verification
- Unit: `kernel:tests/sch/domain_derive_*` on `qemu-x86_64` and `hw-h002`.
- Integration: four-app launch on H-003.
- Fuzz: `kernel:fuzz/sch_domain_derive` one hour nightly without panic.

#### Evidence
- none

### SCH-020 · Add ResourceDomain latency policy setting default intent and deadlines
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: SCH-010, SCH-025, SCH-007
- Baseline: §22, §23

Latency policy on the domain supplies default intent and deadline parameters to member Tasks so applications rarely set per-Task intent. Packaged domain profiles consume this field.

<!-- covers: INV-0433 -->

#### Out of scope
Profile schema (SCH-021). Per-Task override already in SCH-010.

#### Acceptance criteria
- [ ] A domain latency policy of Deadline with a named period is inherited by member Tasks that do not override intent.
- [ ] A per-Task Interactive override still wins over a Background domain default.
- [ ] `os inspect resource` prints the policy and the count of Tasks using the default versus an override.
- [ ] Changing the policy does not affect Tasks that already overrode intent.

#### Verification
- Unit: `kernel:tests/sch/latency_policy_*` on `qemu-x86_64`.
- Integration: compositor domain default plus editor override on H-003.

#### Evidence
- none

### SCH-021 · Define typed ResourceDomain policy profiles for packages and services
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: SCH-020, SCH-006, SCH-008, PKG-011, SVC-011
- Baseline: §23, §28, §32, §66

Define a versioned Layer 2 schema so a Package or service declares domain budget, intent and latency policy. Supervisor restarts (compositor crash-rebind) recreate identical domains from the profile. Not frozen; evolution follows S-014. Required by V0.5-G03 (Compositor crash recovery rebinds every window).

<!-- covers: INV-0440 -->

#### Out of scope
Package manifest identity (PKG). Service restart (SVC). Layer 1 freeze (ABI).

#### Acceptance criteria
- [ ] A profile document names CPU share, memory budget, object limits, default intent and optional deadline, and fails schema validation if a required field is missing.
- [ ] Restarting a service from the same profile yields a domain whose inspectable budgets match the previous instance.
- [ ] Two schema versions coexist: an older client is accepted by a newer supervisor for additive optional fields.
- [ ] Native software still has no cgroup or nice fields in the schema.

#### Verification
- Unit: `userspace:tests/sch/profile_schema_*`.
- Integration: compositor crash-rebind on H-003 recreates the same domain profile.
- Review: IPC Layer 2 evolution review recorded on the pull request.

#### Evidence
- none

### SCH-022 · Replace nice, chrt and cpuset tuning with ResourceDomain policy in the SDK
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: SCH-010, SCH-006, SDK-018, SCH-027
- Baseline: §22, §23, §52, §57

The native SDK exposes only intent and domain policy. No nice, chrt or cpuset entry points in native crates. The four V0.5 apps and the compositor declare policy through their domain.

<!-- covers: INV-0440 -->

#### Out of scope
Personality `nice`/`sched_setattr` (SCH-043). Lint infrastructure (BLD).

#### Acceptance criteria
- [ ] Native SDK crates contain no public symbol named nice, chrt, cpuset or `sched_setscheduler`.
- [ ] Terminal, File Browser, Text Editor, Image Viewer and the compositor start with a domain profile and no scheduler syscall in their native code.
- [ ] CI fails a native crate that calls Linux scheduler syscalls.
- [ ] `os inspect` on each of the four apps shows a ResourceDomain, not a nice value.

#### Verification
- Unit: SDK crate API test listing scheduling symbols.
- Integration: four-app inspect dump on H-003.
- Review: SDK lead sign-off recorded on the pull request.

#### Evidence
- none

### SCH-023 · Make ResourceDomain subsume cgroup isolation and accounting for native software
- Type: build
- Milestone: V0.5
- Status: todo
- Size: L
- Owner: none
- Depends on: SCH-003, SCH-007, SCH-006, SCH-008
- Baseline: §6, §23, §53, §57
- Invariants: I-033, I-057

Native Components, packages and services never touch cgroupfs. Domain creation drives controllers internally, or native accounting, per SCH-003. Required before native init and SVC supervision. A filesystem-diff test proves no native path writes cgroupfs.

<!-- covers: INV-0437 -->

#### Out of scope
Personality cgroups (LNX). The substrate decision (SCH-003).

#### Acceptance criteria
- [ ] A filesystem diff of a native Component lifetime shows no open, read or write of cgroupfs paths.
- [ ] Native init and one supervised service start with ResourceDomain membership and no cgroup manager in their capability set.
- [ ] `os inspect resource` is the only supported way to read native CPU and memory usage.
- [ ] Linux-personality processes in LNX may still use cgroups inside their Component; native crates cannot.

#### Verification
- Integration: filesystem-diff harness on H-003 covering init, compositor and one service.
- Unit: lint from SCH-027 fails a native crate that references cgroupfs.
- Review: kernel lead sign-off recorded on the pull request.

#### Evidence
- none

### SCH-024 · Implement intent inheritance across Channel handoff for shared services
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: SCH-017, IPC-010, SCH-010
- Baseline: §15, §22, §32

Implement SCH-017 on the Channel fast path and in the service runtime so compositor and file-chooser requests from Interactive clients are served at the caller's intent. A Background flood in the service must not invert that order.

<!-- covers: INV-0422 -->

#### Out of scope
The inheritance decision (SCH-017). Client rebind (IPC, SDK).

#### Acceptance criteria
- [ ] An Interactive client calling a shared service that also has Background callers completes while Background work is runnable, on H-003.
- [ ] `os trace` on that call shows the service Task running at Interactive (or the accepted equivalent) for the duration of the request.
- [ ] A client without LowLatency or Interactive intent cannot raise the service above the service domain's default.
- [ ] Regression test with a Background flood stays in CI.

#### Verification
- Unit: `kernel:tests/sch/handoff_intent_*` on `qemu-x86_64`.
- Integration: compositor and file-chooser under Background flood on H-003.
- Bench: B-010 Interactive-under-flood column on H-002.

#### Evidence
- none

### SCH-025 · Add Deadline intent class with declared period and completion deadline
- Type: build
- Milestone: V0.5
- Status: todo
- Size: L
- Owner: none
- Depends on: SCH-004, SCH-010, SCH-006
- Baseline: §22, §40

Deadline work declares period and deadline (SCHED_DEADLINE lineage or native class per SCH-004) with admission control bounded by the domain CPU quota. Required by GFX frame scheduling and SCH-015.

<!-- covers: INV-0406 -->

#### Out of scope
Compositor callbacks (GFX). Realtime class (SCH-042).

#### Acceptance criteria
- [ ] A Task can declare Deadline with period and deadline; admission that would exceed the domain CPU quota returns a typed error and does not start the Task.
- [ ] A Deadline Task that misses its deadline is visible in `os inspect` with a miss count, and continues under the mapping's overrun rule.
- [ ] Background work does not run on the same CPU while a Deadline Task is within its deadline window, on H-002.
- [ ] Native crates expose period and deadline fields, not `sched_setattr` attributes.

#### Verification
- Unit: `kernel:tests/sch/intent_deadline_*` on `qemu-x86_64` and `hw-h002`.
- Integration: compositor vblank Task on H-002.
- Bench: B-010 Deadline column on H-001 and H-002.

#### Evidence
- none

### SCH-026 · Add Throughput and LowLatency intent classes
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: SCH-004, SCH-010
- Baseline: §22

Add Throughput (batch work that maximises total progress) and LowLatency (minimal wakeup delay), mapped per SCH-004. Measured by B-010. These classes left V0 per the critique.

<!-- covers: INV-0402, INV-0403 -->

#### Out of scope
Realtime (SCH-042). EnergyEfficient (SCH-037). Elevated-intent Capability (SCH-048).

#### Acceptance criteria
- [ ] A Task can declare Throughput or LowLatency through the native ABI.
- [ ] Under a mixed load on H-001 and H-002, LowLatency wakeup-to-run p99 is strictly lower than Throughput in the B-010 report.
- [ ] Throughput work makes progress when no Interactive, Deadline or LowLatency Task is runnable.
- [ ] Native crates still expose no numeric priority API.

#### Verification
- Unit: `kernel:tests/sch/intent_throughput_*`, `intent_lowlatency_*` on `qemu-x86_64` and `hw-h002`.
- Bench: B-010 on H-001 and H-002; target per register.

#### Evidence
- none

### SCH-027 · Add CI lints enforcing native intent, universal accounting and no cgroup steps
- Type: build
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: SCH-004, SCH-003, SCH-007, BLD-011
- Baseline: §3, §22, §23, §53, §57
- Invariants: I-032, I-033, I-049, I-057

CI review-gate tests: the native ABI and SDK expose no numeric priority; every Component and Task has a domain at creation; no native code path references cgroupfs or Linux scheduler syscalls. First matters when native init, packages and services appear.

<!-- covers: INV-0398, INV-0424, INV-0994 -->

#### Out of scope
Personality syscalls (LNX). Broader ABI firewall (ABI-003).

#### Acceptance criteria
- [ ] CI fails a native crate that references cgroupfs, `nice`, `chrt`, `sched_setscheduler` or `sched_setattr`.
- [ ] CI fails a Component or Task creation path that does not attach a ResourceDomain.
- [ ] CI fails a native ABI header that documents a numeric priority as the scheduling interface.
- [ ] The lint is in the pre-merge BLD tier.

#### Verification
- Unit: fixture crates that must fail the lint, in `tools/lints/sch/*`.
- Review: BLD and ABI leads record sign-off on the pull request.

#### Evidence
- none

### SCH-028 · Benchmark audio-callback jitter under LowLatency and Realtime intent
- Type: benchmark
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: SCH-042, SCH-026, AUD-007, BEN-005
- Baseline: §22, §54
- Benchmarks: B-010

Harness a periodic audio-like callback under mixed load and publish jitter and missed-period counts per intent class beside PipeWire on Linux. Complements B-028 (AUD/BEN). Reused by SCH-050.

<!-- covers: INV-0407 -->

#### Out of scope
Capture-to-playback round trip (AUD, B-028). IRQ steering (SCH-050).

#### Acceptance criteria
- [ ] The harness runs a periodic callback under LowLatency and under Realtime on H-002 and H-004, with a Background plus Throughput flood.
- [ ] Each run publishes jitter p50/p99 and missed-period counts per class, plus a PipeWire-on-Linux baseline on the same machine.
- [ ] Reports are stored under the B-010 (and linked B-028) report layout for those H-IDs.
- [ ] The harness is callable from the BEN runner.

#### Verification
- Bench: B-010 on H-002 and H-004; target per register. B-028 cited as the complementary AUD metric.
- Integration: callback harness on `hw-h002` and `hw-h004`.

#### Evidence
- none

### SCH-029 · Benchmark Interactive latency while a Throughput build runs
- Type: benchmark
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: SCH-026, SCH-010, BEN-005
- Baseline: §22, §54, §61
- Benchmarks: B-045

Full parallel kernel build under Throughput intent while measuring keystroke-to-render latency of the editor. Publish beside Linux nice on the same machine. Part of the V1 self-hosting story. Numbers live in B-045 (and B-010 for the Interactive wakeup column).

<!-- covers: INV-0410 -->

#### Out of scope
Build-time of the image (B-039, BLD). Editor features (APP).

#### Acceptance criteria
- [ ] The harness starts a Throughput-domain kernel build and an Interactive editor on H-002 and H-004.
- [ ] It publishes keystroke-to-render p50/p99 and Interactive wakeup p99, plus a Linux nice baseline on the same machine.
- [ ] Reports land in the B-045 (and B-010) paths for those H-IDs.
- [ ] The BEN runner can invoke the harness for B-051.

#### Verification
- Bench: B-045 and B-010 on H-002 and H-004; target per register.
- Integration: self-host compile-while-typing scenario on H-004.

#### Evidence
- none

### SCH-030 · Decide which ResourceDomain and intent surfaces are Layer 1 freeze candidates
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: ABI-011, SCH-007, SCH-010, SCH-025, SCH-026, SCH-001
- Baseline: §23, §65, §66
- Decision: D-0254
- Risks: R-007, R-054
- Invariants: I-040, I-055

Decide which domain and intent operations sit in Layer 1 versus the Layer 2 policy-profile schema, recorded against S-009. L1 surfaces are prototyped through V0, freeze candidates at V1, frozen at V4 (I-040). High-level profiles stay in userspace (I-055). This adr names candidates; it does not freeze. Required by V4-G01 (Layer 1 ABI frozen with a conformance suite): the S-009 freeze and its conformance suite (SCH-058) start from the candidate list this decision records.

#### Out of scope
The V4 freeze (ABI-049). Conformance suite (SCH-058).

#### Acceptance criteria
- [ ] The decision file lists at least a small L1 (create, attach, inspect, intent word) with profiles at L2, and a wide L1 that includes GPU/network/energy policy.
- [ ] Each L1 candidate cites SCH-012 or SCH-001 as evidence.
- [ ] S-009 is marked freeze-candidate, not frozen, and Layer 2 profiles remain versioned.
- [ ] Review sign-off is recorded on the pull request.

#### Verification
- Review: ABI lead records sign-off on the pull request and the surfaces register update.

#### Evidence
- none

### SCH-031 · Add ResourceDomain GPU budget enforced through graphics and compute queues
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: SCH-007, SCH-003
- Baseline: §23, §37, §39

GPU time share and GPU memory budget per domain, enforced via the DRM scheduler and MemoryObject charging. V1 GPU acceleration for the Linux-personality browser and IDE makes GPU contention real. GFX owns the spike; SCH owns the domain field and enforcement API.

<!-- covers: INV-0428 -->

#### Out of scope
DRM scheduler internals (GFX). ComputeDevice dispatch (HET). Native GPU driver stack (forbidden before 1.0).

#### Acceptance criteria
- [ ] A domain with a GPU time share cannot exceed it on H-002 under a competing compute plus compositor load, measured through the DRM scheduler stats exported to `os inspect`.
- [ ] A domain GPU-memory cap that is exceeded returns a typed exhaustion error on MemoryObject GPU maps.
- [ ] The compositor domain still presents while a background compute domain is at its cap.
- [ ] Native software does not open DRM device nodes to set the budget.

#### Verification
- Integration: compositor versus compute contention on H-002 and H-004.
- Unit: `kernel:tests/sch/gpu_budget_*` on `qemu-x86_64` (skip where no GPU).
- Review: GFX and SCH leads record sign-off on the pull request.

#### Evidence
- none

### SCH-032 · Add ResourceDomain I/O budget for bandwidth and IOPS of storage Operations
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: SCH-007, SCH-003, TSK-033
- Baseline: §18, §23

Per-domain bandwidth and IOPS limits on storage Operations, using the io controller or native accounting per SCH-003. `os env` builds and indexing must not saturate the disk during daily driving.

<!-- covers: INV-0429 -->

#### Out of scope
Network bandwidth (SCH-033). Storage quota of persistent bytes (SCH-034).

#### Acceptance criteria
- [ ] A domain with an IOPS cap that is exceeded has further storage Operations complete with a typed throttle or exhaustion result, not a hang.
- [ ] An Interactive file-read Operation in another domain still completes while a Throughput domain is at its I/O cap, on H-004.
- [ ] `os inspect resource` reports bytes and IOPS used versus cap.
- [ ] Native software does not write blkio cgroup files.

#### Verification
- Unit: `kernel:tests/sch/io_budget_*` on `qemu-x86_64` and `hw-h004`.
- Integration: `os env` build versus editor save on H-004.

#### Evidence
- none

### SCH-033 · Add ResourceDomain network policy applied to network Capabilities
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: SCH-007, NET-006, NET-012
- Baseline: §9.1, §23

Allowed connectivity and bandwidth on the domain, enforced when a network Capability is exercised. V1 exit requires that a Component without a network Capability cannot connect. NET owns the broker; SCH owns the domain field `os env` reuses for network namespaces.

<!-- covers: INV-0430 -->

#### Out of scope
Capability minting and firewall (NET). `os env` NetworkNamespace (ENV).

#### Acceptance criteria
- [ ] A Component whose domain policy denies network, and that holds no network Capability, cannot open a connection; the result is `Error::Rights`.
- [ ] A domain bandwidth cap that is exceeded throttles further send Operations with a typed result.
- [ ] `os inspect resource` prints the domain network policy and current bytes.
- [ ] Policy is not configured through nftables from native crates.

#### Verification
- Unit: `kernel:tests/sch/net_policy_*` on `qemu-x86_64`.
- Integration: V1 network-denial demo on H-004.
- Compat: C-002 scenario that a sandboxed native helper cannot connect.

#### Evidence
- none

### SCH-034 · Add ResourceDomain storage quota over the domain's persistent objects
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: SCH-007, SCH-003
- Baseline: §23, §25, §27

Quota accounting over content-store and user objects owned by the domain, surfaced as typed errors. Required before third-party packages and environments consume disk in V1. STO owns per-user quota objects at V2; this task is the domain field and the charge points SCH can enforce without that object.

<!-- covers: INV-0431 -->

#### Out of scope
STO-067 user quota objects. Snapshot policy (STO).

#### Acceptance criteria
- [ ] Creating persistent objects past the domain quota returns a typed exhaustion error and does not grow the store for that domain.
- [ ] `os inspect resource` reports persistent bytes versus quota.
- [ ] Deleting objects returns bytes to the domain quota.
- [ ] Native software does not set project quotas through Linux quota tools.

#### Verification
- Unit: `kernel:tests/sch/storage_quota_*` on `qemu-x86_64`.
- Integration: `os env` disk fill on H-004.

#### Evidence
- none

### SCH-035 · Run indexing services under Background intent preempted by Interactive work
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: SCH-010, SCH-041, SCH-021
- Baseline: §22

Search and indexing service domain profile set to Background with I/O priority mapping. Preemption test shows Interactive latency unchanged while indexing runs. APP owns the indexer Component; SCH owns the profile and the preemption proof.

<!-- covers: INV-0411 -->

#### Out of scope
Desktop search UI (APP-027). Store-side index (STO).

#### Acceptance criteria
- [ ] The indexing service domain profile is Background, visible in `os inspect resource`.
- [ ] Interactive editor keystroke-to-render on H-004 with indexing on matches the B-045 idle-index-off band recorded in the verifying report.
- [ ] Indexing I/O is visible as Background in the I/O priority path.
- [ ] Stopping Interactive work lets indexing run to completion.

#### Verification
- Integration: preemption test on H-004.
- Bench: B-045 and B-010 Interactive-with-index column on H-004.

#### Evidence
- none

### SCH-036 · Influence core selection on hybrid CPUs and SMT siblings from intent
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: SCH-026, SCH-042
- Baseline: §22, §38
- Threats: T-015

Performance versus efficiency core placement, and SMT sibling avoidance for LowLatency and Realtime. H-004 (hybrid Intel) enters hardware scope at V1. Side-channel statement at V1 names SMT (T-015); this task implements the placement half, not the public statement.

<!-- covers: INV-0416 -->

#### Out of scope
SEC side-channel position statement. Frequency hints (SCH-038).

#### Acceptance criteria
- [ ] A LowLatency or Realtime Task on H-004 runs on a performance core, shown by `os inspect` CPU id.
- [ ] A Background or EnergyEfficient Task prefers an efficiency core when one is idle.
- [ ] Realtime Tasks do not share an SMT sibling with a different domain's Task when a free core exists.
- [ ] Placement is driven by intent, not by a native cpuset API.

#### Verification
- Integration: hybrid placement tests on H-004.
- Unit: topology fixture on `qemu-x86_64` with a fake hybrid map.
- Bench: B-010 LowLatency column on H-004.

#### Evidence
- none

### SCH-037 · Add EnergyEfficient intent class trading latency for lower power
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: SCH-004, SCH-010
- Baseline: §22, §54, §61

Class that prefers efficient cores and lower frequencies via uclamp.max/EPP hints. V1 publishes idle power and battery runtime on H-004 (B-031). PWR enables the platform; SCH owns the class.

<!-- covers: INV-0405 -->

#### Out of scope
Domain energy policy object (SCH-045). Battery reporting (PWR).

#### Acceptance criteria
- [ ] A Task can declare EnergyEfficient through the native ABI.
- [ ] On H-004, EnergyEfficient Tasks run with EPP or uclamp.max in the efficient range, visible through inspect.
- [ ] Interactive Tasks on the same machine are not placed in that efficient range while runnable.
- [ ] B-031 idle and mixed runs on H-004 include an EnergyEfficient-on versus Interactive-on comparison with no superiority claim.

#### Verification
- Unit: `kernel:tests/sch/intent_energy_*` on `qemu-x86_64`.
- Integration: H-004 placement and EPP inspect dump.
- Bench: B-031 on H-004; target per register (V1 publish).

#### Evidence
- none

### SCH-038 · Drive cpufreq and EPP hints from Scheduling intent
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: SCH-037, SCH-010
- Baseline: §22, §54

Latency-sensitive work is not throttled and Background work is. PWR consumes frequency via SCH intent, so SCH owns the hint mapping. Measured on the H-004 energy gate (B-031).

<!-- covers: INV-0415 -->

#### Out of scope
Platform PM enablement (PWR-009). Governor rewrite (forbidden; retain Linux cpufreq).

#### Acceptance criteria
- [ ] An Interactive or LowLatency Task running on H-004 raises the CPU frequency hint above the idle EnergyEfficient hint, visible in inspect.
- [ ] A Background-only load on H-004 keeps the hint in the efficient range.
- [ ] Native software does not call cpufreq sysfs; hints flow from intent.
- [ ] B-031 runs record the hint mapping used.

#### Verification
- Integration: hint traces on H-004.
- Bench: B-031 on H-004; target per register.
- Review: PWR and SCH leads record sign-off on the pull request.

#### Evidence
- none

### SCH-039 · Map Scheduling intent to GPU priority for RenderQueue and ComputeQueue
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: SCH-010, HET-008, GFX-006
- Baseline: §22, §37, §39

Domain intent sets DRM scheduler priority for render and compute submissions so the compositor wins over background compute. V1 brings GPU acceleration for native and Linux-personality apps. GFX owns RenderQueue; HET owns ComputeQueue; SCH owns the mapping.

<!-- covers: INV-0414 -->

#### Out of scope
GPU time budget (SCH-031). Native GPU drivers.

#### Acceptance criteria
- [ ] Compositor RenderQueue submissions from an Interactive/Deadline domain outrank Throughput ComputeQueue submissions on H-002, shown by DRM scheduler stats in inspect.
- [ ] A Background domain cannot raise GPU priority above the compositor domain.
- [ ] Native apps do not call DRM priority ioctls; the domain intent is the input.
- [ ] Linux-personality GPU clients inherit the enclosing Component domain's GPU priority.

#### Verification
- Integration: compositor versus compute on H-002 and H-004.
- Bench: B-018 on H-002 under background compute.

#### Evidence
- none

### SCH-040 · Write developer guidelines for choosing Scheduling intent and domain budgets
- Type: docs
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: SCH-010, SCH-026, SCH-025, SCH-042, SCH-037, DOC-010
- Baseline: §22, §52, §66

SDK v1 ships at V1 and DOC generates references from IDL. SCH supplies the guide mapping §22 examples (audio, compositor, compiler, indexing, sync) to classes and budgets so third-party developers do not reach for numeric priority.

#### Out of scope
IDL page generation (DOC). SDK crate docs layout (SDK).

#### Acceptance criteria
- [ ] A guide page exists for each of Interactive, Background, Throughput, LowLatency, Deadline, Realtime and EnergyEfficient with one §22 example.
- [ ] The guide states that numeric nice is not a native API and links S-009.
- [ ] The page is linked from the generated IDL reference for ResourceDomain.
- [ ] Review sign-off is recorded.

#### Verification
- Review: DOC and SDK leads record sign-off on the pull request.
- Manual: open the V1 docs site and follow ResourceDomain to the guide.

#### Evidence
- none

### SCH-041 · Map Scheduling intent to I/O priority for storage and network Operations
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: SCH-010, TSK-033
- Baseline: §18, §22, §23

Operations submitted by a domain carry its intent into block-layer and network queue priority so Background indexing does not starve Interactive file access. TSK owns Operation priority ordering; SCH owns the intent-to-priority map.

<!-- covers: INV-0418 -->

#### Out of scope
I/O budget caps (SCH-032). Operation ring (TSK).

#### Acceptance criteria
- [ ] An Interactive Read Operation completes while a Background domain floods storage, on H-004, with completion order matching intent.
- [ ] Network send Operations from a Background domain are queued behind Interactive sends from another domain on the same link.
- [ ] Native software does not call `ionice`.
- [ ] `os trace` shows the mapped I/O class per Operation.

#### Verification
- Unit: `kernel:tests/sch/io_priority_*` on `qemu-x86_64` and `hw-h004`.
- Integration: editor save versus indexer on H-004.

#### Evidence
- none

### SCH-042 · Add Realtime intent class with bounded scheduling latency
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: SCH-004, SCH-026, SCH-016
- Baseline: §22, §9.1

Bounded-latency class with admission control, quota fallback and a grant requirement. Critique moves Realtime to V1 with the native audio path. Validated by SCH-028. This is bounded-latency best effort, not hard real time.

<!-- covers: INV-0404 -->

#### Out of scope
The elevated-intent Capability UI (SCH-048). Audio stream objects (AUD).

#### Acceptance criteria
- [ ] A Task can declare Realtime only when its domain holds the grant named by the V1 grant path; otherwise the result is `Error::Rights`.
- [ ] Admission that would exceed the domain CPU quota returns a typed error and the Task is not started as Realtime.
- [ ] B-010 Realtime wakeup p99 on H-002 and H-004 is published; the class is documented as best effort.
- [ ] Overrun falls back to the domain quota rather than starving Interactive work in other domains.

#### Verification
- Unit: `kernel:tests/sch/intent_realtime_*` on `qemu-x86_64` and `hw-h002`.
- Bench: B-010 and SCH-028 on H-002 and H-004.
- Review: ABI lead records that Realtime is not promised as hard real time.

#### Evidence
- none

### SCH-043 · Map Linux Personality nice, ionice and sched_setattr onto domain-bounded intents
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: SCH-004, SCH-006, LNX-001, SCH-010
- Baseline: §3, §22, §46, §57

V1 daily-driving runs the browser and IDE through the Linux personality. Their scheduler syscalls must translate into intents bounded by the app's ResourceDomain rather than raw priorities. LNX owns the syscall path; SCH owns the mapping policy. Native software still never sees those syscalls. L2 overhead is published as B-026, not claimed here.

#### Out of scope
Syscall implementation (LNX). Native SDK policy (SCH-022).

#### Acceptance criteria
- [ ] `nice`, `ionice` and `sched_setattr` inside a Linux-personality Component change only the mapping inside that Component's ResourceDomain.
- [ ] A personality process cannot obtain Realtime or a CPU share above its domain by calling `SCHED_FIFO`.
- [ ] Native Components still cannot invoke those syscalls (ABI filter).
- [ ] B-026 L2 runs include this mapping with no extra native priority API.

#### Verification
- Integration: browser/IDE personality scheduler syscalls on H-004.
- Compat: C-002 on H-002.
- Bench: B-026 on H-002; target per register (V1 publish).
- Unit: `kernel:tests/sch/lnx_sched_map_*` on `qemu-x86_64`.

#### Evidence
- none

### SCH-044 · Benchmark Interactive and Deadline latency under a scripted mixed desktop workload
- Type: benchmark
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: SCH-025, SCH-035, SCH-046, SCH-047, BEN-005
- Baseline: §22, §54, §62
- Benchmarks: B-019

Run build, indexing, sync and a game concurrently and publish per-intent latency and deadline-miss rates on H-002, H-004 and H-005 beside Linux. This is the SCH half of the V2 compositor deadline-miss gate (B-019). Required by V2-G16 (Prior benchmarks show no unexplained regression): B-019 is published in that session and this harness is its scheduler half.

#### Out of scope
Compositor miss counting (GFX-060). Gaming FPS (B-027, WIN).

#### Acceptance criteria
- [ ] The harness runs the mixed workload on H-002, H-004 and H-005 for the duration named in B-019.
- [ ] It publishes per-intent wakeup latency and compositor deadline-miss rate beside a Linux desktop on the same machine.
- [ ] Reports land in the B-019 (and B-010) paths for those H-IDs.
- [ ] The BEN runner can invoke it for B-051.

#### Verification
- Bench: B-019 and B-010 on H-002, H-004 and H-005; target per register.
- Integration: mixed-workload script on those machines.

#### Evidence
- none

### SCH-045 · Add ResourceDomain energy policy influencing frequency and core selection
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: SCH-037, SCH-038, SCH-036
- Baseline: §22, §23, §62

Domain-level energy policy (performance, balanced, efficient) that PWR reads for frequency governors and that biases core selection. V2 laptops gate on battery estimate and mixed-workload runtime (B-031).

<!-- covers: INV-0432 -->

#### Out of scope
Charge profiles UI (PWR, APP). Per-Task EnergyEfficient class already in SCH-037.

#### Acceptance criteria
- [ ] A domain energy policy of efficient is visible in `os inspect resource` and drives EPP/core selection for member Tasks that did not override intent.
- [ ] Performance policy on an Interactive domain is not overridden by a system-wide efficient default while the user is typing, on H-004 and H-005.
- [ ] PWR frequency ceilings honour the domain policy without native sysfs writes from apps.
- [ ] B-031 mixed-workload runs on both laptops record the policy in use.

#### Verification
- Integration: policy switch on H-004 and H-005.
- Bench: B-031 on H-004 and H-005; target per register.

#### Evidence
- none

### SCH-046 · Run battery-powered sync tasks under EnergyEfficient intent
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: SCH-037, SCH-045
- Baseline: §22, §62

Sync and background network work on battery prefers efficient cores and low frequency. Measured within the V2 mixed-workload battery runtime gate (B-031) on H-004 and H-005.

<!-- covers: INV-0412 -->

#### Out of scope
Sync product (APP/NET). Thermal throttling (SCH-051).

#### Acceptance criteria
- [ ] The sync service domain is EnergyEfficient when the machine is on battery, visible in inspect on H-004 and H-005.
- [ ] On AC, the same service may run Throughput without a code change, driven by policy.
- [ ] B-031 mixed-workload runtime includes this sync domain.
- [ ] Interactive work still preempts sync.

#### Verification
- Integration: AC versus battery inspect dumps on H-004 and H-005.
- Bench: B-031 on H-004 and H-005.

#### Evidence
- none

### SCH-047 · Run game render threads, including Windows Personality games, under latency intent
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: SCH-026, WIN-054, WIN-040
- Baseline: §3, §22, §48, §56.2

Wine/Proton render and present threads inherit a LowLatency intent within the game's domain. Native software still never sees Win32. Verified by B-027 publication against Linux plus Proton, with no superiority claim.

<!-- covers: INV-0409 -->

#### Out of scope
Wine bring-up (WIN). HDR/VRR (GFX). Anti-cheat (non-goal).

#### Acceptance criteria
- [ ] A W1 Gold title's render and present threads are LowLatency inside the game Component's ResourceDomain, shown by inspect on H-002.
- [ ] The game domain cannot raise Realtime without the elevated-intent Capability.
- [ ] B-027 runs on H-002 record the intent mapping.
- [ ] A second Windows title in another Component does not inherit the first title's intent.

#### Verification
- Integration: inspect dump during WIN-029 on H-002.
- Bench: B-027 on H-002; target per register (V2 publish).
- Compat: C-007 W1 Gold sample on H-002.

#### Evidence
- none

### SCH-048 · Gate Realtime and LowLatency intent behind Capabilities shown in the permissions UI
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: SCH-042, SCH-026, SEC-007, SEC-045, PKG-028
- Baseline: §9.1, §22
- Threats: T-001
- Invariants: I-021

Elevated intents are a resource grant, requested in the package manifest and revocable (I-021, T-001). V2 exit requires the permissions UI to list every active grant and revocation to take effect immediately. SEC owns the UI; SCH owns the check at intent-declare time.

<!-- covers: INV-0421 -->

#### Out of scope
Permissions UI widgets (SEC, APP). Manifest schema (PKG).

#### Acceptance criteria
- [ ] Declaring Realtime or LowLatency without the grant returns `Error::Rights` and does not change the Task's intent.
- [ ] The permissions UI lists the grant for a holding Package; revoking it drops member Tasks to the domain default on the next Operation.
- [ ] Interactive and Background do not require the grant.
- [ ] Manifest request plus user deny still launches the app in degraded intent, not a crash.

#### Verification
- Unit: `kernel:tests/sch/intent_grant_*` on `qemu-x86_64`.
- Integration: permissions UI revoke on H-002 with camera-style grant tests extended to intent.
- Review: SEC lead records that elevated intents appear in the grant list.

#### Evidence
- none

### SCH-049 · Influence NUMA and GPU-local memory placement from domain intent
- Type: build
- Milestone: V2
- Status: todo
- Size: L
- Owner: none
- Depends on: SCH-010, MEM-034, MEM-041, HET-014
- Baseline: §17, §22, §37, §38

Domain intent and ComputeDevice preference steer allocation to a NUMA node or GPU-local versus host memory. The V2 ComputeDevice demo dispatches Throughput to the GPU and LowLatency to the CPU. MEM owns attributes; HET owns dispatch; SCH owns the intent bias.

<!-- covers: INV-0417 -->

#### Out of scope
MemoryObject ABI (MEM). ComputeDevice object (HET).

#### Acceptance criteria
- [ ] A LowLatency domain's anonymous allocations prefer the local NUMA node of the running CPU, queryable through inspect.
- [ ] A Throughput GPU dispatch uses GPU-local MemoryObjects when the device has them, on H-002.
- [ ] A domain without GPU budget cannot force GPU-local placement.
- [ ] Native software does not call `mbind` or DRM placement ioctls.

#### Verification
- Integration: HET-013 on H-002.
- Unit: NUMA fixture on `qemu-x86_64` with two nodes.
- Bench: B-048 on H-002; target per register (V2 publish).

#### Evidence
- none

### SCH-050 · Steer interrupt affinity from LowLatency domain CPU policy
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: SCH-006, SCH-042, SCH-028
- Baseline: §22, §33

Domain CPU policy for LowLatency and Realtime domains pins relevant device IRQs to or away from the domain's allowed cores. Effect published through SCH-028 on H-002, H-004 and H-005.

<!-- covers: EXTRA-014 -->

#### Out of scope
User-space driver IRQ delivery (HW). Audio path (AUD).

#### Acceptance criteria
- [ ] A LowLatency audio domain's allowed-core set is reflected in the IRQ affinity of its capture/playback device, visible in inspect.
- [ ] Moving the domain's allowed cores updates IRQ affinity without a native `/proc/irq` write from the app.
- [ ] B-010/audio-callback reports on H-002, H-004 and H-005 include an affinity-on versus affinity-off column.
- [ ] Interactive domains that are not LowLatency/Realtime do not steal device IRQs from a LowLatency domain.

#### Verification
- Integration: IRQ inspect dumps on H-002, H-004 and H-005.
- Bench: SCH-028 on those machines.
- Review: HW lead records that driver IRQ routing remains inherited Linux, steered only by domain policy.

#### Evidence
- none

### SCH-051 · Throttle Background and Throughput domains first under thermal pressure
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: SCH-010, SCH-026, PWR-022, SCH-006
- Baseline: §22, §23, §62

When PWR reports capacity loss the scheduler reduces Throughput and Background quotas before touching Interactive or Deadline, keeping the desktop UX script passing on H-004 and H-005. PWR owns trip points; SCH owns the quota response.

#### Out of scope
Thermal-zone drivers (HW). Capacity-loss signal (PWR-022).

#### Acceptance criteria
- [ ] Injecting a PWR capacity-loss event drops Throughput and Background quotas while Interactive/Deadline quotas remain, on H-004 and H-005.
- [ ] Clearing the event restores the previous quotas.
- [ ] The V2 desktop UX script still passes under a sustained thermal event on both laptops.
- [ ] Native apps do not read thermal sysfs to implement this.

#### Verification
- Integration: fault-injected capacity-loss on H-004 and H-005.
- Demo: desktop UX script under thermal event on H-005.
- Bench: B-019 miss rate on H-004 under the same event.

#### Evidence
- none

### SCH-052 · Create per-session ResourceDomain roots with fair sharing between users
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: SCH-019, SEC-060, SEC-042
- Baseline: §9.1, §23, §63
- Threats: T-026

Each login Session receives a delegated domain root so one user's Throughput work cannot starve another's Interactive session (T-026). Tested with two concurrent sessions. SEC owns Session; SCH owns the domain tree.

#### Out of scope
Session object (SEC). Greeter (APP). Per-user encryption (SEC).

#### Acceptance criteria
- [ ] Two concurrent sessions each have a ResourceDomain root; inspect shows no shared parent that either user can derive from.
- [ ] User A's Throughput flood does not move user B's Interactive wakeup-to-run outside the B-010 band recorded for a quiet session, on a V3 Tier 1 machine.
- [ ] Ending a session destroys its domain tree and reclaims budgets.
- [ ] A user cannot derive into another user's tree (`Error::Rights`).

#### Verification
- Integration: two-session test on H-002 and H-004.
- Bench: B-010 per-session column on H-002.
- Unit: `kernel:tests/sch/session_roots_*` on `qemu-x86_64`.

#### Evidence
- none

### SCH-053 · Apply default ResourceDomain profiles to third-party packages from the public repository
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: SCH-021, SCH-048, PKG-084
- Baseline: §9.1, §23, §28

V3 opens the public repository to third-party packages. Packages without a declared profile receive conservative budgets and Interactive-by-default intent. Elevated intents require the intent-escalation Capability reviewed at install.

#### Out of scope
Repository signing (REL). Manifest identity (PKG).

#### Acceptance criteria
- [ ] Installing a Package with no profile creates a domain with the conservative default budget set named in the profile schema.
- [ ] Default intent is Interactive; Realtime or LowLatency requires the grant at install review.
- [ ] A Package that declares a profile exceeding the session root is rejected at install with a typed error.
- [ ] `os inspect` after install shows the applied profile.

#### Verification
- Integration: third-party Package install on H-002.
- Unit: profile-default tests in `userspace:tests/sch/package_defaults_*`.
- Review: PKG and SEC leads record that install review lists elevated intents.

#### Evidence
- none

### SCH-054 · Add fuzz and soak targets for ResourceDomain and intent syscalls
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: SCH-007, SCH-019, SCH-010, BLD-016, BLD-035
- Baseline: §23, §51, §63
- Risks: R-051

SCH supplies fuzz targets for domain create, delegation, budget updates and intent changes, plus a soak that checks accounting leaks over many domain lifecycles, feeding BLD's fuzzing fleet. V3 requires continuous syscall fuzzing with no known open crasher older than the register window.

#### Out of scope
Fuzz infrastructure (BLD). Operation fuzz (TSK).

#### Acceptance criteria
- [ ] syzlang descriptions exist for domain create, derive, budget update, attach, destroy and intent declare.
- [ ] A soak of domain create/destroy cycles reports no leaked domain handles via inspect enumeration.
- [ ] Targets are registered in BLD continuous fuzzing and appear in the V3 crasher-age dashboard.
- [ ] No SCH target has an open crasher older than the V3 window at the gate.

#### Verification
- Fuzz: BLD fleet overnight on the SCH targets.
- Integration: soak job in nightly CI on H-001.
- Review: BLD lead records the targets on the V3 fuzz gate.

#### Evidence
- none

### SCH-055 · Validate intent and domain benchmarks across the six Tier 1 machines
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: SCH-001, SCH-013, SCH-028, SCH-029, SCH-044, BEN-053
- Baseline: §54, §62, §63

Re-run the SCH harnesses on every V3 Tier 1 topology (H-002, H-004, H-005, H-006, H-007, H-008). B-051 carries the regression band versus V2. Public numbers for B-010, B-011, B-019 and B-031 live in the register, not in this task's prose. Required by V3-G15 (Prior benchmarks re-run across Tier 1): the B-010, B-011 and B-019 rows of that re-run come from these harnesses.

#### Out of scope
Harness implementation (earlier SCH bench tasks). Dashboards (BEN, BLD).

#### Acceptance criteria
- [ ] B-010, B-011, B-019 and the audio-callback harness have V3 reports on H-002, H-004, H-005, H-006, H-007 and H-008.
- [ ] Any result outside the B-051 band has an accepted Decision named in the report, or the result is inside the band.
- [ ] Hybrid Intel, AMD APU and NVIDIA desktop topologies are each represented.
- [ ] The generated benchmark results block lists those reports.

#### Verification
- Bench: B-010, B-011, B-019, B-051 on H-002, H-004, H-005, H-006, H-007, H-008; target per register.
- Review: BEN lead records the V3 SCH table.

#### Evidence
- none

### SCH-056 · Tune EnergyEfficient intent and energy policy on every Tier 1 laptop
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: SCH-045, SCH-037, SCH-038, PWR-027
- Baseline: §22, §23, §54, §62

V4 publishes idle and mixed-workload battery runtime for every Tier 1 laptop beside mainline Linux (B-031). Tune per-machine EPP and core-selection tables and record results without superiority claims. Required by V4-G17 (Prior benchmarks within the V4 regression band): B-031 for every Tier 1 laptop is one of the metrics published there.

#### Out of scope
Power meters and method (PWR, LAB, BEN). Charge-threshold UX (APP).

#### Acceptance criteria
- [ ] Each Tier 1 laptop has a checked-in EPP/core-selection table used by EnergyEfficient and domain energy policy.
- [ ] B-031 idle and mixed-workload reports exist for every Tier 1 laptop beside mainline Linux on the same machine.
- [ ] No documentation claims superiority without those reports (I-061).
- [ ] Changing a table is a reviewed SCH change, not an ad-hoc sysfs tweak.

#### Verification
- Bench: B-031 on every Tier 1 laptop; target per register.
- Review: PWR and SCH leads record the per-machine tables.

#### Evidence
- none

### SCH-057 · Report scheduling latency and budget-exhaustion incidents from the beta fleet
- Type: docs
- Milestone: V4
- Status: todo
- Size: S
- Owner: none
- Depends on: SCH-014, REL-042, BEN-049
- Baseline: §23, §54, §63

V4 requires crash-free sessions across the fleet (B-041). Budget-exhaustion terminations and latency spikes appear as crashes or hangs in opt-in telemetry. SCH publishes an analysis and files fixes before feature freeze. Opt-in only (T-023).

#### Out of scope
Telemetry intake (REL). Crash capture format (OBS). Privacy policy (GOV).

#### Acceptance criteria
- [ ] A report lists budget-exhaustion terminations and wakeup-latency spikes from opt-in fleet data, with machine counts and no identifiers.
- [ ] Each cluster either names a SCH fix task or an accepted Decision that the rate is inside B-041.
- [ ] The report is linked from the V4 roll-up.
- [ ] Review sign-off is recorded.

#### Verification
- Review: REL and SCH leads record sign-off on the pull request.
- Bench: B-041 cited; target per register.

#### Evidence
- none

### SCH-058 · Ship the conformance suite for frozen ResourceDomain and intent Layer 1 surfaces
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: SCH-030, ABI-047, SCH-007, SCH-042, ABI-049
- Baseline: §23, §65, §66
- Freezes: S-009
- Invariants: I-040

V4 freezes Layer 1 with a conformance suite. Tests every domain and intent syscall, error code and negotiation behaviour named by SCH-030 so 1.x binaries stay compatible. This suite carries the S-009 freeze once ABI-049 is accepted.

#### Out of scope
The freeze ADR (ABI). Layer 2 profile evolution tests (IPC).

#### Acceptance criteria
- [ ] Every Layer 1 domain and intent operation named by SCH-030 has a conformance test.
- [ ] A binary built against the freeze candidate passes the suite on a later V4 RC kernel.
- [ ] Wrong-type, `Error::Rights` and exhaustion cases are covered.
- [ ] The suite is in ABI's V4 compatibility run.

#### Verification
- Integration: conformance suite on H-001 and every V4 RC image.
- Review: ABI lead records S-009 frozen only after this suite and ABI-049.

#### Evidence
- none

### SCH-059 · Sign off SCH benchmarks within the B-051 regression band versus V4 on Tier 1
- Type: build
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: SCH-055, SCH-056, BEN-061
- Baseline: §54, §63

1.0 gate: no regression beyond the register band versus V4 final on any tracked SCH benchmark on any Tier 1 machine (B-051). Run the SCH harnesses on the release candidate during the soak and link results in the release notes.

#### Out of scope
Cross-OS publication pack (BEN-063). Channel launch (REL).

#### Acceptance criteria
- [ ] B-010, B-011, B-019 and B-031 1.0 reports exist for every Tier 1 machine against the V4 final reports.
- [ ] Any exceedance of the B-051 1.0 band has an accepted Decision named in the release notes.
- [ ] Release notes link the reports and make no unmeasured claim.
- [ ] The soak candidate is the same image REL ships.

#### Verification
- Bench: B-010, B-011, B-019, B-031, B-051 on every Tier 1 machine; target per register.
- Review: BEN and REL leads record sign-off on the release notes.

#### Evidence
- none

### SCH-060 · Write the 1.0 stability statement for ResourceDomain and Scheduling intent
- Type: docs
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: SCH-030, SCH-058, ABI-049
- Baseline: §22, §23, §65, §66

1.0 publishes an ABI stability statement and explicit non-promises. SCH's section lists frozen surfaces, the versioned profile schema deprecation policy, and states that Realtime is bounded-latency best effort, not hard real time.

#### Out of scope
The global ABI statement (ABI). Layer 3 semver (SDK).

#### Acceptance criteria
- [ ] The SCH section names every frozen S-009 operation and the Layer 2 profile schema deprecation overlap.
- [ ] It states that Realtime is not hard real time and that cgroupfs is not a native API.
- [ ] It lists non-promises: no CHERI enforcement, no superiority on B-010/B-031 without reports.
- [ ] Review sign-off is recorded.

#### Verification
- Review: ABI and SCH leads record sign-off on the pull request.
- Manual: the 1.0 documentation snapshot includes the section.

#### Evidence
- none

### SCH-061 · Allow per-ResourceDomain pluggable scheduling policies via sched_ext
- Type: build
- Milestone: LATER
- Status: todo
- Size: L
- Owner: none
- Depends on: KRN-024, SCH-004, SCH-058
- Baseline: §6, §22, §65
- Risks: R-034

Parking rung: custom user-supplied scheduling policies scoped to a domain depend on KRN's eBPF role adr and add divergence risk (R-034). Deferred until intent classes are frozen and demand is shown. Native software still expresses intent, not BPF programs, unless this task returns with an accepted follow-up Decision. Required by the design critique: "Decide eBPF's native role (tracing substrate, sched_ext, network policy)".

#### Out of scope
eBPF tracing substrate (OBS, KRN). Frozen intent classes (this task must not unfreeze S-009).

#### Acceptance criteria
- [ ] A domain-scoped sched_ext program can be attached only with an explicit Capability, and only after KRN-024 accepts that role.
- [ ] A domain without the Capability is unaffected by another domain's program.
- [ ] The native intent ABI remains the default; BPF is opt-in and inspectable.
- [ ] If the eBPF adr rejects native sched_ext, this task is dropped with that reason rather than implemented.

#### Verification
- Review: KRN and SCH leads record whether demand and the eBPF adr justify promotion off LATER.
- Integration: two-domain isolation test on H-001 if implemented.
- Fuzz: domain-scoped program loader if implemented.

#### Evidence
- none
