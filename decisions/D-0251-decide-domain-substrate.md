# D-0251 · Decide ResourceDomain over cgroup v2 controllers versus native accounting
- Status: proposed
- Task: SCH-003
- Surfaces: none
- Layer: none
- Spikes: SCH-012
- Supersedes: none
- Superseded by: none
- Baseline: §6, §23, §53, §57
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Whether ResourceDomain is implemented over cgroup v2 as a Phase C detail or as native accounting is decided from the B-011 report (§6, §23, §53, §57).

## Options

### Option A · cgroup v2 as an internal Phase C implementation
Summary: cgroups implement domains internally and never appear on the native path.
Consequences: Reuse of mature controllers; per-domain create and teardown cost is cgroup's.
Evidence: none

### Option B · Native accounting with no cgroup controllers on the native path
Summary: Native kernel accounting applied to every Component and Task.
Consequences: Exact fit and no cgroupfs anywhere; kernel implementation effort.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
