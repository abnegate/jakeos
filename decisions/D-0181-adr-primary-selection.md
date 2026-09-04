# D-0181 · Decide X11 primary selection stays inside the bridge
- Status: proposed
- Task: LNX-020
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §41, §47, §57
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
X11 primary selection must be emulated inside the bridge and never cross into the native clipboard (§41, §47, §57, T-032).

## Options

### Option A · Primary selection emulated only inside the bridge
Summary: Primary lives only in the bridge.
Consequences: X11 apps work; no leak into native.
Evidence: none

### Option B · Merging primary into the native clipboard
Summary: Primary and clipboard merge.
Consequences: Convenience; a leak channel.
Evidence: none

### Option C · Dropping primary selection
Summary: Primary is not supported.
Consequences: Simplest; breaks X11 apps.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
