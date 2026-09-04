# D-0153 · Decide how Capabilities and handles cross a VM transport boundary
- Status: proposed
- Task: IPC-056
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §43, §8
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Native guest Components talking to host services need proxied Capability semantics that honour attenuation and revocation (§43, §8), before the VM transport prototype.

## Options

### Option A · Proxied handles with host-side attenuation and revocation
Summary: The host proxies every guest handle.
Consequences: Exact attenuation and revoke; proxy state on the host for every handle.
Evidence: none

### Option B · Cryptographic Capabilities valid across the VM boundary
Summary: Tokens cross the boundary.
Consequences: No proxy state; revocation is weak and keys become trust roots.
Evidence: none

### Option C · No Capability crossing
Summary: Only MemoryObject and data cross.
Consequences: Simplest; limited guest integration.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
