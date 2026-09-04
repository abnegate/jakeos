# D-0185 · Decide Wayland hosting and X11 via Xwayland
- Status: proposed
- Task: LNX-004
- Surfaces: none
- Layer: none
- Spikes: LNX-010
- Supersedes: none
- Superseded by: none
- Baseline: §41, §47, §57, §60
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
How a Wayland Linux app becomes a native compositor window, with X11 only via Xwayland, must be decided (§41, §47, §57, §60).

## Options

### Option A · Nested compatibility compositor
Summary: A nested compositor hosts Wayland apps.
Consequences: Isolation; an extra hop.
Evidence: none

### Option B · In-compositor Wayland serving
Summary: The compositor serves Wayland directly.
Consequences: Fast; Wayland in the core.
Evidence: none

### Option C · Translating bridge Component
Summary: A bridge translates Wayland to the native protocol.
Consequences: Native purity; a translation layer.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
