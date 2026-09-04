# Project Baseline: A Post-Unix Desktop Operating System

> Build the desktop operating system we would design today if Unix, Windows, and their backwards-compatibility constraints had never existed — while still allowing users to run their existing Linux and Windows software.

**Status:** Baseline Architecture / Research
**Initial platform:** x86-64 desktop and laptop
**Kernel strategy:** Radical fork of the Linux kernel
**Native model:** Capability-based, component-oriented, asynchronous, object-based
**Compatibility:** Linux/POSIX + Windows/Win32 as compatibility personalities
**Primary implementation:** Rust for new kernel/platform code; retain C where inherited Linux code remains appropriate

> This file is the citable baseline for the JakeOS roadmap. Section numbers are immutable: tasks cite them as `§N` or `§N.M`. Changing this file requires a GOV decision (`Roadmap-Decision: D-NNNN` commit trailer).

---

# 1. Vision

We are not building:

- another Linux distribution
- another desktop environment
- a Linux userspace with prettier APIs
- a POSIX-compatible operating system
- a Windows clone
- a traditional microkernel for academic purity
- a kernel rewrite for the sake of rewriting a kernel

We are building a **new operating system that inherits the mature hardware foundation of Linux without inheriting Linux as its native programming model**.

Linux gives us decades of engineering around:

- PCIe
- USB
- NVMe
- SATA
- ACPI
- power management
- AMD/Intel/NVIDIA graphics
- Wi-Fi
- Ethernet
- Bluetooth
- audio
- input devices
- filesystems
- virtualization
- x86-64
- ARM64
- RISC-V potential
- thousands of hardware drivers

We keep that enormous advantage.

But Linux does **not** define the native application model.

For native software:

- POSIX is not the API.
- Processes are not the fundamental abstraction.
- File descriptors are not the universal object abstraction.
- Paths are not the security model.
- Threads are not the primary concurrency abstraction.
- Signals are not the primary notification mechanism.
- Containers are not required for isolation.
- Installation does not mutate a shared filesystem.
- Applications do not receive ambient access to the user's machine.
- System state is not an accumulation of mutable files.
- AI does not operate by pretending to be a human moving a mouse.
- Hardware topology is not assumed to remain CPU + RAM + GPU forever.

The fundamental abstractions become:

- `Component`
- `Task`
- `TaskGroup`
- `Object<T>`
- `Capability<T>`
- `Channel<T>`
- `MemoryObject`
- `ResourceDomain`
- `Operation`
- `Package`
- `SystemGeneration`
- `ComputeDevice`
- `Interface<T>`

---

# 2. Core Architectural Principle

## Preserve mature mechanisms. Replace inherited semantics.

We do **not** replace mature subsystems merely because they are old.

Linux's:

- scheduler
- virtual memory
- networking
- block layer
- filesystem implementations
- driver infrastructure
- hardware support

are extremely sophisticated.

The opportunity is not:

> Rewrite Linux faster.

It is:

> Stop requiring native software to use abstractions Linux is forced to preserve.

The project should aggressively distinguish between:

### Mechanisms worth preserving

Examples:

- page tables
- memory allocators
- TCP implementations
- NVMe support
- GPU drivers
- interrupt handling
- hardware discovery

### Semantics worth reconsidering

Examples:

- process model
- thread model
- file descriptors
- global filesystem namespace
- `fork()`
- signals
- ambient permissions
- package mutation
- global dependency installation
- container-based isolation

---

# 3. Compatibility Rule

## Compatibility inherits history. Native software does not.

This is the architectural firewall that prevents the project from eventually becoming Linux again.

Existing Linux applications may see:

- processes
- PIDs
- file descriptors
- signals
- `/proc`
- `/sys`
- sockets
- `fork()`
- `exec()`
- `epoll`
- POSIX paths
- Linux syscalls

Existing Windows applications may see:

- Win32
- NT objects
- Windows processes
- Windows handles
- DirectX
- Windows filesystem semantics
- registry semantics

Native applications see **none of this unless explicitly using a compatibility environment**.

The kernel's native interfaces must never exist merely because POSIX, Linux, or Windows has an equivalent concept.

---

# 4. High-Level Architecture

                         APPLICATIONS

             ┌──────────────┼──────────────┐
             │              │              │
          Native          Linux          Windows
           Apps            Apps            Apps
             │              │              │
      Native Platform   Linux ABI      Win32/NT
             │          Personality    Personality
             │              │              │
             └──────────────┼──────────────┘
                            │
                  Native Kernel Objects
                            │
               ┌────────────┴────────────┐
               │                         │
          User Services            Kernel Core
               │                         │
       ┌───────┼────────┐        ┌───────┼─────────┐
       │       │        │        │       │         │
      UI    Storage   Network   Memory   IPC   Capabilities
                                  │       │
                                  └── Scheduler
                                          │
                              Linux-derived hardware layer
                                          │
                         ┌────────────────┼────────────────┐
                         │                │                │
                        GPU             NVMe             Wi-Fi
                         │                │                │
                                      Hardware

