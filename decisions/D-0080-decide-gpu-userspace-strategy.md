# D-0080 · Decide the GPU userspace strategy from the Mesa-behind-capabilities Spike
- Status: proposed
- Task: GFX-016
- Surfaces: none
- Layer: none
- Spikes: GFX-036
- Supersedes: none
- Superseded by: none
- Baseline: §39, §9.1, §56.1
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
How Mesa is hosted decides whether native Components render without ambient DRM device-node access (§39, §9.1, §56.1); Q-034 closes when this is accepted.

## Options

### Option A · Unmodified Mesa with a brokered descriptor
Summary: Mesa runs in the Component with a render-node descriptor brokered by a privileged Component.
Consequences: No Mesa patches; the descriptor is a Linux object inside a native Component.
Evidence: none

### Option B · Patched Mesa WSI layer
Summary: Mesa's WSI is patched to use native Surface and Buffer.
Consequences: Native integration; a Mesa fork to maintain.
Evidence: none

### Option C · Mesa inside a Linux personality helper
Summary: Rendering happens in a personality helper process.
Consequences: Native Component never sees DRM; a cross-Component render hop.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
