# SVC · Service lifecycle, user-space driver hosting, core system services
- Prefix: SVC
- Lead: none
- Baseline: §32, §33
- Baseline gap: §32 and §33 require restartable services and user-space drivers but do not own init, supervision trees, settings storage, time, locale or hostname.

<!-- roadmap:generated:begin summary -->
Tasks: 43 live, 0 done, 0 in-progress, 43 todo, 0 dropped. Ready: 2. Blocked: 41. Weighted: 0%.
<!-- roadmap:generated:end -->

## Scope

SVC owns native init, the service supervisor, and the Layer 2 service manifest. It starts system and session services as Components from a SystemGeneration, orders them by declared dependencies and readiness, applies restart policy, and re-advertises Interfaces so clients disconnect, rebind, retry and restore-state (§32). It hosts user-space driver Components under the same supervisor (§33), escalates exhausted budgets to explicit degraded recovery or the safe-mode session, and falls back to a text recovery console when no compositor can start.

It also owns core typed system services that the baseline left unnamed: settings storage and restore, time and NTP or NTS, locale and keyboard-layout settings, hostname and machine identity, structured service logs, the default-application registry that opens by Capability, and the `os service` plus `os inspect service` control plane.

## Out of scope

Firmware, bootloader, generation selection and pre-boot unlock UI (BOOT). Package manifests, SystemGeneration compose and the `os history` log (PKG). Compositor surface rebind and frame scheduling (GFX). IDL, client rebind stubs and service discovery (IPC). The `os` CLI binary and client libraries (SDK). Component identity, exit causes and launch (CMP). Object<Device>, driver framework and per-class residency (HW). Identity, login, FDE and location (SEC). Installer, updater and the recovery environment (INS). Inspect transport, trace and crash-capture format (OBS). systemd emulation and personality `/etc` views (LNX). Settings UI, greeter, session chrome and safe-mode UI (APP). Per-service rebind inside audio, network, Bluetooth, print and GPU userspace (AUD, NET, HW, GFX). Deadline representation in the Operation ABI (TSK). Locale data source (TXT). Benchmark register ownership (BEN). Fault-injection CI plumbing (BLD).

## Tasks

### SVC-001 · Benchmark supervisor death-detection to ready latency for a restarted service
- Type: benchmark
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: SVC-015, SVC-008, Q-001, BEN-005, BEN-007, GFX-002, SVC-002, SDK-012
- Baseline: §32
- Benchmarks: B-023, B-024

The V0.5 compositor restart-to-rebound gate cites B-023, owned by BEN. This harness publishes the supervisor-attributable share of that path: death detection, respawn of the Component, and readiness, so regressions can be attributed to SVC rather than to GFX surface rebind. B-024 is the standing service restart-to-rebind metric this same harness feeds once additional services join the matrix.

<!-- covers: INV-0611 -->

#### Out of scope
Compositor surface rebind timing (GFX-009). Register ownership and cross-OS publication (BEN-007).

#### Acceptance criteria
- [ ] Harness `bench:service-rebind` records death-detection, respawn and readiness intervals as separate series on CI matrix entries `qemu-x86_64` and `hw-h002`.
- [ ] A report exists under `reports/benchmarks/B-024/` for H-001 and H-002 meeting the register target kind for V0.5.
- [ ] The report cites B-023 and B-024 and states no superiority claim.

#### Verification
- Bench: B-023 and B-024 on H-001 and H-002; target per register.
- Integration: `runtime:tests/svc/bench_respawn_*` on `qemu-x86_64`.
- Review: BEN lead confirms the series names match the register method.

#### Evidence
- none

### SVC-002 · Run the compositor and one other service under supervision and pass the restart Gate
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: SVC-009, SVC-015, GFX-010, GFX-009, BLD-020, APP-002, SVC-010
- Baseline: §32, §60
- Risks: R-023
- Invariants: I-037

V0.5 proves service restart and client rebind for the compositor and one other service (§60). GFX owns Surface rebind; SVC owns the supervision tree, restart policy and the CI loop that kills the compositor and the file-chooser service. Native applications under the test never observe POSIX process signals.

<!-- covers: INV-0592, INV-1185 -->

#### Out of scope
Surface restore-state (GFX-009). Chooser UI (APP-002). Fault-injection plumbing (BLD-020).

#### Acceptance criteria
- [ ] Killing the compositor 100 consecutive times on CI matrix entry `qemu-virtio-gpu` restarts it under the supervisor and rebinds every open window with no application exit.
- [ ] The same compositor-kill loop passes 20 consecutive times on `hw-h002`.
- [ ] Killing the file-chooser service restarts it; the Image Viewer rebinds to a new chooser Channel and completes a UserSelected open without exiting.
- [ ] `os inspect service` shows the compositor restart count increment and remaining budget after each kill.

#### Verification
- Integration: `runtime:tests/svc/compositor_restart_gate_*` on `qemu-virtio-gpu` and `hw-h002`.
- Demo: V0.5 compositor-kill demo on H-002; the Text Editor keeps its unsaved buffer.
- Review: GFX and SVC leads sign off on the split of rebind versus supervision on the pull request.

#### Evidence
- none

### SVC-003 · Decide native init versus retained initramfs/systemd for early boot
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: BOOT-004
- Baseline: §30, §32
- Decision: D-0300
- Risks: R-010, R-072

V0 boots native Components beside a retained initramfs (BOOT-004). This Decision is the baseline-gap scope for native init: when the native init Component takes over from Linux early userspace, and how disk unlock, verity setup and service supervision sit on that path. Reusing systemd past that point would fossilize Linux semantics into the native boot path (GAP-0170).

<!-- covers: GAP-0170 -->

#### Out of scope
How early boot locates the content store (BOOT-009). Pre-boot unlock UI (BOOT-026). systemd emulation inside the Linux personality (LNX-029).

#### Acceptance criteria
- [ ] Options evaluated include (A) native init from the first instruction after kernel handoff, (B) native init after root-store unlock and verity setup, and (C) native init after a systemd handoff.
- [ ] The accepted option names the Component that owns reboot and halt, and records that V0 retained initramfs is not a V0.5 deliverable (R-010).
- [ ] The Decision names native supervision versus systemd-in-LNX so the two service managers are not confused (R-072).
- [ ] Architecture review sign-off is recorded on the pull request.

#### Verification
- Review: architecture review recorded on the pull request, with BOOT and LNX leads named.
- Report: the Decision file lists at least two options and the rejected ones.

#### Evidence
- none

### SVC-004 · Decide how a service Component reports readiness and liveness to the supervisor
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: SVC-014
- Baseline: §32
- Decision: D-0301
- Risks: R-023

Dependency-ordered startup and rebind timing cannot be built until a service has a defined way to say it is ready and a defined way the supervisor notices it is dead. This Decision picks one readiness signal and one liveness signal for every supervised Component.

<!-- covers: EXTRA-071 -->

#### Out of scope
Restart budgets and backoff (SVC-005). Kernel death notification plumbing (CMP-004). Client rebind (IPC-028).

#### Acceptance criteria
- [ ] Readiness options evaluated include (A) explicit ready notification over a supervisor Channel, (B) interface-advertised-means-ready, and (C) probe-based readiness.
- [ ] Liveness options evaluated include (A) heartbeat over the supervisor Channel and (B) kernel death notification only.
- [ ] The accepted pair is expressible in the service manifest and does not require native software to use a POSIX pidfd or waitpid.
- [ ] Architecture review sign-off is recorded on the pull request.

