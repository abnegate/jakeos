# VIRT · Fallback virtualisation
- Prefix: VIRT
- Lead: none
- Baseline: §5.1, §7, §9.1, §10, §23, §27, §30, §36, §40, §43, §49, §54, §56.1, §56.2, §57, §69
- Baseline gap: The baseline retains KVM and names VM as a Channel transport but specifies no VM manager product, guest tools, JakeOS guest images or personality-failure fallback.

<!-- roadmap:generated:begin summary -->
Tasks: 20 live, 0 done, 0 in-progress, 20 todo, 0 dropped. Ready: 0. Blocked: 20. Weighted: 0%.
<!-- roadmap:generated:end -->

## Scope

VIRT owns the fallback virtualization product that lets a user commit to JakeOS as their only OS when a Personality cannot run a title: a KVM-based VM manager as a capability-scoped Component with typed VirtualMachine objects, inspectable host-access grants, Linux and Windows guest tools, guest-window integration, guest GPU acceleration, OVMF and TPM emulation for guests, VM disk snapshots on the content-addressed store, the compatibility-triage offer that hands a failed ELF or PE to a granted VM, and the optional attach of a dual-boot Windows volume as a guest disk. It also owns JakeOS-as-guest images so V1 developers can onboard on QEMU/KVM, VirtualBox, VMware, Hyper-V and UTM.

Each VM is a Component in a ResourceDomain. Folders, devices, network and clipboard reach a guest only as Capabilities visible in `os inspect` (I-081). Native software never sees POSIX, Win32, `/dev/kvm` or a libvirt descriptor. Release artifacts never contain Windows images or license keys. Kernel-level anti-cheat bypass is out; the VM fallback is the honest answer (I-071, §56.2).

## Out of scope

Kernel KVM retention, `Capability<VirtualMachine>` kernel object and nested-virt CI (KRN). Virtio Channel transport from a native guest Component to a host service (IPC). Compositor remote Surfaces for guest virtio-gpu (GFX). Clipboard, drag/drop and chooser protocol (UIP). Consent chrome, notifications, launcher, taskbar and store (APP). Dual-boot partition layout, BitLocker warnings and the SystemGeneration image builder (INS). NTFS and content-addressed store primitives (STO). Public compatibility database and explanation text (REL). Personality PE failure reasons (WIN). Wine, Proton and the no-obvious-VM ordinary-case nongoal (WIN). IOMMU, VFIO and NVIDIA bring-up (HW). Host TPM service (SEC). Signing, channels and HCL publication (REL, BLD). Docs site pipeline (DOC). Benchmark methodology (BEN). Lab racking (LAB). Network Capability broker (NET). ResourceDomain kernel object (SCH). Inspect transport (OBS). Wine and redistributable licensing (GOV). Native development environments, which do not require a VM (ENV, I-043).

## Tasks

### VIRT-001 · Publish JakeOS guest images for common hypervisors
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: BLD-024, INS-004, INS-001, KRN-036, REL-005, REL-007
- Baseline: none

V1 developers onboard without bare metal by running JakeOS as a guest on the hypervisor they already have. This task publishes signed qcow2, vmdk, vhd and UTM artifacts built from the SystemGeneration image builder, with virtio drivers, a guest agent Component and dynamic resolution so those guests are usable (§5.1). Native software inside the guest still sees the Native ABI, not a hypervisor POSIX.

<!-- covers: EXTRA-055, GAP-0449 -->

#### Out of scope
Host VM manager (VIRT-008). Image builder internals (INS-001). Channel signing policy (REL-003). Kernel nested-virt matrix contents (KRN-036).

#### Acceptance criteria
- [ ] Signed qcow2, vmdk, vhd and UTM artifacts for QEMU/KVM, VirtualBox, VMware, Hyper-V and UTM exist on the V1 developer channel.
- [ ] A JakeOS guest booted from the QEMU/KVM artifact on H-001 and H-015 reaches a native session with virtio block, virtio-net, guest agent and dynamic resolution; `os inspect` names the guest agent Component.
- [ ] A tampered image is rejected against the V1 developer trust root and is not activated.
- [ ] CI boots the QEMU/KVM artifact on H-015 and runs the V0 and V0.5 smoke suite inside the guest.

