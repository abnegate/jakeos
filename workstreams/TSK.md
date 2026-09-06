# TSK · Tasks, operations, structured concurrency
- Prefix: TSK
- Lead: none
- Baseline: §18, §19, §20, §21

<!-- roadmap:generated:begin summary -->
Tasks: 53 live, 1 done, 0 in-progress, 52 todo, 0 dropped. Ready: 3. Blocked: 49. Weighted: 1%.
<!-- roadmap:generated:end -->

## Scope

Task, TaskGroup and Operation<Result> are the native concurrency and outstanding-work primitives. This workstream owns structured concurrency (ownership hierarchy, cancellation propagation, the background-execution Capability check), Task multiplexing over a bounded set of execution contexts, the Operation object (submit, completion, deadline, priority, tracing, ownership, resource accounting), Operation kinds listed in §18, and the io_uring-lineage investigation that informs the Layer 1 ring ABI (S-005) and TaskGroup ABI (S-008). Surfaces stay prototyped through V0, become freeze candidates at V1, and freeze at V4 with a conformance suite. Inventory prefix OPS is absorbed here.

## Out of scope

Handle encoding, kernel entry, error model and Layer 1 freeze process (ABI). Capability rights, derivation and the BackgroundExecution type definition (CAP). Component create, address space and panic (CMP). Channel object, IDL and backpressure (IPC). MemoryObject mapping and transfer (MEM). ResourceDomain budgets and scheduling intent classes (SCH). `os inspect` rendering and trace format (OBS). Userspace runtime, debugger and profiler CLI (SDK). Service supervision and native init (SVC). ComputeDevice semantics (HET). Storage durability contract (STO). NetworkConnection objects (NET). Linux and Windows personalities (LNX, WIN). Fuzz infrastructure (BLD). Benchmark methodology (BEN). Suspend mechanism (PWR). Threat-model document (SEC).

## Tasks

### TSK-001 · Add async-by-default ABI review Gate and blocking-syscall lint
- Type: build
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: TSK-006, TSK-018
- Baseline: §18, §57, §65, §67
- Invariants: I-018, I-030

Standing rules of §18 and Principle 5 (§67) are enforced once: native kernel APIs are asynchronous by default, language runtimes bind to kernel Operations, and signals are not a native notification mechanism. A CI lint and ABI review checklist reject native entry points whose primary mode blocks the calling execution context, a blocking read/write thread-per-call surface, signal-style notification, and runtime async layers that bypass kernel Operations.

<!-- covers: INV-0340, INV-0341, INV-0039, INV-1296, INV-0038 -->

#### Out of scope
Personality syscall retention (LNX). POSIX-shaped names on native crates (ABI-018).

#### Acceptance criteria
- [ ] CI fails a native crate that exposes an entry point whose primary mode blocks the calling execution context.
- [ ] CI fails a native crate that exposes a blocking read/write thread-per-call I/O surface.
- [ ] CI fails a native crate that uses signal-style notification or an async runtime that does not submit kernel Operations.
- [ ] The lint is wired into the V0 merge queue on matrix entry `qemu-x86_64`.

#### Verification
- Unit: `kernel:tests/tsk/async_lint_*` on CI matrix entry `qemu-x86_64`.
- Review: ABI lead sign-off recorded on the pull request that lands the checklist.

#### Evidence
- none

### TSK-002 · Benchmark native Task handoff against Linux thread switch and publish
- Type: benchmark
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: TSK-020, BEN-007, BEN-005
- Baseline: §20, §54, §59
- Benchmarks: B-003
- Risks: R-009

V0 measures context-switch behaviour as native Task handoff versus Linux thread switch on the same hardware and publishes the result. The harness is B-003 (`bench:task-switch`); the V0 target kind is publish. Numbers live only in the register and the committed report.

#### Out of scope
Task creation latency (B-002, BEN-001). IPC round trip (B-004, IPC-008). V1 multiplexer tuning (TSK-046).

#### Acceptance criteria
- [ ] A committed B-003 report exists for H-001 meeting the V0 publish target.
- [ ] A committed B-003 report exists for H-002 meeting the V0 publish target.
- [ ] Each report names the Linux thread-switch and process-switch baselines run in the same session.
- [ ] No public material states a superiority claim without citing those reports (I-061).

#### Verification
- Bench: B-003 on H-001 and H-002; target per register.
- Review: BEN lead confirms the reports follow the accepted methodology.

#### Evidence
- none

### TSK-003 · Decide Task cancellation model and resource cleanup
- Type: adr
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: TSK-017, Q-011, Q-012
- Baseline: §19, §21, §58
- Decision: D-0305
- Invariants: I-018

V0 exit cancels a TaskGroup and never delivers a result from a cancelled Operation (§19, §21, §59). This decision picks the cancellation model native software observes, the cleanup of Capabilities, MemoryObjects and partial ownership a cancelled Task holds, and the contract when a Task is inside an uninterruptible inherited Linux path. The decision studies Trio nurseries, Kotlin coroutine scopes and Swift task groups rather than a separate V0 research spike.

<!-- covers: INV-0396, INV-0394, INV-0395, INV-1149 -->

#### Out of scope
Committed-hardware state machine implementation (TSK-010). Background-execution Capability exception (TSK-025). Personality thread mapping (TSK-043).

#### Acceptance criteria
- [ ] Options evaluated include cooperative cancellation at await points, forced kernel teardown, and staged cancellation with a grace deadline.
- [ ] Each option states cleanup of Capabilities, MemoryObjects and partial ownership, and the observable result of a Task stuck in uninterruptible inherited Linux sleep.
- [ ] The decision cites the spike report for hardware-committed Operations and records at least one rejected signal-like option.
- [ ] ABI lead and TSK lead sign-off is recorded on the pull request.

#### Verification
- Review: ABI lead and TSK lead sign-off recorded on the pull request.
- Report: the decision file lists the Trio, Kotlin and Swift sources consulted and the rejected options.

#### Evidence
- none

### TSK-004 · Decide deadline and timestamp representation in the Operation ABI
- Type: adr
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: TSK-015
- Baseline: §18, §19, §65
- Decision: D-0306
- Risks: R-007

Every Operation carries a deadline, so clock domain, resolution, overflow horizon and the provisional suspend/resume rule are stamped into the Layer 1 Operation ABI (S-005) while the surface stays prototyped. This decision precedes Timer and Wait kinds and the cancel-and-deadline path. Suspend-cycle behaviour on a laptop is implemented later against PWR and the V1 clock-semantics adr.

<!-- covers: GAP-0496 -->

#### Out of scope
Timer kind implementation (TSK-012). Laptop suspend cycles (TSK-041, PWR). Slack and coalescing (TSK-047).

#### Acceptance criteria
- [ ] Options evaluated include a monotonic clock that does not advance during suspend, a boot-time clock that does, and a wall clock, each with a stated resolution and overflow horizon.
- [ ] Each option states what a Timer Operation observes across suspend and resume.
- [ ] The decision names the ABI fields that carry deadline and timestamp and records that S-005 stays prototyped.
- [ ] ABI lead sign-off is recorded on the pull request.

#### Verification
- Review: ABI lead sign-off recorded on the pull request.

#### Evidence
- none

### TSK-005 · Decide whether Operations may complete inline at submit and how the ABI signals it
- Type: adr
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: TSK-014
- Baseline: §18, §19, §65
- Decision: D-0307

Cached reads and already-signalled Waits can finish before submit returns. Whether that is allowed, and how the ABI tells the caller that completion was inline rather than delivered later, is a Layer 1 choice on S-005 and must be fixed before the submit path is built. The surface stays prototyped.

<!-- covers: INV-0345 -->

#### Out of scope
Submit and completion implementation (TSK-018). Transport choice (TSK-007).

#### Acceptance criteria
- [ ] Options evaluated include never completing inline, completing inline with an ABI-visible flag, and completing inline with a distinct submit return code.
- [ ] Each option states how a caller distinguishes inline completion from a later completion record, including the already-signalled Wait case.
- [ ] The decision records that S-005 stays prototyped.
- [ ] ABI lead sign-off is recorded on the pull request.

#### Verification
- Review: ABI lead sign-off recorded on the pull request.

#### Evidence
- none

### TSK-006 · Decide native expression of termination, cancellation and async notification without signals
- Type: adr
- Milestone: V0
- Status: done
- Size: S
- Owner: @agent/claude
- Depends on: none
- Baseline: §1, §18, §19, §21
- Decision: D-0309
- Invariants: I-018
- Verified by: @jakebarnby

Signals have no native equivalent (§1, §18). This decision names Operation completion, typed Channel messages and Wait-able objects as the notification model native software uses for termination, cancellation and asynchronous wake-ups, and records the rejected signal-like options so later lints can forbid them.

<!-- covers: INV-0075, INV-0039 -->

#### Out of scope
Event object implementation (TSK-029). Channel messages (IPC). Personality signal delivery (LNX).

