# D-0334 · Decide Wasm role versus native machine-code Components
- Status: proposed
- Task: WASM-001
- Surfaces: S-029
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §13, §57
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
V0 records how Wasm sits beside machine-code Components so later rungs do not treat Wasm as the Native ABI (§13, §57, I-046).

## Options

### Option A · Wasm as a first-class Component kind beside machine code
Summary: A first-class kind.
Consequences: Portability; two kinds.
Evidence: none

### Option B · Wasm only as an in-process plugin runtime
Summary: Plugins only.
Consequences: Simple; limited.
Evidence: none

### Option C · Wasm as the Native ABI
Summary: Wasm everywhere.
Consequences: Uniform; rejected.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