#### Verification
- Integration: `runtime:tests/virt/guest_image_boot_*` on `qemu-x86_64` and H-015.
- Review: REL lead confirms artifacts are signed once and promoted without rebuild.
- Demo: V1 developer onboarding boots the QEMU artifact on H-001 to a native session.

#### Evidence
- none

### VIRT-002 · Decide the fallback virtualization product and 1.0 scope
- Type: adr
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: KRN-025, WIN-002, WIN-001
- Baseline: §5.1, §56.2, §69
- Decision: D-0330
- Risks: R-036
- Threats: T-036
- Invariants: I-071, I-081

Personalities never cover every title, and kernel-level anti-cheat remains a non-goal (I-071, R-036). This Decision is the baseline-gap scope for VIRT: whether JakeOS ships a host VM product, what 1.0 includes, and how that product sits beside `Capability<VirtualMachine>` owned by KRN-025. WIN-001 still forbids an obvious VM for ordinary software; this Decision covers only software a Personality cannot run.

<!-- covers: GAP-0443, GAP-0510 -->

#### Out of scope
Kernel exposure of KVM as `Capability<VirtualMachine>` (KRN-025). Anti-cheat policy options (WIN-002). Guest-window depth (VIRT-003).

#### Acceptance criteria
- [ ] Options evaluated include (A) a native KVM manager as a capability-scoped Component, (B) libvirt or virt-manager as a Linux-personality application, (C) qemu launched with Capability-wrapped descriptors, and (D) no host VM product.
- [ ] The accepted option names 1.0 scope for manager, guest tools, triage offer, guest GPU, disk snapshots and physical-partition attach, and records consequences for I-081 and T-036.
- [ ] The accepted option does not relax WIN-002 or I-071, and does not move kernel VirtualMachine ownership out of KRN.
- [ ] Architecture review sign-off is recorded on the pull request.

#### Verification
- Review: architecture review recorded on the pull request, with KRN and WIN leads named.
- Manual: the Decision file lists at least two options and the rejected ones with reasons.

#### Evidence
- none

### VIRT-003 · Decide guest-window integration depth and agent protocol
- Type: adr
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: VIRT-002
- Baseline: §40, §49
- Decision: D-0331
- Invariants: I-048

Seamless mode fixes the guest-agent protocol that V2 guest tools ship against, so this Decision lands with those tools rather than after them (§49). GFX-092 remains the compositor remote-Surface work. Native software still does not see Wayland or X11 (I-048).

<!-- covers: GAP-0445 -->

#### Out of scope
Compositor remote Surfaces (GFX-092). Guest-tool binaries (VIRT-006, VIRT-009). Clipboard and drag/drop protocol (UIP).

#### Acceptance criteria
- [ ] Options evaluated include (A) one virtio-gpu Surface per VM, (B) per-application guest windows as native Surfaces with clipboard, drag/drop, chooser and notification bridging, and (C) single-display default with opt-in seamless.
- [ ] The accepted option names the guest-agent protocol messages Linux and Windows guest tools implement at V2.
- [ ] The accepted option records that GFX-092 owns compositor remote-Surface presentation.
- [ ] Architecture review sign-off is recorded on the pull request.

#### Verification
- Review: architecture review recorded on the pull request, with GFX and UIP leads named.
- Manual: the Decision file lists at least two options and the rejected ones with reasons.

#### Evidence
- none

### VIRT-004 · Provide OVMF firmware and TPM emulation for Windows guests
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: VIRT-002, VIRT-008
- Baseline: §5.1
- Invariants: I-070

Windows guests the personalities cannot run need UEFI firmware and TPM 2.0 inside the VM, which is guest firmware in the manager, not the host boot path. BLD OVMF/TPM is the CI matrix for hosting JakeOS. The tree and release artifacts still contain no Windows image or license key (I-070).

Required by the VIRT scope: "OVMF and TPM emulation for guests".

#### Out of scope
Host UEFI and measured boot (BOOT). Host TPM service (SEC-053). CI OVMF matrix for JakeOS-as-host (BLD-012). Windows guest tools (VIRT-009).

