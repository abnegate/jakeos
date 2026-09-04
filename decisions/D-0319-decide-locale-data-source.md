# D-0319 · Decide the locale data source between ICU/CLDR and an ICU4X port
- Status: proposed
- Task: TXT-016
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §41, §67
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Formatting, collation, plural rules and time zones need a CLDR-backed implementation before V2 (§41, §67).

## Options

### Option A · ICU with CLDR
Summary: ICU.
Consequences: Complete; size.
Evidence: none

### Option B · ICU4X port
Summary: ICU4X.
Consequences: Modular Rust; maturity.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