#### Verification
- Review: architecture review recorded on the pull request.
- Unit: the Decision's selected protocol is cited by name in `runtime:tests/svc/readiness_contract_*` once SVC-008 lands.

#### Evidence
- none

### SVC-005 · Decide restart budgets, strategies, backoff and escalation for supervised services
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: SVC-014
- Baseline: §32
- Decision: D-0302
- Risks: R-023
- Invariants: I-037

The V0.5 compositor-restart gate and the V2 safe-mode escalation both need a single restart-policy model: per-service budget window, strategy, backoff bounds, and the escalation target when the budget is exhausted. This Decision is that model; S-020 records the fields the manifest will carry.

<!-- covers: EXTRA-071 -->

#### Out of scope
Readiness and liveness signaling (SVC-004). Safe-mode session contents (SVC-036). BEAM study (SVC-014).

#### Acceptance criteria
- [ ] Options evaluated include (A) BEAM-style one-for-one, rest-for-one and one-for-all strategies with per-service budgets, (B) systemd StartLimit-style windows, and (C) Fuchsia eager versus lazy restart.
- [ ] The accepted option records backoff bounds, the budget window, and the escalation target when the budget is exhausted.
- [ ] The V0.5 compositor-restart loop is expressible under the accepted budget without treating the loop as a policy violation.
- [ ] Architecture review sign-off is recorded on the pull request.

#### Verification
- Review: architecture review recorded on the pull request, with CMP lead named.
- Report: the Decision file lists at least two options and cites `reports/spikes/SVC-014.md`.

#### Evidence
- none

### SVC-006 · Decide the settings storage model: typed schema-versioned objects with history events
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: PKG-009
- Baseline: §31
- Decision: D-0303
- Invariants: I-020, I-062

Without a Decision, every V0.5 application invents a config file and §31 history cannot restore configuration. This Decision picks how settings are stored and how a change becomes a restorable history event. Registry-style global configuration is already forbidden (PKG INV-0535).

<!-- covers: GAP-0278 -->

#### Out of scope
The `os history` log (PKG-022). Settings UI (APP-040). Per-application client crate (SVC-028).

#### Acceptance criteria
- [ ] Options evaluated include (A) typed schema-versioned objects in a settings service, (B) per-application files in ApplicationData, and (C) a registry-style store.
- [ ] Option C is rejected in the Decision with a citation of I-020 and PKG INV-0535.
- [ ] The accepted option names how a change becomes a system history event restorable by `os restore`, and that an application reads only its own Capability-scoped settings.
- [ ] Architecture review sign-off is recorded on the pull request.

#### Verification
- Review: architecture review recorded on the pull request, with PKG lead named.
- Report: the Decision file lists at least two options.

#### Evidence
- none

### SVC-007 · Build the native init Component that starts the supervisor from a SystemGeneration
- Type: build
- Milestone: V0.5
- Status: todo
- Size: L
- Owner: none
- Depends on: SVC-003, BOOT-012, BOOT-009, PKG-021, PKG-019, CMP-027, CAP-023
- Baseline: §30, §32
- Risks: R-010

Native init replaces systemd for the native world per the early-userspace Decision. It mounts the content store of the selected SystemGeneration (already located by BOOT), seeds the root Capability set, starts the supervisor, and owns reboot and halt. V0.5 exit requires the compositor to start from a SystemGeneration; this Component is that start.

<!-- covers: GAP-0171 -->

#### Out of scope
Store locator and read-only mount (BOOT-012). Generation compose (PKG-016). Verity and disk unlock (SVC-023). Supervisor tree (SVC-015).

#### Acceptance criteria
- [ ] After BOOT hands off, init starts the supervisor from the pinned services of the selected SystemGeneration on `qemu-x86_64` and `hw-h002`.
- [ ] Reboot and halt Operations are accepted only from a Capability minted to init; a test Component holding no such Capability receives `Error::Rights` and the machine does not reboot.
- [ ] Selecting the previous SystemGeneration at the boot menu starts that generation's pinned compositor Package, verified by PKG-015.
- [ ] Init does not start Linux init or systemd for native services on the V0.5 image.
- [ ] `os inspect component` names init as the root of the supervision tree.

#### Verification
- Integration: `runtime:tests/svc/init_generation_start_*` on `qemu-x86_64` and `hw-h002`.
- Demo: V0.5 cold-boot-to-native-desktop on H-002.
- Review: BOOT lead confirms the handoff contract on the pull request.

#### Evidence
- none

### SVC-008 · Implement readiness and liveness signalling in the native service runtime
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: SVC-004, SDK-004, SVC-005
- Baseline: §32

Runtime-side implementation of the readiness Decision: a service reports ready over its supervisor Channel, dependents start only after readiness, and liveness loss triggers the restart policy. Shipped as a runtime module jointly with SDK; the supervisor side lives here.

<!-- covers: EXTRA-071, GAP-0171 -->

#### Out of scope
SDK packaging of the crate (SDK). Probe-versus-notification Decision (SVC-004). Restart policy (SVC-005).

#### Acceptance criteria
- [ ] A service that never sends ready leaves every declared dependent unstarted, visible in `os inspect service`.
- [ ] After ready, a declared dependent starts; the order is recorded in the inspect tree.
- [ ] Liveness loss on a ready service triggers the restart policy chosen by SVC-005 and does not start dependents of the replacement until the replacement is ready.
- [ ] Native services use the supervisor Channel, not a POSIX pidfd, to report ready.

#### Verification
- Unit: `runtime:tests/svc/readiness_*` on `qemu-x86_64`.
- Integration: `runtime:tests/svc/liveness_loss_restart_*` on `qemu-x86_64` and `hw-h002`.
- Fuzz: `runtime:fuzz/svc_readiness` one hour nightly without panic.

#### Evidence
- none

### SVC-009 · Re-advertise interfaces after restart so clients rebind by Interface identity
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: SVC-008, IPC-023, IPC-026, CMP-028, SVC-012
- Baseline: §32
- Invariants: I-037

On service death the supervisor restarts the Component and re-registers its Interfaces in the IPC discovery mechanism so SDK clients rebind and retry. This is the supervisor side of the disconnect, rebind, retry and restore-state contract (§32).

<!-- covers: INV-0592, EXTRA-071 -->

#### Out of scope
Generated client proxies (IPC-028, SDK-012). Component slot identity (CMP-028). Discovery mechanism Decision (IPC-023).

#### Acceptance criteria
- [ ] After a supervised service exits, its Interfaces are absent from discovery until the replacement is ready, then present under the same interface identity.
- [ ] A client holding a Channel to the dead instance observes peer-closed, re-resolves by interface identity, and completes a subsequent request on the new Channel.
- [ ] Restore-state is invoked once per rebind on Interfaces that declare it; an Interface that omits it fails the SVC-012.
- [ ] Native clients never rebind by a POSIX pid, path or D-Bus name.

#### Verification
- Integration: `runtime:tests/svc/readvertise_rebind_*` on `qemu-x86_64` and `hw-h002`.
- Unit: `runtime:tests/svc/discovery_absent_until_ready_*` on `qemu-x86_64`.
- Demo: V0.5 compositor-kill demo; Text Editor keeps its unsaved buffer.

#### Evidence
- none

### SVC-010 · Expose service control and supervision-tree state via `os service` and `os inspect`
- Type: build
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: SVC-015, OBS-019, SDK-007
- Baseline: §24, §32, §64