#### Acceptance criteria
- [ ] The VM manager starts a guest with OVMF and an emulated TPM 2.0 device; `os inspect` names firmware and TPM as Capabilities on the VirtualMachine Component.
- [ ] A Linux UEFI guest on H-015 reaches a serial prompt under that firmware path.
- [ ] A user-supplied Windows ISO selected through UserSelected boots under the same firmware path; CI and release artifacts contain no Windows ISO or license key.
- [ ] Native software never opens a POSIX firmware or TPM device node; those paths return `Error::Rights`.

#### Verification
- Integration: `runtime:tests/virt/guest_ovmf_tpm_*` on H-015.
- Manual: user-supplied Windows ISO boot procedure on H-002, recorded on the pull request.
- Review: SEC lead confirms host TPM is untouched.

#### Evidence
- none

### VIRT-005 · Grant VM host access through inspectable Capabilities
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: CAP-037, NET-012, OBS-019, UIP-003, VIRT-008
- Baseline: §7, §9.1, §23
- Threats: T-036
- Invariants: I-021, I-081

Virtualization is the largest ambient-authority hole on existing desktops. Folders, devices, network and clipboard reach a guest only as granted Capabilities, each visible in `os inspect`, with revoke taking effect on the next Operation (§9.1, I-021, T-036). USB and PCI passthrough use these grants; GPU passthrough is a later task.

<!-- covers: GAP-0444 -->

#### Out of scope
GPU passthrough (VIRT-017). Consent UI chrome (APP-025). Network broker internals (NET-012). Clipboard protocol (UIP-003).

#### Acceptance criteria
- [ ] A VM created with no host-access Capabilities cannot enumerate host folders, devices, network or clipboard; those Operations return `Error::Rights` and allocate no handle.
- [ ] Granting a folder, device, network or clipboard Capability is visible on the VirtualMachine Component in `os inspect`.
- [ ] Revoking a grant takes effect on the next Operation from that guest; a subsequent access returns `Error::Rights`.
- [ ] USB and PCI device assignment consumes these grants and fails closed when the Capability is absent.

#### Verification
- Unit: `runtime:tests/virt/host_grants_*` on `qemu-x86_64`.
- Integration: grant, inspect and revoke fixtures on H-015.
- Review: CAP lead confirms revoke matches persistent-grant-store semantics.

#### Evidence
- none

### VIRT-006 · Ship Linux guest tools for folders, clipboard and display
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: VIRT-003, VIRT-005, VIRT-008
- Baseline: §49
- Invariants: I-081

A fallback VM is unusable without shared folders, clipboard, time sync, display resize and a guest agent. Linux guests ship first, speaking the protocol named by VIRT-003. Per-window Surfaces stay in VIRT-012. IPC-058 is native-guest-to-host Channels, not this agent.

<!-- covers: EXTRA-054, GAP-0448 -->

#### Out of scope
Windows guest tools (VIRT-009). Per-window Surfaces (VIRT-012). Native-guest Channel transport (IPC-058).

#### Acceptance criteria
- [ ] A Linux guest on H-015 runs the guest agent; shared folder, clipboard, time sync and display resize each have a passing integration test that requires the matching host-access Capability.
- [ ] Without the matching Capability, the corresponding agent request returns `Error::Rights` and the guest keeps running.
- [ ] The agent speaks only the protocol named by VIRT-003; a mismatch is a typed error.
- [ ] `os inspect` names the guest agent as a child Component of the VirtualMachine.

#### Verification
- Integration: `runtime:tests/virt/linux_guest_tools_*` on H-015.
- Review: UIP lead confirms clipboard bridging uses Capability transfer, not a POSIX socket.

#### Evidence
- none

### VIRT-007 · Evaluate guest GPU paths for a VM gaming fallback
- Type: spike
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: GFX-001, HW-018, VIRT-002
- Baseline: §56.1, §56.2, §57
- Risks: R-035, R-037
- Invariants: I-009, I-045, I-061

GPU-bound software the Windows personality cannot run is why users keep a Windows partition (R-035). This spike compares virtio-gpu Venus/virgl, SR-IOV and full passthrough with multi-GPU on retained Linux DRM (I-045, I-009). No frame-time figure appears in prose; VIRT-010 publishes B-049.

<!-- covers: GAP-0446 -->

