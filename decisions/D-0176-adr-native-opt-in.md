# D-0176 · Decide how native applications opt into a Personality
- Status: proposed
- Task: LNX-016
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §3, §46
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
How a native application explicitly opts into a Personality and what authority that grant carries must be decided (§3, §46), answering Q-002 and shaping S-030.

## Options

### Option A · Capability to the Personality
Summary: A grant to the Personality service.
Consequences: Explicit and revocable; coarse authority.
Evidence: none

### Option B · Embedded Linux Component
Summary: An embedded Linux Component inside the app graph.
Consequences: Isolated per app; heavy.
Evidence: none

### Option C · SDK shim
Summary: An SDK shim translates.
Consequences: Convenient; leaks POSIX shapes into native code.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
