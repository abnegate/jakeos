# D-0312 · Decide Operation Ownership transfer semantics across Tasks and TaskGroups
- Status: proposed
- Task: TSK-028
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §19, §21, §32
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Every Operation has an owner; transfer across Tasks and TaskGroups must define completion delivery, cancellation and accounting (§19, §21, §32).

## Options

### Option A · Move completion delivery to the new owner
Summary: Delivery follows ownership.
Consequences: Clean model; timing races at transfer.
Evidence: none

### Option B · Cancel the Operation on transfer
Summary: Transfer cancels the Operation.
Consequences: Simple; loses in-flight work.
Evidence: none

### Option C · Dual delivery until the new owner accepts
Summary: Both receive until accepted.
Consequences: Safe; complexity.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
