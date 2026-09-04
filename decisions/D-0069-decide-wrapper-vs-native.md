# D-0069 · Decide the Phase A Component implementation strategy
- Status: proposed
- Task: CMP-010
- Surfaces: none
- Layer: none
- Spikes: CMP-015, CMP-016
- Supersedes: none
- Superseded by: none
- Baseline: §6, §10
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
V0 exit names the wrapper-versus-native Component decision as required, chosen from the two V0 creation spikes (§6, §10), with the Native ABI free of task_struct, mm_struct, cgroups and namespaces regardless.

## Options

### Option A · Thin wrapper
Summary: Component is a thin wrapper over Linux process structures.
Consequences: Lowest risk in Phase A; creation cost is Linux's.
Evidence: none

### Option B · Wrapper plus prewarmed templates
Summary: The wrapper is kept and creation is accelerated by prewarmed templates.
Consequences: Better creation cost without a native object; template pool to manage.
Evidence: none

### Option C · Early native object
Summary: A native Component object is built now.
Consequences: Native semantics early; a large Phase C item pulled into V0.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
