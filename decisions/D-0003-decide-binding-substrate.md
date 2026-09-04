# D-0003 · Decide the binding substrate: C-compatible ABI header plus IDL-generated language stubs
- Status: proposed
- Task: ABI-007
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §50, §65
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Languages must bind to Layer 1 without creating a second Native ABI (§50, §65), and the substrate must be chosen before the Rust runtime and the C header are built.

## Options

### Option A · C-compatible header plus IDL-generated stubs
Summary: A stable C-compatible ABI header is the language-neutral substrate and per-language stubs are generated from the IDL on top of it.
Consequences: Any language with a C FFI reaches Layer 1 through one surface; the header becomes a frozen artifact that must track the IDL exactly.
Evidence: none

### Option B · IDL-only substrate
Summary: The IDL is the only description of Layer 1 and every language binding is generated directly from it with no C header.
Consequences: No hand-maintained header can drift; languages without an IDL backend cannot bind at all, and debugging tools lose a conventional C view of the ABI.
Evidence: none

### Option C · Rust-native ABI
Summary: Layer 1 is defined in Rust types and other languages bind through whatever Rust exposes.
Consequences: Fastest path for the Rust runtime; non-Rust languages become second-class and Rust type layout leaks into the ABI contract.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
