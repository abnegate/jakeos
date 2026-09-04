# D-0056 · Decide the Capability<T> and MemoryObject mapping onto hardware capabilities
- Status: proposed
- Task: CAP-033
- Surfaces: none
- Layer: none
- Spikes: CAP-039, CAP-040
- Supersedes: none
- Superseded by: none
- Baseline: §8, §38
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
CHERI/Morello and tagged-memory findings must be recorded as the mapping the ABI stays compatible with while application-visible Capabilities remain conceptually stable (§8, §38).

## Options

### Option A · 1:1 map onto a CHERI capability
Summary: Capability<T> is a CHERI capability.
Consequences: Cleanest hardware enforcement; ties the ABI to one hardware model.
Evidence: none

### Option B · Split: MemoryObject on tagged memory, Capability as a table handle
Summary: Memory uses tags while object handles stay table entries.
Consequences: Uses each hardware feature where it fits; two enforcement models to reason about.
Evidence: none

### Option C · Document-only mapping with no kernel backend
Summary: The mapping is written down and no backend exists until hardware is in the lab.
Consequences: No premature engineering; compatibility claims are untested.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
