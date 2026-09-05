# Decisions

One architectural decision record per file, `D-NNNN-<slug>.md`, created from `TEMPLATE.md`. The number is permanent; the slug may change.

## How a decision is made

1. A task of Type `adr` is opened in the workstream that owns the question, with `Decision: D-NNNN`. Its acceptance criteria name the options to evaluate and the review required; its Verification has a `Review:` line.
2. The decision file is created with `Status: proposed`, the adr task in `Task:`, and at least two options under `## Options`, each with a one-line summary, consequences and evidence (spike or benchmark report paths).
3. Discussion happens on the pull request. Spikes cited under `Spikes:` must be done with reports before the decision can be accepted; a decision listing L1 or L2 surfaces cites at least one done spike per surface, and L1 also cites a benchmark report.
4. Acceptance is one change: tick the adr task's criteria, set the file to `accepted` (or `rejected`, which is also a completed decision), add `decision:D-NNNN` to the task's Evidence, and mark the task done with an independent `Verified by`.

Tasks that depend on a decision list the **adr task ID** in `Depends on`, never the D-ID. `roadmap show` inlines the Decision and Consequences sections of every adr task in a task's closure.

## Rules

- Exactly one adr task per decision and one decision per adr task. The tool checks the coupling in both directions and that the task's status and the file's status agree.
- Accepted decisions are immutable except for `Superseded by`. To revise, open a new adr task and a new file with `Supersedes: D-NNNN`; the old file becomes `superseded` and its task stays done. If the superseding decision touches a frozen surface, the surface returns to `prototyped`.
- `Layer` is the stability layer of the surfaces fixed (§66), or `none` for process, scoping and licensing decisions.
- `Revisit when` is a condition, never a date.
- Roadmap-process decisions (grammar, workstream splits, baseline amendments) are GOV adr tasks; a change to `BASELINE.md` carries the commit trailer `Roadmap-Decision: D-NNNN`.

Full grammar and coupling rules: `CONVENTIONS.md` section 10.

## Index

