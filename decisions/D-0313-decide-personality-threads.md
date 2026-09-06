# D-0313 · Decide how Personality threads map onto native Tasks
- Status: proposed
- Task: TSK-043
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §3, §20, §46, §48
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Linux and Windows personality processes have threads; native software has Tasks and no thread ABI (§3, §20, I-025). Before V1 daily driving through the Linux personality and the non-gated Wine bring-up, this decision fixes how personality threads map onto native Tasks and execution contexts (§46, §48), what cancellation, inspect identity and ResourceDomain charging look like for a personality thread, and answers Q-010. It sits on the Task mapping (D-0315), TaskGroup semantics (TSK-019) and the process-to-Component mapping (CMP-036).

## Options

### Option A · One native Task per personality thread
Summary: Each personality thread is one native Task with kernel identity; clone() creates a Task in the process's TaskGroup.
Consequences: Simple, exact accounting and inspection, and blocking syscalls block only their Task. Thread-heavy Linux programs (browsers, JVMs, games) create hundreds of Tasks, so the per-Task cost decided by D-0314 and D-0315 bounds their performance directly.
Evidence: none

### Option B · M:N personality threads onto native Tasks
Summary: Personality threads are multiplexed M:N onto a pool of native Tasks by the personality runtime.
Consequences: Cheap threads regardless of the native Task cost. Blocking in one personality thread must be detected and compensated exactly as §21 describes for native Tasks, thread-local storage and futex semantics must be emulated over the pool, and inspect shows pool Tasks rather than program threads.
Evidence: none

### Option C · Personality threads as execution contexts wrapping native Tasks
Summary: A personality thread is an execution context object owned by the personality that wraps a native Task and carries the Linux or Windows thread state (TLS base, signal mask, thread id).
Consequences: Personality state lives beside the Task rather than inside it, so the native Task ABI stays thread-free and inspect can show both views. An indirection on every syscall entry to find the context, and two identities per thread to keep in step.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