---

# 5. Kernel Strategy

## 5.1 Fork Linux

Fork a contemporary Linux kernel.

Initially preserve as much as practical:

- boot
- architecture support
- interrupts
- memory-management internals
- scheduler
- PCI
- USB
- ACPI
- DRM/KMS
- networking drivers
- storage drivers
- input
- Bluetooth
- KVM
- hardware discovery
- firmware loading
- power management
- mature filesystems

Do not attempt a heroic rewrite.

The initial objective is to establish an entirely new native application model while retaining Linux's hardware maturity.

---

# 6. Kernel Evolution

## Phase A — Linux-derived foundation

Remain reasonably close to upstream.

Add the native platform alongside Linux facilities.

Linux applications continue functioning normally.

## Phase B — Dual personality

Two execution worlds exist:

    Native ABI
        +
    Linux ABI

Native applications no longer depend on Linux syscall semantics.

Linux becomes a compatibility personality.

## Phase C — Native primitives take over

Native abstractions begin as wrappers around mature Linux internals.

Example:

    Native Component
          │
          ├── task_struct
          ├── mm_struct
          ├── namespaces
          └── cgroups

Later:

    Native Component
          │
          └── native kernel implementation

The abstraction stays stable while the implementation evolves.

## Phase D — Controlled divergence

Major kernel divergence becomes acceptable.

Full upstream merges are no longer mandatory.

Linux becomes a source of:

- driver improvements
- security fixes
- architecture improvements
- algorithms
- scheduler ideas
- memory-management improvements
- subsystem patches

Changes are adapted rather than blindly merged.

## Phase E — Independent Linux-derived kernel

The kernel is no longer reasonably described as Linux.

Linux ancestry remains obvious, but the native ABI, execution model, security model, IPC architecture, and higher-level system semantics are independent.

---

# 7. Native Kernel Object Model

The native kernel exposes typed objects.

Conceptually:

    Object<T>

Examples:

    Object<File>
    Object<Directory>
    Object<Channel<Message>>
    Object<Surface>
    Object<Memory>
    Object<ComputeQueue>
    Object<NetworkConnection>
    Object<AudioStream>
    Object<Camera>
    Object<Component>
    Object<TaskGroup>
    Object<Device>
    Object<Package>

Userspace receives:

    Capability<T>

A capability establishes:

1. what object is being referenced
2. what operations the holder may perform
3. how the object may be transferred or delegated

Capabilities are:

- unforgeable
- typed
- transferable where permitted
- revocable where appropriate
- attenuable
- inspectable
- auditable

Example:

    Capability<File, ReadWrite>

may derive:

    Capability<File, Read>

but never:

    Capability<File, Admin>

without explicit authority.

---

# 8. Hardware-Assisted Capability Future

The capability design must not assume capabilities will always be enforced purely in software.

The ABI should leave room for future architectures such as:

- CHERI-like pointers
- tagged memory
- bounds-enforced pointers
- hardware provenance
- memory capabilities
- secure enclaves
- future memory-safe CPU architectures

Application-visible capabilities should remain conceptually stable regardless of whether enforcement occurs through:

- kernel metadata
- page tables
- CPU tags
- hardware capabilities
- combinations of the above

This avoids freezing the OS around the security limitations of current x86-64 hardware.

---

# 9. Security Model

## 9.1 No ambient authority

An application does not automatically receive:

- filesystem access
- home directory access
- network access
- camera access
- microphone access
- process enumeration
- device access
- clipboard access
- location
- screen contents
- other application data

It starts with a small capability set.

Example:

    PhotoEditor

    Capabilities:
      UI
      GPU
      temporary-storage

It does NOT initially have:

    ~/Pictures
    ~/Documents
    network
    camera
    microphone

The user chooses a photo.

The OS grants:

    Capability<Image, ReadWrite>

for that object.

The application never needed permission to access the user's whole Pictures directory.

---

# 10. Components

The primary isolation primitive is:

    Component

A component contains:

    Component
      ├── Code
      ├── AddressSpace
      ├── Capabilities
      ├── Inputs<T>
      ├── Outputs<T>
      ├── TaskGroup
      └── ResourceDomain

Components must be **extremely cheap to create**.

Isolation should be cheap enough that developers do not ask:

> Is sandboxing this worth the overhead?

The default answer should simply be:

> isolate it.

Target order of magnitude:

    creation: tens to hundreds of microseconds

subject to actual implementation measurements.

---

# 11. Applications Are Component Graphs

Example browser:

    Browser
      │
      ├── UI
      │
      ├── Network
      │
      ├── Tab
      │    ├── Renderer
      │    └── JavaScript Runtime
      │
      ├── Tab
      │    ├── Renderer
      │    └── JavaScript Runtime
      │
      └── Download Manager

Each component gets only the authority it requires.

Example image decoder:

    ImageDecoder

    Capabilities:
      Input<ImageBytes>
      Output<Bitmap>
      Memory: 256 MB
      CPU: 25%

No:

