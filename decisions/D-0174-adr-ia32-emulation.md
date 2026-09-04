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
Whether ia32 emulation is retained must be decided before syscall pruning because Steam and many Windows titles depend on it (§46, §56.3).

## Options

### Option A · Retain ia32 in the fork and CI on H-016
Summary: ia32 stays and is tested.
Consequences: Steam and 32-bit titles work; maintenance and pruning constraints.
Evidence: none

### Option B · Drop ia32 from 1.0
Summary: ia32 is removed.
Consequences: Simpler kernel; breaks Steam and many titles.
Evidence: none

### Option C · ia32 only via VIRT fallback
Summary: 32-bit runs only in a VM.
Consequences: Clean kernel; poor experience for games.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
