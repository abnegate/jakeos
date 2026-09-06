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
Once third parties publish Packages (V3), the repository either rebuilds every Package from source on project infrastructure or accepts publisher-built binaries with provenance and an SBOM (§27, §28, §51). The choice decides whether reproducibility and SBOMs are project-wide guarantees or publisher-dependent claims, and how a client tells a rebuilt object from a prebuilt one. It lands before REL-021 accepts submissions and sits on the repository model (D-0243) and the reproducible-build infrastructure (BLD-054).

## Options

### Option A · Rebuild every Package from source on project infrastructure
Summary: Every published Package is rebuilt from source on project infrastructure; only the rebuilt objects enter the repository.
Consequences: Uniform provenance, reproducibility checks and SBOMs for everything users install, and no publisher key compromise can ship a binary. Build farm cost scales with the ecosystem, proprietary applications (permitted by D-0262) cannot be rebuilt, and build failures become the project's problem.
Evidence: none

### Option B · Accept publisher prebuilts with provenance and SBOM
Summary: Publishers upload prebuilt Packages with a signed provenance attestation and an SBOM; the repository verifies the attestation format and signature, not the build.
Consequences: Scales to any ecosystem size and admits proprietary software. Reproducibility and SBOM accuracy are the publisher's claims, and the client must show that a Package is publisher-built rather than project-built.
Evidence: none

### Option C · Prebuilts only on the non-free channel
Summary: Open-source Packages are rebuilt (A); prebuilts with attestations are accepted only on a separate channel that the store labels as publisher-built.
Consequences: Project-wide guarantees for the main channel and a path for proprietary software with an honest label. Two publishing policies, two channels for users to understand, and a Package that moves between them changes identity.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
