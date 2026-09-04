# D-0086 · Decide compositor rendering backend policy: Vulkan-only or Vulkan plus GL
- Status: proposed
- Task: GFX-018
- Surfaces: none
- Layer: none
- Spikes: GFX-035
- Supersedes: none
- Superseded by: none
- Baseline: §40, §62, §56.1
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The compositor rendering backend must be evaluated against the V2 target machine list so older laptops are not excluded silently (§40, §62, §56.1); the GPU risk spike is the evidence.

## Options

### Option A · Vulkan-only
Summary: The compositor renders through a single Vulkan backend and requires a Vulkan-capable GPU.
Consequences: One rendering path to test and maintain; any listed machine without a working Vulkan driver is excluded from V2.
Evidence: none

### Option B · Vulkan plus GL fallback
Summary: A GL backend is kept for listed SKUs whose Vulkan driver is missing or broken.
Consequences: Older hardware stays supported; two backends must render identical output and both are on the golden-image gate.
Evidence: none

### Option C · Vulkan plus software fallback only
Summary: Vulkan is the only accelerated path and a software renderer covers headless CI and unsupported GPUs.
Consequences: Headless H-001 and H-003 run without a GPU; old GPUs without Vulkan get an unaccelerated desktop.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