<!-- roadmap:generated:begin index -->
| ID | Title | Status | Task | Surfaces |
| --- | --- | --- | --- | --- |
| D-0001 | Roadmap repository process | accepted | GOV-004 | none |
| D-0002 | Decide the 1.x stability declaration superseding the freeze ADR with stable for 1.x | proposed | ABI-053 | none |
| D-0003 | Decide the binding substrate: C-compatible ABI header plus IDL-generated language stubs | proposed | ABI-007 | none |
| D-0004 | Decide the Layer 1 and platform deprecation process: announcement, overlap, detection | proposed | ABI-045 | none |
| D-0005 | Decide the Native ABI entry mechanism and the maximum count of kernel entry points | proposed | ABI-008 | S-002 |
| D-0006 | Decide the Operation result error model: typed enum per kind or uniform error Object | proposed | ABI-009 | S-004 |
| D-0007 | Decide Capability handle representation: dense index, sparse id or sealed value | proposed | ABI-010 | none |
| D-0008 | Decide whether ABI headers carry a syscall-note-style exception for native programs | accepted | ABI-029 | none |
| D-0009 | Decide the Layer 1 freeze: accept the freeze ADR over the reviewed candidate set | proposed | ABI-049 | none |
| D-0010 | Decide Layer 1 scope: enumerate L1 primitives and place every concept in L1 or L2 | proposed | ABI-011 | none |
| D-0011 | Decide whether Layer 2 Interface stability applies at V1 or only at 1.0 | proposed | ABI-037 | none |
| D-0012 | Decide Object-Operation dispatch with async-only submission and move semantics | proposed | ABI-012 | none |
| D-0013 | Decide which Object<T> types live in the kernel and the kernel-residency criteria | proposed | ABI-013 | none |
| D-0014 | Decide whether the Operation kind set is a closed kernel enum or extensible registry | proposed | ABI-014 | none |
| D-0015 | Decide how user space identifies an Operation: Capability, ring index or opaque handle | proposed | ABI-015 | none |
| D-0016 | Decide the Layer 1 version identification and feature-negotiation scheme | proposed | ABI-016 | S-011 |
| D-0017 | Decide assistive-technology access as Capability<AccessibilityTree> with redaction | proposed | ACC-007 | none |
| D-0018 | Decide the assistive-technology protocol: AT-SPI compatible, native, or both | proposed | ACC-001 | none |
| D-0019 | Decide whether semantic actions and accessibility actions share one tree | proposed | ACC-008 | none |
| D-0020 | Decide the accessibility tree model shared by the toolkit and semantic interfaces | proposed | ACC-002 | S-017 |
| D-0021 | Decide the text-to-speech engine for the native screen reader | proposed | ACC-009 | none |
| D-0022 | Decide browser strategy for 1.0 and the native WebView Component | proposed | APP-019 | none |
| D-0023 | Decide the first-party application set native versus bundled at 1.0 | proposed | APP-051 | none |
| D-0024 | Decide which foreign Package formats map to Personality launches | proposed | APP-052 | none |
| D-0025 | Decide IDE strategy for 1.0 and the criteria for a native port | proposed | APP-020 | none |
| D-0026 | Decide the shared PDF renderer for viewer, thumbnails and print preview | proposed | APP-053 | none |
| D-0027 | Decide status-tray policy: StatusNotifierItem compatibility versus none | proposed | APP-054 | none |
| D-0028 | Decide native AudioStream service versus PipeWire-as-native | proposed | AUD-002 | S-025 |
| D-0029 | Decide which audio device classes run in user space | proposed | AUD-016 | none |
| D-0030 | Decide the blocking performance merge-Gate policy | proposed | BEN-033 | none |
| D-0031 | Decide benchmark methodology and target-kind policy | accepted | BEN-007 | none |
| D-0032 | Decide the visible-UI measurement boundary | accepted | BEN-016 | none |
| D-0033 | Decide the top-level build orchestrator for kernel and userspace | accepted | BLD-002 | none |
| D-0034 | Decide the CI platform with self-hosted KVM runners | accepted | BLD-003 | none |
| D-0035 | Decide linker, LTO scope and PGO policy for kernel and userspace | proposed | BLD-039 | none |
| D-0036 | Decide LLVM/Clang as the sole C compiler and reject a custom compiler | accepted | BLD-004 | none |
| D-0037 | Decide repository topology before a second repository exists | accepted | BLD-005 | none |
| D-0038 | Decide SBOM format for Packages and SystemGenerations | proposed | BLD-054 | none |
| D-0039 | Decide a semantic-Interface GUI test harness over pixel scripting | proposed | BLD-018 | none |
| D-0040 | Decide the anti-rollback policy for SystemGenerations older than a security watermark | proposed | BOOT-040 | none |
| D-0041 | Decide what 'boot succeeded' means and which Component may clear the boot counter | proposed | BOOT-020 | none |
| D-0042 | Decide the bootloader: systemd-boot, GRUB or a native Rust UEFI stub | proposed | BOOT-008 | none |
| D-0043 | Decide how early boot locates the content store and the selected SystemGeneration | proposed | BOOT-009 | none |
| D-0044 | Decide the trusted time source policy before network time is available | proposed | BOOT-021 | none |
| D-0045 | Decide ESP policy: reuse the existing OEM ESP or create a dedicated ESP for Generation entries | proposed | BOOT-029 | none |
| D-0046 | Decide atomicity of kernel, driver and firmware updates within a SystemGeneration | proposed | BOOT-030 | none |
| D-0047 | Decide whether GPLv3 components may appear in the boot chain and how Installation Information is met | proposed | BOOT-010 | none |
| D-0048 | Decide Secure Boot distribution: Microsoft-signed shim, enrolled project keys, or both | proposed | BOOT-031 | none |
| D-0049 | Decide UEFI-only boot on x86-64 with no legacy BIOS/CSM support through 1.0 | accepted | BOOT-003 | none |
| D-0050 | Decide whether each SystemGeneration boots as one signed UKI or separately verified parts | proposed | BOOT-011 | none |
| D-0051 | Decide V0 boots Linux init from a retained initramfs with native Components beside it | accepted | BOOT-004 | none |
| D-0052 | Decide Capability persistence across Component restart and reboot | proposed | CAP-020 | none |
| D-0053 | Decide how Capability unforgeability survives machine boundaries | proposed | CAP-047 | none |
| D-0054 | Decide explicit grant sources replacing ambient permissions | proposed | CAP-007 | none |
| D-0055 | Decide the userspace Capability<T> handle representation and table design | proposed | CAP-008 | none |
| D-0056 | Decide the Capability<T> and MemoryObject mapping onto hardware capabilities | proposed | CAP-033 | none |
| D-0057 | Decide the ABI invariants required for hardware-assisted Capability enforcement | proposed | CAP-021 | none |
| D-0058 | Decide revocation semantics: eager vs lazy, in-flight Operations, cost bounds | proposed | CAP-009 | none |
| D-0059 | Decide rights and transfer-rights encoding including Admin authority | proposed | CAP-010 | S-003 |
| D-0060 | Decide how a Component obtains its initial and later Capabilities | proposed | CAP-022 | none |
| D-0061 | Decide whether every Component owns a hardware address space | proposed | CMP-021 | S-007 |
| D-0062 | Decide what replaces PID, parent/child, exit status and process groups | accepted | CMP-006 | none |
| D-0063 | Decide static manifest graphs versus dynamic child instantiation | proposed | CMP-022 | S-019 |
| D-0064 | Decide Component plus ResourceDomain as the native isolation model | accepted | CMP-007 | none |
| D-0065 | Decide the order and equivalence tests for replacing the Component wrapper | proposed | CMP-042 | none |
| D-0066 | Decide Component panic, abort and typed exit-cause semantics | accepted | CMP-008 | none |
| D-0067 | Decide how Personality processes map onto Components | proposed | CMP-036 | none |
| D-0068 | Decide the native Component spawn primitive that replaces fork and exec | accepted | CMP-009 | none |
| D-0069 | Decide the Phase A Component implementation strategy | proposed | CMP-010 | none |
| D-0070 | Decide the documentation taxonomy and ownership | proposed | DOC-008 | none |
| D-0071 | Decide the documentation toolchain, search and snapshots | proposed | DOC-009 | none |
| D-0072 | Decide how environment endpoints are granted without ambient network | proposed | ENV-006 | none |
| D-0073 | Decide whether DevelopmentEnvironment is kernel or userspace | proposed | ENV-007 | none |
| D-0074 | Decide environment.yaml schema versus Package manifest profile | proposed | ENV-008 | S-021 |
| D-0075 | Decide how environment services are hosted and packaged | proposed | ENV-009 | none |
| D-0076 | Decide compositor architecture: monolithic or split display/scene/input | proposed | GFX-012 | S-024 |
| D-0077 | Decide compositor infrastructure reuse versus build-anew | proposed | GFX-013 | none |
| D-0078 | Decide explicit GPU synchronisation as the only path for native Surfaces | proposed | GFX-014 | none |
| D-0079 | Decide the compositor frame scheduling model | proposed | GFX-015 | none |
| D-0080 | Decide the GPU userspace strategy from the Mesa-behind-capabilities Spike | proposed | GFX-016 | none |
| D-0081 | Decide the HDR and colour management pipeline | proposed | GFX-063 | none |
| D-0082 | Decide the GPU API native applications render with | proposed | GFX-017 | none |
| D-0083 | Decide the NVIDIA support stance for 1.0 | proposed | GFX-064 | none |
| D-0084 | Decide the proprietary GPU kernel driver policy | proposed | GFX-047 | none |
| D-0085 | Decide whether RDP/VNC clients and a remote-desktop server are in 1.0 scope | proposed | GFX-089 | none |
| D-0086 | Decide compositor rendering backend policy: Vulkan-only or Vulkan plus GL | proposed | GFX-018 | none |
| D-0087 | Decide which compositor state survives restart and where it is checkpointed | proposed | GFX-019 | none |
| D-0088 | Decide whether Wayland is served by the compositor or by a bridge Component | proposed | GFX-020 | none |
| D-0089 | Decide code hosting forge and repository layout | accepted | GOV-001 | none |
| D-0090 | Decide codec and proprietary-font shipping and patent policy | proposed | GOV-020 | none |
| D-0091 | Decide redistribution licensing for Personality software wrappers | proposed | GOV-049 | none |
| D-0092 | Decide contributor licensing, copyright holder and DCO or CLA | accepted | GOV-002 | none |
| D-0093 | Decide open-source-steward versus manufacturer status under CRA | proposed | GOV-070 | none |
| D-0094 | Decide the license for published benchmark and HCL datasets | proposed | GOV-040 | none |
| D-0095 | Decide the documentation license and translation terms | proposed | GOV-021 | none |
| D-0096 | Assess export-control and cryptography distribution obligations | proposed | GOV-050 | none |
| D-0097 | Decide firmware blob redistribution for official versus non-free | accepted | GOV-022 | none |
| D-0098 | Decide font shipping versus metric-compatible substitutes | accepted | GOV-009 | none |
| D-0099 | Decide the funding model and publish infrastructure-cost finances | proposed | GOV-041 | none |
| D-0100 | Adopt the governance charter for decisions and maintainers | proposed | GOV-023 | none |
| D-0101 | Decide legal-entity form able to hold marks and signing keys | proposed | GOV-024 | none |
| D-0102 | Decide the license firewall and outbound project licenses | accepted | GOV-003 | none |
| D-0103 | Decide whether Linux appears in product naming | proposed | GOV-051 | none |
| D-0104 | Decide OEM partnerships or hardware certification versus later | proposed | GOV-078 | none |
| D-0105 | Decide Open Invention Network membership and a patent pledge | proposed | GOV-052 | none |
| D-0106 | Decide the 1.0 portability commitment as x86-64 only shipping | proposed | GOV-025 | none |
| D-0107 | Decide redistribution policy for third-party Linux and Windows software | proposed | GOV-053 | none |
| D-0108 | Decide Package-repository developer agreement, content and DMCA terms | proposed | GOV-054 | none |
| D-0109 | Decide governance of the standard Semantic Interface catalogue | proposed | GOV-042 | none |
| D-0110 | Decide opt-in usage telemetry policy apart from crash reporting | proposed | GOV-055 | none |
| D-0111 | Decide trademark usage, derivative branding and compatible-with claims | proposed | GOV-056 | none |
| D-0112 | Decide vendor NDA policy for hardware documentation | proposed | GOV-026 | none |
| D-0113 | Decide whether to run a vulnerability reward program | proposed | GOV-079 | none |
| D-0114 | Decide the Workstream-split procedure at the size warning | proposed | GOV-043 | none |
| D-0115 | Decide GPU ComputeDevice backend among Vulkan, DRM, or deferral | proposed | HET-003 | none |
| D-0116 | Decide portable workload representation for heterogeneous dispatch | proposed | HET-004 | none |
| D-0117 | Decide ComputeDevice enumeration ABI and open-ended class taxonomy | proposed | HET-001 | S-028 |
| D-0118 | Decide how ComputeDevice relates to ComputeQueue | proposed | HET-011 | none |
| D-0119 | Decide Bluetooth host placement and required profiles | proposed | HW-040 | none |
| D-0120 | Decide user-space driver access: VFIO, UIO or native Device DMA | proposed | HW-006 | none |
| D-0121 | Decide criteria classifying each driver as inherited, native or rewritten | proposed | HW-016 | none |
| D-0122 | Decide pragmatic driver residency over microkernel purity | accepted | HW-002 | none |
| D-0123 | Decide Capability<InputDevice> rights with no ambient device nodes | proposed | HW-007 | none |
| D-0124 | Decide whether IOMMU is required for user-space drivers and DMA | proposed | HW-017 | none |
| D-0125 | Decide NVIDIA support and Secure Boot handling of proprietary modules | proposed | HW-018 | none |
| D-0126 | Decide driverless-first native printing with PDF spool and CUPS in LNX | proposed | HW-041 | none |
| D-0127 | Decide 1.0 sensor support per device class in or out of scope | proposed | HW-042 | none |
| D-0128 | Decide Hardware Compatibility List tiers and promotion criteria | proposed | HW-043 | none |
| D-0129 | Decide the V1 through V2 Reference machine list and security criteria | accepted | HW-003 | none |
| D-0130 | Decide policy for third-party user-space drivers and firmware packages | proposed | HW-082 | none |
| D-0131 | Decide public vendor priorities for purchase guidance | proposed | HW-044 | none |
| D-0132 | Decide desktop-usable boot-success health criteria | proposed | INS-006 | none |
| D-0133 | Decide installer encryption default with opt-out | proposed | INS-007 | none |
| D-0134 | Decide hardware clock UTC versus localtime for dual-boot | proposed | INS-022 | none |
| D-0135 | Decide installer disk layout, wipe and dual-boot policy | proposed | INS-008 | none |
| D-0136 | Decide Linux home adopt-in-place versus copy | proposed | INS-023 | none |
| D-0137 | Decide dual-boot shared data partition format | proposed | INS-024 | none |
| D-0138 | Decide client update orchestration, metered links and deferral | proposed | INS-009 | none |
| D-0139 | Decide whether the kernel offers synchronous call with time-slice donation beside async send | proposed | IPC-001 | none |
| D-0140 | Decide which Channel syscalls become Layer 1 freeze candidates for SDK v1 | proposed | IPC-041 | none |
| D-0141 | Decide the Interface-evolution rules for Layer 2 Interfaces (prototyped state) | proposed | IPC-002 | none |
| D-0142 | Select the small-message fast-path technique from measured prototypes | proposed | IPC-003 | none |
| D-0143 | Freeze the Channel Layer 1 ABI Surface | proposed | IPC-064 | none |
| D-0144 | Freeze the Layer 2 Interface-evolution rules for SDK v1 | proposed | IPC-042 | S-014 |
| D-0145 | Decide whether IDL-generated code is committed or generated at build time | proposed | IPC-004 | none |
| D-0146 | Decide that IDL compiler output is owned by its user with no copyleft obligation | proposed | IPC-005 | none |
| D-0147 | Decide how the IDL language itself is versioned | proposed | IPC-055 | none |
| D-0148 | Decide the IDL: adopt WIT, FIDL, Cap'n Proto schema or design new | proposed | IPC-006 | none |
| D-0149 | Decide the relationship between the native IDL and WIT | proposed | IPC-022 | none |
| D-0150 | Decide service naming and discovery: kernel-held directory or user-space broker | proposed | IPC-023 | none |
| D-0151 | License IDL files and the ABI specification under a permissive spec license with patent non-assert | proposed | IPC-024 | none |
| D-0152 | Decide the pluggable transport abstraction behind generated stubs | proposed | IPC-025 | none |
| D-0153 | Decide how Capabilities and handles cross a VM transport boundary | proposed | IPC-056 | none |
| D-0154 | Decide the typed-message wire format and inline-payload threshold | proposed | IPC-007 | none |
| D-0155 | Decide which kernel evolution phase is required at 1.0 | proposed | KRN-050 | none |
| D-0156 | Decide eBPF's native role and the Linux Personality's bpf() exposure | proposed | KRN-024 | none |
| D-0157 | Decide kernel-core vs user-space service boundary and the criteria for moving one | accepted | KRN-001 | none |
| D-0158 | Decide kernel strategy: Linux fork vs new microkernel vs Linux-as-hypervisor | accepted | KRN-002 | none |
| D-0159 | Decide how KVM is exposed natively as Capability<VirtualMachine> | proposed | KRN-025 | none |
| D-0160 | Decide that kernel live-patching is a non-goal in favour of generations plus reboot | proposed | KRN-026 | none |
| D-0161 | Decide module signing under Secure Boot for out-of-tree, GPU and local modules | proposed | KRN-027 | none |
| D-0162 | Decide the licence for new native kernel code | accepted | KRN-003 | none |
| D-0163 | Decide out-of-tree module policy: GPL-only native exports and taint semantics | proposed | KRN-028 | none |
| D-0164 | Decide Phase D entry: when full upstream merges stop being mandatory | proposed | KRN-042 | none |
| D-0165 | Decide kernel Rust toolchain pinning relative to the Rust-for-Linux minimum | accepted | KRN-004 | none |
| D-0166 | Decide the upstream Linux tree and LTS series the fork is cut from | accepted | KRN-005 | none |
| D-0167 | Decide the upstream-first policy for the hardware layer and Rust abstractions | accepted | KRN-006 | none |
| D-0168 | Decide upstream tracking: rebase vs merge and cadence per divergence phase | accepted | KRN-007 | none |
| D-0169 | Decide lab site and the remote power, console and capture stack | accepted | LAB-002 | none |
| D-0170 | Decide lab job-scheduler family and unbootable-machine recovery | proposed | LAB-005 | none |
| D-0171 | Decide the Linux Personality container engine Surface | proposed | LNX-012 | none |
| D-0172 | Decide the default Capability bundle for Linux apps | proposed | LNX-013 | none |
| D-0173 | Decide /dev, sysfs and udev coexistence with native drivers | proposed | LNX-014 | none |
| D-0174 | Decide whether ia32 emulation is retained | proposed | LNX-015 | none |
| D-0175 | Decide incremental native-Interface adoption | proposed | LNX-062 | none |
| D-0176 | Decide how native applications opt into a Personality | proposed | LNX-016 | none |
| D-0177 | Decide first-class Linux packaging formats | proposed | LNX-017 | none |
| D-0178 | Decide Linux Personality depth and translation phase | proposed | LNX-003 | S-030 |
| D-0179 | Decide xdg-desktop-portal as the native grant bridge | proposed | LNX-018 | none |
| D-0180 | Decide the POSIX path view of native storage | proposed | LNX-019 | none |
| D-0181 | Decide X11 primary selection stays inside the bridge | proposed | LNX-020 | none |
| D-0182 | Decide terminal-session authority for Linux programs | proposed | LNX-022 | none |
| D-0183 | Decide source-built versus redistributed Linux userland | proposed | LNX-023 | none |
| D-0184 | Decide glibc /usr/lib interoperation with Packages | proposed | LNX-024 | none |
| D-0185 | Decide Wayland hosting and X11 via Xwayland | proposed | LNX-004 | none |
| D-0186 | Decide Camera service model over V4L2 and libcamera | proposed | MED-001 | none |
| D-0187 | Decide VA-API versus Vulkan Video for hardware codecs | proposed | MED-004 | none |
| D-0188 | Decide native media pipeline versus GStreamer or FFmpeg | proposed | MED-007 | S-036 |
| D-0189 | Decide Widevine L3 Personality path and native CDM non-goals | proposed | MED-023 | none |
| D-0190 | Decide the MemoryObject backing-provider abstraction for future memory media | proposed | MEM-017 | S-006 |
| D-0191 | Decide how borrowing lifetimes are enforced across Component boundaries | proposed | MEM-018 | none |
| D-0192 | Decide the MemoryObject sharing coherence model across CPUs and devices | accepted | MEM-001 | none |
| D-0193 | Decide whether dma-buf backs DMA- and GPU-compatible MemoryObjects | proposed | MEM-019 | none |
| D-0194 | Decide encrypted MemoryObject key ownership and hardware encryption | proposed | MEM-045 | none |
| D-0195 | Decide mapping of Capability<File> into a MemoryObject | proposed | MEM-020 | none |
| D-0196 | Decide the per-Component page-table and huge-page policy for MemoryObjects | proposed | MEM-021 | none |
| D-0197 | Decide the MemoryObject kernel implementation basis | proposed | MEM-002 | none |
| D-0198 | Decide the MemoryObject locality and placement attribute model | proposed | MEM-034 | none |
| D-0199 | Decide whether Ownership transfer is kernel-enforced or advisory | proposed | MEM-003 | none |
| D-0200 | Decide whether to ship a native typed remote shell after 1.0 | proposed | NET-040 | none |
| D-0201 | Decide the native TLS library and how it consumes the CA trust store | proposed | NET-005 | none |
| D-0202 | Decide per-application network Capability granularity and inbound firewall | proposed | NET-006 | S-026 |
| D-0203 | Decide NET baseline-gap scope: preserved stack versus native objects | proposed | NET-007 | none |
| D-0204 | Decide whether NetworkConnection wraps the kernel TCP/IP stack | proposed | NET-008 | none |
| D-0205 | Decide whether 1.0 ships an SMB server and where it is hosted | proposed | NET-029 | none |
| D-0206 | Decide the Wi-Fi supplicant: iwd, wpa_supplicant or native Rust | proposed | NET-009 | none |
| D-0207 | Decide the Component crash capture format | proposed | OBS-029 | none |
| D-0208 | Decide the persistent journal record format and retention model | proposed | OBS-030 | none |
| D-0209 | Decide who may trace and inspect which Components | proposed | OBS-014 | none |
| D-0210 | Decide the trace event schema and export format | proposed | OBS-015 | S-035 |
| D-0211 | Decide the tracing substrate and its measured overhead ceiling | proposed | OBS-003 | S-010 |
| D-0212 | Decide whether application-state restore is a 1.0 goal or non-goal | proposed | PKG-069 | none |
| D-0213 | Decide the content hash algorithm and chunking strategy for the store | proposed | PKG-005 | none |
| D-0214 | Decide dependency resolution semantics and lockfile location | proposed | PKG-006 | none |
| D-0215 | Decide how security fixes reach a library pinned by many Packages | proposed | PKG-046 | none |
| D-0216 | Decide what is excluded from a SystemGeneration and how mutable state is separated | proposed | PKG-007 | none |
| D-0217 | Decide how a SystemGeneration is materialised on disk | proposed | PKG-008 | none |
| D-0218 | Decide that Package mutation is replaced by immutable Packages and SystemGenerations | proposed | PKG-009 | none |
| D-0219 | Decide how immutable Packages preserve LGPL relinking rights | proposed | PKG-010 | none |
| D-0220 | Decide the Package manifest schema shape and its Layer 2 evolution rules | proposed | PKG-011 | S-018 |
| D-0221 | Decide the on-disk and on-wire Package format and its relation to the store | proposed | PKG-012 | none |
| D-0222 | Decide that global dependency installation is replaced by per-Package dependency objects | proposed | PKG-013 | none |
| D-0223 | Decide how Linux and Windows compatibility applications are packaged immutably | proposed | PKG-047 | none |
| D-0224 | Decide whether SystemGeneration switches may apply without reboot | proposed | PKG-070 | none |
| D-0225 | Decide which state classes are restorable at each Milestone and in scope for 1.0 | proposed | PKG-048 | none |
| D-0226 | Decide running-application behaviour when its Package is replaced by a new Generation | proposed | PKG-049 | none |
| D-0227 | Decide selective restore semantics and how partial restore avoids inconsistency | proposed | PKG-071 | none |
| D-0228 | Decide the Content-addressed store layout for Packages and SystemGenerations | proposed | PKG-014 | none |
| D-0229 | Decide the verified-once launch trust mechanism | proposed | PKG-050 | none |
| D-0230 | Decide verified-once launch trust for cached Package objects | proposed | PKG-045 | none |
| D-0231 | Decide hibernation policy for 1.0 | proposed | PWR-007 | none |
| D-0232 | Decide the Layer 2 power service model over retained ACPI | proposed | PWR-001 | none |
| D-0233 | Decide the suspend state for V1 reference machines | proposed | PWR-002 | none |
| D-0234 | Decide whether the repository accepts publisher prebuilts or rebuilds every Package | proposed | REL-016 | none |
| D-0235 | Decide origin, CDN and volunteer-mirror topology | proposed | REL-024 | none |
| D-0236 | Define hardware support tiers as the HCL unit | proposed | REL-011 | none |
| D-0237 | Decide publisher identity and Package naming | proposed | REL-025 | none |
| D-0238 | Declare fleet-management and paid-app non-goals for 1.0 | proposed | REL-026 | none |
| D-0239 | Define release-readiness gates and freeze policy | proposed | REL-027 | none |
| D-0240 | Decide release, SystemGeneration and Channel versioning | accepted | REL-001 | none |
| D-0241 | Define repository curation and free versus non-free channels | proposed | REL-028 | none |
| D-0242 | Decide repository retention of past generations | proposed | REL-029 | none |
| D-0243 | Decide the repository model and source trust display | proposed | REL-012 | none |
| D-0244 | Decide the signing key hierarchy and custody model | proposed | REL-002 | none |
| D-0245 | Decide Package and SystemGeneration signing scheme | proposed | REL-003 | none |
| D-0246 | Decide release cadence, LTS window and support lifecycle | proposed | REL-053 | none |
| D-0247 | Decide whether releases use a transparency log | proposed | REL-030 | none |
| D-0248 | Define update channels and promotion criteria | proposed | REL-004 | none |
| D-0249 | Decide behaviour on ResourceDomain budget exhaustion and owner reporting | proposed | SCH-016 | none |
| D-0250 | Decide hierarchical versus flat ResourceDomains and budget delegation via Capability | proposed | SCH-002 | none |
| D-0251 | Decide ResourceDomain over cgroup v2 controllers versus native accounting | proposed | SCH-003 | none |
| D-0252 | Decide intent and priority inheritance across Channel handoff | proposed | SCH-017 | none |
| D-0253 | Decide how intents map onto Linux scheduler mechanisms versus a native class | proposed | SCH-004 | none |
| D-0254 | Decide which ResourceDomain and intent surfaces are Layer 1 freeze candidates | proposed | SCH-030 | none |
| D-0255 | Decide SDK language binding order and milestones | proposed | SDK-024 | none |
| D-0256 | Decide POSIX-Personality shell versus a native Object-aware shell | proposed | SDK-025 | none |
| D-0257 | Decide DAP versus a native debugger protocol | proposed | SDK-052 | none |
| D-0258 | Decide the userspace executor shape for the native runtime | proposed | SDK-010 | none |
| D-0259 | Decide the native linking model and reject path-based loaders | proposed | SDK-026 | none |
| D-0260 | Decide how non-Rust bindings map onto Layer 1 and IDL stubs | proposed | SDK-072 | none |
| D-0261 | Decide profiler export format and Task attribution | proposed | SDK-053 | none |
| D-0262 | Decide the license of the native SDK, runtime and language bindings | proposed | SDK-027 | none |
| D-0263 | Decide the Layer 3 SDK semver and deprecation policy | proposed | SDK-054 | none |
| D-0264 | Decide that Rust std lives only as a Layer 3 crate | proposed | SDK-028 | none |
| D-0265 | Define administrator versus standard user | proposed | SEC-013 | none |
| D-0266 | Decide whether an AI assistant is a distinct principal | proposed | SEC-034 | none |
| D-0267 | Decide authority sources and precedence | proposed | SEC-004 | none |
| D-0268 | Decide disk encryption layer and store interaction | proposed | SEC-005 | none |
| D-0269 | Decide user-mediated grant taxonomy | proposed | SEC-007 | none |
| D-0270 | Decide 1.0 multi-user and per-user encryption scope | proposed | SEC-042 | none |
| D-0271 | Declare formal certifications out of scope for 1.0 | proposed | SEC-074 | none |
| D-0272 | Declare multi-seat, guest, kiosk, and enterprise directory out of scope | proposed | SEC-075 | none |
| D-0273 | Decide permission prompt policy against fatigue | proposed | SEC-043 | none |
| D-0274 | Decide remote-Interface Capability, identity, and encryption rules | proposed | SEC-079 | none |
| D-0275 | Decide disk-key eviction on suspend | proposed | SEC-031 | none |
| D-0276 | Decide TPM 2.0 as requirement versus optional | proposed | SEC-050 | none |
| D-0277 | Decide user identity versus Capability roots | proposed | SEC-012 | none |
| D-0278 | Decide Semantic Interface discovery and caller permissioning | proposed | SEM-004 | none |
| D-0279 | Decide where assistant models execute | proposed | SEM-017 | none |
| D-0280 | Decide the Automation rule format | proposed | SEM-018 | none |
| D-0281 | Decide whether BitLocker volumes are readable via user-space dislocker-style support | proposed | STO-072 | none |
| D-0282 | Decide replacing the global namespace with Capability-scoped storage objects | proposed | STO-012 | S-027 |
| D-0283 | Decide the content-hash algorithm, identifier format, chunking and upgrade path | proposed | STO-013 | none |
| D-0284 | Decide when Write and StorageTransaction data is power-loss safe | proposed | STO-038 | none |
| D-0285 | Decide encryption layering across the verified system store and encrypted user data | proposed | STO-039 | none |
| D-0286 | Decide how the storage model degrades on foreign filesystems lacking its metadata | proposed | STO-055 | none |
| D-0287 | Decide GPT partition and volume layout for store, generations, user data, swap and recovery | proposed | STO-014 | none |
| D-0288 | Decide persistent MemoryObject semantics: storage backing, crash consistency, content addressing | proposed | STO-040 | none |
| D-0289 | Decide how an application gains authority to create one new file in a user-chosen place | proposed | STO-015 | none |
| D-0290 | Decide how configuration and application state become versioned restorable objects | proposed | STO-056 | none |
| D-0291 | Decide the native storage-provider Interface for network and cloud Collections | proposed | STO-073 | none |
| D-0292 | Decide the initial Linux filesystem under the native storage layer | proposed | STO-016 | none |
| D-0293 | Decide how the content store maps onto the chosen filesystem without double storage | proposed | STO-017 | none |
| D-0294 | Decide content-store garbage collection: root set, policy and user control | proposed | STO-041 | none |
| D-0295 | Decide three-view mapping of user data across native and personalities | proposed | STO-042 | none |
| D-0296 | Decide the platform type registry behind choose<T>, UserSelected<T> and file.type | proposed | STO-018 | none |
| D-0297 | Decide how user data maps across native, Linux home and Windows profile views | proposed | STO-068 | none |
| D-0298 | Decide monotonic versus wall-clock semantics for Operation deadlines across suspend | proposed | SVC-016 | none |
| D-0299 | Decide the default-application registry and open-by-Capability model | proposed | SVC-017 | none |
| D-0300 | Decide native init versus retained initramfs/systemd for early boot | proposed | SVC-003 | none |
| D-0301 | Decide how a service Component reports readiness and liveness to the supervisor | proposed | SVC-004 | none |
| D-0302 | Decide restart budgets, strategies, backoff and escalation for supervised services | proposed | SVC-005 | none |
| D-0303 | Decide the settings storage model: typed schema-versioned objects with history events | proposed | SVC-006 | none |
| D-0304 | Decide whether to retain chrony or build a native NTP/NTS client | proposed | SVC-018 | none |
| D-0305 | Decide Task cancellation model and resource cleanup | proposed | TSK-003 | none |
| D-0306 | Decide deadline and timestamp representation in the Operation ABI | proposed | TSK-004 | none |
| D-0307 | Decide whether Operations may complete inline at submit and how the ABI signals it | proposed | TSK-005 | none |
| D-0308 | Decide which Operation ABI surfaces become Layer 1 freeze candidates | proposed | TSK-042 | none |
| D-0309 | Decide native expression of termination, cancellation and async notification without signals | accepted | TSK-006 | none |
| D-0310 | Decide how Operation priority relates to ResourceDomain Scheduling intent | proposed | TSK-027 | none |
| D-0311 | Decide Operation submission/completion transport and batching expression | proposed | TSK-007 | none |
| D-0312 | Decide Operation Ownership transfer semantics across Tasks and TaskGroups | proposed | TSK-028 | none |
| D-0313 | Decide how Personality threads map onto native Tasks | proposed | TSK-043 | none |
| D-0314 | Decide whether every Task has kernel-visible identity | proposed | TSK-008 | none |
| D-0315 | Decide Task mapping onto kernel execution contexts | proposed | TSK-009 | none |
| D-0316 | Decide the default system font set and publish its script coverage matrix | proposed | TXT-002 | none |
| D-0317 | Decide the cross-Component glyph atlas and shaped-text cache sharing model | proposed | TXT-015 | none |
| D-0318 | Decide hosting existing IME engines versus native engines and the 1.0 language list | proposed | TXT-026 | S-016 |
| D-0319 | Decide the locale data source between ICU/CLDR and an ICU4X port | proposed | TXT-016 | none |
| D-0320 | Decide the message catalog format between Fluent and gettext | proposed | TXT-017 | none |
| D-0321 | Decide the shaping and rasterisation libraries for the native text stack | proposed | TXT-003 | none |
| D-0322 | Decide hinting, subpixel positioning and gamma policy across scale factors | proposed | TXT-027 | none |
| D-0323 | Decide whether shaping runs in-Component or in a shared text service Component | proposed | TXT-004 | none |
| D-0324 | Decide clipboard authority policy: paste gesture or Capability, no ambient read | proposed | UIP-004 | none |
| D-0325 | Decide global shortcut model: named actions bound in Settings, no key grabs | proposed | UIP-030 | none |
| D-0326 | Decide input routing and focus arbitration model for focused surfaces | proposed | UIP-005 | none |
| D-0327 | Decide UI protocol model: retained scene tree, client Buffers, or hybrid | proposed | UIP-006 | S-015 |
| D-0328 | Decide new Rust toolkit versus adapting an existing toolkit and renderer | proposed | UIP-007 | none |
| D-0329 | Decide server-side versus client-side decorations for native and compat windows | proposed | UIP-008 | none |
| D-0330 | Decide the fallback virtualization product and 1.0 scope | proposed | VIRT-002 | none |
| D-0331 | Decide guest-window integration depth and agent protocol | proposed | VIRT-003 | none |
| D-0332 | Decide the userspace Wasm runtime and host placement | proposed | WASM-007 | none |
| D-0333 | Decide WASI imports bound to native Capability | proposed | WASM-008 | none |
| D-0334 | Decide Wasm role versus native machine-code Components | proposed | WASM-001 | S-029 |
| D-0335 | Decide the kernel-level anti-cheat policy | proposed | WIN-002 | none |
| D-0336 | Decide anti-cheat vendor engagement and required legal agreements | proposed | WIN-058 | none |
| D-0337 | Decide the case-insensitive view for Windows Personality storage | proposed | WIN-019 | none |
| D-0338 | Decide a clean-room policy for the Windows Personality | proposed | WIN-005 | none |
| D-0339 | Decide how prefix installers become installed applications | proposed | WIN-033 | none |
| D-0340 | Decide NT Object-manager, async I/O, descriptor and section fidelity | proposed | WIN-035 | none |
| D-0341 | Decide how Wine and Proton map onto native Objects | proposed | WIN-036 | none |
| D-0342 | Accept the Windows Personality scoping Decision | accepted | WIN-001 | none |
| D-0343 | Decide that Win32 emulation stays in userspace | proposed | WIN-008 | none |
| D-0344 | Decide 32-bit Win32 support and WoW64 requirements | proposed | WIN-010 | none |
| D-0345 | Decide whether Wine hosts on the Linux Personality or the Native ABI | proposed | WIN-013 | none |
| D-0346 | Retention and exposure of Linux sandbox primitives | proposed | LNX-021 | none |
| D-0347 | SDK v1 crate API surface | proposed | SDK-055 | S-031 |
<!-- roadmap:generated:end -->
