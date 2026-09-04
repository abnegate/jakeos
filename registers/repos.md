# Repository aliases

This register maps evidence aliases to repository URLs. Evidence lines of the form `alias@sha` and `alias#number` resolve here. Aliases are owned by GOV. URLs are placeholders until the corresponding repositories exist; they are not measured values.

### kernel
- URL: https://github.com/jakeos/kernel
Fork of Linux that carries the native kernel ABI, capabilities, components, tasks, channels, MemoryObjects and ResourceDomains.

### runtime
- URL: https://github.com/jakeos/runtime
Native userspace runtime: init, supervisors, typed services, and the small runtime that V0 components link.

### sdk
- URL: https://github.com/jakeos/sdk
Native SDK crates, C headers for the Layer 1 ABI, IDL compiler, and `os` CLI.

### compositor
- URL: https://github.com/jakeos/compositor
Native compositor, DRM/KMS integration, and the UI protocol server.

### apps
- URL: https://github.com/jakeos/apps
Shipped native applications and the desktop shell.

### personality-linux
- URL: https://github.com/jakeos/personality-linux
Linux personality: syscall retention and translation, D-Bus, portals, Wayland and X11 bridges, PipeWire, OCI.

### personality-windows
- URL: https://github.com/jakeos/personality-windows
Windows personality: PE loading, Win32 and selected NT semantics, Wine and Proton integration, DXVK and VKD3D.

### installer
- URL: https://github.com/jakeos/installer
Installer, updater, recovery environment, image builder and migration assistant.

### docs
- URL: https://github.com/jakeos/docs
Public documentation sources, ABI reference, guides and research notes.

### bench
- URL: https://github.com/jakeos/bench
Benchmark harnesses named in `registers/benchmarks.md` and compatibility scenario runners named in `registers/corpora.md`.

### roadmap
- URL: https://github.com/jakeos/roadmap
This roadmap repository.
