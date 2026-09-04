# D-0242 · Decide repository retention of past generations
- Status: proposed
- Task: REL-029
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §30, §31
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Rollback and restore promises are honest only if referenced objects remain downloadable (§30, §31).

## Options

### Option A · Retain every object referenced by a supported generation plus a published N-previous floor
Summary: Referenced objects plus a floor.
Consequences: Honest; storage.
Evidence: none

### Option B · Retain only the current channel head
Summary: Head only.
Consequences: Cheap; rollback breaks.
Evidence: none

### Option C · Retain everything forever
Summary: Everything.
Consequences: Complete; unbounded.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
