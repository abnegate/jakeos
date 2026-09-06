# Invariant register

This register holds standing rules drawn from constraints, non-goals, §57, §65 and §67. It is owned by GOV. Entries are not work items; each becomes `enforced` when a done task lists it under `Invariants:` and installs a lint, gate or test. Status starts `stated`. `Enforced by` stays `none` until tasks exist. Related constraints are merged: one I-ID covers a cluster (for example all ambient-authority denials are I-021).

### I-001 · x86-64 is the only 1.0 architecture
- Baseline: §1
- Enforced by: none
- Status: stated
Desktop and laptop x86-64 only. ARM64 and RISC-V are kept compiling in the fork (I-011, I-012) but are not 1.0 platforms.

### I-002 · Native model is capability, component, async, object
- Baseline: §1
- Enforced by: none
- Status: stated
Every native ABI surface is checked against these four properties.

### I-003 · Not a Linux distribution
- Baseline: §1, §57
- Enforced by: none
- Status: stated
Project identity and gating reject distribution-style deliverables.

### I-004 · Not a desktop environment on Linux userspace
- Baseline: §1
- Enforced by: none
- Status: stated

### I-005 · Native APIs are not thin wrappers over Linux syscalls
- Baseline: §1, §3
- Enforced by: none
- Status: stated

### I-006 · POSIX exists only inside the Linux personality
- Baseline: §1, §3, §57
- Enforced by: none
- Status: stated
The native OS is not POSIX-compatible.

### I-007 · Win32 exists only inside the Windows personality
- Baseline: §1, §3
- Enforced by: none
- Status: stated
The project is not a Windows clone.

### I-008 · Kernel residency is decided by measured cost
- Baseline: §1, §33, §57
- Enforced by: none
- Status: stated
Not a traditional microkernel for academic purity. Drivers may remain in the kernel where the criteria justify it.

### I-009 · No rewrite for the sake of rewriting
- Baseline: §1, §2, §57, §67
- Enforced by: none
- Status: stated
Every rewrite cites a semantic or measured benefit. Mature mechanisms are preserved; inherited semantics are replaced.

### I-010 · Inherit Linux hardware, not the Linux programming model
- Baseline: §1, §2
- Enforced by: none
- Status: stated

### I-011 · ARM64 support in the fork is not removed
- Baseline: §1, §38
- Enforced by: none
- Status: stated
An ARM64 kernel and userspace cross-build stays compiling in CI even though ARM64 is not a 1.0 platform.

### I-012 · RISC-V support in the fork is not removed
- Baseline: §1, §38
- Enforced by: none
- Status: stated

### I-013 · POSIX-shaped native APIs need independent justification
- Baseline: §1, §65
- Enforced by: none
- Status: stated
Each POSIX-resembling native API requires an accepted decision. Rust `std` support, if any, lives at Layer 3.

### I-014 · Component replaces process as the isolation unit
- Baseline: §1, §10
- Enforced by: none
- Status: stated

### I-015 · Typed Capability handles replace file descriptors natively
- Baseline: §1, §7
- Enforced by: none
- Status: stated

### I-016 · Paths are not the security model
- Baseline: §1, §9.1, §25
- Enforced by: none
- Status: stated
No native authority check is based on a filesystem path. Humans still see files and folders.

### I-017 · Task and TaskGroup replace threads as the native concurrency unit
- Baseline: §1, §20, §21
- Enforced by: none
- Status: stated

### I-018 · Operations and channels replace signals
- Baseline: §1, §18, §19
- Enforced by: none
- Status: stated

### I-019 · Native isolation does not require containers
- Baseline: §1, §36
- Enforced by: none
- Status: stated
OCI support belongs to the Linux personality. No native subsystem depends on an OCI runtime.

### I-020 · Installation does not mutate a shared filesystem
- Baseline: §1, §28
- Enforced by: none
- Status: stated
No shared-library copies into shared locations, no registry-style global configuration, no implicit daemons, no drops across shared directories.

