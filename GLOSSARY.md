# Glossary

Canonical terms and their casing. Each `##` heading is the exact spelling; `roadmap fmt` rewrites a case-insensitive whole-word match in a task title to the heading's spelling, and `roadmap check` warns on non-canonical casing elsewhere. Section references are to `BASELINE.md`.

## Component

The primary isolation primitive. A Component owns code, an address space constructed from immutable Packages, a set of Capabilities, typed Inputs and Outputs, a TaskGroup and a ResourceDomain. Applications are graphs of Components, each holding only the authority it needs, so an exploit compromises the smallest useful unit of software. Creation and teardown are cheap enough to use everywhere. §10, §11, §34, §53.

## Task

A lightweight unit of concurrent execution multiplexed by the kernel and runtime over execution contexts; hundreds of thousands of Tasks never imply as many kernel threads. A Task is owned by a TaskGroup and suspends on Operations. §20, §21.

## TaskGroup

The ownership node of structured concurrency. Every Task belongs to a TaskGroup; cancelling a TaskGroup deterministically cancels everything it owns. Applications and Components each hold TaskGroups. §21.

## Object

A typed kernel object, written `Object<T>` (Object<File>, Object<Channel<Message>>, Object<Component>). Userspace never holds an Object directly; it holds a Capability to it. §7.

## Capability

An unforgeable, typed, inspectable and auditable reference to an Object that names the object, the operations the holder may perform and how it may be transferred or delegated, written `Capability<T, Rights>`. Capabilities are attenuable (a holder can derive a weaker one) and revocable, and are the only source of authority in the native model. §7, §8, §9.

## Channel

The typed IPC primitive, written `Channel<T>`. Services expose typed interfaces over Channels; messages carry Capabilities and ownership of MemoryObjects rather than copies. Small-message latency is a first-class performance target. §12, §14, §15, §43.

## Interface

A typed, versioned contract exposed over a Channel, defined in the IDL and compiled to stubs. Interfaces support optional methods, schema evolution in both directions, feature negotiation, streams, futures, ownership transfer, cancellation and failure semantics. §12, §14, §42.

## MemoryObject

The representation of large data with properties such as writable, immutable, shared, copy-on-write, DMA-compatible, GPU-compatible, executable, persistent, pinned and encrypted. Movement prefers ownership transfer over copying. §16, §17, §38.

## ResourceDomain

The native unit of resource accounting: CPU policy, memory budget, GPU budget, I/O budget, network policy, storage quota, energy policy and latency policy. Components belong to ResourceDomains, which subsume cgroups, namespaces and container runtime configuration. §22, §23.

## Operation

The first-class representation of outstanding asynchronous work, written `Operation<Result>`. Every native kernel interaction is a submitted Operation with cancellation, completion, deadline, priority, tracing, ownership and resource accounting. §18, §19.

## Package

An immutable unit of software: manifest, Components, resources, Interfaces, requested Capabilities and dependencies. Installation makes a Package available; multiple versions coexist with deduplication. §28, §29.

## SystemGeneration

An immutable, versioned composition of kernel, shell, compositor, services and Packages. An update creates a new SystemGeneration; a failed boot returns to the previous one; history and restore operate on SystemGenerations. Also written "system generation" in prose; see Generation. §30, §31.

## ComputeDevice

An enumerable execution resource (CPU, GPU, NPU, DSP, FPGA, accelerator) to which workloads are dispatched with preferences for latency, throughput, energy, precision, memory and locality. §37, §38.

## ComputeQueue

The typed submission queue of a ComputeDevice, an Object like any other, inspectable through `os inspect`. §7, §37, §39.

## Surface

The graphics presentation object an application renders into, alongside Buffer, RenderQueue, Display and Frame, over DRM/KMS without exposing DRM. Not to be confused with an ABI surface, which this repository always writes as "ABI surface". §39, §40.

## ABI surface

An ABI or interface shape tracked in `registers/surfaces.md` as `S-NNN`, with a stability Layer and a state of `open`, `prototyped`, `frozen` or `superseded`. Frozen only after a done spike explores it and an accepted Decision lists it; L1 also needs a benchmark report. §65, §66.

