## Workstreams (39)

| Prefix | Name | Scope notes (§ = baseline; gap = no section) |
|---|---|---|
| KRN | Kernel fork and upstream tracking | divergence phases A–E, rebase cadence, retained-mechanism inventory (one task, not 46), watchdogs, eBPF role adr, live-patching non-goal (§2 §5 §6 §55 §56.4) |
| BOOT | Boot and firmware | UEFI, bootloader, generation selection, secure/measured boot, boot counter; V0 boots via retained initramfs, native init is V0.5 (gap) |
| ABI | Native kernel ABI | handles, syscall surface, error model, L1/L2 negotiation from V0, freeze process; L1 surfaces `prototyped` through V0, freeze candidates V1, frozen V4 (§7 §8 §12 §65 §66) |
| CAP | Capabilities | rights encoding, derivation, revocation, audit, grant continuity across updates, CHERI readiness (§7 §8 §9 §51) |
| CMP | Components | creation fast path, address space from packages, panic/abort semantics, graphs, warm start (§10 §11 §34) |
| TSK | Tasks, operations, structured concurrency | Task/TaskGroup, Operation<Result>, cancellation, deadlines, io_uring lineage; absorbs inventory prefix OPS (§18–§21) |
| IPC | Channels and typed interfaces | IDL/codegen, backpressure, versioning rules (frozen V1), fast paths, interface design guidelines (§12 §14 §15 §43) |
| MEM | Memory objects and zero-copy | ownership transfer, charging across domains, file mapping, CoW/DMA/GPU/NUMA (§16 §17 §38) |
| SCH | Scheduling intent and resource domains | budgets incl. kernel-object limits, intent classes phased V0→V1, IRQ affinity (§22 §23 §53) |
| OBS | Observability and tracing | tracing, `os inspect` data, crash capture format, profiler data (§24 §64) |
| SVC | Service lifecycle, user-space driver hosting, core system services | supervision, restart/rebind, native init, safe-mode session, time/NTP/locale/hostname/settings storage (§32 §33) |
| STO | Storage and user-selected authority | filesystem adr (ZFS rejected on licence), CoW/snapshots, content store, chooser authority, change notification, durability contract, foreign/network filesystems, quotas, disk health, backup (§25 §26 §27) |
| PKG | Packages, dependencies, generations, history | manifest with reserved signing fields (V0.5), running-app-during-update behaviour, personality version pinning (§28–§31 §34) |
| GFX | Graphics and compositor | DRM/KMS abstractions, compositor restart, HDR/VRR, overlay planes and direct scanout, hybrid graphics, MST (§32 §39 §40 §56.1) |
| UIP | Native UI protocol and toolkit | protocol, declarative UI, input routing, drag/drop, clipboard, a11y metadata emission at V0.5 (§41) |
| TXT | Text, fonts, input methods, i18n | shaping, rasterisation, read-only glyph atlas service, IME protocol surface before UI freeze, localisation (gap) |
| ACC | Accessibility | tree model adr at V0.5, screen reader, keyboard-only, AT bridging, conformance (§41 §42 §49) |
| SEM | Semantic interfaces, automation, AI | registry → discovery → automation rules → AI broker, in that dependency order (§42–§45) |
| LNX | Linux personality | syscall retention → translation; seccomp/userns/overlayfs/ptrace/inotify/32-bit at V1; D-Bus, Wayland/X11 (primary selection stays inside the bridge), PipeWire, portals, OCI, Steam runtime sub-corpus (§3 §36 §46 §47 §49 §56.3) |
| WIN | Windows personality | V0 scoping adr; V1 non-gated Wine-on-LNX bring-up + Wine test suite CI + W1 definition; V2 gates; redistributables packaged, services/autostart bounded (§3 §48 §49 §56.2) |
| VIRT | Fallback virtualisation | KVM VM manager as capability-scoped component, guest tools, JakeOS guest images, "offer the VM" triage flow (gap; V2+) |
| ENV | Native development environments | §35 §36 |
| HET | Heterogeneous compute | §37 §38 |
| WASM | WebAssembly components | §13 |
| SEC | Security model and hardening | threat model at V0, identity/login, FDE, secrets, permission policy, CA trust store, side-channel statement, audit (§9 §51 §63) |
| NET | Networking | stack retention, capabilities, Wi-Fi/DHCP/DNS/VPN/firewall, remote shell (gap) |
| AUD | Audio | low-latency path, hot-plug/default switching, mixer, Bluetooth codecs, echo cancellation with MED (gap) |
| MED | Media | sandboxed codec components, HW video decode/encode via MemoryObject, camera service, screen-recording encode, protected content (gap) |
| HW | Hardware enablement | reference machines, input devices, Bluetooth, USB, printing/scanning, firmware update service, sensors (§33 §55 §62) |
| PWR | Power management | suspend/resume, battery, thermal, DPMS, frequency via SCH intent (§22 §54 §61 §62) |
| SDK | Native SDK and developer tools | runtime, `std` at Layer 3 only, bindings, `os` CLI, debugger, profiler; absorbs inventory prefix CLI (§50 §52 §64 §66) |
| APP | Native applications and shell | Shell sub-scope (panel, launcher, notifications, lock, greeter, quick settings) and Applications sub-scope (terminal, files, editor, viewer, settings, store client, consent UI, migration assistant, search) (§9 §25 §49 §60–§63) |
| INS | Installer, updater, recovery, migration | image builder + scripted install at V0.5/V1, first boot, rollback UX, crash-report client, dual-boot/foreign-OS coexistence (§30 §31 §63) |
| BLD | Build, toolchain, CI | Rust-in-kernel, reproducible builds, QEMU matrix, kselftests, kernel debug workflow, fuzzing, licence/SBOM CI gates, perf-CI fleet (§50 §51 §54 §55) |
| LAB | Physical hardware lab | machine procurement per tier, power/console/capture rigs, photodiode input-to-photon rig (V0), power meters, HDR reference display + colorimeter, soak scheduling (gap) |
| BEN | Benchmarks | owns registers/benchmarks.md, methodology, results, regression comparisons (§10 §34 §53 §54 §59) |
| REL | Release engineering and security response | signing, repository, channels, CVE ops, end-to-end crash/telemetry pipeline, HCL and compatibility-database publication, notices bundle (§27 §56.4 §63) |
| DOC | Documentation | IDL→docs generation at V1, references, guides, offline `os help`, research studies §58 (§56.5 §63) |
| GOV | Governance, legal, process | licensing policy (GPL boundary, Wine, fonts, codecs), trademark, DCO/CLA, RFC/ADR process, this repo's tooling (§1 §57 §58 §67 §68) |

Prefix remap for coverage: inventory `CLI→SDK`, `OPS→TSK`; lens prefixes `LIC→GOV/BLD/REL`, `SHL→APP`, `SYS→SVC`, `PRT→HW`, `TEL→OBS/INS/REL`, `MIG→INS/STO/APP`.

