# D-0036 · Decide LLVM/Clang as the sole C compiler and reject a custom compiler
- Status: accepted
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
Option A. LLVM/Clang is the sole C compiler for the kernel fork and all native code; GCC-only kernel configurations are dropped from the supported matrix. Rust and C share one LLVM version per release so cross-language LTO and a single CI matrix are possible.

## Consequences
- The kernel CI matrix is Clang-only (BLD); GCC breakage is not tracked.
- Kernel configs that require GCC plugins or GCC-only extensions are disabled or patched in the fork.
- Toolchain pinning (KRN-004) pins LLVM and Rust together.

## Rejected options and why
- Option B (GCC plus Clang matrix) rejected: doubles kernel CI cost and blocks cross-language LTO for no user-visible benefit on the chosen target hardware.
- Option C (project-maintained compiler) rejected: no kernel feature on the roadmap needs compiler changes; the maintenance cost would be permanent.

## Follow-ups
KRN-004 (accepted).