#### Acceptance criteria
- [x] Options evaluated include Operation completion plus Wait-able objects, typed Channel messages as the sole wake-up, and a retained signal-like native event (recorded as rejected if chosen against).
- [x] The decision states how Task termination and Operation cancellation are observed without signals.
- [x] The decision lists the signal-like options it rejects and why.
- [x] ABI lead sign-off is recorded on the pull request.

#### Verification
- Review: ABI lead sign-off recorded on the pull request.

#### Evidence
- decision:D-0309

### TSK-007 · Decide Operation submission/completion transport and batching expression
- Type: adr
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: TSK-014
- Baseline: §18, §19, §65
- Decision: D-0311
- Risks: R-007

Operation completion is the hinge between kernel scheduling and the runtime and cannot change after SDK code depends on it. This decision picks shared submission and completion rings, per-Component queues, syscall-per-Operation, or a hybrid; whether io_uring internals are reused or replaced; and how batches are expressed. S-005 is prototyped, not frozen.

<!-- covers: INV-0344, GAP-0494 -->

#### Out of scope
Submit path implementation (TSK-018). Linked chains (TSK-030). Ring hardening (TSK-040).

#### Acceptance criteria
- [ ] Options evaluated include shared rings, per-Component queues, syscall-per-Operation, and a hybrid, each stating whether io_uring internals are reused or replaced.
- [ ] Each option states how a batch of submissions is expressed and how completion is delivered to the submitting Task.
- [ ] The decision cites the spike report's wake-up measurements via B-009 and records that S-005 stays prototyped.
- [ ] ABI lead sign-off is recorded on the pull request.

#### Verification
- Review: ABI lead sign-off recorded on the pull request.

#### Evidence
- none

### TSK-008 · Decide whether every Task has kernel-visible identity
- Type: adr
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: TSK-016
- Baseline: §20, §38, §65
- Decision: D-0314
- Invariants: I-017, I-057

Orthogonal to the multiplexing model: every Task may be `Object<Task>`, a runtime identity with kernel visibility only for observability and cancellation, or a hybrid. The Task ABI must not embed x86-specific execution-context assumptions (§38, I-057). S-008 stays prototyped.

<!-- covers: INV-0381, INV-0721 -->

#### Out of scope
Multiplexing model (TSK-009). Task implementation (TSK-021). Inspect rendering (OBS-005).

#### Acceptance criteria
- [ ] Options evaluated include every Task as `Object<Task>`, runtime identity with kernel visibility only for observability and cancellation, and a hybrid.
- [ ] Each option states how cancellation and `os inspect` name a Task, and records that the ABI definition is architecture-neutral.
- [ ] The decision records that S-008 stays prototyped.
- [ ] ABI lead sign-off is recorded on the pull request.

#### Verification
- Review: ABI lead sign-off recorded on the pull request.

#### Evidence
- none

### TSK-009 · Decide Task mapping onto kernel execution contexts
- Type: adr
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: TSK-016
- Baseline: §2, §20, §21, §65
- Decision: D-0315
- Risks: R-007
- Invariants: I-017

Required V0 architectural decision: replace the thread model with Task and TaskGroup (§2, §20) and fix the kernel/runtime split. Options are complete models that each state how unexpected kernel blocking (page fault, inherited driver path) is compensated so a stalled worker does not starve multiplexed Tasks. S-008 stays prototyped.

<!-- covers: INV-0071, INV-0380, INV-0038, GAP-0493 -->

#### Out of scope
Task object implementation (TSK-021). Userspace runtime (SDK-004). Personality threads (TSK-043).

#### Acceptance criteria
- [ ] Options evaluated include kernel-managed Tasks, UMCG-style activations with compensating workers, and a pure userspace runtime over async syscalls only.
- [ ] Each option states the kernel/runtime split and how page faults and synchronous inherited driver paths are detected and compensated.
- [ ] The decision cites the spike report against the B-014 live-Task scale and records that S-008 stays prototyped.
- [ ] ABI lead and TSK lead sign-off is recorded on the pull request.

#### Verification
- Review: ABI lead and TSK lead sign-off recorded on the pull request.

#### Evidence
- none

### TSK-010 · Implement Operation cancellation and deadline expiry with Cancelled and DeadlineExceeded results
- Type: build
- Milestone: V0
- Status: todo
- Size: L
- Owner: none
- Depends on: TSK-018, TSK-003, TSK-004, TSK-017, ABI-009
- Baseline: §19, §21, §59

Owner cancel and deadline expiry deliver through the normal completion path as typed `Cancelled` and `DeadlineExceeded` results (the Timeout result of V0-G07). Cancelling a TaskGroup cancels every Operation it owns. The committed-hardware state machine from the NVMe spike is implemented for the NVMe path on H-002 so a cancelled Operation never delivers a successful result.

<!-- covers: INV-0362, INV-0364, INV-0374, GAP-0495 -->

#### Out of scope
TaskGroup hierarchy walk (TSK-022). GPU and Wi-Fi committed-work matrix (TSK-048). Slack coalescing (TSK-047).

#### Acceptance criteria
- [ ] Cancelling an Operation by its owner completes it with `Cancelled` and never delivers a successful result, on `qemu-x86_64` and `hw-h002`.
- [ ] An Operation whose deadline has passed completes with `DeadlineExceeded` through the normal completion path.
- [ ] Cancelling a TaskGroup cancels every outstanding Operation it owns.
- [ ] An in-flight NVMe Read on H-002 follows the spike's committed-work contract and reports the decided partial-result shape.
- [ ] `os inspect` on an in-flight Operation shows owner, deadline and cancellation state (OBS consumes the data).

#### Verification
- Unit: `kernel:tests/tsk/cancel_*` and `kernel:tests/tsk/deadline_*` on `qemu-x86_64` and `hw-h002`.
- Integration: NVMe committed-work scenario on `hw-h002`.
- Fuzz: `kernel:fuzz/tsk_cancel_deadline` nightly without panic.

#### Evidence
- none

### TSK-011 · Implement Read, Write, Send and Receive Operation kinds
- Type: build
- Milestone: V0
- Status: todo
- Size: L
- Owner: none
- Depends on: TSK-018, ABI-014, IPC-010, MEM-005, STO-001
- Baseline: §18, §14, §16, §59

Read and Write run against File, Channel and MemoryObject-backed sources; Send and Receive are the typed Channel<T> kinds used by the V0 demo (§18, §59). Completions carry typed results or typed failures. Native software never sees a blocking read/write thread-per-call surface.

<!-- covers: INV-0346, INV-0347, INV-0348, INV-0349 -->

#### Out of scope
Channel object and backpressure (IPC). File object (STO). MemoryObject map (MEM). Connect and Accept (TSK-032).

#### Acceptance criteria
- [ ] A Read Operation against a File and against a MemoryObject-backed source completes with a typed result on `qemu-x86_64` and `hw-h002`.
- [ ] A Write Operation against a writable object completes with a typed result on those matrix entries.
- [ ] Send and Receive Operations on `Channel<T>` complete with typed messages on those matrix entries.
- [ ] Submitting Read, Write, Send or Receive returns without waiting for completion.
- [ ] The V0 Component-A to Channel to Component-B demo uses these kinds.

#### Verification
- Unit: `kernel:tests/tsk/kind_read_*`, `kind_write_*`, `kind_send_*`, `kind_recv_*` on `qemu-x86_64` and `hw-h002`.
- Integration: V0 demo pipeline on `qemu-x86_64` and `hw-h002`.
- Demo: Component A to Channel to Component B on H-002.

#### Evidence
- none

### TSK-012 · Implement Timer and Wait Operation kinds
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: TSK-018, TSK-004, TSK-006
- Baseline: §18, §19, §59

Timer completes at or after an absolute or relative deadline in the representation decided for the Operation ABI. Wait completes when a referenced object or set of objects reaches a signalled state. Both are V0 exit kinds and the native replacement for timerfd-style and signal-style waits.

<!-- covers: INV-0352, INV-0353 -->

#### Out of scope
User-signalled Event object (TSK-029). Slack and coalescing (TSK-047). Deadline overhead harness (TSK-039).

#### Acceptance criteria
- [ ] A Timer Operation with an absolute deadline completes at or after that deadline with a typed result on `qemu-x86_64` and `hw-h002`.
- [ ] A Timer Operation with a relative deadline completes at or after that deadline on those matrix entries.
- [ ] A Wait Operation completes when the referenced object reaches a signalled state.
- [ ] A Wait on an already-signalled object follows the inline-completion decision.

#### Verification
- Unit: `kernel:tests/tsk/kind_timer_*` and `kind_wait_*` on `qemu-x86_64` and `hw-h002`.
- Bench: B-009 on H-001 and H-002; target per register.

#### Evidence
- none

