# D-0265 · Define administrator versus standard user
- Status: proposed
- Task: SEC-013
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §9.1, §63
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The identity service must create the first account as administrator and Authorization must know which operations elevate (§9.1, §63).

## Options

### Option A · First account is administrator with an elevation Capability
Summary: The first account holds an elevation Capability.
Consequences: Clear bootstrap; a single privileged account.
Evidence: none

### Option B · All local accounts equivalent until first elevation
Summary: Accounts are equal until one elevates.
Consequences: Simple; ambiguity about who administers.
Evidence: none

### Option C · Role bit on the identity object
Summary: A role bit marks administrators.
Consequences: Explicit; uid-like authority by flag.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
