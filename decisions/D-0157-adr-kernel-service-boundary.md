# D-0157 · Decide kernel-core vs user-space service boundary and the criteria for moving one
- Status: proposed
- Task: KRN-001
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §4, §33, §57
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
§4 places Memory, IPC, Capabilities and the scheduler in the kernel core and UI, storage and network in user space; the V0 split and cost criteria for moving a service must be recorded (§4, §33, §57).

## Options

### Option A · §4 split as written
Summary: The baseline split is adopted for V0 with a published metric for later moves.
Consequences: Coherent with the baseline; some services pay a Channel hop.
Evidence: none

### Option B · Move storage or network into the kernel core
Summary: Storage or network becomes kernel-core.
Consequences: Fewer hops; a larger kernel and more privileged code.
Evidence: none

### Option C · Move IPC or the scheduler into user space
Summary: IPC or scheduling leaves the kernel.
Consequences: Microkernel purity; measured cost that §57 says must justify it.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
