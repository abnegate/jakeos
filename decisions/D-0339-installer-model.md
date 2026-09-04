# D-0339 · Decide how prefix installers become installed applications
- Status: proposed
- Task: WIN-033
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §28, §49
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
A setup.exe that writes a prefix must become a launcher-visible application and history event (Q-041, §28, §49).

## Options

### Option A · Treat the prefix as the application
Summary: The prefix is the app.
Consequences: Simple; coarse.
Evidence: none

### Option B · Scan Start Menu links after install
Summary: Scan shortcuts.
Consequences: Familiar; heuristics.
Evidence: none

### Option C · Require a native manifest wrapper
Summary: A manifest wrapper.
Consequences: Clean; effort.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
