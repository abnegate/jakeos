# D-0039 · Decide a semantic-Interface GUI test harness over pixel scripting
- Status: proposed
- Task: BLD-018
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §41, §42, §60
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
V0.5 four apps and compositor-restart need an end-to-end GUI harness (§41, §42, §60), and the project's rule is that Semantic interfaces beat GUI scraping.

## Options

### Option A · Semantic interfaces plus the accessibility tree
Summary: The harness drives applications through Semantic interfaces and the accessibility tree.
Consequences: Tests are robust to layout change and double as accessibility coverage; requires the tree and interfaces to exist early.
Evidence: none

### Option B · Coordinate scripting
Summary: The harness clicks and types by coordinates.
Consequences: Works with any app immediately; brittle and useless for accessibility.
Evidence: none

### Option C · Pixel goldens as the primary driver
Summary: Scenarios are verified by screenshot comparison.
Consequences: Catches rendering regressions; fragile across fonts, scaling and hardware.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
