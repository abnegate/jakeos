# D-0327 · Decide UI protocol model: retained scene tree, client Buffers, or hybrid
- Status: proposed
- Task: UIP-006
- Surfaces: S-015
- Layer: none
- Spikes: UIP-017
- Supersedes: none
- Superseded by: none
- Baseline: §12, §41, §65
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The UI protocol transport for S-015 must be accepted before protocol IDL and the four demo applications (§12, §41, §65); the spike is an input.

## Options

### Option A · Retained scene tree
Summary: Apps ship a scene tree.
Consequences: Accessibility and rebind for free; expressiveness limits.
Evidence: none

### Option B · Client Buffers
Summary: Apps render into Buffers.
Consequences: Flexible rendering; rebind harder.
Evidence: none

### Option C · Hybrid
Summary: Scene tree plus Buffers.
Consequences: Both; complexity.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