V0.5 fault injection and the compositor-restart gate need a controlled kill, restart and status path. OBS owns the inspect transport; SVC supplies supervision-tree data (state, restart counts, budget remaining) as an inspect provider and a Capability-gated `os service` control Interface. Required by V0.5-G03 (Compositor crash recovery rebinds every window).

#### Out of scope
`os` CLI binary (SDK-007). Inspect transport (OBS-019). Fault-injection framework (BLD-020).

#### Acceptance criteria
- [ ] `os inspect service <id>` prints state, restart count, budget remaining, declared dependencies and last exit cause on `qemu-x86_64`.
- [ ] `os service kill <id>` with a control Capability terminates the Component; without that Capability the call returns `Error::Rights` and the service stays running.
- [ ] `os service restart <id>` increments the restart count visible in inspect.

#### Verification
- Integration: `runtime:tests/svc/os_service_control_*` on `qemu-x86_64`.
- Unit: `runtime:tests/svc/inspect_provider_*` on `qemu-x86_64`.
- Review: OBS lead confirms the provider registers through the typed inspect Interface.

#### Evidence
- none

### SVC-011 · Define the service manifest: capabilities, dependencies, restart policy, readiness
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: SVC-005, SVC-004, PKG-011, PKG-031, IPC-035, PKG-027
- Baseline: §28, §32
- Invariants: I-020

Layer 2 schema declaring a system service as a Component with requested Capabilities, dependency edges, restart policy, readiness kind and ResourceDomain. Embedded in the Package manifest so no service is configured outside the immutable store. State is prototyped; versioning follows IPC Layer 2 rules. S-020 is the ABI surface.

<!-- covers: GAP-0171 -->

#### Out of scope
Package identity and Components section (PKG-031). Version lock (SVC-041). Supervisor that consumes the schema (SVC-015).

#### Acceptance criteria
- [ ] A service Package whose manifest omits restart policy or readiness kind fails PKG-027 and is not started.
- [ ] Dependency edges name other service Interfaces by identity; a cycle is rejected at compose time with a typed error and no Component is created.
- [ ] Requested Capabilities in the service manifest are the only Capabilities the supervisor grants at start; a test service that opens an undeclared object receives `Error::Rights`.
- [ ] The schema is IDL-defined and registered with IPC-035; native software never reads a systemd unit file.

#### Verification
- Unit: `runtime:tests/svc/manifest_schema_*` on `qemu-x86_64`.
- Integration: `sdk:tests/manifest/service_embed_*` on `qemu-x86_64`.
- Review: PKG lead confirms the embedding does not introduce registry-style global configuration.

#### Evidence
- none

### SVC-012 · Add a CI lint requiring restart/rebind on every service Interface
- Type: build
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: SVC-011, IPC-032
- Baseline: §32, §67
- Invariants: I-037

INV-1301 is Principle 10 as a standing invariant. Services do not exist until V0.5, so the lint lives here. CI fails when a supervised service's IDL or manifest omits disconnect, rebind, retry and restore-state plus a restart policy. First enforced on the V0.5 compositor, settings and supervisor Interfaces. IPC owns the client-side guideline.

<!-- covers: INV-1301 -->

#### Out of scope
Client-side IDL guidelines (IPC-032). Manifest schema fields (SVC-011).

#### Acceptance criteria
- [ ] A pull request that adds a supervised service Interface without disconnect, rebind, retry and restore-state fails CI.
- [ ] A pull request that adds a service manifest without a restart policy fails CI.
- [ ] The V0.5 compositor, settings and supervisor Interfaces pass the lint on `qemu-x86_64`.

#### Verification
- Unit: `runtime:tests/svc/rebind_lint_*` on `qemu-x86_64`.
- Integration: CI job `svc-rebind-lint` on the compositor, settings and supervisor IDL.
- Review: IPC lead confirms the lint matches the client-side guideline.

#### Evidence
- none

### SVC-013 · Build the settings service storing typed, schema-versioned settings objects
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: SVC-006, SVC-015, PKG-022
- Baseline: §31
- Threats: T-001
- Invariants: I-021, I-062

Implements the settings Decision: a supervised service holding per-system and per-application settings as typed objects with schema migration, change notification, and Capability-scoped access. An application reads only its own scope. Change events are emitted for PKG history to consume later.

<!-- covers: GAP-0278 -->

#### Out of scope
History restore path (SVC-029). Client crate (SVC-028). Settings application (APP-040).

#### Acceptance criteria
- [ ] A Component holding only its application settings Capability reads and writes that scope and receives `Error::Rights` on any other scope, allocating no additional handle.
- [ ] A schema-incompatible write is rejected with a typed error and the previous object remains; a migrated read returns the new schema version.
- [ ] A change notification is delivered to subscribers of that scope and not to other applications.
- [ ] The service restarts under the supervisor and subscribers rebind without losing the last committed object.

#### Verification
- Unit: `runtime:tests/svc/settings_scope_*` on `qemu-x86_64`.
- Integration: `runtime:tests/svc/settings_restart_rebind_*` on `qemu-x86_64` and `hw-h002`.
- Fuzz: `runtime:fuzz/svc_settings_schema` one hour nightly without panic.

#### Evidence
- none

### SVC-014 · Study BEAM and Fuchsia supervision to shape restart strategies
- Type: spike
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: none
- Baseline: §32
- Explores: S-020
- Risks: R-023

Time-boxed comparison of Erlang/BEAM supervision trees, Fuchsia component-manager restart policies and systemd restart units, aimed at the service tree rather than the V0 TaskGroup (CMP-030). The report recommends the strategy set, escalation and backoff shapes that SVC-005 then accepts or rejects. S-020 is the ABI surface those fields will occupy.

<!-- covers: INV-0610 -->

#### Out of scope
Component-side restart identity (CMP-030). XNU launchd study (DOC-005). The Decision itself (SVC-005).

#### Acceptance criteria
- [ ] `reports/spikes/SVC-014.md` exists with the spike skeleton headings.
- [ ] The report compares BEAM one-for-one, rest-for-one and one-for-all, Fuchsia eager and lazy restart, and systemd StartLimit windows, each with consequences for S-020.
- [ ] The report names a recommended strategy set, backoff shape and escalation target without encoding the Decision.

#### Verification
- Report: Which BEAM strategies map onto service trees versus Component graphs? How do Fuchsia eager/lazy restart and systemd StartLimit windows compare on budget exhaustion and thundering herd? What backoff shape lets the V0.5 compositor-restart gate run without being classified as a restart loop? What escalation target is appropriate when a budget is exhausted (parent, safe-mode session, reboot into the previous SystemGeneration)?
- Review: architecture review recorded on the pull request.

#### Evidence
- none

### SVC-015 · Build the service supervisor with dependency-ordered start and restart policies
- Type: build
- Milestone: V0.5
- Status: todo
- Size: L
- Owner: none
- Depends on: SVC-005, SVC-004, SVC-011, SVC-007, CMP-004, CMP-008, CMP-027, CAP-023
- Baseline: §32
- Risks: R-023
- Invariants: I-037

Supervision tree that instantiates services from manifests as Components with declared Capabilities, orders startup by dependencies and readiness, applies the decided restart policy, and records typed exit causes from CMP. This is the layer §32 requires and the V0.5 compositor-restart gate exercises.

<!-- covers: GAP-0171, INV-0592 -->

#### Out of scope
Init handoff (SVC-007). Readiness runtime helpers (SVC-008). Interface re-advertisement (SVC-009). Session trees (SVC-027).