#### Out of scope
V3 virtio-gpu implementation (VIRT-011). V4 SR-IOV and passthrough (VIRT-017). Host DRM driver residency (GFX, KRN). B-049 harness (VIRT-010, BEN-050).

#### Acceptance criteria
- [ ] `reports/spikes/VIRT-007.md` exists with the spike skeleton headings.
- [ ] The report compares virtio-gpu Venus/virgl, SR-IOV and full passthrough on H-002 and H-017, and names which path VIRT-011 implements.
- [ ] The report names which path VIRT-017 implements, or records that passthrough is infeasible on the V4 Tier 1 set.
- [ ] The report cites B-049 for publication and states no superiority claim (I-061).

#### Verification
- Report: how virtio-gpu Venus/virgl, SR-IOV and full passthrough compare for a VM gaming fallback; which path V3 implements; which path V4 implements; what remains infeasible on single-GPU machines; whether titles that keep a Windows partition have a supported path without a native GPU rewrite.
- Review: GFX and HW leads sign off on the pull request that lands the report.

#### Evidence
- none

### VIRT-008 · Implement the KVM VM manager as a Capability-scoped Component
- Type: build
- Milestone: V2
- Status: todo
- Size: L
- Owner: none
- Depends on: CMP-027, KRN-025, KRN-037, OBS-019, SCH-007, VIRT-002
- Baseline: §7, §9.1, §10, §23, §69
- Threats: T-036
- Invariants: I-014, I-021, I-081

The V2 product is a userspace KVM manager that creates each VM as a Component in a ResourceDomain holding `Capability<VirtualMachine>` from the kernel object (§10, §23, I-081). Virtualization does not reopen ambient authority (§9.1, T-036). Native software never opens `/dev/kvm`.

<!-- covers: EXTRA-053, GAP-0444 -->

#### Out of scope
Kernel KVM object (KRN-037). Nested-virt CI harness (KRN-036). Native-guest Channel transport (IPC-058). Manager UI (VIRT-016). Host-access grant table (VIRT-005).

#### Acceptance criteria
- [ ] Creating a VM allocates a Component in a ResourceDomain that holds `Capability<VirtualMachine>` and no host-access Capabilities by default.
- [ ] Start, stop, pause and destroy are Operations; destroy reclaims guest MemoryObject pages, verified by a create/destroy leak test on H-015.
- [ ] Opening `/dev/kvm` or a POSIX qemu descriptor from native software returns `Error::Rights` and allocates no handle.
- [ ] `os inspect` shows the VirtualMachine Component, its ResourceDomain budgets and held Capabilities.
- [ ] Nested create on H-015 boots a Linux guest to a serial prompt using the kernel VirtualMachine object.

#### Verification
- Unit: `runtime:tests/virt/manager_lifecycle_*` on `qemu-x86_64`.
- Integration: nested create/destroy leak test on H-015.
- Review: KRN lead confirms the manager uses only `Capability<VirtualMachine>` from KRN-037.

#### Evidence
- none

### VIRT-009 · Ship Windows guest tools without redistributing Windows images
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: VIRT-004, VIRT-006
- Baseline: §49
- Invariants: I-007, I-070

Windows guests need the same agent protocol as Linux guests plus virtio-win, clipboard, folders, time and display resize, installed into a user-supplied Windows image. CI fails if a Windows image or license key is in tree or in release artifacts (I-070). Native host software still sees no Win32 (I-007).

<!-- covers: EXTRA-054, GAP-0448 -->

#### Out of scope
Per-window Surfaces (VIRT-012). Wine and Proton (WIN). Wine license review (GOV-047). Guest OVMF/TPM (VIRT-004).

#### Acceptance criteria
- [ ] Windows guest tools install from a Package into a user-supplied Windows guest and speak the protocol named by VIRT-003.
- [ ] Shared folder, clipboard, time sync and display resize each have a Manual procedure on H-002 using a user-supplied ISO; each path requires the matching host-access Capability.
- [ ] CI on every merge fails if a Windows ISO, WIM or license key is present in the tree or in release artifacts.
- [ ] Native host Components never import a Win32 type from the guest-tools Package.

