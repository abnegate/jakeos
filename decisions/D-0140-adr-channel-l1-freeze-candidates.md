# D-0140 · Decide which Channel syscalls become Layer 1 freeze candidates for SDK v1
- Status: proposed
- Task: IPC-041
- Surfaces: none
- Layer: none
- Spikes: IPC-017
- Supersedes: none
- Superseded by: none
- Baseline: §65, §66
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
IPC must name which Channel syscalls become Layer 1 freeze candidates for SDK v1 and what stays behind user-space Interfaces (§65, §66); nothing is frozen here (I-040).

## Options

### Option A · Create, send, receive, close, handle-transfer and inspect as candidates
Summary: The full Channel entry set is a freeze candidate.
Consequences: Complete contract for SDK v1; every entry needs spike and benchmark evidence.
Evidence: none

### Option B · Reduced send/receive/close core with handle-transfer at Layer 2
Summary: Only the core is a candidate.
Consequences: Smaller freeze; handle transfer evolves separately.
Evidence: none

### Option C · Defer candidacy to V2
Summary: No Channel candidates at V1.
Consequences: More evidence time; SDK v1 lacks a stable IPC contract.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