#### Acceptance criteria
- [ ] Services start in dependency order; a service whose dependency never becomes ready stays unstarted and is visible in `os inspect service`.
- [ ] A service that exits with a typed panic, OOM or cancelled cause is restarted according to the accepted policy; the exit cause is recorded on the supervision node.
- [ ] Exhausting the restart budget does not loop: the supervisor stops the subtree and emits a typed escalation event.
- [ ] The supervisor grants only Capabilities listed in the service manifest; an extra Capability is not present in the Component's table.
- [ ] Native software never sees a systemd unit, D-Bus activation or POSIX fork/exec as the start mechanism.

#### Verification
- Integration: `runtime:tests/svc/supervisor_order_*` on `qemu-x86_64` and `hw-h002`.
- Unit: `runtime:tests/svc/supervisor_budget_*` on `qemu-x86_64`.
- Fuzz: `runtime:fuzz/svc_supervisor_manifest` one hour nightly without panic.
- Demo: V0.5 compositor-kill demo on H-002.

#### Evidence
- none

### SVC-016 · Decide monotonic versus wall-clock semantics for Operation deadlines across suspend
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: TSK-004, PWR-002
- Baseline: none
- Decision: D-0298

V1 introduces suspend and resume on the Intel laptop. TSK-004 already fixed ABI encoding; this Decision picks which clock Operation deadlines and timers use, whether suspended time counts, and the wall-clock step notification contract that TLS, package signature verification and the time service rely on. TSK-041 implements the chosen semantics.

<!-- covers: GAP-0288 -->

#### Out of scope
Deadline encoding in the Operation ABI (TSK-004). Suspend cycle harness (PWR-014). NTP client (SVC-018).

#### Acceptance criteria
- [ ] Options evaluated include (A) a CLOCK_MONOTONIC-like clock that does not advance during suspend, (B) a CLOCK_BOOTTIME-like clock that does, and (C) separate clocks for deadlines versus wall-clock with an explicit step notification.
- [ ] The accepted option states whether an Operation deadline that expires during suspend completes as `DeadlineExceeded` at resume or is extended by suspended time.
- [ ] The wall-clock step notification contract is named so subscribers observe a step without polling.
- [ ] Architecture review sign-off is recorded on the pull request, with TSK and PWR leads named.

#### Verification
- Review: architecture review recorded on the pull request.
- Report: the Decision file lists at least two options.

#### Evidence
- none

### SVC-017 · Decide the default-application registry and open-by-Capability model
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: SVC-013, APP-002
- Baseline: none
- Decision: D-0299
- Threats: T-002

V0.5 opens files only via the UserSelected chooser. Daily-driving needs a default handler for typed kinds and URL schemes that still mints a Capability to only that object. A shared path-based MIME database would reintroduce ambient filesystem authority (T-002).

<!-- covers: GAP-0280 -->

#### Out of scope
Package handler declarations (PKG-056). Default-apps Settings panel (APP-026). Linux xdg-open and portals (LNX).

#### Acceptance criteria
- [ ] Options evaluated include (A) user-chosen default per kind or scheme with Package handler candidates, (B) a shared MIME/xdg-data database, and (C) always-ask chooser with remembered last choice only.
- [ ] Option B is rejected in the Decision as path-based ambient authority.
- [ ] The accepted option states how STO typed kinds and URL schemes map to a handler Package, how user defaults override Package offers, and that open mints a Capability to only that object.
- [ ] Architecture review sign-off is recorded on the pull request.

#### Verification
- Review: architecture review recorded on the pull request, with STO and APP leads named.
- Report: the Decision file lists at least two options.

#### Evidence
- none

### SVC-018 · Decide whether to retain chrony or build a native NTP/NTS client
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: BOOT-021, SVC-015
- Baseline: none
- Decision: D-0304

TLS, package signature verification and Operation deadlines fail with a wrong clock. This Decision picks the time-sync client that the V1 time service hosts: a retained chrony as a supervised personality service, a native Rust NTS client Component, or a systemd-timesyncd-class minimal client. It must precede SVC-032 in the same rung.

<!-- covers: GAP-0288 -->

#### Out of scope
Early-boot clock floor (BOOT-021). Timezone auto-detection (SVC-037). Deadline clock (SVC-016).

#### Acceptance criteria
- [ ] Options evaluated include (A) chrony hosted as a supervised personality service, (B) a native Rust NTS client Component, and (C) a systemd-timesyncd-class minimal client.
- [ ] The accepted option names NTS versus NTP, how the RTC is synchronized, and that native software does not call a POSIX `ntp_adjtime`.
- [ ] Architecture review sign-off is recorded on the pull request.

#### Verification
- Review: architecture review recorded on the pull request, with SEC lead named for TLS implications.
- Report: the Decision file lists at least two options.

#### Evidence
- none

### SVC-019 · Build the default-application registry that opens files by Capability
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: SVC-017, PKG-056, SVC-013, SVC-015
- Baseline: none
- Threats: T-002
- Invariants: I-021, I-035

Implements the default-application Decision: a supervised service that resolves the user-chosen handler for a typed kind or URL scheme from Package handler declarations, persists defaults in the settings store, and launches the handler with a Capability to just that object. APP File Browser and launcher consume the Interface.

<!-- covers: GAP-0280 -->

#### Out of scope
Manifest handler declarations (PKG-056). Picker UI (APP-026). Linux xdg-open and portals (LNX).

#### Acceptance criteria
- [ ] Opening a typed object with a user default launches the declared handler Package with a Capability to only that object; a test that the handler enumerates the parent collection returns `Error::Rights`.
- [ ] Clearing the user default falls back to the Package offer or to the always-ask path named by the Decision, and does not consult a MIME database on disk.
- [ ] A handler Package that does not declare the kind is not launched; the registry returns a typed error and allocates no handle.
- [ ] Defaults persist in the settings store across supervisor restart.

#### Verification
- Integration: `runtime:tests/svc/default_app_open_*` on `qemu-x86_64` and `hw-h002`.
- Unit: `runtime:tests/svc/default_app_scope_*` on `qemu-x86_64`.
- Review: STO lead confirms the minted Capability matches UserSelected authority.

#### Evidence
- none

### SVC-020 · Emit typed degraded-recovery events when seamless restart is impossible
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: SVC-015, OBS-019
- Baseline: §32
- Invariants: I-037

Where hardware or driver constraints prevent seamless recovery, degraded recovery is still explicit (§32). The supervisor publishes typed events (service lost, degraded, recovered) into the OBS audit and log path and to subscribed shells. This is a prerequisite for V2 shell indicators and safe-mode escalation.

<!-- covers: INV-0607 -->

#### Out of scope
Shell indicators (APP-031). Safe-mode session (SVC-036). Crash-capture format (OBS-026).

#### Acceptance criteria
- [ ] Exhausting a service restart budget emits a `service-lost` event with the service identity and last exit cause, visible in `os inspect service` and the audit log.
- [ ] A subsequent successful ready after operator restart emits `recovered`; a start that cannot obtain a required device Capability emits `degraded` rather than looping.
- [ ] A shell Component holding a subscribe Capability receives the events; a Component without it receives `Error::Rights`.
- [ ] Events are typed IDL records, not POSIX syslog lines.

#### Verification
- Integration: `runtime:tests/svc/degraded_events_*` on `qemu-x86_64` and `hw-h002`.
- Unit: `runtime:tests/svc/degraded_rights_*` on `qemu-x86_64`.
- Review: OBS lead confirms the event schema matches the crash-capture format.

