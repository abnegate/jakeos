# D-0247 · Decide whether releases use a transparency log
- Status: proposed
- Task: REL-030
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §51, §63
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Binary transparency defends against a compromised pipeline shipping targeted builds (§51, §63).

## Options

### Option A · Sigstore or Rekor-style log with client inclusion proofs on stable
Summary: A log with client proofs.
Consequences: Strong; client work.
Evidence: none

### Option B · Log without client proofs
Summary: A log only.
Consequences: Auditability; no client guarantee.
Evidence: none

### Option C · No transparency log for 1.0
Summary: No log.
Consequences: Simple; blind.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
