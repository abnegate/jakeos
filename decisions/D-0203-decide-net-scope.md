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
BASELINE.md has no networking section, so the first NET Decision records the scope: preserve Linux TCP/IP with native NetworkConnection, rejecting a rewritten stack and POSIX sockets as native API (§57).

## Options

### Option A · Preserve Linux TCP/IP, nftables and inherited drivers with native NetworkConnection
Summary: The Linux stack stays and native objects wrap it.
Consequences: Mature stack; wrapping work.
Evidence: none

### Option B · Rewrite a userspace TCP stack
Summary: A new stack in userspace.
Consequences: Isolation; violates I-009 and I-010.
Evidence: none

### Option C · Expose Linux sockets as the native API
Summary: Sockets are the native API.
Consequences: Easy; rejected by I-005 and I-049.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
