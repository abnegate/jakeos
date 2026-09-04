# D-0032 · Decide the visible-UI measurement boundary
- Status: proposed
- Task: BEN-016
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §34, §54
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
V0.5 startup gates are incomparable unless the visible-UI boundary is fixed; B-016 cites Q-029 and this Decision answers it (§34, §54).

## Options

### Option A · First compositor presentation of a non-blank frame
Summary: Startup ends when the compositor presents the first non-blank frame.
Consequences: Measurable in software and comparable across apps; ignores display latency.
Evidence: none

### Option B · First client commit
Summary: Startup ends when the client commits its first frame.
Consequences: Cheapest to measure; a committed frame may not yet be visible.
Evidence: none

### Option C · First photodiode edge
Summary: Startup ends when a photodiode sees the display change.
Consequences: True end-to-end; needs lab hardware and is not comparable across systems.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
