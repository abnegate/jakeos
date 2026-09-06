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
Per-milestone Personality depth and the Phase B through D move from a direct Linux syscall ABI to translation onto native primitives must be decided (§6, §46, §56.3); the spike measures the options.

## Options

### Option A · In-kernel retain of the Linux syscall path
Summary: Linux syscalls are served directly.
Consequences: Fastest; no translation progress.
Evidence: none

### Option B · In-kernel translation onto native primitives
Summary: The kernel translates Linux syscalls onto native objects.
Consequences: Native mapping; kernel work.
Evidence: none

### Option C · gVisor-style userspace Personality
Summary: A userspace kernel translates.
Consequences: Isolation; overhead.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
