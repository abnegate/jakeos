# D-0065 · Decide the order and equivalence tests for replacing the Component wrapper
- Status: proposed
- Task: CMP-042
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §6
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Phase C later replaces the Component wrapper pieces (mm_struct, namespaces, cgroups, task_struct) and needs an order and an ABI-equivalence strategy (§6).

## Options

### Option A · Address-space-first
Summary: mm_struct is replaced first, then membership and identity.
Consequences: Isolation becomes native earliest; scheduling and accounting stay Linux-shaped longer.
Evidence: none

### Option B · Membership-first
Summary: cgroups and namespaces are replaced first.
Consequences: ResourceDomain becomes native earliest; address spaces stay Linux-shaped longer.
Evidence: none

### Option C · Big-bang replacement
Summary: All wrapper pieces are replaced together.
Consequences: No intermediate hybrid; a large, risky flag day.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
