# D-0326 · Decide input routing and focus arbitration model for focused surfaces
- Status: proposed
- Task: UIP-005
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §9, §41, §60
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Pointer, keyboard, touch, pen and gamepad events must reach only the focused Surface with focus arbitrated without grabs (§9, §41, §60).

## Options

### Option A · Compositor-owned focus with delivery only to the focused Surface
Summary: The compositor owns focus.
Consequences: Simple; compositor policy.
Evidence: none

### Option B · Shell-owned focus arbitration with compositor as delivery path
Summary: The shell owns focus.
Consequences: Flexible; an extra hop.
Evidence: none

### Option C · Per-seat input-broker Component
Summary: A broker per seat.
Consequences: Isolation; latency.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
