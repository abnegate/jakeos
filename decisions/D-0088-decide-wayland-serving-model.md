# D-0088 · Decide whether Wayland is served by the compositor or by a bridge Component
- Status: proposed
- Task: GFX-020
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §40, §41, §47
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Whether LNX's Wayland bridge translates into the native UI protocol or the compositor speaks Wayland directly must land before the V0.5 Wayland application gate (§40, §41, §47).

## Options

### Option A · Compositor as Wayland server
Summary: The compositor implements Wayland alongside the native protocol.
Consequences: Fewest hops for Linux apps; Wayland lives in the core compositor.
Evidence: none

### Option B · Bridge Component translating Wayland into the native protocol
Summary: A separate Component speaks Wayland and forwards native protocol.
Consequences: Compositor stays native-only; an extra hop and a translation layer to maintain.
Evidence: none

### Option C · Nested Wayland inside the Linux personality only
Summary: A nested compositor runs in the personality.
Consequences: Full isolation; Linux windows live in a separate surface.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
