# D-0309 · Decide native expression of termination, cancellation and async notification without signals
- Status: proposed
- Task: TSK-006
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §1, §18, §19, §21
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Signals have no native equivalent (§1, §18); the notification model for termination, cancellation and async wake-ups must be named with rejected signal-like options.

## Options

### Option A · Operation completion plus Wait-able objects
Summary: Completions and Wait-able objects notify.
Consequences: Uniform and typed; no signals anywhere.
Evidence: none

### Option B · Typed Channel messages as the sole wake-up
Summary: Only Channel messages wake a Task.
Consequences: Simple; message overhead for every event.
Evidence: none

### Option C · Retained signal-like native event
Summary: A signal-like event exists.
Consequences: Familiar; rejected as a POSIX shape.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
