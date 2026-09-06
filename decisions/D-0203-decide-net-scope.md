# D-0203 · Decide NET baseline-gap scope: preserved stack versus native objects
- Status: proposed
- Task: NET-007
- Surfaces: none
- Layer: none
- Spikes: NET-002
- Supersedes: none
- Superseded by: none
- Baseline: none
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
BASELINE.md has no networking section, so this first NET decision records the workstream's scope before any V1 NET build task starts: what is preserved from Linux, what the native object is, and what is rejected. §57 forbids POSIX sockets as the native API (I-005, I-049) and rewriting a mature subsystem without a recorded benefit (I-009, I-010). NET-002 studies the stack; the V0 task cap keeps this decision at V1.

## Options

### Option A · Preserve Linux TCP/IP, nftables and inherited drivers with native NetworkConnection
Summary: Linux TCP/IP, nftables and the inherited network drivers are the mechanism; `Object<NetworkConnection>` is the native handle over it; connection management (Wi-Fi, DHCP, DNS, VPN) is an SVC-hosted Component, not kernel code.
Consequences: Decades of stack maturity and driver coverage are kept, the native surface is one typed object, and management is supervised and inspectable like any service. Every native networking feature is a wrapper design, and the personality's raw socket access must be reconciled with the native rights model.
Evidence: `reports/spikes/NET-002.md`

### Option B · Rewrite a userspace TCP stack
Summary: A user-space TCP/IP stack in Rust owns networking; the kernel provides raw packet access.
Consequences: Isolation of the stack and a clean native design. Rewrites a mature subsystem with no measured benefit (I-009, I-010), loses offloads and driver features, and takes years to reach parity; rejected.
Evidence: `reports/spikes/NET-002.md`

### Option C · Expose Linux sockets as the native API
Summary: Linux sockets are the native networking API.
Consequences: Every existing library works. It makes POSIX the native ABI for networking, which §57, I-005 and I-049 forbid; recorded as rejected, with sockets confined to the personalities.
Evidence: `reports/spikes/NET-002.md`

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
