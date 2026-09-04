# D-0148 · Decide the IDL: adopt WIT, FIDL, Cap'n Proto schema or design new
- Status: proposed
- Task: IPC-006
- Surfaces: none
- Layer: none
- Spikes: IPC-018
- Supersedes: none
- Superseded by: none
- Baseline: §12, §14, §13
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The IDL is the language every platform Interface is written in and switching after V1 would invalidate every binding (§12, §14, §13); V0 exit requires an accepted IDL Decision with rejected options.

## Options

### Option A · Adopt WIT
Summary: The WebAssembly Interface Type language is the IDL.
Consequences: Alignment with Wasm components; ownership transfer and Capability passing must be added.
Evidence: none

### Option B · Adopt FIDL
Summary: Fuchsia's IDL is adopted.
Consequences: Designed for handles, evolution and multi-language codegen; dependence on Fuchsia tooling.
Evidence: none

### Option C · Adopt Cap'n Proto schema
Summary: Cap'n Proto's schema language is adopted.
Consequences: Zero-copy encoding and mature codegen; its capability model differs from the native one.
Evidence: none

### Option D · Design a new IDL
Summary: A project IDL is designed.
Consequences: Exact fit for ownership, Capabilities and versioning; all tooling built from scratch.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
