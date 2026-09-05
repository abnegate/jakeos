# D-0033 · Decide the top-level build orchestrator for kernel and userspace
- Status: accepted
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
Option A. The kernel repository builds with Kbuild (Clang, pinned toolchain); the platform repository is a Cargo workspace. A thin orchestrator (scripts in the platform repository) composes kernel, runtime and packages into bootable images and system generations. No new build language is introduced before the content-addressed store exists.

## Consequences
- Reproducibility comes from pinned toolchains and content-addressed outputs (PKG), not from the build tool.
- The orchestrator is the natural place to adopt derivation-style builds later; that becomes a decision when PKG lands the store.
- Cross-language LTO between kernel C and Rust is available through the shared LLVM (D-0036).

## Rejected options and why
- Option B (derivation-based builder) rejected for now: the strongest reproducibility story, but a steep investment before V0 has anything to build; revisit when the content-addressed store exists.
- Option C (Bazel or Buck2) rejected: wrapping Kbuild and Rust-for-Linux in a hermetic build system is a project of its own.

## Follow-ups
Revisit the derivation-based builder when PKG ships the content-addressed store.
