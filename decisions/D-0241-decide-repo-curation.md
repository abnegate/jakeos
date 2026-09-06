# D-0241 · Define repository curation and free versus non-free channels
- Status: proposed
- Task: REL-028
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §28, §63
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Steam, proprietary drivers and firmware need an explicit non-free channel with user consent; the main channel needs review criteria, approval roles and an appeal path (§28, §63). GOV owns redistribution licensing (GOV-022); this decision is REL's operational policy, sitting on the repository model (D-0243) and the prebuilt policy (D-0234). The accepted option names roles, appeal path and how non-free is shown before install (I-021).

## Options

### Option A · Automated scan plus audit with a non-free channel requiring explicit consent
Summary: Automated scanning (licence, SBOM, malware, manifest lint) on every submission plus periodic human audit; a separate non-free channel that the store shows as non-free and enables only after explicit consent.
Consequences: Scales with the ecosystem and Steam works for users who opt in. Automated checks miss what they were not written for, so audits must be scheduled and funded, and the non-free consent screen must be honest without being alarming.
Evidence: none

### Option B · Full human review of every Package
Summary: Every Package is reviewed by a human before publication.
Consequences: Highest quality bar. A bottleneck proportional to submissions and a single reviewer at first, so publication latency grows unboundedly; rejected as the sole gate.
Evidence: none

### Option C · No non-free channel
Summary: No non-free channel; only free software is published.
Consequences: Ideological clarity and one policy. No Steam, no proprietary GPU or Wi-Fi firmware beyond what D-0097 already ships, which fails the §56.2 compatibility goal; rejected.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
