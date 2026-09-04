# D-0277 · Decide user identity versus Capability roots
- Status: proposed
- Task: SEC-012
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §7, §9.1
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Kernel uid-like identity versus a userspace identity service mapped to Capability roots must be chosen so V1 identity work is not blocked (§7, §9.1).

## Options

### Option A · Kernel uid-like identity
Summary: A kernel uid.
Consequences: Familiar; uid becomes authority.
Evidence: none

### Option B · Userspace identity-service mapped to Capability roots
Summary: A userspace service.
Consequences: Capability-pure; a service to run.
Evidence: none

### Option C · Hybrid kernel identifier plus userspace root holder
Summary: Kernel id plus userspace holder.
Consequences: Both; complexity.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
