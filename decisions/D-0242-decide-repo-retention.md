# D-0242 · Decide repository retention of past generations
- Status: proposed
- Task: REL-029
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §30, §31
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Rollback and restore promises are only honest if the objects an older generation references remain downloadable (§30, §31). This decision bounds the 1.0 guarantee of N previous generations and the PKG garbage collector's repository-side counterpart (PKG-052): what a client may still fetch after a generation leaves the channel head. It sits on the channel model (D-0248).

## Options

### Option A · Retain every object referenced by a supported generation plus a published N-previous floor
Summary: Every object referenced by any generation still within the supported window (per channel) is retained, plus a published floor of N previous generations regardless of age.
Consequences: Rollback within the window always works and the promise is a number users can read. Storage grows with the window times the churn, and the window must be defined per channel with LTS the longest.
Evidence: none

### Option B · Retain only the current channel head
Summary: Only objects referenced by the current channel head are retained.
Consequences: Minimal storage. Rolling back to any generation that references a changed object fails to fetch, so the rollback guarantee is void the moment the head moves; rejected.
Evidence: none

### Option C · Retain everything forever
Summary: Every object ever published is retained forever.
Consequences: Every historical generation is reproducible. Unbounded storage with no funding plan, and it retains objects with known vulnerabilities indefinitely; rejected, though a transparency log (D-0247) may keep hashes forever.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
