# D-0245 · Decide Package and SystemGeneration signing scheme
- Status: proposed
- Task: REL-003
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §27, §28, §30
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
V1 daily-driving requires a signed remote repository whose clients reject tampering (§27, §28, §30); the scheme must be fixed before the first signed artifacts.

## Options

### Option A · Signed content-addressed index at V1 with TUF roles at V3
Summary: Index now, TUF later.
Consequences: Incremental; interim gaps.
Evidence: none

### Option B · TUF root, targets, snapshot and timestamp from the first channel
Summary: TUF from the start.
Consequences: Complete; upfront.
Evidence: none

### Option C · Per-Package signatures only
Summary: Per-Package only.
Consequences: Simple; replay and mix-and-match undetected.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
