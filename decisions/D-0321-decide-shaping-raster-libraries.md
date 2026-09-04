# D-0321 · Decide the shaping and rasterisation libraries for the native text stack
- Status: proposed
- Task: TXT-003
- Surfaces: none
- Layer: none
- Spikes: TXT-011
- Supersedes: none
- Superseded by: none
- Baseline: §41, §67
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Every V0.5 application renders text through one stack; the spike and Principle 15 apply (§41, §67).

## Options

### Option A · Retain HarfBuzz plus FreeType
Summary: HarfBuzz and FreeType.
Consequences: Mature; a C unsafe surface.
Evidence: none

### Option B · Rust-native rustybuzz, swash or cosmic-text
Summary: A Rust-native stack.
Consequences: Safety; maturity.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
