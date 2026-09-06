# Question register

This register holds every inventory and gap item of kind `question`. A Q-ID is the only way to record an external impediment as a dependency: tasks list it in `Depends on`. Each entry names the owning workstream. Status starts `open`. `Answered by` stays `none` until a task or decision closes the question. Q-001 is the V0 benchmark-methodology question so later methodology work can depend on it; Q-029 is the visible-UI measurement boundary cited by B-016.

### Q-001 · What is the benchmark methodology standard
- Workstream: BEN
- Status: open
- Answered by: BEN-064
Reference hardware list, warm and cold definitions, percentiles reported, iteration counts, CPU frequency pinning and mitigation settings. Every later B-ID target assumes this answer.

### Q-002 · How does a native application opt into a personality
- Workstream: LNX
- Status: open
- Answered by: LNX-016
Capability to a personality, embedded Linux component, or SDK shim, and what authority that grant carries.

### Q-003 · Which kernel evolution phase is required for 1.0
- Workstream: GOV
- Status: open
- Answered by: KRN-050
Whether Phase C, D or E of §6 is a hard requirement for the 1.0 release gate.

### Q-004 · What are revocation semantics
- Workstream: CAP
- Status: open
- Answered by: none
Eager versus lazy invalidation, revocation of in-flight operations, and cost bounds on revocation trees.

### Q-005 · What is the inline versus MemoryObject size threshold
- Workstream: IPC
- Status: open
- Answered by: IPC-007
Size threshold and heuristics for an inline small message versus MemoryObject transfer.

### Q-006 · What are persistent MemoryObject semantics
- Workstream: STO
- Status: open
- Answered by: none
Relation to the storage model, crash consistency, and content addressing of persistent MemoryObjects.

### Q-007 · What are encrypted MemoryObject semantics
- Workstream: SEC
- Status: open
- Answered by: MEM-045
Key ownership, who can map plaintext, and interaction with hardware memory encryption.

### Q-008 · How are MemoryObject borrows enforced across components
- Workstream: MEM
- Status: open
- Answered by: MEM-018
Revocation on return, timeouts, or trust in the borrower.

### Q-009 · How does cancellation treat hardware-committed Operations
- Workstream: TSK
- Status: open
- Answered by: none
In-flight DMA, GPU dispatch and network send: whether cancel waits, fails, or returns best-effort, and how partially completed results are reported.

### Q-010 · How do personality threads map onto native Tasks
- Workstream: CMP
- Status: open
- Answered by: none
Linux personality threads and Windows personality threads versus native Tasks and execution contexts.

### Q-011 · What is cleanup for a cancelled Task holding objects
- Workstream: CMP
- Status: open
- Answered by: none
Capabilities, MemoryObjects, or partially transferred ownership: released immediately, on Task exit, or by the owner.

### Q-012 · How does cancellation treat uninterruptible Linux paths
- Workstream: CMP
- Status: open
- Answered by: none
A Task blocked in an inherited Linux kernel path that cannot be interrupted (uninterruptible sleep).

### Q-013 · Which scheduling intents require a capability
- Workstream: SEC
- Status: open
- Answered by: SEC-007
Whether Realtime and LowLatency require a capability to request, and how an unprivileged Component is prevented from starving others.

### Q-014 · How does intent inherit across a Channel handoff
- Workstream: SCH
- Status: open
- Answered by: none
So a LowLatency client calling a shared service does not queue behind Background work in the service.

### Q-015 · What happens on ResourceDomain budget exhaustion
- Workstream: SCH
- Status: open
- Answered by: none
Memory reclaim, throttling, Operation failure, or Component termination, and how it is reported to the owner.

### Q-016 · How do persistent grants survive restart
- Workstream: CAP
- Status: open
- Answered by: none
How an application retains access to a user-selected object across restarts (durable capability, recent-files list) and how the user revokes it.

### Q-017 · How does save-as grant create authority
- Workstream: STO
- Status: open
- Answered by: none
How an application obtains authority to create a new file in a user-chosen location without directory access.

