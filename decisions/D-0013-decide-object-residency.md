# D-0013 · Decide which Object<T> types live in the kernel and the kernel-residency criteria
- Status: proposed
- Task: ABI-013
- Surfaces: none
- Layer: none
- Spikes: ABI-022
- Supersedes: none
- Superseded by: none
- Baseline: §7, §33, §65
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Which Object<T> types are kernel-resident and which are user-service objects reached through Channels must follow written residency criteria (§65 rule 2, I-008) before V0 object types are placed.

## Options

### Option A · Isolation or privilege only
Summary: An object is kernel-resident only when isolation or privilege requires it.
Consequences: Smallest kernel and clearest Layer 2 boundary; some latency-sensitive semantics pay a Channel round trip.
Evidence: none

### Option B · Measured cost
Summary: An object is kernel-resident when a published measurement shows the user-service form cannot meet its budget.
Consequences: Residency follows evidence; every placement needs a benchmark and the criterion can be gamed by choosing budgets.
Evidence: none

### Option C · All typed objects in kernel
Summary: Every typed object is a kernel object.
Consequences: Uniform model and no Channel hop; the kernel grows high-level semantics that §65 wants in Layer 2.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
