# D-0157 · Decide kernel-core vs user-space service boundary and the criteria for moving one
- Status: accepted
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
Option A. The §4 split stands for V0 and beyond: the kernel core owns memory, IPC, capabilities and the scheduler; UI, storage services and network services are user-space Components hosted by SVC. Moving any subsystem across the boundary requires a decision that cites a measured cost per HW-002 criteria (latency, DMA safety, interrupt performance, inherited-driver constraints), never ideology.

## Consequences
- Native storage and network Objects are minted by user-space services over inherited kernel mechanisms; the kernel exposes block, network and DRM mechanisms, not policy.
- Service restart and rebind (§32) are first-class requirements for everything outside the core.
- Any proposal to move a subsystem is an adr task with a benchmark report attached.

## Rejected options and why
- Option B (storage or network into the core) rejected: it would freeze Linux in-kernel stacks as native semantics before their user-space shape is designed.
- Option C (IPC or scheduler in user space) rejected: microkernel purity is a stated non-goal (§33, §57) and the isolation and IPC cost targets depend on kernel-resident handoff.

## Follow-ups
none
