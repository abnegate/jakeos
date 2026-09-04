# D-0038 · Decide SBOM format for Packages and SystemGenerations
- Status: proposed
- Task: BLD-054
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §27, §28
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The V2 SBOM generator needs a format decided before store and repository SBOMs and V3 attestations (§27, §28); BLD emits from the content-addressed graph and REL publishes.

## Options

### Option A · SPDX only
Summary: SBOMs are emitted in SPDX.
Consequences: ISO-standard format favoured by compliance; weaker vulnerability tooling ecosystem.
Evidence: none

### Option B · CycloneDX only
Summary: SBOMs are emitted in CycloneDX.
Consequences: Strong security-tooling support; not the format regulators cite most.
Evidence: none

### Option C · Both emitted from the same graph
Summary: Both formats are emitted from one graph.
Consequences: Every consumer served; two emitters to keep equivalent.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