### TSK-013 · Implement Operation<Result> kernel Object with owner, typed result, priority and trace points
- Type: build
- Milestone: V0
- Status: todo
- Size: L
- Owner: none
- Depends on: TSK-023, TSK-007, ABI-015, ABI-009
- Baseline: §7, §18, §19, §69

Operation<Result> is the first-class unit of outstanding asynchronous work (§19, §69). Each Operation has an owner Task or TaskGroup so it can be cancelled structurally, a typed result or typed failure, a priority field carried for later ordering, and tracing hooks OBS consumes. Identity and handle encoding come from ABI-015.

<!-- covers: INV-0054, INV-0361, INV-0363, INV-0365, INV-0367, INV-1317 -->

#### Out of scope
Submit and completion transport (TSK-018). Priority ordering of I/O (TSK-033). Inspect rendering (OBS-007).

#### Acceptance criteria
- [ ] Userspace holds a Capability to an Operation, not the Object itself, using the encoding ABI decided.
- [ ] Creating an Operation records owner, kind, deadline, priority and trace identity inspectable through the OBS provider hooks.
- [ ] Completing an Operation delivers a typed result or a typed failure and no other payload.
- [ ] Destroying the owning TaskGroup without the cancel path still reclaims the Operation object with no kernel-memory leak in the V0 leak test.
- [ ] No `unsafe` outside the TSK Operation kernel files named on the pull request.

#### Verification
- Unit: `kernel:tests/tsk/operation_object_*` on `qemu-x86_64` and `hw-h002`.
- Integration: leak test creating and destroying Operations at the scale recorded in the verifying test on `qemu-x86_64`.

#### Evidence
- none

### TSK-014 · Prototype Operation submission/completion transports and measure wake-up latency
- Type: spike
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: none
- Baseline: §18, §58, §65
- Explores: S-005
- Risks: R-007

Studies io_uring submission and completion rings, linked operations and cancellation, then prototypes a shared ring, a per-Task wait object and a hybrid completion path on H-001 and H-002. Wake-up latency is measured for each option under B-009's method so decide-operation-transport is evidence-based. Nothing on S-005 is frozen.

<!-- covers: INV-1144, GAP-0494 -->

#### Out of scope
The transport decision (TSK-007). Permanent harness (TSK-026).

#### Acceptance criteria
- [ ] The report describes shared-ring, per-Task wait-object and hybrid prototypes that ran on H-001 and H-002.
- [ ] The report records wake-up latency for each prototype using the B-009 method and names the Linux io_uring NOP baseline.
- [ ] The report states whether io_uring internals can be reused or must be replaced, with the reason.
- [ ] The report lists what is not decided and does not freeze S-005.

#### Verification
- Report: answers which transport meets the V0 demo, how batches would be expressed, and whether io_uring internals are reusable; path `reports/spikes/TSK-014.md`.
- Bench: B-009 method on H-001 and H-002 for the three prototypes; no V0 absolute target.

#### Evidence
- none

### TSK-015 · Prototype in-kernel deadline enforcement and measure per-Operation overhead
- Type: spike
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: none
- Baseline: §19, §54
- Benchmarks: B-009
- Explores: S-005
- Risks: R-007

Prototypes a timer wheel and an hrtimer-per-Operation and measures per-Operation deadline overhead at high submission rates before Timer and the deadline path are built. Results feed decide-deadline-representation and the later permanent harness.

<!-- covers: INV-0376 -->

#### Out of scope
Deadline representation decision (TSK-004). Permanent harness (TSK-039). Timer kind (TSK-012).

#### Acceptance criteria
- [ ] The report describes timer-wheel and hrtimer-per-Operation prototypes that ran on H-001 and H-002.
- [ ] The report records submit-to-completion overhead with and without a deadline using the B-009 method.
- [ ] The report states whether a no-deadline Operation stays off the enforcement data structure.
- [ ] The report does not freeze S-005.

#### Verification
- Report: answers wheel versus per-Operation timer, overhead at high submission rates, and the no-deadline fast path; path `reports/spikes/TSK-015.md`.
- Bench: B-009 method on H-001 and H-002 for both prototypes.

#### Evidence
- none

### TSK-016 · Prototype Task multiplexing models and measure hidden blocking
- Type: spike
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: none
- Baseline: §20, §65
- Benchmarks: B-014
- Explores: S-008
- Risks: R-007

One of the V0 spikes that inform Layer 1 surfaces. Prototypes kernel-notified user scheduling (UMCG-style activations), a pure userspace runtime with async syscalls only, and kernel-managed lightweight Tasks at the live-Task scale recorded in B-014, and measures how page faults and synchronous inherited driver paths stall a worker. Feeds decide-task-mapping. Nothing on S-008 is frozen.

<!-- covers: GAP-0492, INV-0385, INV-0377, GAP-0493 -->

#### Out of scope
The mapping decision (TSK-009). Multiplexer implementation (TSK-019).

#### Acceptance criteria
- [ ] The report describes the three prototypes running on H-001 and H-002 at the B-014 live-Task scale.
- [ ] The report records how a page fault and a synchronous inherited driver path stall a worker in each prototype, and the compensation attempted.
- [ ] The report names which model is viable for V0 and which costs remain.
- [ ] The report does not freeze S-008.

#### Verification
- Report: answers kernel versus runtime split, hidden-blocking compensation, and B-014 viability per model; path `reports/spikes/TSK-016.md`.
- Bench: B-014 method on H-001 and H-002 for each prototype.

#### Evidence
- none

### TSK-017 · Prototype cancellation state machine for hardware-committed Operations on NVMe
- Type: spike
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: Q-009
- Baseline: §19
- Explores: S-005

Uniform cancellation is promised, but hardware makes some Operations uncancellable once DMA is issued. This spike prototypes the state machine against a real NVMe read on H-002 and answers whether cancel waits, fails, or is best-effort, and how partial results are reported. Feeds operation-cancel-deadline. GPU and Wi-Fi come later.

<!-- covers: GAP-0495, INV-0374 -->

#### Out of scope
Kernel implementation of the chosen machine (TSK-010). GPU and Wi-Fi matrix (TSK-048). GPUDispatch (TSK-049).

#### Acceptance criteria
- [ ] The report describes an NVMe Read issued on H-002 that cannot be aborted after DMA start.
- [ ] The report states the caller-visible result of cancel in that state: wait, fail, or best-effort, and the partial-result shape.
- [ ] The report names the rejected alternatives.
- [ ] The report does not freeze S-005.

#### Verification
- Report: answers wait versus fail versus best-effort, partial-result encoding, and what GPU/network work must reuse; path `reports/spikes/TSK-017.md`.
- Manual: NVMe Read cancel after DMA start on H-002, procedure recorded in the report.

#### Evidence
- none

### TSK-018 · Implement submit(Operation), completion delivery and poll/wait in the Native ABI
- Type: build
- Milestone: V0
- Status: todo
- Size: L
- Owner: none
- Depends on: TSK-013, TSK-007, TSK-005, ABI-002, ABI-012
- Baseline: §18, §19, §59, §65
- Benchmarks: B-009
- Invariants: I-030

Builds the transport chosen by decide-operation-transport and the inline-completion signalling chosen by decide-inline-completion. `submit` enqueues an Operation and returns without waiting. Completions report results back to the submitting Task. Poll and wait are the native observation paths. This is the V0 exit that asynchronous submission and completion work.

<!-- covers: INV-0342, INV-0343, INV-1160, INV-0340 -->

#### Out of scope
Kind implementations (TSK-011, TSK-012). Task wake integration (TSK-020). Ring hardening (TSK-040).

#### Acceptance criteria
- [ ] `submit` of a no-op Operation returns without waiting for completion on `qemu-x86_64` and `hw-h002`.
- [ ] A completed Operation is delivered to the submitting Task through the chosen completion path.
- [ ] Inline completion, if the decision allows it, is signalled exactly as the decision specifies.
- [ ] Poll and wait observe completion without a blocking native syscall as the primary mode.
- [ ] A B-009 publish run for no-op Operations exists on H-001 and H-002.

#### Verification
- Unit: `kernel:tests/tsk/submit_*` and `complete_*` on `qemu-x86_64` and `hw-h002`.
- Fuzz: `kernel:fuzz/tsk_submit` nightly without panic.
- Bench: B-009 on H-001 and H-002; target per register.

#### Evidence
- none

### TSK-019 · Implement Task multiplexing across bounded execution contexts
- Type: build
- Milestone: V0
- Status: todo
- Size: L
- Owner: none
- Depends on: TSK-009, TSK-021
- Baseline: §20, §59
- Benchmarks: B-014
- Invariants: I-017

The kernel and runtime multiplex Tasks across a bounded set of execution contexts so native software can create the live-Task population recorded in B-014 without a kernel thread per Task (§20). This implements the model chosen by decide-task-mapping, including hidden-blocking compensation. Wake-on-completion is a follow-on; V1 tuning is out of this task.

