# D-0314 · Decide whether every Task has kernel-visible identity
- Status: proposed
- Task: TSK-008
- Surfaces: none
- Layer: none
- Spikes: TSK-016
- Supersedes: none
- Superseded by: none
- Baseline: §20, §38, §65
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Whether every Task has kernel-visible identity is orthogonal to multiplexing (§20, §38, §65); S-008 stays prototyped.

## Options

### Option A · Every Task as Object<Task>
Summary: Tasks are kernel objects.
Consequences: Inspectable; a cost per Task.
Evidence: none

### Option B · Runtime identity with kernel visibility for observability and cancellation only
Summary: Runtime identity.
Consequences: Cheap; limited visibility.
Evidence: none

### Option C · Hybrid
Summary: Kernel identity for some Tasks.
Consequences: Balance; complexity.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
