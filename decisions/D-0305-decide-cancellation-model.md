# D-0305 · Decide Task cancellation model and resource cleanup
- Status: proposed
- Task: TSK-003
- Surfaces: none
- Layer: none
- Spikes: TSK-017
- Supersedes: none
- Superseded by: none
- Baseline: §19, §21, §58
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
V0 exit cancels a TaskGroup and never delivers a result from a cancelled Operation (§19, §21, §58); the model and cleanup must be fixed.

## Options

### Option A · Cooperative cancellation at await points
Summary: Tasks observe cancellation at await points and clean up themselves.
Consequences: Clean cleanup; uncooperative Tasks linger.
Evidence: none

### Option B · Forced kernel teardown
Summary: The kernel tears the Task down.
Consequences: Guaranteed termination; leaked resources and partial state.
Evidence: none

### Option C · Staged cancellation with a grace deadline
Summary: Cooperative first, forced after a deadline.
Consequences: Balance; two paths to specify.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