### I-021 · No ambient authority
- Baseline: §9.1, §67
- Enforced by: none
- Status: stated
A native component starts with exactly the capabilities it was handed. It does not automatically receive filesystem, home, network, camera, microphone, process enumeration, device, clipboard, location, screen or other-application access. Selecting one file never grants the containing directory.

### I-022 · System state is versioned generations plus history
- Baseline: §1, §30, §31
- Enforced by: none
- Status: stated
Not an accumulation of mutable files. The running image is not mutated in place.

### I-023 · AI uses semantic interfaces and capabilities
- Baseline: §1, §42, §44, §57
- Enforced by: none
- Status: stated
AI does not operate by pretending to be a human moving a mouse. AI work is gated on a done semantic-registry task.

### I-024 · Hardware topology is not assumed to stay CPU plus RAM plus GPU
- Baseline: §1, §37, §38
- Enforced by: none
- Status: stated
ComputeDevice taxonomy is open-ended. MemoryObject is abstract enough for CXL, persistent, disaggregated, unified and accelerator-local memory.

### I-025 · Compatibility is a subsystem, not the architecture
- Baseline: §3, §67
- Enforced by: none
- Status: stated
Personalities consume the native ABI and never extend it. Native applications see none of the Linux or Windows constructs unless they opt into a compatibility environment.

### I-026 · Native interfaces never exist merely because POSIX has an equivalent
- Baseline: §3, §65
- Enforced by: none
- Status: stated

### I-027 · All three application paths terminate in native kernel objects
- Baseline: §4
- Enforced by: none
- Status: stated

### I-028 · Capabilities are unforgeable, typed and attenuable
- Baseline: §7, §8
- Enforced by: none
- Status: stated
Userspace cannot mint a valid capability. Using the wrong type fails at the kernel boundary. Derive never widens rights. The ABI reserves room for hardware enforcement.

### I-029 · Isolation is cheap enough to be the default
- Baseline: §10, §53, §67
- Enforced by: none
- Status: stated
SDK defaults create separate components. An exploit compromises the smallest useful unit.

### I-030 · Native kernel APIs are asynchronous by default
- Baseline: §18, §65
- Enforced by: none
- Status: stated
No native syscall blocks the calling execution context as its primary mode. No blocking read/write thread-per-call model as the native I/O interface.

### I-031 · No Task outlives its TaskGroup without a background capability
- Baseline: §21
- Enforced by: none
- Status: stated

### I-032 · Scheduling expresses intent, not only a numeric priority
- Baseline: §22
- Enforced by: none
- Status: stated

### I-033 · Resource accounting is a native kernel concept
- Baseline: §23
- Enforced by: none
- Status: stated
Applied to every Component and Task, including kernel-object limits.

### I-034 · Every primitive is observable in the same change that ships it
- Baseline: §24, §64, §67
- Enforced by: none
- Status: stated
`os inspect` and `os trace` ship in V0. Tooling does not reconstruct semantics from raw kernel events when the OS already knows them.

### I-035 · The chooser object carries the authority
- Baseline: §9.1, §25
- Enforced by: none
- Status: stated
No path-based permission check follows selection. Choosers render through trusted UI.

### I-036 · Packages and dependencies are immutable content-addressed objects
- Baseline: §28, §29
- Enforced by: none
- Status: stated
No global dependency conflict. Installing a package never breaks another installed package.

### I-037 · Failure and restart are part of every typed interface
- Baseline: §32, §67
- Enforced by: none
- Status: stated
Where hardware prevents seamless recovery the system reports explicit degraded recovery.

### I-038 · User-space drivers require IOMMU protection for DMA
- Baseline: §33
- Enforced by: none
- Status: stated

### I-039 · Application startup maps immutable package objects
- Baseline: §34
- Enforced by: none
- Status: stated
No dynamic file lookup and dependency resolution at launch as the native path.

### I-040 · No Layer 1 surface is frozen before V4
- Baseline: §65, §66
- Enforced by: none
- Status: stated
V0 lists no surfaces to freeze. V1 names freeze candidates. A `Freezes:` task requires a spike and a decision in its closure.

### I-041 · Version negotiation exists from V0
- Baseline: §12, §65
- Enforced by: none
- Status: stated
The Layer 1 handshake is tested, not only IDL messages.

