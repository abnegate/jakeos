# D-0005 · Decide the Native ABI entry mechanism and the maximum count of kernel entry points
- Status: proposed
- Task: ABI-008
- Surfaces: S-002
- Layer: L1
- Spikes: ABI-019
- Supersedes: none
- Superseded by: none
- Baseline: §65
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
How a Component enters the kernel bounds the Layer 1 entry-point count (§65 rule 1) and must be decided against the B-009 reports from ABI-001, with S-002 recorded as prototyped.

## Options

### Option A · Syscall per Operation
Summary: Each Operation kind has its own syscall instruction entry.
Consequences: Familiar and debuggable; the entry-point count scales with Operation kinds and every submission pays a full kernel transition.
Evidence: none

### Option B · Shared submission page with rare doorbell syscalls
Summary: Operations are written to a shared submission page and the kernel is woken by a small number of doorbell syscalls.
Consequences: Very few entry points and batched submission; correctness depends on page layout, memory ordering and a wake-up policy that must be measured.
Evidence: none

### Option C · vDSO-style trampolines
Summary: The kernel maps trampoline code into each Component and entry goes through it.
Consequences: Entry mechanism can evolve without ABI change; trampoline code is itself an ABI artifact and complicates CHERI and tagged-memory futures.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