## Personality

A compatibility subsystem that presents a foreign operating-system model (processes, file descriptors, signals, Win32 handles) to foreign software while native software sees none of it. Compatibility inherits history; native software does not. §3, §46, §48.

## Linux personality

The Personality that runs Linux software: the Linux syscall ABI retained directly at first, then translated onto native primitives, with the POSIX process model, D-Bus, Wayland and X11 bridging, PipeWire, portals and OCI containers. A product, not a shim. §3, §36, §46, §47, §49, §56.3.

## Windows personality

The Personality that runs Windows software, starting from Wine and Proton concepts atop the Linux personality and moving toward native bindings: PE loading, Win32 and selected NT semantics, registry and filesystem emulation, DXVK and VKD3D, gaming. §3, §48, §49, §56.2.

## Layer 1

The native kernel ABI: extremely small, extremely stable, frozen at V4 and declared stable at 1.0. Any change after 1.0 is a new major version. §65, §66.

## Layer 2

Core platform interfaces: strongly versioned schemas served for the whole 1.x line with a published deprecation policy. Evolution rules frozen at V1, versions locked at V4. §66.

## Layer 3

The native SDK. Evolves relatively quickly under semantic versioning; Rust `std` support lives here as a compatibility crate, never in Layer 1. §52, §66.

## Layer 4

Frameworks. Evolve rapidly; no stability promise beyond semantic versioning. §66.

## Native ABI

The Layer 1 interface between native software and the kernel: minimal, capability-based, asynchronous, ownership-transferring, versioned, free of POSIX shapes unless independently justified, and designed to survive decades and future hardware. The project's most permanent decision. §7, §65, §66.

## Spike

A task of Type `spike`: a time-boxed investigation that builds, measures and rules out, and ends in `reports/spikes/<TASK-ID>.md`. Spikes explore ABI surfaces before Decisions fix them. §65.

## Decision

A recorded architectural choice, also called an ADR: a file `decisions/D-NNNN-<slug>.md` with at least two options, coupled to exactly one task of Type `adr`. Immutable once accepted; revised only by a superseding Decision. §65, §67.

## Gate

A named exit criterion of a Milestone, `<TOKEN>-GNN`, of Kind functional, benchmark, compatibility, demo or process. Satisfied when its verifying tasks are done and its cited benchmark target or corpus threshold is met. Never hand-ticked. §59 to §63.

## Demo

A named end-to-end demonstration of a Milestone, `<TOKEN>-DNN`, verified by tasks, shown on a named reference machine. §59 to §63.

## Corpus

A defined set of Linux (L0 to L5) or Windows (W1 to W3) applications with scripted scenarios, a rating scale and per-Milestone thresholds, tracked as `C-NNN` in `registers/corpora.md`. Compatibility gates cite a Corpus; results are committed under `reports/compat/`. §46, §48, §49.

## Tier 1

A reference machine that is fully tested every release and on which every listed feature works, including suspend and resume, Wi-Fi, Bluetooth, external displays and HDR or VRR where supported. Named in `registers/hardware.md`. §62, §63.

## Tier 2

A community-reported machine in the compatibility database, supported on a best-effort basis with a documented promotion path to Tier 1. Anything unlisted is unsupported. §62, §63.

## Milestone

A rung of the ladder (V0, V0.5, V1, V2, V3, V4, 1.0, LATER) defined by `milestones/<TOKEN>.md`: purpose, gates, demos, hardware scope, surfaces to freeze and risks to retire. Status is derived, never stored. §59 to §63.

## Rung

A position on the Milestone ladder, identified by rank. Dependencies must sit in the same or an earlier Rung than their dependents. §59 to §63.

## Workstream

One of the 39 areas of ownership, identified by a prefix (KRN, IPC, LNX). Each has one file `workstreams/<PREFIX>.md` holding its scope, exclusions and tasks. §4.

## Register

A typed list of entries with IDs that tasks, gates and Decisions reference: risks (R), benchmarks (B), corpora (C), threats (T), invariants (I), questions (Q), hardware (H), ABI surfaces (S) and repository aliases. Registers hold definitions and targets, never measured results.

