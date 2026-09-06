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
Every Operation has an owner Task or TaskGroup that receives its completion and may cancel it (§19, §21). Service restart under SVC re-owns in-flight work during rebind (§32), so ownership transfer must define where completion is delivered after transfer, who may cancel, which ResourceDomain is charged, and the typed error a caller observes when transfer is refused. This decision precedes the supervisor's rebind path and the TaskGroup cancellation semantics of TSK-003.

## Options

### Option A · Move completion delivery to the new owner
Summary: Transfer atomically re-points completion delivery and cancellation authority to the new owner; the completion, if it has already fired, is re-queued to the new owner's transport.
Consequences: One owner at every instant, so accounting and cancellation are unambiguous and the supervisor can adopt in-flight work on rebind. The transfer races with completion, so the kernel must define the instant of handover and re-queue already-fired completions; a completion delivered to the old owner's now-dead ring is lost unless re-queued.
Evidence: none

### Option B · Cancel the Operation on transfer
Summary: Transfer is not supported; an Operation whose owner dies or hands off is cancelled and the new owner resubmits.
Consequences: No handover semantics to specify and no race. In-flight work is lost at every service restart, which makes rebind observable to clients (a write in progress fails), and idempotency becomes every service's problem.
Evidence: none

### Option C · Dual delivery until the new owner accepts
Summary: Both old and new owner receive completions until the new owner explicitly accepts, after which the old owner is detached.
Consequences: No completion is ever lost during the handover window. Two parties may cancel and two are charged during the window, `os inspect` shows an Operation with two owners, and the accept step is a protocol every service must implement correctly.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
