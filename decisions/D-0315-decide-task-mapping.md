# D-0315 · Decide Task mapping onto kernel execution contexts
- Status: proposed
- Task: TSK-009
- Surfaces: none
- Layer: none
- Spikes: TSK-016
- Supersedes: none
- Superseded by: none
- Baseline: §2, §20, §21, §65
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Replacing the thread model with Task and TaskGroup and fixing the kernel/runtime split is a required V0 decision (§2, §20, §21, §65); S-008 stays prototyped.

## Options

### Option A · Kernel-managed Tasks
Summary: The kernel schedules every Task.
Consequences: Simple; scale limits.
Evidence: none

### Option B · UMCG-style activations with compensating workers
Summary: User-managed concurrency with compensation.
Consequences: Efficient; complexity.
Evidence: none

### Option C · Pure userspace runtime over async syscalls only
Summary: A userspace runtime.
Consequences: Cheap; blocking stalls workers.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