<!-- covers: INV-0379, INV-1156, INV-0377 -->

#### Out of scope
Completion wake (TSK-020). Userspace runtime crate (SDK-004). V1 affinity tuning (TSK-046).

#### Acceptance criteria
- [ ] Creating the B-014 live-Task population in one Component does not create one kernel thread per Task, on `qemu-x86_64` and `hw-h002`.
- [ ] A worker that takes a page fault or a synchronous inherited driver path is compensated so other Tasks on that worker become runnable.
- [ ] Destroying the Component reclaims every execution context and Task with no unbounded kernel-memory growth in the leak test.
- [ ] Native software has no thread-create ABI.

#### Verification
- Unit: `kernel:tests/tsk/mux_*` on `qemu-x86_64` and `hw-h002`.
- Bench: B-014 on H-001 and H-002; target per register.
- Integration: hidden-blocking compensation scenario on `hw-h002`.

#### Evidence
- none

### TSK-020 · Integrate Operation completion with Task suspension and wake on execution contexts
- Type: build
- Milestone: V0
- Status: todo
- Size: L
- Owner: none
- Depends on: TSK-019, TSK-018
- Baseline: §18, §20, §67
- Benchmarks: B-003, B-014
- Invariants: I-030

Completion integrates directly with Task scheduling (§18). The kernel records which Operation a suspended Task awaits and wakes that Task on completion so the native runtime binds to kernel Operations rather than reinventing async I/O (Principle 5). B-014's waiting Tasks and B-003's handoff measurement need this path.

<!-- covers: INV-1296 -->

#### Out of scope
Inspect rendering of the awaited Operation (OBS-005). Debugger stacks (TSK-038). Direct Channel handoff (IPC-015).

#### Acceptance criteria
- [ ] A Task that submits an Operation and suspends is recorded as waiting on that Operation, visible to the OBS provider hook.
- [ ] Completing the Operation makes the Task runnable on an execution context without a kernel thread per Task.
- [ ] The B-014 population blocking on Wait Operations all become runnable when those Waits signal.
- [ ] Native Task-to-Task handoff is measurable under B-003 on H-001 and H-002.

#### Verification
- Unit: `kernel:tests/tsk/wake_*` on `qemu-x86_64` and `hw-h002`.
- Bench: B-003 and B-014 on H-001 and H-002; targets per register.
- Integration: B-014 Wait-and-wake scenario on `qemu-x86_64`.

#### Evidence
- none

### TSK-021 · Implement Task as the native concurrency abstraction
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: TSK-008, TSK-023
- Baseline: §1, §20, §59, §69
- Benchmarks: B-002
- Invariants: I-017

V0 creates a native Task (§59, §69). Threads are not a native API. This object is what native software runs, implemented per decide-task-identity, owned by a TaskGroup. Spawn latency is published under B-002.

<!-- covers: INV-0047, INV-1314 -->

#### Out of scope
Multiplexing (TSK-019). Cancellation walk (TSK-022). Personality threads (TSK-043).

#### Acceptance criteria
- [ ] A Component can spawn a Task into its TaskGroup and the Task becomes runnable, on `qemu-x86_64` and `hw-h002`.
- [ ] The Task identity matches decide-task-identity (kernel object or runtime identity with kernel visibility).
- [ ] Native software has no thread-create, thread-join or thread-kill ABI.
- [ ] Destroying the Task reclaims its kernel state with no leak in the V0 leak test.
- [ ] A B-002 publish run exists on H-001 and H-002.

#### Verification
- Unit: `kernel:tests/tsk/task_object_*` on `qemu-x86_64` and `hw-h002`.
- Bench: B-002 on H-001 and H-002; target per register.

#### Evidence
- none

### TSK-022 · Propagate cancellation through the TaskGroup hierarchy
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: TSK-023, TSK-021, TSK-003, TSK-010
- Baseline: §21, §59
- Invariants: I-031

V0 cancellation demo: cancelling a TaskGroup cancels owned Tasks, child groups and outstanding Operations; application cancel waits until every owned Task has terminated. A regression proves no Task survives group cancel or application exit. Operation `Cancelled` results are produced by the cancel-deadline path; the background-execution Capability exception is V0.5.

<!-- covers: INV-0389, INV-0390, INV-1165, INV-0393, INV-0392 -->

#### Out of scope
Background-execution Capability (TSK-025). Deadline inheritance (TSK-036). Operation result encoding (TSK-010).

#### Acceptance criteria
- [ ] Cancelling a TaskGroup cancels owned Tasks, child TaskGroups and their outstanding Operations on `qemu-x86_64` and `hw-h002`.
- [ ] Application cancel completes only after every owned Task has terminated.
- [ ] A regression in CI proves no Task remains runnable after group cancel and after application exit.
- [ ] A multi-level tree cancel completes with every Task terminated before the test returns (V0-G08).

#### Verification
- Unit: `kernel:tests/tsk/taskgroup_cancel_*` on `qemu-x86_64` and `hw-h002`.
- Integration: multi-level tree cancel scenario on `qemu-x86_64`.
- Demo: killing the parent TaskGroup tears down A, B and in-flight Operations on H-002.

#### Evidence
- none

### TSK-023 · Implement Object<TaskGroup> with ownership hierarchy
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: TSK-008, ABI-010, CMP-014
- Baseline: §7, §21, §59
- Invariants: I-017

`Object<TaskGroup>` is a Capability-referenced kernel object (§7, §21). Application owns TaskGroups; TaskGroups own Tasks and child TaskGroups; each Component owns a TaskGroup. V0 creates a TaskGroup with hierarchical parent/child relationships.

<!-- covers: INV-0048, INV-0169, INV-0387, INV-0388, INV-1157 -->

#### Out of scope
Cancellation propagation (TSK-022). Component create wrapping this object (CMP). Inspect of the tree at V0.5 (OBS-018).

#### Acceptance criteria
- [ ] A Component's owned TaskGroup is a kernel object referenced by Capability, on `qemu-x86_64` and `hw-h002`.
- [ ] A TaskGroup can own Tasks and child TaskGroups; parent/child relationships are recorded.
- [ ] Destroying a TaskGroup without children and Tasks reclaims the object with no leak.
- [ ] Userspace cannot mint a TaskGroup handle (forgery returns `Error::Rights` and allocates no handle).

#### Verification
- Unit: `kernel:tests/tsk/taskgroup_object_*` on `qemu-x86_64` and `hw-h002`.
- Integration: Component-owned TaskGroup create/destroy in the V0 Component leak test.

#### Evidence
- none

### TSK-024 · Build V0 Operation acceptance suite for six kinds, deadline and cancellation
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: TSK-011, TSK-012, TSK-010, TSK-022, TSK-020, TSK-001
- Baseline: §18, §19, §59

Verifies the V0-G07 and V0-G08 exit criteria: Read, Write, Send, Receive, Timer and Wait complete; a deadline yields `DeadlineExceeded`; a cancelled Operation never delivers a successful result; cancelling a TaskGroup cancels owned work. The suite runs on H-001 and H-002 in CI.

<!-- covers: INV-1160, INV-0362, INV-0364 -->

#### Out of scope
Connect, Accept, DeviceOperation (V0.5 kinds). Benchmark publication (TSK-002, BEN-001).

#### Acceptance criteria
- [ ] The suite passes Read, Write, Send, Receive, Timer and Wait completion on `qemu-x86_64` and `hw-h002`.
- [ ] A deadline case in the suite yields `DeadlineExceeded` and no successful result.
- [ ] A cancel case in the suite yields `Cancelled` and no successful result.
- [ ] TaskGroup cancel in the suite leaves no runnable owned Task.
- [ ] The suite is a required CI job on every merge to main.

#### Verification
- Integration: `kernel:tests/tsk/v0_acceptance_*` on `qemu-x86_64` and `hw-h002`.
- Demo: V0 cancellation demo on H-002.

#### Evidence
- none

### TSK-025 · Require an explicit Capability for Tasks that outlive their TaskGroup
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: TSK-022, CAP-017
- Baseline: §21
- Invariants: I-031

Persistent background execution requires an explicit Capability and no accidental orphans (§21). Native init and SVC-supervised services appear at V0.5. CAP defines `Capability<BackgroundExecution>` and the rights check; this task enforces the Operation and Task side: a Task that would outlive its TaskGroup without that Capability is refused, and the V0 cancel regression still holds for everything else.

<!-- covers: INV-0392 -->

#### Out of scope
Capability type and rights word (CAP-017). Service supervision (SVC). Automation background rules (SEM).

#### Acceptance criteria
- [ ] Spawning a Task that would outlive its TaskGroup without `Capability<BackgroundExecution>` returns `Error::Rights` and allocates no Task.
- [ ] A Task holding that Capability remains runnable after its original TaskGroup is cancelled, and is listed as a background Task in inspect data.
- [ ] The V0 no-orphan regression still passes for Components that do not hold the Capability, on `qemu-x86_64` and `hw-h002`.
- [ ] Dropping the Capability cancels the background Task through the normal cancellation path.

