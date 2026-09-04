# D-0184 · Decide glibc /usr/lib interoperation with Packages
- Status: proposed
- Task: LNX-024
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §29, §46, §56.3
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
How the native dependency model interoperates with Linux shared libraries expecting /usr/lib must be decided for unmodified glibc at V1 (§29, §46, §56.3), answering Q-022.

## Options

### Option A · Personality-only /usr/lib view over Package contents
Summary: A view synthesises /usr/lib from Packages.
Consequences: Clean; view work.
Evidence: none

### Option B · Copied FHS tree
Summary: An FHS tree is copied.
Consequences: Simple; duplication.
Evidence: none

### Option C · FHS as the native store
Summary: FHS is native.
Consequences: Compatibility; rejected against I-020.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
