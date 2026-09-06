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
The small-message Channel send is the most frequent kernel crossing in the native model (§15) and its cost bounds B-004 and B-005. IPC-017 prototypes each technique on H-001 and H-002 and reports same-core and cross-core round trip, throughput under batching and the cost of waking a sleeping receiver. This decision fixes the ABI-visible send path on S-012 before IPC-016 implements Channel kernel semantics; S-012 stays prototyped and no number appears here, only in the report and the B-IDs. Rejected techniques are named so no later task re-prototypes them (§53).

## Options

### Option A · Shared ring
Summary: Sender and receiver share a bounded ring of message slots in a MemoryObject; the kernel is entered only to wake a sleeping receiver or when the ring is full.
Consequences: Batching is natural and most sends never enter the kernel, which favours throughput and B-005. The slot layout, ring depth and full/empty protocol become Layer 1 ABI on S-012 and can only change behind a version. Backpressure is the ring depth and is visible to both ends. A sleeping receiver still costs a wake-up, so same-core round trip does not beat handoff on B-004.
Evidence: `reports/spikes/IPC-017.md`

### Option B · CPU-register-carried messages
Summary: A send syscall carries a payload of a few machine words in registers straight into the receiver's completion record.
Consequences: No copy and no shared memory for the smallest messages; the cost is exactly one syscall pair. The payload cap is what registers hold, so every larger message needs a second mechanism and the split point becomes ABI. An architecture-neutral definition is awkward because register count differs per ISA (I-057).
Evidence: `reports/spikes/IPC-017.md`

### Option C · Scheduler-aware handoff
Summary: Send donates the remaining slice and the CPU to the receiver Task when it is runnable on the same core, so the receiver runs before the sender is rescheduled.
Consequences: Lowest same-core round trip and no wake-up latency, which makes B-004 easiest to meet. Couples Channels to scheduler internals and to SCH intent classes: the decision must state what happens when the receiver is on another core, is budget-exhausted in its ResourceDomain, or holds a lower intent, which is where priority inheritance across a handoff (Q-014) surfaces. Fairness is harder to reason about.
Evidence: `reports/spikes/IPC-017.md`

### Option D · Lock-free cross-core queues
Summary: Per-pair single-producer queues with atomic indices and no locks; the receiver polls or is woken by an inter-processor interrupt.
Consequences: Best cross-core throughput and no kernel on the data path. No same-core advantage over a ring, and a polling receiver burns CPU against EnergyEfficient intent. The memory-ordering contract is ABI and differs per ISA, so the conformance suite must state it per architecture.
Evidence: `reports/spikes/IPC-017.md`

### Option E · Recorded combination
Summary: A named pairing with a stated selection rule, for example handoff when the receiver is idle on the same core and a shared ring otherwise.
Consequences: Takes the best measured case of two techniques. Both layouts and the selection rule are ABI on S-012, so twice as much must be specified, tested and frozen at V4, and the conformance suite covers both paths. Chosen only if the report shows one technique alone misses a B-004 or B-005 target.
Evidence: `reports/spikes/IPC-017.md`

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