#### Verification
- Unit: `kernel:tests/tsk/background_cap_*` on `qemu-x86_64` and `hw-h002`.
- Integration: supervised service Task surviving window close on `qemu-x86_64`.

#### Evidence
- none

### TSK-026 · Benchmark submit-to-completion wake-up latency and publish
- Type: benchmark
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: TSK-018, TSK-012, TSK-030, BEN-005
- Baseline: §18, §54
- Benchmarks: B-009
- Risks: R-009

Turns the V0 spike's one-off wake-up measurement into a permanent harness: no-op, Timer and Wait Operations, same-core and cross-core, single and batched, so V0.5 and V1 regression gates rest on B-009 rather than a claim. The V0.5 target kind is regression versus V0.

<!-- covers: INV-0359 -->

#### Out of scope
IPC round trip (B-004). Deadline-enforcement overhead as a distinct series (TSK-039). Methodology (BEN-007).

#### Acceptance criteria
- [ ] A committed B-009 report exists for H-001, H-002 and H-003 meeting the V0.5 regression target versus V0.
- [ ] The harness covers no-op, Timer and Wait kinds, same-core and cross-core, single and batched submission.
- [ ] Each report names the io_uring NOP, timerfd and eventfd baselines run in the same session.
- [ ] No public material states a superiority claim without citing those reports.

#### Verification
- Bench: B-009 on H-001, H-002 and H-003; target per register.
- Review: BEN lead confirms the reports follow the accepted methodology.

#### Evidence
- none

### TSK-027 · Decide how Operation priority relates to ResourceDomain Scheduling intent
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: TSK-013, SCH-004
- Baseline: §19, §22
- Decision: D-0310
- Invariants: I-032

Every Operation carries a priority field (§19). This decision picks whether that priority is inherited from the owning ResourceDomain's scheduling intent, a per-Operation override bounded by the domain, or independent of intent. It must land before the kernel orders I/O and IPC by it and before Interactive and Background classes are consumed by compositor Operations.

<!-- covers: INV-0365 -->

#### Out of scope
Intent class implementation (SCH). Ordering implementation (TSK-033). Channel handoff inheritance (SCH-017).

#### Acceptance criteria
- [ ] Options evaluated include inherit-from-domain intent, per-Operation override bounded by the domain, and independent Operation priority.
- [ ] Each option states how a compositor Operation and a Background Operation are ordered when both are in flight.
- [ ] The decision records that native software expresses intent, not a POSIX nice value (I-032).
- [ ] SCH lead and TSK lead sign-off is recorded on the pull request.

#### Verification
- Review: SCH lead and TSK lead sign-off recorded on the pull request.

#### Evidence
- none

### TSK-028 · Decide Operation Ownership transfer semantics across Tasks and TaskGroups
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: TSK-013, TSK-003
- Baseline: §19, §21, §32
- Decision: D-0312

operation-object gives every Operation an owner. This decision says what transfer of that owner across Tasks and TaskGroups means for completion delivery, cancellation and ResourceDomain accounting. Service restart re-owns in-flight work during rebind and cannot be built without it.

<!-- covers: INV-0367 -->

#### Out of scope
Rebind implementation (TSK-035). Capability transfer over Channels (IPC). MemoryObject ownership transfer (MEM).

#### Acceptance criteria
- [ ] Options evaluated include moving completion delivery to the new owner, cancelling the Operation on transfer, and dual delivery until the new owner accepts.
- [ ] Each option states who may cancel after transfer and which ResourceDomain is charged.
- [ ] The decision names the typed error a caller observes if transfer is refused.
- [ ] TSK lead sign-off is recorded on the pull request.

#### Verification
- Review: TSK lead sign-off recorded on the pull request.

#### Evidence
- none

### TSK-029 · Implement a native Event Object signalled by user space and consumed via Wait
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: TSK-006, TSK-012
- Baseline: §18, §19, §41
- Invariants: I-018

Realises decide-native-notification: a kernel Event object that user space signals and that Wait Operations consume. It is the native replacement for futex, eventfd and signal wake-ups that the toolkit, compositor and Wayland bridge need for cross-Task synchronisation without ambient signals.

<!-- covers: INV-0039 -->

#### Out of scope
Channel messages (IPC). Personality futex and eventfd (LNX). Wait kind itself (TSK-012).

#### Acceptance criteria
- [ ] A Task can create an Event object, signal it, and a Wait Operation on that Event completes, on `qemu-x86_64` and `hw-h002`.
- [ ] Signalling an Event with no waiter records the signalled state so a later Wait completes per the inline-completion decision.
- [ ] Native software has no futex, eventfd or signal ABI.
- [ ] Forgery of an Event handle returns `Error::Rights` and allocates no handle.

#### Verification
- Unit: `kernel:tests/tsk/event_*` on `qemu-x86_64` and `hw-h002`.
- Integration: two-Task Event wake on `qemu-x86_64`.

#### Evidence
- none

### TSK-030 · Implement batched submission and linked Operation chains
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: TSK-007, TSK-018, TSK-011, TSK-012
- Baseline: §18, §58

decide-operation-transport fixes how batches are expressed. This task implements batch submit and io_uring-style links (Read then Send, Timer-bounded chains) that the compositor frame loop and File Browser need at V0.5. A failed link stops the chain with a typed result. Required by V0.5-G01 (Native compositor presents on the reference GPU): the compositor frame loop submits its per-frame Operations as linked batches.

#### Out of scope
Transport decision (TSK-007). IPC batched send/receive at V1 (IPC-043).

#### Acceptance criteria
- [ ] A batch of Operations submitted together is accepted in one submit on `qemu-x86_64` and `hw-h002`.
- [ ] A linked Read-then-Send chain runs the Send only after the Read completes successfully.
- [ ] A linked Timer-bounded chain completes with `DeadlineExceeded` and does not run later links when the Timer fires first.
- [ ] A failed link completes remaining linked Operations with a typed failure and does not run their side effects.

#### Verification
- Unit: `kernel:tests/tsk/batch_*` and `link_*` on `qemu-x86_64` and `hw-h002`.
- Fuzz: `kernel:fuzz/tsk_batch_link` nightly without panic.

#### Evidence
- none

### TSK-031 · Implement DeviceOperation kind for Object<Device> including user-space drivers
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: TSK-018, ABI-014, HW-008
- Baseline: §18, §33, §39

Asynchronous requests to `Object<Device>` instances, including devices backed by kernel drivers and by SVC-hosted user-space drivers. The V0.5 compositor drives DRM/KMS through this kind. Native software never opens a DRM device node.

<!-- covers: INV-0355 -->

#### Out of scope
`Object<Device>` definition (HW-008). User-space driver hosting (SVC, HW-029). GPUDispatch (TSK-049). DRM ioctls as a native API (GFX).

#### Acceptance criteria
- [ ] A DeviceOperation against an `Object<Device>` Capability completes with a typed result on `qemu-x86_64` and `hw-h002`.
- [ ] Submitting DeviceOperation without a Device Capability returns `Error::Rights` and allocates no Operation.
- [ ] A DeviceOperation to a SVC-hosted user-space driver uses the same kind and completion path as a kernel-backed device.
- [ ] Native crates have no DRM ioctl or `/dev/dri` entry point.

#### Verification
- Unit: `kernel:tests/tsk/kind_device_*` on `qemu-x86_64` and `hw-h002`.
- Integration: compositor DeviceOperation path on `hw-h002` and `qemu-x86_64` with virtio-gpu (H-003).

#### Evidence
- none

### TSK-032 · Implement Connect and Accept Operation kinds
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: TSK-018, ABI-014, IPC-010
- Baseline: §18, §32

Connect establishes a connection to a service or listening object; Accept receives inbound connections. Compositor clients and service rebind need both at V0.5. `Object<NetworkConnection>` arrives with NET at V1 over the same kinds, so the kind ABI must not assume a socket.

<!-- covers: INV-0350, INV-0351 -->

#### Out of scope
NetworkConnection object (NET-014). Service discovery (IPC-023). Channel close (IPC-026).

#### Acceptance criteria
- [ ] A Connect Operation to a listening service completes with a typed connection object on `qemu-x86_64` and `hw-h002`.
- [ ] An Accept Operation on a listening object completes with a typed inbound connection.
- [ ] Connect without the required Capability returns `Error::Rights` and allocates no Operation.
- [ ] Native software has no socket, bind, listen or accept ABI.

#### Verification
- Unit: `kernel:tests/tsk/kind_connect_*` and `kind_accept_*` on `qemu-x86_64` and `hw-h002`.
- Integration: compositor client Connect against a listening compositor on `qemu-x86_64`.

#### Evidence
- none

