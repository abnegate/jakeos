# D-0118 · Decide how ComputeDevice relates to ComputeQueue
- Status: proposed
- Task: HET-011
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §7, §37, §39
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Whether a ComputeQueue derives from a ComputeDevice Capability and who owns scheduling between them must be settled before Object<ComputeQueue> and GPU backends (§7, §37, §39), answering Q-033.

## Options

### Option A · ComputeQueue derived from Capability<ComputeDevice>
Summary: Queues are derived from a device Capability.
Consequences: Attenuation is natural; every queue holder had device authority.
Evidence: none

### Option B · Independent queue objects minted by the kernel
Summary: Queues are separate kernel objects.
Consequences: Queues can be handed out without device authority; two object types to relate.
Evidence: none

### Option C · GFX ComputeQueue as the GPU queue, CPU has no queue
Summary: The GPU queue is GFX's and CPUs dispatch without a queue.
Consequences: Reuse; asymmetric model across devices.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
