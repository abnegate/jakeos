# D-0244 · Decide the signing key hierarchy and custody model
- Status: proposed
- Task: REL-002
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §28, §30, §51, §63
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Every update, rollback and Secure Boot chain rests on who can sign a SystemGeneration (§28, §30, §51, §63, T-028, T-029).

## Options

### Option A · Offline root with HSM-backed threshold intermediates per channel plus publisher keys
Summary: An offline root.
Consequences: Robust; operational rigor.
Evidence: none

### Option B · Software-held root with delayed HSM migration
Summary: A software root.
Consequences: Fast start; exposure.
Evidence: none

### Option C · Single online root for all artifacts
Summary: An online root.
Consequences: Simple; catastrophic compromise.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
