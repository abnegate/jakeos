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
D-0334 fixed Wasm's role; WASM-003 measures the candidate runtimes. This decision picks the userspace runtime and whether it runs inside each Component or as a shared host service (§13). In-kernel embedding is rejected on GPLv2 incompatibility with the candidate runtimes' licences and on I-046 (GAP-0024); the runtime is a trusted Component in the threat model (T-045), so its isolation and update path matter as much as its speed (§51).

## Options

### Option A · Wasmtime as a shared userspace host service
Summary: Wasmtime runs as a shared host-service Component that instantiates Wasm Components on behalf of Packages and exposes them as ordinary Components with Channels.
Consequences: One runtime to update and audit, compiled-module caching across applications, and the Component Model and WIT support Wasmtime leads on. The service is a shared trust boundary: a runtime escape (T-045) reaches every Wasm Component, and the service is on every Wasm call path.
Evidence: `reports/spikes/WASM-003.md`

### Option B · Wasmtime in-Component
Summary: Wasmtime is linked into each Wasm-hosting Component; every Wasm Component is its own native Component with an embedded runtime.
Consequences: An escape compromises only that Component, and the ResourceDomain budget is exact per Wasm Component. Each Component carries a copy of the runtime and its JIT, duplicating memory and start-up cost, and updates reach it only through each Package's rebuild.
Evidence: `reports/spikes/WASM-003.md`

### Option C · WAMR in-Component
Summary: WAMR linked into each hosting Component.
Consequences: Small footprint and an interpreter mode suited to constrained Components. Weaker Component Model support and a C code base under the C-library strategy (SDK-097), and slower than Wasmtime's compiled mode on the reference machines.
Evidence: `reports/spikes/WASM-003.md`

### Option D · Wasmer as a shared userspace host service
Summary: Wasmer as a shared host service.
Consequences: An alternative with several backends and a large package ecosystem. Component Model support lags Wasmtime, the governance history is less predictable, and the licence mix must be checked against D-0102.
Evidence: `reports/spikes/WASM-003.md`

### Option E · Custom userspace runtime
Summary: A custom runtime written for the native model.
Consequences: Exact fit with no external dependency. A Wasm runtime with a JIT is a multi-year security-critical project the roadmap has no room for before 1.0; recorded as rejected.
Evidence: `reports/spikes/WASM-003.md`

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
