# D-0016 · Decide the Layer 1 version identification and feature-negotiation scheme
- Status: proposed
- Task: ABI-016
- Surfaces: S-011
- Layer: L1
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §12, §65
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The Layer 1 handshake must negotiate version, feature bits or both (§65 rule 6) so older and newer Components and kernels interoperate, with S-011 recorded as prototyped.

## Options

### Option A · Version word
Summary: First entry exchanges a single version word.
Consequences: Trivial to implement and freeze; feature-level differences need a new version for every change.
Evidence: none

### Option B · Feature bits
Summary: First entry exchanges a feature bitmap.
Consequences: Fine-grained optional features; no total order of compatibility and bit exhaustion must be planned.
Evidence: none

### Option C · Version word plus feature bits
Summary: A version word carries the base contract and feature bits carry optional extensions.
Consequences: Both coarse and fine negotiation; two mechanisms to document and test.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
