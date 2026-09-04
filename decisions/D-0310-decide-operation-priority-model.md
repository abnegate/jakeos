# D-0310 · Decide how Operation priority relates to ResourceDomain Scheduling intent
- Status: proposed
- Task: TSK-027
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §19, §22
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Every Operation carries a priority (§19, §22); its relation to ResourceDomain intent must land before the kernel orders I/O by it.

## Options

### Option A · Inherit-from-domain intent
Summary: Priority comes from the domain's intent.
Consequences: Consistent; no per-Operation control.
Evidence: none

### Option B · Per-Operation override bounded by the domain
Summary: Overrides within the domain's bound.
Consequences: Flexibility; complexity in ordering.
Evidence: none

### Option C · Independent Operation priority
Summary: Priority is independent of intent.
Consequences: Full control; conflicts with intent.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
