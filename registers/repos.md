# Repository aliases

This register maps evidence aliases to repository URLs. Evidence lines of the form `alias@sha` and `alias#number` resolve here, and Verification lines name test paths as `alias:path`. Aliases are owned by GOV. Every alias below `platform` is a directory inside the jakeos-platform monorepo whose layout the platform-layout task fixes; URLs are placeholders until the corresponding directories exist.

### kernel
- URL: https://github.com/abnegate/jakeos-kernel
Fork of Linux that carries the native kernel ABI, capabilities, components, tasks, channels, MemoryObjects and ResourceDomains.

### runtime
- URL: https://github.com/abnegate/jakeos-platform/tree/main/runtime
Native userspace runtime: init, supervisors, typed services, and the small runtime that V0 components link.

### sdk
- URL: https://github.com/abnegate/jakeos-platform/tree/main/sdk
Native SDK crates, C headers for the Layer 1 ABI, IDL compiler, and `os` CLI.

### compositor
- URL: https://github.com/abnegate/jakeos-platform/tree/main/compositor
Native compositor, DRM/KMS integration, and the UI protocol server.

### apps
- URL: https://github.com/abnegate/jakeos-platform/tree/main/apps
Shipped native applications and the desktop shell.

### personality-linux
- URL: https://github.com/abnegate/jakeos-platform/tree/main/personality-linux
Linux personality: syscall retention and translation, D-Bus, portals, Wayland and X11 bridges, PipeWire, OCI.

### personality-windows
- URL: https://github.com/abnegate/jakeos-platform/tree/main/personality-windows
Windows personality: PE loading, Win32 and selected NT semantics, Wine and Proton integration, DXVK and VKD3D.

### installer
- URL: https://github.com/abnegate/jakeos-platform/tree/main/installer
Installer, updater, recovery environment, image builder and migration assistant.

### docs
- URL: https://github.com/abnegate/jakeos-platform/tree/main/docs
Public documentation sources, ABI reference, guides and research notes.

### bench
- URL: https://github.com/abnegate/jakeos-platform/tree/main/bench
Benchmark harnesses named in `registers/benchmarks.md` and compatibility scenario runners named in `registers/corpora.md`.

### gfx
- URL: https://github.com/abnegate/jakeos-platform/tree/main/compositor
Compositor, DRM/KMS plumbing and GPU broker tests (`gfx:` Verification paths).

### pkg
- URL: https://github.com/abnegate/jakeos-platform/tree/main/packages
Package manager, content store and SystemGeneration code.

### personality
- URL: https://github.com/abnegate/jakeos-platform/tree/main/personality-linux
Linux personality tests written with the `personality:` prefix.

### win
- URL: https://github.com/abnegate/jakeos-platform/tree/main/personality-windows
Windows personality tests written with the `win:` prefix.

### hw
- URL: https://github.com/abnegate/jakeos-platform/tree/main/hardware
User-space driver framework, device bring-up and hardware regression tests.

### storage
- URL: https://github.com/abnegate/jakeos-platform/tree/main/storage
Native storage service, chooser and storage Object tests.

### rel
- URL: https://github.com/abnegate/jakeos-platform/tree/main/release
Release engineering: signing, repository, channels and update tooling.

### idl
- URL: https://github.com/abnegate/jakeos-platform/tree/main/idl
IDL compiler, generated stubs and wire-format tests.

### sem
- URL: https://github.com/abnegate/jakeos-platform/tree/main/semantic
Semantic Interface registry, automation rules and AI broker.

### text
- URL: https://github.com/abnegate/jakeos-platform/tree/main/text
Text stack: shaping, rasterisation, atlas service, input methods and localisation.

### env
- URL: https://github.com/abnegate/jakeos-platform/tree/main/environments
Native development environments and `os env`.

### media
- URL: https://github.com/abnegate/jakeos-platform/tree/main/media
Media pipeline, codec Components and camera service.

### acc
- URL: https://github.com/abnegate/jakeos-platform/tree/main/accessibility
Accessibility tree, screen reader and assistive-technology bridges.

### lab
- URL: https://github.com/abnegate/jakeos-platform/tree/main/lab
Lab fixtures, power and capture rigs and soak schedulers.

### wasm
- URL: https://github.com/abnegate/jakeos-platform/tree/main/wasm
Wasm runtime host and Component Model integration.

### tools
- URL: https://github.com/abnegate/jakeos-platform/tree/main/tools
Repository tooling and CI lints that are not the roadmap validator.

### compat
- URL: https://github.com/abnegate/jakeos-platform/tree/main/bench/compat
Compatibility scenario runners for the corpora in `registers/corpora.md`.

### ipc
- URL: https://github.com/abnegate/jakeos-platform/tree/main/ipc
Channel runtime, transports and Layer 2 evolution tests outside the kernel.

### bld
- URL: https://github.com/abnegate/jakeos-platform/tree/main/build
Build orchestration, image assembly and CI plumbing.

### userspace
- URL: https://github.com/abnegate/jakeos-platform/tree/main/.
Whole-monorepo checks that span more than one crate.

### abi
- URL: https://github.com/abnegate/jakeos-platform/tree/main/abi
Native ABI definition, generators, snapshot and conformance cases.

### boot
- URL: https://github.com/abnegate/jakeos-platform/tree/main/boot
Boot manager, UEFI stub and generation selection.

### cmp
- URL: https://github.com/abnegate/jakeos-platform/tree/main/runtime/component
Component runtime tests hosted in the runtime crate.

### obs
- URL: https://github.com/abnegate/jakeos-platform/tree/main/observability
Observability data plane, trace export and inspect providers.

### sec
- URL: https://github.com/abnegate/jakeos-platform/tree/main/security
Security services: identity, secrets, grants and lock-screen policy.

### uip
- URL: https://github.com/abnegate/jakeos-platform/tree/main/ui
UI protocol server, toolkit and input routing.

### platform
- URL: https://github.com/abnegate/jakeos-platform
Userspace monorepo above the kernel ABI: runtime, IDL compiler, SDK, compositor, shell, applications, personalities, installer, benchmark harnesses and documentation (D-0037).

### roadmap
- URL: https://github.com/abnegate/jakeos
This roadmap repository.
