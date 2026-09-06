# ABI surface register

This register names every Layer 1, Layer 2 and Layer 3 ABI surface that can be frozen. It is owned by ABI for Layer 1, by the workstream in `Owner` for Layer 2 and Layer 3. State starts `open`. Reverse links (`Explored by`, `Decided by`, `Frozen by`) stay `none` until tasks exist. Layer 1 surfaces are prototyped through V0, become freeze candidates at V1, and freeze at V4. Layer 2 interface-evolution rules freeze at V1. Layer 3 never freezes in this register beyond the SDK v1 crate API named as a V1 freeze candidate. No Layer 1 surface is frozen before V4.

### S-001 · Object handle table
- Layer: L1
- Owner: ABI
- State: open
- Explored by: ABI-022, ABI-042, CAP-012, CAP-013, CAP-015, CAP-038, CAP-052
- Decided by: ABI-010, CAP-008
- Frozen by: none
The representation of a live kernel-object handle in a component's capability table: index width, generation, type tag and the rules for allocation, lookup and reuse.

### S-002 · Syscall and entry mechanism
- Layer: L1
- Owner: ABI
- State: open
- Explored by: ABI-019
- Decided by: ABI-008
- Frozen by: none
How a component enters the kernel: syscall instruction, register convention, dispatch table and the prohibition on POSIX-shaped entry points.

### S-003 · Capability rights encoding
- Layer: L1
- Owner: CAP
- State: open
- Explored by: CAP-012, CAP-014, CAP-015, CAP-027, CAP-038, CAP-039, CAP-040, CAP-052
- Decided by: CAP-010
- Frozen by: none
The rights word on `Capability<T, Rights>` so attenuation is a subset check and a future hardware-tag path can enforce it without kernel metadata.

### S-004 · Error model
- Layer: L1
- Owner: ABI
- State: open
- Explored by: ABI-020
- Decided by: ABI-009
- Frozen by: none
Typed error codes returned at the kernel boundary (`Error::Rights`, exhaustion, disconnect, timeout) and the rule that native errors are not errno values.

### S-005 · Operation ring ABI
- Layer: L1
- Owner: TSK
- State: open
- Explored by: TSK-014, TSK-015, TSK-017
- Decided by: TSK-007
- Frozen by: none
Submission and completion of `Operation<Result>`: ring layout, inline completion, deadlines, cancellation bits and how a cancelled operation never delivers a result.

### S-006 · MemoryObject mapping
- Layer: L1
- Owner: MEM
- State: open
- Explored by: MEM-011, MEM-030, MEM-031, MEM-043, MEM-049, MEM-056
- Decided by: MEM-017
- Frozen by: none
Map, unmap, ownership transfer, borrow, charging across ResourceDomains, and the properties (writable, immutable, DMA, GPU, persistent) without exposing page-table layout.

### S-007 · Component creation
- Layer: L1
- Owner: CMP
- State: open
- Explored by: CMP-015, CMP-016, CMP-032
- Decided by: CMP-021
- Frozen by: none
The create, start, destroy and panic/abort operations for a Component: address space from packages, initial capability set, TaskGroup and ResourceDomain, and the rule that a panic aborts only that component.

### S-008 · TaskGroup and cancellation
- Layer: L1
- Owner: TSK
- State: open
- Explored by: TSK-016
- Decided by: TSK-008
- Frozen by: none
Task and TaskGroup identifiers, spawn, structured cancellation propagation, and the bound that no Task outlives its group without an explicit background-execution capability.

### S-009 · ResourceDomain
- Layer: L1
- Owner: SCH
- State: open
- Explored by: SCH-012
- Decided by: SCH-002
- Frozen by: none
CPU share, memory budget, kernel-object limits and the typed exhaustion errors. Intent classes are expressed as policy inputs, not as a POSIX nice value.

### S-010 · Tracing event
- Layer: L1
- Owner: OBS
- State: open
- Explored by: OBS-010
- Decided by: OBS-003
- Frozen by: none
The event record emitted for every primitive: identity, timestamps, relationships, and the enable bit that keeps a disabled tracepoint off the hot path.

### S-011 · Version negotiation
- Layer: L1
- Owner: ABI
- State: open
- Explored by: ABI-021
- Decided by: ABI-016
- Frozen by: none
The Layer 1 handshake that identifies ABI version and features. An unknown newer field is accepted by an older receiver and an older message by a newer receiver. This is a kernel-entry handshake, not only an IDL message test.