### TSK-033 · Order I/O, IPC and wake-up by Operation priority
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: TSK-027, TSK-013, SCH-010, SCH-025
- Baseline: §19, §22, §40
- Invariants: I-032

Plumbs the carried Operation priority into block I/O ordering, Channel queueing and completion wake ordering per decide-operation-priority-model so compositor Operations preempt Background work for the V0.5 dropped-frame gate. Scheduling intent remains a SCH concept; this task is the Operation-side ordering.

<!-- covers: INV-0365 -->

#### Out of scope
Intent classes (SCH). Channel backpressure policy (IPC). Frame scheduling (GFX, SCH-015).

#### Acceptance criteria
- [ ] Under a Background flood, an Interactive or Deadline Operation is selected for I/O, Channel service and wake before Background Operations, on `qemu-x86_64` and `hw-h002`.
- [ ] Ordering matches the accepted priority model (inherit, bounded override, or independent).
- [ ] Native software has no `nice` or `SCHED_FIFO` ABI.
- [ ] A regression in CI records the ordering with `os trace` scheduling-delay points.

#### Verification
- Unit: `kernel:tests/tsk/priority_order_*` on `qemu-x86_64` and `hw-h002`.
- Integration: compositor Operations versus Background flood on `hw-h002`.

#### Evidence
- none

### TSK-034 · Charge outstanding Operations and completion queue memory to the owning ResourceDomain
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: TSK-013, SCH-009, SCH-007
- Baseline: §19, §23
- Threats: T-016
- Invariants: I-033

§19 lists resource accounting as an Operation property. In-flight Operations and ring slots count against SCH kernel-object limits so a runaway Component cannot exhaust kernel memory once multiple applications run at V0.5. Exhaustion returns a typed error.

<!-- covers: EXTRA-002, INV-0368 -->

#### Out of scope
ResourceDomain object and limits (SCH). Channel queue charging (IPC-027). MemoryObject charging (MEM).

#### Acceptance criteria
- [ ] Outstanding Operations and completion-queue slots are charged to the owning ResourceDomain, visible via inspect data.
- [ ] Hitting the domain's outstanding-Operation limit returns a typed exhaustion error and allocates no Operation, on `qemu-x86_64` and `hw-h002`.
- [ ] Completing or cancelling an Operation releases the charge.
- [ ] A runaway submit loop cannot grow kernel memory unbounded in the leak test.

#### Verification
- Unit: `kernel:tests/tsk/op_accounting_*` on `qemu-x86_64` and `hw-h002`.
- Integration: exhaustion and reclaim scenario on `qemu-x86_64`.

#### Evidence
- none

### TSK-035 · Complete in-flight Operations against a restarted service with typed Disconnected results
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: TSK-028, TSK-011, TSK-031, TSK-032, IPC-026
- Baseline: §19, §32
- Invariants: I-037

V0.5 exit: killing the compositor rebinds all windows with no application exit. Outstanding Send, Receive and DeviceOperation work against the dead instance must fail typed (`Disconnected`) and be resubmittable against the rebound endpoint, coordinated with SVC supervision and IPC rebind.

<!-- covers: INV-1185 -->

#### Out of scope
Supervisor restart policy (SVC). Client rebind codegen (IPC-028, SDK-012). Surface rebind (GFX).

#### Acceptance criteria
- [ ] In-flight Send, Receive and DeviceOperation against a killed service complete with typed `Disconnected` and never deliver a successful result, on `qemu-x86_64` and `hw-h002`.
- [ ] After rebind, a new Operation against the same interface identity can be submitted and completed.
- [ ] Ownership of in-flight Operations follows decide-ownership-transfer; no Operation remains owned by the dead instance.
- [ ] The compositor-kill loop named by SVC-002 observes only typed failures, not process death of the client.

#### Verification
- Integration: `kernel:tests/tsk/rebind_inflight_*` on `qemu-x86_64` and `hw-h002`.
- Demo: compositor kill and window rebind with in-flight DeviceOperation on H-002.

#### Evidence
- none

### TSK-036 · Inherit TaskGroup deadlines and outstanding-Operation limits by owned Operations
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: TSK-023, TSK-010, TSK-034
- Baseline: §21, §19

Operations submitted inside a TaskGroup take the minimum of their own deadline and the group's deadline, and share the group's outstanding-Operation budget, so cancelling or timing out a group tears down in-flight work deterministically. Hardens the V0 cancellation demo for real applications.

<!-- covers: INV-0389 -->

#### Out of scope
Background-execution exception (TSK-025). Domain-wide kernel-object limits (SCH).

#### Acceptance criteria
- [ ] An Operation whose own deadline is later than its TaskGroup deadline completes with `DeadlineExceeded` at the group deadline, on `qemu-x86_64` and `hw-h002`.
- [ ] Operations submitted in a TaskGroup share that group's outstanding-Operation budget; exceeding it returns a typed exhaustion error.
- [ ] Cancelling or timing out the group cancels every in-flight owned Operation.
- [ ] A child TaskGroup inherits the tighter of its own and its parent's deadline.

#### Verification
- Unit: `kernel:tests/tsk/taskgroup_deadline_*` on `qemu-x86_64` and `hw-h002`.
- Integration: nested group timeout scenario on `qemu-x86_64`.

#### Evidence
- none

### TSK-037 · Publish Operation, deadline and cancellation guidelines for service authors
- Type: docs
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: TSK-024, TSK-003, IPC-032
- Baseline: §18, §19, §21, §52

V1 developer preview needs guidelines on when to add an Operation kind versus a Channel method, deadline conventions, and the cancellation contract callers may observe. Complements IPC's interface design guidelines. DOC publishes; TSK authors the Operation semantics.

#### Out of scope
IDL authoring guidelines (IPC-032). Runtime binding contracts (TSK-045). Layer 1 reference pages (TSK-050).

#### Acceptance criteria
- [ ] A committed guide exists covering Operation versus Channel method, deadline conventions and the cancellation contract.
- [ ] The guide cites §18 to §21 and names the typed results `Cancelled`, `DeadlineExceeded` and `Disconnected`.
- [ ] DOC and SDK leads record review on the pull request.

#### Verification
- Review: DOC lead and SDK lead sign-off recorded on the pull request.

#### Evidence
- none

### TSK-038 · Maintain logical async Task stacks and await chains for debugger and profiler
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: TSK-020, OBS-005
- Baseline: §20, §64

V1 exit: a debugger breaks inside an async Task and shows the logical Task stack; a profiler attributes samples to Task and Component. The runtime records parent and await relationships that OBS and SDK tools read. TSK owns the kernel and runtime metadata; SDK owns the debugger and profiler CLIs.

<!-- covers: INV-0984, EXTRA-032 -->

#### Out of scope
Debugger attach (SDK-038). Profiler sampling (SDK-046). Inspect command rendering (OBS, SDK).

#### Acceptance criteria
- [ ] A suspended Task records its parent Task and the Operation it awaits, readable through the OBS inspect provider, on `qemu-x86_64` and `hw-h004`.
- [ ] An await chain of nested Tasks is returned as a logical stack with no kernel-thread stack required.
- [ ] Sample-attribution identity for a running Task is stable for the duration of the Task.
- [ ] Debugger attach remains a debug Capability, not a same-uid check (T-027).

#### Verification
- Unit: `kernel:tests/tsk/async_stack_*` on `qemu-x86_64` and `hw-h004`.
- Integration: SDK debugger reads the logical stack in a fixture on `qemu-x86_64`.

#### Evidence
- none

### TSK-039 · Benchmark per-Operation deadline overhead at high submission rates and publish
- Type: benchmark
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: TSK-015, TSK-010, TSK-026, BEN-005
- Baseline: §19, §54
- Benchmarks: B-009
- Risks: R-009

Makes the V0 spike's deadline-overhead measurement a permanent regression harness under BEN's anti-fake-claim policy (I-061) before V1 overhead gates are judged. Uses B-009 with a deadline-on versus deadline-off comparison at high submission rates.

<!-- covers: INV-0376 -->

#### Out of scope
Timer slack (TSK-047). Linux personality overhead (B-026, BEN-027).

#### Acceptance criteria
- [ ] A committed B-009 report exists for H-001, H-002 and H-004 comparing deadline-on versus deadline-off at high submission rates.
- [ ] The V1 regression target versus V0.5 is met or an accepted decision documents the exception.
- [ ] No public material states a superiority claim without citing those reports.

#### Verification
- Bench: B-009 on H-001, H-002 and H-004; target per register.
- Review: BEN lead confirms the deadline-on/off series is labelled in the report.

#### Evidence
- none

### TSK-040 · Harden shared submission/completion memory against forgery, TOCTOU and exhaustion
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: TSK-018, TSK-034, SEC-002
- Baseline: §18, §19, §51
- Threats: T-003, T-016

