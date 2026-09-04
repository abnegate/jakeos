# D-0298 · Decide monotonic versus wall-clock semantics for Operation deadlines across suspend
- Status: proposed
- Task: SVC-016
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: none
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
V1 introduces suspend, so which clock Operation deadlines use and whether suspended time counts must be decided.

## Options

### Option A · CLOCK_MONOTONIC-like clock that does not advance during suspend
Summary: A monotonic clock.
Consequences: Predictable; deadlines stretch.
Evidence: none

### Option B · CLOCK_BOOTTIME-like clock that does
Summary: A boottime clock.
Consequences: Real elapsed time; expiry on resume.
Evidence: none

### Option C · Separate clocks with explicit step notification
Summary: Two clocks.
Consequences: Precise; two clocks.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