### Q-018 · How do personalities present native storage objects
- Workstream: LNX
- Status: open
- Answered by: LNX-019
POSIX paths and Windows drive letters while respecting capability scope.

### Q-019 · What is content-store garbage collection
- Workstream: STO
- Status: open
- Answered by: none
Root set (live generations, installed packages, pinned objects), collection policy, and user control.

### Q-020 · How are compatibility applications packaged immutably
- Workstream: PKG
- Status: open
- Answered by: PKG-047
OCI images, Flatpak-like bundles and Wine prefixes while still satisfying the no-mutation rule.

### Q-021 · How do security fixes reach a pinned library
- Workstream: PKG
- Status: open
- Answered by: PKG-046
Rebuild and republish dependents, grafting or substitution rules, or runtime relinking, without global mutation.

### Q-022 · How does the native dependency model meet glibc
- Workstream: LNX
- Status: open
- Answered by: LNX-024
Linux personality shared libraries that expect a global `/usr/lib` namespace.

### Q-023 · What is excluded from a system generation
- Workstream: PKG
- Status: open
- Answered by: PKG-007
User data, ApplicationData, logs, caches, and how mutable state is separated from the immutable image.

### Q-024 · How atomic are kernel plus firmware updates
- Workstream: BOOT
- Status: open
- Answered by: BOOT-030
Atomicity of kernel plus driver plus firmware updates within a generation, including firmware that cannot be rolled back.

### Q-025 · How is configuration captured for restore
- Workstream: STO
- Status: open
- Answered by: none
Structured settings store versus snapshotting ApplicationData so configuration participates in restore.

### Q-026 · How does selective restore avoid inconsistency
- Workstream: PKG
- Status: open
- Answered by: none
Restore only apps, only OS, and how a partial restore avoids inconsistent combinations.

### Q-027 · Which compositor state is restored after restart
- Workstream: GFX
- Status: open
- Answered by: none
Window geometry, stacking order, focus, workspace, and where it is checkpointed.

### Q-028 · How does a user-space driver coexist with Linux device nodes
- Workstream: LNX
- Status: open
- Answered by: LNX-014
`/dev` nodes, sysfs and udev events for the same device.

### Q-029 · What is the visible-UI measurement boundary
- Workstream: BEN
- Status: open
- Answered by: none
The measurement boundary for visible UI is the first compositor presentation of a non-blank frame, so startup numbers are comparable across applications and systems. B-016 cites this entry.

### Q-030 · How are environment.yaml version specifiers locked
- Workstream: ENV
- Status: open
- Answered by: none
How specifiers such as language and database versions are locked, where the lock file lives, and how service packages are discovered.

### Q-031 · How does an environment expose service endpoints
- Workstream: ENV
- Status: open
- Answered by: none
How the environment's network namespace exposes service endpoints to the developer's shell, IDE and browser without granting ambient network authority.

### Q-032 · Is Docker socket API compatibility required
- Workstream: LNX
- Status: open
- Answered by: LNX-012
Whether Docker socket API compatibility is required for developer adoption or whether podman-compatible tooling suffices.

### Q-033 · How does ComputeDevice relate to ComputeQueue
- Workstream: HET
- Status: open
- Answered by: none
Whether a queue is derived from a device capability, and who owns scheduling between them.

### Q-034 · How do native components use Mesa without DRM
- Workstream: GFX
- Status: open
- Answered by: none
Mesa depends on libdrm and Linux device nodes; native components must not expose DRM to the application.

### Q-035 · Are semantic actions and accessibility actions one tree
- Workstream: ACC
- Status: open
- Answered by: ACC-008
Whether they are one tree or two, and how they stay consistent.

### Q-036 · How is capability unforgeability preserved across machines
- Workstream: CAP
- Status: open
- Answered by: none
Cryptographic capabilities, sturdy references, or proxies.

### Q-037 · Is an AI assistant a distinct security principal
- Workstream: SEC
- Status: open
- Answered by: SEC-034
Whether an AI assistant is a distinct security principal from the user for audit and revocation.