- network
- arbitrary filesystem
- microphone
- process enumeration
- package installation

An exploit should compromise the smallest useful unit of software.

---

# 12. Component ABI

The component ABI must be designed for long-term evolution.

It must support:

- typed interfaces
- interface versioning
- optional methods
- backwards-compatible schema evolution
- forwards-compatible extension
- feature negotiation
- asynchronous methods
- streams
- futures
- ownership transfer
- cancellation
- failure semantics

Do not design a typed IPC system without simultaneously designing how interfaces evolve across decades.

---

# 13. WebAssembly Component Compatibility

The WebAssembly Component Model and WASI should be treated as major sources of architectural inspiration.

Do not automatically duplicate concepts that already have strong ecosystem momentum.

Potential role:

    Native machine-code components
           +
    Portable WebAssembly components

Wasm should be considered a first-class format for:

- plugins
- sandboxed extensions
- portable services
- automation modules
- downloaded components
- cross-architecture workloads

But Wasm should **not** automatically become the native machine ABI.

Maximum-performance native software should remain free to compile directly to machine code.

The native component model should ideally map cleanly enough that Wasm components can participate naturally in the same object/capability/interface ecosystem.

---

# 14. IPC

IPC is one of the highest-priority performance subsystems.

Primary abstraction:

    Channel<T>

Example:

    Channel<ImageDecodeRequest>

Services expose typed interfaces:

    interface ImageDecoder {
        async decode(Image) -> Bitmap;
    }

Not:

    socket
    byte[]
    arbitrary serialization format
    hand-written protocol
    duplicated schema
    hope both sides agree

Tooling generates:

- wire representation
- stubs
- ownership semantics
- version negotiation
- tracing metadata

---

# 15. IPC Performance Strategy

Small messages:

    sender
      │
      │ minimal-copy fast path
      ▼
    receiver

Possible implementation techniques:

- shared ring buffers
- register transfer
- scheduler-aware handoff
- batched submission
- lock-free queues

Large payloads:

    Component A
        │
        │ transfer MemoryObject capability
        ▼
    Component B

The payload itself should not move when avoidable.

---

# 16. Memory Objects

Large data is represented by:

    MemoryObject

A `MemoryObject` can carry properties such as:

- writable
- immutable
- shared
- copy-on-write
- DMA-compatible
- GPU-compatible
- executable
- persistent
- pinned
- encrypted

Memory movement should prefer capability/ownership transfer over copying.

---

# 17. Zero-Copy Dataflow

Traditional:

    Network
       ↓
    kernel buffer
       ↓ copy
    application
       ↓ copy
    decoder
       ↓ copy
    graphics
       ↓
    GPU

Target:

    NIC
     ↓ DMA
    MemoryObject
     ↓ ownership transfer
    Decoder
     ↓ ownership transfer
    Renderer
     ↓
    GPU

Same underlying physical memory where hardware permits.

The system should understand:

- ownership
- borrowing
- sharing
- immutable mappings
- copy-on-write
- DMA suitability
- device locality
- NUMA locality
- GPU accessibility

---

# 18. Async-First Kernel Interface

Native APIs are asynchronous by default.

Not:

    read()
    block thread
    wake thread
    write()
    block thread

Instead:

    submit(operation)

Operations include:

    Read
    Write
    Receive
    Send
    Connect
    Accept
    Timer
    Wait
    GPUDispatch
    DeviceOperation
    StorageTransaction

Completion integrates directly with task scheduling.

---

# 19. Operations

`Operation` is a first-class concept.

Example:

    Operation<Result>

An operation supports:

- cancellation
- completion
- deadline
- priority
- tracing
- ownership
- resource accounting

Potential APIs:

    operation.cancel()
    operation.deadline(...)
    operation.await

This gives the OS a uniform representation of outstanding work.

---

# 20. Tasks

Applications should support enormous numbers of lightweight tasks.

Example:

    500,000 tasks

must NOT imply:

    500,000 kernel threads

The kernel/runtime cooperate to multiplex tasks across execution contexts.

Conceptually:

    async fn serve(request: Request) {
        let row = database.get(request.id).await;
        request.respond(row).await;
    }

The platform understands that the task is suspended waiting for a specific operation.

---

# 21. Structured Concurrency

Tasks have ownership.

    Application
        │
        ├── TaskGroup
        │      ├── Task
        │      ├── Task
        │      └── Task
        │
        └── Component
               │
               └── TaskGroup
                      ├── Task
                      └── Task

Cancellation propagates through the hierarchy.

    app.cancel()

can deterministically cancel owned work.

Persistent background execution requires an explicit capability.

No accidental orphan processes.

---

# 22. Scheduling Intent

The scheduler should understand more than generic priority.

`ResourceDomain` and tasks should be able to express intent such as:

    Interactive
    Background
    Throughput
    LowLatency
    Realtime
    EnergyEfficient
    Deadline

Examples:

- audio rendering → strict low latency
- compositor → interactive/deadline-sensitive
- game render thread → latency-sensitive
- compiler → throughput
- indexing → background
- battery sync → energy-efficient

