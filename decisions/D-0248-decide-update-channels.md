# D-0248 · Define update channels and promotion criteria
- Status: proposed
- Task: REL-004
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §30, §61
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Internal daily driving at V1 needs a nightly channel and a written path to testing; stable and LTS follow with public releases (§30, §61). Promotion publishes an existing SystemGeneration to another channel and never rebuilds it (I-086); live patching stays a non-goal. This decision names the channels per rung, the soak criteria for promotion, and who may promote.

## Options

### Option A · Nightly and testing at V1 with stable and LTS later
Summary: Nightly and testing at V1; stable at V3 with the public alpha; LTS at 1.0 with the support statement.
Consequences: Each channel appears when something needs it and the promotion path (nightly soak, then testing soak, then stable) is exercised for two rungs before the public sees it. The stable and LTS criteria are written later, so the V1 decision must reserve their names and the promotion grammar now.
Evidence: none

### Option B · All four channels from V1
Summary: All four channels exist from V1.
Consequences: One complete model from the start and no later channel decision. Stable and LTS have no consumers and no soak evidence at V1, so their criteria are invented rather than learned.
Evidence: none

### Option C · Single rolling channel with named tags
Summary: One rolling channel with named tags for releases.
Consequences: Nothing to promote. Users cannot choose stability, the 1.0 support statement has no branch to attach to, and a bad nightly reaches everyone; rejected.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
