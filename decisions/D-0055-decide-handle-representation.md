# D-0055 · Decide the userspace Capability<T> handle representation and table design
- Status: proposed
- Task: CAP-008
- Surfaces: S-001
- Layer: L1
- Spikes: CAP-013, CAP-012, CAP-015
- Supersedes: none
- Superseded by: none
- Baseline: §7, §8, §38, §65
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
How a Capability<T> is represented to userspace and laid out in the per-Component table replaces file descriptors and must reserve room for hardware enforcement (§7, §8, §38, §65), staying prototyped through V0 and feeding ABI-010.

## Options

### Option A · Dense per-Component handle-table index
Summary: A handle is an index into a dense per-Component table.
Consequences: Compact and fast; generation counters are needed against reuse and the layout is not CHERI-shaped.
Evidence: none

### Option B · Sparse unforgeable 64-bit token
Summary: A handle is a wide token that cannot be guessed.
Consequences: Harder to forge by accident; lookup is a map and the token is still validated in software.
Evidence: none

### Option C · Sealed-pointer layout reservable for CHERI
Summary: A handle uses a layout that a CHERI sealed capability can occupy.
Consequences: Hardware path reserved now; software sealing on x86-64 today.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
