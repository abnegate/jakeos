# D-0184 · Decide glibc /usr/lib interoperation with Packages
- Status: proposed
- Task: LNX-024
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §29, §46, §56.3
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Unmodified glibc and its dynamic loader search a global `/usr/lib` (and friends) for shared libraries; the native dependency model has no global directory (D-0222). V1 requires unmodified glibc to run (§46, §56.3), so the personality must present a library namespace that satisfies the loader while Packages remain per-Package content-identified objects (§29). This answers Q-022 and records that native software never searches `/usr/lib` (I-020).

## Options

### Option A · Personality-only /usr/lib view over Package contents
Summary: The personality synthesises a per-process `/usr/lib` view from the Package's declared Linux dependencies: each library object appears at the path and soname the loader expects.
Consequences: The loader is satisfied, two applications with different library versions each see their own `/usr/lib`, and nothing global exists. The view is built per process start (or per personality instance), soname aliasing and ld.so.cache expectations must be emulated, and libraries not declared as dependencies are simply absent.
Evidence: none

### Option B · Copied FHS tree
Summary: A conventional FHS tree is copied from a base distribution image into each personality instance and Packages install libraries into it.
Consequences: Every Linux tool works as on a distribution. Libraries are duplicated per instance, the tree is mutable and drifts, and Package immutability holds only at the instance boundary.
Evidence: none

### Option C · FHS as the native store
Summary: The native store is laid out as an FHS tree so `/usr/lib` is real.
Consequences: No view to build. A global mutable library namespace becomes the store, which D-0222 and I-020 forbid; recorded as rejected.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
