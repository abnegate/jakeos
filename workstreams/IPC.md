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

Call semantics change scheduler and Native ABI shape irreversibly (§15, §18). Same-core and cross-core round trips from IPC-017 are reviewed with SCH and TSK before Channel send is fixed. Native software sees an Operation, never a blocking syscall. Option B is admissible only as an Operation whose completion is awaited with time-slice donation; ABI-012 already forbids any entry that blocks the calling execution context except wait-for-completion.

<!-- covers: GAP-0481 -->

#### Out of scope
Fast-path technique selection (IPC-003). Direct-switch hook implementation (IPC-015, SCH-005).

#### Acceptance criteria
- [ ] Option A (async send and receive only) and option B (async send plus synchronous call with time-slice donation to the callee) are evaluated against the spike's same-core and cross-core reports.
- [ ] The Decision names the rejected option and the ABI-visible entry points that follow from the choice.
- [ ] SCH and TSK leads record Review sign-off on the pull request.

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

Every Interface carries an explicit evolution strategy (§12). This Decision records how Interfaces version, add fields, add optional methods and negotiate, using findings from IPC-019. The rules are prototyped in V0; freeze is IPC-042 at V1 (R-005).

<!-- covers: INV-0247, INV-0260, INV-0249 -->

#### Out of scope
V1 freeze of the rules (IPC-042). Layer 1 handshake (ABI-016, ABI-004).

#### Acceptance criteria
- [ ] Option A (schema-indexed optional fields with generated negotiation), option B (self-describing envelopes with unknown-field preservation), and option C (explicit major/minor with dual-stack during overlap) are evaluated against the three-revision spike.
- [ ] The Decision states how optional methods, unknown fields and version identity appear on the wire, and records that the IDL ships with an evolution story.
- [ ] ABI lead records Review sign-off on the pull request.

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

V0 exit requires an accepted fast-path-mechanism Decision listing rejected options, chosen from IPC-017 before Channel kernel semantics are fixed (§15). The selected technique is the one IPC-016 implements. Numbers live only in the spike report and in B-004 and B-005.

<!-- covers: INV-0299, GAP-0480 -->

#### Out of scope
Call versus send (IPC-001). Production fast path (IPC-016).

#### Acceptance criteria
- [ ] Option A (shared ring), option B (CPU-register-carried messages), option C (scheduler-aware handoff), option D (lock-free cross-core queues), and option E (a recorded combination) are evaluated against the spike report.
- [ ] The Decision names the rejected techniques and the ABI-visible send path that remains.
- [ ] ABI lead records Review sign-off on the pull request.

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

Typed IPC across a multi-repo or multi-language ecosystem breaks when generated stubs drift from their IDL. This Decision is accepted before the first generated Rust stubs land. Determinism CI is IPC-034.

<!-- covers: GAP-0098 -->

#### Out of scope
Generator determinism check (IPC-034). Generated-code license exception (IPC-005).

#### Acceptance criteria
- [ ] Option A (commit generated stubs next to the IDL) and option B (emit stubs at build time from the IDL) are evaluated against drift, reviewability and multi-language backends.
- [ ] The Decision states how CI proves generator output is deterministic and matches the IDL.
- [ ] BLD lead records Review sign-off on the pull request.

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

Generated stubs land in every application from V0 onward. Unclear terms would contaminate the ecosystem. The Decision is encoded as a generated-code exception in every emitted file header by IPC-012 and is reviewed with GOV licensing policy.

<!-- covers: GAP-0007 -->

#### Out of scope
IDL and ABI specification license (IPC-024). Firewall mapping of Layers (GOV-003).

#### Acceptance criteria
- [ ] Option A (generated-code exception, output owned by the compiler user with no copyleft obligation), option B (output inherits the compiler license), and option C (output dedicated to the public domain) are evaluated with GOV.
- [ ] The Decision states the header text every backend emits.
- [ ] GOV lead records Review sign-off on the pull request.

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

The IDL is the language every platform Interface is written in; switching after V1 would invalidate every generated binding. The evaluation covers ownership transfer, Capability passing, versioning, optional methods, streams and multi-language codegen (§12, §14). V0 exit requires an accepted IDL Decision with rejected options. Relationship to WIT is a later Decision (IPC-022).

<!-- covers: GAP-0519, INV-0261, INV-0260, INV-0247 -->

