# D-0253 · Decide how intents map onto Linux scheduler mechanisms versus a native class
- Status: proposed
- Task: SCH-004
- Surfaces: none
- Layer: none
- Spikes: SCH-012
- Supersedes: none
- Superseded by: none
- Baseline: §5.1, §6, §22, §65
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The native intent interface must map onto the retained Linux scheduler or a native class (§5.1, §6, §22, §65); the spike scores the options and S-009 stays prototyped.

## Options

### Option A · Mapping onto retained Linux scheduler knobs
Summary: Intent maps to EEVDF nice, latency-nice, SCHED_DEADLINE, uclamp and cgroup cpu controls.
Consequences: No scheduler changes; a leaky mapping that exposes Linux behaviour.
Evidence: none

### Option B · sched_ext BPF scheduler
Summary: A sched_ext scheduler implements intent classes.
Consequences: Flexible policy iteration; dependence on BPF and sched_ext availability.
Evidence: none

### Option C · New native scheduling class
Summary: A native scheduling class is added to the kernel.
Consequences: Exact semantics; kernel scheduler work and upstream divergence.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
