# D-0332 · Decide the userspace Wasm runtime and host placement
- Status: proposed
- Task: WASM-007
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §13, §51
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The userspace Wasm runtime and host placement must be chosen; in-kernel embed is rejected on GPLv2 incompatibility (§13, §51).

## Options

### Option A · Wasmtime as a shared userspace host service
Summary: Wasmtime as a service.
Consequences: Sharing; a service.
Evidence: none

### Option B · Wasmtime in-Component
Summary: Wasmtime per Component.
Consequences: Isolation; duplication.
Evidence: none

### Option C · WAMR in-Component
Summary: WAMR per Component.
Consequences: Small; fewer features.
Evidence: none

### Option D · Wasmer as a shared userspace host service
Summary: Wasmer as a service.
Consequences: Alternative; ecosystem.
Evidence: none

### Option E · Custom userspace runtime
Summary: A custom runtime.
Consequences: Fit; effort.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