#### Verification
- Integration: tree and artifact scanners `runtime:tests/virt/no_windows_redistrib_*` on `qemu-x86_64`.
- Manual: user-supplied Windows guest tools procedure on H-002, recorded on the pull request.
- Review: GOV lead confirms no Microsoft image or key is redistributed.

#### Evidence
- none

### VIRT-010 · Publish guest GPU frame-time against host and Proton
- Type: benchmark
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: BEN-050, VIRT-011, VIRT-007
- Baseline: §54, §57
- Benchmarks: B-049
- Invariants: I-061

V2 gaming gates publish personality frame-time against Linux plus Proton via B-027. The VM fallback needs the same published comparison on the path VIRT-007 selects, as the VM-guest GPU fraction of B-049. Publish-only at V3; the register holds targets. The harness is reused by VIRT-017.

#### Out of scope
Register ownership and NTFS/SMB/migration legs of B-049 (BEN-050). Personality frame-time (B-027, WIN). Choosing the GPU path (VIRT-007).

#### Acceptance criteria
- [ ] Harness `bench:interop` records VM-guest application launch time and GPU fraction as named series for the path VIRT-011 implements.
- [ ] A report exists under `reports/benchmarks/B-049/` for every V3 in-scope H-ID that can run that path, meeting the register target kind for V3.
- [ ] The report cites B-049 and the Proton/host baselines and states no superiority claim (I-061).

#### Verification
- Bench: B-049 on H-002, H-003 and H-015; target per register.
- Review: BEN lead confirms series names match the register method and are not double-counted against B-027.

#### Evidence
- none

### VIRT-011 · Implement virtio-gpu acceleration for Linux and Windows guests
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: GFX-092, VIRT-006, VIRT-007, VIRT-009
- Baseline: §40, §56.1, §57
- Invariants: I-009, I-045

V3 experimental gamers need a 3D path before VFIO passthrough. This task implements the virtio-gpu option VIRT-007 names, presenting guest output through GFX-092. Host DRM stays the driver stack; this is not a native GPU rewrite (I-045, I-009).

Required by the VIRT scope: "guest GPU acceleration".

#### Out of scope
SR-IOV and full passthrough (VIRT-017). Compositor remote-Surface protocol (GFX-092). B-049 publication (VIRT-010).

#### Acceptance criteria
- [ ] A Linux guest on H-015 and H-003 presents virtio-gpu output as a compositor Surface owned by the VM Component.
- [ ] A user-supplied Windows guest on H-002 presents virtio-gpu output on the same path; CI contains no Windows image.
- [ ] Native software never receives a DRM ioctl or device node from this path (I-045).
- [ ] The implementation matches the virtio-gpu option named in `reports/spikes/VIRT-007.md`.

#### Verification
- Integration: `runtime:tests/virt/guest_virtio_gpu_*` on H-015 and H-003.
- Manual: user-supplied Windows virtio-gpu procedure on H-002.
- Review: GFX lead confirms Surfaces come from GFX-092.

#### Evidence
- none

### VIRT-012 · Bridge guest windows, clipboard, drag/drop and notifications
- Type: build
- Milestone: V3
- Status: todo
- Size: L
- Owner: none
- Depends on: APP-002, APP-014, APP-043, GFX-092, UIP-003, UIP-032, VIRT-003, VIRT-006, VIRT-009
- Baseline: §40, §49
- Invariants: I-048

Implements the depth VIRT-003 chose so guests meet §49 (taskbar, clipboard, chooser, notifications) instead of an obvious VM. V3 because GFX-092 lands then. Native software still does not see Wayland or X11 (I-048).

<!-- covers: EXTRA-054 -->

#### Out of scope
Compositor remote-Surface transport (GFX-092). Clipboard and drag/drop protocol (UIP). Notify service and chooser chrome (APP). Guest-agent protocol choice (VIRT-003).

#### Acceptance criteria
- [ ] Guest tools speak the agent protocol named by VIRT-003; a mismatch returns a typed error and the VM keeps running.
- [ ] Clipboard, drag/drop, UserSelected chooser and notification bridging each have an integration test on H-015 for Linux guests, or the Decision records that the accepted option omits that bridge.
- [ ] A guest window appears as a compositor Surface owned by the VM Component and in the native taskbar; closing the Surface does not destroy the VirtualMachine.
- [ ] Native software never sees a Wayland or X11 socket on this path (I-048).
- [ ] Windows guests use the same bridges via a Manual procedure on H-002 with a user-supplied ISO.

