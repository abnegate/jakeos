# D-0219 · Decide how immutable Packages preserve LGPL relinking rights
- Status: proposed
- Task: PKG-010
- Surfaces: none
- Layer: none
- Spikes: none
- Supersedes: none
- Superseded by: none
- Baseline: §28
- Revisit when: an accepted later Decision supersedes this one, or a spike shows the chosen option cannot meet a Gate that cites it

## Context
Content-addressed packaging must preserve LGPL section 6 substitution for libraries such as glibc and Wine (§28), coordinated with GOV.

## Options

### Option A · Dynamically linked separate store objects the user can replace
Summary: Libraries are separate objects.
Consequences: Natural substitution; linking constraints.
Evidence: none

### Option B · Shipping relinkable object files inside the Package
Summary: Object files ship.
Consequences: Works for static links; bloat.
Evidence: none

### Option C · Documented local-generation substitution flow
Summary: A documented flow.
Consequences: Compliance path; user effort.
Evidence: none

## Decision
Proposed. Not yet accepted.

## Consequences
None until Status is accepted.

## Rejected options and why
None until Status is accepted.

## Follow-ups
none
