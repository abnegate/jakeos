# D-0240 · Decide release, SystemGeneration and Channel versioning
- Status: proposed
- Task: REL-001
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §28, §30, §66
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Packages, SystemGenerations and public release names need one vocabulary before the first immutable install (§28, §30, §66, I-080).

## Options

### Option A · Generation counters plus channel names for OS artifacts and semver for Layer 2 and Layer 3
Summary: Mixed scheme.
Consequences: Fits each artifact; two schemes.
Evidence: none

### Option B · Semver for every artifact including generations
Summary: Semver everywhere.
Consequences: Uniform; generations are not semver.
Evidence: none

### Option C · Calendar-free public names plus opaque generation IDs
Summary: Names plus IDs.
Consequences: Marketing-friendly; opaque.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
