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
§9.1 says a native application must not automatically receive network access but never defines a network Capability, so granularity and default-deny inbound must be fixed; S-026 stays prototyped.

## Options

### Option A · Binary any/none
Summary: Network access is all or nothing.
Consequences: Simple to grant; coarse and over-privileging.
Evidence: none

### Option B · GAP-0293 set
Summary: Rights for any, internet-only, local-network, specific hosts/ports and listen.
Consequences: Useful granularity; more rights to register and explain.
Evidence: none

### Option C · Flow-level per connection
Summary: Each connection is granted.
Consequences: Precise; unusable prompts.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
