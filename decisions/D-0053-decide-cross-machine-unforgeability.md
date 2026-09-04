# D-0053 · Decide how Capability unforgeability survives machine boundaries
- Status: proposed
- Task: CAP-047
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §7, §43, §57
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The §43 question of how Capability unforgeability survives machine boundaries must be answered so the ABI does not foreclose later remote transports (§7, §43, §57), while distribution stays outside 1.0 and outside the kernel.

## Options

### Option A · Cryptographic Capabilities
Summary: Capabilities are signed or MAC-protected tokens valid off-machine.
Consequences: Portable authority; revocation and attenuation need online checks and keys become the trust root.
Evidence: none

### Option B · Sturdy references
Summary: Remote parties hold sturdy references redeemed against the owning machine.
Consequences: Owning machine keeps control; every use is a round trip to the owner.
Evidence: none

### Option C · Proxy objects on the sending machine
Summary: A proxy on the sender mediates every remote use.
Consequences: Attenuation and revoke stay local and exact; the proxy is a userspace service on the hot path.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
