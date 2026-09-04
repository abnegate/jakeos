# D-0249 · Decide behaviour on ResourceDomain budget exhaustion and owner reporting
- Status: proposed
- Task: SCH-016
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §23, §32
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
What happens when a ResourceDomain exhausts a budget must be typed and observable per budget kind (§23, §32, T-016).

## Options

### Option A · Fail-closed typed Operation errors with an owner event
Summary: Operations fail with a typed error and the owner is notified.
Consequences: Observable and recoverable; applications must handle the error.
Evidence: none

### Option B · Component termination after reclaim
Summary: The Component is terminated after reclaim fails.
Consequences: Simple; abrupt for the user.
Evidence: none

### Option C · Per-kind mix
Summary: Memory, CPU and object limits each pick a behaviour.
Consequences: Fits each kind; more to specify.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
