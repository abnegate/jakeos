# D-0007 · Decide the Layer 1 handle word: packing of the CAP-008 representation, type tag and generation
- Status: proposed
- Task: ABI-010
- Surfaces: S-001
- Layer: L1
- Spikes: ABI-022
- Supersedes: none
- Superseded by: none
- Baseline: §7, §8, §65
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
CAP-008 (D-0055) chooses the handle representation and the per-Component table layout. This Decision fixes only how that representation is packed into the Layer 1 syscall word (S-001): where the type tag and generation live, how many bits stay reserved for a future sealed-pointer layout, and what the kernel checks at the boundary (§7, §8, §65). Deciding the representation twice was the dissonance this split removes; S-001 stays prototyped.

## Options

### Option A · Type tag and generation inline in the word
Summary: The syscall word carries the CAP-008 index or token plus an inline type tag and generation the kernel checks before the table lookup.
Consequences: Wrong-type and stale-handle misuse fails before touching the table; the word spends bits on metadata that a sealed-pointer layout must also reserve.
Evidence: none

### Option B · Opaque word with tag and generation held in the table
Summary: The syscall word is the bare CAP-008 representation; type tag and generation are read from the table entry on every Operation.
Consequences: Smallest word and the simplest mapping onto a sealed capability; every check is a table read and a forged word is only rejected at the table.
Evidence: none

### Option C · Hybrid with a reserved sealed-pointer region
Summary: Inline tag for the common check plus a reserved region of the word that a CHERI-class path can later replace with hardware sealing.
Consequences: Keeps the hardware escape hatch explicit at the cost of a wider word now.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
