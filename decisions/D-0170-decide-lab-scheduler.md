# D-0170 · Decide lab job-scheduler family and unbootable-machine recovery
- Status: proposed
- Task: LAB-005
- Surfaces: none
- Layer: none
- Spikes: LAB-008
- Supersedes: none
- Superseded by: none
- Baseline: §30
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
GAP-0141 needs a lab scheduler family and a recovery path via firmware boot-order fallback so V1 nightly hardware jobs are not manual (§30); the spike is the evidence.

## Options

### Option A · LAVA
Summary: LAVA schedules lab jobs.
Consequences: Mature with device-type abstractions; complex to operate.
Evidence: none

### Option B · KernelCI
Summary: KernelCI schedules lab jobs.
Consequences: Kernel-focused and community-run; less flexible for desktop and graphics jobs.
Evidence: none

### Option C · Custom scheduler
Summary: A project scheduler is built.
Consequences: Exact fit for generation flashing; engineering effort.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
