# D-0078 · Decide explicit GPU synchronisation as the only path for native Surfaces
- Status: proposed
- Task: GFX-014
- Surfaces: none
- Layer: none
- Spikes: GFX-036
- Supersedes: none
- Superseded by: none
- Baseline: §39, §40
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Explicit GPU synchronisation must be decided once as the only path for native Surfaces so the protocol does not repeat Wayland's implicit-sync retrofit (§39, §40).

## Options

### Option A · Explicit-only
Summary: Every native Surface commit carries a timeline semaphore; commits without one fail.
Consequences: No implicit-sync path ever; personalities need a bridge.
Evidence: none

### Option B · Explicit with an implicit bridge for personalities
Summary: Native is explicit-only; personality buffers are imported through an implicit-sync bridge.
Consequences: Linux apps keep working; the bridge is a second sync path.
Evidence: none

### Option C · Implicit default
Summary: Implicit sync is the default with explicit as an option.
Consequences: Easiest for ported code; the native protocol inherits the retrofit problem.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
