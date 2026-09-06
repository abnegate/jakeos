# D-0154 · Decide the typed-message wire format and inline-payload threshold
- Status: proposed
- Task: IPC-007
- Surfaces: S-013
- Layer: L2
- Spikes: IPC-020, IPC-018
- Supersedes: none
- Superseded by: none
- Baseline: §14, §15
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The typed-message wire format and inline-versus-MemoryObject threshold are chosen from the wire-encoding spike (§14, §15), answering Q-005 and naming S-013.

## Options

### Option A · Fixed layout
Summary: Messages are fixed-layout structs.
Consequences: Zero-copy and fastest validation; rigid evolution.
Evidence: none

### Option B · Self-describing
Summary: Messages carry their own field descriptions.
Consequences: Flexible across versions; decode cost and larger messages.
Evidence: none

### Option C · Schema-indexed
Summary: Fields are indexed by a schema id.
Consequences: Compact with evolution; a schema registry is needed.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