#### Evidence
- none

### SVC-021 · Write the guide for authoring a supervised service with rebind and restore-state
- Type: docs
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: SVC-011, SVC-009, SVC-005
- Baseline: §32

V1 SDK v1 and IDL guidelines need the service-side counterpart: how to declare a manifest, signal readiness, design disconnect, rebind, retry and restore-state per §32, and choose a restart policy. DOC publishes; SVC authors. Required by V3-G12 (Layer 1 ABI reference pages exist for every entry point).

#### Out of scope
Site generator (DOC-010). Client-side IDL guidelines (IPC-032). SDK crate layout (SDK).

#### Acceptance criteria
- [ ] A guide page exists covering manifest fields, readiness, restart policy, and the disconnect/rebind/retry/restore-state contract, citing §32.
- [ ] The page is linked from the V1 developer guide and contains no performance numbers.
- [ ] DOC and SVC review sign-off is recorded on the pull request.

#### Verification
- Review: DOC and SVC leads sign off on the pull request.
- Manual: a new contributor can follow the guide to declare a sample supervised service whose IDL passes SVC-012.

#### Evidence
- none

### SVC-022 · Host user-space driver Components under supervision with per-device restart
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: SVC-015, SVC-020, HW-029, HW-006, HW-017
- Baseline: §33
- Invariants: I-008, I-038

HW builds the driver framework and re-issues device Capabilities; SVC provides the driver host that launches driver Components from Packages, supervises each device instance, and escalates exhausted budgets to an explicit degraded-device state. Kernel residency remains acceptable where HW's classification Decision says the cost is not acceptable (I-008).

#### Out of scope
Object<Device> and DMA MemoryObjects (HW-029). Capability re-issue to holders (HW-030). Per-class residency (HW-016).

#### Acceptance criteria
- [ ] A USB HID driver Component that panics is restarted by the host; the device instance returns to ready or to `degraded` without a machine reboot on `hw-h004`.
- [ ] Exhausting that instance's restart budget emits a degraded-device event and does not restart the sibling device instance.
- [ ] The host grants only the Device Capability named in the driver Package; a driver that maps MemoryObjects outside its IOMMU domain is not started (I-038).
- [ ] Native software never opens a Linux device node to talk to the hosted driver.

#### Verification
- Integration: `runtime:tests/svc/driver_host_restart_*` on `hw-h004` and `qemu-x86_64`.
- Unit: `runtime:tests/svc/driver_host_budget_*` on `qemu-x86_64`.
- Review: HW lead confirms the host consumes Object<Device> rather than wrapping sysfs.

#### Evidence
- none

### SVC-023 · Build the early-boot init stage for verity setup and root-store unlock
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: SVC-003, SVC-007, BOOT-026, BOOT-009
- Baseline: §30
- Invariants: I-021, I-073

The early-userspace Decision names disk unlock and verity setup as early-boot work. V1 allows manually configured encryption; this stage runs before the content store is available, prompts for unlock through BOOT's text UI, sets up verity, and hands over to init-core.

<!-- covers: GAP-0171, GAP-0170 -->

#### Out of scope
Unlock UI (BOOT-026). FDE installer default (INS-007, SEC). Generation verity at boot (BOOT-032).

#### Acceptance criteria
- [ ] On an image with a locked root store, the stage prompts through BOOT's text unlock UI and does not start init-core until unlock succeeds on `hw-h002` and `hw-h004`.
- [ ] A wrong passphrase leaves the store locked, records a typed failure, and does not skip verity.
- [ ] After unlock, init-core starts from the selected SystemGeneration as in the V0.5 path.
- [ ] Native software after handoff holds no ambient passphrase or disk-key Capability.

#### Verification
- Integration: `runtime:tests/svc/early_unlock_*` on `qemu-x86_64` and `hw-h004`.
- Manual: unlock then handoff on H-004 with a manually configured encrypted store.
- Review: BOOT and SEC leads confirm the stage does not duplicate the unlock UI.

#### Evidence
- none

### SVC-024 · Build the hostname and machine-identity settings service
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: SVC-013, SVC-015
- Baseline: none
- Threats: T-001
- Invariants: I-078

V1 networking and V2 mDNS need a typed hostname, pretty-name and machine-id object. Stable hardware identifiers are not ambient (I-078): machine-id is Capability-gated. The Linux personality `/etc/hostname` view is LNX.

<!-- covers: GAP-0236 -->

#### Out of scope
mDNS (NET-024). Personality `/etc/hostname` (LNX). DHCP client (NET-015).

#### Acceptance criteria
- [ ] Native services read hostname and pretty-name through a typed Interface; a Component without the Capability receives `Error::Rights`.
- [ ] machine-id is a distinct right from hostname; holding hostname does not reveal machine-id (I-078).
- [ ] A hostname change persists in the settings store and is visible to subsequent subscribers after supervisor restart.
- [ ] Native software never reads `/etc/hostname` or `/etc/machine-id`.

#### Verification
- Unit: `runtime:tests/svc/hostname_rights_*` on `qemu-x86_64`.
- Integration: `runtime:tests/svc/hostname_persist_*` on `qemu-x86_64` and `hw-h002`.
- Review: NET lead confirms DHCP and mDNS can consume the Interface.

#### Evidence
- none

### SVC-025 · Build the system locale, keyboard-layout and timezone settings service
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: SVC-013, TXT-016
- Baseline: none

TXT decides locale data (ICU/CLDR) at V1; V2 gates a localization framework. This service holds the system-wide typed locale, keyboard-layout and timezone settings and their change notifications. Personality projections such as `LANG` and `/etc/localtime` stay in LNX. Keyboard layout data is HW-027. Required by V4-G11 (Localization and CJK input methods).

#### Out of scope
Locale data source (TXT-016). Typed Locale object in the SDK (TXT-031). XKB layout tables (HW-027). Personality environment variables (LNX).

#### Acceptance criteria
- [ ] A Component with the locale settings Capability reads and writes system locale, keyboard layout and timezone; without it, writes return `Error::Rights`.
- [ ] A change notification is delivered to subscribers; the settings store retains the last committed object across restart.
- [ ] Timezone is a setting with a user override; auto-detection is absent until SVC-037.
- [ ] Native software never reads `/etc/localtime` or `LANG`.

#### Verification
- Unit: `runtime:tests/svc/locale_settings_*` on `qemu-x86_64`.
- Integration: `runtime:tests/svc/locale_persist_*` on `qemu-x86_64` and `hw-h002`.
- Review: TXT lead confirms the object shape does not pre-empt TXT-031.

#### Evidence
- none

### SVC-026 · Supervise Linux-Personality daemons as services with the same restart semantics
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: SVC-015, SVC-008, SVC-020, LNX-059
- Baseline: §32, §46
- Risks: R-072
- Invariants: I-006, I-037

V1 ships the Linux personality as a product with D-Bus, PipeWire, XWayland and portals. LNX decides the systemd-emulation level; SVC provides the adapter that runs a personality process as a supervised service with readiness, restart budgets and degraded events so I-037 holds for them too. Native software still never sees POSIX. Required by V1-G06 (Native audio and PipeWire compatibility coexist).

#### Out of scope
systemd surface inside the personality (LNX-029). PipeWire server (AUD-001). D-Bus and portals (LNX).

