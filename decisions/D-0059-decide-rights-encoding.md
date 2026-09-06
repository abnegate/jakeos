# D-0059 · Decide rights and transfer-rights encoding including Admin authority
- Status: proposed
- Task: CAP-010
- Surfaces: S-003
- Layer: L1
- Spikes: CAP-013, CAP-012
- Supersedes: none
- Superseded by: none
- Baseline: §7, §8
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Rights and transfer or delegation rights, including Admin authority, must be encoded so attenuation is a subset check a hardware-tag path can perform without kernel metadata (§7, §8, S-003), staying prototyped through V0.

## Options

### Option A · Generic bitmask
Summary: One rights word of generic bits shared by every object type.
Consequences: Subset check is a mask; per-type meaning of bits is implicit.
Evidence: none

### Option B · Per-object-type typed rights
Summary: Each object type defines its own rights set.
Consequences: Precise and self-documenting; the subset check needs per-type tables.
Evidence: none

### Option C · Rights as separate kernel objects
Summary: Rights are first-class objects attached to a Capability.
Consequences: Very expressive delegation; heavy and not hardware-checkable.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
