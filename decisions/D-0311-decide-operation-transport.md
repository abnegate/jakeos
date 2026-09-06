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
Operations are the only asynchronous primitive (§18, §19): everything the kernel does for a Component is submitted and completed through this transport, so wake-up latency (B-009), batching and the shape of a completion sit under every benchmark. TSK-014 prototypes the transports on H-001 and H-002. io_uring lineage is retained mechanism (§2); the question is what becomes ABI on S-005, which stays prototyped until V1 candidacy (§65).

## Options

### Option A · Shared rings
Summary: Per-Component submission and completion rings in a MemoryObject, io_uring-style, with one enter syscall used only to wake; a batch is a run of contiguous entries.
Consequences: Amortised syscalls and natural batching, and the runtime reaps completions without entering the kernel. Entry layout, flags and memory ordering are Layer 1 ABI and must be architecture-neutral (I-057). Rings are per Component, so a misbehaving Component only corrupts its own; the kernel must still validate every entry against time-of-check races. A sleeping submitter pays one syscall to be woken.
Evidence: `reports/spikes/TSK-014.md`

### Option B · Per-Component queues
Summary: Kernel-owned per-Component queues; submission and completion each cross through a syscall that passes entries by pointer, with no shared ring layout.
Consequences: No shared-memory layout in the ABI, so the entry format can evolve behind the syscall, and the kernel copies then validates every entry once. Every batch costs a syscall pair and a copy, which B-009 will show; simpler capability checking with no time-of-check race.
Evidence: `reports/spikes/TSK-014.md`

### Option C · Syscall-per-Operation
Summary: One syscall submits one Operation and a wait syscall delivers completions.
Consequences: The simplest ABI to specify and verify. No batching, so B-009 and the Channel round trip pay a full kernel entry per Operation and mitigations make that the dominant cost. Viable only as the fallback path of a hybrid.
Evidence: `reports/spikes/TSK-014.md`

### Option D · Hybrid
Summary: Shared rings on the hot path plus a syscall path for large or privileged Operations and for Components that opt out of shared memory.
Consequences: The best measured latency with a safe fallback. Two paths to specify, test and freeze at V4, and the rule that selects between them is itself ABI; the conformance suite covers both.
Evidence: `reports/spikes/TSK-014.md`

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
