# TSK · Tasks, operations, structured concurrency
- Prefix: TSK
- Lead: none
- Baseline: §18, §19, §20, §21

<!-- roadmap:generated:begin summary -->
Tasks: 53 live, 1 done, 0 in-progress, 52 todo, 0 dropped. Ready: 4. Blocked: 48. Weighted: 1%.
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
- Depends on: TSK-006, TSK-018, BLD-082
- Baseline: §18, §57, §65, §67
- Invariants: I-018, I-030

Native kernel APIs are asynchronous by default, language runtimes bind to kernel Operations, and signals are not a native notification mechanism (§18, Principle 5 of §67, D-0309). These standing rules become one lint and one checklist section rather than a task per rule. `tools/async-lint/` (crate `jakeos-tools-async-lint`) scans the ABI specification (`abi/spec/`) and native crates' public items: an entry point or public function whose documented primary mode blocks the calling execution context (any name in `build/lints/blocking-shapes.toml`: `read`, `write`, `recv`, `send`, `accept`, `wait` and `join` variants that return the result directly rather than an Operation or future, `sleep`, `usleep`, `select`, `poll`, `epoll_wait`), a thread-per-call I/O surface (`spawn_blocking` style wrappers exported publicly), signal-style notification (`signal`, `sigaction`, `kill`, `SIGUSR` symbols), and an async runtime whose reactor does not submit kernel Operations (a crate depending on `mio`, `tokio` or `async-std` with a Linux backend, detected through `cargo metadata`) each fail the `async-lint` job in `pre-merge`. The one permitted blocking shape is explicit wait-for-completion on an Operation (`operation.wait`), named in the allowlist.

The reviewer half is a section of the ABI-006 checklist: an API whose only mode is synchronous, or a notification that interrupts running code, is rejected with a pointer to D-0309.

<!-- covers: INV-0340, INV-0341, INV-0039, INV-1296, INV-0038 -->

#### Out of scope
Personality syscall retention (LNX). POSIX-shaped names on native crates (ABI-018). Process-shape lint (CMP-013). The runtime executor (SDK-004).

