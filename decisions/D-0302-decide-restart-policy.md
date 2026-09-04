# D-0302 · Decide restart budgets, strategies, backoff and escalation for supervised services
- Status: proposed
- Task: SVC-005
- Surfaces: none
- Layer: none
- Spikes: SVC-014
- Supersedes: none
- Superseded by: none
- Baseline: §32
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The V0.5 compositor-restart gate and V2 safe-mode escalation need one restart-policy model (§32); S-020 records the fields.

## Options

### Option A · BEAM-style strategies with per-service budgets
Summary: one-for-one, rest-for-one and one-for-all strategies with per-service budgets.
Consequences: Expressive dependency-aware restarts; a learning curve for service authors.
Evidence: none

### Option B · systemd StartLimit-style windows
Summary: Restart limits within a time window.
Consequences: Familiar; coarse and no dependency semantics.
Evidence: none

### Option C · Fuchsia eager versus lazy restart
Summary: Each service is eager or lazy.
Consequences: Simple to declare; limited expressiveness.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