This intent can influence:

- CPU scheduling
- GPU scheduling
- frequency scaling
- core selection
- memory placement
- I/O scheduling

---

# 23. Resource Domains

Resource accounting is native.

    ResourceDomain
      ├── CPU policy
      ├── Memory budget
      ├── GPU budget
      ├── I/O budget
      ├── Network policy
      ├── Storage quota
      ├── Energy policy
      └── Latency policy

Applications/components belong to resource domains.

This subsumes much of what currently requires combinations of:

- cgroups
- namespaces
- container runtimes
- scheduler configuration
- vendor-specific GPU controls

---

# 24. Observability as Architecture

Every major OS primitive should be observable.

Examples:

    Component
    Task
    TaskGroup
    Channel
    Capability
    Operation
    MemoryObject
    ResourceDomain
    ComputeQueue

The system should expose:

- creation
- ownership
- relationships
- resource use
- latency
- queueing
- failures
- capability grants
- IPC flow
- scheduling delays

Tracing must be:

- extremely low overhead
- structured
- semantic
- dynamically enabled
- security-aware

The objective is to gain the power of modern tracing/eBPF-style observability without reconstructing application semantics from raw kernel events whenever the OS already knows them.

---

# 25. Storage Model

Humans still get files and folders.

Applications do not receive a universal filesystem namespace by default.

Instead:

    File
    Directory
    Collection
    Blob
    PackageData
    ApplicationData
    UserSelected<T>

Example:

    let photo = files.choose<Image>().await?;

The chooser belongs to the OS.

The returned object carries authority.

