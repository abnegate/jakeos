# D-0215 · Decide how security fixes reach a library pinned by many Packages
- Status: proposed
- Task: PKG-046
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §29
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
How a security fix reaches a library pinned by many Packages without global mutation must be decided before REL's first CVE response (§29), answering Q-021.

## Options

### Option A · Rebuild-and-republish of dependents
Summary: Dependents are rebuilt.
Consequences: Clean identities; slow response.
Evidence: none

### Option B · Grafting a substitute object into a new generation
Summary: A substitute is grafted.
Consequences: Fast; correctness risk.
Evidence: none

### Option C · Runtime relinking to a patched object
Summary: Relink at runtime.
Consequences: Fastest; complexity.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
