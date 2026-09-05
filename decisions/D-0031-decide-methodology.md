# D-0031 · Decide benchmark methodology and target-kind policy
- Status: accepted
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
Option A. Targets live only in registers/benchmarks.md as a target kind per milestone: publish (measure and publish, no threshold), absolute (a stated threshold) or regression (a bound against the latest report for an earlier milestone on the same hardware). Every V0 target is publish. Milestone gates cite B-IDs and never restate numbers. No prose anywhere in the project states a performance number without a B-ID.

## Consequences
- The register is the only place a number lives; the validator already warns on hand-typed numbers in prose.
- V0 closes on published measurements, never on hitting a figure; the first absolute targets appear at V1.
- Regression targets compare committed reports on the same H-ID, so every gate run must commit its report.

## Rejected options and why
- Option B (numeric V0 exits) rejected: it would have the project promise figures before any hardware or harness exists, exactly the unmeasured claim §57 forbids.
- Option C (restate numbers in milestone files) rejected: two copies of every target drift.

## Follow-ups
none