### Q-038 · How is file.type determined for automation
- Workstream: STO
- Status: open
- Answered by: none
Typed object kinds versus MIME sniffing, and where the type registry lives.

### Q-039 · How do Linux-personality processes acquire capabilities
- Workstream: LNX
- Status: open
- Answered by: LNX-013
Default compatibility capability profile, path-to-capability mapping, and how the user grants more.

### Q-040 · Are 32-bit x86 Windows applications supported at 1.0
- Workstream: WIN
- Status: open
- Answered by: none
WoW64, and what kernel support that requires. Tied to the V1 ia32 decision.

### Q-041 · How do Windows installers become installed applications
- Workstream: WIN
- Status: open
- Answered by: none
How `setup.exe` writing into a prefix becomes an installed application visible in the launcher and system history.

### Q-042 · Where do typed object kinds such as Image come from
- Workstream: STO
- Status: open
- Answered by: none
A platform type registry so `files.choose::<Image>()` and `UserSelected<T>` share one type system.

### Q-043 · What is the kernel-level anti-cheat policy
- Workstream: WIN
- Status: open
- Answered by: none
Refuse, VM fallback, or vendor engagement, for kernel-level anti-cheat drivers that are incompatible with the capability model.

### Q-044 · What is the browser strategy
- Workstream: APP
- Status: open
- Answered by: none
Chromium or Firefox through the Linux personality, a native port, or both, and the criteria for switching.

### Q-045 · What is the IDE strategy
- Workstream: APP
- Status: open
- Answered by: none
VS Code or JetBrains through the Linux personality versus a native editor or IDE, and the criteria for switching.

### Q-046 · What are the V4 and 1.0 criteria the baseline omitted
- Workstream: GOV
- Status: open
- Answered by: GOV-076
The baseline defines no criteria for V4 beta or 1.0 stable. Stability SLOs, security audit, ABI freeze, support commitment, hardware list and compatibility pass rates are defined by the milestone files; this question closes when those files are accepted as the 1.0 definition.

### Q-047 · Which concepts are Layer 1 versus Layer 2
- Workstream: ABI
- Status: open
- Answered by: ABI-011
Whether the compositor protocol, the package format and ResourceDomain policies are Layer 1 or Layer 2. S-001 through S-034 record the working assignment; this question closes when the V0 ABI-shape decision accepts it.

### Q-048 · How is KVM exposed natively
- Workstream: KRN
- Status: open
- Answered by: KRN-025
Whether virtualization is `Capability<VirtualMachine>`, and whether compatibility personalities may fall back to VMs for unsupported software.

### Q-049 · Which jurisdiction should the legal entity be in
- Workstream: GOV
- Status: open
- Answered by: none
Export controls, GDPR, trademark cost and tax treatment of donations.

### Q-050 · Does in-kernel PE loading conflict with Wine licensing
- Workstream: WIN
- Status: open
- Answered by: none
Whether NT-object or PE-loading emulation in the GPLv2 kernel conflicts with LGPL Wine components, or whether all Win32 emulation must remain in userspace.

### Q-051 · How does the fork track upstream's minimum Rust version
- Workstream: KRN
- Status: answered
- Answered by: KRN-004
When upstream Linux raises its minimum Rust version or stabilises features the fork's Rust code depends on, whether to track upstream's minimum or pin independently.

### Q-052 · May a generation switch apply without reboot
- Workstream: PKG
- Status: open
- Answered by: PKG-070
kexec into the new kernel or a live userspace-only generation switch, given that lockdown forbids unsigned kexec and measured-boot values would need re-derivation.

### Q-053 · What is the funding plan for infrastructure through 1.0
- Workstream: GOV
- Status: open
- Answered by: GOV-041
CDN bandwidth, build farm, hardware lab and signing hardware.

### Q-054 · Can Wi-Fi credentials be imported during migration
- Workstream: INS
- Status: open
- Answered by: INS-044
NetworkManager keyfiles and Windows WLAN profiles (DPAPI-encrypted), or whether the user must re-enter them.

### Q-055 · Should OEM partnerships or certification precede 1.0
- Workstream: GOV
- Status: open
- Answered by: GOV-078
OEM partnerships or a hardware certification programme before 1.0, or leave it post-1.0.

