# D-0204 · Decide whether NetworkConnection wraps the kernel TCP/IP stack
- Status: proposed
- Task: NET-008
- Surfaces: none
- Layer: none
- Spikes: NET-002
- Supersedes: none
- Superseded by: none
- Baseline: §4, §7
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
INV-0110 places a network Capability broker in userspace while the TCP stack stays in the kernel; wrap, byte-broker or hybrid is chosen from the spike (§4, §7), with the surface prototyped until V4.

## Options

### Option A · Wrap retained TCP/IP directly
Summary: NetworkConnection wraps kernel sockets.
Consequences: Fast data path; Capability checks close to the kernel.
Evidence: none

### Option B · Pass every byte through a userspace service
Summary: A userspace broker relays bytes.
Consequences: Policy in userspace; copy cost on every byte.
Evidence: none

### Option C · Hybrid kernel data path with userspace policy
Summary: Data in kernel, policy in userspace.
Consequences: Balance; two layers to keep consistent.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
