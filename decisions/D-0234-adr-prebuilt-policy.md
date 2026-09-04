# D-0234 · Decide whether the repository accepts publisher prebuilts or rebuilds every Package
- Status: proposed
- Task: REL-016
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §27, §28, §51
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Public third-party Packages force a rebuild-versus-prebuilt choice that decides whether reproducibility and SBOMs are project-wide (§27, §28, §51).

## Options

### Option A · Rebuild every Package from source on project infrastructure
Summary: Everything is rebuilt.
Consequences: Uniform provenance; infrastructure cost.
Evidence: none

### Option B · Accept publisher prebuilts with provenance and SBOM
Summary: Prebuilts with attestations.
Consequences: Scales; publisher-dependent trust.
Evidence: none

### Option C · Prebuilts only on the non-free channel
Summary: Prebuilts confined to non-free.
Consequences: Clean main channel; two policies.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
