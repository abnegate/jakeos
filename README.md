# JakeOS roadmap

[![roadmap](https://github.com/abnegate/jakeos/actions/workflows/roadmap.yml/badge.svg)](https://github.com/abnegate/jakeos/actions/workflows/roadmap.yml) [![pages](https://github.com/abnegate/jakeos/actions/workflows/pages.yml/badge.svg)](https://github.com/abnegate/jakeos/actions/workflows/pages.yml)

JakeOS is a post-Unix desktop operating system: a radical fork of the Linux kernel that keeps Linux's hardware maturity and exposes a new native model (capabilities, cheap isolated components, typed channels, asynchronous operations, memory objects with ownership transfer, resource domains, immutable packages and system generations, semantic interfaces), with Linux and Windows compatibility personalities. Rust first, x86-64 first.

This repository is the roadmap. It breaks the path from an empty kernel fork to a 1.0 public stable release into about two thousand tasks across 39 workstreams and a seven-rung milestone ladder, and it is validated by a small Rust tool so that it stays truthful for years.

## What this repository is and is not

It is: the source of truth for every task, decision, gate, benchmark definition, compatibility corpus, risk, threat, invariant, question, reference machine and ABI surface of the project, and the committed evidence that proves each done.

It is not: source code for the operating system (those live in the repositories listed in `registers/repos.md`), an issue tracker for bugs, a place for measured results outside the `reports/` skeletons, or a document with dates in it.


**Dashboard.** `generated/dashboard.html` is a self-contained page produced by `roadmap gen`: ladder progress, workstream heatmap, gates with their verifying tasks, ready queue, blockers, decision leverage, critical path and a searchable task explorer with a detail drawer. The published copy is at https://abnegate.github.io/jakeos/ after every push to `main`. Open it locally with `open generated/dashboard.html`; it embeds the same data as `generated/index.json`, so it is exactly as current as the last `roadmap gen`.

## Sixty-second reading guide

1. `ROADMAP.md` (generated): the ladder, which milestone is active, gate status, progress as count, size-weighted and gate percentages, the ready head and the critical path.
2. `milestones/V0.md` and its siblings: what each rung proves, its gates and demos, hardware scope, surfaces to freeze and risks to retire.
3. One workstream, for example `workstreams/IPC.md`: its scope, what it excludes, and its tasks in ID order.
4. One task: `roadmap show IPC-001` prints the block with its dependencies, dependents, decisions and gate criteria resolved. A task ID is enough to start work.

Then read `CONVENTIONS.md` before editing anything, and `AGENTS.md` if you are an AI agent.

## Running the tool

The tool is a Rust crate at `tools/roadmap/`, binary `roadmap`. From the repository root:

```sh
cargo run -q --manifest-path tools/roadmap/Cargo.toml -- check
```

Or build once and use the binary:

```sh
cargo build --release --manifest-path tools/roadmap/Cargo.toml
tools/roadmap/target/release/roadmap check
```

Common commands: `check`, `fmt`, `gen`, `show <ID>`, `ready`, `blocked`, `gate <TOKEN>`, `critical-path`, `coverage`, `progress`. Enable the pre-commit hook with `git config core.hooksPath .githooks`; it runs `fmt`, `gen` and `check` on every commit.

## How progress is updated

A task's status is changed only in its block in `workstreams/<PREFIX>.md`, following the transitions in `CONVENTIONS.md`: claim by setting an owner, tick acceptance boxes and append evidence as work lands, and mark done only with evidence and (from V1, and always for decisions and surface freezes) an independent verifier. Everything else is derived: `ROADMAP.md`, `STATUS.md`, `generated/**` and the marker blocks inside milestone, register and decision files are rewritten by `roadmap gen` and must never be edited by hand. Milestone status is derived from gates, and gates are satisfied by their verifying tasks being done.

## Documents

| Document | Purpose |
|---|---|
| `CONVENTIONS.md` | Normative rules: grammar, IDs, status model, milestones, decisions, registers, freeze discipline, commit and pull request policy |
| `AGENTS.md` | Load order, hard rules, commands and heuristics for AI agents |
| `BASELINE.md` | The architecture baseline with immutable section numbers; the only citable target |
| `GLOSSARY.md` | Canonical terms and casing |
| `decisions/README.md` | How decisions work, with the generated index |
| `reports/README.md` | Evidence report skeletons and what is committed |

## Workstreams

| Prefix | Name | Prefix | Name |
|---|---|---|---|
| KRN | Kernel fork and upstream tracking | WASM | WebAssembly components |
| BOOT | Boot and firmware | SEC | Security model and hardening |
| ABI | Native kernel ABI | NET | Networking |
| CAP | Capabilities | AUD | Audio |
| CMP | Components | MED | Media |
| TSK | Tasks, operations and structured concurrency | HW | Hardware enablement |
| IPC | Channels and typed interfaces | PWR | Power management |
| MEM | Memory objects and zero-copy dataflow | SDK | Native SDK and developer tools |
| SCH | Scheduling intent and resource domains | APP | Native applications and shell |
| OBS | Observability and tracing | INS | Installer, updater, recovery and migration |
| SVC | Service lifecycle and core system services | BLD | Build, toolchain and CI |
| STO | Storage and user-selected authority | LAB | Physical hardware lab |
| PKG | Packages, dependencies and system generations | BEN | Benchmarks |
| GFX | Graphics and compositor | REL | Release engineering and security response |
| UIP | Native UI protocol and toolkit | DOC | Documentation |
| TXT | Text, fonts, input methods and internationalization | GOV | Governance, legal and process |
| ACC | Accessibility | | |
| SEM | Semantic interfaces, automation and AI | | |
| LNX | Linux personality | | |
| WIN | Windows personality | | |
| VIRT | Fallback virtualization | | |
| ENV | Native development environments | | |
| HET | Heterogeneous compute | | |

## Milestone ladder

| Token | Rung | One line |
|---|---|---|
| V0 | Execution model | The fork boots on QEMU and one reference desktop and proves Component, Task, Capability, Channel, Operation, MemoryObject, ResourceDomain and tracing with L1 negotiation; L0 Linux corpus has zero regressions; every benchmark is publish-only |
| V0.5 | Application model | Compositor with crash rebind, declarative UI with accessibility metadata, immutable packages, generations with boot rollback, native init, the UserSelected chooser, four native applications and one Wayland application |
| V1 | Developer preview | Self-hosting and daily driving: SDK v1 with L1 freeze candidates, `os env`, Wi-Fi, audio and suspend on an Intel laptop, GPU-accelerated Linux browser and IDE, signed repository, debugger, profiler, first upstream rebase; Wine bring-up ungated |
| V2 | Desktop preview | Three target machines, multi-monitor, HDR, VRR, Bluetooth, suspend soak, Windows personality W1 gates, permissions UI, snapshots, store client, automation rules then an AI demo, screen-reader prototype, ComputeDevice demo, safe-mode session |
| V3 | Public alpha | Installer with full-disk encryption and Secure Boot on six Tier 1 machines, updater with automatic rollback, opt-in crash reporting, Hardware Compatibility List, public repository, exercised security response, continuous fuzzing, multi-user, complete L1 reference documentation |
| V4 | Beta | L1 frozen with a conformance suite, L2 versions locked, feature freeze, external audit closed, crash-free beta fleet, upgrades from V3, ten Tier 1 machines, accessibility script complete, ten languages, reproducible builds |
| 1.0 | Public stable | Release-candidate soak with no P0 or P1, ABI stability statement, published support window and CVE response commitment, update and rollback guarantee, every §54 metric published against Linux, Windows and macOS, complete documentation, 1.x branch and 2.0 RFC |
| LATER | Parking rung | Deferred beyond 1.0: casting, NFC, WWAN, MIDI, ARM64, native filesystem and GPU stack, distributed interfaces |

Everything under `generated/`, plus `ROADMAP.md` and `STATUS.md`, is generated. Do not edit it.
