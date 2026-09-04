# D-0313 · Decide how Personality threads map onto native Tasks
- Status: proposed
- Task: TSK-043
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §3, §20, §46, §48
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Linux and Windows personality threads must map onto native Tasks before V1 daily-driving (§3, §20, §46, §48).

## Options

### Option A · One native Task per personality thread
Summary: A 1:1 mapping.
Consequences: Simple; Task count equals thread count.
Evidence: none

### Option B · M:N personality threads onto native Tasks
Summary: M:N multiplexing.
Consequences: Efficient; complexity.
Evidence: none

### Option C · Personality threads as execution contexts wrapping native Tasks
Summary: Threads as contexts.
Consequences: Flexible; indirection.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
