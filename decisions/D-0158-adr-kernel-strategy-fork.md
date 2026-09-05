# D-0158 · Decide kernel strategy: Linux fork vs new microkernel vs Linux-as-hypervisor
- Status: accepted
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
Option A. JakeOS is a radical fork of the Linux kernel. Linux supplies boot, memory management, scheduling internals, networking, storage, drivers and KVM; the native model (Components, Tasks, Capabilities, Channels, Operations, MemoryObjects, ResourceDomains) is added in-tree behind the native ABI, first as wrappers over Linux internals (phase A to C) and later as native implementations (phase D to E) per §6.

## Consequences
- Every KRN, ABI, CMP, TSK, IPC, MEM and SCH task stands as written; the V0 ladder and its hardware scope are unchanged.
- The fork is a permanent maintenance obligation (§56.4): CVE intake, driver adaptation and upstream tracking are first-class workstream duties (KRN-005, KRN-006, KRN-007).
- The firewall of §3 becomes a validator and lint concern: no native ABI entry point may exist because Linux has an equivalent (ABI-003).

## Rejected options and why
- Option B (new microkernel, Linux drivers in a VM) rejected: it discards the mature mechanisms §2 tells us to preserve, adds a second kernel to maintain, and puts every device behind a VM boundary before the native model has proven itself.
- Option C (Linux as hypervisor) rejected: two schedulers and two memory managers on every boot make the cheap-isolation and zero-copy goals of §10 and §17 structurally unreachable.

## Follow-ups
KRN-003, KRN-005, KRN-006, KRN-007 (all accepted with this Decision).
