# D-0083 · Decide the NVIDIA support stance for 1.0
- Status: proposed
- Task: GFX-064
- Surfaces: none
- Layer: none
- Spikes: GFX-087
- Supersedes: none
- Superseded by: none
- Baseline: §56.1, §62
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
V3 Tier 1 adds an NVIDIA desktop decided here (§56.1, §62); proprietary userspace drivers assume ambient device access and may not fit behind capabilities.

## Options

### Option A · Open kernel modules plus NVK/Mesa
Summary: NVIDIA is supported only via open modules and NVK.
Consequences: Capability-mediated path preserved; feature and performance gaps.
Evidence: none

### Option B · Proprietary userspace via the Linux personality
Summary: The proprietary stack runs only inside the personality.
Consequences: Full features for Linux apps; native apps get nothing.
Evidence: none

### Option C · Unsupported
Summary: NVIDIA is not supported for 1.0.
Consequences: No commitment; excludes much hardware.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
