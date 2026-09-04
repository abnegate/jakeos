# D-0033 · Decide the top-level build orchestrator for kernel and userspace
- Status: proposed
- Task: BLD-002
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §27, §50
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The orchestrator for a mixed C plus Rust kernel and a Rust-first userspace decides whether hermeticity, remote caching and bit-for-bit identity are properties of the graph or bolt-ons (§27, §50), before the one-command Linux-host build.

## Options

### Option A · Kbuild driven by Cargo workspaces
Summary: Kbuild builds the kernel and Cargo drives userspace with a thin top-level driver.
Consequences: Least friction with upstream Linux; hermeticity and caching are external.
Evidence: none

### Option B · Bazel or Buck2
Summary: A graph-based build system orchestrates everything.
Consequences: Hermetic, cached and reproducible by construction; kernel integration is heavy and upstream-hostile.
Evidence: none

### Option C · Derivation-based builder aligned with the content-addressed store
Summary: A Nix-style derivation builder shares identity with the Package store.
Consequences: Build outputs are store objects directly; developer ergonomics and IDE integration are weaker.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
