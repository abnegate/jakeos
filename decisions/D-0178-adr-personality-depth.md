# D-0178 · Decide Linux Personality depth and translation phase
- Status: proposed
- Task: LNX-003
- Surfaces: S-030
- Layer: L2
- Spikes: LNX-009
- Supersedes: none
- Superseded by: none
- Baseline: §6, §46, §56.3
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The Linux personality starts as the retained syscall ABI and must move through kernel phases B to D (§6) towards translation onto native primitives (§46) without regressing the L corpora (§56.3). LNX-009 measures the fork syscall path, an in-kernel translation prototype and a user-space personality against B-026. This decision names the depth in force at V1, V2 and V3, which fixes when LNX-090 and KRN-050 can start and what S-030 (the personality boundary at Layer 2) promises at each rung.

## Options

### Option A · In-kernel retain of the Linux syscall path
Summary: Linux syscalls are served by the retained kernel path at every rung; the personality is the unchanged Linux ABI with native services beside it.
Consequences: Fastest possible personality and zero regression risk for the corpora. Nothing is translated, so Linux processes never become Components with Capabilities, personality processes keep ambient authority, and the kernel-evolution phases stall at B; the 1.0 story for §46 is then untrue.
Evidence: `reports/spikes/LNX-009.md`

### Option B · In-kernel translation onto native primitives
Summary: The kernel translates Linux syscalls onto native objects: a Linux process is a Component, file descriptors are Capabilities, and the syscall table is an in-kernel compatibility layer.
Consequences: Personality processes gain the native security model and `os inspect` sees them as Components, which §46 asks for. The translation layer is kernel work in Rust on the hot path of every syscall, must pass the L corpora at each rung, and the phase at which it goes default (V2 by the roadmap) is a gate risk B-026 measures.
Evidence: `reports/spikes/LNX-009.md`

### Option C · gVisor-style userspace Personality
Summary: A user-space personality Component implements the Linux ABI (gVisor-style) over native Operations; the kernel exposes only native primitives to it.
Consequences: The kernel core is smallest and the personality is an ordinary supervised Component, isolated from the kernel by construction. Every syscall pays a Component boundary crossing, so the corpora regress on B-026 unless the fast paths of D-0142 make it cheap; ptrace, seccomp and io_uring inside the personality are re-implemented.
Evidence: `reports/spikes/LNX-009.md`

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
