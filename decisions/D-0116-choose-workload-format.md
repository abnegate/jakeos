# D-0116 · Decide portable workload representation for heterogeneous dispatch
- Status: proposed
- Task: HET-004
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §13, §37, §57, §65
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
compute.dispatch needs a decided workload bytes layout before V2 placement (§13, §37, §57, §65); Wasm as the only native machine ABI is rejected by I-046 and S-028 is not frozen.

## Options

### Option A · SPIR-V only
Summary: Every workload is a SPIR-V module.
Consequences: Natural for GPUs and Vulkan compute; poor fit for CPU and NPU targets.
Evidence: none

### Option B · Wasm only
Summary: Every workload is a Wasm module compiled per device.
Consequences: Portable across every device class; rejected as the sole format by I-046 and slow on GPUs.
Evidence: none

### Option C · Native kernels per device
Summary: Each device class takes its own native binary.
Consequences: Best performance; no portability and a build matrix per device.
Evidence: none

### Option D · Multi-format envelope
Summary: An envelope carries one or more representations and the device picks one it can consume.
Consequences: Portability with native fast paths; the envelope format is itself a surface to version.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
