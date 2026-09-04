# D-0146 · Decide that IDL compiler output is owned by its user with no copyleft obligation
- Status: proposed
- Task: IPC-005
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §14, §51
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Generated stubs land in every application from V0, so the licence of IDL compiler output must be clear to avoid contaminating the ecosystem (§14, §51).

## Options

### Option A · Generated-code exception
Summary: Output is owned by the compiler user with no copyleft obligation.
Consequences: Ecosystem-safe; header text to maintain.
Evidence: none

### Option B · Output inherits the compiler license
Summary: Stubs carry the compiler's license.
Consequences: Simple; copyleft contamination if the compiler is copyleft.
Evidence: none

### Option C · Output dedicated to the public domain
Summary: Stubs are public domain.
Consequences: No obligations; some jurisdictions lack public domain.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
