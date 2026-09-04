# D-0155 · Decide which kernel evolution phase is required at 1.0
- Status: proposed
- Task: KRN-050
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §6
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Which kernel evolution phase (C, D or E) is a hard 1.0 requirement is KRN divergence policy (§6), answering Q-003.

## Options

### Option A · Phase C wrappers still allowed at 1.0
Summary: Wrapper implementations may remain at 1.0.
Consequences: Achievable schedule; Linux internals persist under the ABI.
Evidence: none

### Option B · Phase D controlled divergence required
Summary: Controlled divergence must have begun before 1.0.
Consequences: Guaranteed progress; upstream merge cost rises.
Evidence: none

### Option C · Phase E independent native ABI required
Summary: Full independence from Linux internals is required.
Consequences: Architectural purity; likely to slip 1.0.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
