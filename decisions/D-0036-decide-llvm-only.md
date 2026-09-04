# D-0036 · Decide LLVM/Clang as the sole C compiler and reject a custom compiler
- Status: proposed
- Task: BLD-004
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §50, §51
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Rust-in-kernel needs bindgen against libclang, and mixing GCC-built C with rustc-built Rust doubles the sanitizer and ABI matrix (§50, §51), so the C compiler policy must be fixed and a custom compiler rejected.

## Options

### Option A · LLVM/Clang as the sole C compiler
Summary: Clang is the only supported C compiler; GCC kernel builds are unsupported.
Consequences: One toolchain and one ABI matrix; some upstream GCC-only paths need attention.
Evidence: none

### Option B · Dual GCC plus Clang matrix
Summary: Both compilers are supported.
Consequences: Matches upstream Linux; doubles CI and sanitizer coverage.
Evidence: none

### Option C · Project-maintained compiler or forked LLVM
Summary: The project carries its own compiler fork.
Consequences: Full control over codegen; violates I-089 and is a permanent maintenance burden.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