#### Out of scope
Native IDL versus WIT mapping (IPC-022). Compiler implementation (IPC-012). Wire encoding (IPC-007).

#### Acceptance criteria
- [ ] Option A (adopt WIT), option B (adopt FIDL), option C (adopt Cap'n Proto schema), and option D (design a new IDL) are evaluated in a written matrix against ownership transfer, Capability passing, versioning, optional methods, streams and multi-language codegen.
- [ ] The Decision records that every Interface carries an explicit evolution strategy and that the IDL ships with an evolution story.
- [ ] WASM and ABI leads record Review sign-off on the pull request.

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

Chooses fixed-layout, self-describing or schema-indexed representation and its zero-copy properties, and resolves the size threshold and heuristics for inline small message versus MemoryObject transfer, from IPC-020 measurements (§14, §15). Answers Q-005. Payload bytes do not move when avoidable (I-063).

<!-- covers: INV-0291, INV-0302, GAP-0520 -->

#### Out of scope
MemoryObject backing and transfer enforcement (MEM-010, MEM-003). Production lowering of large payloads (IPC-036).

#### Acceptance criteria
- [ ] Option A (fixed layout), option B (self-describing), and option C (schema-indexed) are evaluated against encode, decode and receiver-side validation cost in the spike report.
- [ ] The Decision states the inline-versus-MemoryObject threshold rule without restating a number in prose, and names S-013.
- [ ] ABI and MEM leads record Review sign-off on the pull request.

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
- Depends on: IPC-016, IPC-015, BEN-007, BEN-005
- Baseline: §14, §54, §53
- Benchmarks: B-004, B-005
- Invariants: I-061

V0 benchmark Gate: small-message same-core and cross-core p50/p99 measured on H-001 and H-002 and published beside Unix-domain-socket and pipe numbers, publish-only in V0 (B-004, B-005). Registered with BEN and run in nightly CI. Tracing-overhead measurement on this path is OBS/BEN (B-012).

<!-- covers: INV-0277 -->

#### Out of scope
Methodology and publication (BEN-003, BEN-007). Tracing-overhead ratio (B-012, OBS). V1 absolute targets (IPC-054).

#### Acceptance criteria
- [ ] Harness `bench:ipc-roundtrip-same-core` and `bench:ipc-roundtrip-cross-core` run on H-001 and H-002 and emit reports under `reports/benchmarks/B-004/` and `reports/benchmarks/B-005/`.
- [ ] Each report includes Linux Unix-domain-socket and pipe ping-pong on the same machine.
- [ ] Nightly CI runs the harness; V0 target kind is publish per the Register.

#### Verification
- Bench: B-004 and B-005 on H-001 and H-002; target per Register.
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

Bounded queue depth per Channel, sender behavior as block-as-Operation, fail, or drop-by-policy, and queue depth exposed to OBS for `os inspect channel` (R-073). Unspecified backpressure livelocks the fast path. Native software never sees a socket buffer API.

<!-- covers: EXTRA-001 -->

#### Out of scope
ResourceDomain charging of queue memory (IPC-027). Inspect CLI rendering (SDK-007). Stream flow control (IPC-039).

#### Acceptance criteria
- [ ] Each Channel has a bounded depth visible via the inspect payload consumed by `os inspect channel`.
- [ ] A slow receiver causes the sender Operation to complete with the declared policy (block, fail, or drop) and never livelocks the fast path.
- [ ] Exceeding depth allocates no unbounded kernel memory; T-016 exhaustion is a typed error.

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
- Depends on: ABI-002, ABI-005, CAP-005, TSK-013, KRN-001, CMP-014
- Baseline: §4, §7, §14, §59
- Invariants: I-018

Kernel Core owns Channel transport (§4). Channel Object handles, endpoint pairs, queuing, peer-closed state and the ownership and relationship data behind `os inspect channel` are a V0 exit criterion. Built on the wrapper strategy from CMP and handle representation from ABI. Native IPC is not a socket (I-018).

<!-- covers: INV-0051, INV-0112, INV-0161, INV-0278, INV-1159, INV-1324 -->

#### Out of scope
Small-message fast path (IPC-016). Handle slots in messages (IPC-014). Inspect CLI (SDK-007). Typed inspect Interface (OBS-007).

#### Acceptance criteria
- [ ] Creating a Channel yields a typed endpoint pair; send and receive are Operations and complete with typed results.
- [ ] `os inspect channel` data includes endpoints, message type, queue depth, waiting senders and receivers, and peer-closed state for every live Channel.
- [ ] Native crates cannot open a Channel as a socket or byte pipe; the ABI review Gate rejects such entry points.
- [ ] Destroying both endpoints reclaims queue memory; the V0 Component leak test shows no Channel residue.

#### Verification
- Unit: `kernel:tests/ipc/channel_*` on `qemu-x86_64` and `hw-h002`.
- Integration: V0 Demo pipeline with CMP-011.
- Review: ABI review Gate checklist includes Channel.

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

V0 fault Demo: B panics and A observes a typed disconnect. IDL error taxonomy, disconnect result on peer death, and DeadlineExceeded result via Operation deadlines (§12, §32). Failure and restart are part of every typed Interface (I-037).

<!-- covers: INV-0259 -->

#### Out of scope
Client rebind and retry codegen (IPC-028). Supervisor restart (SVC). Panic abort policy (CMP-008).

#### Acceptance criteria
- [ ] Peer death completes in-flight receive Operations with a typed disconnect result and delivers no further payload.
- [ ] An Operation with an expired deadline completes with DeadlineExceeded and never delivers a late result.
- [ ] The V0 fault Demo shows Component B panicking and Component A observing the typed disconnect.

#### Verification
- Unit: `kernel:tests/ipc/failure_*` on `qemu-x86_64` and `hw-h002`.
- Demo: V0 fault Demo on H-002.
- Integration: deadline and peer-death cases in TSK-024.

#### Evidence
- none

### IPC-012 · Implement the IDL compiler with Rust wire layout, stub, ownership and tracing codegen
- Type: build
- Milestone: V0
- Status: todo
- Size: L
- Owner: none
- Depends on: IPC-006, IPC-007, IPC-004, IPC-005, ABI-007
- Baseline: §14, §24, §53

Front end (parse, typecheck, move/borrow/share annotations) plus the first backend: generated wire representation, client and server stubs, ownership semantics and per-method tracing metadata so IPC calls appear as semantic spans in `os trace` (§14, §24). The IDL is the single schema source; emitted headers carry the generated-code license exception.

<!-- covers: INV-0292, INV-0286, INV-0287, INV-0288, INV-1005, INV-0285, INV-0474, GAP-0007 -->

#### Out of scope
Async proxy semantics (IPC-013). C backend (IPC-048). Plugin API (IPC-047). Trace substrate (OBS).

#### Acceptance criteria
- [ ] The compiler parses the chosen IDL, typechecks move/borrow/share annotations, and emits Rust wire layout, client stubs, server stubs and per-method tracing metadata.
- [ ] Every emitted file carries the generated-code license exception header from IPC-005.
- [ ] Re-running the compiler on the same IDL produces byte-identical output; CI fails on drift.
- [ ] A native crate cannot hand-write a parallel schema for a compiled Interface; IPC-034 rejects it.

#### Verification
- Unit: `idl:tests/frontend_*` and `idl:tests/rust_backend_*` on host CI.
- Integration: ImageDecoder sample from SDK-002 compiles against generated stubs.

#### Evidence
- none

### IPC-013 · Generate Interface<T> proxies with async methods, futures and in-flight cancellation
- Type: build
- Milestone: V0
- Status: todo
- Size: L
- Owner: none
- Depends on: IPC-012, TSK-011, TSK-010
- Baseline: §12, §14, §59
- Invariants: I-030

Typed service contracts as declared in the IDL; methods surface as Operation futures from TSK and cancelling the Operation cancels the in-flight call. Required by the V0 Demo and cancellation Demo (§12, §14, §59). Native APIs are asynchronous by default (I-030).

<!-- covers: INV-0058, INV-0248, INV-0254, INV-0256, INV-0258, INV-0279, INV-1316 -->

#### Out of scope
Streams (IPC-039). Version negotiation codegen (IPC-033). Runtime executor (SDK-004).

#### Acceptance criteria
- [ ] Generated proxies expose each IDL method as an Operation future; awaiting it yields the typed result.
- [ ] Cancelling the Operation cancels the in-flight call; the server observes cancellation and the client never receives a result.
- [ ] The V0 Demo ImageDecoder Interface is generated from IDL and used by CMP-011.

#### Verification
- Unit: `idl:tests/async_stubs_*` on host CI.
- Integration: V0 Demo and cancellation Demo on `qemu-x86_64` and `hw-h002`.
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

V0 Demo and exit criterion: a MemoryObject Capability moves between Components without copying the payload, verified by physical-page identity; handle slots in the wire format with move semantics so large payloads never copy per hop (§15, §16). Confused-deputy extra handles are rejected (T-002).

<!-- covers: INV-0257, INV-1006, INV-1003, INV-0301 -->

#### Out of scope
MemoryObject map and backing (MEM). Capability derive and revocation (CAP). Physical-page identity harness (MEM-012).

#### Acceptance criteria
- [ ] A message can move a Capability and a MemoryObject; after send the sender's handle is invalid and the receiver holds the only handle.
- [ ] Physical-page identity of a transferred MemoryObject is unchanged; MEM-012 passes through this path.
- [ ] A message that names more handles than its type permits is rejected with a typed error and allocates no handle in the receiver.

#### Verification
- Unit: `kernel:tests/ipc/handle_transfer_*` on `qemu-x86_64` and `hw-h002`.
- Integration: V0 Demo pipeline with MEM-012.

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

Native IPC shape (§53): a send to a waiting receiver switches directly to the receiver. Coordinated with SCH for the direct-switch hook and with IPC-001. V0 benchmark Gate measures native Task handoff versus Linux thread switch (B-003, B-004).

<!-- covers: INV-1007 -->

#### Out of scope
Scheduler class mapping (SCH-004). Task multiplexer (TSK-019). Intent inheritance across handoff (SCH-024).

#### Acceptance criteria
- [ ] When the receiver Task is waiting on receive, send switches to that Task without an extra run-queue hop, as traced by OBS.
- [ ] The path is the one named by IPC-001; the rejected call shape is absent from the ABI snapshot.
- [ ] B-004 same-core reports include this handoff configuration.

#### Verification
- Unit: `kernel:tests/ipc/handoff_*` on `qemu-x86_64` and `hw-h002`.
- Bench: B-004 on H-001 and H-002; target per Register.

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

Implements IPC-003 so the common small-message case needs no userland serialization or deserialization step (§15, §53). Measured by IPC-008. Native IPC must not require a userland serialize/deserialize step for that case.

<!-- covers: INV-0293, INV-1002, INV-1004, INV-1324 -->

#### Out of scope
Technique selection (IPC-003). Batching productionisation (IPC-043). V1 tuning (IPC-054).

#### Acceptance criteria
- [ ] Small messages on the selected path have no userland serialize or deserialize step: a unit test counts zero payload copies between the sender store and the receiver load; IPC-034 later turns this into a standing lint.
- [ ] The implementation matches the technique named by IPC-003; rejected techniques are not reachable from generated stubs.
- [ ] IPC-008 runs against this path on H-001 and H-002.

#### Verification
- Unit: `kernel:tests/ipc/fast_path_*` on `qemu-x86_64` and `hw-h002`.
- Bench: B-004 and B-005 on H-001 and H-002; target per Register.

#### Evidence
- none

### IPC-017 · Prototype and measure ring, GPR-carried, handoff and batched small-message fast paths
- Type: spike
- Milestone: V0
- Status: todo
- Size: L
- Owner: none
- Depends on: ABI-019, TSK-021, BLD-012, BEN-007
- Baseline: §15, §53, §58
- Benchmarks: B-004, B-005
- Explores: S-012

Single measured comparison on identical hardware (H-001 and H-002) of shared ring buffers, CPU-register-carried messages, seL4/LRPC-style direct handoff, lock-free cross-core queues and io_uring-style batching; same-core and cross-core round trips recorded separately. Publish-only numbers. Precedes IPC-003 and IPC-001.

<!-- covers: INV-0294, INV-0295, INV-0297, INV-0298, GAP-0480, GAP-0481 -->

#### Out of scope
Selecting the production technique (IPC-003). Standing harness (IPC-008).

#### Acceptance criteria
- [ ] Prototypes for shared ring, CPU-register-carried messages, scheduler-aware handoff, lock-free cross-core queues and batched submission run on H-001 and H-002.
- [ ] The report records same-core and cross-core round trips separately for each prototype with Linux Unix-domain-socket and pipe baselines.
- [ ] The report names which techniques remain candidates for IPC-003 and which are ruled out.

#### Verification
- Report: `reports/spikes/IPC-017.md` answers cost, complexity, ABI impact and reject reasons per technique, with same-core and cross-core tables.
- Bench: B-004 and B-005 on H-001 and H-002; target per Register (publish).

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

Written study of Capability semantics across transports, synchronous message passing and restartable resource managers; feeds IPC-006, IPC-007 and IPC-001 (§43, §58). One of the research studies kept in V0 because it informs V0 ABI surfaces.

<!-- covers: INV-0816, INV-1134 -->

#### Out of scope
IDL selection (IPC-006). Wasm Component Model study (WASM-002).

#### Acceptance criteria
- [ ] The report covers Cap'n Proto RPC, Fuchsia FIDL/Overnet, Genode and QNX on Capability passing, sync versus async, restart and wire encoding.
- [ ] Each system is scored against ownership transfer, versioning, streams and multi-language codegen.
- [ ] Findings are cited by IPC-006, IPC-007 and IPC-001.

#### Verification
- Report: `reports/spikes/IPC-018.md` answers what to steal, what to reject, and which questions remain for the three V0 Decisions.
- Review: ABI lead sign-off recorded on the pull request.

#### Evidence
- none

### IPC-019 · Evolve one real V0 Interface through three incompatible revisions to exercise the versioning scheme
- Type: spike
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: IPC-012, IPC-013
- Baseline: §12
- Explores: S-014
- Risks: R-005

The versioning design must fail on a real evolution before any rule is frozen; uses the V0 Demo ImageDecoder-style Interface and produces findings for IPC-002 (§12).

<!-- covers: GAP-0521 -->

#### Out of scope
Accepting evolution rules (IPC-002). Permanent UI protocol bump test (IPC-040).

#### Acceptance criteria
- [ ] One real V0 Interface is evolved through three incompatible revisions using the compiler and stubs.
- [ ] The report records which changes were compatible, forward-only or breaking, and where the prototype scheme failed.
- [ ] Findings are inputs to IPC-002; no Layer 2 rule is frozen in this Spike.

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

Measures encoding, decoding and receiver-side validation cost for representative message shapes and sizes so IPC-007 and the inline-versus-MemoryObject threshold are decided on numbers (§14, §15, §53).

<!-- covers: GAP-0520, INV-0302 -->

#### Out of scope
Choosing the format (IPC-007). Production validation hardening (IPC-037).

#### Acceptance criteria
- [ ] In-place zero-copy and compact encode/decode are measured for representative shapes and sizes, including receiver-side validation, on H-001 and H-002.
- [ ] The report recommends an inline-versus-MemoryObject threshold rule without stating a public performance claim.
- [ ] Findings are inputs to IPC-007.

#### Verification
- Report: `reports/spikes/IPC-020.md` answers encode, decode and validation cost per candidate, zero-copy properties, and the threshold heuristic.
- Bench: B-004 on H-001 and H-002; target per Register (publish).

#### Evidence
- none

### IPC-021 · Add the version header and forward/backward unknown-field compatibility tests
- Type: build
- Milestone: V0
- Status: todo
- Size: M
- Owner: none
- Depends on: IPC-007, IPC-002, IPC-012, ABI-004
- Baseline: §12, §65
- Invariants: I-041

V0 exit criterion: a message with an unknown newer field is accepted by an older receiver and an older message by a newer receiver. Includes the message-level part of the Layer 1 negotiation handshake test that ABI owns. The full schema-evolution feature set lands in IPC-038. Version negotiation exists from V0 (I-041).

<!-- covers: INV-0251, INV-0252 -->

#### Out of scope
Layer 1 handshake implementation (ABI-004). Optional methods and schema evolution (IPC-038).

#### Acceptance criteria
- [ ] A message with an unknown newer field is accepted by an older receiver; an older message is accepted by a newer receiver.
- [ ] The version header is present on every generated message type used by the V0 Demo.
- [ ] The test is retained permanently and is the message-level counterpart of ABI-004.

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

### IPC-042 · Freeze the Layer 2 Interface-evolution rules for SDK v1
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