### I-042 · Warm-startup figures are measurement targets, never public guarantees
- Baseline: §34, §54, §57
- Enforced by: none
- Status: stated
The Terminal and Editor warm-startup target in B-016 is a measurement target and is never a guarantee in public material.

### I-043 · Native development environments do not require Docker, a Linux VM or overlayfs
- Baseline: §35
- Enforced by: none
- Status: stated
They map immutable package objects and storage snapshots. Environment.yaml plus lock is reproducible.

### I-044 · No native filesystem or object store before 1.0
- Baseline: §26, §57
- Enforced by: none
- Status: stated
Storage semantics sit on a mature Linux filesystem. CDDL ZFS is excluded from in-kernel use.

### I-045 · No native GPU driver stack before 1.0
- Baseline: §39, §56.1, §57
- Enforced by: none
- Status: stated
DRM/KMS and Mesa are retained. Native graphics APIs do not expose DRM ioctls or device nodes to applications.

### I-046 · Wasm is not the native machine ABI
- Baseline: §13, §57
- Enforced by: none
- Status: stated
Maximum-performance native software compiles to machine code. The Wasm runtime stays in userspace on license grounds.

### I-047 · Distribution is not a kernel concern
- Baseline: §43, §57
- Enforced by: none
- Status: stated
Interface design permits in-process, same-machine, VM and remote transports without making the kernel distributed. Remote use still honours capabilities, identity, encryption, latency and explicit user policy.

### I-048 · Wayland and X11 are not the native UI API
- Baseline: §41, §57
- Enforced by: none
- Status: stated
X11 primary selection exists only inside the personality bridge.

### I-049 · Linux syscalls are not native APIs
- Baseline: §3, §57
- Enforced by: none
- Status: stated
A syscall-filter verifies that native components cannot invoke Linux syscalls.

### I-050 · No impossible speedups in public communication
- Baseline: §54, §57
- Enforced by: none
- Status: stated

### I-051 · No AI before the semantic object model
- Baseline: §44, §57
- Enforced by: none
- Status: stated
An AI-broker task depends on a done semantic-registry task. The assistant is not an omnipotent privileged process.

### I-052 · Benchmark priorities tie to user-perceivable outcomes
- Baseline: §54, §57
- Enforced by: none
- Status: stated
Do not optimize trivia while ignoring desktop UX.

### I-053 · Upstream mergeability does not outrank the architecture
- Baseline: §57
- Enforced by: none
- Status: stated

### I-054 · Hardware support is not broken without need
- Baseline: §2, §55, §57
- Enforced by: none
- Status: stated
When native-platform work conflicts with keeping Linux hardware support working, hardware support wins until an accepted decision says otherwise. DRM, PCI, USB, NVMe, networking and ACPI are not destabilised while the native platform is built.

### I-055 · Keep the kernel ABI minimal
- Baseline: §65
- Enforced by: none
- Status: stated
High-level semantics live in userspace services.

### I-056 · Capabilities, async, ownership transfer and versioning are fundamental
- Baseline: §65
- Enforced by: none
- Status: stated
Every object reference crossing the ABI is a Capability. Ownership transfer is the ABI default for MemoryObjects. The ABI includes version identification from V0.

### I-057 · The native ABI does not expose implementation details
- Baseline: §65, §38
- Enforced by: none
- Status: stated
Not task_struct, mm_struct, cgroups, namespaces or page-table layout. The ABI is architecture-neutral in its definitions.

### I-058 · Preserve escape hatches for future hardware
- Baseline: §8, §38, §65
- Enforced by: none
- Status: stated
Capability representation, memory model and ComputeDevice. Do not freeze the OS around current x86-64 security limitations.

### I-059 · Design the ABI to survive decades
- Baseline: §65, §66
- Enforced by: none
- Status: stated
A Layer 1 change ships with a deprecation strategy. High-level ideas live in Layer 3 and Layer 4 so they are not frozen forever.

### I-060 · Security comes from authority design, not deny lists
- Baseline: §9, §67
- Enforced by: none
- Status: stated
Blocklist-based sandboxing is not the primary security mechanism.

