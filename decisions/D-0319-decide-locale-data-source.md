# D-0319 · Decide the locale data source between ICU/CLDR and an ICU4X port
- Status: accepted
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
Option B. Locale data (number, date and list formatting, collation, calendars, plural rules) comes from ICU4X, the Rust-native implementation of CLDR data. Each text Component links only the data it needs.

## Consequences
- The SDK exposes ICU4X types directly; no C++ ICU library ships in the native platform.
- The Linux personality keeps its own ICU for guests; the two never share state.
- Features ICU4X lacks are tracked as TXT tasks with upstream contributions preferred.

## Rejected options and why
- Option A (ICU with CLDR in C++) rejected: a large foreign-object-model dependency inside every text-rendering Component, against the Rust-first SDK.

## Follow-ups
none