#### Acceptance criteria
- [ ] A declared personality daemon is instantiated as a supervised service with a restart policy from the service manifest.
- [ ] Killing that daemon restarts it under the same budget rules as a native service; exhausting the budget emits `degraded` rather than a native panic.
- [ ] Native Components cannot obtain a POSIX pid or unit name for the daemon; they see only the typed Interface the adapter exports, if any.
- [ ] The adapter is documented as the native supervisor, not a second systemd (R-072).

#### Verification
- Integration: `runtime:tests/svc/personality_host_*` on `qemu-x86_64`.
- Review: LNX lead confirms the adapter does not replace LNX-029.
- Manual: D-Bus and PipeWire personality daemons restart under `os inspect service` on H-002.

#### Evidence
- none

### SVC-027 · Build per-user session supervision trees under the system supervisor
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: SVC-015, SEC-028, SEC-020
- Baseline: §32, §9

V1 introduces single-user login and session lock. Session-scoped services (shell, panel, agents) must die with the session and restart independently of system services. This task separates system and session supervision scopes; V2 APP session management consumes the session tree. Required by V3-G11 (Multi-user sessions isolate capability stores).

#### Out of scope
Session object, lock and identity (SEC-028). Greeter and lock UI (APP). Multi-user switching (SVC-039).

#### Acceptance criteria
- [ ] Ending a Session tears down the session supervision tree and leaves system services running, verified by `os inspect service`.
- [ ] Restarting a session-scoped service does not restart a system service, and the reverse.
- [ ] A session service is granted only Capabilities derived from that Session's root; a test that it uses a system-only Capability returns `Error::Rights`.
- [ ] Login through SEC-020 creates the session tree before the thin panel starts.

#### Verification
- Integration: `runtime:tests/svc/session_tree_*` on `qemu-x86_64` and `hw-h004`.
- Unit: `runtime:tests/svc/session_cap_scope_*` on `qemu-x86_64`.
- Review: SEC lead confirms the tree is rooted at Object<Session>.

#### Evidence
- none

### SVC-028 · Provide the per-application settings Interface and client crate for the SDK
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: SVC-013
- Baseline: §31

IDL Interface and Rust client for typed, schema-versioned per-application settings with change subscriptions, shipped in SDK v1 so native apps stop inventing config files. SDK owns packaging; SVC owns the Interface and service side.

<!-- covers: GAP-0278 -->

#### Out of scope
SDK crate publish (SDK-018). Settings store (SVC-013). Settings UI (APP-040).

#### Acceptance criteria
- [ ] The IDL Interface supports get, set, migrate and subscribe for a schema-versioned object, generated into a Rust client crate.
- [ ] A sample application using the crate persists a setting across restart and receives a change notification from another holder of the same scope.
- [ ] A client without the settings Capability receives `Error::Rights` and writes no file into ApplicationData as a fallback.
- [ ] The crate is listed in SDK v1 and has no POSIX `open` of a config path on the native path.

#### Verification
- Unit: `sdk:tests/settings_client_*` on `qemu-x86_64`.
- Integration: `runtime:tests/svc/settings_client_roundtrip_*` on `qemu-x86_64`.
- Review: SDK lead confirms crate layout under Layer 3 semver.

#### Evidence
- none

### SVC-029 · Record settings changes as system history events and restore them via `os restore`
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: SVC-013, PKG-022, PKG-059, PKG-060, PKG-048
- Baseline: §31
- Invariants: I-022

V1 exit: `os restore` restores kernel, packages and system configuration. PKG owns `os history` v1; SVC feeds settings change events and implements the configuration restore path from a history point.

<!-- covers: GAP-0278 -->

#### Out of scope
Generation restore (PKG-060). History log durability (PKG-022). Application-state restore (PKG-069).

#### Acceptance criteria
- [ ] A settings write appears as a typed history event listed by `os history`.
- [ ] `os restore` to that event restores the settings object and not the running SystemGeneration kernel; a subsequent read returns the restored value.
- [ ] Restoring a SystemGeneration does not clobber user settings that live outside the generation boundary named by PKG-007.
- [ ] A Component without restore authority receives `Error::Rights` and settings are unchanged.

#### Verification
- Integration: `runtime:tests/svc/settings_restore_*` on `qemu-x86_64` and `hw-h002`.
- Unit: `runtime:tests/svc/settings_history_event_*` on `qemu-x86_64`.
- Review: PKG lead confirms event types are in the history catalog.

#### Evidence
- none

### SVC-030 · Implement ordered shutdown, reboot and Generation-switch sequencing in init
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: SVC-007, SVC-015, PWR-013, PWR-012, BOOT-020, PKG-020
- Baseline: §30, §32

V1 exit requires reboot into a new generation and `os restore`. Init stops services in reverse dependency order with bounded timeouts, honors InhibitSuspend and InhibitIdle Capabilities, flushes storage and hands off to the BOOT generation switch. APP session inhibitors at V2 extend this path; they are not required here. Required by V1-G11 (os restore returns to a previous generation).

#### Out of scope
Power Component Operations (PWR-013). Boot-success definition (BOOT-020). Logout UI and V2 inhibitors (APP-039).

#### Acceptance criteria
- [ ] Reboot stops session services then system services in reverse dependency order; a service that misses its stop timeout is killed and the sequence continues.
- [ ] A holder of InhibitSuspend delays suspend until the Capability is dropped or the deadline named by PWR expires; shutdown still proceeds.
- [ ] Generation-switch reboot leaves the next-boot default set by `os generation switch` and does not start the outgoing generation's services after handoff.
- [ ] Native software never calls POSIX `reboot(2)` or `logind`.

#### Verification
- Integration: `runtime:tests/svc/shutdown_order_*` on `qemu-x86_64` and `hw-h002`.
- Integration: `runtime:tests/svc/generation_switch_reboot_*` on `qemu-x86_64`.
- Review: PWR lead confirms Inhibit Capabilities are honored.

#### Evidence
- none

### SVC-031 · Build the structured log collection service for system and session services
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: SVC-015, SVC-027, OBS-026
- Baseline: §24, §32
- Invariants: I-077

Collects structured log records from supervised services with per-service retention and Capability-gated read, in the OBS crash-capture and trace format. Required by V1 daily-driving debugging and by the V2 safe-mode session's log export.

<!-- covers: GAP-0290 -->

#### Out of scope
Crash-capture format (OBS-026). Crash-report client (INS-020). Safe-mode export UI (APP-037).

#### Acceptance criteria
- [ ] A supervised service's structured records are readable by a Component holding that service's log Capability and denied with `Error::Rights` otherwise.
- [ ] Session logs are not readable under a different Session's Capability.
- [ ] Records use the OBS crash-capture schema and contain no disk keys or unlocked secrets (I-077), verified by a redaction test.
- [ ] The log service restarts under the supervisor and does not lose records already committed to its store.

#### Verification
- Integration: `runtime:tests/svc/system_log_scope_*` on `qemu-x86_64` and `hw-h002`.
- Unit: `runtime:tests/svc/system_log_redact_*` on `qemu-x86_64`.
- Review: OBS lead confirms schema compatibility.

#### Evidence
- none

### SVC-032 · Build the time service: NTP/NTS sync, RTC synchronisation, clock-step events
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: SVC-018, SVC-016, SVC-015, BOOT-021
- Baseline: none

Supervised service implementing the time-sync Decision: sync the RTC, expose a Capability-gated set-time Interface, and broadcast clock-step events so TLS, package signature verification and deadlines observe steps. Timezone auto-detection is deferred to V2 with the SEC location service.

<!-- covers: GAP-0288 -->

