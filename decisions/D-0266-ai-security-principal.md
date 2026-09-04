# D-0266 · Decide whether an AI assistant is a distinct principal
- Status: proposed
- Task: SEC-034
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §44, §57
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
The V2 AI demo is Capability-only, so whether the assistant is a distinct principal for audit and revocation must be decided (§44, §57), answering Q-037.

## Options

### Option A · Assistant is a distinct principal
Summary: The assistant has its own identity and grants.
Consequences: Auditable and revocable separately; grant complexity.
Evidence: none

### Option B · Assistant acts as the user
Summary: The assistant uses the user's grants.
Consequences: Simple; no attribution.
Evidence: none

### Option C · Hybrid with distinct audit identity and user-held grants
Summary: Audit identity is distinct while grants are the user's.
Consequences: Attribution without a second grant set; complexity.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
