# D-0007 · Decide Capability handle representation: dense index, sparse id or sealed value
- Status: proposed
- Task: ABI-010
- Surfaces: none
- Layer: none
- Spikes: ABI-022
- Supersedes: none
- Superseded by: none
- Baseline: §7, §8, §65
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The userspace representation of a live kernel-object handle (S-001) must be fixed with CHERI and tagged-memory implications recorded so a hardware-tag path can enforce unforgeability later (§7, §8, §65); CAP-008 is the input and S-001 stays prototyped.

## Options

### Option A · Dense index
Summary: A handle is a small index into the Component's handle table.
Consequences: Compact and cache-friendly lookup; index reuse needs generation counters and a forged index is only caught at the table check.
Evidence: none

### Option B · Sparse id
Summary: A handle is a wide sparse identifier that is hard to guess and checked in a per-Component map.
Consequences: Accidental reuse and guessing are harder; lookup is a map rather than an array and the id must still be validated at the boundary.
Evidence: none

### Option C · Sealed value
Summary: A handle is a sealed value the kernel mints and userspace cannot construct, shaped to map onto a CHERI sealed capability.
Consequences: Direct path to hardware enforcement; on x86-64 today the seal is software and the layout must be reserved now.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