### I-061 · No performance claim without a harness report
- Baseline: §54, §57
- Enforced by: none
- Status: stated
No documentation, release note or announcement may claim performance superiority, or state a performance number, without a report produced by the harness named on the cited B-ID.

### I-062 · Immutable state beats mutation wherever practical
- Baseline: §67, §28, §30
- Enforced by: none
- Status: stated

### I-063 · Ownership transfer beats copying
- Baseline: §16, §17, §67
- Enforced by: none
- Status: stated
SDK APIs default to move semantics for MemoryObjects. Payload bytes do not move when avoidable.

### I-064 · Hardware diversity is abstracted without hiding cost
- Baseline: §37, §67
- Enforced by: none
- Status: stated
Abstractions expose locality and cost information.

### I-065 · Do not chase novelty for its own sake
- Baseline: §67, §68
- Enforced by: none
- Status: stated
The acceptance test for borrowing an idea is whether it produces the strongest coherent system. A design whose litmus answer is history is reconsidered and recorded.

### I-066 · Do not optimize an expensive abstraction that can be eliminated
- Baseline: §67
- Enforced by: none
- Status: stated

### I-067 · Kernel dependency licenses are GPLv2-compatible
- Baseline: §5.1
- Enforced by: none
- Status: stated
Apache-2.0-only, CDDL and other GPLv2-incompatible code are forbidden in the kernel. Vendored Rust crates are taken under their MIT option. Enforced in CI.

### I-068 · Userspace allowlist excludes AGPL, SSPL and BUSL from the default image
- Baseline: §50
- Enforced by: none
- Status: stated

### I-069 · Immutable packages preserve LGPL relinking rights
- Baseline: §28
- Enforced by: none
- Status: stated

### I-070 · Windows personality follows a clean-room policy
- Baseline: §48
- Enforced by: none
- Status: stated
No disassembly of Microsoft binaries, no contributors exposed to leaked Windows source, no bundling of Microsoft redistributables or fonts.

### I-071 · Compatibility work never circumvents DRM or anti-tamper
- Baseline: §48, §56.2
- Enforced by: none
- Status: stated
Kernel-level anti-cheat bypass is a non-goal. Blocked titles are documented.

### I-072 · Personality POSIX authority is bounded by the enclosing component
- Baseline: §3, §9.1, §46
- Enforced by: none
- Status: stated
A Linux process cannot reach a resource its component was not granted, regardless of uid or file mode. Native components never depend on the Linux personality.

### I-073 · Installer encrypts user data by default
- Baseline: §63
- Enforced by: none
- Status: stated
Opt-out, not opt-in.

### I-074 · Target hardware has IOMMU, TPM 2.0 and Secure Boot enrolment
- Baseline: §55, §62
- Enforced by: none
- Status: stated
IOMMU is enabled and DMA-capable devices sit behind it. Thunderbolt and USB4 require authorisation before PCIe tunnelling.

### I-075 · A compositor crash while locked restarts locked
- Baseline: §32, §40
- Enforced by: none
- Status: stated

### I-076 · Each Windows application is its own component and prefix
- Baseline: §48, §49
- Enforced by: none
- Status: stated
One compromised title cannot read another application's files or credentials.

### I-077 · Crash dumps never contain disk keys or unlocked secrets
- Baseline: §51, §63
- Enforced by: none
- Status: stated

### I-078 · Stable hardware identifiers are not ambient
- Baseline: §9.1
- Enforced by: none
- Status: stated
Machine ID, TPM EK, disk serials and MAC addresses require a capability. Per-application derived identifiers are provided where needed.

### I-079 · Boot is UEFI-only
- Baseline: §5.1
- Enforced by: none
- Status: stated
No legacy BIOS or CSM for 1.0. The kernel command line is part of the signed generation.

### I-080 · Signature fields are reserved in manifests before the first install
- Baseline: §28, §30
- Enforced by: none
- Status: stated

### I-081 · Each VM is a capability-scoped Component
- Baseline: §36, §69
- Enforced by: none
- Status: stated
Host access is granted through capabilities visible in `os inspect`.

