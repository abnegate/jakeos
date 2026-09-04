# D-0152 · Decide the pluggable transport abstraction behind generated stubs
- Status: proposed
- Task: IPC-025
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §43, §57
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Generated stubs must bind to same-Component, same-machine and later VM transports without regeneration (§43, §57), with remote transports LATER (I-047).

## Options

### Option A · Pluggable transport trait behind generated stubs
Summary: Stubs call a transport trait selected at runtime.
Consequences: Transports swap without regeneration; an indirection on every call.
Evidence: none

### Option B · Compile-time transport selection per Interface
Summary: The transport is chosen when the stub is generated.
Consequences: No indirection; regeneration per transport.
Evidence: none

### Option C · Single same-machine transport with later forks
Summary: One transport now and forks later.
Consequences: Simplest today; forking later duplicates stubs.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