### Q-056 · Is application-state restore achievable
- Workstream: PKG
- Status: open
- Answered by: none
Checkpointing, application-cooperative state interfaces, or not at all, and whether to scope it as a 1.0 non-goal if evidence is negative.

<!-- roadmap:generated:begin status -->
| ID | Title | Status |
| --- | --- | --- |
| Q-001 | What is the benchmark methodology standard | open |
| Q-002 | How does a native application opt into a personality | open |
| Q-003 | Which kernel evolution phase is required for 1.0 | open |
| Q-004 | What are revocation semantics | open |
| Q-005 | What is the inline versus MemoryObject size threshold | open |
| Q-006 | What are persistent MemoryObject semantics | open |
| Q-007 | What are encrypted MemoryObject semantics | open |
| Q-008 | How are MemoryObject borrows enforced across components | open |
| Q-009 | How does cancellation treat hardware-committed Operations | open |
| Q-010 | How do personality threads map onto native Tasks | open |
| Q-011 | What is cleanup for a cancelled Task holding objects | open |
| Q-012 | How does cancellation treat uninterruptible Linux paths | open |
| Q-013 | Which scheduling intents require a capability | open |
| Q-014 | How does intent inherit across a Channel handoff | open |
| Q-015 | What happens on ResourceDomain budget exhaustion | open |
| Q-016 | How do persistent grants survive restart | open |
| Q-017 | How does save-as grant create authority | open |
| Q-018 | How do personalities present native storage objects | open |
| Q-019 | What is content-store garbage collection | open |
| Q-020 | How are compatibility applications packaged immutably | open |
| Q-021 | How do security fixes reach a pinned library | open |
| Q-022 | How does the native dependency model meet glibc | open |
| Q-023 | What is excluded from a system generation | open |
| Q-024 | How atomic are kernel plus firmware updates | open |
| Q-025 | How is configuration captured for restore | open |
| Q-026 | How does selective restore avoid inconsistency | open |
| Q-027 | Which compositor state is restored after restart | open |
| Q-028 | How does a user-space driver coexist with Linux device nodes | open |
| Q-029 | What is the visible-UI measurement boundary | open |
| Q-030 | How are environment.yaml version specifiers locked | open |
| Q-031 | How does an environment expose service endpoints | open |
| Q-032 | Is Docker socket API compatibility required | open |
| Q-033 | How does ComputeDevice relate to ComputeQueue | open |
| Q-034 | How do native components use Mesa without DRM | open |
| Q-035 | Are semantic actions and accessibility actions one tree | open |
| Q-036 | How is capability unforgeability preserved across machines | open |
| Q-037 | Is an AI assistant a distinct security principal | open |
| Q-038 | How is file.type determined for automation | open |
| Q-039 | How do Linux-personality processes acquire capabilities | open |
| Q-040 | Are 32-bit x86 Windows applications supported at 1.0 | open |
| Q-041 | How do Windows installers become installed applications | open |
| Q-042 | Where do typed object kinds such as Image come from | open |
| Q-043 | What is the kernel-level anti-cheat policy | open |
| Q-044 | What is the browser strategy | open |
| Q-045 | What is the IDE strategy | open |
| Q-046 | What are the V4 and 1.0 criteria the baseline omitted | open |
| Q-047 | Which concepts are Layer 1 versus Layer 2 | open |
| Q-048 | How is KVM exposed natively | open |
| Q-049 | Which jurisdiction should the legal entity be in | open |
| Q-050 | Does in-kernel PE loading conflict with Wine licensing | open |
| Q-051 | How does the fork track upstream's minimum Rust version | answered |
| Q-052 | May a generation switch apply without reboot | open |
| Q-053 | What is the funding plan for infrastructure through 1.0 | open |
| Q-054 | Can Wi-Fi credentials be imported during migration | open |
| Q-055 | Should OEM partnerships or certification precede 1.0 | open |
| Q-056 | Is application-state restore achievable | open |
<!-- roadmap:generated:end -->
