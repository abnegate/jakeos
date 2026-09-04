# D-0294 · Decide content-store garbage collection: root set, policy and user control
- Status: proposed
- Task: STO-041
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §27
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Content-store garbage collection must be answered before the V1 repository produces many generations (§27).

## Options

### Option A · Generation-count roots
Summary: Keep N generations.
Consequences: Predictable; not age-aware.
Evidence: none

### Option B · Age-based collection
Summary: Collect by age.
Consequences: Intuitive; may drop needed objects.
Evidence: none

### Option C · User-pinned roots
Summary: Users pin roots.
Consequences: Control; manual.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
