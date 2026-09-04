# D-0252 · Decide intent and priority inheritance across Channel handoff
- Status: proposed
- Task: SCH-017
- Surfaces: none
- Layer: none
- Spikes: SCH-011
- Supersedes: none
- Superseded by: none
- Baseline: §15, §22, §32
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
A LowLatency client calling a shared service must not queue behind Background work; the rule is needed before rebind ships (§15, §22, §32).

## Options

### Option A · Per-request intent propagation
Summary: Each message carries the caller's intent and the service honours it.
Consequences: Precise prioritisation; attenuation rules to stop clients inflating intent.
Evidence: none

### Option B · Receiver-side boosting
Summary: The receiving Task is boosted while serving a higher-intent request.
Consequences: Simple to implement; coarse and still queues behind earlier work.
Evidence: none

### Option C · Dedicated per-class service Tasks
Summary: Services run one Task per intent class.
Consequences: Isolation between classes; more Tasks and memory per service.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
