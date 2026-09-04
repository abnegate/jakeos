# D-0122 · Decide pragmatic driver residency over microkernel purity
- Status: proposed
- Task: HW-002
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §33, §55, §57
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
§33 and §55 forbid microkernel purity and flag-day driver replacement, so pragmatic residency must be recorded as a standing non-goal (§33, §55, §57).

## Options

### Option A · All drivers in user space
Summary: Every driver class is moved out of the kernel.
Consequences: Maximum isolation; unmeasured cost and a flag day per class that §55 forbids.
Evidence: none

### Option B · All drivers in-kernel
Summary: No driver class ever moves.
Consequences: Hardware support is never at risk; no isolation progress and no evidence gathered.
Evidence: none

### Option C · Pragmatic residency by measured cost per class
Summary: Each class moves only when a measured Decision names it, with a dual-path period.
Consequences: Evidence-driven and safe for hardware support; ongoing classification work per class.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
