# D-0141 · Decide the Interface-evolution rules for Layer 2 Interfaces (prototyped state)
- Status: proposed
- Task: IPC-002
- Surfaces: none
- Layer: none
- Spikes: IPC-019
- Supersedes: none
- Superseded by: none
- Baseline: §12, §66
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Every Interface carries an explicit evolution strategy (§12, §66), so how Interfaces version, add fields, add optional methods and negotiate is recorded from the versioning spike, prototyped in V0.

## Options

### Option A · Schema-indexed optional fields with generated negotiation
Summary: Fields are indexed by schema and stubs negotiate presence.
Consequences: Compact and generated; schema registry discipline required.
Evidence: none

### Option B · Self-describing envelopes with unknown-field preservation
Summary: Messages carry their own field descriptions.
Consequences: Robust to skew; larger messages and slower decode.
Evidence: none

### Option C · Explicit major/minor with dual-stack during overlap
Summary: Interfaces version explicitly and both versions run during overlap.
Consequences: Clear compatibility statements; dual implementations during transitions.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
