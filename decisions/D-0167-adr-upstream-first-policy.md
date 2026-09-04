# D-0167 · Decide the upstream-first policy for the hardware layer and Rust abstractions
- Status: proposed
- Task: KRN-006
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §5.1, §6, §55
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Fixes and reusable Rust abstractions either go upstream first or accumulate as fork-only patches (§5.1, §6, §55).

## Options

### Option A · Upstream-first
Summary: Patches land upstream before or with the fork.
Consequences: Minimal divergence and shared maintenance; slower landing in the fork.
Evidence: none

### Option B · Alongside
Summary: Patches are posted upstream in the same week they land in the fork.
Consequences: Balance of speed and alignment; two trees to track per patch.
Evidence: none

### Option C · Fork-only with later contribution
Summary: Patches land in the fork first and are contributed later.
Consequences: Fastest local progress; growing divergence and rebase pain.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
