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
Q-048 asks whether virtualization is Capability<VirtualMachine> and whether personalities may fall back to VMs (§36, §69).

## Options

### Option A · Capability<VirtualMachine> as a kernel object
Summary: KVM is exposed as a native kernel object.
Consequences: Native, inspectable VMs; a kernel object to design.
Evidence: none

### Option B · KVM retained only for the Linux personality
Summary: VMs exist only through the personality.
Consequences: No native surface; weaker integration.
Evidence: none

### Option C · No host VM product
Summary: No virtualization product.
Consequences: Simplicity; no fallback for unsupported software.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
