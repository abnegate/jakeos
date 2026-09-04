# D-0263 · Decide the Layer 3 SDK semver and deprecation policy
- Status: proposed
- Task: SDK-054
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §52, §66
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The V1 gate requires an accepted SDK stability policy; Layer 3 evolves with semver, and S-031 is a freeze candidate (§52, §66).

## Options

### Option A · Semver with recorded deprecation windows
Summary: Layer 3 follows semver with deprecation windows.
Consequences: Predictable evolution; discipline in every release.
Evidence: none

### Option B · Lockstep with Layer 2 interface versions
Summary: Layer 3 versions track Layer 2.
Consequences: Simple mapping; coupling of unrelated changes.
Evidence: none

### Option C · Freeze Layer 3 at V1
Summary: Layer 3 freezes.
Consequences: Stability; premature and contrary to R-028.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
