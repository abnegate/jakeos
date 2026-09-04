# D-0285 · Decide encryption layering across the verified system store and encrypted user data
- Status: proposed
- Task: STO-039
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §26, §27, §51
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Whether the publicly known system store is verified-but-unencrypted while user data is encrypted, and how dedup interacts with encryption, must precede the V3 FDE installer (§26, §27, §51).

## Options

### Option A · Verified-unencrypted system store plus encrypted user data
Summary: A split policy.
Consequences: Dedup preserved; the store is visible.
Evidence: none

### Option B · Encrypt-everything
Summary: Everything encrypted.
Consequences: Uniform; dedup loss.
Evidence: none

### Option C · Encrypt-store with convergent encryption
Summary: Convergent encryption.
Consequences: Dedup with encryption; known-plaintext exposure.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
