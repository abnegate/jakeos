# D-0158 · Decide kernel strategy: Linux fork vs new microkernel vs Linux-as-hypervisor
- Status: proposed
- Task: KRN-002
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §1, §5, §57
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
V0 requires an accepted kernel strategy Decision recording the hardware-maturity versus native-model rationale (§1, §5, §57).

## Options

### Option A · Radical fork of Linux
Summary: Fork Linux and evolve it toward the native model.
Consequences: Mature hardware support from day one; Linux gravity pulls the native model back toward POSIX.
Evidence: none

### Option B · New microkernel with Linux drivers in a VM
Summary: A new microkernel hosts Linux only for drivers.
Consequences: A clean core; driver VM overhead and a second kernel to maintain.
Evidence: none

### Option C · Linux-as-hypervisor with the native model in a guest
Summary: The native model runs as a guest on Linux.
Consequences: Strong isolation; two kernels on every boot and hardware behind a hypervisor.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
