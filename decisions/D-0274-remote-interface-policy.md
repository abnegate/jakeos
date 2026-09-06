# D-0274 · Decide remote-Interface Capability, identity, and encryption rules
- Status: proposed
- Task: SEC-079
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §43, §57
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Distribution is parked at LATER (§43, §57): the kernel is never a distributed system (I-047). When remote transports for Interfaces exist, they must honour Capabilities across machines (CAP-047, Q-036), endpoint identity, encryption and explicit user policy (T-019). This decision fixes those rules in advance so any post-1.0 transport is designed against them.

## Options

### Option A · Capability-honouring encrypted transport with explicit user policy in userspace
Summary: A user-space transport Component: an encrypted, mutually authenticated connection between two identity-bearing endpoints, over which Capabilities are proxied with attenuated rights the user granted for that peer; the kernel sees only local Channels.
Consequences: The model extends across machines without the kernel learning about networks, and every remote right is a visible local grant. Capability unforgeability across machines (CAP-047) must be solved by the proxy's identity binding, and latency semantics differ from local Channels.
Evidence: none

### Option B · Keep remote out until a later major version
Summary: No remote Interfaces until a later major version; remote access is personality-hosted only.
Consequences: Nothing to secure now. The native model has no distributed story at all, which limits the platform to single machines indefinitely.
Evidence: none

### Option C · Kernel-mediated remote
Summary: The kernel mediates remote Channels directly.
Consequences: Lowest latency in principle. It makes distribution a kernel concern, which §57 and I-047 forbid; recorded as rejected.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
