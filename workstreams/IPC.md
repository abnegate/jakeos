# IPC · Channels and typed interfaces
- Prefix: IPC
- Lead: none
- Baseline: §12, §14, §15, §43

<!-- roadmap:generated:begin summary -->
Tasks: 71 live, 0 done, 0 in-progress, 71 todo, 0 dropped. Ready: 1. Blocked: 70. Weighted: 0%.
<!-- roadmap:generated:end -->

## Scope
This workstream owns Channel as the typed IPC primitive, the IDL and its compilers, generated stubs and wire layout, Interface versioning and Layer 2 evolution rules, small-message fast paths, Capability and MemoryObject transfer in messages, backpressure, feature negotiation, streams, and the pluggable transport behind generated stubs. It authors Channel Layer 1 reference pages, Interface design and evolution guidelines, and the fuzz and conformance surfaces for Channel syscalls and generated Interfaces.

Kernel Core owns Channel transport (§4, §14). IPC does not own Capability rights encoding, MemoryObject backing, Operation submission, ResourceDomain charging, service supervision, Wasm host mapping, UI protocol messages, the Semantic interface catalog, or documentation site build.

## Out of scope
Handle representation and Layer 1 handshake (ABI). Capability mint, derive, revocation and audit (CAP). Component graphs and Inputs/Outputs binding at launch (CMP). Operation rings, cancellation and deadlines (TSK). MemoryObject backing, map and transfer enforcement (MEM). ResourceDomain budgets and scheduler handoff hooks (SCH). Inspect CLI and trace substrate (OBS). Service supervisor, restart policy and native init (SVC). Wasm runtime and WASI imports (WASM). UI protocol IDL (UIP). Semantic interface catalog and AI broker (SEM). SDK crates, C wrappers and `os inspect` rendering (SDK). Fuzz fleet and CI plumbing (BLD). Benchmark methodology and publication (BEN). Docs site build (DOC). VM manager product (VIRT). Licensing policy (GOV). Threat model document (SEC).

## Tasks

### IPC-001 · Decide whether the kernel offers synchronous call with time-slice donation beside async send
- Type: adr
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: IPC-017, IPC-018, SCH-004, TSK-009, ABI-012
- Baseline: §15, §18, §53
- Decision: D-0139

