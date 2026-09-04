# D-0046 · Decide atomicity of kernel, driver and firmware updates within a SystemGeneration
- Status: proposed
- Task: BOOT-030
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §30, §31
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
§30 rollback conflicts with firmware that cannot be rolled back, so the Decision defines what a generation guarantees and what §31 history records as irreversible, answering Q-024.

## Options

### Option A · Firmware outside generations
Summary: Firmware updates are independent of generations.
Consequences: Generations stay purely software; restore cannot reason about firmware state.
Evidence: none

### Option B · Firmware staged with the generation and flagged irreversible
Summary: Firmware is part of the generation but marked as a one-way step.
Consequences: History is honest about irreversibility; a generation may be unrestorable in full.
Evidence: none

### Option C · Firmware applied only after boot-success
Summary: Firmware applies once the new generation is marked good.
Consequences: Rollback of the software half stays clean; firmware lags by one boot.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