#### Verification
- Integration: `runtime:tests/virt/guest_window_bridge_*` on H-015.
- Manual: Windows guest window procedure on H-002.
- Demo: V3 Linux guest window beside a native Terminal on H-002, with clipboard transfer both ways.
- Review: UIP and APP leads sign off on Capability transfer versus path leakage.

#### Evidence
- none

### VIRT-013 · Offer the VM fallback from compatibility triage
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: APP-052, REL-015, VIRT-002, VIRT-008, VIRT-016, WIN-018, WIN-064
- Baseline: §49, §56.2
- Risks: R-036
- Invariants: I-071

When an ELF or PE fails under a Personality, the OS offers to run it in a granted VM rather than failing silently (§49, R-036). REL owns explanation text and the public compatibility database; WIN-064 owns the PE reason; this task is the VM-offer action and handoff into the manager. Ordinary software still launches without an obvious VM (WIN-001).

<!-- covers: EXTRA-056, GAP-0450, GAP-0510 -->

#### Out of scope
Compatibility database publication (REL-015). PE failure reason strings (WIN-064). Manager UI chrome (VIRT-016). Anti-cheat bypass (I-071).

#### Acceptance criteria
- [ ] A PE that WIN-064 rejects presents an offer to create a VM with no host-access Capabilities except those the user grants in the manager UI.
- [ ] Accepting the offer starts VIRT-008 with the binary attached as guest media; declining leaves no VirtualMachine Component allocated.
- [ ] The offer cites the REL compatibility-database entry when one exists, and cites WIN-018 when the reason is kernel-level anti-cheat.
- [ ] Double-click of a W1 title that the Windows personality runs does not open this offer (WIN-001).

#### Verification
- Integration: `runtime:tests/virt/triage_offer_*` on H-015.
- Demo: V3 PE anti-cheat miss offers a VM on H-002; a Gold W1 title still launches in the personality.
- Review: WIN and REL leads sign off on the split of reason, database and offer.

#### Evidence
- none

### VIRT-014 · Evaluate booting a physical Windows partition as a VM
- Type: spike
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: INS-008, INS-026, LAB-015, STO-060, VIRT-004
- Baseline: §49
- Invariants: I-070

Reusing the dual-boot Windows volume as a guest removes the reboot cost of dual-boot. This spike records activation, TPM and BitLocker implications of attaching that volume, once the V3 installer coexistence path exists, and tells VIRT-018 whether a supported path exists. INS owns partition layout; STO owns NTFS.

<!-- covers: GAP-0447 -->

#### Out of scope
Installer coexistence (INS-026). NTFS driver policy (STO-060). V4 attach implementation (VIRT-018).

#### Acceptance criteria
- [ ] `reports/spikes/VIRT-014.md` exists with the spike skeleton headings.
- [ ] The report rules in or out unsupported, hibernate-and-attach, and both-ways dual boot, with activation, TPM and BitLocker consequences for each.
- [ ] The report names the host Operations that must refuse dirty or Fast Startup NTFS, and records that no Windows image or key is redistributed (I-070).
- [ ] The report states whether VIRT-018 ships at V4 or is dropped as infeasible.

#### Verification
- Report: whether attaching the dual-boot Windows partition is unsupported, hibernate-and-attach or both-ways; what activation, TPM and BitLocker do on each path; which dirty-volume cases INS and STO already refuse; whether a V4 path can ship without redistributing Windows images.
- Review: INS, STO and SEC leads sign off on the pull request that lands the report.

#### Evidence
- none

### VIRT-015 · Publish the VM fallback chapter of the compatibility guide
- Type: docs
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: DOC-028, VIRT-006, VIRT-013, VIRT-009, WIN-018
- Baseline: §49, §56.2
- Invariants: I-070, I-071

V3 documentation and the 1.0 compatibility guide need an honest VM-fallback chapter: how to offer a VM, which guests are supported, and the ban on redistributing Windows images or keys (I-070). DOC owns the site pipeline; VIRT owns this product chapter.

