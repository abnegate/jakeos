# D-0211 · Decide the tracing substrate and its measured overhead ceiling
- Status: proposed
- Task: OBS-003
- Surfaces: S-010
- Layer: L1
- Spikes: OBS-010
- Supersedes: none
- Superseded by: none
- Baseline: §24, §58
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The V0 tracing substrate and overhead ceiling that B-012 verifies must be decided with S-010 prototyped (§24, §58).

## Options

### Option A · Extend ftrace/tracepoints/eBPF
Summary: Extend inherited tracing.
Consequences: Mature; Linux shape.
Evidence: none

### Option B · Native per-Component structured ring
Summary: A native ring per Component.
Consequences: Exact fit; effort.
Evidence: none

### Option C · Native semantic schema over eBPF
Summary: A native schema on eBPF.
Consequences: Reuse plus semantics; two layers.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