#### Out of scope
Timezone auto-detection (SVC-037). Deadline clock implementation (TSK-041). Early-boot time floor (BOOT-021).

#### Acceptance criteria
- [ ] After network is up, the service synchronizes wall-clock per the accepted client and writes the RTC, verified on `hw-h002` and `hw-h004`.
- [ ] A clock step broadcasts a typed event; a subscriber observes the step without polling.
- [ ] set-time returns `Error::Rights` without the set-time Capability and does not change the clock.
- [ ] Native software never calls POSIX `ntp_adjtime` or `clock_settime` on the native path.

#### Verification
- Integration: `runtime:tests/svc/time_sync_*` on `qemu-x86_64` and `hw-h004`.
- Unit: `runtime:tests/svc/time_step_event_*` on `qemu-x86_64`.
- Review: SEC lead confirms TLS and signature verification subscribe to step events.

#### Evidence
- none

### SVC-033 · Wire kernel watchdog and hung-Task signals into supervisor escalation
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: SVC-007, SVC-015, KRN-041, BOOT-018
- Baseline: §32

KRN retains hardware watchdog, softlockup and hung-task detection and emits typed events. SVC pets the hardware watchdog from init, treats hung-task reports as liveness loss, and escalates repeated failures to reboot into the previous SystemGeneration via the boot counter.

#### Out of scope
Kernel detectors (KRN-041). Boot counter and last-known-good (BOOT-018). Crash-capture (OBS-027).

#### Acceptance criteria
- [ ] Init pets the hardware watchdog; stopping init without a clean shutdown causes a watchdog reboot on `hw-h002`.
- [ ] A hung-task report for a supervised service is treated as liveness loss and consumes restart budget.
- [ ] Repeated watchdog or hung-task escalations set the boot-success Capability so BOOT marks the generation bad and the next boot selects the previous SystemGeneration.
- [ ] Native software never opens `/dev/watchdog`.

#### Verification
- Integration: `runtime:tests/svc/watchdog_pet_*` on `hw-h002`.
- Integration: `runtime:tests/svc/hung_task_escalate_*` on `qemu-x86_64`.
- Review: KRN and BOOT leads confirm the event and boot-success contracts.

#### Evidence
- none

### SVC-034 · Run the §32 restartable-service matrix on every target machine
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: SVC-015, SVC-020, SVC-002, AUD-005, NET-015, HW-038, HW-030, GFX-083, BLD-020, SVC-022
- Baseline: §32
- Invariants: I-037

§32 lists compositor, audio, network, Bluetooth, printing, device-management and GPU userspace services as restartable. V2 gates Bluetooth stack crash recovery without reboot. This task registers each in-rung service in a hardware CI matrix that kills it and verifies rebind or explicit degraded recovery. Printing joins at V3 (HW-072).

#### Out of scope
Per-service rebind implementations (AUD, NET, HW, GFX). Printing (HW-072). Chaos plumbing (BLD-020).

#### Acceptance criteria
- [ ] The matrix kills compositor, audio, network, Bluetooth, device-management and GPU userspace services, each in isolation, on H-002, H-004 and H-005.
- [ ] Each kill either rebinds clients without session exit or emits a typed `degraded` event; a silent failure fails the job.
- [ ] Bluetooth stack crash recovers without reboot on H-004 and H-005, matching the V2 gate.
- [ ] Results are recorded per machine and per service; printing is listed as not-in-rung.

#### Verification
- Integration: `runtime:tests/svc/restart_matrix_*` on `hw-h002`, `hw-h004` and `hw-h005`.
- Bench: B-024 on H-002, H-004 and H-005; target per register.
- Review: AUD, NET, HW and GFX leads confirm their rebind paths are the ones the matrix kills.

#### Evidence
- none

### SVC-035 · Escalate exhausted shell or compositor restart budgets to the safe-mode session
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: SVC-005, SVC-027, SVC-020
- Baseline: §32
- Invariants: I-037

When the shell or compositor exhausts its restart budget the session tree is torn down and the safe-mode session starts instead of a restart loop. Verified on the three V2 target machines.

<!-- covers: EXTRA-016 -->

#### Out of scope
Safe-mode session contents (SVC-036). Restart policy Decision (SVC-005). Lock-restart-locked (GFX-045).

#### Acceptance criteria
- [ ] Exhausting the compositor restart budget tears down the session tree and starts the safe-mode session on H-002, H-004 and H-005.
- [ ] Exhausting the shell restart budget does the same and does not reboot the machine.
- [ ] System services outside the session tree stay running.
- [ ] A restart loop is not observed: the supervisor does not start the failed compositor again after escalation.

#### Verification
- Integration: `runtime:tests/svc/safe_mode_escalate_*` on `hw-h002`, `hw-h004` and `hw-h005`.
- Demo: V2 fault-injected compositor-budget exhaustion on H-002.
- Review: APP lead confirms the recovery UI is the one started.

#### Evidence
- none

### SVC-036 · Build the safe-mode session with settings, log export and `os restore`
- Type: build
- Milestone: V2
- Status: todo
- Size: L
- Owner: none
- Depends on: SVC-015, SVC-031, SVC-013, SVC-029, PKG-060
- Baseline: §32, §31

Minimal recovery session running on a minimal compositor configuration, offering settings reset, log-bundle export via the system log service, and `os restore` to a previous generation or settings point. APP owns the recovery UI; SVC owns the session tree, granted Capabilities and the compositor configuration it runs on.

<!-- covers: EXTRA-016 -->

#### Out of scope
Recovery UI chrome (APP-037). Separately booted recovery environment (INS-041). Text console last resort (SVC-040).

#### Acceptance criteria
- [ ] The safe-mode session starts with a minimal compositor configuration and the recovery UI, without the normal shell tree.
- [ ] Settings reset writes through the settings service and is visible after the next login.
- [ ] Log export produces a bundle from SVC-031 with I-077 redaction applied.
- [ ] `os restore` to a previous SystemGeneration or settings point is invocable from this session with the restore Capability and denied without it.
- [ ] The session runs on H-002, H-004 and H-005.

#### Verification
- Integration: `runtime:tests/svc/safe_mode_session_*` on `hw-h002`, `hw-h004` and `hw-h005`.
- Demo: V2 safe-mode restore on H-002.
- Review: APP and PKG leads confirm UI and restore contracts.

#### Evidence
- none

### SVC-037 · Add timezone auto-detection via the location Capability and the time settings Interface
- Type: build
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: SVC-032, SVC-025, SEC-041
- Baseline: none
- Threats: T-001
- Invariants: I-021

Completes GAP-0288 once SEC ships the coarse-location Capability at V2: the time service derives timezone from it with user override, and exposes the typed date/time settings Interface the APP Settings date/time panel binds to.

<!-- covers: GAP-0288 -->

#### Out of scope
Coarse-location Capability (SEC-041). Date/time Settings panel (APP-040). Locale data (TXT).

#### Acceptance criteria
- [ ] Holding coarse-location allows the time service to propose a timezone; without that Capability no location lookup occurs and the user override remains.
- [ ] A user override in locale settings wins over auto-detection and persists across restart.
- [ ] The date/time settings Interface exposes timezone, 24-hour preference and NTP enablement to APP without granting location.
- [ ] Precise location is never requested by this service.

#### Verification
- Integration: `runtime:tests/svc/timezone_autodetect_*` on `hw-h004` and `hw-h005`.
- Unit: `runtime:tests/svc/timezone_override_*` on `qemu-x86_64`.
- Review: SEC lead confirms only the coarse-location Capability is requested.