### S-012 · Channel transport
- Layer: L1
- Owner: IPC
- State: open
- Explored by: IPC-017, IPC-018, SCH-011
- Decided by: IPC-003
- Frozen by: none
Endpoint handles, small-message fast path, capability and MemoryObject transfer, backpressure (bounded depth, sender behavior) and the rule that native IPC is not a socket.

### S-013 · IDL wire format
- Layer: L2
- Owner: IPC
- State: open
- Explored by: IPC-018, IPC-020
- Decided by: IPC-007
- Frozen by: none
The encoding of typed messages generated from the IDL: fields, handles, MemoryObject references, streams and futures.

### S-014 · Interface evolution rules
- Layer: L2
- Owner: IPC
- State: open
- Explored by: IPC-019
- Decided by: IPC-042
- Frozen by: none
How a Layer 2 interface adds optional methods, deprecates fields and remains forward and backward compatible. Freeze candidate at V1; core interface versions lock at V4.

### S-015 · UI protocol
- Layer: L2
- Owner: UIP
- State: open
- Explored by: UIP-017
- Decided by: UIP-006
- Frozen by: none
Windows, input, declarative widgets, clipboard, drag and drop, and accessibility metadata. Freeze candidate at V2. Not Wayland and not X11.

### S-016 · Input-method protocol
- Layer: L2
- Owner: TXT
- State: open
- Explored by: TXT-011
- Decided by: TXT-026
- Frozen by: none
Preedit, commit and surrounding text. Registered before the UI protocol freezes even though IME engines ship at V2.

### S-017 · Accessibility tree
- Layer: L2
- Owner: ACC
- State: open
- Explored by: ACC-004
- Decided by: ACC-002
- Frozen by: none
Role, name and state emitted by every widget, and the relation to semantic actions. The tree-model decision precedes the UI protocol freeze.

### S-018 · Package manifest
- Layer: L2
- Owner: PKG
- State: open
- Explored by: PKG-041
- Decided by: PKG-011
- Frozen by: none
Immutable package description: components, requested capabilities, dependencies, and reserved signature, signer and trust-policy fields. Freeze candidate at V2.

### S-019 · Component manifest
- Layer: L2
- Owner: CMP
- State: open
- Explored by: CMP-031
- Decided by: CMP-022
- Frozen by: none
Per-component Inputs, Outputs, capability requests and ResourceDomain hints inside a package. Freeze candidate at V2.

### S-020 · Service manifest
- Layer: L2
- Owner: SVC
- State: open
- Explored by: SVC-014
- Decided by: SVC-004
- Frozen by: none
Supervision: restart budget, rebind protocol, readiness reporting and the capabilities a system service is started with.

### S-021 · environment.yaml
- Layer: L2
- Owner: ENV
- State: open
- Explored by: ENV-021
- Decided by: ENV-008
- Frozen by: none
Development-environment declaration: packages, services, ResourceDomain, storage snapshot, capability and network namespaces.

### S-022 · Grant schema
- Layer: L2
- Owner: SEC
- State: open
- Explored by: CAP-026
- Decided by: SEC-007
- Frozen by: none
One-time, session, persistent and revocable-by-user grants, keyed on package identity plus publisher. The taxonomy decision precedes the package capability-request schema.

### S-023 · Semantic registry
- Layer: L2
- Owner: SEM
- State: open
- Explored by: SEM-003
- Decided by: SEM-004
- Frozen by: none
Typed interface discovery for Terminal, Editor and later applications. The AI broker depends on a done registry task.

### S-024 · Compositor surface protocol
- Layer: L2
- Owner: GFX
- State: open
- Explored by: GFX-032, GFX-033, GFX-036
- Decided by: GFX-012
- Frozen by: none
Surface, Buffer, Display and Frame objects over DRM/KMS without exposing DRM to applications. Explicit GPU synchronization is mandatory; there is no implicit-sync path.

### S-025 · Audio objects
- Layer: L2
- Owner: AUD
- State: open
- Explored by: AUD-008
- Decided by: AUD-002
- Frozen by: none
Native audio devices, streams, the low-latency path and per-application volume. PipeWire compatibility lives in the Linux personality.

### S-026 · Network capabilities
- Layer: L2
- Owner: NET
- State: open
- Explored by: NET-002
- Decided by: NET-006
- Frozen by: none
Listen, connect, Wi-Fi and firewall as typed capabilities. A component without a network capability cannot open a connection.

### S-027 · Storage objects
- Layer: L2
- Owner: STO
- State: open
- Explored by: STO-027
- Decided by: STO-012
- Frozen by: none
`Capability<File>`, `Capability<Directory>`, UserSelected, StorageTransaction durability and the typed change-notification Operation.

