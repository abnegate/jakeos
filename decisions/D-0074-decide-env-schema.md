# D-0074 · Decide environment.yaml schema versus Package manifest profile
- Status: proposed
- Task: ENV-008
- Surfaces: S-021
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §28, §29, §35
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
V1 os env needs a canonical project definition (§28, §29, §35), choosing whether environment.yaml is a distinct schema or a Package manifest profile and recording lockfile and version pinning; S-021 stays prototyped.

## Options

### Option A · Distinct YAML environment.yaml plus sibling lockfile
Summary: A separate YAML schema with a lockfile and repository name discovery.
Consequences: Familiar to developers; a second manifest dialect to version.
Evidence: none

### Option B · Profile of the Package manifest
Summary: The environment is a profile of the typed Package manifest with locks as Dependencies.
Consequences: One schema and one lock model; less approachable than YAML for newcomers.
Evidence: none

### Option C · Distinct typed non-YAML environment manifest plus lockfile
Summary: A separate typed manifest format with a lockfile.
Consequences: Typed and validated; another format to learn.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
