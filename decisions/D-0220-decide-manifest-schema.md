# D-0220 · Decide the Package manifest schema shape and its Layer 2 evolution rules
- Status: proposed
- Task: PKG-011
- Surfaces: S-018
- Layer: L2
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §11, §12, §28, §66
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The Package manifest schema shape and its Layer 2 evolution rules are a hard-to-change decision depending on SEC's authority-source and grant-taxonomy adrs (§11, §12, §28, §66); S-018 is the freeze candidate.

## Options

### Option A · Single typed manifest document
Summary: One document.
Consequences: Simple; large.
Evidence: none

### Option B · Split Package-plus-Component manifest pair
Summary: A pair matching S-018 and S-019.
Consequences: Modular; two schemas.
Evidence: none

### Option C · IDL-defined manifest served as a Layer 2 interface
Summary: The manifest is an IDL type.
Consequences: Unified evolution; indirection.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
