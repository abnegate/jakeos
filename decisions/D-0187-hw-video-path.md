# D-0187 · Decide VA-API versus Vulkan Video for hardware codecs
- Status: proposed
- Task: MED-004
- Surfaces: none
- Layer: none
- Spikes: MED-005
- Supersedes: none
- Superseded by: none
- Baseline: §17, §37, §39, §57
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The hardware-decode path must be decided before V2 builds: which retained Mesa path is native, exposed later through ComputeDevice (§17, §37, §39, §57); the spike supplies measurements.

## Options

### Option A · VA-API on retained Mesa
Summary: Native decode goes through VA-API.
Consequences: Mature and widely supported; an older API with its own descriptor model.
Evidence: none

### Option B · Vulkan Video on retained Mesa
Summary: Native decode goes through Vulkan Video.
Consequences: Unified with the GPU API; less driver coverage today.
Evidence: none

### Option C · Both, selected per GPU
Summary: The path is chosen per GPU.
Consequences: Best coverage; two paths behind one Interface.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
