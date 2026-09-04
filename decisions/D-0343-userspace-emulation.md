# D-0343 · Decide that Win32 emulation stays in userspace
- Status: proposed
- Task: WIN-008
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §3, §48, §5.1
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Whether NT-object or PE-loading emulation in the GPLv2 kernel conflicts with LGPL Wine must be answered before bring-up (Q-050, §3, §48, §5.1).

## Options

### Option A · All Win32, NT and PE emulation in userspace
Summary: Userspace only.
Consequences: Clean licensing; performance.
Evidence: none

### Option B · PE loader in-kernel
Summary: A kernel PE loader.
Consequences: Speed; licence risk.
Evidence: none

### Option C · NT objects in-kernel
Summary: Kernel NT objects.
Consequences: Fidelity; licence risk.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
