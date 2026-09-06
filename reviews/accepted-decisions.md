# Accepted decisions at the start of V0

Forty decisions are accepted and carry the maintainer as verifier. Every V0 task inherits them. One line each: the option taken, then what it forecloses. Grouped by area. Read it as a set; a line you would not have chosen is a superseding decision to raise before the swarm starts, not after.

## Kernel

- **D-0158 · Kernel strategy.** Radical fork of Linux; the native model is added in-tree behind the native ABI, wrappers first (phases A to C), native later (D to E). Forecloses a new microkernel and Linux-as-hypervisor.
- **D-0166 · Base tree.** Cut from the newest mainline tag (v7.2 at decision time); release candidates are never bases. Forecloses an LTS base and the years of frozen Rust-for-Linux APIs that would come with it.
- **D-0168 · Upstream tracking.** Merge every mainline tag and stable point release as merge commits; history is never rewritten. Forecloses periodic rebase of the public kernel repository.
- **D-0167 · Upstream-first.** Driver fixes, hardware enablement and generic Rust-for-Linux abstractions go to the Linux lists before or with landing in the fork; native-model code is fork-only. Forecloses a fork-only hardware layer.
- **D-0157 · Kernel-core boundary.** The §4 split stands: memory, IPC, capabilities, scheduler in the core; UI, storage and network services in user space. Moving anything across needs a measured decision. Forecloses storage or network in the core and IPC or the scheduler in user space.
- **D-0162 · Kernel licence.** All new kernel code is GPLv2-only; reusable code is authored as an MIT crate in the platform repository and vendored under a recorded exception. Forecloses per-file dual licensing and GPLv2-or-later.
- **D-0165 · Rust toolchain pin.** One toolchain file per release, allowed to lag the Rust-for-Linux minimum of the merged tag by at most two Rust releases. Forecloses tracking upstream exactly and pinning independently.
- **D-0036 · C compiler.** LLVM/Clang only; GCC-only kernel configurations are dropped; Rust and C share one LLVM per release. Forecloses a GCC matrix and a project compiler.
- **D-0122 · Driver residency.** Per device class by measured cost; inherited drivers stay in-kernel by default; a class moves to user space only after a spike. Forecloses microkernel purity and all-in-kernel forever.

## Execution model

- **D-0062 · Process identity.** `Object<Component>` handles are the only native identity: no PIDs, no kernel process tree, no exit integers, no process groups. Lineage and exit causes live in the supervisor. Forecloses kernel-visible lineage.
- **D-0064 · Isolation model.** Component plus ResourceDomain is the only native isolation; OCI containers are a Linux-personality feature. Forecloses a native container runtime.
- **D-0066 · Fault semantics.** Panic, stack overflow, out-of-memory and capability violation terminate the whole Component with a typed exit cause; nothing unwinds across the ABI. Forecloses unwinding to the Component boundary and per-Component fault policy.
- **D-0068 · Spawn primitive.** A ComponentBuilder object: attach code, Capabilities, ResourceDomain, Channels and supervisor, then one start Operation; nothing is inherited implicitly. Forecloses one-shot spawn and fork-style template clone.
- **D-0309 · No signals.** Termination, cancellation and asynchronous notification are Operation completions and waitable Objects; running code is never interrupted asynchronously. Forecloses a signal-like native event.
- **D-0192 · Memory coherence.** Every CPU mapping of a MemoryObject is coherent; device visibility is explicit through typed synchronisation and handoff Operations. Forecloses per-mapping coherence attributes.

## Boot and platform

- **D-0049 · Firmware.** UEFI-only on x86-64 through 1.0; the installer refuses machines without it. Forecloses BIOS/CSM.
- **D-0051 · V0 boot.** V0 boots exactly as Linux does, retained initramfs and Linux init, with the native runtime started beside it; native init replaces Linux init at V0.5. Forecloses native init at V0.
- **D-0033 · Build.** Kbuild for the kernel, a Cargo workspace for the platform, thin scripts to compose images; no new build language before the content-addressed store exists. Forecloses Bazel/Buck2 and a derivation-based builder for now.
- **D-0034 · CI.** Lint, unit tests and the roadmap validator on GitHub-hosted runners; every boot, hardware and benchmark job on self-hosted lab runners, with a quiet dedicated subset for benchmarks. Forecloses all-hosted and all-self-hosted.
- **D-0037 · Repositories.** Two code repositories, the kernel fork with full upstream history and the jakeos-platform monorepo for everything above the ABI, plus this roadmap. Forecloses a single monorepo with the kernel and a pinned multi-repo.
- **D-0089 · Hosting.** GitHub under the abnegate account, public, Actions and Pages. Forecloses a self-hosted forge for now.
- **D-0169 · Lab.** An office or home lab: PDU power control, USB-debug or serial consoles, HDMI capture, the photon rig on the same bench, laptops physically reachable. Forecloses colocation.

