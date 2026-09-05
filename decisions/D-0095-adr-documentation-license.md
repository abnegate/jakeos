# D-0095 · Decide the documentation license and translation terms
- Status: accepted
- Task: GOV-021
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §56.5, §66
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Documentation and translation terms must be fixed before the V1 docs site and V3 translations (§56.5, §66); DOC consumes the Decision.

## Options

### Option A · CC-BY-SA
Summary: Documentation is licensed share-alike with attribution and translations must carry the same license.
Consequences: Derivatives and translations stay open; incorporation into differently licensed commercial documentation is blocked.
Evidence: none

### Option B · CC-BY
Summary: Documentation requires attribution only.
Consequences: Widest reuse including commercial guides; downstream copies may become closed.
Evidence: none

### Option C · CC0
Summary: Documentation is dedicated to the public domain.
Consequences: No friction for any reuse; no attribution and no way to require translations to remain open.
Evidence: none

## Decision
Option B. Documentation is licensed CC-BY 4.0. It may be quoted, translated, embedded in books, courses and commercial products with attribution.

## Consequences
- The docs repository and every generated reference page carry the licence.
- Contributions to documentation fall under the same CLA as the platform repository (D-0092 terms).
- Baseline and decision records inside the roadmap repository carry the same licence.

## Rejected options and why
- Option A (CC-BY-SA) rejected: share-alike discourages commercial documentation reuse the ecosystem benefits from.
- Option C (CC0) rejected: attribution is the one thing the project wants back.

## Follow-ups
none
