# D-0159 · Decide how KVM is exposed natively as Capability<VirtualMachine>
- Status: proposed
- Task: KRN-025
- Surfaces: none
- Layer: none
- Spikes: KRN-017
- Supersedes: none
- Superseded by: none
- Baseline: §36, §69
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
KVM is retained mechanism (§36). Q-048 asks whether virtualisation is exposed natively as `Capability<VirtualMachine>`, whether compatibility personalities may fall back to a VM for software they cannot run, and how host resources reach a guest. The VIRT VM manager and the kernel object task depend on this; each VM remains a capability-scoped Component whose host access is visible in `os inspect` (§69). KRN-017's inventory records what KVM retains.

## Options

### Option A · Capability<VirtualMachine> as a kernel object
Summary: `Object<VirtualMachine>` is a native kernel object: creating one takes a Capability, the VM runs as a Component, and its devices are Capabilities to host objects.
Consequences: VMs are inspectable, budgeted by ResourceDomain and grantable like everything else, and the VIRT fallback for unsupported software is a first-class product. The kernel gains an object that wraps KVM's ioctl model, which is a Layer 1 surface to design and eventually freeze, and vCPU scheduling meets SCH intent.
Evidence: `reports/spikes/KRN-017.md`

### Option B · KVM retained only for the Linux personality
Summary: KVM is reachable only through the Linux personality's `/dev/kvm`; a VM manager is a personality application.
Consequences: No native surface to design and existing Linux VM tooling runs unchanged. Host resources reach the VM by personality authority rather than Capabilities, `os inspect` sees a Linux process, and a native fallback product cannot exist without the personality.
Evidence: `reports/spikes/KRN-017.md`

### Option C · No host VM product
Summary: No host virtualisation product before 1.0; KVM is kept in the kernel only for CI.
Consequences: The simplest scope. The VIRT fallback the anti-cheat and 32-bit policies rely on does not exist, so the titles those decisions route to a VM are simply unsupported.
Evidence: `reports/spikes/KRN-017.md`

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
