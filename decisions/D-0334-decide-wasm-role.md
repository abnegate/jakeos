# D-0334 · Decide Wasm role versus native machine-code Components
- Status: proposed
- Task: WASM-001
- Surfaces: S-029
- Layer: L2
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §13, §57
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
§13 makes Wasm modules cheap isolated components that reuse the Component Model and WASI; §57 forbids forcing everything into Wasm, and I-046 keeps native machine code first-class. This V0 decision fixes what a Wasm module is in the native model so the V1 integration knows whether it builds a Component kind, a plugin runtime or both, and so WIT and WASI are mapped once rather than duplicated under native names. S-029 is the WIT-to-IDL mapping surface at Layer 2.

## Options

### Option A · Wasm as a first-class Component kind beside machine code
Summary: A Wasm module is a Component kind beside machine code: the same Capabilities, Channels and ResourceDomain, loaded by a Wasm runtime Component with WIT imports mapped to native Interfaces once on S-029.
Consequences: Portable sandboxed components under the same authority model, with WIT and WASI reused as the import surface. Every native concept needs a Wasm binding, two component kinds appear in packaging and inspection, and the runtime is a trusted Component in the threat model (T-045).
Evidence: none

### Option B · Wasm only as an in-process plugin runtime
Summary: Wasm is a library a machine-code Component may embed to run plugins; the operating system does not know about Wasm.
Consequences: Nothing new in the platform. Portability stops at plugins inside one application, Wasm components cannot appear in packages or the store, and WASI mapping becomes each application's problem, so it is done several times differently.
Evidence: none

### Option C · Wasm as the Native ABI
Summary: Every application is a Wasm module and machine code exists only inside the runtime.
Consequences: One uniform target and strong sandboxing. Removes native performance, GPU and DMA paths and contradicts §57 and I-046 directly; recorded so it is never re-proposed.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
