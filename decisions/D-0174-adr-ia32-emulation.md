# D-0174 · Decide whether ia32 emulation is retained
- Status: proposed
- Task: LNX-015
- Surfaces: none
- Layer: none
- Spikes: KRN-017
- Supersedes: none
- Superseded by: none
- Baseline: §46, §56.3
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Steam, its runtime and a large share of Windows titles under Wine are 32-bit and need ia32 emulation in the kernel (§46, §56.3). Syscall pruning in KRN (Phase B and later) could disable it, so this decision comes first and H-016 (`qemu-ia32`) holds the choice in CI (R-032). The maintainer's direction on Q-040 (32-bit Windows in 1.0 via WoW64) depends on retaining it.

## Options

### Option A · Retain ia32 in the fork and CI on H-016
Summary: ia32 emulation stays in the fork, pruning may never remove it before 1.0, and H-016 runs the ia32 userland tests on every kernel change.
Consequences: Steam, the Windows personality's WoW64 path and every 32-bit Linux title keep working, and the corpora can include them. The compat syscall table is a second surface pruning must preserve, and the personality ships a 32-bit userland (LNX-035) and multilib (LNX-086).
Evidence: `reports/spikes/KRN-017.md`

### Option B · Drop ia32 from 1.0
Summary: ia32 is removed from the fork before 1.0.
Consequences: A smaller kernel and one syscall table to translate. Steam does not run, WoW64 does not run, and the W corpora lose most of their entries; incompatible with the owner's direction on Q-040.
Evidence: `reports/spikes/KRN-017.md`

### Option C · ia32 only via VIRT fallback
Summary: ia32 exists only inside the VIRT fallback VM.
Consequences: The host kernel stays 64-bit only. Every 32-bit game runs in a VM with VM graphics and input latency, which is the experience §56 says must not be visible; rejected for games in particular.
Evidence: `reports/spikes/KRN-017.md`

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
