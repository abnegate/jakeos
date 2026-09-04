# D-0114 · Decide the Workstream-split procedure at the size warning
- Status: proposed
- Task: GOV-043
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §65
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Files over the size warning are remedied by a GOV adr, so the split procedure, prefix permanence and See also must be recorded (§65).

## Options

### Option A · Split when the file exceeds the line warning
Summary: A workstream file is split mechanically when the warning fires.
Consequences: Predictable and tool-checkable; splits may cut across a coherent scope.
Evidence: none

### Option B · Split by sub-scope on a named condition
Summary: A file is split when a coherent sub-scope emerges, recorded in a GOV adr.
Consequences: Meaningful boundaries; judgement required each time.
Evidence: none

### Option C · Never split
Summary: New tasks stay in the original file regardless of size.
Consequences: No churn in IDs or files; unwieldy files that are hard to review.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