<!-- covers: GAP-0448 -->

#### Out of scope
Compatibility guide chassis (DOC-028). Anti-cheat title list (WIN-018). IDL-to-docs generation (DOC).

#### Acceptance criteria
- [ ] The compatibility guide contains a VM-fallback chapter covering offer flow, supported Linux and Windows guests, host-access grants and the Windows-image ban.
- [ ] The chapter cites I-071 and WIN-018 and does not describe an anti-cheat bypass.
- [ ] The chapter is linked from DOC-028 and builds in documentation CI.

#### Verification
- Review: DOC and VIRT leads sign off on the pull request.
- Manual: chapter headings cover offer, guests, grants and the image ban.

#### Evidence
- none

### VIRT-016 · Build the native VM manager UI with Capability review
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: APP-025, VIRT-005, VIRT-008
- Baseline: §9.1, §49
- Invariants: I-021

V3 public-alpha strangers cannot be asked to drive VMs from a CLI. This UI creates, starts and stops VMs, picks disks and firmware, and reviews or denies host-access Capability requests through the trusted consent prompt. APP owns shell, store and consent chrome; this UI is the VIRT product surface. The V2 manager may be CLI-only.

<!-- covers: GAP-0443 -->

#### Out of scope
Consent prompt chrome (APP-025). Grant table (VIRT-005). Store client (APP-045). Shell panel (APP-043).

#### Acceptance criteria
- [ ] The UI creates, starts, stops and destroys a VirtualMachine on H-015 without a CLI.
- [ ] A host-access request for folder, device, network or clipboard presents the consent UI; deny leaves the Capability unallocated and the VM running.
- [ ] Disk and firmware pickers mint UserSelected objects; the UI cannot enumerate host volumes it was not granted.
- [ ] `os inspect` after a session matches the grants the UI displayed.

#### Verification
- Integration: `runtime:tests/virt/manager_ui_*` on H-015.
- Demo: V3 create-and-grant flow on H-002 using APP-025.
- Review: APP lead confirms consent chrome is reused, not forked.

#### Evidence
- none

### VIRT-017 · Implement SR-IOV and GPU passthrough for VM guests
- Type: build
- Milestone: V4
- Status: todo
- Size: L
- Owner: none
- Depends on: HW-017, HW-026, HW-070, VIRT-010, VIRT-011, VIRT-005, VIRT-007
- Baseline: §56.1, §57
- Invariants: I-009, I-038, I-045

Remaining GAP-0446 path after virtio-gpu: SR-IOV and full passthrough on the V4 multi-GPU and NVIDIA Tier 1 set, which is the 1.0 answer for GPU-bound software the Windows personality cannot run. Host DRM stays the driver stack (I-045). IOMMU protection is mandatory (I-038). Size L because the work crosses VIRT, GFX and HW.

<!-- covers: GAP-0446 -->

#### Out of scope
IOMMU policy (HW-017). NVIDIA host bring-up (HW-070). virtio-gpu path (VIRT-011). Host-access grant table (VIRT-005).

#### Acceptance criteria
- [ ] SR-IOV VF assignment to a VM on H-017 fails closed when the holder lacks the device Capability, returning `Error::Rights` and allocating no handle.
- [ ] Full GPU passthrough on H-006 and H-017 presents the GPU as a Capability on the VM Component; the host compositor does not hold that Capability while the guest runs.
- [ ] Releasing the VM returns the GPU to the host compositor without a host reboot on H-002.
- [ ] Native software never opens a VFIO POSIX device node; those paths return `Error::Rights`.
- [ ] B-049 on H-017 reuses the V3 harness for the passthrough path; the report states no superiority claim.

#### Verification
- Integration: `runtime:tests/virt/gpu_passthrough_*` on H-017.
- Bench: B-049 on H-017; target per register.
- Manual: NVIDIA passthrough procedure on H-006.
- Review: HW and GFX leads confirm host DRM residency is unchanged (I-045).

#### Evidence
- none

### VIRT-018 · Boot an existing Windows dual-boot partition as a VM guest
- Type: build
- Milestone: V4
- Status: todo
- Size: L
- Owner: none
- Depends on: INS-026, STO-060, VIRT-004, VIRT-014, VIRT-008, VIRT-009
- Baseline: §49
- Invariants: I-070

