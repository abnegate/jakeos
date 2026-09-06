# D-0314 · Decide whether every Task has kernel-visible identity
- Status: proposed
- Task: TSK-008
- Surfaces: S-008
- Layer: L1
- Spikes: TSK-016
- Supersedes: none
- Superseded by: none
- Baseline: §20, §38, §65
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
With Task and TaskGroup replacing threads (§20), whether every Task is a kernel object decides what `os inspect` can name, how cancellation reaches a Task and what a Task costs against the B-014 live-Task scale. TSK-016 measures multiplexing models and hidden blocking. The Task ABI must not embed x86 execution-context assumptions (§38, I-057). S-008 stays prototyped.

## Options

### Option A · Every Task as Object<Task>
Summary: Every Task is a kernel object with a handle; cancellation, deadlines and inspection are kernel Operations on `Object<Task>`.
Consequences: Complete observability and cancellation from outside the Component, and scheduler intent applies per Task. A kernel allocation and handle per Task caps the live-Task scale, makes short-lived Tasks expensive and turns TaskGroup fan-out into repeated kernel work.
Evidence: `reports/spikes/TSK-016.md`

### Option B · Runtime identity with kernel visibility for observability and cancellation only
Summary: Tasks are runtime structures; the runtime publishes a compact identity table that the kernel reads for tracing plus a cancellation doorbell per Component.
Consequences: Tasks cost what the runtime chooses, so millions can exist. The kernel cannot schedule them individually, so intent applies to the worker rather than the Task. Inspection and cancellation depend on runtime cooperation, and a wedged runtime hides its Tasks from `os inspect`.
Evidence: `reports/spikes/TSK-016.md`

### Option C · Hybrid
Summary: Tasks are runtime identities by default and are promoted to `Object<Task>` when they need kernel-visible cancellation, a distinct intent or a hardware-committed Operation.
Consequences: Pays the kernel cost only where it buys something. Two identity kinds appear in `os inspect`, the promotion rule is ABI on S-008, and every tool handles both.
Evidence: `reports/spikes/TSK-016.md`

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
