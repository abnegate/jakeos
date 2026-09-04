# D-0009 · Decide the Layer 1 freeze: accept the freeze ADR over the reviewed candidate set
- Status: proposed
- Task: ABI-049
- Surfaces: none
- Layer: none
- Spikes: ABI-042
- Supersedes: none
- Superseded by: none
- Baseline: §65, §66
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The Layer 1 freeze is accepted or rejected over the reviewed candidate set (§65, §66); I-040 forbids freezing earlier and after acceptance a Layer 1 change is a new major OS version.

## Options

### Option A · Freeze the full candidate set
Summary: Every reviewed candidate surface, including S-001, S-002, S-004 and S-011, is frozen.
Consequences: The ABI contract is complete at V4; any candidate lacking spike or benchmark evidence blocks the whole freeze.
Evidence: none

### Option B · Freeze a reduced core
Summary: Only the surfaces with complete evidence are frozen and the rest are explicitly deferred.
Consequences: Freeze lands on schedule; deferred surfaces remain unstable into 1.0 planning and must be listed one by one.
Evidence: none

### Option C · Defer the freeze to 1.0
Summary: No Layer 1 surface freezes at V4 and the freeze moves to the 1.0 rung.
Consequences: More time for evidence; SDK v1 and third parties build against surfaces that may still change.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
