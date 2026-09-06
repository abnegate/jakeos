# D-0311 · Decide Operation submission/completion transport and batching expression
- Status: proposed
- Task: TSK-007
- Surfaces: S-005
- Layer: L1
- Spikes: TSK-014
- Supersedes: none
- Superseded by: none
- Baseline: §18, §19, §65
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Operation completion is the hinge between kernel scheduling and the runtime (§18, §19, §65); transport and batching are chosen from the spike with S-005 prototyped.

## Options

### Option A · Shared rings
Summary: Shared submission and completion rings.
Consequences: Batching and few syscalls; ring layout is ABI.
Evidence: none

### Option B · Per-Component queues
Summary: Queues per Component.
Consequences: Isolation; per-queue overhead.
Evidence: none

### Option C · Syscall-per-Operation
Summary: One syscall each.
Consequences: Simple; no batching.
Evidence: none

### Option D · Hybrid
Summary: Rings with a syscall fallback.
Consequences: Balance; complexity.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
