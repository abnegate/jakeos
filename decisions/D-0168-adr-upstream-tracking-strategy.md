# D-0168 · Decide upstream tracking: rebase vs merge and cadence per divergence phase
- Status: proposed
- Task: KRN-007
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §6, §56.4
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Rebase versus merge and cadence for phases A through C must be fixed for the merge bot and V1 rebase gate (§6, §56.4).

## Options

### Option A · Periodic rebase onto the chosen series
Summary: The fork rebases regularly.
Consequences: Clean patch series and easy delta reports; conflict pain concentrated at each rebase.
Evidence: none

### Option B · Merge each upstream tag
Summary: Upstream tags are merged.
Consequences: Simple and continuous; a messy history that hides the delta.
Evidence: none

### Option C · Rebase for LTS plus merge for mainline
Summary: Hybrid per series.
Consequences: Best of both; two workflows for the merge bot.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
