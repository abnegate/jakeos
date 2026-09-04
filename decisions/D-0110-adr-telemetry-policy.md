# D-0110 · Decide opt-in usage telemetry policy apart from crash reporting
- Status: proposed
- Task: GOV-055
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §24, §63
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The V3 opt-in alpha telemetry Gate needs a policy for collection, anonymisation, retention and the user-visible toggle, apart from crash reporting (§24, §63).

## Options

### Option A · No usage telemetry
Summary: The project collects no usage data at all.
Consequences: Maximum privacy and nothing to secure; product decisions are made without usage evidence.
Evidence: none

### Option B · Opt-in counters with a user-visible toggle
Summary: Anonymised counters are collected only while a visible toggle is on.
Consequences: Consented data with a clear off switch; low participation biases the sample.
Evidence: none

### Option C · Opt-out telemetry
Summary: Data is collected unless the user disables it.
Consequences: High participation; rejected unless the Decision records why consent-by-default is acceptable.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
