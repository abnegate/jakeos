# D-0279 · Decide where assistant models execute
- Status: proposed
- Task: SEM-017
- Surfaces: none
- Layer: none
- Spikes: SEM-032
- Supersedes: none
- Superseded by: none
- Baseline: §37, §44, §57
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Where assistant models execute must be decided without bundling a model runtime in the OS (§37, §44, §57); the spike is the evidence.

## Options

### Option A · Local ComputeDevice or NPU only
Summary: Models run locally.
Consequences: Privacy; hardware limits.
Evidence: none

### Option B · Remote service only
Summary: Models run remotely.
Consequences: Capability; privacy and network dependence.
Evidence: none

### Option C · User-selectable both
Summary: The user chooses.
Consequences: Choice; complexity.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
