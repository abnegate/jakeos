# D-0031 · Decide benchmark methodology and target-kind policy
- Status: proposed
- Task: BEN-007
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §54, §59
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Benchmark methodology and target-kind policy must answer Q-001 and keep numeric targets only in registers/benchmarks.md (§54, §59), rejecting a numeric V0 IPC exit.

## Options

### Option A · Register target kinds with V0 publish-only
Summary: Kinds publish, absolute and regression live only in the register and V0 gates publish numbers without targets.
Consequences: Prose never restates numbers; V0 cannot fail on performance.
Evidence: none

### Option B · Milestone files restate numeric exits
Summary: Milestone files carry numbers beside B-IDs.
Consequences: Self-contained milestone text; two sources of truth that drift.
Evidence: none

### Option C · Numeric V0 exits including a same-core IPC absolute
Summary: V0 has absolute targets from the start.
Consequences: Early discipline; targets set before hardware and methodology are calibrated.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
