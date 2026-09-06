# D-0333 · Decide WASI imports bound to native Capability
- Status: proposed
- Task: WASM-008
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §13, §9, §9.1
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
A Wasm Component's imports are its whole authority (§13). WASI Preview1 preopens hand a module ambient directories and sockets, which is POSIX-shaped ambient authority (§9.1, T-001, I-021). This decision lists which WASI Preview2 worlds bind to native Capabilities (clocks, random, UserSelected, NetworkConnection), which imports are forbidden, and that an undeclared import fails closed with `Error::Rights` and allocates no handle, so WASM-013 has a closed import table on S-029. V1-G12's Wasm channel prototype needs it.

## Options

### Option A · Documented WASI Preview2 subset bound to Capability, Preview1 forbidden
Summary: A documented Preview2 subset: `wasi:clocks` and `wasi:random` bound to kernel Objects, `wasi:filesystem` bound only to UserSelected objects the Component holds, `wasi:sockets` bound to `NetworkConnection` rights; Preview1 and preopens are absent.
Consequences: Every import is a Capability the Package requested, so a Wasm Component is exactly as powerful as its manifest says, and the mapping is written once for all languages. Modules that assume a preopened root or ambient sockets fail to instantiate until ported, which excludes part of the existing WASI ecosystem at V1.
Evidence: none

### Option B · Full WASI Preview2 including filesystem and socket worlds
Summary: Full WASI Preview2 including the filesystem and sockets worlds with conventional preopens.
Consequences: Most existing Wasm binaries run unchanged. Preopens and ambient sockets are exactly the POSIX shapes §9.1 and I-021 forbid, so the runtime would become the personality by another name; rejected for native Wasm Components and left to the Linux personality's own Wasm tooling.
Evidence: none

### Option C · No WASI worlds; only native IDL imports
Summary: No WASI worlds; Wasm Components import only native IDL interfaces through WIT bindings.
Consequences: The purest model with one interface language. Nothing compiled against WASI runs, so the Wasm ecosystem's portability argument (§13) is lost and every module is written for JakeOS alone.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
