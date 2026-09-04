# D-0030 · Decide the blocking performance merge-Gate policy
- Status: proposed
- Task: BEN-033
- Surfaces: none
- Layer: none
- Spikes: BEN-022
- Supersedes: none
- Superseded by: none
- Baseline: §54
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
V1 is the first rung with absolute performance targets, so the blocking merge-Gate policy that BLD's CI jobs enforce must be fixed (§54), with the noise band taken only from BEN-022.

## Options

### Option A · Block merge on regression beyond the noise band
Summary: A regression beyond the calibrated band on B-001, B-004, B-016 and B-020 blocks the merge.
Consequences: Regressions cannot land silently; flaky noise calibration blocks unrelated work.
Evidence: none

### Option B · Nightly-only fail
Summary: Regressions fail a nightly job but never block merges.
Consequences: Merge velocity preserved; regressions are found late and bisected by hand.
Evidence: none

### Option C · Warn with a required Decision
Summary: A regression warns and may merge only with an accepted Decision.
Consequences: Deliberate trade-offs are recorded; the Decision path can become a rubber stamp.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
