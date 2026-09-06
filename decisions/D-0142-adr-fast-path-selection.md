# D-0142 · Select the small-message fast-path technique from measured prototypes
- Status: proposed
- Task: IPC-003
- Surfaces: S-012
- Layer: L1
- Spikes: IPC-017
- Supersedes: none
- Superseded by: none
- Baseline: §15, §53
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
V0 exit requires an accepted fast-path Decision listing rejected options, chosen from IPC-017 before Channel semantics are fixed (§15, §53).

## Options

### Option A · Shared ring
Summary: Messages pass through a shared ring.
Consequences: Batching and few entries; ring layout is ABI.
Evidence: none

### Option B · CPU-register-carried messages
Summary: Small messages travel in registers.
Consequences: Minimal copies; tiny payloads only.
Evidence: none

### Option C · Scheduler-aware handoff
Summary: Send hands the CPU to the receiver.
Consequences: Low same-core latency; scheduler coupling.
Evidence: none

### Option D · Lock-free cross-core queues
Summary: Cross-core queues without locks.
Consequences: Cross-core throughput; same-core no better.
Evidence: none

### Option E · Recorded combination
Summary: A named mix of the above.
Consequences: Best of each; more to specify.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