Implements VIRT-014 if the report names a supported path; otherwise this task is dropped with reason infeasible. VIRT attaches the volume as a VirtualMachine disk with TPM and BitLocker caveats recorded. INS owns partition layout; STO owns NTFS. No Windows image or key is stored (I-070).

#### Out of scope
Installer dual-boot layout (INS-026). NTFS dirty-volume policy (STO-060). Guest OVMF/TPM (VIRT-004). Spike report (VIRT-014).

#### Acceptance criteria
- [ ] Attaching the dual-boot Windows volume as a VirtualMachine disk starts the guest through OVMF and emulated TPM on the path the spike named.
- [ ] Dirty or Fast Startup NTFS is refused with a typed error and the host volume is left unmodified.
- [ ] BitLocker and activation caveats from the spike appear in `os inspect` on the VM and in the manager UI.
- [ ] The content-addressed store and release artifacts gain no Windows ISO or license key (I-070).
- [ ] INS partition layout and STO NTFS policy are unchanged; this task only attaches an existing volume.

#### Verification
- Integration: refuse-dirty-NTFS fixture on H-015.
- Manual: attach procedure on H-002 using LAB-015, recorded on the pull request.
- Review: INS and STO leads confirm layout and NTFS policy are not forked.

#### Evidence
- none

### VIRT-019 · Snapshot and clone VM disks via the Content-addressed store
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: STO-009, STO-011, STO-025, VIRT-008
- Baseline: §27, §30
- Invariants: I-044

A broken guest must be restorable the same way a SystemGeneration is. STO owns CoW and snapshots; VIRT owns VM disk objects that snapshot, clone and roll back without a second storage model (I-044). Needed before 1.0, after the V2 manager exists.

#### Out of scope
Store substrate and snapshot Operations (STO-025, STO-009). SystemGeneration rollback (PKG). User-data snapshot catalog UI (STO-070).

#### Acceptance criteria
- [ ] Snapshot of a VM disk is a content-addressed object; clone is a CoW child visible in `os inspect storage`.
- [ ] Restore of a snapshot returns the guest disk to that object identity; a subsequent guest boot reads the restored content.
- [ ] Restoring a guest disk does not mutate the live SystemGeneration.
- [ ] No second storage model appears in `os inspect storage` for VM disks.

#### Verification
- Integration: `runtime:tests/virt/disk_snapshot_*` on H-015.
- Review: STO lead confirms objects are store Blobs plus snapshot Operations, not a parallel volume manager.

#### Evidence
- none

### VIRT-020 · Verify the VM fallback on Tier 1 without shipping Windows images
- Type: build
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: VIRT-017, VIRT-011, VIRT-005, VIRT-006, VIRT-013, VIRT-007, VIRT-015, VIRT-008, VIRT-009, VIRT-012
- Baseline: §49, §56.2, §57
- Invariants: I-070, I-071

1.0 completeness check for the VIRT product: manager, guest tools, triage offer and GPU path run on every Tier 1 machine, and release artifacts contain no Windows ISO or key. Anchors the workstream to the 1.0 compatibility-guide and anti-cheat-non-promise exits.

<!-- covers: GAP-0448 -->

#### Out of scope
HCL publication (REL-048). Unsupported-title statement authorship (WIN-083). Compatibility guide chassis (DOC-041).

#### Acceptance criteria
- [ ] Manager, Linux guest tools, Windows guest tools, triage offer and the GPU path named by VIRT-007 run on every 1.0 Tier 1 machine.
- [ ] Release artifacts and the tree contain no Windows ISO and no Windows license key (I-070).
- [ ] `os inspect` on a running fallback VM shows the Component, ResourceDomain and host-access Capabilities; a revoked grant fails the next Operation.

#### Verification
- Integration: `runtime:tests/virt/fallback_1_0_gate_*` on the 1.0 hardware scope.
- Review: REL lead confirms artifacts contain no Windows image or key.
- Manual: checklist that the compatibility guide chapter and WIN-083 still name the VM fallback as the anti-cheat answer (I-071).

#### Evidence
- none