## Draft ID

A branch-only task identifier of the form `PREFIX-@slug`, accepted wherever a task ID is accepted and converted to a real `PREFIX-NNN` by `roadmap assign-ids`. Never present on `main`.

## Evidence

The `#### Evidence` section of a task: commits and pull requests in registered repositories, report files, Decisions and review URLs that prove the work landed. Prose is not Evidence.

## Verification

The `#### Verification` section of a task: the plan for proving it done, written before work starts, as lines of kind Unit, Integration, Fuzz, Bench, Compat, Manual, Review, Demo or Report. The independent verifier reruns it.

## Unanchored task

A task that no Gate or Demo reaches, directly as a verifying task or transitively through dependencies. Reported in `STATUS.md`; the target count is zero.

## Derived state

A display state computed by the tool and never stored: ready, blocked, in-review, stale for tasks; satisfied for Gates; complete, active, next, planned for Milestones.

## Generation

Short for SystemGeneration in prose about updates, history, rollback and boot selection ("boot the previous generation"). Not to be confused with the roadmap's own generated output. §30, §31, §32.

## Content-addressed store

Storage in which immutable data is identified by the hash of its content (for example `sha256:…`), giving deduplication, integrity verification and cheap sharing of Packages and SystemGenerations. §27.

## UserSelected

The storage authority pattern, written `UserSelected<T>`: the operating system owns the chooser, and the object it returns carries the authority to use exactly what the user picked, so applications need no filesystem namespace. §25.

## Structured concurrency

The rule that every Task is owned by a TaskGroup, cancellation propagates down the ownership tree, and persistent background execution requires an explicit Capability, so there are no orphan processes. §21.

## Scheduling intent

A declared class such as Interactive, Background, Throughput, LowLatency, Realtime, EnergyEfficient or Deadline that a ResourceDomain or Task expresses and that influences CPU and GPU scheduling, frequency scaling, core selection, memory placement and I/O scheduling. §22.

## Zero-copy

Dataflow in which a MemoryObject moves from network to decoder to GPU by ownership transfer rather than by copying at each stage. §17.

## Ownership transfer

Moving a MemoryObject or Capability from one holder to another so that exactly one party owns it afterward; the default over copying and over shared mutable state. §16, §17, §65, §67.

## Semantic interface

A typed Interface an application exposes for what it does (open a project, search mail, run tests) rather than how it is drawn, enabling automation and AI without GUI scraping. §42, §43, §45.

## Automation rule

A user-defined trigger-and-action composition over Semantic interfaces ("when Download.completed and the file is an archive, extract it") using typed Capabilities instead of keyboard macros. §45.

## AI broker

The component through which an AI assistant obtains typed, permissioned, logged, revocable and scoped Capabilities to act on Semantic interfaces. The assistant never receives ambient authority. Built only after the semantic registry exists. §44, §57, §67.

## Hardware Compatibility List

The published list of machines by Tier with probe data and per-feature verdicts, fed by the compatibility database and consulted by the installer. §62, §63.

## Reference machine

A named physical machine or QEMU profile in `registers/hardware.md` (`H-NNN`) on which gates, benchmarks and demos run. Every Milestone declares its hardware scope as a list of Reference machines. §59, §62.

## Input-to-photon latency

The time from a physical input event to the corresponding change of light on the display, measured with a photodiode or high-speed camera rig on a Reference machine and compared against Linux and Windows on the same hardware. §54.

## Error vocabulary

The provisional names for typed kernel-boundary results used across tasks until ABI-009 fixes the error model: `Error::Rights` (forged handle, wrong type, missing right or denied derivation), `Error::Exhausted` (a ResourceDomain or kernel-object limit), `Error::Cancelled` (owner cancel), `Error::DeadlineExceeded` (deadline passed; the roadmap never calls this Timeout), `Error::Revoked` (use after revocation), `Error::Disconnected` (peer death), `Error::Integrity` (failed content or signature verification). ABI-009 may rename them; tasks written after it use its names. §7, §19, §65.

