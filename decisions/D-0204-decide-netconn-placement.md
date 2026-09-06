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
INV-0110 places the network Capability broker in user space while the TCP stack stays in the kernel (§4, §7). NET-002 measures three placements of `Object<NetworkConnection>`: a thin wrapper over kernel sockets, a user-space service that relays every byte, and a hybrid. The choice fixes where Capability checks run relative to the stack and shapes the firewall and VPN designs; the surface stays prototyped until V4 (I-040). ABI-013 supplies the handle model.

## Options

### Option A · Wrap retained TCP/IP directly
Summary: `NetworkConnection` is a kernel object that wraps a retained socket; rights are checked in the kernel at connect, listen and send.
Consequences: The data path is the Linux one with no extra copies, and checks happen where they cannot be bypassed. Rights semantics (hosts and ports, local-network) must be evaluated in kernel code, and policy changes mean kernel changes.
Evidence: `reports/spikes/NET-002.md`

### Option B · Pass every byte through a userspace service
Summary: A user-space network service owns all sockets; Components send and receive bytes to it over Channels.
Consequences: All policy, DNS, pinning and VPN routing live in one supervised Component the kernel knows nothing about. Every byte is copied and crosses a Component boundary, which the throughput benchmark will show, and the service is a single point of failure for all networking.
Evidence: `reports/spikes/NET-002.md`

### Option C · Hybrid kernel data path with userspace policy
Summary: The kernel object carries the data path and enforces the coarse right (any, none, listen); a user-space policy Component decides fine-grained rights at connection setup and installs the result as nftables rules and per-connection marks.
Consequences: Line-rate data with policy in user space, and nftables stays the enforcement mechanism (§2). Two layers must agree on what a right means, setup latency includes a policy round trip, and a policy Component crash must fail closed.
Evidence: `reports/spikes/NET-002.md`

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
