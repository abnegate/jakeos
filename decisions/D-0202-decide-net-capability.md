# D-0202 · Decide per-application network Capability granularity and inbound firewall
- Status: proposed
- Task: NET-006
- Surfaces: S-026
- Layer: L2
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §9.1
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
A native application receives no network access by default (§9.1) but the baseline never defines a network Capability. This decision fixes the granularity of `NetworkConnection` rights on S-026 (prototyped until V4), records that default-deny inbound is retained nftables rather than a rewritten filter, that listen is an explicit right never granted ambiently (T-001, I-021), and names the rights word CAP-036 registers. It sits on the NET scope (NET-007) and placement (NET-008) decisions.

## Options

### Option A · Binary any/none
Summary: One right: network or no network.
Consequences: Trivial to grant and explain. Every application that needs to reach one service gets the whole internet and the local network, and there is no way to express "may listen", so inbound is either open or impossible; over-privileging by construction.
Evidence: none

### Option B · GAP-0293 set
Summary: Rights for any, internet-only, local-network, named hosts and ports, and listen, composable and attenuable like other rights.
Consequences: A store client gets internet-only, a printer tool gets local-network, a development server gets listen on one port, and the grant taxonomy (D-0269) can class them. More rights to register, a hosts-and-ports right needs a resolver that binds names to the right, and `os inspect` must render the set legibly.
Evidence: none

### Option C · Flow-level per connection
Summary: Every connection is individually granted.
Consequences: Perfect precision. A browser opens hundreds of connections per page, so the prompt rate makes the model unusable; rejected for interactive use and recorded so it is not re-proposed except as an audit mode.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