### S-028 · ComputeDevice
- Layer: L2
- Owner: HET
- State: open
- Explored by: HET-002, HET-010
- Decided by: HET-001
- Frozen by: none
Enumeration and dispatch of CPU, GPU and future accelerators with latency, throughput and energy preferences. The class taxonomy is open-ended.

### S-029 · Wasm component ABI
- Layer: L2
- Owner: WASM
- State: open
- Explored by: WASM-002
- Decided by: WASM-001
- Frozen by: none
How a Wasm component participates in a native Channel. Wasm is not the native machine ABI; the runtime stays in userspace.

### S-030 · Personality interfaces
- Layer: L2
- Owner: LNX
- State: open
- Explored by: LNX-009, LNX-010, LNX-060, WIN-014
- Decided by: LNX-003
- Frozen by: none
How the Linux and Windows personalities attach to native objects: opt-in capability, clipboard and file-chooser bridges, and the rule that personalities consume the native ABI and never extend it. WIN shares this surface for the Windows side.

### S-031 · SDK crate API
- Layer: L3
- Owner: SDK
- State: open
- Explored by: SDK-031
- Decided by: SDK-055
- Frozen by: none
The Rust SDK v1 crate surface with semver. Freeze candidate at V1. Rust `std` support, if any, lives at Layer 3 as a compatibility crate; no Layer 1 entry point is justified by `std`.

### S-032 · Clipboard capability
- Layer: L2
- Owner: UIP
- State: open
- Explored by: UIP-017
- Decided by: UIP-004
- Frozen by: none
Clipboard as a capability. X11 primary selection is emulated inside the Wayland and X11 bridge and never crosses into this surface.

### S-033 · UserSelected chooser
- Layer: L2
- Owner: STO
- State: open
- Explored by: STO-027
- Decided by: STO-018
- Frozen by: none
The OS-owned file chooser that returns `UserSelected<T>` through a trusted-UI surface applications cannot overlay or spoof.

### S-034 · Screen-capture capability
- Layer: L2
- Owner: GFX
- State: open
- Explored by: GFX-033
- Decided by: GFX-099
- Frozen by: none
Explicit screen-share and screen-record capability with a persistent indicator. An application without it receives a denied or black surface.


### S-035 · Inspection and trace interfaces
- Layer: L2
- Owner: OBS
- State: open
- Explored by: OBS-023
- Decided by: OBS-015
- Frozen by: none
Layer 2 inspect and trace Interface versions: `os inspect` object graphs, trace export schema and the evolution rules that keep old clients working after a version bump. Distinct from L1 tracing event S-010.

### S-036 · Media objects and codec interfaces
- Layer: L2
- Owner: MED
- State: open
- Explored by: MED-010
- Decided by: MED-007
- Frozen by: none
Layer 2 decoder, encoder, Camera, Frame and playback-session Interfaces. Codec Packages stay sandboxed Components; this surface is not a native in-kernel codec API.

<!-- roadmap:generated:begin status -->
| ID | Title | Status |
| --- | --- | --- |
| S-001 | Object handle table |  |
| S-002 | Syscall and entry mechanism |  |
| S-003 | Capability rights encoding |  |
| S-004 | Error model |  |
| S-005 | Operation ring ABI |  |
| S-006 | MemoryObject mapping |  |
| S-007 | Component creation |  |
| S-008 | TaskGroup and cancellation |  |
| S-009 | ResourceDomain |  |
| S-010 | Tracing event |  |
| S-011 | Version negotiation |  |
| S-012 | Channel transport |  |
| S-013 | IDL wire format |  |
| S-014 | Interface evolution rules |  |
| S-015 | UI protocol |  |
| S-016 | Input-method protocol |  |
| S-017 | Accessibility tree |  |
| S-018 | Package manifest |  |
| S-019 | Component manifest |  |
| S-020 | Service manifest |  |
| S-021 | environment.yaml |  |
| S-022 | Grant schema |  |
| S-023 | Semantic registry |  |
| S-024 | Compositor surface protocol |  |
| S-025 | Audio objects |  |
| S-026 | Network capabilities |  |
| S-027 | Storage objects |  |
| S-028 | ComputeDevice |  |
| S-029 | Wasm component ABI |  |
| S-030 | Personality interfaces |  |
| S-031 | SDK crate API |  |
| S-032 | Clipboard capability |  |
| S-033 | UserSelected chooser |  |
| S-034 | Screen-capture capability |  |
| S-035 | Inspection and trace interfaces |  |
| S-036 | Media objects and codec interfaces |  |
<!-- roadmap:generated:end -->