The V0 threat model names shared rings as an attack surface. Before external developers submit Operations at V1, the kernel validates user-writable entries once, bounds outstanding work, and rejects forged completions. TOCTOU between validate and execute is closed. Required by V3-G10 (Kernel and IPC fuzzing has no stale open crasher): the forged-completion and exhaustion oracles of TSK-051 assume this hardening.

#### Out of scope
Threat-model document (SEC-002). Handle-table generation (ABI). Capability rights encoding (CAP).

#### Acceptance criteria
- [ ] A forged completion record is rejected, delivers no result to any Task, and allocates no handle, on `qemu-x86_64` and `hw-h004`.
- [ ] Mutating a submission entry after validation and before execute cannot change the accepted Operation (TOCTOU test).
- [ ] Exhausting outstanding work returns a typed exhaustion error and grows no unbounded kernel memory.
- [ ] Negative tests for T-003 and T-016 are required CI on `qemu-x86_64`.

#### Verification
- Unit: `kernel:tests/tsk/ring_harden_*` on `qemu-x86_64` and `hw-h004`.
- Fuzz: `kernel:fuzz/tsk_ring` nightly without panic.
- Review: SEC lead records that T-003 and T-016 are addressed by the tests.

#### Evidence
- none

### TSK-041 · Preserve Operation deadlines correctly across suspend and resume
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: TSK-004, TSK-012, SVC-016, PWR-014
- Baseline: §19, §61
- Benchmarks: B-030

Implements the suspend/resume rule from decide-deadline-representation, refined by SVC's clock-semantics adr, on H-004 and H-002. V1 suspend-cycle gates require Timers and deadlines to behave as the decision states after wake. PWR owns the suspend mechanism; TSK owns deadline arithmetic across it.

<!-- covers: GAP-0496 -->

#### Out of scope
Suspend/resume implementation (PWR-014). Clock-semantics decision (SVC-016). Slack coalescing (TSK-047).

#### Acceptance criteria
- [ ] A Timer Operation submitted before suspend completes according to the accepted clock rule after resume, on `hw-h004` and `hw-h002`.
- [ ] An Operation whose deadline passed during suspend completes with `DeadlineExceeded` after resume if the decision says suspended time counts, or remains waiting if it does not.
- [ ] Post-resume, new Timer Operations complete at the decided deadline with services functional as required by the V1 suspend gate.
- [ ] The behaviour is recorded in the PWR suspend-cycle harness logs.

#### Verification
- Integration: `kernel:tests/tsk/deadline_suspend_*` on `hw-h004` and `hw-h002`.
- Bench: B-030 on H-004; target per register.
- Manual: one instrumented suspend cycle on H-004 with a Timer in flight, procedure on the pull request.

#### Evidence
- none

### TSK-042 · Decide which Operation ABI surfaces become Layer 1 freeze candidates
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: TSK-024, TSK-007, TSK-004, TSK-005, TSK-003, ABI-011
- Baseline: §65, §66
- Decision: D-0308
- Risks: R-007, R-028
- Invariants: I-040

L1 freeze candidates are named at V1 with SDK v1 and frozen at V4 (I-040). This decision names the submission entry, completion record layout, result encoding, deadline representation and kind set that enter the ABI snapshot check. It does not freeze S-005 or S-008. Required by V4-G01 (Layer 1 ABI frozen with a conformance suite): the V4 freeze snapshots only the surfaces named as candidates at V1.

#### Out of scope
The V4 freeze (ABI-049). Conformance suite (TSK-052). Channel freeze candidates (IPC).

#### Acceptance criteria
- [ ] Options evaluated include the full Operation candidate set, a reduced core (submit, complete, cancel, six V0 kinds), and deferring naming to V4.
- [ ] The decision lists each named candidate with its spike and adr, and records that no Layer 1 surface is frozen.
- [ ] S-005 and S-008 remain prototyped in the surfaces register after this task is done.
- [ ] ABI lead sign-off is recorded on the pull request.

#### Verification
- Review: ABI lead sign-off recorded on the pull request.

#### Evidence
- none

### TSK-043 · Decide how Personality threads map onto native Tasks
- Type: adr
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: TSK-009, TSK-019, CMP-036
- Baseline: §3, §20, §46, §48
- Decision: D-0313
- Invariants: I-025

How Linux and Windows personality threads map onto native Tasks and execution contexts. Needed before V1 daily-driving through LNX and non-gated Wine bring-up. TSK owns the native side of the mapping; LNX and WIN consume it. Native software still never sees a thread ABI.

<!-- covers: INV-0386 -->

#### Out of scope
Component mapping of personality processes (CMP-036). Wine hosting (WIN-013). Linux syscall path (LNX).

#### Acceptance criteria
- [ ] Options evaluated include one native Task per personality thread, M:N personality threads onto native Tasks, and personality threads as execution contexts wrapping native Tasks.
- [ ] Each option states how cancellation, inspect identity and ResourceDomain charging look for a personality thread.
- [ ] The decision records that native software has no thread ABI and that personalities consume the native ABI (I-025).
- [ ] LNX lead, WIN lead and TSK lead sign-off is recorded on the pull request.

#### Verification
- Review: LNX lead, WIN lead and TSK lead sign-off recorded on the pull request.

#### Evidence
- none

### TSK-044 · Implement StorageTransaction Operation kind over the storage durability contract
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: TSK-018, ABI-014, STO-038, STO-031
- Baseline: §18, §26

§18 lists StorageTransaction. STO's durability contract lands at V1 with system history and `os env` snapshots. TSK owns the async Operation kind plumbing (submit, complete, cancel, deadline) over that contract. STO owns commit and abort semantics and the power-cut test.

<!-- covers: INV-0356 -->

#### Out of scope
Durability contract (STO-038). Multi-object transactions (STO-051). Power-cut test (STO-048).

#### Acceptance criteria
- [ ] A StorageTransaction Operation submits, completes with commit or abort, and is cancellable with `Cancelled`, on `qemu-x86_64` and `hw-h004`.
- [ ] Deadline expiry on a StorageTransaction yields `DeadlineExceeded` and does not commit.
- [ ] Native software has no `fsync` or `fdatasync` ABI.
- [ ] The kind is listed in the V1 ABI snapshot check as prototyped.

#### Verification
- Unit: `kernel:tests/tsk/kind_storage_txn_*` on `qemu-x86_64` and `hw-h004`.
- Integration: STO commit/abort fixture driven through the Operation kind on `qemu-x86_64`.

#### Evidence
- none

### TSK-045 · Document how language runtimes bind to kernel Operations
- Type: docs
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: TSK-018, TSK-010, TSK-020, SDK-005
- Baseline: §18, §52, §67
- Invariants: I-030

SDK v1 and C bindings ship at V1. The guide specifies waker, completion-record and cancellation contracts so Rust, C and later runtimes bind to kernel Operations rather than reinventing async I/O (Principle 5).

<!-- covers: INV-1296 -->

#### Out of scope
C binding implementation (SDK-033). Native runtime (SDK-004). Service-author guidelines (TSK-037).

#### Acceptance criteria
- [ ] A committed guide exists covering waker registration, completion-record layout and the cancellation contract for Rust and C.
- [ ] The guide forbids a runtime async layer that does not submit kernel Operations.
- [ ] SDK lead sign-off is recorded on the pull request.

#### Verification
- Review: SDK lead sign-off recorded on the pull request.

#### Evidence
- none

### TSK-046 · Tune Task multiplexing with per-core completion affinity and work distribution
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: TSK-020, TSK-026, TSK-002
- Baseline: §20, §54
- Benchmarks: B-003, B-004, B-005, B-009

V1 sets absolute IPC targets (B-004, B-005) and must not regress V0 Task handoff (B-003) or Operation completion (B-009). Completion affinity, wake batching and idle-Task memory footprint are the TSK levers. Numbers live only in the registers. Required by V1-G15 (IPC round trip meets the V1 absolute target): the B-004 and B-005 runs depend on completion affinity and wake batching.

#### Out of scope
IPC fast-path internals (IPC-054). Idle-component memory (CMP, B-008). Hidden-blocking model (TSK-009).

#### Acceptance criteria
- [ ] B-003, B-004, B-005 and B-009 meet their V1 targets on H-002, or an accepted decision documents the exception.
- [ ] Per-core completion affinity is inspectable and can be pinned in the harness.
- [ ] Idle-Task memory is published under B-008 on H-002 and meets the V1 target or the exception path above.
- [ ] V0 B-003 and B-009 reports are not regressed beyond the register band on H-001 and H-002.

#### Verification
- Bench: B-003, B-004, B-005, B-009 on H-001, H-002 and H-004; targets per register.
- Integration: completion-affinity pin test on `hw-h002`.

#### Evidence
- none

### TSK-047 · Add slack and coalescing to Timer Operations for energy efficiency
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: TSK-012, TSK-027, SCH-037
- Baseline: §19, §22, §54
- Benchmarks: B-031