#### Deliverables
- tools:async-lint/ · Crate `jakeos-tools-async-lint`: spec scan, public-item scan, dependency scan, allowlist lookup.
- bld:lints/blocking-shapes.toml · Forbidden blocking, thread-per-call and signal shapes; the `operation.wait` allowlist entry.
- platform:.github/workflows/pre-merge.yml · The `async-lint` job over native crates and `abi/spec`.
- kernel:.github/workflows/pre-merge.yml · The `async-lint` job over `include/uapi/linux/jakeos/`.
- abi:review/checklist.md · The async-by-default reviewer items (extending ABI-006's file).
- tools:async-lint/fixtures/ · A crate exporting a blocking `read`, a crate using `sigaction`, a crate whose reactor is `mio`.

#### Acceptance criteria
- [ ] The `async-lint` job fails a native crate that exposes an entry point or public function whose primary mode blocks the calling execution context, except `operation.wait`.
- [ ] The job fails a native crate that exposes a blocking read/write thread-per-call I/O surface or uses signal-style notification symbols.
- [ ] The job fails a native crate whose async runtime reactor does not submit kernel Operations (a Linux-backend `mio`, `tokio` or `async-std` dependency), and passes the SDK runtime (SDK-004) that does.
- [ ] The job is wired into `pre-merge` of both repositories on matrix entry `qemu-x86_64`, and the three fixtures fail it.

#### Verification
- Unit: `kernel:tests/tsk/async_lint_*` on CI matrix entry `qemu-x86_64` over the fixtures.
- Review: ABI lead sign-off recorded on the pull request that lands the checklist section.

#### Evidence
- none

### TSK-002 · Benchmark native Task handoff against Linux thread switch and publish
- Type: benchmark
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: TSK-020, BEN-007, BEN-005, BLD-082
- Baseline: §20, §54, §59
- Benchmarks: B-003
- Risks: R-009

Context switching is what the native Task model claims to make cheap (§20, §54); B-003 publishes native Task handoff against Linux thread and process switch on the same hardware. The harness `bench/harness/B-003/` (crate `jakeos-bench-task-switch`, scenario `task-switch`) runs a ping-pong of two Tasks in one Component alternating through Wait Operations (TSK-020's wake path), same core and cross core, and in the same session runs a Linux `futex` ping-pong between two threads and between two processes on the personality side of the same kernel; per BEN-064 it pins CPUs, records mitigation state, and emits BEN-005 records for each. V0's target kind is `publish`; TSK-046 tunes the multiplexer at V1 against the absolute target.

Reports go to `reports/benchmarks/B-003/h001.md` and `h002.md`; no public material states a superiority claim without citing them (I-061).

#### Out of scope
Task creation latency (B-002, BEN-001). IPC round trip (B-004, IPC-008). V1 multiplexer tuning (TSK-046).

#### Deliverables
- bench:harness/B-003/ · Crate `jakeos-bench-task-switch`: `task-switch-same-core`, `task-switch-cross-core`, `baseline-thread-futex`, `baseline-process-futex` scenarios.
- bench:harness/B-003/README.md · How the ping-pong is constructed and how baselines run in the same session.
- roadmap:reports/benchmarks/B-003/h001.md · The H-001 report (labelled QEMU).
- roadmap:reports/benchmarks/B-003/h002.md · The H-002 report V0-G16 cites.

#### Acceptance criteria
- [ ] `reports/benchmarks/B-003/h001.md` and `h002.md` exist with same-core and cross-core native Task handoff p50 and p99 and meet the V0 `publish` target kind.
- [ ] Each report names the Linux thread-switch and process-switch `futex` baselines run in the same session on the same machine, with mitigation state recorded.
- [ ] The harness emits a BEN-005 record per scenario from the BLD-010 nightly job on both matrix entries.
- [ ] No public material states a superiority claim without citing those reports (I-061).

#### Verification
- Bench: B-003 on H-001 and H-002; target per register.
- Unit: `bench:tests/B-003/scenario_*` asserting record shape and that baselines and native runs share a session.
- Review: BEN lead confirms the reports follow the accepted methodology.

#### Evidence
- none

### TSK-003 · Decide Task cancellation model and resource cleanup
- Type: adr
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: TSK-017
- Baseline: §19, §21, §58
- Decision: D-0305
- Invariants: I-018

V0 exit cancels a TaskGroup and never delivers a result from a cancelled Operation (§19, §21). D-0305 picks the cancellation model native software observes: cooperative cancellation at await points (a cancelled Task runs to its next await and then unwinds its own frames), forced kernel teardown (the kernel terminates the Task's execution context and reclaims what it held), or staged cancellation with a grace deadline (cooperative first, forced after the deadline). Each option states the cleanup of Capabilities, MemoryObjects and partial ownership a cancelled Task holds, and the observable result when a Task sits in an uninterruptible retained Linux path (an NVMe DMA, per TSK-017's report): whether the caller waits, fails, or gets a best-effort completion. It answers Q-011 and Q-012, studies Trio nurseries, Kotlin coroutine scopes and Swift task groups in the decision's Context rather than a separate spike, and rejects at least one signal-like option explicitly (D-0309).

The executing agent writes the options with those consequences, cites `reports/spikes/TSK-017.md`, records the Decision as the model plus the cleanup and uninterruptible-path rules, and marks Q-011 and Q-012 answered.

<!-- covers: INV-0396, INV-0394, INV-0395, INV-1149 -->

#### Out of scope
Committed-hardware state machine implementation (TSK-010). Background-execution Capability exception (TSK-025). Personality thread mapping (TSK-043). Cancellation propagation (TSK-022).

#### Deliverables
- roadmap:decisions/D-0305-decide-cancellation-model.md · Options with cleanup and uninterruptible-path consequences, the Trio, Kotlin and Swift study in Context, Evidence citing `reports/spikes/TSK-017.md`, the Decision, rejected options including a signal-like one, follow-ups.
- roadmap:registers/questions.md · Q-011 and Q-012 `Status: answered`.

#### Acceptance criteria
- [ ] D-0305 evaluates cooperative cancellation at await points, forced kernel teardown, and staged cancellation with a grace deadline as named options.
- [ ] Each option states the cleanup of Capabilities, MemoryObjects and partial ownership, and the observable result of a Task stuck in an uninterruptible retained Linux sleep, citing `reports/spikes/TSK-017.md`.
- [ ] The Decision records at least one rejected signal-like option with the D-0309 reason, and Q-011 and Q-012 are marked answered by TSK-003.
- [ ] Review records ABI lead and TSK lead sign-off on the pull request.

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

Every Operation carries a deadline (§18, §19), so its clock domain, resolution, overflow horizon and the provisional suspend and resume rule are part of the Layer 1 Operation ABI on S-005 while the surface stays prototyped (§65). D-0306 chooses among a monotonic clock that pauses in suspend, a boot-time clock that advances through it, and a wall clock, each with a stated resolution (nanoseconds) and horizon (a 64-bit count), states what a Timer Operation observes across suspend and resume under each, and names the ABI fields that carry deadline and timestamp (the `deadline` field of the submission record and the `completed_at` field of the completion record, both `u64` nanoseconds in the chosen domain). TSK-015's report on in-kernel deadline enforcement is the evidence; SVC-016 decides the laptop suspend semantics at V1 on top of this representation.

<!-- covers: GAP-0496 -->

#### Out of scope
Timer kind implementation (TSK-012). Laptop suspend cycles (TSK-041, PWR). Slack and coalescing (TSK-047). Clock semantics across suspend (SVC-016).

#### Deliverables
- roadmap:decisions/D-0306-decide-deadline-representation.md · Options with suspend consequences, Evidence citing `reports/spikes/TSK-015.md`, the Decision naming the clock, resolution, horizon and the two ABI fields, rejected options, follow-ups.

#### Acceptance criteria
- [ ] D-0306 evaluates a monotonic clock that does not advance during suspend, a boot-time clock that does, and a wall clock, each with a stated resolution and overflow horizon.
- [ ] Each option states what a Timer Operation observes across suspend and resume.
- [ ] The Decision names the ABI fields that carry deadline and timestamp, their type and unit, cites `reports/spikes/TSK-015.md`, and records that S-005 stays `prototyped`.
- [ ] Review records ABI lead sign-off on the pull request.

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

A cached read or an already-signalled Wait can be finished before `submit` returns (§18, §19). Whether that is allowed, and how the caller learns that completion was inline rather than delivered later, is a Layer 1 choice on S-005 that must be fixed before TSK-018 builds the submit path. D-0307 chooses among never completing inline (every completion goes through the completion transport), completing inline with an ABI-visible flag in the completion record, and completing inline with a distinct `submit` return code, and states for each how a caller distinguishes inline from later completion, including the already-signalled Wait case, and what it means for the TSK-007 transport's ring accounting. TSK-014's report on transport prototypes is the evidence.

<!-- covers: INV-0345 -->

#### Out of scope
Submit and completion implementation (TSK-018). Transport choice (TSK-007). Wait kind (TSK-012).

#### Deliverables
- roadmap:decisions/D-0307-decide-inline-completion.md · Options with transport consequences, Evidence citing `reports/spikes/TSK-014.md`, the Decision as the rule and the signalling mechanism, rejected options, follow-ups.

#### Acceptance criteria
- [ ] D-0307 evaluates never completing inline, completing inline with an ABI-visible flag, and completing inline with a distinct submit return code as named options.
- [ ] Each option states how a caller distinguishes inline completion from a later completion record, including the already-signalled Wait case, and its effect on ring accounting.
- [ ] The Decision cites `reports/spikes/TSK-014.md` and records that S-005 stays `prototyped`.
- [ ] Review records ABI lead sign-off on the pull request.

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

Operation completion is the hinge between kernel scheduling and the runtime and cannot change after SDK code depends on it (§18, §19). D-0311 picks the transport from TSK-014's prototypes: shared submission and completion rings, per-Component kernel queues, syscall-per-Operation, or a hybrid; whether io_uring internals are reused (the ring structures and `io_uring_enter` lineage) or replaced; and how a batch of submissions is expressed (contiguous ring entries, a linked list, or one entry per call). Each option states how completion reaches the submitting Task and cites the wake-up measurements from `reports/spikes/TSK-014.md` and the B-009 records. S-005 stays prototyped; TSK-018 implements the result and TSK-024 verifies it.

<!-- covers: INV-0344, GAP-0494 -->

#### Out of scope
Submit path implementation (TSK-018). Linked chains (TSK-030). Ring hardening (TSK-040). Entry mechanism (ABI-008).

#### Deliverables
- roadmap:decisions/D-0311-decide-operation-transport.md · Options with io_uring reuse and batching consequences, Evidence citing `reports/spikes/TSK-014.md` and B-009, the Decision as transport, reuse rule and batch expression, rejected options, follow-ups.

#### Acceptance criteria
- [ ] D-0311 evaluates shared rings, per-Component queues, syscall-per-Operation, and a hybrid, each stating whether io_uring internals are reused or replaced.
- [ ] Each option states how a batch of submissions is expressed and how completion is delivered to the submitting Task.
- [ ] The Decision cites the spike report's wake-up measurements through B-009 and records that S-005 stays `prototyped`.
- [ ] Review records ABI lead sign-off on the pull request.

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

Orthogonal to how Tasks are multiplexed (TSK-009) is whether every Task has an identity the kernel knows (§20). D-0314 chooses among every Task as `Object<Task>` with a handle, a runtime identity with kernel visibility only for observability and cancellation (a compact identity table the kernel reads plus a cancellation doorbell), and a hybrid that promotes a Task to a kernel object when it needs kernel-visible cancellation or a distinct intent. Each option states how cancellation and `os inspect task` name a Task and confirms the ABI definition is architecture-neutral with no x86 execution-context assumption (§38, I-057). TSK-016's report against the B-014 live-Task scale is the evidence. S-008 stays prototyped; TSK-021 implements the result.

<!-- covers: INV-0381, INV-0721 -->

#### Out of scope
Multiplexing model (TSK-009). Task implementation (TSK-021). Inspect rendering (OBS-005).

#### Deliverables
- roadmap:decisions/D-0314-decide-task-identity.md · Options with cancellation and inspect consequences, Evidence citing `reports/spikes/TSK-016.md`, the Decision, rejected options, follow-ups.
- roadmap:registers/surfaces.md · S-008 `Decided by` adds TSK-008, `State: prototyped`.

#### Acceptance criteria
- [ ] D-0314 evaluates every Task as `Object<Task>`, runtime identity with kernel visibility only for observability and cancellation, and a hybrid as named options.
- [ ] Each option states how cancellation and `os inspect task` name a Task and records that the ABI definition is architecture-neutral (I-057).
- [ ] The Decision cites `reports/spikes/TSK-016.md` and records that S-008 stays `prototyped` with TSK-008 under `Decided by`.
- [ ] Review records ABI lead sign-off on the pull request.

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

Replacing threads with Task and TaskGroup (§2, §20) requires fixing the kernel and runtime split and how hidden blocking is compensated (§21). D-0315 chooses among kernel-managed Tasks (one kernel execution context per Task), UMCG-style activations with compensating workers (a bounded worker set with the kernel reporting blocked workers), and a pure user-space runtime over async syscalls only; each option states the split, how a page fault or a synchronous retained driver path is detected and compensated so a stalled worker does not starve multiplexed Tasks, and its viability at the B-014 live-Task scale from `reports/spikes/TSK-016.md`. S-008 stays prototyped; TSK-019 implements the result and SDK-004 builds the runtime side.

<!-- covers: INV-0071, INV-0380, INV-0038, GAP-0493 -->

#### Out of scope
Task object implementation (TSK-021). Userspace runtime (SDK-004). Personality threads (TSK-043). The multiplexer (TSK-019).

#### Deliverables
- roadmap:decisions/D-0315-decide-task-mapping.md · Options with the split and compensation consequences, Evidence citing `reports/spikes/TSK-016.md` and B-014, the Decision, rejected options, follow-ups.
- roadmap:registers/surfaces.md · S-008 `Decided by` adds TSK-009.

#### Acceptance criteria
- [ ] D-0315 evaluates kernel-managed Tasks, UMCG-style activations with compensating workers, and a pure userspace runtime over async syscalls only as named options.
- [ ] Each option states the kernel and runtime split and how page faults and synchronous retained driver paths are detected and compensated.
- [ ] The Decision cites `reports/spikes/TSK-016.md` against the B-014 live-Task scale and records that S-008 stays `prototyped`.
- [ ] Review records ABI lead and TSK lead sign-off on the pull request.

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

Cancellation and deadline expiry are completions, not interruptions (§19, §21, D-0309): `jakeos/tsk/cancel.rs` implements `operation.cancel(id)` for the owner (identity per D-0015), which marks the Operation and drives the kind's cancel hook; `jakeos/tsk/deadline.rs` implements deadline enforcement in the representation D-0306 chose (a timer wheel or hrtimer per Operation, per TSK-015's finding) and expires Operations whose deadline passed. Both deliver through the normal completion path (TSK-018) as the typed results `Error::Cancelled` and `Error::DeadlineExceeded` (D-0006), and a cancelled or expired Operation never delivers a successful result afterwards. Cancelling a TaskGroup (TSK-022) calls `cancel` on every Operation it owns.

For hardware-committed work, `jakeos/tsk/committed.rs` implements the state machine TSK-017 reported for the NVMe path on `hw-h002` (`Submitted`, `Committed`, `Completing`, with the cancel-after-commit result the D-0305 model chose: wait, fail or best-effort, and the partial-result shape) so a cancel after DMA start behaves as decided and never reports success. `os inspect operation <id>` shows owner, deadline and cancellation state through the OBS-007 provider.

<!-- covers: INV-0362, INV-0364, INV-0374, GAP-0495 -->

#### Out of scope
TaskGroup hierarchy walk (TSK-022). GPU and Wi-Fi committed-work matrix (TSK-048). Slack coalescing (TSK-047). Transport (TSK-018).

#### Deliverables
- kernel:jakeos/tsk/cancel.rs · `operation.cancel` handler, owner check, kind cancel hooks, `Error::Cancelled` completion.
- kernel:jakeos/tsk/deadline.rs · Deadline enforcement structure per TSK-015 and `Error::DeadlineExceeded` completion.
- kernel:jakeos/tsk/committed.rs · The committed-work state machine for the NVMe path.
- kernel:tools/jakeos/fuzz/tsk_cancel_deadline/ · Fuzz target over cancel and deadline races (`kernel:fuzz/tsk_cancel_deadline`).
- kernel:tools/testing/selftests/jakeos/tsk/cancel_*.rs · Selftests: owner cancel, non-owner refusal, cancel-after-completion race, TaskGroup cancel fan-out.
- kernel:tools/testing/selftests/jakeos/tsk/deadline_*.rs · Selftests: absolute and relative expiry, expiry during submission, no late success.
- kernel:tools/testing/selftests/jakeos/tsk/nvme_committed_*.sh · The `hw-h002` NVMe committed-work scenario.

#### Acceptance criteria
- [ ] Cancelling an Operation by its owner completes it with `Error::Cancelled` and never delivers a successful result afterwards, on `qemu-x86_64` and `hw-h002`; a cancel by a non-owner returns `Error::Rights`.
- [ ] An Operation whose deadline has passed completes with `Error::DeadlineExceeded` through the normal completion path, including one whose deadline passes between submission and enqueue.
- [ ] Cancelling a TaskGroup cancels every outstanding Operation it owns (asserted by counting completions).
- [ ] An in-flight NVMe Read on `hw-h002` cancelled after DMA start follows the D-0305 committed-work rule and reports the decided partial-result shape, never a plain success.
- [ ] `os inspect operation <id>` on an in-flight Operation shows owner, deadline and cancellation state.

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

The four data-moving kinds of §18 are implemented as kind handlers under `jakeos/tsk/kinds/`, each registered in the D-0014 kind table (`jakeos/abi/kinds.rs`) with the right it requires (CAP-011): `read.rs` and `write.rs` operate on a target object that implements the kernel `Readable` or `Writable` trait (`jakeos/tsk/kinds/traits.rs`) with a MemoryObject-backed buffer described by `(Capability<MemoryObject>, offset, length)`, and V0 targets are the STO-001 File object, a Channel endpoint (byte mode is not offered; Read and Write on a Channel are refused with the typed error so that Send and Receive remain the only Channel kinds) and MemoryObject-to-MemoryObject copies; `send.rs` and `receive.rs` are the typed `Channel<T>` kinds over IPC-010's queue, carrying a message described by `(Capability<MemoryObject>, length, handle slots)` per IPC-007's wire format. Submitting any of the four returns without waiting; completion carries a typed result (bytes moved, or the message descriptor) or a typed failure.

No blocking read or write thread-per-call surface exists (TSK-001 enforces it); the V0 demo (CMP-011) uses Send and Receive.

<!-- covers: INV-0346, INV-0347, INV-0348, INV-0349 -->

#### Out of scope
Channel object and backpressure (IPC-009, IPC-010). File object (STO-001). MemoryObject map (MEM-007). Connect and Accept (TSK-032). Transport (TSK-018).

#### Deliverables
- kernel:jakeos/tsk/kinds/traits.rs · `Readable` and `Writable` kernel traits and the buffer descriptor type.
- kernel:jakeos/tsk/kinds/read.rs · Read handler over `Readable` targets.
- kernel:jakeos/tsk/kinds/write.rs · Write handler over `Writable` targets.
- kernel:jakeos/tsk/kinds/send.rs · Send handler over the IPC-010 queue.
- kernel:jakeos/tsk/kinds/receive.rs · Receive handler over the IPC-010 queue.
- kernel:jakeos/abi/kinds.rs · The four kinds registered with their required rights (extending ABI-002's file).
- kernel:tools/testing/selftests/jakeos/tsk/kind_read_*.rs · Selftests for Read against File and MemoryObject targets and the Channel refusal.
- kernel:tools/testing/selftests/jakeos/tsk/kind_write_*.rs · Selftests for Write.
- kernel:tools/testing/selftests/jakeos/tsk/kind_send_*.rs · Selftests for Send.
- kernel:tools/testing/selftests/jakeos/tsk/kind_recv_*.rs · Selftests for Receive.

#### Acceptance criteria
- [ ] A Read against a File and against a MemoryObject-backed source completes with a typed result (bytes moved) on `qemu-x86_64` and `hw-h002`; a Read against a Channel endpoint is refused with the typed error.
- [ ] A Write against a writable File or MemoryObject completes with a typed result on both matrix entries; a Write without the `Write` right returns `Error::Rights` and moves nothing.
- [ ] Send and Receive on `Channel<T>` complete with the typed message descriptor on both matrix entries, and a Receive on an empty Channel stays outstanding until a Send arrives.
- [ ] Submitting Read, Write, Send or Receive returns without waiting for completion, and the V0 demo (CMP-011) uses Send and Receive for its request and reply.

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

Timer and Wait are the native replacements for `timerfd` and signal-style waiting (§18, §19, D-0309). `jakeos/tsk/kinds/timer.rs` completes at or after an absolute deadline, or a relative one converted at submission, in the D-0306 representation, using the TSK-010 deadline structure so a Timer is an Operation whose deadline is its purpose. `jakeos/tsk/kinds/wait.rs` completes when a referenced waitable object (an `Event` from TSK-029 later, a Component's exit, another Operation's completion, or a Channel's readability at V0) reaches a signalled state; a Wait on a set of objects completes on the first to signal and reports which. A Wait on an already-signalled object follows D-0307 (inline completion or not). Both kinds are registered in the kind table with the `Wait` right on the referenced object.

<!-- covers: INV-0352, INV-0353 -->

#### Out of scope
User-signalled Event object (TSK-029). Slack and coalescing (TSK-047). Deadline overhead harness (TSK-039). Deadline enforcement itself (TSK-010).

#### Deliverables
- kernel:jakeos/tsk/kinds/timer.rs · Timer handler over the deadline structure, absolute and relative forms.
- kernel:jakeos/tsk/kinds/wait.rs · Wait handler over waitable objects and sets, with the D-0307 already-signalled behaviour.
- kernel:jakeos/tsk/kinds/waitable.rs · The kernel `Waitable` trait implemented by Component, Operation and Channel at V0.
- kernel:tools/testing/selftests/jakeos/tsk/kind_timer_*.rs · Selftests: absolute, relative, past deadline, cancel before expiry.
- kernel:tools/testing/selftests/jakeos/tsk/kind_wait_*.rs · Selftests: single object, set, already signalled, cancel while waiting.

#### Acceptance criteria
- [ ] A Timer with an absolute deadline completes at or after that deadline with a typed result on `qemu-x86_64` and `hw-h002`, and never before it.
- [ ] A Timer with a relative deadline completes at or after submission time plus the relative amount on both matrix entries.
- [ ] A Wait on a Component completes when it exits, a Wait on an Operation completes when it completes, and a Wait on a set completes on the first signalled member and reports which.
- [ ] A Wait on an already-signalled object follows D-0307's inline-completion rule exactly (asserted by the flag or return code the decision names).

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

`Operation<Result>` is the first-class unit of outstanding asynchronous work (§19, §69). `jakeos/tsk/operation.rs` registers the `Operation` type id with the ABI-005 registry and defines the object: owner (a Task or TaskGroup handle, so cancellation is structural), kind (D-0014), typed result or typed failure slot (D-0006 encoding), deadline and `completed_at` (D-0306 fields), a priority field carried for later ordering (TSK-027 decides its relation to intent), the trace identity OBS consumes (a span id allocated at creation and emitted on the S-010 substrate at submit, complete and cancel), and the state (`Created`, `Submitted`, `Completed`, `Cancelled`). User space names an Operation as D-0015 (ABI-015) chose; whichever it is, user space holds a Capability or index, never the object. Destroying the owning TaskGroup without the cancel path still reclaims every Operation it owns, which the CMP-004 leak test covers.

`unsafe` is confined to the files the pull request names; the OBS-007 provider reads the object for `os inspect operation`.

<!-- covers: INV-0054, INV-0361, INV-0363, INV-0365, INV-0367, INV-1317 -->

#### Out of scope
Submit and completion transport (TSK-018). Priority ordering of I/O (TSK-033). Inspect rendering (OBS-007). Kind handlers (TSK-011, TSK-012).

#### Deliverables
- kernel:jakeos/tsk/tsk.rs · Crate root for the TSK area.
- kernel:jakeos/tsk/operation.rs · `Object<Operation>`: header, owner, kind, result slot, deadline fields, priority, trace identity, state machine.
- kernel:jakeos/tsk/trace.rs · Span emission at submit, complete and cancel on the S-010 substrate.
- kernel:jakeos/cap/rights_decl.rs · The `Operation` rights vocabulary (`Cancel`, `Wait`, `Inspect`) (extending CAP-011's file).
- kernel:tools/testing/selftests/jakeos/tsk/operation_object_*.rs · Selftests: creation records, result delivery, identity per D-0015, reclamation without cancel.
- kernel:Documentation/jakeos/tsk/operation.md · The object's fields, states and what each consumer (transport, cancel, OBS) reads.

#### Acceptance criteria
- [ ] User space holds a Capability to or index of an Operation per D-0015, never the object; a forged reference returns `Error::Rights` and allocates nothing.
- [ ] Creating an Operation records owner, kind, deadline, priority and a trace span id, all inspectable through the OBS-007 provider hooks, and a span is emitted at submit, complete and cancel.
- [ ] Completing an Operation delivers a typed result or a typed failure in the D-0006 encoding and no other payload.
- [ ] Destroying the owning TaskGroup without the cancel path reclaims every Operation it owns with no kernel-memory leak in the CMP-004 leak test.
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

TSK-007 must choose the Operation transport from running code (§18, §65). This spike first studies io_uring's submission and completion rings, linked operations and cancellation at the merged upstream tag (§58), then prototypes three transports under `jakeos/spikes/tsk/transport/` behind `CONFIG_JAKEOS_SPIKES`: `ring.rs` (a shared submission and completion ring per process in a mapped page, entered through the ABI-019 syscall prototype), `waitobj.rs` (per-Task wait objects with a kernel queue and a syscall per submission), and `hybrid.rs` (a ring for submission with a wait object for completion delivery). The driver `runtime/spikes/tsk-transport-driver/` submits no-op Operations singly and in batches of 8 and 64 and measures submit-to-completion and wake-up latency under the B-009 method on `qemu-x86_64` and `hw-h002`, with io_uring `NOP` submit-to-completion as the baseline in the same session.

The report `reports/spikes/TSK-014.md` records wake-up latency per prototype and batch size, states whether io_uring's internal structures can be reused (and which) or must be replaced (and why), describes how a batch is expressed in each, and lists what stays undecided. Nothing on S-005 is frozen.

<!-- covers: INV-1144, GAP-0494 -->

#### Out of scope
The transport decision (TSK-007). Permanent harness (TSK-026). The entry mechanism (ABI-019).

#### Deliverables
- kernel:jakeos/spikes/tsk/transport/ring.rs · Shared ring prototype.
- kernel:jakeos/spikes/tsk/transport/waitobj.rs · Per-Task wait-object prototype.
- kernel:jakeos/spikes/tsk/transport/hybrid.rs · Hybrid prototype.
- runtime:spikes/tsk-transport-driver/ · Submission driver with batch sizes, B-009 method, io_uring `NOP` baseline.
- roadmap:reports/spikes/TSK-014.md · The report.

#### Acceptance criteria
- [ ] `reports/spikes/TSK-014.md` describes shared-ring, per-Task wait-object and hybrid prototypes that ran on `qemu-x86_64` and `hw-h002` and completed no-op Operations singly and in batches of 8 and 64.
- [ ] The report records submit-to-completion and wake-up latency for each prototype and batch size under the B-009 method, with the io_uring `NOP` baseline from the same session, all labelled unpublished prototype measurements.
- [ ] The report states which io_uring internals can be reused and which must be replaced, with the reason for each.
- [ ] The report lists what is not decided and does not freeze S-005.

#### Verification
- Report: which transport meets the V0 demo, how batches would be expressed, and whether io_uring internals are reusable; path `reports/spikes/TSK-014.md`.
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

Every Operation carries a deadline (§19); if enforcing it costs a timer per Operation the submit path pays for it on every call. Under `jakeos/spikes/tsk/deadline/` behind `CONFIG_JAKEOS_SPIKES`, two enforcement structures are prototyped: `wheel.rs` (a hierarchical timer wheel keyed by deadline with lazy cascade) and `hrtimer.rs` (an `hrtimer` armed per Operation with a deadline), plus a `none` path for Operations without a deadline so the spike can show whether the no-deadline case stays off the structure entirely. The driver `runtime/spikes/tsk-deadline-driver/` submits no-op Operations at increasing rates with and without deadlines and measures submit-to-completion overhead under the B-009 method on `qemu-x86_64` and `hw-h002`, plus the accuracy of expiry (how late a `DeadlineExceeded` fires).

The report `reports/spikes/TSK-015.md` feeds TSK-004 (representation) and TSK-010 (implementation) and the later standing harness TSK-039. S-005 is not frozen.

<!-- covers: INV-0376 -->

#### Out of scope
Deadline representation decision (TSK-004). Permanent harness (TSK-039). Timer kind (TSK-012). Enforcement implementation (TSK-010).

#### Deliverables
- kernel:jakeos/spikes/tsk/deadline/wheel.rs · Timer-wheel prototype.
- kernel:jakeos/spikes/tsk/deadline/hrtimer.rs · hrtimer-per-Operation prototype.
- runtime:spikes/tsk-deadline-driver/ · Rate sweep with and without deadlines, expiry-accuracy measurement.
- roadmap:reports/spikes/TSK-015.md · The report.

#### Acceptance criteria
- [ ] `reports/spikes/TSK-015.md` describes timer-wheel and hrtimer-per-Operation prototypes that ran on `qemu-x86_64` and `hw-h002` across the submission-rate sweep.
- [ ] The report records submit-to-completion overhead with and without a deadline for each structure under the B-009 method, and the expiry lateness distribution, all labelled unpublished prototype measurements.
- [ ] The report states whether a no-deadline Operation stays off the enforcement data structure in each prototype.
- [ ] The report does not freeze S-005.

#### Verification
- Report: wheel versus per-Operation timer, overhead at high submission rates, expiry accuracy, and the no-deadline fast path; path `reports/spikes/TSK-015.md`.
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

TSK-008 and TSK-009 fix how Tasks are identified and mapped (§20, §65); this spike gives them evidence. Under `jakeos/spikes/tsk/mux/` behind `CONFIG_JAKEOS_SPIKES`, three models are prototyped far enough to run the B-014 live-Task population in one process: `umcg.rs` (kernel-notified user scheduling in the UMCG lineage: a bounded worker set, a server thread receiving block and wake notifications, user-space Task switching), `userspace.rs` (a pure user-space runtime over the ABI-019 syscall prototype with no kernel help), and `kernel.rs` (kernel-managed lightweight Tasks, one execution context each). The driver `runtime/spikes/tsk-mux-driver/` creates the B-014 population, measures memory per Task and creation wall time, then injects hidden blocking (a page fault on a mapped-but-unpopulated page, and a synchronous retained driver path: a blocking `read` on a virtio-serial port from a worker) and records how long other Tasks on that worker stall and what compensation each model can apply.

The report `reports/spikes/TSK-016.md` names which model is viable for V0 at B-014 scale, the compensation each achieved, and the costs that remain. S-008 is not frozen.

<!-- covers: GAP-0492, INV-0385, INV-0377, GAP-0493 -->

#### Out of scope
The mapping decision (TSK-009). Identity decision (TSK-008). Multiplexer implementation (TSK-019).

#### Deliverables
- kernel:jakeos/spikes/tsk/mux/umcg.rs · UMCG-style activation prototype.
- kernel:jakeos/spikes/tsk/mux/userspace.rs · Pure user-space runtime prototype's kernel side (async entry only).
- kernel:jakeos/spikes/tsk/mux/kernel.rs · Kernel-managed lightweight Task prototype.
- runtime:spikes/tsk-mux-driver/ · B-014 population, hidden-blocking injection, stall measurement, per-model compensation.
- roadmap:reports/spikes/TSK-016.md · The report.

#### Acceptance criteria
- [ ] `reports/spikes/TSK-016.md` describes the three prototypes running on `qemu-x86_64` and `hw-h002` at the B-014 live-Task scale, with memory per Task and creation wall time labelled unpublished prototype measurements.
- [ ] The report records how a page fault and a synchronous retained driver path stall a worker in each model, how long other Tasks on that worker waited, and the compensation attempted and its effect.
- [ ] The report names which model is viable for V0 and which costs remain for each.
- [ ] The report does not freeze S-008.

#### Verification
- Report: kernel versus runtime split, hidden-blocking compensation, and B-014 viability per model; path `reports/spikes/TSK-016.md`.
- Bench: B-014 method on H-001 and H-002 for each prototype.

#### Evidence
- none

### TSK-017 · Prototype cancellation state machine for hardware-committed Operations on NVMe
- Type: spike
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: none
- Baseline: §19
- Explores: S-005

Uniform cancellation is promised (§19), but once an NVMe command's DMA has been issued the hardware finishes it whatever the caller wants. This spike answers Q-009 with real hardware: under `jakeos/spikes/tsk/committed/` behind `CONFIG_JAKEOS_SPIKES`, a state machine (`Submitted`, `Committed` once the command is in the device queue, `Completing`) is wrapped around a raw NVMe read on `hw-h002`'s NVMe drive using the retained block layer, and the driver `runtime/spikes/tsk-committed-driver/` issues a large read, cancels at controlled points (before queueing, after queueing, during DMA), and records what the caller can be told in each: cancel waits for the hardware and then reports the buffer as consumed; cancel fails with a typed reason; or cancel is best-effort and the completion later reports whether data landed. The partial-result shape (which bytes are valid) is prototyped for the best-effort path.

The report `reports/spikes/TSK-017.md` names the recommended caller-visible contract and the rejected alternatives, and what GPU and network work (TSK-048) must reuse. TSK-003 decides from it; TSK-010 implements it. S-005 is not frozen.

<!-- covers: GAP-0495, INV-0374 -->

#### Out of scope
Kernel implementation of the chosen machine (TSK-010). GPU and Wi-Fi matrix (TSK-048). GPUDispatch (TSK-049). The cancellation model decision (TSK-003).

#### Deliverables
- kernel:jakeos/spikes/tsk/committed/ · The state machine around a raw NVMe read through the retained block layer.
- runtime:spikes/tsk-committed-driver/ · Cancel-at-controlled-point driver and partial-result prototype.
- roadmap:reports/spikes/TSK-017.md · The report, including the manual procedure for the `hw-h002` run.

#### Acceptance criteria
- [ ] `reports/spikes/TSK-017.md` describes an NVMe read issued on `hw-h002` that cannot be aborted after DMA start, with the state machine and the controlled cancel points.
- [ ] The report states the caller-visible result of cancel in each state (wait, fail, or best-effort) and the partial-result shape for the best-effort path.
- [ ] The report names the recommended contract, the rejected alternatives with reasons, and what GPU and network work must reuse.
- [ ] The report does not freeze S-005.

#### Verification
- Report: wait versus fail versus best-effort, partial-result encoding, and what GPU and network work must reuse; path `reports/spikes/TSK-017.md`.
- Manual: NVMe read cancel after DMA start on `hw-h002`, procedure recorded in the report.

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

The transport D-0311 (TSK-007) chose and the inline-completion rule D-0307 (TSK-005) chose become the V0 submit and completion path (§18, §19). `jakeos/tsk/submit.rs` implements `operation.submit` through the ABI-002 entry layer: it validates the submission record (handle word, kind, buffer descriptor, deadline, priority), creates or reuses the Operation object (TSK-013), dispatches to the kind handler and returns without waiting; `jakeos/tsk/ring.rs` (if D-0311 chose rings) holds the per-Component submission and completion ring layout in a MemoryObject the runtime maps, with the memory-ordering rules the decision fixed; `jakeos/tsk/complete.rs` delivers a completion record (Operation identity per D-0015, typed result per D-0006, `completed_at`) to the submitting Task's transport and, when D-0307 allows, completes inline with the decided flag or return code. `operation.poll` (non-blocking read of pending completions) and `operation.wait` (the one blocking entry, awaiting at least one completion or a deadline) are the observation paths.

The runtime side (`runtime/task/src/transport.rs` in `jakeos-runtime-task`) maps the ring and drives `poll` and `wait`; SDK-004's executor sits on it. B-009 no-op submit-to-completion is published from this path.

<!-- covers: INV-0342, INV-0343, INV-1160, INV-0340 -->

#### Out of scope
Kind implementations (TSK-011, TSK-012). Task wake integration (TSK-020). Ring hardening against a hostile Component (TSK-040). The Operation object (TSK-013).

#### Deliverables
- kernel:jakeos/tsk/submit.rs · `operation.submit`: record validation, object creation, kind dispatch, immediate return.
- kernel:jakeos/tsk/ring.rs · Ring layout and ordering rules per D-0311 (present only if rings were chosen).
- kernel:jakeos/tsk/complete.rs · Completion delivery, inline completion per D-0307, `operation.poll` and `operation.wait` handlers.
- runtime:task/src/transport.rs · Runtime mapping of the transport and the `poll` and `wait` bindings in `jakeos-runtime-task`.
- kernel:tools/jakeos/fuzz/tsk_submit/ · Fuzz target over submission records (`kernel:fuzz/tsk_submit`).
- kernel:tools/testing/selftests/jakeos/tsk/submit_*.rs · Selftests: no-op submit returns immediately, malformed record refused with a typed error.
- kernel:tools/testing/selftests/jakeos/tsk/complete_*.rs · Selftests: delivery to the submitter, inline rule, poll and wait semantics.
- kernel:Documentation/jakeos/tsk/transport.md · The submission and completion record layouts and the ordering rules.

#### Acceptance criteria
- [ ] `operation.submit` of a no-op Operation returns without waiting for completion on `qemu-x86_64` and `hw-h002`, and a malformed submission record is refused with the D-0006 typed error and creates no Operation.
- [ ] A completed Operation is delivered to the submitting Task through the D-0311 transport with identity, typed result and `completed_at`.
- [ ] Inline completion, if D-0307 allows it, is signalled exactly as the decision specifies, and a selftest distinguishes it from a later completion.
- [ ] `operation.poll` never blocks; `operation.wait` is the only blocking entry and returns on the first completion or its own deadline (I-030).
- [ ] A B-009 publish run for no-op Operations exists on H-001 and H-002 from this path.

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

Native software creates the B-014 live-Task population without a kernel thread per Task (§20): `jakeos/tsk/mux.rs` implements the model D-0315 (TSK-009) chose. For a UMCG-style model it is the kernel half: a bounded set of execution contexts (workers) per Component, a `TaskServer` object the runtime holds that receives typed notifications when a worker blocks in the kernel (page fault, synchronous retained driver path) or unblocks, and `worker.switch(task)` to run a Task on a worker; for a kernel-managed model it is the lightweight context allocator and scheduler hook; either way the compensation D-0315 named (activating a spare worker when one blocks) lives here. The runtime half is `runtime/task/src/scheduler.rs` in `jakeos-runtime-task`: the user-space Task queue and the switch loop, on which SDK-004's executor runs.

Destroying the Component reclaims every worker and Task (the CMP-004 leak test covers it), and native software has no thread-create ABI: the only creation Operation is `task.spawn` into a TaskGroup (TSK-021).

<!-- covers: INV-0379, INV-1156, INV-0377 -->

#### Out of scope
Completion wake (TSK-020). Userspace runtime executor (SDK-004). V1 affinity tuning (TSK-046). Task object (TSK-021).

#### Deliverables
- kernel:jakeos/tsk/mux.rs · The D-0315 kernel half: worker set, `TaskServer` notifications, `worker.switch`, blocked-worker compensation.
- runtime:task/src/scheduler.rs · The user-space Task queue and switch loop in `jakeos-runtime-task`.
- kernel:tools/testing/selftests/jakeos/tsk/mux_*.rs · Selftests: B-014 population without a thread per Task, blocked-worker compensation, reclamation on destroy, no thread-create ABI.
- kernel:Documentation/jakeos/tsk/multiplexing.md · The implemented model, the notification protocol and the compensation rule.

#### Acceptance criteria
- [ ] Creating the B-014 live-Task population in one Component does not create one kernel execution context per Task, on `qemu-x86_64` and `hw-h002` (asserted by counting kernel contexts through the OBS provider).
- [ ] A worker that takes a page fault or enters a synchronous retained driver path triggers the D-0315 compensation, and other Tasks queued on that worker become runnable within the bound the selftest records.
- [ ] Destroying the Component reclaims every execution context and Task with no unbounded kernel-memory growth in the CMP-004 leak test.
- [ ] Native software has no thread-create ABI: the ABI spec and the SDK expose `task.spawn` into a TaskGroup only (TSK-001's lint asserts it).

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

Completion integrates directly with Task scheduling (§18, Principle 5 of §67): the runtime never polls a socket to learn that work finished. `jakeos/tsk/wake.rs` records, when a Task suspends in `operation.wait` (or in the runtime's switch loop after registering interest), which Operation or wait set it awaits (the `awaited_operation` field on the Task object that OBS-005 reads), and on completion of that Operation makes the Task runnable: for a UMCG-style model by notifying the `TaskServer` and switching a worker to it, for a kernel-managed model by enqueuing its context. Native Task-to-Task handoff (A completes an Operation B awaits; B runs next on the same worker) is the B-003 measurement; the B-014 population blocking on Wait Operations all becoming runnable when their Waits signal is the scale test.

The runtime binding is in `runtime/task/src/wake.rs` (the executor's `Waker` implementation over this path, per the SDK-010 contract).

<!-- covers: INV-1296 -->

#### Out of scope
Inspect rendering of the awaited Operation (OBS-005). Debugger stacks (TSK-038). Direct Channel handoff on send (IPC-015). Multiplexing (TSK-019).

#### Deliverables
- kernel:jakeos/tsk/wake.rs · Awaited-Operation recording on the Task object and the completion-to-runnable path for the D-0315 model.
- runtime:task/src/wake.rs · The `Waker` implementation over the kernel path in `jakeos-runtime-task`.
- kernel:tools/testing/selftests/jakeos/tsk/wake_*.rs · Selftests: awaited-Operation visible, wake on completion without a thread per Task, B-014 population wake, handoff ordering.

#### Acceptance criteria
- [ ] A Task that submits an Operation and suspends is recorded as waiting on that Operation, visible through the OBS-005 provider as the awaited Operation identity and type.
- [ ] Completing the Operation makes the Task runnable on an execution context without a kernel thread per Task and without a second submit or poll syscall.
- [ ] The B-014 population blocking on Wait Operations all become runnable when those Waits signal, on `qemu-x86_64` and `hw-h002`.
- [ ] Native Task-to-Task handoff is measurable under B-003 on H-001 and H-002 from this path.

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

A Task is what native software runs (§1, §20, §69); threads are not a native API. `jakeos/tsk/task.rs` implements the identity D-0314 (TSK-008) chose: for `Object<Task>` it registers the type id with ABI-005, embeds the header and holds state (`Runnable`, `Running`, `Waiting { awaited }`, `Terminated`), the owning TaskGroup, the intent override (SCH-010) and the awaited Operation (TSK-020); for a runtime identity it holds the compact identity record the kernel reads for tracing and cancellation. `task.spawn(taskgroup, entry, argument)` creates a Task in a TaskGroup the caller holds with the `Spawn` right and makes it runnable; `task.terminate` from inside ends it. There is no join, kill or detach: the TaskGroup (TSK-022) owns termination. Spawn latency is published under B-002 by BEN-001 using this path.

The runtime side is `runtime/task/src/task.rs` in `jakeos-runtime-task`, the typed `Task` handle the SDK exposes.

<!-- covers: INV-0047, INV-1314 -->

#### Out of scope
Multiplexing (TSK-019). Cancellation walk (TSK-022). Personality threads (TSK-043). Identity decision (TSK-008).

#### Deliverables
- kernel:jakeos/tsk/task.rs · The Task identity per D-0314, `task.spawn`, `task.terminate`, state machine.
- kernel:jakeos/cap/rights_decl.rs · The `Task` rights vocabulary if D-0314 made it a kernel object (extending CAP-011's file).
- runtime:task/src/task.rs · The typed `Task` handle in `jakeos-runtime-task`.
- kernel:tools/testing/selftests/jakeos/tsk/task_object_*.rs · Selftests: spawn into a TaskGroup, identity per D-0314, no thread ABI, reclamation.

#### Acceptance criteria
- [ ] A Component spawns a Task into its TaskGroup through `task.spawn` and the Task becomes runnable, on `qemu-x86_64` and `hw-h002`; spawn into a TaskGroup without the `Spawn` right returns `Error::Rights`.
- [ ] The Task's identity matches D-0314 (kernel object with a handle, or runtime identity with kernel visibility), and `os inspect task` names it accordingly.
- [ ] Native software has no thread-create, thread-join or thread-kill ABI; the ABI spec contains only `task.spawn` and `task.terminate` and TSK-001's lint passes.
- [ ] A terminated Task's kernel state is reclaimed with no leak in the CMP-004 leak test, and a B-002 publish run exists on H-001 and H-002.

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

Structured concurrency (§21): cancelling a TaskGroup cancels every Task it owns, every child TaskGroup and every outstanding Operation any of them owns, and the cancel completes only when every owned Task has terminated. `jakeos/tsk/taskgroup_cancel.rs` implements `taskgroup.cancel` for a holder with the `Cancel` right: it walks the hierarchy depth-first, calls TSK-010's `cancel` on each owned Operation (which completes them with `Error::Cancelled`), applies the D-0305 cancellation model to each Task (cooperative, forced or staged), and completes the caller's `taskgroup.cancel` Operation once the last owned Task is `Terminated`. Application exit is a cancel of the Component's root TaskGroup (CMP-004). A permanent regression proves no Task survives group cancel or application exit.

The V0-G08 bound: a three-level tree of 1,000 Tasks with 1,000 outstanding Timer Operations is cancelled with every Task terminated within 50 ms of the call on `hw-h002` and within 200 ms on `qemu-x86_64`. The background-execution Capability exception is TSK-025 at V0.5.

<!-- covers: INV-0389, INV-0390, INV-1165, INV-0393, INV-0392 -->

#### Out of scope
Background-execution Capability (TSK-025). Deadline inheritance (TSK-036). Operation result encoding (TSK-010). The cancellation model (TSK-003).

#### Deliverables
- kernel:jakeos/tsk/taskgroup_cancel.rs · `taskgroup.cancel`: hierarchy walk, Operation cancel fan-out, D-0305 per-Task model, completion after the last termination.
- kernel:tools/testing/selftests/jakeos/tsk/taskgroup_cancel_*.rs · Selftests: fan-out to Tasks, children and Operations; completion ordering; the permanent no-survivor regression.
- kernel:tools/testing/selftests/jakeos/tsk/taskgroup_cancel_tree.rs · The three-level 1,000-Task, 1,000-Timer scenario with the V0-G08 timing assertions.

#### Acceptance criteria
- [ ] Cancelling a TaskGroup cancels owned Tasks, child TaskGroups and their outstanding Operations (each completing with `Error::Cancelled`) on `qemu-x86_64` and `hw-h002`.
- [ ] `taskgroup.cancel` completes only after every owned Task is `Terminated`, and application exit cancels the Component's root TaskGroup the same way.
- [ ] A permanent `pre-merge` regression proves no Task remains runnable after group cancel and after application exit.
- [ ] A three-level tree of 1,000 Tasks with 1,000 outstanding Timer Operations is cancelled with every Task terminated within 50 ms of the cancel call on `hw-h002` and within 200 ms on `qemu-x86_64` (the V0-G08 bound).

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

`Object<TaskGroup>` is the ownership unit of concurrency (§7, §21): an application owns TaskGroups, a TaskGroup owns Tasks and child TaskGroups, and each Component owns a root TaskGroup (CMP-014). `jakeos/tsk/taskgroup.rs` registers the type id with ABI-005, embeds the header, and holds the parent link, the child set, the owned Task set, the owned Operation set (TSK-013 Operations record their owner here), and the intent (SCH-010). Operations: `taskgroup.create(parent)` for a holder of the parent with the `Derive` right, returning a `Capability<TaskGroup>`; `taskgroup.inspect`; cancel is TSK-022. Destroying a TaskGroup with no children and no Tasks reclaims it; user space cannot mint a TaskGroup handle (a forged word returns `Error::Rights`). Its rights vocabulary is `Spawn`, `Derive`, `Cancel`, `Inspect`, `Admin`.

<!-- covers: INV-0048, INV-0169, INV-0387, INV-0388, INV-1157 -->

#### Out of scope
Cancellation propagation (TSK-022). Component create wrapping this object (CMP-005). Inspect of the tree at V0.5 (OBS-018).

#### Deliverables
- kernel:jakeos/tsk/taskgroup.rs · `Object<TaskGroup>`: header, hierarchy links, owned sets, `create` and `inspect` handlers.
- kernel:jakeos/cap/rights_decl.rs · The `TaskGroup` rights vocabulary (extending CAP-011's file).
- kernel:jakeos/obs/providers/taskgroup.rs · The `taskgroup` inspect provider (parent, children, Tasks, Operations, intent).
- kernel:tools/testing/selftests/jakeos/tsk/taskgroup_object_*.rs · Selftests: creation by Capability, hierarchy recording, reclamation, forgery refusal.

#### Acceptance criteria
- [ ] A Component's root TaskGroup is a kernel object referenced by `Capability<TaskGroup>`, and `taskgroup.create` on it with the `Derive` right returns a child whose parent link is recorded, on `qemu-x86_64` and `hw-h002`.
- [ ] A TaskGroup records its owned Tasks, child TaskGroups and owned Operations, visible through the `taskgroup` inspect provider.
- [ ] Destroying a TaskGroup with no children and no Tasks reclaims the object with no leak in the CMP-004 leak test.
- [ ] User space cannot mint a TaskGroup handle: a forged word returns `Error::Rights` and allocates no handle.

#### Verification
- Unit: `kernel:tests/tsk/taskgroup_object_*` on `qemu-x86_64` and `hw-h002`.
- Integration: Component-owned TaskGroup create and destroy in the V0 Component leak test.

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

V0-G07 and V0-G08 need one suite that a verifier can run and a gate can cite (§18, §19, §59). `tools/testing/selftests/jakeos/tsk/v0_acceptance_*.rs` is that suite, run by the BLD-006 guest agent as suite `tsk-v0-acceptance` on `qemu-x86_64` in `pre-merge` and on `hw-h002` nightly: Read, Write, Send, Receive, Timer and Wait each complete with a typed result; an Operation with a past deadline completes with `Error::DeadlineExceeded` and no successful result; a cancelled Operation completes with `Error::Cancelled` and no successful result; cancelling a TaskGroup leaves no runnable owned Task; and every case is written against the SDK wrappers (SDK-005) rather than raw entry so the suite also proves the SDK surface. Each case prints the roadmap task ID it verifies so BLD-006 attributes failures.

#### Out of scope
Connect, Accept, DeviceOperation (V0.5 kinds). Benchmark publication (TSK-002, BEN-001). Kind implementations (TSK-011, TSK-012).

#### Deliverables
- kernel:tools/testing/selftests/jakeos/tsk/v0_acceptance_kinds.rs · The six-kind completion cases.
- kernel:tools/testing/selftests/jakeos/tsk/v0_acceptance_deadline.rs · The deadline case.
- kernel:tools/testing/selftests/jakeos/tsk/v0_acceptance_cancel.rs · The Operation cancel and TaskGroup cancel cases.
- bld:image/suites/tsk-v0-acceptance.toml · The BLD-006 suite definition listing the binaries and their task IDs.
- kernel:.github/workflows/pre-merge.yml · The `tsk-v0-acceptance` job on `qemu-x86_64`.
- kernel:.github/workflows/nightly.yml · The same suite on `hw-h002`.

#### Acceptance criteria
- [ ] The suite passes Read, Write, Send, Receive, Timer and Wait completion on `qemu-x86_64` and `hw-h002` through the SDK-005 wrappers.
- [ ] The deadline case yields `Error::DeadlineExceeded` and no successful result; the cancel case yields `Error::Cancelled` and no successful result.
- [ ] The TaskGroup cancel case leaves no runnable owned Task, asserted through the OBS-005 provider after the cancel completes.
- [ ] The suite is a required `pre-merge` job on every merge to `main` of the kernel repository and runs nightly on `hw-h002`, with each case attributing failures to a task ID through BLD-006.

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

Realises TSK-006: a kernel Event object that user space signals and that Wait Operations consume. It is the native replacement for futex, eventfd and signal wake-ups that the toolkit, compositor and Wayland bridge need for cross-Task synchronisation without ambient signals.

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

TSK-007 fixes how batches are expressed. This task implements batch submit and io_uring-style links (Read then Send, Timer-bounded chains) that the compositor frame loop and File Browser need at V0.5. A failed link stops the chain with a typed result. Required by V0.5-G01 (Native compositor presents on the reference GPU): the compositor frame loop submits its per-frame Operations as linked batches.

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

Plumbs the carried Operation priority into block I/O ordering, Channel queueing and completion wake ordering per TSK-027 so compositor Operations preempt Background work for the V0.5 dropped-frame gate. Scheduling intent remains a SCH concept; this task is the Operation-side ordering.

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
- [ ] Ownership of in-flight Operations follows TSK-028; no Operation remains owned by the dead instance.
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

Implements the suspend/resume rule from TSK-004, refined by SVC's clock-semantics adr, on H-004 and H-002. V1 suspend-cycle gates require Timers and deadlines to behave as the decision states after wake. PWR owns the suspend mechanism; TSK owns deadline arithmetic across it.

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
- Depends on: TSK-018, TSK-010, TSK-017, ABI-014, HET-019, HET-016, HET-003, HET-012
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

V4 exit: Layer 1 frozen with a conformance suite. This suite covers every Operation kind, inline-completion signalling, deadline representation and cancellation results named by TSK-042. Binaries built against the freeze candidate run on every subsequent beta build. This task carries the S-005 and S-008 freeze once ABI-049 is accepted.

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