## Licensing and governance

- **D-0102 · Licence firewall.** Layer 1 GPLv2-only; Layers 2 to 4 MIT; the ABI boundary is the licence boundary. Forecloses Apache-2.0, MPL-2.0 and dual MIT/Apache user space.
- **D-0008 · ABI headers.** GPLv2 headers with a syscall-note-style exception, as Linux UAPI; the SDK regenerates typed bindings under MIT. Forecloses SDK-licence-only and dual-licensed headers.
- **D-0092 · Contributions.** DCO on the kernel; DCO plus a non-assignment CLA on the MIT repositories so the licence can move to a future OSI licence. Forecloses DCO-only everywhere and copyright assignment.
- **D-0095 · Documentation.** CC-BY 4.0. Forecloses share-alike and CC0.
- **D-0097 · Firmware blobs.** The official image ships the redistributable subset of linux-firmware; non-redistributable firmware is never shipped and excludes hardware from Tier 1. Forecloses a separate non-free firmware repository and download-on-demand.
- **D-0098 · Fonts.** OSI-licensed fonts plus a documented map of metric-compatible substitutes for proprietary names; no proprietary font ever ships. Forecloses substitute-free and font-free images.
- **D-0001 · This roadmap.** Markdown with a strict grammar validated and rendered by the Rust `roadmap` tool; IDs permanent; status stored, everything else derived. Forecloses Markdown-only, a Python validator and an issue tracker.
- **D-0114 · Workstream splits.** Only by a GOV decision when two separable sub-scopes with distinct leads or gates exist; file length warns, never splits. Forecloses mechanical splits.

## Hardware and reference machines

- **D-0129 · Reference machines.** Three named SKUs: an AMD desktop, an Intel laptop, an AMD laptop, each with IOMMU, TPM 2.0 and Secure Boot user-key enrolment; nothing gates on other hardware before V3. Forecloses families without SKUs and an unconstrained PC list.
- **D-0128 · HCL tiers.** Two tiers: Tier 1 in the lab and tested every release, Tier 2 community-reported with probe data; unlisted is unsupported. Forecloses a middle tier.
- **D-0127 · Sensors.** Lid switch and ambient light sensor in 1.0; accelerometer and tablet mode are LATER. Forecloses convertibles as reference hardware.
- **D-0126 · Printing.** Driverless-first native print service (IPP Everywhere, AirPrint-class), OS-owned dialog minting `Capability<PrintJob>`, PDF spool; CUPS only inside the Linux personality for driver printers. Forecloses CUPS as the native service.

## Measurement and release

- **D-0031 · Benchmark policy.** Targets live only in the benchmark register as publish, absolute or regression per milestone; every V0 target is publish; no prose states a number without a B-ID. Forecloses numeric V0 exits and restated numbers.
- **D-0032 · Startup clock.** Application startup stops at the compositor's first scanout of a non-placeholder frame from the new application. Forecloses first client commit and the photodiode edge as the startup boundary.
- **D-0240 · Versioning.** OS artifacts are a monotonic SystemGeneration counter plus a channel name; Layer 2 interfaces and Layer 3 crates use semver; no identifier carries a date. Forecloses semver for generations.

## Text and personalities

- **D-0319 · Locale data.** ICU4X, linking only the data each Component needs. Forecloses ICU with CLDR in C++.
- **D-0320 · Message catalogs.** Fluent; grammar lives in the catalog, not in code. Forecloses gettext.
- **D-0342 · Windows personality.** Starts from Wine and Proton on the Linux personality, with native bindings replacing Linux dependencies over time; a double-clicked .exe is a normal window with no visible VM, desktop or wizard; kernel anti-cheat and vendor DRM excluded from 1.0. Forecloses deferring Windows past 1.0 and a clean-room Win32.
