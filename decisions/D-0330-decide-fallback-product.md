# D-0330 · Decide the fallback virtualization product and 1.0 scope
- Status: proposed
- Task: VIRT-002
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §5.1, §56.2, §69
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Personalities never cover every title, and kernel-level anti-cheat stays a non-goal (I-071, R-036); the maintainer's direction on D-0335 routes such titles to a VM. This decision is VIRT's baseline-gap scope (§5.1, §56.2, §69): whether JakeOS ships a host VM product, what 1.0 includes (manager, guest tools, triage offer, guest GPU, disk snapshots, physical-partition attach), and how it sits beside `Capability<VirtualMachine>` owned by KRN-025. WIN-001 still forbids an obvious VM for ordinary software; consequences for I-081 and T-036 are recorded.

## Options

### Option A · Native KVM manager as a capability-scoped Component
Summary: A native VM manager Component over `Object<VirtualMachine>`: capability-scoped devices, guest tools for Linux and Windows guests, the "offer the VM" triage flow, virtio-gpu guest graphics, disk snapshots; physical-partition attach is LATER.
Consequences: The fallback is a first-class, inspectable product consistent with the security model, and the triage flow can hand a failing title to it. The largest VIRT effort, and every device passthrough is a Capability design; guest GPU acceleration beyond virtio-gpu waits for VFIO work.
Evidence: none

### Option B · libvirt or virt-manager as a Linux-personality application
Summary: libvirt and virt-manager run as Linux-personality applications over the personality's `/dev/kvm`.
Consequences: Everything exists today. VMs are personality processes with personality authority, not Capabilities, so `os inspect` and the grant model do not see them, and the product is visibly a Linux tool inside JakeOS.
Evidence: none

### Option C · qemu launched with Capability-wrapped descriptors
Summary: A thin native launcher starts qemu with Capability-wrapped descriptors for disks, network and display; qemu itself is personality-hosted.
Consequences: Pragmatic reuse of qemu with native authority at the edges. qemu's device model is configured by command line rather than typed objects, the wrapper is on every VM start, and guest tools and triage still need building.
Evidence: none

### Option D · No host VM product
Summary: No host VM product before 1.0.
Consequences: Focus. The anti-cheat and 32-bit fallbacks the owner's directions rely on cease to exist, so those titles are simply unsupported; rejected.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
