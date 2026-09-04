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
Personalities never cover every title, so whether JakeOS ships a host VM product and its 1.0 scope must be decided (§5.1, §56.2, §69).

## Options

### Option A · Native KVM manager as a capability-scoped Component
Summary: A native manager.
Consequences: Integrated; effort.
Evidence: none

### Option B · libvirt or virt-manager as a Linux-personality application
Summary: libvirt in the personality.
Consequences: Reuse; personality-hosted.
Evidence: none

### Option C · qemu launched with Capability-wrapped descriptors
Summary: Wrapped qemu.
Consequences: Pragmatic; wrapping.
Evidence: none

### Option D · No host VM product
Summary: None.
Consequences: Focus; no fallback.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
