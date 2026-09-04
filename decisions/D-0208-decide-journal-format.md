# D-0208 · Decide the persistent journal record format and retention model
- Status: proposed
- Task: OBS-030
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §24, §30
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Persistent structured logs from previous boots need a format and retention model that respects the generation boundary (§24, §30).

## Options

### Option A · systemd-journal-compatible export
Summary: Journal format.
Consequences: Existing tooling; systemd shape.
Evidence: none

### Option B · Native typed records over the trace schema
Summary: Native records.
Consequences: Unified with tracing; new tooling.
Evidence: none

### Option C · Plain structured text
Summary: Text lines.
Consequences: Simple; weak indexing.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