V1 idle power and battery runtime (B-031) need Timer Operations to carry slack derived from EnergyEfficient and Background intent so idle desktops coalesce wake-ups instead of ticking per Component. Interactive and Deadline Timers do not take slack that would miss their deadline.

#### Out of scope
EnergyEfficient intent class (SCH-037). Power meters and methodology (LAB, BEN). Frequency hints (SCH-038).

#### Acceptance criteria
- [ ] A Timer submitted under EnergyEfficient or Background intent carries slack and may complete after its deadline by at most that slack, on `hw-h004`.
- [ ] Two Timers in the same slack window coalesce to one wake-up, observable in `os trace`.
- [ ] Interactive and Deadline Timers complete at or after their deadline without slack delay.
- [ ] B-031 idle-desktop runs on H-004 are taken with coalescing enabled.

#### Verification
- Unit: `kernel:tests/tsk/timer_slack_*` on `qemu-x86_64` and `hw-h004`.
- Bench: B-031 on H-004; target per register.

#### Evidence
- none

### TSK-048 · Verify cancellation state machine against NVMe, GPU and Wi-Fi on all target machines
- Type: build
- Milestone: V2
- Status: todo
- Size: M
- Owner: none
- Depends on: TSK-010, TSK-049, NET-014, HET-012
- Baseline: §19, §62

The V0 spike covered one NVMe path on H-002. V2's three target machines add laptops, Wi-Fi and GPUs, so the committed-work contract must be exercised per hardware class in CI. TSK owns the Operation-visible results; HET owns GPU abort semantics; NET owns the Wi-Fi send path.

<!-- covers: GAP-0495 -->

#### Out of scope
GPUDispatch kind (TSK-049). GPU abort semantics (HET-012). NetworkConnection object (NET).

#### Acceptance criteria
- [ ] NVMe Read cancel-after-DMA matches the V0 contract on H-002, H-004 and H-005.
- [ ] GPUDispatch cancel-after-submit matches HET-012 on H-002, H-004 and H-005.
- [ ] Wi-Fi send cancel-after-issue matches the committed-work contract on H-004 and H-005.
- [ ] Each class records the caller-visible result (`Cancelled`, wait, or partial) in CI logs.

#### Verification
- Integration: `kernel:tests/tsk/cancel_matrix_*` on `hw-h002`, `hw-h004` and `hw-h005`.
- Manual: one GPU and one Wi-Fi cancel-after-submit on H-004, procedure on the pull request.

#### Evidence
- none

### TSK-049 · Implement GPUDispatch Operation kind with committed-work cancellation
- Type: build
- Milestone: V2
- Status: todo
- Size: L
- Owner: none
- Depends on: TSK-018, TSK-010, TSK-017, ABI-014, HET-019, HET-016, HET-003
- Baseline: §18, §37, §19
- Benchmarks: B-048

§18 lists GPUDispatch. The V2 ComputeDevice demo dispatches to CPU and GPU. TSK owns the Operation kind, completion and committed-work cancellation; HET owns ComputeDevice semantics and the GPU-signal path. Native software never sees a Vulkan or DRM queue ABI.

<!-- covers: INV-0354 -->

#### Out of scope
ComputeDevice object (HET). Vulkan/DRM backend (HET-003). RenderQueue (GFX). Hardware cancel matrix (TSK-048).

#### Acceptance criteria
- [ ] A GPUDispatch Operation submits and completes when the GPU signals, on `hw-h002`.
- [ ] Cancel after GPU submit follows the committed-work contract from the NVMe spike as specialised by HET-012.
- [ ] Submitting GPUDispatch without a ComputeDevice Capability returns `Error::Rights` and allocates no Operation.
- [ ] Native crates have no Vulkan or DRM submission ABI.
- [ ] B-048 publish runs on H-002.

#### Verification
- Unit: `kernel:tests/tsk/kind_gpu_dispatch_*` on `qemu-x86_64` and `hw-h002`.
- Integration: V2 Throughput-on-GPU demo dispatch on `hw-h002`.
- Bench: B-048 on H-002; target per register.

#### Evidence
- none

### TSK-050 · Write Layer 1 reference pages for every Operation kind, result and completion record
- Type: docs
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: TSK-042, TSK-024, TSK-031, TSK-032, TSK-044, TSK-049
- Baseline: §18, §19, §56.5, §65

V3 exit requires Layer 1 reference documentation for every entry point. DOC generates pages from IDL; TSK authors Operation semantics, result codes, deadline representation and cancellation observability text for every kind. Required by V3-G12 (Layer 1 ABI reference pages exist for every entry point): Operation kinds, results and completion records are Layer 1 entry points.

#### Out of scope
IDL-to-docs pipeline (DOC). ABI specification (ABI-046). Channel pages (IPC).

#### Acceptance criteria
- [ ] Every Operation kind has a reference page covering submit, completion, cancel, deadline and typed results.
- [ ] Completion-record layout, inline-completion signalling and deadline representation are documented.
- [ ] DOC lead records that TSK-authored prose is wired into the generated Layer 1 set.
- [ ] A CI check fails if a registered Operation kind lacks a page.

#### Verification
- Review: DOC lead and ABI lead sign-off recorded on the pull request.
- Integration: docs CI kind-to-page completeness check.

#### Evidence
- none

### TSK-051 · Build continuous fuzzing harness for Operation submission and completion
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: TSK-024, TSK-030, TSK-040, BLD-016, BLD-035
- Baseline: §18, §19, §51
- Risks: R-051

V3 exit: continuous syscall and IPC fuzzing. TSK supplies the grammar and oracle for submit, batch, link, cancel and deadline paths on BLD's fuzzing infrastructure. Oracles include: a cancelled Operation never delivers a successful result; forged completions are rejected; outstanding-work limits return typed exhaustion.

<!-- covers: GAP-0127 -->

#### Out of scope
Fuzz infrastructure (BLD). Channel fuzz targets (IPC-044). MemoryObject oracles (MEM).

#### Acceptance criteria
- [ ] A syzkaller (or successor) grammar covers submit, batch, link, cancel, deadline, Read, Write, Send, Receive, Timer, Wait, Connect, Accept, DeviceOperation, StorageTransaction and GPUDispatch.
- [ ] Oracles fail the run if a cancelled Operation delivers a successful result or a forged completion is accepted.
- [ ] The harness runs on BLD continuous fuzzing and files crashes into the tracker.
- [ ] No known open TSK-owned crasher older than the V3 register window remains at gate time.

#### Verification
- Fuzz: TSK Operation grammar on BLD continuous fuzzing; crasher-age report attached to the pull request.
- Review: BLD lead confirms the grammar is wired into the V3 crasher-age gate.

#### Evidence
- none

### TSK-052 · Build Layer 1 conformance suite for the frozen Operation ABI
- Type: build
- Milestone: V4
- Status: todo
- Size: L
- Owner: none
- Depends on: TSK-042, TSK-050, TSK-051, TSK-048, ABI-049
- Baseline: §65, §66
- Freezes: S-005, S-008
- Invariants: I-040

V4 exit: Layer 1 frozen with a conformance suite. This suite covers every Operation kind, inline-completion signalling, deadline representation and cancellation results named by decide-l1-freeze-candidates. Binaries built against the freeze candidate run on every subsequent beta build. This task carries the S-005 and S-008 freeze once ABI-049 is accepted.

#### Out of scope
Freeze ADR (ABI-049). Channel conformance (IPC-065). Component conformance (CMP).

#### Acceptance criteria
- [ ] Every freeze-candidate Operation kind has a conformance test for submit, complete, cancel and deadline.
- [ ] Inline-completion signalling and deadline representation match the V1 candidate decision.
- [ ] A binary built against the freeze candidate passes the suite on a later V4 beta image on H-002.
- [ ] The suite is a required CI job on every V4 RC.

#### Verification
- Integration: `kernel:tests/tsk/conformance_*` on every V4 hardware-scope machine named in `milestones/V4.md`.
- Compat: ABI-047 run including TSK tests on H-002.

#### Evidence
- none

### TSK-053 · Audit Operation surfaces for the 1.0 ABI stability statement
- Type: docs
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: TSK-052, ABI-053
- Baseline: §43, §57, §65, §66
- Invariants: I-047, I-059

1.0 definition: confirm every Operation kind and result code is in the frozen Layer 1 statement or a versioned Layer 2 interface with deprecation policy, and list async non-promises (no distributed Operations; distribution is not a kernel concern, I-047).

#### Out of scope
ABI stability declaration (ABI-053). Remote transport (IPC, LATER). Fossilization review of ComputeDevice (HET).

#### Acceptance criteria
- [ ] An audit table lists every Operation kind and result code as Layer 1 frozen or Layer 2 versioned with a deprecation policy.
- [ ] The audit lists async non-promises, including no distributed Operations in the kernel.
- [ ] ABI lead records that the table is attached to the 1.0 stability statement.

#### Verification
- Review: ABI lead sign-off recorded on the pull request.

#### Evidence
- none