The application does not need:

    /home/jake/Pictures/**

---

# 26. Storage Architecture

Storage should support:

- copy-on-write
- snapshots
- deduplication
- checksums
- immutable versions
- cheap clones
- atomic replacement
- transactions
- integrity verification

Initially this can be implemented over an existing mature Linux filesystem.

Do **not** write a new filesystem merely because the overall OS is new.

A native filesystem/object store should only be pursued when its semantic or performance benefits justify the cost.

---

# 27. Content-Addressed Objects

Immutable data should preferentially be identified by content.

Example:

    sha256:8f3ab...

Benefits:

- deduplication
- integrity
- deterministic package identity
- safe caching
- snapshot efficiency
- reproducibility

This model applies naturally to:

- packages
- binaries
- assets
- dependencies
- system generations
- cached build outputs

---

# 28. Packages

Applications are immutable packages.

    Package
      ├── Manifest
      ├── Components
      ├── Resources
      ├── Interfaces
      ├── RequestedCapabilities
      └── Dependencies

Installation means making a package available.

It does NOT mean:

    copy DLL here
    write registry there
    install random daemon
    modify system config
    drop files across shared directories

---

# 29. Dependency Model

Dependencies are immutable objects.

Multiple versions can coexist naturally.

    App A
       └── Library 1.4

    App B
       └── Library 2.1

No global dependency conflict.

Identical objects are deduplicated automatically.

---

# 30. System Generations

The operating system is immutable/versioned.

Example:

    Generation 1842
      ├── kernel
      ├── shell
      ├── compositor
      ├── services
      └── packages

An update creates:

    Generation 1843

It does not destroy 1842.

If boot fails:

    boot Generation 1842

Rollback must be a first-class operation rather than emergency tooling.

---

# 31. System History

State changes become explicit events.

Example:

    $ system history

    20:41 Installed Blender
    19:32 Updated IDE
    17:08 Changed project environment
    14:22 OS update
    12:10 Driver update

Potential operation:

    $ system restore 17:08

Long-term ambition:

restore some or all of:

- OS version
- applications
- package versions
- configuration
- user data
- workspaces
- application state

without maintaining complete duplicate machines.

---

# 32. Recovery Model

Failure and restart semantics should be part of interface design.

Critical userspace services should be restartable where practical:

- compositor
- audio server
- network services
- Bluetooth
- printing
- device-management services
- selected GPU userspace services

Clients should support:

    disconnect
    rebind
    retry
    restore-state

rather than assuming service death destroys the whole session.

Example:

    compositor crashes
          ↓
    compositor restarted
          ↓
    surfaces rebound
          ↓
    applications continue

Where hardware or driver constraints prevent seamless recovery, degraded recovery should still be explicit.

---

# 33. User-Space Drivers

Drivers should live outside the kernel where practical.

Good candidates may include:

- Bluetooth
- some USB devices
- audio devices
- sensors
- printers
- peripheral classes
- higher-level GPU services

Kernel residency remains acceptable where required for:

- latency
- DMA safety
- interrupt performance
- hardware architecture
- compatibility with inherited Linux drivers

The project does not pursue microkernel purity.

The goal is:

> isolate failure wherever the performance cost is acceptable.

---

# 34. Application Startup

Application startup should exploit immutable packages.

Executables and dependencies can be:

- memory mapped
- verified once
- cached aggressively
- shared
- precompiled
- deduplicated

Target:

    click → visible UI

should feel effectively immediate.

Initial aspirational target:

    simple native app < 20 ms warm startup

This is a measurement target, not a guaranteed architectural claim.

---

# 35. Native Development Environments

Development isolation is an OS primitive.

No mandatory:

- Docker daemon
- Linux VM
- overlay filesystem stack
- image extraction
- Docker Desktop

Project definition:

    environment.yaml

Example:

    runtime:
      php: 8.6

    services:
      postgres: 19
      redis: 10

    resources:
      memory: 4GB
      cpu: 4

Command:

    os env enter

The OS creates:

    ResourceDomain
    + StorageSnapshot
    + CapabilityNamespace
    + NetworkNamespace
    + Components

Target:

    cached environment startup < 50 ms

where practical.

---

# 36. Containers Become Compatibility

OCI containers remain supported.

But they belong to the Linux compatibility environment.

Native software should rarely need them.

Native isolation should be cheaper and more fundamental.

---

# 37. Heterogeneous Compute

The platform must not assume the future is primarily CPU execution.

Expose:

    ComputeDevice

Possible devices:

    CPU
    GPU
    NPU
    DSP
    FPGA
    accelerator
    future hardware

Workloads express requirements and preferences:

    latency
    throughput
    energy
    precision
    memory requirements
    locality

Example:

    compute.dispatch(
        workload,
        preference: Throughput
    )

The platform can select an appropriate execution resource.

---

# 38. Future Hardware Escape Hatch

The native platform must avoid assuming:

- x86-64 forever
- coherent memory forever
- conventional DRAM forever
- fixed CPU/GPU separation
- uniform memory access
- local-only memory
- page tables exactly as they exist today

`MemoryObject`, `ComputeDevice`, `Task`, and capabilities should be abstract enough to accommodate:

- CXL memory
- persistent memory
- disaggregated memory
- unified CPU/GPU memory
- accelerator-local memory
- CHERI
- NPUs
- novel compute architectures

The goal is not merely:

> modern for 2026.

It is:

> difficult to fossilize at 2026.

---

# 39. Graphics

Initial implementation should retain Linux DRM/KMS and mature GPU drivers.

Native graphics APIs should not expose DRM directly.

Applications interact with abstractions such as:

    Surface
    Buffer
    RenderQueue
    ComputeQueue
    Display
    Frame

The compositor is a privileged user-space service.

---

# 40. Desktop Compositor

Build a new compositor specifically around the native object model.

Requirements:

- GPU accelerated
- extremely low latency
- variable refresh rate
- HDR
- fractional scaling
- multi-GPU awareness
- per-display scaling
- robust crash recovery
- remote surfaces
- explicit screen-sharing capabilities
- frame scheduling
- latency-aware presentation
- power-aware rendering

No application receives unrestricted screen capture.

---

# 41. Native UI Platform

Do not inherit X11.

Do not make Wayland the native API.

Wayland compatibility may exist for Linux applications.

Native applications use a new UI protocol.

Requirements:

- declarative UI support
- retained-mode rendering
- accessibility metadata
- semantic actions
- responsive layout
- GPU rendering
- animation
- input
- text
- drag/drop
- clipboard capabilities
- automation interfaces
- localization
- adaptive form factors
- high-DPI support

---

# 42. Semantic Application Interfaces

Applications may expose typed semantic interfaces.

Example:

    interface IDE {
        openProject(Project);
        searchSymbols(Query) -> Symbol[];
        runTests(TestSelection) -> TestRun;
    }

Example:

    interface Mail {
        search(Query) -> Message[];
        compose(Draft);
        reply(Message, Draft);
    }

These interfaces support:

- automation
- accessibility
- scripting
- cross-app integration
- AI
- testing

without pretending to be mouse/keyboard input.

---

# 43. Location-Independent Interfaces

`Channel<T>` and semantic interfaces should not hard-code the assumption that both endpoints live in the same process or even on the same machine.

Do **not** make distributed computing a kernel concern.

But interfaces should be designed so transports may eventually include:

    same component
    same machine
    VM
    remote machine

without changing application-level semantics unnecessarily.

Remote use must still honor:

- capabilities
- identity
- encryption
- latency
- failure semantics
- explicit user policy

---

# 44. AI-Native OS

AI is not embedded as an omnipotent privileged process.

It operates through capabilities like everything else.

User request:

> Find the Postgres backup branch I was working on yesterday and run its tests.

Potential action graph:

    Workspace.search(...)
          ↓
    Project capability
          ↓
    IDE.openProject(...)
          ↓
    IDE.runTests(...)

Actions are:

- typed
- permissioned
- logged
- revocable
- inspectable
- scoped
- composable

The assistant does not receive arbitrary access merely because it is part of the OS.

---

# 45. Automation

Semantic interfaces create an automation layer that does not depend on GUI scraping.

Example:

    when Download.completed(file):
        if file.type == Archive:
            Extractor.extract(file)

Or:

    on Meeting.start:
        Notes.create(meeting)

These operations use typed capabilities rather than keyboard macros.

---

# 46. Linux Compatibility Personality

Existing Linux software is critical.

Initially retain the Linux syscall ABI directly.

Long-term architecture:

    Linux ELF binary
          │
          ▼
    Linux Personality
          │
          ├── syscall translation
          ├── POSIX process model
          ├── fd namespace
          ├── signals
          ├── /proc
          ├── mounts
          └── Linux security semantics
          │
          ▼
    Native kernel primitives

Eventually constructs such as:

    fork()
    epoll
    signals
    pid namespaces

may become compatibility-layer behavior implemented over native primitives rather than core native abstractions.

---

# 47. Linux GUI Compatibility

Linux GUI applications require compatibility for:

- Wayland
- X11 where necessary
- PulseAudio/PipeWire
- D-Bus
- desktop portals
- system conventions

These belong to the Linux personality.

They must integrate into the native desktop so users do not perceive them as VM applications.

They should support:

- native windows
- clipboard
- drag/drop
- audio
- GPU acceleration
- file picker integration
- notifications
- task switching
- screen scaling

---

# 48. Windows Compatibility

Use Wine/Proton concepts as a starting point.

Architecture:

    Windows executable
           │
           ▼
       Win32 APIs
           │
           ▼
    Windows Personality
           │
       ┌───┼────┐
       │   │    │
      FS  GPU  Objects
       │   │    │
       └───┼────┘
           │
    Native Kernel API

Windows compatibility includes:

- PE executables
- Win32
- selected NT semantics
- Windows filesystem behavior
- registry emulation
- DirectX translation
- Windows process behavior

Gaming compatibility should be considered a major product objective.

---

# 49. Compatibility UX

Compatibility must feel native.

A user should be able to:

    double-click.exe

and have it appear like a normal application.

No obvious VM.

No separate desktop.

No compatibility wizard for ordinary software.

Windows and Linux applications should integrate with:

- taskbar
- launcher
- notifications
- clipboard
- audio
- graphics
- file chooser
- input
- accessibility where possible

---

# 50. Native Language Strategy

New kernel/platform code:

    Rust-first

Inherited mature Linux code:

    retain C where rewriting provides insufficient value

User-space SDKs should eventually support multiple languages.

Likely priority:

    Rust
    C
    C++
    Swift
    Kotlin
    C#
    TypeScript/Wasm

Do not bind the OS philosophically to a single language.

---

# 51. Memory-Safety Strategy

"Use Rust" is not sufficient as the complete strategy.

Memory safety should exist at multiple layers:

    language safety
    capability safety
    process/component isolation
    memory-object ownership
    hardware enforcement where available

Unsafe native code remains possible where needed.

But unsafe authority should be minimized and isolated.

---

# 52. Native SDK

The SDK should make the secure/fast path the easiest path.

Conceptual example:

    #[component]
    async fn main(ctx: Context) -> Result<()> {
        let window = ctx.ui.window().await?;

        let image = ctx.files
            .choose::<Image>()
            .await?;

        let bitmap = decode(image).await?;

        window.render(bitmap).await?;

        Ok(())
    }

This code should naturally result in:

- explicit capabilities
- structured task ownership
- async I/O
- typed IPC
- safe object lifetimes
- automatic tracing
- appropriate cancellation

Developers should not have to fight the OS to do the right thing.

---

# 53. Performance Philosophy

The project must avoid fake performance claims.

Linux is already extremely fast.

The goal is not:

    "everything is 2x faster"

The goal is:

> change semantics so some expensive operations cease to be expensive.

Examples:

### Isolation

Traditional:

    runtime daemon
    namespace setup
    overlay mount
    filesystem layers
    cgroups
    process startup

Native:

    create ResourceDomain
    create Component
    attach capabilities
    map immutable package
    schedule

### IPC

Traditional:

    serialize
    syscall
    copy
    deserialize

Potential native:

    typed message
    ownership transfer
    scheduler-aware handoff

### Startup

Traditional:

    lookup files
    load shared libraries
    resolve dependencies
    execute constructors

Native:

    map known immutable objects
    reuse verified pages
    launch

The principle is:

> Don't merely optimize expensive abstractions. Remove the need for them.

---

# 54. Performance Requirements

Every architectural claim should eventually have a benchmark.

Track at minimum:

    component creation latency
    task creation latency
    IPC round-trip
    throughput
    cross-core IPC
    MemoryObject transfer
    application startup
    environment startup
    filesystem/object access
    GUI input-to-photon latency
    compositor frame latency
    syscall compatibility overhead
    Windows compatibility overhead
    Linux compatibility overhead
    energy use

Compare against:

- Linux
- Windows
- macOS
- containers
- relevant language runtimes

Never claim performance superiority without measurement.

---

# 55. Driver Strategy

Drivers are the largest practical blocker.

Initial rule:

> Keep Linux hardware support working for as long as possible.

Do not destabilize:

- DRM
- PCI
- USB
- NVMe
- networking
- ACPI

while simultaneously building the native platform.

Long-term:

    inherited Linux drivers
            +
    userspace native drivers
            +
    selected rewritten drivers

The transition must be incremental.

---

# 56. Major Known Blockers

## 56.1 GPU Drivers

Probably the hardest hardware problem.

Need excellent:

- AMD
- Intel
- NVIDIA

support.

This strongly favors preserving Linux DRM/kernel driver infrastructure initially.

## 56.2 Windows Gaming Compatibility

Requires:

- Wine/Proton integration
- DXVK
- VKD3D
- anti-cheat realities
- input
- audio
- HDR
- VRR
- GPU integration

Do not underestimate this.

## 56.3 Linux Compatibility

Running binaries is easier than reproducing a complete Linux environment.

Real applications depend on:

- glibc
- systemd assumptions
- D-Bus
- procfs
- sysfs
- namespaces
- Wayland
- X11
- PipeWire
- portals
- filesystem conventions

Compatibility must be treated as a product, not a syscall shim.

## 56.4 Kernel Fork Maintenance

Eventually diverging from upstream means owning:

- CVE response
- driver adaptation
- subsystem maintenance
- architecture support
- hardware enablement

This is a permanent engineering cost.

## 56.5 Ecosystem

No OS wins solely because its kernel is beautiful.

Need:

- browser
- IDE
- terminal
- package ecosystem
- SDK
- documentation
- compatibility
- games
- media
- hardware support

Hence compatibility is existential.

---

# 57. What Not To Do

Do not:

- rewrite every Linux subsystem
- invent a new filesystem immediately
- build a brand-new GPU stack
- make everything userspace for ideological purity
- force every application into Wasm
- make everything distributed
- expose POSIX as the native ABI
- expose Wayland as the native UI API
- expose Linux syscalls as native APIs
- promise impossible speedups
- build AI before the semantic object model exists
- optimize benchmark trivia while ignoring desktop UX
- preserve upstream mergeability at the expense of the architecture
- break hardware support unnecessarily

---

# 58. Research Inspirations

We should explicitly study and steal good ideas from:

- Linux
- seL4
- Fuchsia/Zircon
- QNX
- Redox
- Genode
- Windows NT
- macOS/XNU
- Android
- Nix/NixOS
- WebAssembly Component Model
- WASI
- CHERI
- io_uring
- eBPF
- Erlang/BEAM supervision
- Rust async
- Tokio
- structured concurrency systems
- capability-oriented research OSes

The project should not care where an idea comes from.

The test is:

> Does it produce the strongest coherent system?

---

# 59. V0 Goal

V0 is **not** a usable desktop.

V0 proves the native execution model.

Required:

- boot kernel fork
- create native `Component`
- create native `Task`
- create `TaskGroup`
- create capabilities
- create typed channels
- asynchronous operation submission/completion
- `MemoryObject`
- ownership transfer
- resource domains
- process isolation
- cancellation
- tracing
- tiny native runtime
- basic Linux compatibility still intact

Demo:

    Native Component A
           │
           │ Channel<Request>
           ▼
    Native Component B
           │
           │ MemoryObject transfer
           ▼
    result

Measured:

- latency
- memory overhead
- creation cost
- context-switch behavior
- cross-core performance

---

# 60. V0.5 Goal

Prove the application model.

Build:

- native compositor
- window objects
- input
- basic declarative UI
- package format
- immutable installation
- basic capability-based file selection
- service restart/rebind
- simple system generations

Demo native applications:

    Terminal
    File Browser
    Text Editor
    Image Viewer

Linux GUI compatibility may still use existing Linux mechanisms underneath.

---

# 61. V1 Developer Preview

Goal:

> The OS is useful enough that its developers can build the OS while running the OS.

Requirements:

- stable native SDK
- self-hosted terminal
- editor/IDE
- Git
- browser
- networking
- native package manager
- dev environments
- Linux compatibility
- audio
- GPU acceleration
- power management
- suspend/resume
- debugging
- tracing tools

Daily-driving it internally becomes mandatory.

---

# 62. V2 Desktop Preview

Requirements:

- polished desktop shell
- multi-monitor
- HDR
- VRR
- Bluetooth
- Wi-Fi
- laptops
- batteries
- suspend
- external displays
- Linux applications
- Windows applications
- gaming proof-of-concept
- rollback
- snapshots
- native app store/repository
- semantic automation

Target hardware should remain deliberately constrained.

Example:

    selected AMD desktop
    selected Intel laptop
    selected AMD laptop

Do not initially promise universal PC compatibility.

---

# 63. V3 Public Alpha

Target:

- enthusiasts
- developers
- OS researchers
- gamers willing to experiment

Requirements:

- installer
- updater
- rollback
- recovery
- crash reporting
- hardware compatibility database
- package repository
- secure signing
- permissions UI
- Linux compatibility
- meaningful Windows compatibility
- polished desktop
- documentation

---

# 64. First-Class Developer Tooling

Build tools from the beginning:

    os inspect component
    os inspect task
    os inspect channel
    os inspect capability
    os inspect memory
    os inspect resource
    os trace
    os history
    os restore
    os env

Example:

    $ os inspect component 81

    ImageDecoder
    State: Waiting
    Memory: 34 MB
    CPU: 1.8%
    Capabilities:
      input:image-buffer
      output:bitmap
    Tasks:
      decode#193
    Waiting:
      Channel<ImageDecodeRequest>

The operating system should be unusually understandable while running.

---

# 65. Native ABI Philosophy

The native ABI is perhaps the project's most permanent decision.

Rules:

1. Keep the kernel ABI minimal.
2. Prefer high-level semantics in user-space services.
3. Make capabilities fundamental.
4. Make async fundamental.
5. Make ownership transfer fundamental.
6. Make versioning fundamental.
7. Do not expose implementation details unnecessarily.
8. Preserve escape hatches for future hardware.
9. Avoid POSIX-shaped APIs unless independently justified.
10. Expect this ABI to survive decades.

Kernel ABI mistakes are much harder to fix than SDK mistakes.

---

# 66. Stability Layers

Not everything should have identical compatibility guarantees.

Potential model:

    Layer 1
    Native kernel ABI
    Extremely small and extremely stable.

    Layer 2
    Core platform interfaces
    Strongly versioned.

    Layer 3
    Native SDK
    Evolves relatively quickly.

    Layer 4
    Frameworks
    Can evolve rapidly.

This avoids freezing every high-level idea forever.

---

# 67. Project Principles

## Principle 1

**Compatibility is a subsystem, not the architecture.**

## Principle 2

**Preserve mature mechanisms. Replace inherited semantics.**

## Principle 3

**Security comes from authority design, not endless deny lists.**

## Principle 4

**Isolation should be cheap enough to use everywhere.**

## Principle 5

**Async is a platform concept, not something every runtime reinvents.**

## Principle 6

**Ownership transfer beats copying.**

## Principle 7

**Immutable state beats mutation wherever practical.**

## Principle 8

**Human-facing files can remain without making paths the security model.**

## Principle 9

**Hardware diversity should be abstracted without hiding performance reality.**

## Principle 10

**Failure and recovery are normal system states.**

## Principle 11

**Observability is part of architecture, not debugging glue.**

## Principle 12

**Semantic interfaces beat GUI automation.**

## Principle 13

**AI receives capabilities, not omnipotence.**

## Principle 14

**Native architecture should accommodate future hardware without redesigning the world.**

## Principle 15

**Never replace something mature unless the new model provides meaningful benefit.**

## Principle 16

**Do not chase novelty for its own sake.**

## Principle 17

**Do not optimize expensive abstractions when we can eliminate them.**

---

# 68. The Litmus Test

Whenever designing a subsystem, ask:

> Are we doing this because it is the best model we know today, or because Unix/Windows/Linux historically did it this way?

If the answer is history:

reconsider it.

Then ask:

> Are we replacing something mature merely because our alternative feels newer?

If yes:

reconsider that too.

The goal is not maximal novelty.

The goal is maximal capability, coherence, safety, performance, maintainability, and adaptability.

---

# 69. Ultimate Target Architecture

                         NATIVE SOFTWARE
                               │
                               ▼
                  ┌────────────────────────┐
                  │    Native Platform     │
                  │                        │
                  │ Components             │
                  │ Tasks / TaskGroups     │
                  │ Capabilities           │
                  │ Typed Interfaces       │
                  │ Async Operations       │
                  │ Memory Objects         │
                  │ Resource Domains       │
                  │ Compute Devices        │
                  └───────────┬────────────┘
                              │
                              ▼
                  ┌────────────────────────┐
                  │ Linux-Derived Kernel   │
                  │                        │
                  │ Native ABI             │
                  │ Scheduler              │
                  │ Memory                 │
                  │ IPC                    │
                  │ Capability enforcement │
                  │ Hardware abstraction   │
                  └───────────┬────────────┘
                              │
                ┌─────────────┼─────────────┐
                │             │             │
             Drivers       Hardware       KVM
                │
                ▼
             PHYSICAL
             HARDWARE


             COMPATIBILITY PERSONALITIES

         ┌────────────────┴────────────────┐
         │                                 │
       Linux                            Windows
         │                                 │
    Linux/POSIX ABI                  Win32 / NT
         │                                 │
         └──────────── native kernel ──────┘

Compatibility software gets the past.

Native software gets the future.

---

# 70. Project Mission

Build:

> **A new desktop operating system with a Linux-derived hardware foundation, but a completely new native computing model centered on capabilities, cheap isolated components, structured asynchronous execution, typed interfaces, transferable memory ownership, immutable system state, heterogeneous compute, semantic automation, and hardware-independent abstractions — while making Linux and Windows software feel like native citizens through compatibility personalities.**

The defining technical strategy is:

> **Use Linux for everything Linux spent decades becoming exceptionally good at.**

> **Replace the assumptions that prevent us from designing the operating system we would choose today.**

And the defining constraint is:

> **Do not merely build the most modern operating system possible in 2026. Build an operating system deliberately designed not to fossilize at 2026.**
