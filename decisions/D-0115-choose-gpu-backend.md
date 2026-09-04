# D-0115 · Decide GPU ComputeDevice backend among Vulkan, DRM, or deferral
- Status: proposed
- Task: HET-003
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §37, §39, §56.1, §57
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
GPU ComputeDevice dispatch needs an explicit backend so V2 does not ship a half-built API (§37, §39, §56.1, §57), retaining DRM/Mesa rather than a native GPU stack (I-045).

## Options

### Option A · Vulkan compute
Summary: Dispatch through Vulkan compute on Mesa.
Consequences: Portable across GPUs; Vulkan semantics leak into ComputeQueue.
Evidence: none

### Option B · DRM job interface
Summary: Dispatch through a lower-level DRM job interface.
Consequences: Closer to hardware; per-driver work.
Evidence: none

### Option C · Defer GPU ComputeDevice past 1.0
Summary: No GPU dispatch in 1.0.
Consequences: Nothing half-built; the V2 Throughput-on-GPU criterion is unmet.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