#### Evidence
- none

### SVC-038 · Benchmark the init-to-session service startup timeline on Tier 1 machines
- Type: benchmark
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: SVC-007, SVC-027, SVC-008, Q-001
- Baseline: §32, §34
- Benchmarks: B-032

The V3 boot-time-to-login gate publishes B-032 per Tier 1 machine beside mainline Linux. This harness publishes the init and service critical path (dependency waits, readiness latency per service) as the SVC-attributable share of that number, so tuning is measured rather than claimed.

#### Out of scope
Firmware-to-greeter publication (BEN-048, BOOT-016). Unlock-to-desktop (B-033).

#### Acceptance criteria
- [ ] Harness records per-service ready timestamps from init handoff to session-tree ready on every V3 Tier 1 machine in hardware scope.
- [ ] A report exists under `reports/benchmarks/B-032/` that includes the SVC critical-path series and states no superiority claim.
- [ ] The series names match the register method for B-032.

#### Verification
- Bench: B-032 on H-002, H-004, H-005, H-006, H-007 and H-008; target per register.
- Review: BEN lead confirms the SVC series does not duplicate the firmware-to-greeter clock.

#### Evidence
- none

### SVC-039 · Support concurrent per-user session trees with switching that preserves services
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: SVC-027, SEC-064, APP-063, SVC-013
- Baseline: §63

V3 exit: two users with separate sessions and separate Capability stores; switching preserves state. Extends the V1 session tree to concurrent sessions with per-user settings scopes. APP owns greeter chrome; SEC owns session objects.

#### Out of scope
Greeter chrome (APP-063). Session object switching (SEC-064). Cross-user Component isolation (CMP-051).

#### Acceptance criteria
- [ ] Two Sessions run concurrent supervision trees; `os inspect service` shows both, and a service in one tree cannot hold a Capability from the other (`Error::Rights`).
- [ ] Switching away from a Session leaves its tree running; switching back presents the same ready services without restart.
- [ ] Per-user settings scopes do not leak: user A's locale read under user B's Capability returns `Error::Rights`.
- [ ] System services remain a single tree shared by both Sessions.

#### Verification
- Integration: `runtime:tests/svc/multiuser_trees_*` on `hw-h002` and `hw-h004`.
- Unit: `runtime:tests/svc/multiuser_settings_scope_*` on `qemu-x86_64`.
- Demo: V3 two-user switch on H-002.

#### Evidence
- none

### SVC-040 · Fall back to a text recovery console offering previous-Generation boot when safe mode fails
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: SVC-007, SVC-035, INS-041, INS-013
- Baseline: §30, §63

V3 gate: a recovery environment restores a system whose current generation is corrupted, and automatic rollback on a failing generation. INS owns the separately booted recovery environment; SVC owns init's last-resort console when no compositor can start, offering rollback and handoff to that environment.

#### Out of scope
Recovery environment (INS-041). Boot menu (BOOT-014). Safe-mode graphical session (SVC-036).

#### Acceptance criteria
- [ ] If the safe-mode session cannot start a compositor, init presents a text console offering `boot previous generation` and `handoff to recovery environment`.
- [ ] Selecting previous generation reboots into the last-known-good SystemGeneration via BOOT.
- [ ] Selecting recovery handoff boots INS-013 without starting the failed compositor.
- [ ] The console runs on every V3 Tier 1 machine in hardware scope.

#### Verification
- Integration: `runtime:tests/svc/recovery_console_*` on `qemu-x86_64` and `hw-h002`.
- Manual: fault-injected compositor Package absence on H-004 reaches the console and boots the previous generation.
- Review: INS and BOOT leads confirm handoff versus boot-menu ownership.

#### Evidence
- none

### SVC-041 · Lock the service manifest and settings schema versions for 1.x
- Type: build
- Milestone: V4
- Status: todo
- Size: S
- Owner: none
- Depends on: SVC-011, SVC-028, SVC-010, IPC-042
- Baseline: §66
- Freezes: S-020

V4 gate: Layer 2 interface versions for 1.x are enumerated and locked with old-client/new-service tests. The service manifest, settings object schema and supervisor control Interface are SVC-owned Layer 2 surfaces. S-020 is prototyped until the spike, Decision and freeze closure complete; this task enumerates and locks the versions that closure names.

#### Out of scope
Layer 2 evolution rules (IPC-042). Package manifest lock (PKG-090). Layer 1 freeze (ABI).

#### Acceptance criteria
- [ ] Service manifest, settings object schema and supervisor control Interface versions for 1.x are listed and tagged as locked.
- [ ] Old-client/new-service and new-client/old-service tests pass for each listed Interface on `qemu-x86_64`.
- [ ] A pull request that changes a locked field without a version bump fails CI.

#### Verification
- Integration: `runtime:tests/svc/l2_lock_evolution_*` on `qemu-x86_64`.
- Review: IPC lead confirms the listed versions match the Layer 2 lock catalog.
- Unit: CI job `svc-l2-lock` rejects an unversioned field add.

#### Evidence
- none

### SVC-042 · Ship the supervision conformance suite run on every Tier 1 machine per RC
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: SVC-034, SVC-035, SVC-033, HW-086, HW-072
- Baseline: §32, §62
- Invariants: I-037

V4 gate: every Tier 1 machine passes the full hardware test suite each RC with zero P0/P1. This task turns the restartable-service matrix (now including printing), safe-mode escalation and watchdog escalation into a conformance suite with published per-machine results, exercised in both RC cycles.

#### Out of scope
Hardware suite orchestration (HW-086). Per-service rebind implementations (AUD, NET, HW, GFX).

#### Acceptance criteria
- [ ] The suite runs the restart matrix (including printing), safe-mode escalation and watchdog escalation on every V4 Tier 1 machine each RC.
- [ ] Per-machine results are published with the RC; a P0 or P1 supervision failure fails the RC.
- [ ] Printing kill preserves queued jobs or emits `degraded`, matching HW-072.

#### Verification
- Integration: `runtime:tests/svc/conformance_suite_*` on every V4 Tier 1 hardware-scope entry.
- Review: HW lead confirms the suite is invoked from the hardware RC job.
- Demo: one RC report attached as evidence when the task is done.

#### Evidence
- none

### SVC-043 · Verify by fault injection that rollback and `os restore` preserve and restore settings
- Type: build
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: SVC-029, BOOT-049, PKG-060
- Baseline: §31
- Invariants: I-022

1.0 definition (3): `os history` and `os restore` cover OS, packages and configuration, and rollback never loses user data. This task adds the configuration leg to the INS/PKG/BOOT fault-injection run on every Tier 1 machine, with results published in the release notes.

#### Out of scope
Generation rollback (BOOT-049). Package-set restore (PKG-077). User-data snapshots (STO).

#### Acceptance criteria
- [ ] Fault-injected failing generation rollback on every 1.0 Tier 1 machine leaves user and system settings identical to the pre-update values.
- [ ] `os restore` to a recorded settings history point restores those objects on every 1.0 Tier 1 machine.
- [ ] Release notes cite this run and contain no settings-loss P0.

#### Verification
- Integration: `runtime:tests/svc/settings_restore_guarantee_*` on every 1.0 Tier 1 hardware-scope entry.
- Review: INS and PKG leads confirm the configuration leg is in the same fault-injection run as generation rollback.
- Manual: release-notes checklist includes the settings-restore result.

#### Evidence
- none
