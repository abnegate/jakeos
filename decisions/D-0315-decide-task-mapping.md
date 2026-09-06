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
Replacing the thread model with Task and TaskGroup (§2, §20) requires fixing the kernel and runtime split and how unexpected kernel blocking (a page fault, a synchronous retained driver path) is compensated so a stalled worker does not starve multiplexed Tasks (§21). TSK-016 measures the models against the B-014 live-Task scale. S-008 stays prototyped.

## Options

### Option A · Kernel-managed Tasks
Summary: Every Task is a kernel-scheduled execution context; the runtime never multiplexes.
Consequences: Blocking anywhere stalls only that Task, the scheduler sees everything and intent is exact. Context-switch and memory cost per Task bound the scale, TaskGroup fan-out is kernel work, and B-014 is decided by kernel per-context cost.
Evidence: `reports/spikes/TSK-016.md`

### Option B · UMCG-style activations with compensating workers
Summary: A small set of kernel workers per Component; the runtime multiplexes Tasks on them and the kernel reports a blocked worker so the runtime activates another.
Consequences: User-space scheduler scale with the kernel telling it about hidden blocking, which is the case B-014 favours. The activation protocol is a new kernel interface with UMCG lineage, the runtime scheduler becomes correctness-critical, and two schedulers must agree on intent.
Evidence: `reports/spikes/TSK-016.md`

### Option C · Pure userspace runtime over async syscalls only
Summary: The runtime multiplexes Tasks on a fixed worker pool and all kernel work is asynchronous Operations, so nothing should block.
Consequences: The simplest kernel. Page faults and synchronous retained driver paths still block a worker with no signal, starving the Tasks on it, and compensation is heuristic (timeouts, spare workers). Fails §21 unless every blocking path is removed first.
Evidence: `reports/spikes/TSK-016.md`

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
