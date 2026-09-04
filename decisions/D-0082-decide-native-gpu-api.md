# D-0082 · Decide the GPU API native applications render with
- Status: proposed
- Task: GFX-017
- Surfaces: none
- Layer: none
- Spikes: GFX-036
- Supersedes: none
- Superseded by: none
- Baseline: §39, §50
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The GPU API native applications render with must be decided before RenderQueue and SDK v1 (§39, §50), as a native Interface rather than a Linux syscall wrapper (I-005).

## Options

### Option A · Vulkan via Mesa inside the Component
Summary: Applications use Vulkan directly.
Consequences: Full GPU access and existing tooling; DRM objects inside the Component.
Evidence: none

### Option B · WebGPU-like native API over RenderQueue
Summary: A native IDL API modelled on WebGPU.
Consequences: Portable and Capability-shaped; less than full GPU feature access.
Evidence: none

### Option C · Both with Vulkan as the escape hatch
Summary: Native API by default with Vulkan available.
Consequences: Coverage; two APIs to support.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