### I-082 · New kernel and platform code is Rust unless a decision exempts a file
- Baseline: §50, §51
- Enforced by: none
- Status: stated
`cargo clippy -D warnings` and rustfmt are clean in CI. Unsafe authority is minimised and inventoried.

### I-083 · The glyph atlas is a read-only object minted by a text service
- Baseline: §41, §51
- Enforced by: none
- Status: stated
A shared writable atlas is a cross-domain channel and a font-parsing attack surface.

### I-084 · Explicit GPU synchronization is mandatory
- Baseline: §39, §40
- Enforced by: none
- Status: stated
No implicit-sync path for native surfaces.

### I-085 · Screen capture requires an explicit capability
- Baseline: §9.1, §40
- Enforced by: none
- Status: stated

### I-086 · Kernel live-patching is a non-goal
- Baseline: §30, §56.4
- Enforced by: none
- Status: stated
System generations plus reboot are the update model.

### I-087 · No calendar dates in roadmap sources the tool reads
- Baseline: §70
- Enforced by: none
- Status: stated
Sequence is expressed by Milestone and `Depends on`.

### I-088 · Milestone gates cite B-IDs and C-IDs, never restate numbers
- Baseline: §54
- Enforced by: none
- Status: stated

### I-089 · Do not build a custom compiler or forked LLVM
- Baseline: §50
- Enforced by: none
- Status: stated
Only minimal, upstream-bound patches against rustc and LLVM.

### I-090 · Do not support native builds from Windows or macOS hosts before 1.0
- Baseline: §50
- Enforced by: none
- Status: stated
A containerised Linux build environment is provided for those hosts.

### I-091 · Formal security certifications are a non-goal for 1.0
- Baseline: §51
- Enforced by: none
- Status: stated
Common Criteria and FIPS 140 are not 1.0 gates.

### I-092 · Enterprise directory, MDM and multi-seat are non-goals for 1.0
- Baseline: §63
- Enforced by: none
- Status: stated
Active Directory, Group Policy, fleet management, kiosk and guest sessions are out. The identity service's authority model must still allow them later.

### I-093 · Casting, NFC, WWAN and MIDI are post-1.0
- Baseline: §62
- Enforced by: none
- Status: stated
They appear in the published non-goal list and live in LATER.

### I-094 · V0 is not a usable desktop
- Baseline: §59
- Enforced by: none
- Status: stated
V0 exit criteria include no UI or desktop deliverable.

### I-095 · Do not promise universal PC compatibility
- Baseline: §62
- Enforced by: none
- Status: stated
A published Hardware Compatibility List is the support surface. Unlisted hardware is unsupported; the installer warns.

### I-096 · Compatibility is treated as a product
- Baseline: §46, §49, §56.3, §56.5
- Enforced by: none
- Status: stated
Own owner, conformance suite, UX acceptance and release notes. Ecosystem gaps are filled through personalities before native alternatives exist. Linux and Windows software integrate with native launcher, taskbar, clipboard, audio, file chooser, notifications and scaling.

### I-097 · Driver transition is incremental per device class
- Baseline: §55
- Enforced by: none
- Status: stated
No flag-day replacement of any driver class.

### I-098 · Retain kselftests for every retained subsystem
- Baseline: §6, §55
- Enforced by: none
- Status: stated

### I-099 · Merge queue on main, no direct pushes
- Baseline: §70
- Enforced by: none
- Status: stated

### I-100 · Do not fossilize the ABI around current hardware
- Baseline: §38, §70
- Enforced by: none
- Status: stated
The 1.0 gate includes a fossilization review of the ABI, MemoryObject and ComputeDevice so later hardware does not require a major-version break.

### I-101 · Native kernel Rust never panics on user-controlled input and allocates fallibly
- Baseline: §50, §51
- Enforced by: none
- Status: stated
A Rust panic inside the kernel is a kernel crash. New native kernel code uses fallible allocation, returns typed errors for every user-reachable failure, and is linted against `unwrap`, `expect`, indexing panics and infallible allocation outside boot-time initialisation.
