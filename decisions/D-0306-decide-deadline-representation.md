# D-0306 · Decide deadline and timestamp representation in the Operation ABI
- Status: proposed
- Task: TSK-004
- Surfaces: none
- Layer: none
- Spikes: TSK-015
- Supersedes: none
- Superseded by: none
- Baseline: §18, §19, §65
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Every Operation carries a deadline, so clock domain, resolution and overflow horizon are stamped into S-005 while prototyped (§18, §19, §65).

## Options

### Option A · Monotonic clock that does not advance during suspend
Summary: A monotonic clock.
Consequences: Predictable timers; deadlines stretch across suspend.
Evidence: none

### Option B · Boot-time clock that does
Summary: A boottime clock.
Consequences: Real elapsed time; timers expire on resume.
Evidence: none

### Option C · Wall clock
Summary: A wall clock.
Consequences: Human-readable; steps and jumps break deadlines.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
