# D-0180 · Decide the POSIX path view of native storage
- Status: proposed
- Task: LNX-019
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §25, §46
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
How the Personality presents native storage as POSIX paths while respecting Capability scope must be decided (§25, §46), answering Q-018.

## Options

### Option A · Capability-scoped path facade
Summary: Paths are visible only within the Capability scope.
Consequences: Correct authority; facade complexity.
Evidence: none

### Option B · Copy-on-first-use tree
Summary: Files are copied into a POSIX tree on first use.
Consequences: Simple; divergence.
Evidence: none

### Option C · Global FHS as native storage
Summary: FHS is the native store.
Consequences: Compatibility; rejected against I-016.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
