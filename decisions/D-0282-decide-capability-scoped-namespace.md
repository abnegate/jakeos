# D-0282 · Decide replacing the global namespace with Capability-scoped storage objects
- Status: proposed
- Task: STO-012
- Surfaces: S-027
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §25, §67
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The §25 model ADR: applications see typed objects while humans keep files and folders through privileged holders (§25, §67).

## Options

### Option A · Per-component roots
Summary: Each Component has its own roots.
Consequences: Simple; hierarchy remains.
Evidence: none

### Option B · Object graphs
Summary: A graph of typed objects.
Consequences: Flexible; complexity.
Evidence: none

### Option C · Hybrid path facades for personalities
Summary: A graph plus path facades.
Consequences: Compatibility; two models.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
