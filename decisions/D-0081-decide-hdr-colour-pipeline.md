# D-0081 · Decide the HDR and colour management pipeline
- Status: proposed
- Task: GFX-063
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §40, §62
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
V2 exit requires an accepted HDR and colour pipeline before HDR10 output and tone mapping land (§40, §62).

## Options

### Option A · scRGB linear compositing
Summary: Everything is composited in linear scRGB.
Consequences: One internal space; high bandwidth and no passthrough.
Evidence: none

### Option B · PQ/HLG passthrough with per-surface transforms
Summary: HDR surfaces pass through with per-surface colour transforms.
Consequences: Efficient for fullscreen video; mixed SDR and HDR needs careful blending.
Evidence: none

### Option C · Hybrid with per-plane hardware LUTs
Summary: Hardware LUTs per plane with a software fallback.
Consequences: Low power; depends on driver support per GPU.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
