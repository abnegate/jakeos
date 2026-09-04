# D-0139 · Decide whether the kernel offers synchronous call with time-slice donation beside async send
- Status: proposed
- Task: IPC-001
- Surfaces: none
- Layer: none
- Spikes: IPC-017, IPC-018
- Supersedes: none
- Superseded by: none
- Baseline: §15, §18, §53
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Synchronous call with time-slice donation changes scheduler and Native ABI shape irreversibly (§15, §18, §53), so it is reviewed with SCH and TSK against the fast-path spike before Channel send is fixed.

## Options

### Option A · Async send and receive only
Summary: The kernel offers only asynchronous send and receive.
Consequences: One ABI shape and no donation logic; request-reply latency depends on the scheduler.
Evidence: none

### Option B · Async send plus synchronous call with time-slice donation
Summary: A call entry donates the caller's slice to the callee.
Consequences: Low same-core round trip; a blocking-shaped entry and scheduler coupling to freeze.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
