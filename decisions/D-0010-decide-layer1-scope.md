# D-0010 · Decide Layer 1 scope: enumerate L1 primitives and place every concept in L1 or L2
- Status: proposed
- Task: ABI-011
- Surfaces: none
- Layer: none
- Spikes: ABI-022
- Supersedes: none
- Superseded by: none
- Baseline: §66, §65
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Every public concept must be placed in Layer 1 or Layer 2 and Q-047 answered for compositor protocol, Package format and ResourceDomain policy (§65, §66) before any Layer 1 surface can be a freeze candidate.

## Options

### Option A · Minimal Layer 1
Summary: Layer 1 is handles, entry, errors, negotiation and object type ids; everything else is Layer 2.
Consequences: Smallest frozen surface and the most freedom to evolve; more concepts depend on Layer 2 versioning discipline.
Evidence: none

### Option B · Layer 1 including Channel and ResourceDomain
Summary: Channel and ResourceDomain are kernel objects in Layer 1 alongside the minimal set.
Consequences: IPC and accounting shapes are stable for the ecosystem; both surfaces must reach freeze-quality evidence by V4.
Evidence: none

### Option C · Layer 1 including compositor protocol and Package format
Summary: Compositor protocol and Package format are also Layer 1.
Consequences: Applications get the strongest guarantees; UI and packaging cannot evolve without a major OS version.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
