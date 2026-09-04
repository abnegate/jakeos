# D-0002 · Decide the 1.x stability declaration superseding the freeze ADR with stable for 1.x
- Status: proposed
- Task: ABI-053
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §65, §66
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The Layer 1 freeze ADR is immutable, so declaring Layer 1 stable for the 1.x line requires a superseding Decision that states whether any Layer 1 change after 1.0 forces a new major OS version (§65, §66). The public stability policy text must match what is accepted here.

## Options

### Option A · Stable for 1.x as frozen
Summary: Every frozen Layer 1 surface is declared stable for the whole 1.x line with no exceptions.
Consequences: Any Layer 1 change requires a new major OS version; the policy text is one sentence and third parties can rely on it without reading exception lists; mistakes discovered after 1.0 cannot be corrected inside 1.x.
Evidence: none

### Option B · Stable with a listed exception set
Summary: Layer 1 is declared stable except for an enumerated set of surfaces that may still change within 1.x.
Consequences: Known weak spots can be corrected without a major version; the exception list becomes a second contract to maintain and dilutes the stability promise.
Evidence: none

### Option C · Decline to declare stable
Summary: Layer 1 stays in freeze-candidate state through 1.x and the stability statement is deferred.
Consequences: No premature promise; SDK and ecosystem cannot rely on a Layer 1 contract, undermining the 1.0 stability declaration that §66 requires.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
