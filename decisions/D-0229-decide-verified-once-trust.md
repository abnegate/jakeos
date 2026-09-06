# D-0229 · Decide the verified-once launch trust mechanism
- Status: proposed
- Task: PKG-050
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §34
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
V1 introduces signed activation: a SystemGeneration is activated only after its signature chain (D-0245) verifies, and launch must then trust cached objects without re-hashing them (§34). This decision revisits D-0230's V0.5 answer for the signed world and states how it composes with the verification cache (PKG-044) and generation verity (STO-052), and what a tampered object fails with at activation.

## Options

### Option A · dm-verity-style block verification
Summary: Activation checks the signature over the generation's verity root hash; dm-verity then verifies every block on read, so launch trusts any mapping and tampering faults at the page that was altered.
Consequences: Continuous kernel enforcement with one signed root per generation, which is also what measured boot attests. Requires the generation to be materialised as a verity-protected image or volume (D-0217), and per-object updates always produce a new root.
Evidence: none

### Option B · Signed content-store index
Summary: Activation verifies a signed content-store index; the verification cache records which objects were hashed against it; launch trusts cache hits.
Consequences: Works with any store layout and any filesystem, and cache misses degrade to a hash rather than a failure. Trust is only as strong as the store volume's integrity between verification and use; a local attacker who can write the store defeats it unless the volume is otherwise protected (D-0268).
Evidence: none

### Option C · Per-launch hash of a small manifest
Summary: Launch hashes a per-Package manifest and trusts the objects it names.
Consequences: Cheap and layout-agnostic. It re-checks only the manifest, so the objects themselves rely on install-time verification and V1's signed activation would prove nothing about them at run time; a tampered object runs. Rejected as the sole mechanism.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
