# D-0199 · Decide whether Ownership transfer is kernel-enforced or advisory
- Status: proposed
- Task: MEM-003
- Surfaces: none
- Layer: none
- Spikes: MEM-011
- Supersedes: none
- Superseded by: none
- Baseline: §16, §17, §65, §67
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Ownership transfer is the ABI default for MemoryObjects (§16, §17, §65, §67), so whether the kernel unmaps the sender or transfer is advisory must be fixed with measured cost.

## Options

### Option A · Kernel unmaps the sender and invalidates the sender handle
Summary: Transfer is enforced by the kernel.
Consequences: Safe by construction; unmap and TLB cost.
Evidence: none

### Option B · Advisory transfer
Summary: The sender promises not to touch the object.
Consequences: Cheap; a misbehaving sender can corrupt.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