Whether the kernel offers a synchronous call beside asynchronous send changes the scheduler and the Native ABI irreversibly (§15, §18, §53). D-0139 evaluates option A (asynchronous send and receive only; a request-reply is two Operations and the runtime pairs them) against option B (asynchronous send plus a `channel.call` Operation whose completion is awaited with the caller's remaining time slice donated to the callee, in the seL4 and LRPC lineage), using IPC-017's same-core and cross-core round-trip tables and IPC-018's study of Cap'n Proto RPC, FIDL, Genode and QNX. Option B is admissible only as an Operation awaited through `operation.wait`: D-0012 (ABI-012) already forbids any entry that blocks the caller except wait-for-completion, so a blocking `call` syscall is not an option. The decision names the ABI-visible entry points that follow (which kinds exist in the D-0014 table) and coordinates with the scheduler mapping D-0253 (SCH-004) and the Task mapping D-0315 (TSK-009) because donation is a scheduler operation.

The executing agent writes both options with their round-trip, fairness and complexity consequences, cites `reports/spikes/IPC-017.md` and `reports/spikes/IPC-018.md`, records the Decision as the kind set and the rejected option, and lists follow-ups (IPC-015's handoff hook shape).

<!-- covers: GAP-0481 -->

#### Out of scope
Fast-path technique selection (IPC-003). Direct-switch hook implementation (IPC-015, SCH-005). Intent inheritance across handoff (SCH-017).

#### Deliverables
- roadmap:decisions/D-0139-decide-call-semantics.md · Both options with consequences, Evidence citing the two spike reports, the Decision as the Operation kind set and the rejected option, follow-ups.

#### Acceptance criteria
- [ ] D-0139 evaluates option A (asynchronous send and receive only) and option B (asynchronous send plus synchronous call with time-slice donation, as an awaited Operation) against the IPC-017 same-core and cross-core reports and the IPC-018 study.
- [ ] The Decision names the rejected option, the Operation kinds that follow from the choice in the D-0014 table, and states that no entry blocks the caller except `operation.wait`.
- [ ] Review records SCH and TSK lead sign-off on the pull request.

#### Verification
- Review: SCH and TSK leads sign off on the pull request; the Decision lists both options.

#### Evidence
- none

### IPC-002 · Decide the Interface-evolution rules for Layer 2 Interfaces (prototyped state)
- Type: adr
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: IPC-019, IPC-006
- Baseline: §12, §66
- Decision: D-0141
- Risks: R-005
- Invariants: I-041

Every Interface carries an explicit evolution strategy (§12): how it versions, how fields and optional methods are added, how peers negotiate, and what an old client sees from a new server. D-0141 records those rules for Layer 2 (S-014) in prototyped state, choosing among option A (schema-indexed optional fields with generated negotiation: every field has a stable index, absent fields take defaults, the generated stubs negotiate a feature set at bind), option B (self-describing envelopes with unknown-field preservation: each message carries its field table and receivers forward what they do not understand), and option C (explicit major and minor versions with dual-stack serving during an overlap window), using IPC-019's three-revision evolution of a real V0 Interface as the evidence. The IDL chosen by D-0148 (IPC-006) must be able to express the accepted rules. The rules freeze at V1 (IPC-042, R-005).

The executing agent writes each option's consequences for wire cost, codegen complexity and what breaks old clients, cites `reports/spikes/IPC-019.md`, records the Decision as the rule set the IDL compiler enforces (IPC-021 implements the header and tests; IPC-038 the rest), and updates S-014's register entry to `prototyped`.

<!-- covers: INV-0247, INV-0260, INV-0249 -->

#### Out of scope
V1 freeze of the rules (IPC-042). Layer 1 handshake (ABI-016, ABI-004). IDL selection (IPC-006). Full schema-evolution implementation (IPC-038).

#### Deliverables
- roadmap:decisions/D-0141-decide-evolution-rules.md · Options with wire, codegen and compatibility consequences, Evidence citing `reports/spikes/IPC-019.md`, the Decision as the enforceable rule set, rejected options, follow-ups.
- roadmap:registers/surfaces.md · S-014 `Decided by: IPC-002`, `State: prototyped`.

#### Acceptance criteria
- [ ] D-0141 evaluates option A (schema-indexed optional fields with generated negotiation), option B (self-describing envelopes with unknown-field preservation), and option C (explicit major and minor versions with dual-stack overlap) against the three-revision spike.
- [ ] The Decision states how optional methods, unknown fields and version identity appear on the wire, and records that the IDL ships with an evolution story the compiler enforces.
- [ ] S-014 is `prototyped` with IPC-002 under `Decided by`, and Review records ABI lead sign-off on the pull request.

#### Verification
- Review: ABI lead sign-off recorded on the pull request.

#### Evidence
- none

### IPC-003 · Select the small-message fast-path technique from measured prototypes
- Type: adr
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: IPC-017
- Baseline: §15, §53
- Decision: D-0142

V0 exit requires an accepted fast-path decision with rejected options, chosen from IPC-017's measurements before Channel kernel semantics are fixed (§15, §53). D-0142 selects among shared ring, CPU-register-carried messages, scheduler-aware handoff, lock-free cross-core queues and a recorded combination (the option texts already carry each technique's consequences); the accepted option names the ABI-visible send path that remains (what a Channel send is on S-012) and the techniques IPC-016 implements, and lists the rejected techniques so no later task re-prototypes them. Numbers live only in `reports/spikes/IPC-017.md` and the B-004 and B-005 records.

The executing agent fills the Decision and Consequences from the report's same-core and cross-core tables, cites the report in Evidence, and sets S-012's register entry `Decided by: IPC-003`, `State: prototyped`.

<!-- covers: INV-0299, GAP-0480 -->

#### Out of scope
Call versus send (IPC-001). Production fast path (IPC-016). The prototypes (IPC-017).

#### Deliverables
- roadmap:decisions/D-0142-adr-fast-path-selection.md · The Decision, Consequences, rejected techniques and follow-ups filled from the IPC-017 report, with the report cited in Evidence.
- roadmap:registers/surfaces.md · S-012 `Decided by: IPC-003`, `State: prototyped`.

#### Acceptance criteria
- [ ] D-0142 evaluates option A (shared ring), option B (CPU-register-carried messages), option C (scheduler-aware handoff), option D (lock-free cross-core queues) and option E (a recorded combination) against `reports/spikes/IPC-017.md`.
- [ ] The Decision names the rejected techniques, the technique or combination IPC-016 implements, and the ABI-visible send path that remains on S-012.
- [ ] S-012 is `prototyped` with IPC-003 under `Decided by`, and Review records ABI lead sign-off on the pull request.

#### Verification
- Review: ABI lead sign-off recorded on the pull request.

#### Evidence
- none

### IPC-004 · Decide whether IDL-generated code is committed or generated at build time
- Type: adr
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: IPC-006
- Baseline: §14
- Decision: D-0145

Typed IPC across two repositories and several languages breaks when generated stubs drift from their IDL (§14). D-0145 decides, before the first Rust stubs land, whether generated code is committed beside the IDL (reviewable diffs, drift caught by a CI regeneration check) or emitted at build time by a `build.rs` step in every consuming crate (no drift by construction, generator on every build), evaluating drift, reviewability, build time, multi-language backends and how the kernel repository (which cannot depend on the platform's compiler at build time) consumes the same IDL. The accepted option states how CI proves the generator is deterministic and its output matches the IDL (IPC-034 implements that check).

<!-- covers: GAP-0098 -->

#### Out of scope
Generator determinism check (IPC-034). Generated-code licence exception (IPC-005). The compiler (IPC-012).

#### Deliverables
- roadmap:decisions/D-0145-decide-generated-code-placement.md · Options with drift, review, build-time and cross-repository consequences, the Decision as the placement rule and the CI proof, rejected option, follow-ups.

#### Acceptance criteria
- [ ] D-0145 evaluates option A (commit generated stubs beside the IDL) and option B (emit stubs at build time from the IDL) against drift, reviewability, build time, multi-language backends and kernel-repository consumption.
- [ ] The Decision states how CI proves generator output is deterministic and matches the IDL, and where generated files live (or do not) in each repository.
- [ ] Review records BLD lead sign-off on the pull request.

#### Verification
- Review: BLD lead sign-off recorded on the pull request.

#### Evidence
- none

### IPC-005 · Decide that IDL compiler output is owned by its user with no copyleft obligation
- Type: adr
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: IPC-006, GOV-003
- Baseline: §14, §51
- Decision: D-0146

Generated stubs land in every application from V0 onward (§14); unclear terms would contaminate the ecosystem (§51). D-0146 decides, with GOV's licence firewall (D-0102), whether compiler output is owned by the compiler's user with no copyleft obligation (a generated-code exception in every emitted file header), inherits the compiler's licence, or is dedicated to the public domain, and states the exact header text every backend emits (IPC-012 emits it). Proprietary native applications are permitted (D-0262), so the exception must be unambiguous for them.

<!-- covers: GAP-0007 -->

#### Out of scope
IDL and ABI specification licence (IPC-024). Firewall mapping of layers (GOV-003). The compiler (IPC-012).

#### Deliverables
- roadmap:decisions/D-0146-decide-generated-code-licence.md · Options with ecosystem consequences, the Decision with the verbatim header text, rejected options, follow-ups.

#### Acceptance criteria
- [ ] D-0146 evaluates option A (generated-code exception, output owned by the compiler user with no copyleft obligation), option B (output inherits the compiler licence), and option C (output dedicated to the public domain) with GOV.
- [ ] The Decision states the header text every backend emits, verbatim, and confirms it is compatible with D-0102 and with proprietary applications (D-0262).
- [ ] Review records GOV lead sign-off on the pull request.

#### Verification
- Review: GOV lead sign-off recorded on the pull request.

#### Evidence
- none

### IPC-006 · Decide the IDL: adopt WIT, FIDL, Cap'n Proto schema or design new
- Type: adr
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: IPC-018, WASM-002
- Baseline: §12, §14, §13
- Decision: D-0148

The IDL is the language every platform Interface is written in (§12, §14); switching after V1 would invalidate every generated binding. D-0148 evaluates adopting WIT (the Wasm Component Model's IDL, studied by WASM-002), FIDL, the Cap'n Proto schema language, or designing a new IDL, in a written matrix against the requirements the native model imposes: ownership transfer of Capabilities and MemoryObjects as first-class parameter modes (`move`, `borrow`, `share`), Capability passing, the evolution rules D-0141 will need (versioning, optional fields and methods), streams, multi-language codegen through a pluggable backend, and the ability to express the tracing metadata OBS-004 emits. IPC-018's study of Cap'n Proto RPC, FIDL and Overnet, Genode and QNX is the evidence. V0 exit requires the accepted decision with rejected options; the relationship to WIT for Wasm Components is IPC-022 later.

The executing agent writes the matrix as the Options section, records the Decision as the language (and, for an adopted one, the named revision and the extensions the project adds), and lists follow-ups: IPC-012 (compiler), IPC-004 (placement), IPC-005 (licence).

<!-- covers: GAP-0519, INV-0261, INV-0260, INV-0247 -->

#### Out of scope
Native IDL versus WIT mapping (IPC-022). Compiler implementation (IPC-012). Wire encoding (IPC-007).

#### Deliverables
- roadmap:decisions/D-0148-decide-idl.md · The evaluation matrix as options, Evidence citing `reports/spikes/IPC-018.md` and `reports/spikes/WASM-002.md`, the Decision naming the language, revision and extensions, rejected options, follow-ups.

#### Acceptance criteria
- [ ] D-0148 evaluates option A (adopt WIT), option B (adopt FIDL), option C (adopt Cap'n Proto schema) and option D (design a new IDL) in a written matrix against ownership transfer modes, Capability passing, versioning, optional methods, streams, multi-language codegen and tracing metadata.
- [ ] The Decision records that every Interface carries an explicit evolution strategy, that the IDL ships with an evolution story, and names the revision and extensions for an adopted language.
- [ ] Review records WASM and ABI lead sign-off on the pull request.

#### Verification
- Review: WASM and ABI leads sign off on the pull request.

#### Evidence
- none

### IPC-007 · Decide the typed-message wire format and inline-payload threshold
- Type: adr
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: IPC-020, IPC-018
- Baseline: §14, §15
- Decision: D-0154
- Invariants: I-063

The wire format is what a Channel carries (§14, §15) and, with the IDL, the whole Layer 2 message surface S-013. D-0154 chooses among a fixed layout (fields at compile-time offsets, in-place zero-copy access, no schema on the wire), a self-describing format (tagged fields, larger but tolerant), and a schema-indexed format (field indices with a compact table, the middle ground), using IPC-020's encode, decode and receiver-side validation measurements. It also answers Q-005: the rule for when a payload travels inline in the message versus as a MemoryObject ownership transfer, stated as a rule (for example "inline when the encoded message fits one message slot as fixed by the ring layout; otherwise a MemoryObject") without a number in prose (numbers live in the register and the report). Payload bytes never move when avoidable (I-063). S-013 becomes `prototyped`.

<!-- covers: INV-0291, INV-0302, GAP-0520 -->

#### Out of scope
MemoryObject backing and transfer enforcement (MEM-002, MEM-003). Production lowering of large payloads (IPC-036). Validation hardening (IPC-037).

#### Deliverables
- roadmap:decisions/D-0154-decide-wire-format.md · Options with the IPC-020 measurements, the Decision as the format plus the inline-versus-MemoryObject rule, rejected options, follow-ups.
- roadmap:registers/surfaces.md · S-013 `Decided by: IPC-007`, `State: prototyped`.
- roadmap:registers/questions.md · Q-005 `Status: answered`.

#### Acceptance criteria
- [ ] D-0154 evaluates option A (fixed layout), option B (self-describing) and option C (schema-indexed) against encode, decode and receiver-side validation cost in `reports/spikes/IPC-020.md`.
- [ ] The Decision states the inline-versus-MemoryObject threshold as a rule without a number in prose, names S-013, and marks Q-005 answered by IPC-007.
- [ ] S-013 is `prototyped` with IPC-007 under `Decided by`, and Review records ABI and MEM lead sign-off on the pull request.

#### Verification
- Review: ABI and MEM leads sign off on the pull request.

#### Evidence
- none

### IPC-008 · Build the IPC round-trip benchmark against Linux UDS and pipe ping-pong
- Type: benchmark
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: IPC-016, IPC-015, BEN-007, BEN-005, BLD-082
- Baseline: §14, §54, §53
- Benchmarks: B-004, B-005
- Invariants: I-061

The IPC round trip is the number the native model is judged by (§14, §53, §54). The harness `bench/harness/B-004/` (crate `jakeos-bench-ipc-roundtrip`, scenarios `ipc-roundtrip-same-core` and `ipc-roundtrip-cross-core`) runs a small-message ping-pong between two Components over the IPC-016 fast path with the IPC-015 handoff, pinned per BEN-064, and in the same session runs Unix-domain-socket and pipe ping-pong between two processes on the personality side of the same kernel; `bench/harness/B-005/` (scenario `ipc-throughput`) streams messages one way and records messages per second at the register's payload sizes. Both emit BEN-005 records from the BLD-010 nightly job on `qemu-x86_64` and `hw-h002`; reports go to `reports/benchmarks/B-004/` and `B-005/` as `h001.md` and `h002.md`. V0 is publish-only; V1 absolute targets are IPC-054's.

<!-- covers: INV-0277 -->

#### Out of scope
Methodology and publication policy (BEN-003, BEN-007). Tracing-overhead ratio (B-012, OBS-001). V1 absolute targets (IPC-054).

#### Deliverables
- bench:harness/B-004/ · Crate `jakeos-bench-ipc-roundtrip`: `ipc-roundtrip-same-core`, `ipc-roundtrip-cross-core`, `baseline-uds`, `baseline-pipe`.
- bench:harness/B-005/ · Crate `jakeos-bench-ipc-throughput`: `ipc-throughput` at the register payload sizes with the same baselines.
- roadmap:reports/benchmarks/B-004/h001.md · H-001 round-trip report (labelled QEMU).
- roadmap:reports/benchmarks/B-004/h002.md · H-002 round-trip report V0-G13 cites.
- roadmap:reports/benchmarks/B-005/h001.md · H-001 throughput report.
- roadmap:reports/benchmarks/B-005/h002.md · H-002 throughput report.

#### Acceptance criteria
- [ ] `ipc-roundtrip-same-core` and `ipc-roundtrip-cross-core` run on `qemu-x86_64` and `hw-h002` and the reports under `reports/benchmarks/B-004/` carry p50 and p99 for each, following the BEN-005 skeleton.
- [ ] Each report includes Linux Unix-domain-socket and pipe ping-pong measured on the same machine in the same session, with mitigation state recorded.
- [ ] `ipc-throughput` reports under `reports/benchmarks/B-005/` cover the register's payload sizes on both machines.
- [ ] The BLD-010 nightly job runs both harnesses and the V0 target kind of B-004 and B-005 is `publish`.

#### Verification
- Bench: B-004 and B-005 on H-001 and H-002; target per register.
- Integration: `ipc:benches/roundtrip_*` on CI matrix entries `qemu-x86_64` and `hw-h002`.

#### Evidence
- none

### IPC-009 · Define Channel<T> backpressure: bounded depth, slow-receiver policy, depth in os inspect
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: IPC-010, TSK-011
- Baseline: §14, §15, §23
- Risks: R-073
- Threats: T-016

Unspecified backpressure livelocks the fast path and lets a slow receiver exhaust kernel memory (R-073, T-016). `jakeos/ipc/queue.rs` gives every Channel a bounded queue depth fixed at creation (`channel.create(depth, policy)`), a slow-receiver policy chosen by the creator from `Block` (the Send Operation stays outstanding until a slot frees, which is the default), `Fail` (Send completes with `Error::Exhausted`) and `Drop` (the oldest queued message is dropped and the Send completes; only for Channel types whose IDL marks them lossy), and exposes depth, occupancy, policy and the count of blocked senders and receivers through the Channel inspect provider (OBS-007) for `os inspect channel`. Queue memory is charged to the creator's ResourceDomain (IPC-027 refines charging at V0.5) and a Channel's depth bounds its kernel memory, so exceeding depth never allocates.

Native software never sees a socket-buffer API; the policy is a typed argument, not a `setsockopt`.

<!-- covers: EXTRA-001 -->

#### Out of scope
ResourceDomain charging of queue memory (IPC-027). Inspect CLI rendering (SDK-007). Stream flow control (IPC-039). The Channel object (IPC-010).

#### Deliverables
- kernel:jakeos/ipc/queue.rs · Bounded queue, the three policies, occupancy accounting, blocked-party counts.
- kernel:jakeos/ipc/channel.rs · `channel.create(depth, policy)` arguments (extending IPC-010's file).
- kernel:jakeos/obs/providers/channel.rs · Depth, occupancy, policy and blocked counts in the `channel` provider (extending OBS-007's file).
- kernel:tools/testing/selftests/jakeos/ipc/backpressure_*.rs · Selftests: each policy under a slow receiver, no allocation past depth, inspect fields.
- kernel:Documentation/jakeos/ipc/backpressure.md · The policies, their semantics for senders and receivers, and the lossy-type rule.

#### Acceptance criteria
- [ ] Each Channel has a bounded depth and a policy fixed at `channel.create`, visible through `os inspect channel` with current occupancy and blocked sender and receiver counts.
- [ ] A slow receiver causes the sender's Send Operation to behave per the declared policy (`Block` stays outstanding, `Fail` completes with `Error::Exhausted`, `Drop` discards the oldest and completes) and never livelocks the fast path, on `qemu-x86_64` and `hw-h002`.
- [ ] Exceeding depth allocates no additional kernel memory; the T-016 exhaustion is the typed error, and a `Drop` policy is refused at creation for a Channel type not marked lossy in its IDL.

#### Verification
- Unit: `kernel:tests/ipc/backpressure_*` on `qemu-x86_64` and `hw-h002`.
- Integration: slow-receiver fixture under BLD-006.

#### Evidence
- none

### IPC-010 · Implement the Channel kernel Object with typed endpoints, send, receive and inspect data
- Type: build
- Milestone: V0
- Status: todo
- Size: L
- Owner: none
- Depends on: ABI-002, ABI-005, CAP-005, TSK-013, KRN-001, CMP-014, KRN-013
- Baseline: §4, §7, §14, §59
- Invariants: I-018

Channel transport is kernel core (§4, D-0157). `jakeos/ipc/channel.rs` registers the `Channel` type id with ABI-005 and implements `channel.create` returning a pair of `Capability<Channel>` endpoints of one Interface type id (from the IDL compiler's type registry, IPC-012) with rights `Send`, `Receive`, `Transfer`, `Inspect`; each endpoint holds the queue (IPC-009), the peer link, and peer-closed state. Send and Receive are the TSK-011 Operation kinds: `send.rs` and `recv.rs` are the glue that enqueues a message descriptor (IPC-007 format) and completes a waiting Receive, moving handle slots through CAP-006's `move_between` at commit (IPC-014). Closing the last handle to an endpoint marks the peer closed, which IPC-011 turns into typed disconnects. The `channel` inspect provider reports endpoints, Interface type, queue state, waiting senders and receivers and peer-closed state (V0-G10). Destroying both endpoints reclaims queue memory (the CMP-004 leak test covers it).

Native IPC is not a socket (I-018): there is no byte-stream mode, no `read` on a Channel, and the ABI-006 gate rejects such entries.

<!-- covers: INV-0051, INV-0112, INV-0161, INV-0278, INV-1159, INV-1324 -->

#### Out of scope
Small-message fast path (IPC-016). Handle slots in messages (IPC-014). Inspect CLI (SDK-007). Typed inspect Interface (OBS-006). Backpressure policy (IPC-009).

#### Deliverables
- kernel:jakeos/ipc/ipc.rs · Crate root for the IPC area.
- kernel:jakeos/ipc/channel.rs · `Object<Channel>`: endpoint pair, Interface type id, peer link, peer-closed state, `channel.create`.
- kernel:jakeos/ipc/send.rs · Send-kind glue: enqueue, handle-slot commit hook, completion.
- kernel:jakeos/ipc/recv.rs · Receive-kind glue: dequeue or park, completion with the message descriptor.
- kernel:jakeos/cap/rights_decl.rs · The `Channel` rights vocabulary (extending CAP-011's file).
- kernel:jakeos/obs/providers/channel.rs · The `channel` inspect provider.
- kernel:tools/testing/selftests/jakeos/ipc/channel_*.rs · Selftests: create, typed send and receive, peer-closed, reclamation, no byte mode.
- kernel:Documentation/jakeos/ipc/channel.md · The object, its Operations and states.

#### Acceptance criteria
- [ ] `channel.create` yields a typed endpoint pair of one Interface type id; Send and Receive are Operations that complete with typed results, and a Send of a message of another type id is refused with the typed error, on `qemu-x86_64` and `hw-h002`.
- [ ] `os inspect channel` data includes endpoints, Interface type, queue depth and occupancy, waiting senders and receivers, and peer-closed state for every live Channel (V0-G10).
- [ ] No native entry reads or writes a Channel as a byte stream; the ABI-006 gate and TSK-011's Read refusal enforce it.
- [ ] Destroying both endpoints reclaims queue memory; the CMP-004 leak test shows no Channel residue.

#### Verification
- Unit: `kernel:tests/ipc/channel_*` on `qemu-x86_64` and `hw-h002`.
- Integration: V0 demo pipeline with CMP-011.
- Review: ABI review gate checklist includes Channel.

#### Evidence
- none

### IPC-011 · Define and implement typed error, peer-death and timeout semantics for calls
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: IPC-010, TSK-010, CMP-008, ABI-009
- Baseline: §12, §32
- Invariants: I-037

Failure and restart are part of every typed Interface (§12, §32, I-037). `jakeos/ipc/failure.rs` implements the peer-death path: when the last handle to an endpoint is dropped (its Component exited with any D-0066 cause, or closed it), every outstanding Receive on the peer completes with `Error::Disconnected`, every outstanding Send completes with `Error::Disconnected` without delivering, and no further payload is ever delivered on that Channel; a later Send returns `Error::Disconnected` immediately. Deadlines on Channel Operations are TSK-010's (`Error::DeadlineExceeded`, never a late result). The IDL error taxonomy is the D-0006 vocabulary plus per-Interface application errors declared in the IDL (`error` types in the IPC-012 front end), which travel in the reply message as typed values distinct from transport failures.

The V0 fault demo (V0-D03, CMP-011) exercises it: B panics and A observes `Error::Disconnected` on its in-flight call. Client rebind and retry codegen is IPC-028 at V0.5; A rebinds manually in the demo.

<!-- covers: INV-0259 -->

#### Out of scope
Client rebind and retry codegen (IPC-028). Supervisor restart (SVC-004). Panic abort policy (CMP-008). Deadline enforcement (TSK-010).

#### Deliverables
- kernel:jakeos/ipc/failure.rs · Peer-death detection on last-handle drop and `Error::Disconnected` completion of outstanding Operations.
- idl:src/frontend/errors.rs · IDL `error` type declarations and their placement in reply messages (extending IPC-012's front end).
- kernel:tools/testing/selftests/jakeos/ipc/failure_*.rs · Selftests: peer death during Receive, during Send, after close; no late payload; deadline case.
- kernel:Documentation/jakeos/ipc/failure.md · The failure taxonomy: transport errors versus IDL application errors, and what each side observes on peer death.

#### Acceptance criteria
- [ ] Peer death completes every outstanding Receive on the surviving endpoint with `Error::Disconnected`, completes outstanding Sends without delivering, and delivers no further payload, on `qemu-x86_64` and `hw-h002`.
- [ ] An Operation with an expired deadline completes with `Error::DeadlineExceeded` and never delivers a late result (through TSK-010's path).
- [ ] IDL-declared application errors travel in the reply as typed values distinct from transport failures, and the V0 fault demo shows Component B panicking and Component A observing `Error::Disconnected`.

#### Verification
- Unit: `kernel:tests/ipc/failure_*` on `qemu-x86_64` and `hw-h002`.
- Demo: V0 fault demo on H-002.
- Integration: deadline and peer-death cases in TSK-024.

#### Evidence
- none

### IPC-012 · Implement the IDL compiler with Rust wire layout, stub, ownership and tracing codegen
- Type: build
- Milestone: V0
- Status: todo
- Size: L
- Owner: none
- Depends on: IPC-006, IPC-007, IPC-004, IPC-005, ABI-007, BLD-082
- Baseline: §14, §24, §53

The IDL compiler is the single schema source for every Interface (§14). `idl/` in the platform monorepo holds the crate `jakeos-idl-compiler` (binary `jakeos-idl`): a front end that parses the language D-0148 (IPC-006) chose from `idl/interfaces/<area>/<Name>.idl`, typechecks it (Interfaces, methods, message and error types, `move`, `borrow` and `share` parameter annotations for Capability and MemoryObject arguments, the D-0141 evolution attributes), and an IR (`idl/src/ir.rs`) that backends consume; and the first backend, `idl/src/backend/rust/`, emitting the D-0154 wire representation (`<Name>_wire.rs`: layout, encode, decode, receiver-side validation), client stubs (`<Name>Client` with one `async fn` per method returning the typed result, built on the IPC-013 proxy layer), server stubs (`<Name>Server` trait plus dispatch), ownership semantics (moved arguments consume the handle in the generated signature), and per-method tracing metadata (a `TRACE_SCHEMA` table of Interface name, method name and message type ids that OBS-004 shapes and `os trace` reads). Every emitted file begins with the D-0146 (IPC-005) header text. Generated code is placed per D-0145 (IPC-004).

The compiler is deterministic: the same IDL produces byte-identical output, and IPC-034 later turns that into a CI check; the sample Interface is `idl/interfaces/sdk/ImageDecoder.idl` (SDK-002).

<!-- covers: INV-0292, INV-0286, INV-0287, INV-0288, INV-1005, INV-0285, INV-0474, GAP-0007 -->

#### Out of scope
Async proxy semantics (IPC-013). C backend (IPC-048). Plugin API for other backends (IPC-047). Trace substrate (OBS-003). Determinism CI check (IPC-034).

#### Deliverables
- idl:Cargo.toml · Crate `jakeos-idl-compiler`, binary `jakeos-idl`.
- idl:src/frontend/ · Lexer, parser, typechecker for the D-0148 language with the ownership annotations and evolution attributes.
- idl:src/ir.rs · The backend-facing IR.
- idl:src/backend/rust/ · Wire layout, client stubs, server stubs, ownership signatures and `TRACE_SCHEMA` emission.
- idl:src/header.rs · The D-0146 generated-code header text, emitted first in every file.
- idl:interfaces/sdk/ImageDecoder.idl · The V0 sample Interface.
- idl:tests/frontend_*.rs · Parse and typecheck tests including annotation and evolution-attribute errors.
- idl:tests/rust_backend_*.rs · Golden-output tests for the sample Interface and determinism across two runs.
- docs:idl/language.md · The language reference as accepted by D-0148 with the project's extensions.

#### Acceptance criteria
- [ ] `jakeos-idl` parses `idl/interfaces/sdk/ImageDecoder.idl`, typechecks `move`, `borrow` and `share` annotations (rejecting a `share` of a Capability type with a typed diagnostic), and emits Rust wire layout, client stubs, server stubs and `TRACE_SCHEMA` metadata.
- [ ] Every emitted file begins with the D-0146 generated-code header text verbatim.
- [ ] Running the compiler twice on the same IDL produces byte-identical output, asserted by `idl:tests/rust_backend_*`; a golden-output diff fails the test.
- [ ] A native crate cannot hand-write a parallel schema for a compiled Interface: the generated wire types are the only types with the Interface's type id, and IPC-034 later lints it.

#### Verification
- Unit: `idl:tests/frontend_*` and `idl:tests/rust_backend_*` on host CI.
- Integration: the ImageDecoder sample from SDK-002 compiles against the generated stubs.

#### Evidence
- none

### IPC-013 · Generate Interface<T> proxies with async methods, futures and in-flight cancellation
- Type: build
- Milestone: V0
- Status: todo
- Size: L
- Owner: none
- Depends on: IPC-012, TSK-011, TSK-010, BLD-082
- Baseline: §12, §14, §59
- Invariants: I-030

A typed service contract in the IDL becomes an `Interface<T>` proxy the caller awaits (§12, §14): `ipc/` in the platform monorepo holds the crate `jakeos-ipc-runtime`, the user-space Channel layer the generated stubs sit on. `ipc/src/proxy.rs` implements `Interface<T>` over a `Channel<T>` endpoint: each generated method encodes its request (IPC-012 wire), submits a Send Operation and a Receive Operation through the SDK's Operation wrappers (SDK-005), and returns a future that resolves to the typed reply or a typed failure; `ipc/src/call.rs` pairs request and reply by a per-call id in the wire header and handles the D-0139 call shape if one was chosen. Cancelling the returned future cancels the in-flight Operations (TSK-010), so the server observes cancellation through its Receive completing with `Error::Cancelled` and the client never receives a result. Native APIs are asynchronous by default (I-030): no generated method blocks.

The V0 demo (CMP-011) uses the generated `ImageDecoderClient` and the cancellation demo cancels a `decode` in flight.

<!-- covers: INV-0058, INV-0248, INV-0254, INV-0256, INV-0258, INV-0279, INV-1316 -->

#### Out of scope
Streams (IPC-039). Version negotiation codegen (IPC-033). Runtime executor (SDK-004). The compiler (IPC-012).

#### Deliverables
- ipc:Cargo.toml · Crate `jakeos-ipc-runtime`.
- ipc:src/proxy.rs · `Interface<T>` over a Channel endpoint, method dispatch to Send and Receive Operations, future construction.
- ipc:src/call.rs · Request and reply pairing by call id, D-0139 call shape if chosen, cancellation propagation.
- idl:src/backend/rust/proxy.rs · The generated-client shape that targets `Interface<T>` (extending IPC-012's backend).
- idl:tests/async_stubs_*.rs · Tests: each method is a future, cancellation reaches the server, no blocking method exists.
- ipc:tests/ipc/proxy_*.rs · Runtime tests for pairing, cancellation and typed failures.

#### Acceptance criteria
- [ ] Generated proxies expose each IDL method as an `async fn` returning a future; awaiting it yields the typed result, and no generated method blocks the calling execution context (TSK-001's lint passes on generated code).
- [ ] Cancelling the future cancels the in-flight Send and Receive Operations; the server's Receive completes with `Error::Cancelled` and the client never receives a result, on `qemu-x86_64` and `hw-h002`.
- [ ] The V0 demo's `ImageDecoderClient` is generated from `idl/interfaces/sdk/ImageDecoder.idl` and used by CMP-011 without hand-written glue.

#### Verification
- Unit: `idl:tests/async_stubs_*` on host CI.
- Integration: V0 demo and cancellation demo on `qemu-x86_64` and `hw-h002`.
- Demo: V0 Component A to Channel to MemoryObject round trip on H-002.

#### Evidence
- none

### IPC-014 · Transfer Capability and MemoryObject ownership inside Channel messages
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: IPC-010, IPC-007, CAP-006, MEM-010, MEM-003
- Baseline: §12, §15, §16
- Threats: T-002
- Invariants: I-063

Large payloads never copy per hop because they travel as MemoryObject Capabilities inside messages (§15, §16). The D-0154 wire format reserves handle slots in the message header: `jakeos/ipc/handles.rs` validates at Send commit that the message names no more handle slots than its Interface type permits (the IDL declares the count per message type; an excess is a confused-deputy attempt, T-002, refused with the typed error and no receiver handle), and calls CAP-006's `move_between` for each slot so the sender's handle is gone and the receiver's exists in one commit; MemoryObject slots additionally trigger MEM-010's ownership transfer (unmap or invalidation per D-0199). `borrow` and `share` annotations from the IDL map onto the CAP-010 transfer rights: a `borrow` slot moves a derived Capability with a lifetime the receiver must return (MEM-018 at V0.5 enforces borrows; V0 treats `borrow` as a move of an attenuated Capability), a `share` slot is refused for MemoryObjects at V0.

The V0 demo's reply carries the result MemoryObject this way; MEM-012's physical-page identity check passes through this path.

<!-- covers: INV-0257, INV-1006, INV-1003, INV-0301 -->

#### Out of scope
MemoryObject map and backing (MEM-005, MEM-007). Capability derive and revocation (CAP-003, CAP-004). Physical-page identity harness (MEM-012). Borrow enforcement (MEM-018).

#### Deliverables
- kernel:jakeos/ipc/handles.rs · Handle-slot validation against the Interface's declared count, `move_between` per slot at commit, MemoryObject transfer hook, `borrow` and `share` mapping.
- idl:src/backend/rust/wire.rs · Handle-slot count and kinds emitted per message type from the IDL (extending IPC-012's backend).
- kernel:tools/testing/selftests/jakeos/ipc/handle_transfer_*.rs · Selftests: Capability move, MemoryObject move with page identity, excess-slot refusal, `share` refusal for MemoryObjects.

#### Acceptance criteria
- [ ] A message moves a Capability and a MemoryObject; after Send commit the sender's handles are invalid and the receiver holds the only handles, on `qemu-x86_64` and `hw-h002`.
- [ ] The physical-page identity of a transferred MemoryObject is unchanged, and MEM-012's check passes through this path.
- [ ] A message that names more handle slots than its Interface type permits is refused with the typed error at Send commit and allocates no handle in the receiver; a `share` of a MemoryObject is refused at V0.

#### Verification
- Unit: `kernel:tests/ipc/handle_transfer_*` on `qemu-x86_64` and `hw-h002`.
- Integration: V0 demo pipeline with MEM-012.

#### Evidence
- none

### IPC-015 · Switch directly to a waiting receiver on send without a run-queue round trip
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: IPC-001, IPC-016, SCH-005, TSK-020
- Baseline: §15, §53

The native IPC shape (§53) is that a send to a receiver already parked in Receive switches directly to that receiver's Task rather than marking it runnable and waiting for the scheduler to pick it. `jakeos/ipc/handoff.rs` implements the hook: at Send commit, if the peer endpoint has a Receive Operation outstanding whose Task is suspended on it (TSK-020's awaited-Operation record) and the receiver is eligible to run on the current CPU (same core, or the SCH-005 direct-switch rule allows a migration), the completion of the Receive is delivered and the CPU is handed to the receiver's execution context through SCH-005's direct-switch path with the D-0139 donation semantics if a call shape was chosen; otherwise the normal wake path (TSK-020) applies. OBS traces record `handoff` versus `wake` per Send so `os trace` and B-004 can attribute latency to the path taken.

The rejected shape from D-0139 (a blocking call syscall) does not exist in the ABI snapshot; B-004's same-core report includes the handoff configuration.

<!-- covers: INV-1007 -->

#### Out of scope
Scheduler class mapping (SCH-004). Task multiplexer (TSK-019). Intent inheritance across handoff (SCH-017, SCH-024). The direct-switch scheduler hook itself (SCH-005).

#### Deliverables
- kernel:jakeos/ipc/handoff.rs · Eligibility check at Send commit and the call into SCH-005's direct switch, with the `handoff` versus `wake` trace event.
- kernel:tools/testing/selftests/jakeos/ipc/handoff_*.rs · Selftests: handoff taken when the receiver is parked on the same core, wake path taken otherwise, no blocking entry involved.

#### Acceptance criteria
- [ ] When the receiver Task is parked in Receive on the same core, Send switches to that Task without an extra run-queue hop, shown by the `handoff` trace event on `qemu-x86_64` and `hw-h002`; when it is not, the `wake` event appears and the TSK-020 path is used.
- [ ] The path is the one D-0139 (IPC-001) named; the rejected blocking call shape is absent from the ABI snapshot (ABI-017).
- [ ] B-004 same-core reports include the handoff configuration as a labelled row.

#### Verification
- Unit: `kernel:tests/ipc/handoff_*` on `qemu-x86_64` and `hw-h002`.
- Bench: B-004 on H-001 and H-002; target per register.

#### Evidence
- none

### IPC-016 · Implement the selected minimal-copy small-message fast path
- Type: build
- Milestone: V0
- Status: todo
- Size: L
- Owner: none
- Depends on: IPC-003, IPC-010, IPC-007
- Baseline: §15, §53
- Invariants: I-066

The common small-message case needs no user-space serialise or deserialise step (§15, §53, I-066). `jakeos/ipc/fastpath.rs` implements the technique or combination D-0142 (IPC-003) selected (shared ring, register-carried, handoff-coupled, lock-free cross-core, or the recorded mix) for messages that fit inline per the D-0154 rule: the generated wire layout (IPC-012) is the in-memory layout, so the sender's stores land in the message slot and the receiver reads them in place; the kernel validates the header and handle slots (IPC-014) and never copies the payload. Messages above the inline rule take the MemoryObject path (IPC-014). The rejected techniques from D-0142 are not reachable from generated stubs.

A unit test counts payload copies between the sender's store and the receiver's load (zero on the fast path) using the kernel's copy accounting hooks; IPC-034 later turns that into a standing lint. IPC-008 measures this path for B-004 and B-005.

<!-- covers: INV-0293, INV-1002, INV-1004, INV-1324 -->

#### Out of scope
Technique selection (IPC-003). Batching productionisation (IPC-043). V1 tuning (IPC-054). Large-payload lowering (IPC-036).

#### Deliverables
- kernel:jakeos/ipc/fastpath.rs · The D-0142 technique on the inline path, header and slot validation, zero payload copies.
- kernel:jakeos/ipc/copy_account.rs · Copy accounting hook the copy-count test and IPC-034 read (debug builds only).
- kernel:tools/testing/selftests/jakeos/ipc/fast_path_*.rs · Selftests: zero copies on inline messages, MemoryObject path above the rule, rejected techniques unreachable.
- kernel:Documentation/jakeos/ipc/fastpath.md · The implemented technique and the inline rule as decided.

#### Acceptance criteria
- [ ] Small messages on the selected path have no user-space serialise or deserialise step: the copy-count test records zero payload copies between the sender's store and the receiver's load on `qemu-x86_64` and `hw-h002`.
- [ ] The implementation matches the technique D-0142 named; rejected techniques have no entry reachable from generated stubs (asserted by the ABI-017 snapshot).
- [ ] IPC-008 runs B-004 and B-005 against this path on H-001 and H-002.

#### Verification
- Unit: `kernel:tests/ipc/fast_path_*` on `qemu-x86_64` and `hw-h002`.
- Bench: B-004 and B-005 on H-001 and H-002; target per register.

#### Evidence
- none

### IPC-017 · Prototype and measure ring, GPR-carried, handoff and batched small-message fast paths
- Type: spike
- Milestone: V0
- Status: todo
- Size: L
- Owner: none
- Depends on: ABI-019, TSK-021, BLD-012, BEN-007
- Baseline: §15, §53, §58, §65
- Benchmarks: B-004, B-005
- Explores: S-012

IPC-003 and IPC-001 must be decided from one measured comparison on identical hardware (§15, §53, §58). Under `jakeos/spikes/ipc/fastpath/` behind `CONFIG_JAKEOS_SPIKES`, five techniques are prototyped far enough to ping-pong a small message between two processes: `ring.rs` (shared ring in a mapped page), `gpr.rs` (message in registers through the ABI-019 syscall prototype), `handoff.rs` (seL4 and LRPC-style direct switch to the receiver), `xcore.rs` (lock-free cross-core queues with IPI wake) and `batch.rs` (io_uring-style batched submission). The driver `runtime/spikes/ipc-fastpath-driver/` measures same-core and cross-core round trip (B-004 method) and one-way throughput (B-005 method) per BEN-064 on `qemu-x86_64` and `hw-h002`, with Unix-domain-socket and pipe ping-pong baselines in the same session.

The report `reports/spikes/IPC-017.md` gives separate same-core and cross-core tables per technique, cost, complexity, ABI impact (what each would make ABI on S-012) and the reject reasons, and names the candidates that remain for IPC-003. Publish-only numbers; S-012 is not frozen.

<!-- covers: INV-0294, INV-0295, INV-0297, INV-0298, GAP-0480, GAP-0481 -->

#### Out of scope
Selecting the production technique (IPC-003). Standing harness (IPC-008). Call semantics decision (IPC-001).

#### Deliverables
- kernel:jakeos/spikes/ipc/fastpath/ring.rs · Shared-ring prototype.
- kernel:jakeos/spikes/ipc/fastpath/gpr.rs · Register-carried prototype.
- kernel:jakeos/spikes/ipc/fastpath/handoff.rs · Direct-handoff prototype.
- kernel:jakeos/spikes/ipc/fastpath/xcore.rs · Lock-free cross-core queue prototype.
- kernel:jakeos/spikes/ipc/fastpath/batch.rs · Batched-submission prototype.
- runtime:spikes/ipc-fastpath-driver/ · Same-core and cross-core round trip and throughput driver with the two baselines.
- roadmap:reports/spikes/IPC-017.md · The report with per-technique tables and reject reasons.

#### Acceptance criteria
- [ ] Prototypes for shared ring, register-carried messages, scheduler-aware handoff, lock-free cross-core queues and batched submission run on `qemu-x86_64` and `hw-h002` and complete a ping-pong.
- [ ] `reports/spikes/IPC-017.md` records same-core and cross-core round trips separately for each prototype under the B-004 method, throughput under the B-005 method, and the Unix-domain-socket and pipe baselines from the same session, all labelled unpublished prototype measurements.
- [ ] The report names, per technique, its cost, complexity, what it would make ABI on S-012 and the reject reason if any, and states which techniques remain candidates for IPC-003.

#### Verification
- Report: `reports/spikes/IPC-017.md` answers cost, complexity, ABI impact and reject reasons per technique, with same-core and cross-core tables.
- Bench: B-004 and B-005 on H-001 and H-002; target per register (publish).

#### Evidence
- none

### IPC-018 · Study Cap'n Proto RPC, FIDL/Overnet, Genode and QNX before fixing the Channel wire model
- Type: spike
- Milestone: V0
- Status: todo
- Size: S
- Owner: none
- Depends on: none
- Baseline: §43, §58
- Explores: S-012, S-013

Before the IDL (IPC-006), the wire format (IPC-007) and the call shape (IPC-001) are fixed, the systems that solved the same problems are read (§58): Cap'n Proto RPC (capability passing over the wire, promise pipelining, zero-copy encoding), Fuchsia FIDL and Overnet (handle transfer, versioning, transport independence, §43), Genode (synchronous RPC with capability delegation, restartable servers) and QNX (message passing, resource managers, restart). The report `reports/spikes/IPC-018.md` describes each on Capability passing, synchronous versus asynchronous shapes, restart behaviour and wire encoding, scores each against the native requirements (ownership transfer, versioning, streams, multi-language codegen), and says what to take, what to reject and why, mapped to S-012 (Channel wire) and S-013 (message format) and to the three decisions.

<!-- covers: INV-0816, INV-1134 -->

#### Out of scope
IDL selection (IPC-006). Wasm Component Model study (WASM-002). Zircon object study (ABI-022).

#### Deliverables
- roadmap:reports/spikes/IPC-018.md · The study with per-system sections, the scoring table and the take-or-reject mapping to S-012, S-013, IPC-006, IPC-007 and IPC-001.

#### Acceptance criteria
- [ ] `reports/spikes/IPC-018.md` covers Cap'n Proto RPC, Fuchsia FIDL and Overnet, Genode and QNX on Capability passing, synchronous versus asynchronous shapes, restart and wire encoding, with sources cited by revision.
- [ ] Each system is scored against ownership transfer, versioning, streams and multi-language codegen in one table.
- [ ] The report's findings are mapped to S-012 and S-013 and cited by IPC-006, IPC-007 and IPC-001.

#### Verification
- Report: `reports/spikes/IPC-018.md` answers what to take, what to reject, and which questions remain for the three V0 decisions.
- Review: ABI lead sign-off recorded on the pull request.

#### Evidence
- none

### IPC-019 · Evolve one real V0 Interface through three incompatible revisions to exercise the versioning scheme
- Type: spike
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: IPC-012, IPC-013, BLD-082
- Baseline: §12
- Explores: S-014
- Risks: R-005

The evolution rules (IPC-002) must fail on a real evolution before any rule is frozen (§12, R-005). Under `idl/spikes/evolve/`, the V0 ImageDecoder Interface is taken through three revisions using the compiler and stubs: revision 1 adds an optional field to the request and a new optional method; revision 2 renames a field and changes a type (a breaking change under any scheme, to see how each candidate rule set reports it); revision 3 removes a method and adds a required field. For each of D-0141's three candidate rule sets, a prototype of the rule (a compiler flag in a spike branch) is applied and an old client is run against a new server and a new client against an old server on `qemu-x86_64`, recording which changes were compatible, forward-only or breaking, and where the candidate scheme failed to detect or handle a change.

The report `reports/spikes/IPC-019.md` is the evidence for IPC-002; no Layer 2 rule is frozen here.

<!-- covers: GAP-0521 -->

#### Out of scope
Accepting evolution rules (IPC-002). Permanent UI protocol bump test (IPC-040). The compiler (IPC-012).

#### Deliverables
- idl:spikes/evolve/ImageDecoder.v1.idl · Revision 1 (optional field, optional method).
- idl:spikes/evolve/ImageDecoder.v2.idl · Revision 2 (rename and type change).
- idl:spikes/evolve/ImageDecoder.v3.idl · Revision 3 (method removal, required field).
- idl:spikes/evolve/runner.rs · Runs old-client-new-server and new-client-old-server for each revision pair under each candidate rule set.
- idl:tests/evolve_*.rs · The three-revision fixture as a test that records the outcome matrix.
- roadmap:reports/spikes/IPC-019.md · The outcome matrix and where each candidate scheme failed.

#### Acceptance criteria
- [ ] The ImageDecoder Interface is evolved through the three revisions under `idl/spikes/evolve/` using the compiler and generated stubs, and `idl:tests/evolve_*` runs every old-and-new pairing for each candidate rule set.
- [ ] `reports/spikes/IPC-019.md` records, per revision and rule set, which changes were compatible, forward-only or breaking, and where the candidate scheme failed to detect or handle a change.
- [ ] The report's findings are inputs to IPC-002 and it freezes no Layer 2 rule.

#### Verification
- Report: `reports/spikes/IPC-019.md` answers how fields, methods and types evolved, where old clients broke, and which rule shapes remain viable.
- Integration: three-revision fixture in `idl:tests/evolve_*`.

#### Evidence
- none

### IPC-020 · Benchmark in-place Zero-copy access versus compact encode/decode including validation cost
- Type: spike
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: IPC-018, BEN-007
- Baseline: §14, §15, §53
- Benchmarks: B-004
- Explores: S-013

The wire format (IPC-007) and the inline-versus-MemoryObject threshold must be decided on numbers (§14, §15, §53). Under `idl/spikes/wire/`, the three D-0154 candidates are prototyped as encoders and decoders for a fixed set of representative message shapes (a small request with three scalar fields, a mid-size message with a string and a list, a large message with a nested structure and a byte array at the sizes the B-004 register names): `fixed.rs` (in-place layout with zero-copy field access), `selfdesc.rs` (tagged self-describing) and `indexed.rs` (schema-indexed with a compact table). The driver `runtime/spikes/ipc-wire-driver/` measures encode cost, decode cost and receiver-side validation cost (bounds, tags, handle-slot counts, the cost a hostile sender imposes) per candidate and shape on `qemu-x86_64` and `hw-h002` per BEN-064, and measures where inline transport stops beating a MemoryObject transfer as payload grows.

The report `reports/spikes/IPC-020.md` recommends a threshold rule without stating a public performance claim, and feeds IPC-007 and IPC-037 (validation hardening).

<!-- covers: GAP-0520, INV-0302 -->

#### Out of scope
Choosing the format (IPC-007). Production validation hardening (IPC-037). The transport (TSK-014).

#### Deliverables
- idl:spikes/wire/fixed.rs · Fixed-layout candidate encoder and decoder.
- idl:spikes/wire/selfdesc.rs · Self-describing candidate.
- idl:spikes/wire/indexed.rs · Schema-indexed candidate.
- idl:spikes/wire/shapes.rs · The representative message shapes and sizes.
- runtime:spikes/ipc-wire-driver/ · Encode, decode, validation and inline-versus-MemoryObject crossover measurements.
- roadmap:reports/spikes/IPC-020.md · Per-candidate costs, zero-copy properties, the crossover and the recommended threshold rule.

#### Acceptance criteria
- [ ] In-place zero-copy and compact encode and decode are measured for the representative shapes and sizes, including receiver-side validation under a hostile sender, on `qemu-x86_64` and `hw-h002`, labelled unpublished prototype measurements.
- [ ] `reports/spikes/IPC-020.md` recommends an inline-versus-MemoryObject threshold rule from the measured crossover without stating a public performance claim.
- [ ] The report's findings are inputs to IPC-007 and IPC-037.

#### Verification
- Report: `reports/spikes/IPC-020.md` answers encode, decode and validation cost per candidate, zero-copy properties, and the threshold heuristic.
- Bench: B-004 on H-001 and H-002; target per register (publish).

#### Evidence
- none

### IPC-021 · Add the version header and forward/backward unknown-field compatibility tests
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: IPC-007, IPC-002, IPC-012, ABI-004, BLD-082
- Baseline: §12, §65
- Invariants: I-041

Version negotiation exists from V0 (§12, I-041). Every message type the compiler emits carries the D-0141 version header in the D-0154 wire format: `idl/src/backend/rust/header.rs` emits the Interface version identity and the field table or index base the evolution rules need, and `ipc/src/compat.rs` in `jakeos-ipc-runtime` implements the receiver rule: a message with an unknown newer field is accepted by an older receiver (the field is skipped or preserved per D-0141) and an older message is accepted by a newer receiver (absent fields take their declared defaults). `idl/tests/wire_compat_*.rs` is the permanent test: the ImageDecoder Interface compiled at two revisions, each side decoding the other's messages; it is the message-level counterpart of ABI-004's Layer 1 handshake case, and the V0 exit criterion (V0-G05) cites both. The full schema-evolution feature set (optional methods, deprecation) is IPC-038.

<!-- covers: INV-0251, INV-0252 -->

#### Out of scope
Layer 1 handshake implementation (ABI-004). Optional methods and schema evolution (IPC-038). The rules decision (IPC-002).

#### Deliverables
- idl:src/backend/rust/header.rs · Version header emission per D-0141 on every message type (extending IPC-012's backend).
- ipc:src/compat.rs · Receiver-side unknown-field and absent-field handling per D-0141.
- idl:tests/wire_compat_*.rs · The permanent two-revision forward and backward compatibility test.
- kernel:tools/testing/selftests/jakeos/ipc/wire_compat_*.rs · The same case run end to end over a real Channel on both matrix entries.

#### Acceptance criteria
- [ ] A message with an unknown newer field is accepted by an older receiver and an older message is accepted by a newer receiver, in `idl:tests/wire_compat_*` on host CI and over a real Channel on `qemu-x86_64` and `hw-h002`.
- [ ] The version header is present on every generated message type used by the V0 demo, asserted by a test over the generated wire types.
- [ ] `idl:tests/wire_compat_*` is retained permanently, is the message-level counterpart of ABI-004's conformance case 0, and both are cited by V0-G05.

#### Verification
- Unit: `idl:tests/wire_compat_*` on host CI.
- Integration: V0 exit unknown-field case on `qemu-x86_64` and `hw-h002`.

#### Evidence
- none

### IPC-022 · Decide the relationship between the native IDL and WIT
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: IPC-006, WASM-002
- Baseline: §13, §14
- Decision: D-0149

Same language, bidirectional mapping or independent; must precede the V1 Wasm-component-on-native-Channel prototype owned by WASM (§13). Native machine code remains first-class; Wasm is not the Native ABI.

<!-- covers: INV-0276, GAP-0522 -->

#### Out of scope
Wasm runtime selection (WASM-007). WASI imports (WASM-008). Channel mapping implementation (WASM-013).

#### Acceptance criteria
- [ ] Option A (native IDL is WIT), option B (bidirectional mapping between native IDL and WIT), and option C (independent languages with an explicit bridge) are evaluated against Capability passing and versioning.
- [ ] The Decision states what WASM-013 may assume and that native Components are not forced into Wasm.
- [ ] WASM lead records Review sign-off on the pull request.

#### Verification
- Review: WASM lead sign-off recorded on the pull request.

#### Evidence
- none

### IPC-023 · Decide service naming and discovery: kernel-held directory or user-space broker
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: IPC-010, CAP-022, SVC-004
- Baseline: §32, §14
- Decision: D-0150

Clients must rebind by Interface identity across restarts for the V0.5 compositor crash-recovery Gate; decided with SVC supervision semantics (§32). CAP-022 covers how a Component obtains Capabilities; this Decision covers how a client finds a named Interface.

<!-- covers: INV-0609 -->

#### Out of scope
Supervisor restart policy (SVC-005). Generated rebind stubs (IPC-028). Capability bootstrap (CAP-022).

#### Acceptance criteria
- [ ] Option A (kernel-held directory of Interface identities) and option B (user-space broker Component) are evaluated against restart, attenuation and inspectability.
- [ ] The Decision states how a client re-resolves by Interface identity after peer death and what is visible in `os inspect`.
- [ ] SVC and CAP leads record Review sign-off on the pull request.

#### Verification
- Review: SVC and CAP leads sign off on the pull request.

#### Evidence
- none

### IPC-024 · License IDL files and the ABI specification under a permissive spec license with patent non-assert
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: GOV-003, IPC-006
- Baseline: §14, §66
- Decision: D-0151

Layer 2 Interface definitions first appear in V0.5 (compositor, package, storage); the license must be settled before third parties see them. A decades-long ABI must be reimplementable without legal exposure. Reviewed with GOV.

<!-- covers: GAP-0008 -->

#### Out of scope
Generated stub license (IPC-005). ABI header exception (ABI-029).

#### Acceptance criteria
- [ ] Option A (permissive specification license plus royalty-free patent non-assert), option B (the Layer 2 userspace license from GOV-003 with no extra patent grant), and option C (CC0-class dedication) are evaluated with GOV.
- [ ] The Decision names the license text applied to IDL files and the ABI specification.
- [ ] GOV lead records Review sign-off on the pull request.

#### Verification
- Review: GOV lead sign-off recorded on the pull request.

#### Evidence
- none

### IPC-025 · Decide the pluggable transport abstraction behind generated stubs
- Type: adr
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: IPC-013, IPC-034
- Baseline: §43, §57
- Decision: D-0152
- Invariants: I-047

Fixes how generated stubs bind to same-Component, same-machine (default) and later VM transports without regenerating Interfaces (§43). Distribution itself stays out of the kernel and remote transports remain LATER (I-047).

<!-- covers: INV-0815, INV-0803, INV-0805, INV-0806 -->

#### Out of scope
In-process implementation (IPC-030). VM transport (IPC-058). Remote-machine prototype (IPC-071).

#### Acceptance criteria
- [ ] Option A (pluggable transport trait behind generated stubs), option B (compile-time transport selection per Interface), and option C (a single same-machine transport with later forks) are evaluated against re-emit cost and I-047.
- [ ] The Decision names the default transport and forbids kernel-side remote-machine logic.
- [ ] ABI lead records Review sign-off on the pull request.

#### Verification
- Review: ABI lead sign-off recorded on the pull request.

#### Evidence
- none

### IPC-026 · Define graceful close, drain and peer-closed delivery for Channel endpoints
- Type: build
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: IPC-011, IPC-010, IPC-009
- Baseline: §32, §12

Service restart and client rebind (V0.5 exit) need deterministic close ordering: in-flight messages drained or failed with a typed result, peer-closed observable as an Operation, no lost handles on close (§32).

<!-- covers: INV-0259 -->

#### Out of scope
Generated rebind (IPC-028). Handle transfer (IPC-014). Supervisor death detection (SVC).

#### Acceptance criteria
- [ ] Closing a sender endpoint drains or fails in-flight messages with a typed result; no Capability handle in the queue is leaked.
- [ ] The receiver observes peer-closed as an Operation completion, not as a hang.
- [ ] Close of both ends reclaims queue memory charged to the ResourceDomain.

#### Verification
- Unit: `kernel:tests/ipc/close_*` on `qemu-x86_64` and `hw-h002`.
- Integration: compositor kill/rebind fixture with SVC-002.

#### Evidence
- none

### IPC-027 · Charge Channel queue memory and handle slots to the owning ResourceDomain
- Type: build
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: IPC-009, SCH-009, SCH-008
- Baseline: §23
- Risks: R-074
- Threats: T-016

SCH scope includes kernel-object limits; queued messages must not evade the ResourceDomain memory budget proven in V0, and bounded depth from IPC-009 needs an accounting home before real applications ship in V0.5 (§23).

<!-- covers: EXTRA-002 -->

#### Out of scope
Budget policy and exhaustion (SCH-016, SCH-008). Backpressure policy (IPC-009).

#### Acceptance criteria
- [ ] Queued message bytes and handle slots are charged to the sending Component's ResourceDomain.
- [ ] Exceeding the domain memory or object limit completes send with a typed error and does not grow the queue.
- [ ] `os inspect resource` shows Channel queue consumption attributed to the domain.

#### Verification
- Unit: `kernel:tests/ipc/queue_charge_*` on `qemu-x86_64` and `hw-h002`.
- Integration: over-budget send under SCH-001.

#### Evidence
- none

### IPC-028 · Generate client-side disconnect, rebind and retry support for restartable services
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: IPC-023, IPC-026, IPC-013, IPC-011, SVC-009
- Baseline: §32, §12
- Invariants: I-037

V0.5 exit: killing the compositor rebinds all windows with no application exit. Generated proxies observe disconnect, re-resolve by Interface identity via the discovery mechanism and re-establish per the Interface's declared restart policy (§32).

<!-- covers: INV-0591, INV-0609 -->

#### Out of scope
Supervisor respawn (SVC-015). SDK reconnect library wrapping (SDK-012). Surface persistence (GFX).

#### Acceptance criteria
- [ ] Generated proxies observe peer-closed, re-resolve the Interface identity, and obtain a new Channel without the client Component exiting.
- [ ] Idempotent methods retry per the Interface restart policy; non-idempotent methods complete with a typed disconnect.
- [ ] The compositor-rebind Gate runs against these proxies on `qemu-x86_64` and H-002.

#### Verification
- Unit: `idl:tests/rebind_*` on host CI.
- Integration: SVC-002 on `qemu-x86_64` and `hw-h002`.

#### Evidence
- none

### IPC-029 · Emit fuzz harnesses and structure-aware mutators from the IDL compiler
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: IPC-012, IPC-007
- Baseline: §14, §51

Every typed IPC boundary is a trust boundary; harnesses generated for each Interface and wire format feed BLD's fuzzing pipeline and the V3 continuous IPC fuzzing Gate.

<!-- covers: GAP-0129 -->

#### Out of scope
Kernel Channel syscall fuzz (IPC-044). Fuzz fleet (BLD-035). Compiler front-end fuzz (IPC-060).

#### Acceptance criteria
- [ ] The compiler emits a structure-aware mutator and harness for every compiled Interface and wire format.
- [ ] Each harness builds against BLD-016 and is listed in the IPC fuzz inventory.
- [ ] A malformed message is rejected by generated validation without kernel panic.

#### Verification
- Unit: `idl:tests/fuzz_emit_*` on host CI.
- Fuzz: generated harness for ImageDecoder for one CI cycle without panic.

#### Evidence
- none

### IPC-030 · Implement the same-Component transport for generated Interface stubs
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: IPC-025, IPC-013
- Baseline: §43
- Invariants: I-047

First alternate transport proving IPC-025: an Interface served in-process with identical application semantics, used by Component graphs and the toolkit (§43).

<!-- covers: INV-0805 -->

#### Out of scope
Same-machine kernel Channel (IPC-010). VM transport (IPC-058). Graph wiring (CMP-024).

#### Acceptance criteria
- [ ] An Interface served in-process uses the same generated stubs as a cross-Component Channel.
- [ ] Client-visible errors, cancellation and handle transfer match the same-machine transport.
- [ ] Stubs do not hard-code in-process; transport is selected per IPC-025.

#### Verification
- Unit: `idl:tests/in_process_transport_*` on host CI.
- Integration: toolkit or Component-graph fixture on `qemu-x86_64`.

#### Evidence
- none

### IPC-031 · Generate typed Inputs<T> and Outputs<T> endpoint bundles for Components
- Type: build
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: IPC-012, IPC-013
- Baseline: §10, §14

Components declare typed receiving and sending endpoints; the Inputs/Outputs manifest moves to V0.5 with CMP Component graphs, so codegen lands here and CMP consumes the types in the manifest (§10).

<!-- covers: INV-0224, INV-0225 -->

#### Out of scope
Manifest binding at launch (CMP-025). Graph instantiation (CMP-024).

#### Acceptance criteria
- [ ] The compiler emits typed Inputs and Outputs bundles from IDL endpoint declarations.
- [ ] Generated types are usable as Component endpoint fields without a Channel socket shape.
- [ ] CMP-025 compiles against the emitted types.

#### Verification
- Unit: `idl:tests/endpoints_*` on host CI.
- Integration: CMP manifest fixture on host CI.

#### Evidence
- none

### IPC-032 · Publish Interface design guidelines for IDL authors including failure and restart semantics
- Type: docs
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: IPC-002, IPC-011
- Baseline: §12, §32
- Invariants: I-037

Naming, error taxonomy, async and stream patterns, Capability-passing idioms and required client behavior on service loss (disconnect, rebind, retry, restore-state), reviewed before the first Layer 2 Interface freezes (§12, §32).

<!-- covers: EXTRA-033, INV-0591 -->

#### Out of scope
Lint enforcing the guidelines (IPC-050). Evolution guidelines (IPC-053). Docs site (DOC).

#### Acceptance criteria
- [ ] The guide covers naming, error taxonomy, async and stream patterns, Capability-passing idioms, and client behavior on service loss.
- [ ] Failure and restart semantics are required sections, not optional appendices (I-037).
- [ ] DOC and SDK leads record Review sign-off before the first Layer 2 Interface ships.

#### Verification
- Review: DOC and SDK leads sign off on the pull request.

#### Evidence
- none

### IPC-033 · Generate Interface version identities and negotiation code
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: IPC-002, IPC-012, ABI-016
- Baseline: §12, §14
- Invariants: I-041

Per-Interface version identity and generated negotiation at connect time per IPC-002; required by the V0.5 exit criterion that bumps the UI protocol v0 to v0.1 with old clients still running (§12, §14).

<!-- covers: INV-0249, INV-0289 -->

#### Out of scope
UI protocol bump test (IPC-040). Feature negotiation (IPC-045). Layer 1 handshake (ABI-004).

#### Acceptance criteria
- [ ] Each compiled Interface carries a version identity on the wire.
- [ ] Generated connect-time negotiation accepts the overlap named by IPC-002 and fails with a typed error outside it.
- [ ] Old v0 clients still connect after a v0.1 bump in IPC-040.

#### Verification
- Unit: `idl:tests/version_nego_*` on host CI.
- Integration: UI protocol v0 to v0.1 case on `qemu-x86_64`.

#### Evidence
- none

### IPC-034 · Add CI lints for IPC non-goals, transport-agnostic stubs and generator determinism
- Type: build
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: IPC-012, IPC-004, ABI-003
- Baseline: §14, §43, §53, §57
- Invariants: I-047

Standing rules enforced once: no sockets, raw byte payloads, per-service serialization, hand-written protocols or duplicated schemas in native code; generated stubs must not assume same-process or same-machine endpoints and no distribution logic may enter the kernel; generator output is deterministic and matches checked-in IDL (I-047).

<!-- covers: INV-0281, INV-0282, INV-0283, INV-0284, INV-0285, INV-1002, INV-1003, INV-1004, INV-0804, INV-1120, INV-0803, INV-0805, INV-0806, GAP-0098 -->

#### Out of scope
ABI personality firewall (ABI-003). Fuzz targets (IPC-044). Transport implementations (IPC-030).

#### Acceptance criteria
- [ ] CI fails a native crate that uses sockets, untyped byte payloads, a per-service serializer, a hand-written protocol or a duplicated schema.
- [ ] CI fails generated stubs that hard-code same-process or same-machine endpoints, and fails any kernel patch that adds remote-machine logic.
- [ ] CI fails when generator output is non-deterministic or disagrees with the IDL under the chosen in-tree policy.

#### Verification
- Unit: `idl:tests/lint_nongoals_*` and `idl:tests/determinism_*` on host CI.
- Integration: negative fixtures in BLD-011.

#### Evidence
- none

### IPC-035 · Register Layer 2 core platform Interfaces with strong versions and a CI version check
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: IPC-033, IPC-024, IPC-012
- Baseline: §66, §12

Compositor, storage, network, audio and package Interfaces are declared in the IDL with version identities; CI rejects an Interface change without a version bump or evolution-rule compliance (§66).

<!-- covers: INV-1285 -->

#### Out of scope
Owning those services (GFX, STO, NET, AUD, PKG). Evolution diff tool (IPC-052). Version lock (IPC-068).

#### Acceptance criteria
- [ ] Core platform Interfaces are listed with version identities in the IDL tree.
- [ ] CI fails an Interface change that lacks a version bump or violates IPC-002.
- [ ] Each listed Interface file carries the specification license from IPC-024.

#### Verification
- Unit: `idl:tests/l2_registry_*` on host CI.
- Integration: CI check in BLD-011.

#### Evidence
- none

### IPC-036 · Lower large and variable-size payload types to MemoryObject transfer in codegen
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: IPC-007, IPC-012, IPC-014, MEM-010, IPC-034
- Baseline: §15, §16
- Invariants: I-063

The IDL must make MemoryObject transfer the natural encoding for large data; the compiler applies the IPC-007 threshold so image, buffer and file payloads in the V0.5 apps never move bytes (§15).

<!-- covers: INV-0301, INV-0302 -->

#### Out of scope
MemoryObject backing (MEM). Zero-copy API lint (MEM-032). Inline small messages (IPC-016).

#### Acceptance criteria
- [ ] Payload types above the Decision threshold are emitted as MemoryObject moves, not inline bytes.
- [ ] Image, buffer and file payloads used by V0.5 apps take this path; physical-page identity is unchanged after the call.
- [ ] IPC-034 fails an IDL method that inlines a large byte array where a MemoryObject move is possible.

#### Verification
- Unit: `idl:tests/lowering_*` on host CI.
- Integration: Image Viewer decode path with MEM-022.

#### Evidence
- none

### IPC-037 · Harden receiver-side wire validation for bounds, handle counts and type tags
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: IPC-007, IPC-014, SEC-002
- Baseline: §14, §51
- Threats: T-025

Typed IPC boundaries are the trust boundaries between Components; validation cost measured in IPC-020 is spent here with hostile-message tests before untrusted Wayland-bridge and package clients appear in V0.5. Required by V4-G04 (External security audit High and Critical closed): the audit covers IPC, and receiver-side validation is what keeps hostile-message findings out of the High and Critical classes.

#### Out of scope
Threat model document (SEC-002). Generated fuzz mutators (IPC-029).

#### Acceptance criteria
- [ ] Out-of-bounds lengths, wrong handle counts and unknown type tags are rejected with a typed error and allocate no handle.
- [ ] Hostile-message tests cover truncated, oversized and tag-confused payloads without kernel panic.
- [ ] Validation runs on the receiver before any Capability is installed.

#### Verification
- Unit: `kernel:tests/ipc/validate_*` on `qemu-x86_64` and `hw-h002`.
- Fuzz: hostile-message corpus for one CI cycle without panic.

#### Evidence
- none

### IPC-038 · Support optional methods and forward/backward schema evolution in the IDL
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: IPC-002, IPC-012, IPC-021
- Baseline: §12

Adding fields without breaking old receivers, unknown-field preservation by new receivers and optional methods with typed unsupported results, as exercised by the V0.5 UI protocol bump (§12).

<!-- covers: INV-0250, INV-0251, INV-0252 -->

#### Out of scope
UI protocol bump regression (IPC-040). Feature sets (IPC-045).

#### Acceptance criteria
- [ ] Adding a field does not break old receivers; new receivers preserve unknown fields.
- [ ] An optional method missing on the server completes with a typed unsupported result.
- [ ] The v0 to v0.1 UI protocol bump in IPC-040 uses these features.

#### Verification
- Unit: `idl:tests/schema_evo_*` on host CI.
- Integration: UI protocol v0.1 case on `qemu-x86_64`.

#### Evidence
- none

### IPC-039 · Support stream (multi-value) results with flow control in the IDL and runtime
- Type: build
- Milestone: V0.5
- Status: todo
- Size: M
- Owner: none
- Depends on: IPC-009, IPC-013, IPC-012
- Baseline: §12, §14

Compositor frame events, input events and file listings in V0.5 need multi-value results; stream flow control reuses Channel backpressure policy (§12).

<!-- covers: INV-0255 -->

#### Out of scope
Backpressure policy (IPC-009). Frame scheduling (GFX). Input routing (UIP).

#### Acceptance criteria
- [ ] IDL stream results generate a typed multi-value client API backed by Channel receive.
- [ ] Stream flow control uses the Channel backpressure policy; a slow consumer does not unbounded-buffer on the producer.
- [ ] A compositor frame-event stream and a file-listing stream run in V0.5 fixtures.

#### Verification
- Unit: `idl:tests/streams_*` on host CI.
- Integration: compositor event stream on `qemu-x86_64`.

#### Evidence
- none

### IPC-040 · Add the permanent UI protocol v0 to v0.1 Interface-versioning regression test
- Type: build
- Milestone: V0.5
- Status: todo
- Size: S
- Owner: none
- Depends on: IPC-038, IPC-033, UIP-015
- Baseline: §12, §41

V0.5 exit criterion: Interface versioning exercised end to end by bumping the UI protocol with an added optional method while old clients still run, retained permanently as a regression test; IPC verifies its own Gate with UIP.

#### Out of scope
UI protocol IDL (UIP-013). Toolkit (UIP).

#### Acceptance criteria
- [ ] UI protocol v0 is bumped to v0.1 by adding an optional method; v0 clients still run.
- [ ] The test is retained in CI and fails if old clients stop connecting.
- [ ] The bump uses optional methods and version negotiation from this workstream.

#### Verification
- Integration: `ipc:tests/ui_protocol_v0_v01_*` on `qemu-x86_64` and `hw-h002`.
- Unit: generated v0 client against v0.1 server in `idl:tests/ui_bump_*`.

#### Evidence
- none

### IPC-041 · Decide which Channel syscalls become Layer 1 freeze candidates for SDK v1
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: ABI-034, IPC-010, IPC-016, IPC-014, IPC-009, IPC-008, IPC-017
- Baseline: §65, §66
- Decision: D-0140
- Risks: R-007
- Invariants: I-040

L1 ABI surfaces are prototyped through V0, freeze candidates at V1, frozen at V4. IPC names its candidate entry points and what stays behind user-space Interfaces, feeding ABI's freeze process (§65, §66). Nothing L1 is frozen here (I-040). Required by V4-G01 (Layer 1 ABI frozen with a conformance suite): the V4 freeze of S-012 starts from the candidate list this Decision records.

#### Out of scope
V4 freeze (IPC-064). ABI candidate review process (ABI-034).

#### Acceptance criteria
- [ ] Option A (create, send, receive, close, handle-transfer and inspect as L1 candidates), option B (a reduced send/receive/close core with handle-transfer at L2), and option C (defer candidacy to V2) are evaluated against S-012 and the V0 spike and benchmark reports.
- [ ] The Decision lists each candidate entry point, its spike and B-004/B-005 reports, and what remains a user-space Interface.
- [ ] ABI lead records Review sign-off on the pull request.

#### Verification
- Review: ABI lead sign-off recorded on the pull request.

#### Evidence
- none

### IPC-042 · Decide whether the Layer 2 Interface-evolution rules freeze at V1 with SDK v1
- Type: adr
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: IPC-002, IPC-038, IPC-040, IPC-019
- Baseline: §12, §66
- Decision: D-0144
- Risks: R-005
- Invariants: I-041

L2 evolution rules freeze at V1 with SDK v1, after the V0 prototype and V0.5 UI protocol bump have exercised them; options include deferring the freeze to V2.

<!-- covers: INV-0247, INV-0260, INV-0262 -->

#### Out of scope
Diff tool (IPC-052). Published guidelines (IPC-053). L2 version lock (IPC-068).

#### Acceptance criteria
- [ ] Option A (freeze the V0 prototyped rules as S-014 at V1) and option B (keep prototyped and freeze at V2) are evaluated against the UI protocol bump and the three-revision spike.
- [ ] If option A is taken, S-014 is named as freeze candidate with the spike and this Decision in its closure.
- [ ] ABI and SDK leads record Review sign-off on the pull request.

#### Verification
- Review: ABI and SDK leads sign off on the pull request.

#### Evidence
- none

### IPC-043 · Implement batched Channel send and receive submission over Operations
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: IPC-017, IPC-010, TSK-030, IPC-016
- Baseline: §15, §18
- Benchmarks: B-006

Productionises the batching measured in IPC-017 for high-rate streams (compositor frames, audio) that gate V1, using TSK's Operation submission path (§15, §18).

<!-- covers: INV-0297 -->

#### Out of scope
Operation batch ABI (TSK-007, TSK-030). Stream IDL (IPC-039).

#### Acceptance criteria
- [ ] Send and receive submit as batched Operations on a Channel; completion order matches the batch links TSK defines.
- [ ] A compositor frame stream and an audio stream fixture use this path.
- [ ] B-006 reports include the batched configuration on H-002.

#### Verification
- Unit: `kernel:tests/ipc/batch_*` on `qemu-x86_64` and `hw-h002`.
- Bench: B-006 on H-002; target per Register.

#### Evidence
- none

### IPC-044 · Add structure-aware fuzz targets for the Channel syscall Surface
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: IPC-010, IPC-014, IPC-026, BLD-016, BLD-035
- Baseline: §14, §51

Kernel-side counterpart of the generated Interface fuzzers: targets for Channel create, send, receive, handle transfer and close, handed to BLD's fuzzing infrastructure and required by the V3 continuous IPC fuzzing Gate.

<!-- covers: GAP-0129 -->

#### Out of scope
IDL-emitted harnesses (IPC-029). Fuzz fleet (BLD). Coverage Gate report (IPC-061).

#### Acceptance criteria
- [ ] Targets exist for create, send, receive, handle transfer and close.
- [ ] Each target is registered with BLD-035 and has a structure-aware mutator.
- [ ] A CI cycle on the targets produces no kernel panic.

#### Verification
- Fuzz: Channel syscall targets on BLD's Native ABI fuzzing for one CI cycle without panic.
- Unit: `kernel:fuzz/channel_*` builds on host CI.

#### Evidence
- none

### IPC-045 · Implement feature negotiation between Interface endpoints
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: IPC-002, IPC-033, IPC-042
- Baseline: §12

Endpoints advertise and agree optional feature sets at connect time per IPC-002; needed by SDK v1 so applications built against v1.0.0 keep working across v1.x services (§12).

<!-- covers: INV-0253 -->

#### Out of scope
Version identity (IPC-033). SDK compatibility suite (SDK-036).

#### Acceptance criteria
- [ ] Endpoints advertise feature sets at connect; agreement is the intersection named by the evolution rules.
- [ ] A client built against a subset of features runs against a server that added features.
- [ ] Unknown features are ignored or rejected with a typed error per the Decision, never as a hang.

#### Verification
- Unit: `idl:tests/features_*` on host CI.
- Integration: SDK v1 compatibility case on `qemu-x86_64`.

#### Evidence
- none

### IPC-046 · Freeze Layer 2 Interface-evolution rules for SDK v1
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: IPC-019, IPC-042, IPC-052
- Baseline: §12, §66
- Freezes: S-014
- Invariants: I-040

V1 freezes Layer 2 Interface-evolution rules S-014 after the versioning spike and accepted Decision (§12, §66). Core Interface versions lock later at V4. This task is the freeze record and the CI rule that optional methods and deprecations follow the accepted rules.

#### Out of scope
The Decision (IPC-042). V4 version lock (IPC-068). Layer 1 freeze (ABI).

#### Acceptance criteria
- [ ] Surface S-014 is listed as frozen by this task in the surfaces register.
- [ ] CI rejects an Interface change that violates the accepted evolution rules.
- [ ] No Layer 1 surface is marked frozen by this task (I-040).

#### Verification
- Integration: `ipc:tests/l2/evolution_rules_freeze_*` on `qemu-x86_64`.
- Review: IPC and ABI leads sign off on the pull request that lands the freeze.

#### Evidence
- none

### IPC-047 · Define the IDL compiler backend Interface so SDK languages add codegen targets
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: IPC-012
- Baseline: §14, §50

Codegen must target every supported SDK language (§50); a stable backend API over the compiler's typed IR lets SDK add languages without touching the front end.

<!-- covers: INV-0946 -->

#### Out of scope
C backend (IPC-048). Remaining languages (IPC-057). Language order Decision (SDK-024).

#### Acceptance criteria
- [ ] The compiler exposes a typed IR and a backend API sufficient to emit wire layout, stubs and ownership for a new language.
- [ ] The Rust backend is expressed through this API; a second backend can be added without front-end changes.
- [ ] API documentation lists the IR nodes a backend must handle.

#### Verification
- Unit: `idl:tests/backend_api_*` on host CI.
- Review: SDK lead sign-off recorded on the pull request.

#### Evidence
- none

### IPC-048 · Generate C bindings from the IDL for the Layer 1 ABI and core interfaces
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: IPC-047, SDK-024, ABI-007
- Baseline: §14, §50

V1 scope: C bindings for the Layer 1 ABI ship with SDK v1; C is the second codegen target and validates IPC-047.

<!-- covers: INV-0946 -->

#### Out of scope
Safe C wrappers and packaging (SDK-033, SDK-034). Other languages (IPC-057).

#### Acceptance criteria
- [ ] The C backend emits headers and stubs for Layer 1 Channel entry points and core Layer 2 Interfaces.
- [ ] Emitted headers carry the generated-code license exception.
- [ ] A C fixture round-trips a typed message against a Rust server.

#### Verification
- Unit: `idl:tests/c_backend_*` on host CI.
- Integration: C-to-Rust Channel fixture on `qemu-x86_64`.

#### Evidence
- none

### IPC-049 · Carry doc comments and semantic metadata through the IDL compiler IR
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: IPC-012
- Baseline: §14, §56.5

IDL-to-docs emission must exist at V1 with SDK v1 (DOC owns the generator); the compiler must preserve documentation and deprecation metadata for it. Required by V1-G12 (Semantic interfaces and a Wasm channel prototype): IDL-to-docs generation reads doc comments and deprecation metadata from this IR.

#### Out of scope
Docs generator and site (DOC-010). Semantic verb annotations (IPC-051).

#### Acceptance criteria
- [ ] Doc comments and deprecation metadata survive parse and appear on the typed IR.
- [ ] DOC-010 can read the IR and emit a page for a sample Interface.
- [ ] Stripping comments is not the default; CI fails if comments are dropped.

#### Verification
- Unit: `idl:tests/doc_ir_*` on host CI.
- Integration: DOC generator fixture on host CI.

#### Evidence
- none

### IPC-050 · Add an IDL lint enforcing the Interface design guidelines
- Type: build
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: IPC-032, IPC-012
- Baseline: §12, §32

Turns the V0.5 guidelines into CI checks (naming, error taxonomy, stream and Capability-passing idioms) before third parties author Interfaces against SDK v1.

<!-- covers: EXTRA-033 -->

#### Out of scope
Guideline prose (IPC-032). Evolution diff (IPC-052).

#### Acceptance criteria
- [ ] The lint fails Interfaces that violate naming, error taxonomy, stream or Capability-passing rules in the guidelines.
- [ ] The lint runs in CI on every Layer 2 Interface in the tree.
- [ ] A documented suppressions path does not exist for core platform Interfaces.

#### Verification
- Unit: `idl:tests/lint_guidelines_*` on host CI.
- Integration: BLD-011.

#### Evidence
- none

### IPC-051 · Add IDL annotations for Semantic interfaces consumed by the SEM catalog
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: IPC-012, IPC-032, SEM-003
- Baseline: §42, §57

V1 exit requires Semantic interface v0 for Terminal and Editor; SEM owns the catalog but needs IDL attributes for semantic verbs, Object types and automation exposure, kept in dependency order catalog before AI broker (§42, §57).

<!-- covers: INV-0788 -->

#### Out of scope
Catalog service (SEM-007, SEM-006). AI broker (SEM-010).

#### Acceptance criteria
- [ ] IDL accepts annotations for semantic verbs, object types and automation exposure.
- [ ] Annotated Interfaces compile to the same wire format as unannotated ones; annotations are metadata only.
- [ ] SEM-006 can consume the annotations without a second schema.

#### Verification
- Unit: `idl:tests/semantic_ann_*` on host CI.
- Integration: Terminal.run and Editor.open IDL fixtures used by SEM-008.

#### Evidence
- none

### IPC-052 · Build the Interface compatibility tool that diffs two IDL versions
- Type: build
- Milestone: V1
- Status: todo
- Size: M
- Owner: none
- Depends on: IPC-042, IPC-012, IPC-035
- Baseline: §12
- Risks: R-005

Classifies changes as compatible, forward-only or breaking against the frozen evolution rules and runs in CI on every Layer 2 Interface change (§12).

<!-- covers: INV-0262 -->

#### Out of scope
Published evolution guidelines (IPC-053). L2 evolution test matrix (IPC-062).

#### Acceptance criteria
- [ ] Diffing two IDL versions reports compatible, forward-only or breaking per IPC-042.
- [ ] CI fails a breaking Layer 2 change that does not bump the version identity.
- [ ] The UI protocol v0 to v0.1 bump is classified as compatible.

#### Verification
- Unit: `idl:tests/diff_*` on host CI.
- Integration: CI hook on Layer 2 Interface changes.

#### Evidence
- none

### IPC-053 · Publish Interface evolution guidelines for SDK authors
- Type: docs
- Milestone: V1
- Status: todo
- Size: S
- Owner: none
- Depends on: IPC-042, IPC-052
- Baseline: §12

Companion to the frozen rules and diff tool for SDK v1 authors; DOC publishes, IPC authors the content (§12).

<!-- covers: INV-0262 -->

#### Out of scope
Docs site (DOC). Diff tool (IPC-052). Design guidelines (IPC-032).

#### Acceptance criteria
- [ ] The guide explains compatible, forward-only and breaking changes with examples from the frozen rules.
- [ ] It points authors at IPC-052 and the deprecation overlap policy.
- [ ] SDK and DOC leads record Review sign-off on the pull request.

#### Verification
- Review: SDK and DOC leads sign off on the pull request.

#### Evidence
- none

### IPC-054 · Tune the IPC fast path to the V1 same-core and cross-core round-trip targets
- Type: build
- Milestone: V1
- Status: todo
- Size: L
- Owner: none
- Depends on: IPC-016, IPC-015, IPC-043, IPC-008, TSK-046, BEN-029
- Baseline: §14, §53, §54
- Benchmarks: B-004, B-005
- Invariants: I-061

V1 benchmark Gate applies the first absolute B-004 and B-005 targets on H-002, published beside Linux UDS and futex ping-pong. This is the IPC-side tuning; numbers live only in the Register.

<!-- covers: INV-0277 -->

#### Out of scope
Harness (IPC-008). Merge-gate policy (BEN-033). Scheduler multiplexer (TSK-046).

#### Acceptance criteria
- [ ] B-004 and B-005 reports on H-002 meet the V1 absolute target kind in the Register, or an accepted Decision explains the miss.
- [ ] Linux Unix-domain-socket and futex ping-pong appear in the same reports.
- [ ] No public material states a number except by citing B-004 or B-005.

#### Verification
- Bench: B-004 and B-005 on H-001, H-002 and H-004; target per Register.
- Integration: BEN-029 includes these reports.

#### Evidence
- none

### IPC-055 · Decide how the IDL language itself is versioned
- Type: adr
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: IPC-006, IPC-042
- Baseline: §14, §66
- Decision: D-0147

V3 opens the public repository to third-party packages authoring Interfaces; a language version pragma and compatibility policy for the compiler must exist before that. Required by V3-G10 (Kernel and IPC fuzzing has no stale open crasher): the IDL front-end fuzz harness in that gate's closure fails closed on the language versions this Decision defines.

#### Out of scope
Compiler fuzz of third-party files (IPC-060). Interface evolution rules (IPC-042).

#### Acceptance criteria
- [ ] Option A (language version pragma with a published compatibility window), option B (compiler major version as the language version), and option C (edition flags per file) are evaluated against third-party packages.
- [ ] The Decision states how an older compiler treats a newer pragma and the reverse.
- [ ] SDK lead records Review sign-off on the pull request.

#### Verification
- Review: SDK lead sign-off recorded on the pull request.

#### Evidence
- none

### IPC-056 · Decide how Capabilities and handles cross a VM transport boundary
- Type: adr
- Milestone: V2
- Status: todo
- Size: S
- Owner: none
- Depends on: IPC-025, CAP-006, VIRT-002
- Baseline: §43, §8
- Decision: D-0153

Native guest Components talking to host services need proxied Capability semantics that honor attenuation and revocation (§43); precedes IPC-058 and VIRT's guest tools.

<!-- covers: INV-0807 -->

#### Out of scope
Prototype (IPC-058). Cross-machine unforgeability (CAP-047). Remote-machine transport (IPC-071).

#### Acceptance criteria
- [ ] Option A (proxied handles with host-side attenuation and revocation), option B (cryptographic Capabilities valid across the VM boundary), and option C (no Capability crossing; MemoryObject and data only) are evaluated against §8 and §43.
- [ ] The Decision states what IPC-058 may implement and that the kernel is not a distributed system.
- [ ] CAP and VIRT leads record Review sign-off on the pull request.

#### Verification
- Review: CAP and VIRT leads sign off on the pull request.

#### Evidence
- none

### IPC-057 · Add IDL codegen backends for the remaining supported SDK languages
- Type: build
- Milestone: V2
- Status: todo
- Size: L
- Owner: none
- Depends on: IPC-047, IPC-048, SDK-024, SDK-072
- Baseline: §14, §50

Languages beyond Rust and C selected by SDK's bindings Decision; each backend passes the shared conformance corpus so all bindings interoperate over one wire format (§50).

<!-- covers: INV-0946 -->

#### Out of scope
SDK language crates (SDK-063 and later). Plugin API (IPC-047). Binding order (SDK-024).

#### Acceptance criteria
- [ ] A backend exists for each remaining language named by SDK-024 at this Milestone.
- [ ] Each backend passes the shared conformance corpus against the Rust and C backends on one wire format.
- [ ] Emitted files carry the generated-code license exception.

#### Verification
- Unit: `idl:tests/lang_backends_*` on host CI.
- Integration: polyglot Channel fixture with SDK-070.

#### Evidence
- none

### IPC-058 · Build the virtio VM transport from a native guest Component to a host service
- Type: build
- Milestone: V2
- Status: todo
- Size: L
- Owner: none
- Depends on: IPC-056, IPC-025, IPC-030, VIRT-008
- Baseline: §43
- Invariants: I-047

First non-local transport behind IPC-025, non-gated in V2 alongside VIRT's KVM manager and JakeOS guest images (§43). Remote-machine transport remains out of scope through 1.0.

<!-- covers: INV-0818, INV-0807 -->

#### Out of scope
VM manager product (VIRT-008). Guest tools (VIRT-006). Remote-machine transport (IPC-071).

#### Acceptance criteria
- [ ] A native guest Component talks to a host service over virtio using generated stubs without regenerating the Interface.
- [ ] Capability and handle crossing matches IPC-056; attenuation and revocation still hold.
- [ ] No remote-machine or kernel-distributed logic is introduced (I-047).

#### Verification
- Integration: guest-to-host Channel fixture on H-015.
- Unit: transport plugin tests in `idl:tests/vm_transport_*`.

#### Evidence
- none

### IPC-059 · Write reference pages for every Channel Layer 1 entry point and Object type
- Type: docs
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: IPC-041, IPC-049, DOC-010
- Baseline: §7, §14, §65

V3 exit requires reference pages for 100 percent of Layer 1 ABI entry points; DOC owns the build, IPC authors the Channel, handle-transfer and error-type pages.

#### Out of scope
Docs generator and site (DOC-023). ABI normative semantics (ABI-046).

#### Acceptance criteria
- [ ] Every Channel Layer 1 entry point and object type named by IPC-041 has an authored reference page.
- [ ] Handle-transfer and error types are documented with examples that compile against the IDL.
- [ ] DOC lead records Review sign-off on the pull request.

#### Verification
- Review: DOC lead sign-off recorded on the pull request.
- Integration: DOC coverage Gate includes Channel pages.

#### Evidence
- none

### IPC-060 · Fuzz the IDL compiler front end against malformed third-party Interface files
- Type: build
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: IPC-012, IPC-055, BLD-035
- Baseline: §14, §51

Third-party packages in the V3 public repository submit IDL; the compiler becomes an attack surface in the build pipeline and joins BLD's continuous fuzzing.

<!-- covers: GAP-0130 -->

#### Out of scope
Channel syscall fuzz (IPC-044). Generated Interface mutators (IPC-029).

#### Acceptance criteria
- [ ] A front-end fuzz harness mutates IDL files and is registered with BLD-035.
- [ ] Malformed input is rejected without compiler panic or unbounded memory growth.
- [ ] Language-version pragmas outside the supported set fail closed.

#### Verification
- Fuzz: IDL front-end harness on BLD's fleet for one CI cycle without panic.
- Unit: `idl:fuzz/frontend_*` builds on host CI.

#### Evidence
- none

### IPC-061 · Measure fuzz coverage of every Channel syscall and Layer 2 Interface for the V3 Gate
- Type: build
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: IPC-044, IPC-029, IPC-060
- Baseline: §51

V3 exit: kernel and IPC fuzzing run continuously with no open crasher older than the Gate window; this report proves every IPC surface has a generated or hand-built target and tracks crasher age.

#### Out of scope
Fuzz fleet and crasher-age Gate (BLD-063, BLD-035).

#### Acceptance criteria
- [ ] An inventory lists every Channel syscall and every Layer 2 Interface with its fuzz target.
- [ ] No IPC surface in the inventory lacks a target.
- [ ] Open crasher age is reported for BLD-063; IPC owns the inventory, BLD owns the Gate.

#### Verification
- Integration: `ipc:fuzz/coverage_inventory_*` on `qemu-x86_64` produces the per-syscall and per-Interface coverage table the V3 gate reads.
- Report: inventory committed under `reports/` paths named by BLD, covering every IPC surface.
- Review: BLD lead sign-off recorded on the pull request.

#### Evidence
- none

### IPC-062 · Run old-client/new-service and new-client/old-service tests for every Layer 2 Interface
- Type: build
- Milestone: V3
- Status: todo
- Size: M
- Owner: none
- Depends on: IPC-052, IPC-035, IPC-045
- Baseline: §12, §66

V4 exit requires the Interface-evolution test to pass for every core Interface; the matrix is built in V3 when third-party packages start depending on Layer 2 versions so V4 locks on evidence.

<!-- covers: INV-1285 -->

#### Out of scope
Version lock (IPC-068). Per-domain evolution tests (UIP, OBS, SEM).

#### Acceptance criteria
- [ ] Old-client/new-service and new-client/old-service cases exist for every Interface in IPC-035.
- [ ] CI runs the matrix on `qemu-x86_64`; a breaking pair fails the job.
- [ ] Results are retained as Evidence for IPC-068.

#### Verification
- Integration: `ipc:tests/l2_matrix_*` on `qemu-x86_64`.
- Unit: matrix generator over the L2 Interface list.

#### Evidence
- none

### IPC-063 · Review that Interfaces permit a remote transport with surfaced latency and failure
- Type: docs
- Milestone: V3
- Status: todo
- Size: S
- Owner: none
- Depends on: IPC-025, IPC-011, IPC-032
- Baseline: §43, §32, §57
- Invariants: I-047

Design review of core Interfaces confirming nothing precludes a future remote-machine transport, that latency would be surfaced not hidden and that disconnect, timeout and partial failure are explicit; no remote transport is built (1.0 non-promise, §43).

<!-- covers: INV-0808, INV-0812, INV-0813 -->

#### Out of scope
Building a remote transport (IPC-071). Kernel distribution (I-047).

#### Acceptance criteria
- [ ] The review lists each core Layer 2 Interface and records whether latency, disconnect, timeout and partial failure are explicit.
- [ ] Findings that would preclude a future remote transport are filed as follow-up work or accepted exceptions.
- [ ] ABI lead records Review sign-off; no remote transport ships.

#### Verification
- Review: ABI lead sign-off recorded on the pull request.

#### Evidence
- none

### IPC-064 · Freeze the Channel Layer 1 ABI Surface
- Type: adr
- Milestone: V4
- Status: todo
- Size: S
- Owner: none
- Depends on: IPC-041, ABI-049, IPC-065, IPC-008, IPC-017
- Baseline: §65, §66
- Decision: D-0143
- Risks: R-007
- Invariants: I-040

V4 exit: Layer 1 frozen with the freeze Decision accepted; IPC's amendment covers Channel syscalls, handle-transfer layout and the version header, with deprecated entry points removed (§65, §66).

#### Out of scope
ABI freeze Decision (ABI-049). Conformance tests (IPC-065). Deprecated-entry removal plumbing (ABI-048).

#### Acceptance criteria
- [ ] Option A (freeze the V1 candidate set as S-012), option B (freeze a reduced send/receive/close core), and option C (defer freeze to 1.0) are evaluated against the V0 spike, B-004/B-005 reports and the conformance suite.
- [ ] The Decision lists Channel syscalls, handle-transfer layout and the version header, and names deprecated entry points to remove.
- [ ] ABI lead records Review sign-off on the pull request.

#### Verification
- Review: ABI lead sign-off recorded on the pull request.

#### Evidence
- none

### IPC-065 · Add conformance tests for every frozen Channel Layer 1 entry point
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: IPC-041, ABI-047, IPC-010, ABI-049
- Baseline: §65, §66
- Freezes: S-012

V4 exit: every Layer 1 entry point has a conformance test and binaries built against the freeze candidate run on every subsequent beta build.

#### Out of scope
ABI golden binary suite (ABI-047). Freeze Decision (IPC-064).

#### Acceptance criteria
- [ ] Every Channel Layer 1 entry point named by IPC-041 has a conformance test.
- [ ] A binary built against the freeze candidate runs on a subsequent beta image in CI.
- [ ] Deprecated entry points named by the freeze Decision are absent from the suite's required set.

#### Verification
- Integration: Channel slice of ABI-047 on `qemu-x86_64` and H-002.
- Unit: `kernel:tests/ipc/conformance_*` on `qemu-x86_64`.

#### Evidence
- none

### IPC-066 · Close High and Critical findings from the external IPC security audit
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: SEC-070, SEC-067, IPC-010, IPC-037
- Baseline: §51, §9

V4 scope: external audit of IPC among kernel Capability enforcement and personalities with all High and Critical fixed before RC1.

#### Out of scope
Commissioning the audit (SEC-070). Auditor re-verify (SEC-069). Medium triage (SEC-068).

#### Acceptance criteria
- [ ] Every High and Critical finding tagged IPC has a fix and a regression test.
- [ ] SEC-067 records those findings closed.
- [ ] No High or Critical IPC finding remains open at RC1.

#### Verification
- Review: SEC lead sign-off recorded on the pull request.
- Unit: regression tests named on each finding, on `qemu-x86_64` and `hw-h002`.

#### Evidence
- none

### IPC-067 · Publish the unsafe-code inventory for IPC kernel and runtime code
- Type: docs
- Milestone: V4
- Status: todo
- Size: S
- Owner: none
- Depends on: IPC-010, IPC-016, IPC-014
- Baseline: §51

V4 exit requires an unsafe-code inventory with justification per block; IPC contributes its fast-path and handle-transfer blocks (§51).

#### Out of scope
Project-wide unsafe inventory (BLD-011). Kernel live-patching (I-086).

#### Acceptance criteria
- [ ] Every `unsafe` block in IPC kernel and runtime code is listed with a justification.
- [ ] Fast-path and handle-transfer blocks are included.
- [ ] SEC or kernel lead records Review sign-off on the pull request.

#### Verification
- Review: SEC or kernel lead sign-off recorded on the pull request.
- Unit: CI inventory check fails on an unlisted `unsafe` block in IPC paths.

#### Evidence
- none

### IPC-068 · Enumerate and lock the Layer 2 Interface versions served for 1.x
- Type: build
- Milestone: V4
- Status: todo
- Size: M
- Owner: none
- Depends on: IPC-062, IPC-042, ABI-037, ABI-039
- Baseline: §66
- Freezes: S-013

V4 exit: Layer 2 versions for 1.x enumerated and locked with the evolution test matrix green for every core Interface (§66).

<!-- covers: INV-1285 -->

#### Out of scope
Layer 1 freeze (ABI-049). 1.0 supported-versions document (IPC-070).

#### Acceptance criteria
- [ ] Every core Layer 2 Interface has a locked version identity listed for 1.x.
- [ ] IPC-062 is green for every listed Interface.
- [ ] Adding a new Layer 2 version after lock fails CI without a superseding Decision.

#### Verification
- Integration: lock check in CI on `qemu-x86_64`.
- Review: ABI lead sign-off recorded on the pull request.

#### Evidence
- none

### IPC-069 · Extend the IPC benchmark with Windows and macOS baselines for 1.0 publication
- Type: build
- Milestone: 1.0
- Status: todo
- Size: M
- Owner: none
- Depends on: IPC-008, BEN-060, BEN-047, BEN-046
- Baseline: §14, §54
- Benchmarks: B-004, B-005
- Invariants: I-061

1.0 exit: every §54 metric published on Tier 1 hardware against Linux, Windows and macOS; adds ALPC and Mach-port ping-pong baselines to the V0 harness under BEN methodology with no unmeasured claim.

<!-- covers: INV-0277 -->

#### Out of scope
Publication dashboards (BEN-060). Methodology (BEN-007).

#### Acceptance criteria
- [ ] B-004 and B-005 reports on every in-scope Tier 1 machine include Linux, Windows (where dual-boot exists) and macOS (where a comparable class exists) baselines.
- [ ] ALPC and Mach-port ping-pong are named baselines in those reports.
- [ ] No 1.0 announcement cites an IPC number except by B-ID.

#### Verification
- Bench: B-004 and B-005 on every 1.0 hardware-scope H-ID; target per Register.
- Review: BEN lead sign-off recorded on the pull request.

#### Evidence
- none

### IPC-070 · Publish the supported Layer 2 Interface versions and deprecation policy for 1.x
- Type: docs
- Milestone: 1.0
- Status: todo
- Size: S
- Owner: none
- Depends on: IPC-068, ABI-039, GOV-083
- Baseline: §66

1.0 exit: a published compatibility document lists supported Layer 2 versions with a minimum two-minor-release deprecation overlap; IPC owns the versioning rules, ABI and GOV sign off.

#### Out of scope
ABI stability statement (ABI). Governance contract (GOV-083). Semantic catalog pages (SEM-043).

#### Acceptance criteria
- [ ] The document lists every locked Layer 2 Interface version served for 1.x.
- [ ] The deprecation policy states a minimum two-minor-release overlap.
- [ ] ABI and GOV leads record Review sign-off on the pull request.

#### Verification
- Review: ABI and GOV leads sign off on the pull request.

#### Evidence
- none

### IPC-071 · Prototype a remote-machine transport honoring Capabilities, identity and encryption
- Type: build
- Milestone: LATER
- Status: todo
- Size: L
- Owner: none
- Depends on: IPC-063, IPC-025, IPC-056
- Baseline: §43, §57
- Invariants: I-047

Distributed and remote Interfaces are explicitly not promised by 1.0 and parked in LATER; the prototype must surface latency, expose explicit failure semantics and stay outside the kernel (§43, §57).

<!-- covers: INV-0808, INV-0812, INV-0813 -->

#### Out of scope
Kernel distribution (I-047). VM transport (IPC-058). 1.0 productization.

#### Acceptance criteria
- [ ] A userspace remote-machine transport plugin speaks generated stubs without regenerating Interfaces.
- [ ] Latency is visible to clients; disconnect, timeout and partial failure are typed results.
- [ ] Capabilities, identity and encryption are honored; no kernel remote-machine logic is added.

#### Verification
- Integration: two-machine fixture using generated stubs; failure and latency are explicit.
- Review: ABI lead sign-off recorded on the pull request.

#### Evidence
- none
