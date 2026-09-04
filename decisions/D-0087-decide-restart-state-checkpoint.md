# D-0087 · Decide which compositor state survives restart and where it is checkpointed
- Status: proposed
- Task: GFX-019
- Surfaces: none
- Layer: none
- Spikes: GFX-033
- Supersedes: none
- Superseded by: none
- Baseline: §32, §40
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Which compositor state survives restart and where it is checkpointed answers the §32 question for geometry, stacking, focus and workspace (§32, §40); Q-027 closes on acceptance.

## Options

### Option A · Kernel-owned window objects
Summary: Window state lives in kernel objects.
Consequences: Survives any userspace crash; kernel grows UI semantics.
Evidence: none

### Option B · Persistent broker Component
Summary: A small broker holds the checkpoint.
Consequences: Kernel stays minimal; the broker itself can crash.
Evidence: none

### Option C · Client-replayed state
Summary: Clients re-send their state after restart.
Consequences: No central store; restoration depends on client cooperation.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
